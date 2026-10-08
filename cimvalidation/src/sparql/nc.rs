//! NCP's `sh:sparql` constraints, and its two shapes with a `sh:SPARQLTarget`,
//! written by hand.
//!
//! Two groups, by where the APL puts them:
//!
//! - [`validate_local`] — the five in files every per-profile manifest
//!   imports (`DatasetMetadata-AP-Con-SHACL.ttl`,
//!   `NC-AP-Con-ClassCount-Complex-SHACL.ttl`): they describe one dataset and
//!   run on each NC file.
//! - [`validate_merged`] — the 32 in the Complex files, which only the
//!   combined `NCP-AP-Con-Complex-Validation` manifest imports. They relate
//!   objects across datasets, so they run once on the merged dataset, beside
//!   the Complex files' table shapes (profile code
//!   `cimschema::shacl::resolve::MERGED_PROFILE`).
//!
//! Each rule follows its SPARQL; where it departs, the rule says so. Presence
//! is the SPARQL's `EXISTS { $this p ?o }`: any value, text or reference.
//! A target class matches as the table's targets do: a concrete class itself,
//! an abstract one its concrete descendants.

use cimmodel::base::{FastMap, FieldValue};
use cimmodel::{CimDataset, Element};

use crate::Violation;

// ── element access ─────────────────────────────────────────────────────────

fn has(e: &Element, key: &str) -> bool {
    e.fields().contains_key(key)
}

/// Every reference `key` holds.
fn refs<'a>(e: &'a Element, key: &str) -> &'a [String] {
    match e.fields().get(key) {
        Some(FieldValue::Resource(r)) => std::slice::from_ref(r),
        Some(FieldValue::ResourceList(rs)) => rs,
        _ => &[],
    }
}

/// Every text value `key` holds.
fn texts<'a>(e: &'a Element, key: &str) -> &'a [String] {
    match e.fields().get(key) {
        Some(FieldValue::Text(t)) => std::slice::from_ref(t),
        Some(FieldValue::TextList(ts)) => ts,
        _ => &[],
    }
}

/// Whether `key` holds the enumeration value `value` (`Kind.literal`).
fn is(e: &Element, key: &str, value: &str) -> bool {
    refs(e, key).iter().any(|r| r == value)
}

/// The elements `sh:targetClass` reaches for each of `classes`.
fn targets<'a>(ds: &'a CimDataset, classes: &[&str]) -> Vec<(&'a String, &'a Element)> {
    let reg = cimmodel::registry::type_registry();
    let mut out = Vec::new();
    for class in classes {
        let keys = match reg.by_type_name(class) {
            Some(c) if c.concrete => vec![c.qualified],
            Some(_) => reg.concrete_descendants(class),
            None => Vec::new(),
        };
        for key in keys {
            for mrid in ds.by_type.get(key).into_iter().flatten() {
                out.push((mrid, &ds.entries[mrid]));
            }
        }
    }
    out
}

/// The class of the element `mrid` names, bare (`ApparentPowerLimit`), or
/// `None` when the dataset does not hold it — the SPARQL's `rdf:type` step,
/// which binds nothing for a node without a type.
fn class_of(ds: &CimDataset, mrid: &str) -> Option<&'static str> {
    ds.entries.get(mrid).map(Element::local_name)
}

/// Who points at whom, for the inverse paths the rules take. Built over the
/// listed fields only.
struct Inverse<'a> {
    by_key: FastMap<&'static str, FastMap<&'a str, Vec<&'a String>>>,
}

impl<'a> Inverse<'a> {
    fn new(ds: &'a CimDataset, keys: &[&'static str]) -> Self {
        let mut by_key: FastMap<&'static str, FastMap<&'a str, Vec<&'a String>>> = FastMap::default();
        for (mrid, e) in &ds.entries {
            for key in keys {
                for r in refs(e, key) {
                    by_key.entry(key).or_default().entry(r.as_str()).or_default().push(mrid);
                }
            }
        }
        Self { by_key }
    }

    /// The elements whose `key` references `target`.
    fn of(&self, key: &str, target: &str) -> &[&'a String] {
        self.by_key.get(key).and_then(|m| m.get(target)).map_or(&[], Vec::as_slice)
    }
}

/// A rule's fixed report text.
struct Rule {
    id: &'static str,
    name: &'static str,
    property: &'static str,
    message: &'static str,
    severity: &'static str,
}

impl Rule {
    fn at(&self, mrid: &str, e: &Element) -> Violation {
        self.with_message(mrid, e, self.message.to_string())
    }

    fn with_message(&self, mrid: &str, e: &Element, message: String) -> Violation {
        Violation {
            object_id: mrid.to_string(),
            rule_id: self.id.into(),
            name: self.name.into(),
            class: e.type_name().into(),
            property: self.property.into(),
            message,
            severity: self.severity.into(),
            description: String::new(),
        }
    }
}

const VIOLATION: &str = "sh:Violation";
const TYPE: &str = "rdf:type";

/// Run `rule` on the targets of `classes`, reporting each element `fails`.
fn each(
    ds: &CimDataset,
    classes: &[&str],
    rule: &Rule,
    mut fails: impl FnMut(&String, &Element) -> bool,
    out: &mut Vec<Violation>,
) {
    for (mrid, e) in targets(ds, classes) {
        if fails(mrid, e) {
            out.push(rule.at(mrid, e));
        }
    }
}

// ── per dataset ────────────────────────────────────────────────────────────

const HEADERS: &[&str] = &["nc:Dataset", "nc:DifferenceSet"];

/// The rules of the files every NC profile imports, on one dataset.
pub fn validate_local(ds: &CimDataset) -> Vec<Violation> {
    let mut v = Vec::new();
    class_count(ds, &mut v);
    each(ds, HEADERS, &Rule {
        id: "dm:conformsTo-NC-cardinality",
        name: "C:MD:ALL:Dataset.conformsTo:cardinality",
        property: TYPE,
        message: "Cardinality violation. Missing required property.",
        severity: VIOLATION,
    }, |_, e| {
        !has(e, "conformsTo")
            && refs(e, "spatial").iter().any(|s| s != "https://energy.referencedata.eu/Frame/BoundaryModel")
    }, &mut v);
    for (key, id, message) in [
        ("description", "dm:description.langTag-presence", "Language tag is not declared or it is not en."),
        ("versionNotes", "dm:versionNotes.langTag-presence", "Language tag is not declared."),
    ] {
        // `lang(?value) != "en"` for any literal value; a value with no tag
        // has `lang` "". A reference has no `lang` at all, so it never fails.
        each(ds, HEADERS, &Rule { id, name: &id[3..], property: key, message, severity: VIOLATION }, |_, e| {
            texts(e, key).len() > e.langs(key).filter(|l| *l == "en").count()
        }, &mut v);
    }
    each(ds, HEADERS, &Rule {
        id: "dm:alternativeVersionAndPreferredVersion-dependency",
        name: "alternativeVersionAndPreferredVersion-dependency",
        property: TYPE,
        message: "One of the properties dcatcim:alternativeVersionOf or dcatcim:preferredVersion is not present.",
        severity: VIOLATION,
    }, |_, e| has(e, "alternativeVersionOf") != has(e, "preferredVersion"), &mut v);
    v
}

/// `ClassCount` (`sh:Info`): for each dataset header, how many instances of
/// each class the dataset holds — one finding per class, in class order.
fn class_count(ds: &CimDataset, out: &mut Vec<Violation>) {
    let rule = Rule {
        id: "cc:ClassCount-property",
        name: "ClassCount",
        property: TYPE,
        message: "",
        severity: "sh:Info",
    };
    let mut counts: Vec<(&str, usize)> = ds.by_type.iter().map(|(k, v)| (k.as_str(), v.len())).collect();
    counts.sort_unstable();
    for (mrid, e) in targets(ds, &["nc:Dataset"]) {
        for (class, n) in &counts {
            out.push(rule.with_message(mrid, e, format!("The class {class} appears {n} times in the data graph.")));
        }
    }
}

// ── merged datasets ────────────────────────────────────────────────────────

const INVERSE_KEYS: &[&str] = &[
    "AssessedElement.OperationalLimit",
    "BoundaryPointBorderLink.BoundaryPoint",
    "DCPole.DCTieCorridor",
    "HydroGeneratingUnit.HydroPowerPlant",
    "HydroPump.HydroPowerPlant",
    "PowerSchedule.DCPole",
    "PowerSchedule.SchedulingArea",
    "PowerScheduleAction.PowerSchedule",
    "PowerTimePoint.PowerSchedule",
    "RemedialActionScheduleDependency.RemedialActionScheduleGroup",
    "RotatingMachine.GeneratingUnit",
];

/// The rules of the Complex files, on the merged dataset.
pub fn validate_merged(ds: &CimDataset) -> Vec<Violation> {
    let inv = Inverse::new(ds, INVERSE_KEYS);
    let mut v = Vec::new();
    assessed_element(ds, &inv, &mut v);
    peer_temporal_dependencies(ds, &mut v);
    equipment_reliability(ds, &inv, &mut v);
    power_schedule(ds, &inv, &mut v);
    remedial_action_schedule(ds, &inv, &mut v);
    security_analysis_result(ds, &mut v);
    power_bid_schedule(ds, &mut v);
    dangling_references(ds, &mut v);
    v
}

fn assessed_element(ds: &CimDataset, inv: &Inverse, v: &mut Vec<Violation>) {
    // `$this ^nc:AssessedElement.OperationalLimit/nc:AssessedElement.inBaseCase true`.
    // RDF/XML writes the flag as untyped text, which never equals the typed
    // `true` the SPARQL names, so read as written every limit would fail; the
    // text `true` is taken as the boolean, as everywhere else.
    each(ds, &["nc:BaseCaseCurrentLimit"], &Rule {
        id: "aec:OperationalLimit.AssessedElement-required",
        name: "C:NC:AE:OperationalLimit.AssessedElement:required",
        property: TYPE,
        message: "BaseCaseCurrentLimit is not associated with an AssessedElement with AssessedElement.inBaseCase set to true.",
        severity: VIOLATION,
    }, |mrid, _| {
        !inv.of("AssessedElement.OperationalLimit", mrid).iter().any(|ae| {
            texts(&ds.entries[*ae], "AssessedElement.inBaseCase").iter().any(|t| t.trim() == "true")
        })
    }, v);
}

/// The same rule in three profiles, each on its own dependency class.
fn peer_temporal_dependencies(ds: &CimDataset, v: &mut Vec<Violation>) {
    const ATTRS: [&str; 4] = [
        "PeerTemporalDependency.overlap",
        "PeerTemporalDependency.startToStartLag",
        "PeerTemporalDependency.finishToStartLag",
        "PeerTemporalDependency.finishToFinishLag",
    ];
    for (class, id, name) in [
        ("nc:OutageDependency", "asc:PeerTemporalDependency.constraintKind-exclusive", "C:NC:AS:PeerTemporalDependency.constraintKind:exclusive"),
        ("nc:ThermalGeneratingUnitDependency", "erc:PeerTemporalDependency.constraintKind-exclusive", "C:NC:ER:PeerTemporalDependency.constraintKind:exclusive"),
        ("nc:PowerBidDependency", "sisc:PeerTemporalDependency.constraintKind-exclusive", "C:NC:SIS:PeerTemporalDependency.constraintKind:exclusive"),
    ] {
        each(ds, &[class], &Rule {
            id,
            name,
            property: TYPE,
            message: "Some of the following attributes are defined: PeerTemporalDependency.overlap, PeerTemporalDependency.startToStartLag, PeerTemporalDependency.finishToStartLag, PeerTemporalDependency.finishToFinishLag when PeerTemporalDependency.constraintKind is set to DependencyConstraintKind.exclusive.",
            severity: VIOLATION,
        }, |_, e| {
            is(e, "PeerTemporalDependency.constraintKind", "DependencyConstraintKind.exclusive")
                && ATTRS.iter().any(|a| has(e, a))
        }, v);
    }
}

fn equipment_reliability(ds: &CimDataset, inv: &Inverse, v: &mut Vec<Violation>) {
    // The machines' operating modes, reached from a plant through its
    // generating units or its pumps. The SPARQL lists the cim16, cim17 and
    // CIM100 spellings of each path; field keys carry no namespace, so one
    // walk covers them.
    each(ds, &["nc:HydroPowerPlant", "HydroPowerPlant"], &Rule {
        id: "erc:HydroPowerPlant-operatingMode",
        name: "C:NC:ER:HydroPowerPlant:operatingMode",
        property: TYPE,
        message: "The SynchronousMachine.operatingMode is not consistent for all SynchronousMachine objects part of a HydroPowerPlant.",
        severity: VIOLATION,
    }, |mrid, _| {
        let mut machines: Vec<&String> = Vec::new();
        for unit in inv.of("HydroGeneratingUnit.HydroPowerPlant", mrid) {
            machines.extend(inv.of("RotatingMachine.GeneratingUnit", unit));
        }
        for pump in inv.of("HydroPump.HydroPowerPlant", mrid) {
            machines.extend(refs(&ds.entries[*pump], "HydroPump.RotatingMachine"));
        }
        let mut modes: Vec<&str> = machines
            .iter()
            .filter_map(|m| ds.entries.get(m.as_str()))
            .flat_map(|m| refs(m, "SynchronousMachine.operatingMode"))
            .map(String::as_str)
            .collect();
        modes.sort_unstable();
        modes.dedup();
        modes.len() > 1
    }, v);

    // Every ordered pair of distinct borders must be the same two different
    // zones in opposite directions.
    each(ds, &["nc:CrossZonalLine", "nc:CrossZonalPowerTransformer", "nc:CrossZonalDCPole"], &Rule {
        id: "erc:CrossZonalNetworkElement.BiddingZoneBorder-usage",
        name: "C:NC:ER:CrossZonalNetworkElement.BiddingZoneBorder:usage",
        property: "CrossZonalNetworkElement.BiddingZoneBorder",
        message: "The two BiddingZoneBorder instances do not represent the same pair of different bidding zones in opposite directions.",
        severity: VIOLATION,
    }, |_, e| {
        let ends = |b: &str| -> Vec<(&str, &str)> {
            let Some(b) = ds.entries.get(b) else { return Vec::new() };
            let to = refs(b, "BiddingZoneBorder.ToBiddingZone");
            refs(b, "BiddingZoneBorder.FromBiddingZone")
                .iter()
                .flat_map(|f| to.iter().map(move |t| (f.as_str(), t.as_str())))
                .collect()
        };
        let borders = refs(e, "CrossZonalNetworkElement.BiddingZoneBorder");
        borders.iter().any(|b1| {
            borders.iter().filter(|b2| *b2 != b1).any(|b2| {
                ends(b1).iter().any(|(from1, to1)| {
                    ends(b2).iter().any(|(from2, to2)| from1 == to1 || from1 != to2 || to1 != from2)
                })
            })
        })
    }, v);

    each(ds, &["nc:BoundaryPoint"], &Rule {
        id: "erc:BoundaryPoint-requiredAssociation",
        name: "C:NC:ER:BoundaryPoint:requiredAssociation",
        property: TYPE,
        message: "BoundaryPoint is not associated with neither BoundaryPointBorder nor BoundaryPointBorderLink.",
        severity: VIOLATION,
    }, |mrid, e| {
        // Both or neither.
        has(e, "BoundaryPoint.BoundaryPointBorder") != inv.of("BoundaryPointBorderLink.BoundaryPoint", mrid).is_empty()
    }, v);

    for (class, one, two, id, name, message) in [
        ("nc:BoundaryPoint", "BoundaryPoint.partyOneResourceName", "BoundaryPoint.partyTwoResourceName",
         "erc:BoundaryPoint-naming", "C:NC:ER:BoundaryPoint:naming",
         "BoundaryPoint name is not consistent with partyOneResourceName and partyTwoResourceName."),
        ("nc:BoundaryPointBorder", "BoundaryPointBorder.partyOneName", "BoundaryPointBorder.partyTwoName",
         "erc:BoundaryPointBorder-naming", "C:NC:ER:BoundaryPointBorder:naming",
         "BoundaryPointBorder name is not consistent with partyOneName and partyTwoName."),
    ] {
        // For every pair of party names: the name is absent or differs.
        each(ds, &[class], &Rule { id, name, property: "IdentifiedObject.name", message, severity: VIOLATION }, |_, e| {
            let names = texts(e, "IdentifiedObject.name");
            texts(e, one).iter().any(|p1| {
                texts(e, two).iter().any(|p2| {
                    let expected = format!("{p1}-{p2}");
                    names.is_empty() || names.iter().any(|n| *n != expected)
                })
            })
        }, v);
    }

    each(ds, &["nc:PointOfCommonCoupling", "nc:BoundaryPoint", "nc:GridConnectionPoint", "nc:EnergyExchangePoint"], &Rule {
        id: "erc:CommonResponsibilityPoint-requiredAssociation",
        name: "C:NC:ER:CommonResponsibilityPoint:requiredAssociation",
        property: TYPE,
        message: "CommonResponsibilityPoint in not associated with neither DCNode nor ConnectivityNode or it is associated with both.",
        severity: VIOLATION,
    }, |_, e| has(e, "CommonResponsibilityPoint.DCNode") == has(e, "CommonResponsibilityPoint.ConnectivityNode"), v);

    // A link is only for a boundary point that two links tie to two
    // different borders.
    each(ds, &["nc:BoundaryPointBorderLink"], &Rule {
        id: "erc:BoundaryPointBorderLink-usage",
        name: "C:NC:ER:BoundaryPointBorderLink:usage",
        property: "BoundaryPointBorderLink.BoundaryPoint",
        message: "BoundaryPointBorderLink is used for a BoundaryPoint that is not associated with more than one BoundaryPointBorder.",
        severity: VIOLATION,
    }, |_, e| {
        refs(e, "BoundaryPointBorderLink.BoundaryPoint").iter().any(|bp| {
            let links = inv.of("BoundaryPointBorderLink.BoundaryPoint", bp);
            let border_of = |l: &str| refs(&ds.entries[l], "BoundaryPointBorderLink.BoundaryPointBorder");
            !links.iter().any(|l1| {
                links.iter().filter(|l2| *l2 != l1).any(|l2| {
                    border_of(l1).iter().any(|b1| border_of(l2).iter().any(|b2| b1 != b2))
                })
            })
        })
    }, v);
}

/// `PowerSchedule.currency` is required once a time point has a price. The
/// PowerSchedule and RemedialActionSchedule Complex files each state it, under
/// their own shape.
fn priced_without_currency(ds: &CimDataset, inv: &Inverse, mrid: &str, e: &Element) -> bool {
    !has(e, "PowerSchedule.currency")
        && inv
            .of("PowerTimePoint.PowerSchedule", mrid)
            .iter()
            .any(|tp| has(&ds.entries[*tp], "PowerTimePoint.price"))
}

fn power_schedule(ds: &CimDataset, inv: &Inverse, v: &mut Vec<Violation>) {
    // Only when the dataset holds a PowerSchedule at all — typed exactly so,
    // as `?powerSchedule rdf:type nc:PowerSchedule` reads.
    let any_schedule = ds.by_type.get("nc:PowerSchedule").is_some_and(|s| !s.is_empty());
    each(ds, &["nc:DCTieCorridor"], &Rule {
        id: "psc:DCTieCorridor-powerSchedule",
        name: "C:NC:PS:PowerSchedule:dc-associations",
        property: TYPE,
        message: "The PowerSchedule associated with the DCTieCorridor is not defined through its SchedulingArea or one of its DCPole instances.",
        severity: VIOLATION,
    }, |mrid, e| {
        let by_area = refs(e, "DCTieCorridor.SchedulingArea")
            .iter()
            .any(|a| !inv.of("PowerSchedule.SchedulingArea", a).is_empty());
        let by_pole = inv
            .of("DCPole.DCTieCorridor", mrid)
            .iter()
            .any(|p| !inv.of("PowerSchedule.DCPole", p).is_empty());
        any_schedule && by_area && by_pole
    }, v);
    each(ds, &["nc:PowerSchedule"], &Rule {
        id: "psc:PowerSchedule-currency-property",
        name: "C:NC:RAS:PowerSchedule:currency",
        property: "PowerSchedule.currency",
        message: "PowerSchedule.currency is not provided but a PowerTimePoint.price is provided.",
        severity: VIOLATION,
    }, |mrid, e| priced_without_currency(ds, inv, mrid, e), v);
}

fn remedial_action_schedule(ds: &CimDataset, inv: &Inverse, v: &mut Vec<Violation>) {
    each(ds, &["nc:RemedialActionScheduleGroup"], &Rule {
        id: "rasc:RemedialActionScheduleDependency.kind-cardinality",
        name: "C:NC:RAS:RemedialActionScheduleDependency.kind:cardinality",
        property: TYPE,
        message: "RemedialActionScheduleDependency.kind is not provided for RemedialActionScheduleDependency objects part of a RemedialActionScheduleGroup.",
        severity: VIOLATION,
    }, |mrid, _| {
        !inv.of("RemedialActionScheduleDependency.RemedialActionScheduleGroup", mrid)
            .iter()
            .any(|d| has(&ds.entries[*d], "RemedialActionScheduleDependency.kind"))
    }, v);
    each(ds, &["nc:RemedialActionScheduleDependency"], &Rule {
        id: "rasc:RemedialActionScheduleDependency.kind-applicability",
        name: "C:NC:RAS:RemedialActionScheduleDependency.kind:applicability",
        property: TYPE,
        message: "RemedialActionScheduleDependency.kind is provided for RemedialActionScheduleDependency objects not part of a RemedialActionScheduleGroup.",
        severity: VIOLATION,
    }, |_, e| {
        has(e, "RemedialActionScheduleDependency.kind")
            && !has(e, "RemedialActionScheduleDependency.RemedialActionScheduleGroup")
    }, v);
    each(ds, &["nc:RemedialActionScheduleResponse"], &Rule {
        id: "rasc:RemedialActionScheduleResponse.rejectionReasonKind-required",
        name: "C:NC:RAS:RemedialActionScheduleResponse.rejectionReasonKind:required",
        property: TYPE,
        message: "RemedialActionScheduleResponse.rejectionReasonKind is missing when RemedialActionScheduleResponse.kind equals RemedialActionScheduleResponseKind.refused.",
        severity: VIOLATION,
    }, |_, e| {
        is(e, "RemedialActionScheduleResponse.kind", "RemedialActionScheduleResponseKind.refused")
            && !has(e, "RemedialActionScheduleResponse.rejectionReasonKind")
    }, v);
    each(ds, &["nc:RemedialActionScheduleResponse"], &Rule {
        id: "rasc:RemedialActionScheduleResponse.rejectionReason-applicability",
        name: "C:NC:RAS:RemedialActionScheduleResponse.rejectionReason:applicability",
        property: TYPE,
        message: "RemedialActionScheduleResponse.rejectionReason is missing when RemedialActionScheduleResponse.rejectionReasonKind equals RejectionReasonKind.other.",
        severity: "sh:Warning",
    }, |_, e| {
        is(e, "RemedialActionScheduleResponse.rejectionReasonKind", "RejectionReasonKind.other")
            && !has(e, "RemedialActionScheduleResponse.rejectionReason")
    }, v);
    each(ds, &["nc:PowerSchedule"], &Rule {
        id: "rasc:PowerSchedule-currencyConsistency",
        name: "C:NC:RAS:PowerSchedule:currencyConsistency",
        property: "PowerSchedule.currency",
        message: "Currency is missing. If PowerTimePoint.price is provided, currency shall be provided either on PowerSchedule or on the associated PowerScheduleAction.",
        severity: VIOLATION,
    }, |mrid, e| {
        priced_without_currency(ds, inv, mrid, e)
            && inv.of("PowerScheduleAction.PowerSchedule", mrid).iter().any(|a| {
                let a = &ds.entries[*a];
                matches!(a.type_name(), "nc:CountertradeScheduleAction" | "nc:RedispatchScheduleAction")
                    && !has(a, "PowerScheduleAction.currency")
            })
    }, v);
    each(ds, &["nc:PowerSchedule"], &Rule {
        id: "rasc:PowerSchedule-currency-property",
        name: "C:NC:RAS:PowerSchedule:currency",
        property: "PowerSchedule.currency",
        message: "PowerSchedule.currency is not provided but a PowerTimePoint.price is provided.",
        severity: VIOLATION,
    }, |mrid, e| priced_without_currency(ds, inv, mrid, e), v);
    each(ds, &["nc:CountertradeScheduleAction", "nc:RedispatchScheduleAction"], &Rule {
        id: "rasc:PowerScheduleAction-currency-property",
        name: "C:NC:RAS:PowerScheduleAction:currency",
        property: "PowerScheduleAction.currency",
        message: "PowerScheduleAction.currency is not provided but a PowerScheduleAction.energyPrice is provided.",
        severity: VIOLATION,
    }, |_, e| has(e, "PowerScheduleAction.energyPrice") && !has(e, "PowerScheduleAction.currency"), v);
}

const POWER_FLOW_RESULTS: &[&str] = &["nc:BaseCasePowerFlowResult", "nc:ContingencyPowerFlowResult"];

fn security_analysis_result(ds: &CimDataset, v: &mut Vec<Violation>) {
    each(ds, POWER_FLOW_RESULTS, &Rule {
        id: "sarc:PowerFlowResult.value",
        name: "C:NC:SAR:PowerFlowResult:value",
        property: TYPE,
        message: "PowerFlowResult is associated with OperationalLimit but PowerFlowResult.value and/or PowerFlowResult.absoluteValue are not provided.",
        severity: VIOLATION,
    }, |_, e| {
        has(e, "PowerFlowResult.OperationalLimit")
            && !(has(e, "PowerFlowResult.value") && has(e, "PowerFlowResult.absoluteValue"))
    }, v);

    // A value required by the kind of limit referenced. VoltageLimit is a
    // node shape with a SPARQL target and `sh:minCount 1`; the rest are
    // SPARQL constraints. The limit's class is read wherever the dataset holds
    // it, in either family; a limit it does not hold binds no type. Neither
    // CGMES 3.0 nor NCP defines ReactivePowerLimit (the SPARQL names its
    // cim16/cim17 spellings), so that rule fires only on a class table that
    // does, loaded from RDFS.
    for (limit, value, id, name, message) in [
        ("ApparentPowerLimit", "PowerFlowResult.valueVA", "sarc:PowerFlowResult-ApparentPowerLimit", "C:NC:SAR:PowerFlowResult:ApparentPowerLimit", "PowerFlowResult.valueVA is not provided for ApparentPowerLimit."),
        ("ActivePowerLimit", "PowerFlowResult.valueW", "sarc:PowerFlowResult-ActivePowerLimit", "C:NC:SAR:PowerFlowResult:ActivePowerLimit", "PowerFlowResult.valueW is not provided for ActivePowerLimit."),
        ("ReactivePowerLimit", "PowerFlowResult.valueVAR", "sarc:PowerFlowResult-ReactivePowerLimit", "C:NC:SAR:PowerFlowResult:ReactivePowerLimit", "PowerFlowResult.valueVAR is not provided for ReactivePowerLimit."),
        ("VoltageAngleLimit", "PowerFlowResult.valueAngle", "sarc:PowerFlowResult-VoltageAngleLimit", "C:NC:SAR:PowerFlowResult:VoltageAngleLimit", "PowerFlowResult.valueAngle is not provided for VoltageAngleLimit."),
        ("CurrentLimit", "PowerFlowResult.valueA", "sarc:PowerFlowResult-CurrentLimit", "C:NC:SAR:PowerFlowResult:CurrentLimit", "PowerFlowResult.valueA is not provided for CurrentLimit."),
        ("VoltageLimit", "PowerFlowResult.valueV", "sarc:PowerFlowResult-VoltageLimit", "C:NC:SAR:PowerFlowResult:VoltageLimit", "PowerFlowResult.valueV is required attribute when a VoltageLimit is referenced by the association end PowerFlowResult.OperationalLimit."),
    ] {
        each(ds, POWER_FLOW_RESULTS, &Rule { id, name, property: value, message, severity: VIOLATION }, |_, e| {
            !has(e, value)
                && refs(e, "PowerFlowResult.OperationalLimit").iter().any(|l| class_of(ds, l) == Some(limit))
        }, v);
    }

    // The SPARQL binds the class to `?ratype` and tests an unbound `?ra`, so
    // as written it never reports; this is the rule its description states.
    each(ds, &["nc:RemedialActionApplied"], &Rule {
        id: "sarc:RemedialActionApplied.StageForRemedialActionScheme-cardinality",
        name: "C:NC:SAR:RemedialActionApplied.StageForRemedialActionScheme:cardinality",
        property: TYPE,
        message: "RemedialActionApplied is either not associated with Stage for a SchemeRemedialAction or it is provided for other type of RemedialAction.",
        severity: VIOLATION,
    }, |_, e| {
        let stage = has(e, "RemedialActionApplied.StageForRemedialActionScheme");
        refs(e, "RemedialActionApplied.RemedialAction")
            .iter()
            .filter_map(|ra| class_of(ds, ra))
            .any(|class| (class == "SchemeRemedialAction") != stage)
    }, v);

    each(ds, POWER_FLOW_RESULTS, &Rule {
        id: "sarc:PowerFlowResult-voltageAndAngle",
        name: "C:NC:SAR:PowerFlowResult:voltageAndAngle",
        property: TYPE,
        message: "PowerFlowResult.valueV and PowerFlowResult.valueAngle are instantiated if PowerFlowResult is associated with ACDCTerminal or PowerTransferCorridor.",
        severity: VIOLATION,
    }, |_, e| {
        (has(e, "PowerFlowResult.valueV") || has(e, "PowerFlowResult.valueAngle"))
            && (has(e, "PowerFlowResult.ACDCTerminal") || has(e, "PowerFlowResult.PowerTransferCorridor"))
    }, v);
    each(ds, POWER_FLOW_RESULTS, &Rule {
        id: "sarc:PowerFlowResult-topologicalNode",
        name: "C:NC:SAR:PowerFlowResult:topologicalNode",
        property: TYPE,
        message: "PowerFlowResult.valueW, PowerFlowResult.valueVA, PowerFlowResult.valueVAR and PowerFlowResult.valueA are instantiated if PowerFlowResult is associated with TopologicalNode.",
        severity: VIOLATION,
    }, |_, e| {
        has(e, "PowerFlowResult.TopologicalNode")
            && ["PowerFlowResult.valueW", "PowerFlowResult.valueVA", "PowerFlowResult.valueVAR", "PowerFlowResult.valueA"]
                .iter()
                .any(|k| has(e, k))
    }, v);
}

fn power_bid_schedule(ds: &CimDataset, v: &mut Vec<Violation>) {
    // Allowed: activation cost alone, shutdown cost alone, or any of the
    // others without either cost.
    each(ds, &["nc:PowerBidScheduleTimePoint"], &Rule {
        id: "sisc:PowerBidScheduleTimePoint-attributes",
        name: "C:NC:SIS:PowerBidScheduleTimePoint:attributes",
        property: TYPE,
        message: "PowerBidScheduleTimePoint does not have one of the following combinations of optional attributes: 1) activationCost, 2) shutdownCost or 3) any of the following p, minimumActivationP, price, reservePrice, stepIncrementP.",
        severity: VIOLATION,
    }, |_, e| {
        let activation = has(e, "PowerBidScheduleTimePoint.activationCost");
        let shutdown = has(e, "PowerBidScheduleTimePoint.shutdownCost");
        let other = ["p", "minimumActivationP", "price", "reservePrice", "stepIncrementP"]
            .iter()
            .any(|a| has(e, &format!("PowerBidScheduleTimePoint.{a}")));
        (activation && shutdown) || ((activation || shutdown) && other)
    }, v);
}

/// `com:All-DanglingReferences` (FBOD4 for NC): a reference to a CIM object
/// the merged dataset does not hold. The SPARQL target is the missing object
/// itself; like the CGMES rule, this reports the NC element that refers to
/// it, with the field, so the finding can be acted on. Only NC elements'
/// references are read: CGMES elements have the CGMES rule.
///
/// A CIM identifier is what the SPARQL matches — `urn:uuid:…` or a `#_…`
/// fragment, which the decoder stores as `_…`.
fn dangling_references(ds: &CimDataset, v: &mut Vec<Violation>) {
    let rule = Rule {
        id: "com:All-DanglingReferences",
        name: "C:600:ALL:NA:FBOD4",
        property: "",
        message: "",
        severity: VIOLATION,
    };
    let mut nc: Vec<(&String, &Element)> =
        ds.entries.iter().filter(|(_, e)| e.type_name().starts_with("nc:")).collect();
    nc.sort_unstable_by_key(|(m, _)| m.as_str());
    for (mrid, e) in nc {
        let mut fields: Vec<(&&'static str, &FieldValue)> = e.fields().iter().collect();
        fields.sort_unstable_by_key(|(k, _)| **k);
        for (key, value) in fields {
            let targets: &[String] = match value {
                FieldValue::Resource(r) => std::slice::from_ref(r),
                FieldValue::ResourceList(rs) => rs,
                _ => continue,
            };
            for t in targets {
                let cim_id = t.starts_with("urn:uuid:") || (t.starts_with('_') && t.len() > 1);
                if cim_id && !ds.entries.contains_key(t.as_str()) {
                    let mut f = rule.with_message(mrid, e, format!("Dangling reference to '{t}'."));
                    f.property = key.to_string();
                    v.push(f);
                }
            }
        }
    }
}
