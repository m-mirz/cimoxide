use cimmodel::base::{FastMap as HashMap, FastSet as HashSet};
use cimmodel::CimDataset;
use crate::Violation;
use super::Fields;

/// The rules are independent, so they run on their own threads: those that
/// need the topology once it is built, the rest meanwhile. Results are joined
/// in the order listed.
pub fn validate(dataset: &CimDataset) -> Vec<Violation> {
    type Plain = fn(&CimDataset) -> Vec<Violation>;
    type OnTopology = fn(&CimDataset, &Topology) -> Vec<Violation>;
    const PLAIN: &[Plain] = &[
        check_dangling_references,
        check_sv_tap_step_position_sync,
        check_sv_shunt_compensator_sections_sync,
        check_regulating_control_contradictory,
    ];
    const ON_TOPOLOGY: &[OnTopology] = &[
        check_angle_reference,
        check_state_variables_instantiated,
        check_sv_shunt_compensator_sections_instance,
        check_sv_tap_step_instance,
        check_regulating_control_same_island,
    ];
    // Outside the scope, so the threads can borrow it.
    let built = std::sync::OnceLock::new();
    std::thread::scope(|s| {
        let plain: Vec<_> = PLAIN.iter().map(|f| s.spawn(move || f(dataset))).collect();
        let topo = built.get_or_init(|| Topology::build(dataset));
        let on_topology: Vec<_> = ON_TOPOLOGY.iter().map(|f| s.spawn(move || f(dataset, topo))).collect();
        on_topology
            .into_iter()
            .chain(plain)
            .flat_map(|h| h.join().expect("validation thread panicked"))
            .collect()
    })
}

/// Islands and terminals, built once per [`validate`] call.
///
/// Five rules mapped topological nodes to islands, and six scanned every
/// terminal into their own `String`-keyed maps; each did it from scratch.
/// Borrowed keys and one build serve them all. Maps fill in `by_type` order,
/// so a later entry wins exactly as it did when each rule built its own.
struct Topology<'a> {
    /// TopologicalNode → the TopologicalIsland listing it.
    tn_to_island: HashMap<&'a str, &'a str>,
    /// Terminal → its TopologicalNode.
    term_tn: HashMap<&'a str, &'a str>,
    /// ConductingEquipment → the TopologicalNodes of its terminals that have one.
    equip_tns: HashMap<&'a str, Vec<&'a str>>,
    /// ConductingEquipment → its terminals.
    equip_terms: HashMap<&'a str, Vec<&'a str>>,
}

impl<'a> Topology<'a> {
    fn build(dataset: &'a CimDataset) -> Self {
        let mut tn_to_island: HashMap<&'a str, &'a str> = HashMap::default();
        for mrid in dataset.by_type.get("TopologicalIsland").into_iter().flatten() {
            let entry = &dataset.entries[mrid];
            if let Some(island) = Fields::of_class(entry, "TopologicalIsland") {
                for tn in island.references("TopologicalIsland.TopologicalNodes") {
                    tn_to_island.insert(tn.trim_start_matches('#'), mrid);
                }
            }
        }
        let mut term_tn: HashMap<&'a str, &'a str> = HashMap::default();
        let mut equip_tns: HashMap<&'a str, Vec<&'a str>> = HashMap::default();
        let mut equip_terms: HashMap<&'a str, Vec<&'a str>> = HashMap::default();
        for mrid in dataset.by_type.get("Terminal").into_iter().flatten() {
            let entry = &dataset.entries[mrid];
            let Some(term) = Fields::of_class(entry, "Terminal") else { continue };
            let tn = term.reference("Terminal.TopologicalNode").map(|r| r.trim_start_matches('#'));
            if let Some(tn) = tn {
                term_tn.insert(mrid, tn);
            }
            if let Some(ce) = &term.reference("Terminal.ConductingEquipment") {
                let eq = ce.trim_start_matches('#');
                equip_terms.entry(eq).or_default().push(mrid);
                if let Some(tn) = tn {
                    equip_tns.entry(eq).or_default().push(tn);
                }
            }
        }
        Self { tn_to_island, term_tn, equip_tns, equip_terms }
    }

    /// Is any terminal of this equipment on a node in a topological island?
    fn energized(&self, eq: &str) -> bool {
        self.equip_tns.get(eq).is_some_and(|tns| tns.iter().any(|tn| self.tn_to_island.contains_key(tn)))
    }
}

fn check_angle_reference(dataset: &CimDataset, topo: &Topology) -> Vec<Violation> {
    let mut angle_ref_tns: HashSet<String> = HashSet::default();
    for mrid in dataset.by_type.get("TopologicalIsland").into_iter().flatten() {
        let entry = &dataset.entries[mrid];
        if let Some(island) = Fields::of_class(entry, "TopologicalIsland")
            && let Some(r) = &island.reference("TopologicalIsland.AngleRefTopologicalNode") {
                angle_ref_tns.insert(r.trim_start_matches('#').to_string());
            }
    }

    // Find SMs with highest referencePriority (> 0)
    let mut min_priority = i64::MAX;
    let mut highest_prio_sms: Vec<String> = Vec::new();
    for mrid in dataset.by_type.get("SynchronousMachine").into_iter().flatten() {
        let entry = &dataset.entries[mrid];
        if let Some(sm) = Fields::of_class(entry, "SynchronousMachine") {
            let prio = sm.i64("SynchronousMachine.referencePriority").unwrap_or(0);
            if prio <= 0 { continue; }
            if prio < min_priority {
                min_priority = prio;
                highest_prio_sms = vec![mrid.clone()];
            } else if prio == min_priority {
                highest_prio_sms.push(mrid.clone());
            }
        }
    }

    if highest_prio_sms.is_empty() { return Vec::new(); }

    let mut v = Vec::new();
    if highest_prio_sms.len() > 1 {
        v.push(Violation {
            object_id:   "global".into(),
            rule_id:     "sm456:Model-angleReference".into(),
            name:        "C:456:SSH:NA:angleReference".into(),
            class:       "SynchronousMachine".into(),
            property:    "referencePriority".into(),
            message:     "Multiple machines with highest SynchronousMachine.referencePriority found.".into(),
            severity:    "sh:Violation".into(),
            description: String::new(),
        });
    }

    for sm_id in &highest_prio_sms {
        let found = topo.equip_tns.get(sm_id.as_str()).is_some_and(|tns| tns.iter().any(|tn| angle_ref_tns.contains(*tn)));
        if !found {
            v.push(Violation {
                object_id:   sm_id.clone(),
                rule_id:     "sm456:Model-angleReference".into(),
                name:        "C:456:SSH:NA:angleReference".into(),
                class:       "SynchronousMachine".into(),
                property:    "referencePriority".into(),
                message:     "The SynchronousMachine with highest priority is not connected to a TopologicalIsland.AngleRefTopologicalNode.".into(),
                severity:    "sh:Violation".into(),
                description: String::new(),
            });
        }
    }
    v
}

/// The equipment whose `SvStatus.inService` is `true` — the
/// `^cim:SvStatus.ConductingEquipment/cim:SvStatus.inService true` the SV rules
/// start from.
fn sv_in_service(dataset: &CimDataset) -> HashSet<&str> {
    dataset
        .by_type
        .get("SvStatus")
        .into_iter()
        .flatten()
        .filter_map(|m| Fields::of_class(&dataset.entries[m], "SvStatus"))
        .filter(|svs| svs.bool("SvStatus.inService") == Some(true))
        .filter_map(|svs| svs.reference("SvStatus.ConductingEquipment"))
        .map(|r| r.trim_start_matches('#'))
        .collect()
}

/// The PowerTransformer a tap changer's end belongs to. The SPARQL steps
/// `cim:TransformerEnd.PowerTransformer`, which CGMES 3.0 does not define, so as
/// written it never matches; the attribute is `PowerTransformerEnd.PowerTransformer`.
fn tap_changer_transformer<'a>(dataset: &'a CimDataset, tc: &'a cimmodel::Element) -> Option<&'a str> {
    let f = Fields::of(tc);
    let end = f.reference("RatioTapChanger.TransformerEnd").or_else(|| f.reference("PhaseTapChanger.TransformerEnd"))?;
    let end = dataset.entries.get(end.trim_start_matches('#'))?;
    Fields::of(end).reference("PowerTransformerEnd.PowerTransformer").map(|r| r.trim_start_matches('#'))
}

const TAP_CHANGERS: [&str; 5] = ["RatioTapChanger", "PhaseTapChangerLinear", "PhaseTapChangerSymmetrical",
    "PhaseTapChangerAsymmetrical", "PhaseTapChangerTabular"];

/// A reference that names a CIM object this dataset does not hold.
///
/// The SPARQL matches `urn:uuid:…` and IRIs with a `#_…` fragment. The decoder
/// stores a reference after its last `#`, so the latter arrive as `_…`; testing
/// the stored value for `#_` missed every one of them, and FBOD4 caught only
/// `urn:uuid:` references.
fn is_dangling(dataset: &CimDataset, target: &str) -> bool {
    let is_cim_id = target.starts_with("urn:uuid:") || (target.starts_with('_') && target.len() > 1);
    is_cim_id && !dataset.entries.contains_key(target)
}

fn check_dangling_references(dataset: &CimDataset) -> Vec<Violation> {
    // A walk over every CGMES element, split into runs on their own threads.
    // NC elements have their own rule (`com:All-DanglingReferences`, in
    // `sparql::nc`), which the NC pass runs on the same merged dataset.
    let all: Vec<(&String, &cimmodel::Element)> =
        dataset.entries.iter().filter(|(_, e)| !e.type_name().starts_with("nc:")).collect();
    let threads = crate::par::threads_for(all.len());
    crate::par::par_concat(&crate::par::runs(&all, threads, |_| 1), |run| dangling_in(dataset, run))
}

fn dangling_in(dataset: &CimDataset, run: &[(&String, &cimmodel::Element)]) -> Vec<Violation> {
    use cimmodel::base::FieldValue;
    let mut v = Vec::new();
    for &(id, entry) in run {
        let mut fields: Vec<(&&'static str, &[String])> = entry
            .fields()
            .iter()
            .filter_map(|(k, val)| match val {
                FieldValue::Resource(r) => Some((k, std::slice::from_ref(r))),
                FieldValue::ResourceList(rs) => Some((k, rs.as_slice())),
                _ => None,
            })
            .filter(|(_, refs)| refs.iter().any(|r| is_dangling(dataset, r)))
            .collect();
        fields.sort_unstable_by_key(|(k, _)| **k);
        for (field, refs) in fields {
            for target in refs.iter().filter(|r| is_dangling(dataset, r)) {
                v.push(Violation {
                    object_id:   id.clone(),
                    rule_id:     "sm600:All-DanglingReferences".into(),
                    name:        "C:600:ALL:NA:FBOD4".into(),
                    class:       entry.type_name().to_string(),
                    property:    field.to_string(),
                    message:     format!("Dangling reference to '{}'.", target),
                    severity:    "sh:Violation".into(),
                    description: String::new(),
                });
            }
        }
    }
    v
}

fn check_state_variables_instantiated(dataset: &CimDataset, topo: &Topology) -> Vec<Violation> {
    let tn_to_island = &topo.tn_to_island;
    let mut v = Vec::new();

    // 1. SvVoltage for each TN in island
    let mut tn_has_sv_voltage: HashSet<String> = HashSet::default();
    for mrid in dataset.by_type.get("SvVoltage").into_iter().flatten() {
        let entry = &dataset.entries[mrid];
        if let Some(svv) = Fields::of_class(entry, "SvVoltage")
            && let Some(r) = &svv.reference("SvVoltage.TopologicalNode") {
                tn_has_sv_voltage.insert(r.trim_start_matches('#').to_string());
            }
    }
    for (tn_id, island_id) in tn_to_island {
        if !tn_has_sv_voltage.contains(*tn_id) {
            v.push(Violation {
                object_id:   tn_id.to_string(),
                rule_id:     "sm600:SvVoltage-SV__4".into(),
                name:        "C:600:SV:SvVoltage:SV__4".into(),
                class:       "TopologicalNode".into(),
                property:    "rdf:type".into(),
                message:     format!("SvVoltage is not instantiated for energized TopologicalNode part of island {}.", island_id),
                severity:    "sh:Violation".into(),
                description: String::new(),
            });
        }
    }

    let equip_tns = &topo.equip_tns;

    // 2. SvSwitch for energized retained switches
    let mut sw_has_sv_switch: HashSet<String> = HashSet::default();
    for mrid in dataset.by_type.get("SvSwitch").into_iter().flatten() {
        let entry = &dataset.entries[mrid];
        if let Some(svsw) = Fields::of_class(entry, "SvSwitch")
            && let Some(r) = &svsw.reference("SvSwitch.Switch") {
                sw_has_sv_switch.insert(r.trim_start_matches('#').to_string());
            }
    }
    // Every switch class the shape targets, in service as the SV states it.
    let in_service = sv_in_service(dataset);
    const SWITCHES: [&str; 9] = ["Switch", "Fuse", "GroundDisconnector", "Jumper", "Breaker",
        "DisconnectingCircuitBreaker", "LoadBreakSwitch", "Disconnector", "Cut"];
    for class in SWITCHES {
    for mrid in dataset.by_type.get(class).into_iter().flatten() {
        let entry = &dataset.entries[mrid];
        if let Some(sw) = Fields::of_class(entry, class) {
            if sw.bool("Switch.retained") != Some(true) { continue; }
            if !in_service.contains(mrid.as_str()) { continue; }
            if !topo.energized(mrid) { continue; }
            if !sw_has_sv_switch.contains(mrid) {
                v.push(Violation {
                    object_id:   mrid.clone(),
                    rule_id:     "sm600:SvSwitch-SV__4".into(),
                    name:        "C:600:SV:SvSwitch:SV__4".into(),
                    class:       class.into(),
                    property:    "rdf:type".into(),
                    message:     "SvSwitch not instantiated for energized retained Switch.".into(),
                    severity:    "sh:Violation".into(),
                    description: String::new(),
                });
            }
        }
    }
    }

    // 3. SvStatus for all energized ConductingEquipment
    let mut ce_has_sv_status: HashSet<String> = HashSet::default();
    for mrid in dataset.by_type.get("SvStatus").into_iter().flatten() {
        let entry = &dataset.entries[mrid];
        if let Some(svs) = Fields::of_class(entry, "SvStatus")
            && let Some(r) = &svs.reference("SvStatus.ConductingEquipment") {
                ce_has_sv_status.insert(r.trim_start_matches('#').to_string());
            }
    }
    for (eq_id, tns) in equip_tns {
        let energized = tns.iter().any(|tn| tn_to_island.contains_key(tn));
        if !energized { continue; }
        if !ce_has_sv_status.contains(*eq_id) {
            let type_name = dataset.entries.get(*eq_id).map_or("ConductingEquipment", |e| e.type_name());
            v.push(Violation {
                object_id:   eq_id.to_string(),
                rule_id:     "sm600:SvStatus-SV__4".into(),
                name:        "C:600:SV:SvStatus:SV__4".into(),
                class:       type_name.to_string(),
                property:    "rdf:type".into(),
                message:     "SvStatus is not instantiated for energized ConductingEquipment.".into(),
                severity:    "sh:Violation".into(),
                description: String::new(),
            });
        }
    }
    v
}

/// `C:452:EQ:RegulatingControl:samePoint`: on one TopologicalNode, the enabled
/// controls of one mode must agree on their target. Every control on the node
/// with a target value is reported, as the SPARQL's `?rcfail`.
fn check_regulating_control_contradictory(dataset: &CimDataset) -> Vec<Violation> {
    let node_of = |rc: &Fields| -> Option<&str> {
        let term = rc.reference("RegulatingControl.Terminal")?.trim_start_matches('#');
        let t = Fields::get(dataset, term, "Terminal")?;
        t.reference("Terminal.TopologicalNode").map(|r| r.trim_start_matches('#'))
    };
    // TopologicalNode → its controls with a target value; and, per (node, mode),
    // the targets of the enabled ones.
    let mut on_node: HashMap<&str, Vec<&String>> = HashMap::default();
    let mut targets: HashMap<(&str, &str), Vec<f64>> = HashMap::default();
    for mrid in dataset.by_type.get("RegulatingControl").into_iter().flatten() {
        let Some(rc) = Fields::of_class(&dataset.entries[mrid], "RegulatingControl") else { continue };
        let Some(target) = rc.f64("RegulatingControl.targetValue") else { continue };
        let Some(node) = node_of(&rc) else { continue };
        on_node.entry(node).or_default().push(mrid);
        if rc.bool("RegulatingControl.enabled") == Some(true)
            && let Some(mode) = rc.enumeration("RegulatingControl.mode") {
                targets.entry((node, mode)).or_default().push(target);
            }
    }
    let mut failing: Vec<&str> = targets
        .iter()
        .filter(|(_, t)| t.iter().any(|x| *x != t[0]))
        .map(|((node, _), _)| *node)
        .collect();
    failing.sort_unstable();
    failing.dedup();
    let mut v = Vec::new();
    for node in failing {
        let mut rcs = on_node[node].clone();
        rcs.sort_unstable();
        for rc_id in rcs {
            v.push(Violation {
                object_id:   rc_id.clone(),
                rule_id:     "sm6002:RegulatingControl-samePoint".into(),
                name:        "C:452:EQ:RegulatingControl:samePoint".into(),
                class:       "RegulatingControl".into(),
                property:    "RegulatingControl.targetValue".into(),
                message:     format!("Enabled RegulatingControl-s of the same type associated with the same TopologicalNode ({node}) have different target values."),
                severity:    "sh:Violation".into(),
                description: String::new(),
            });
        }
    }
    v
}

/// The SV's section count must match the SSH's for a shunt compensator that
/// does not regulate: its `controlEnabled` false, or its control disabled. The
/// SPARQL binds the SV status, `controlEnabled` and both counts as required
/// patterns; the control's `enabled` only when given.
fn check_sv_shunt_compensator_sections_sync(dataset: &CimDataset) -> Vec<Violation> {
    let in_service = sv_in_service(dataset);
    let mut v = Vec::new();
    for mrid in dataset.by_type.get("SvShuntCompensatorSections").into_iter().flatten() {
        let entry = &dataset.entries[mrid];
        let svsc = match Fields::of_class(entry, "SvShuntCompensatorSections") { Some(s) => s, None => continue };
        let sc_id = match svsc.reference("SvShuntCompensatorSections.ShuntCompensator") { Some(r) => r.trim_start_matches('#'), None => continue };
        let Some(sv_sections) = svsc.f64("SvShuntCompensatorSections.sections") else { continue };
        if !in_service.contains(sc_id) { continue; }
        let Some(sc_entry) = dataset.entries.get(sc_id) else { continue };
        let type_name = match sc_entry.type_name() {
            t @ ("LinearShuntCompensator" | "NonlinearShuntCompensator") => t,
            _ => continue,
        };
        let sc = Fields::of(sc_entry);
        let (Some(control_enabled), Some(sections)) =
            (sc.bool("RegulatingCondEq.controlEnabled"), sc.f64("ShuntCompensator.sections")) else { continue };
        let rc_disabled = sc
            .reference("RegulatingCondEq.RegulatingControl")
            .and_then(|id| dataset.entries.get(id.trim_start_matches('#')))
            .and_then(|e| Fields::of(e).bool("RegulatingControl.enabled"))
            == Some(false);
        if (!control_enabled || rc_disabled) && sv_sections != sections {
            v.push(Violation {
                object_id:   sc_id.to_string(),
                rule_id:     "sm600:SvShuntCompensatorSections.sections-SV__4".into(),
                name:        "C:600:SV:SvShuntCompensatorSections.sections:SV__4".into(),
                class:       type_name.to_string(),
                property:    "ShuntCompensator.sections".into(),
                message:     format!("SvShuntCompensatorSections.sections ({}) is not the same as ShuntCompensator.sections ({}) for non-regulating ShuntCompensator.", sv_sections, sections),
                severity:    "sh:Violation".into(),
                description: String::new(),
            });
        }
    }
    v
}

/// As [`check_sv_shunt_compensator_sections_sync`], for a tap changer's step,
/// in service through its transformer.
fn check_sv_tap_step_position_sync(dataset: &CimDataset) -> Vec<Violation> {
    let in_service = sv_in_service(dataset);
    let mut v = Vec::new();
    for mrid in dataset.by_type.get("SvTapStep").into_iter().flatten() {
        let entry = &dataset.entries[mrid];
        let svts = match Fields::of_class(entry, "SvTapStep") { Some(s) => s, None => continue };
        let tc_id = match svts.reference("SvTapStep.TapChanger") { Some(r) => r.trim_start_matches('#'), None => continue };
        let Some(position) = svts.f64("SvTapStep.position") else { continue };
        let Some(tc_entry) = dataset.entries.get(tc_id) else { continue };
        let Some(type_name) = TAP_CHANGERS.into_iter().find(|t| *t == tc_entry.type_name()) else { continue };
        if !tap_changer_transformer(dataset, tc_entry).is_some_and(|pt| in_service.contains(pt)) { continue; }
        let tc = Fields::of(tc_entry);
        let (Some(control_enabled), Some(step)) = (tc.bool("TapChanger.controlEnabled"), tc.f64("TapChanger.step")) else { continue };
        let rc_disabled = tc
            .reference("TapChanger.TapChangerControl")
            .and_then(|id| dataset.entries.get(id.trim_start_matches('#')))
            .and_then(|e| Fields::of(e).bool("RegulatingControl.enabled"))
            == Some(false);
        if (!control_enabled || rc_disabled) && position != step {
            v.push(Violation {
                object_id:   tc_id.to_string(),
                rule_id:     "sm600:SvTapStep.position-SV__4".into(),
                name:        "C:600:SV:SvTapStep.position:SV__4".into(),
                class:       type_name.to_string(),
                property:    "TapChanger.step".into(),
                message:     format!("SvTapStep.position ({}) is not the same as TapChanger.step ({}) for non-regulating TapChanger.", position, step),
                severity:    "sh:Violation".into(),
                description: String::new(),
            });
        }
    }
    v
}

fn check_sv_shunt_compensator_sections_instance(dataset: &CimDataset, topo: &Topology) -> Vec<Violation> {
    let mut sc_has_sv: HashSet<String> = HashSet::default();
    for mrid in dataset.by_type.get("SvShuntCompensatorSections").into_iter().flatten() {
        let entry = &dataset.entries[mrid];
        if let Some(svsc) = Fields::of_class(entry, "SvShuntCompensatorSections")
            && let Some(r) = &svsc.reference("SvShuntCompensatorSections.ShuntCompensator") {
                sc_has_sv.insert(r.trim_start_matches('#').to_string());
            }
    }
    let in_service = sv_in_service(dataset);
    let mut v = Vec::new();
    for type_name in &["LinearShuntCompensator", "NonlinearShuntCompensator"] {
        for mrid in dataset.by_type.get(*type_name).into_iter().flatten() {
            if !in_service.contains(mrid.as_str()) || !topo.energized(mrid) { continue; }
            if !sc_has_sv.contains(mrid) {
                v.push(Violation {
                    object_id:   mrid.clone(),
                    rule_id:     "sm600:SvShuntCompensatorSections-SV__4".into(),
                    name:        "C:600:SV:SvShuntCompensatorSections:SV__4".into(),
                    class:       type_name.to_string(),
                    property:    "rdf:type".into(),
                    message:     "SvShuntCompensatorSections is not instantiated for an energized ShuntCompensator.".into(),
                    severity:    "sh:Violation".into(),
                    description: String::new(),
                });
            }
        }
    }
    v
}

fn check_sv_tap_step_instance(dataset: &CimDataset, topo: &Topology) -> Vec<Violation> {

    // TapChanger → energized via TransformerEnd → Terminal → TN
    let mut te_terminal: HashMap<String, String> = HashMap::default();
    for type_name in &["PowerTransformerEnd"] {
        for mrid in dataset.by_type.get(*type_name).into_iter().flatten() {
            let entry = &dataset.entries[mrid];
            if let Some(pte) = Fields::of_class(entry, "PowerTransformerEnd")
                && let Some(r) = &pte.reference("TransformerEnd.Terminal") {
                    te_terminal.insert(mrid.clone(), r.trim_start_matches('#').to_string());
                }
        }
    }
    let is_tc_energized = |te_id: &str| -> bool {
        let term_id = match te_terminal.get(te_id) { Some(t) => t, None => return false };
        let tn_id = match topo.term_tn.get(term_id.as_str()) { Some(t) => t, None => return false };
        topo.tn_to_island.contains_key(tn_id)
    };

    let mut tc_has_sv: HashSet<String> = HashSet::default();
    for mrid in dataset.by_type.get("SvTapStep").into_iter().flatten() {
        let entry = &dataset.entries[mrid];
        if let Some(svts) = Fields::of_class(entry, "SvTapStep")
            && let Some(r) = &svts.reference("SvTapStep.TapChanger") {
                tc_has_sv.insert(r.trim_start_matches('#').to_string());
            }
    }

    let in_service = sv_in_service(dataset);
    let mut v = Vec::new();
    let tc_types = ["RatioTapChanger", "PhaseTapChangerLinear", "PhaseTapChangerSymmetrical",
                    "PhaseTapChangerAsymmetrical", "PhaseTapChangerTabular"];
    for type_name in &tc_types {
        for mrid in dataset.by_type.get(*type_name).into_iter().flatten() {
            // Get TransformerEnd from this tap changer
            let te_id = dataset.entries.get(mrid).and_then(|e| {
                if let Some(tc) = Fields::of_class(e, "RatioTapChanger") {
                    tc.reference("RatioTapChanger.TransformerEnd").map(|r| r.trim_start_matches('#').to_string())
                } else if let Some(tc) = Fields::of_class(e, "PhaseTapChangerLinear") {
                    tc.reference("PhaseTapChanger.TransformerEnd").map(|r| r.trim_start_matches('#').to_string())
                } else if let Some(tc) = Fields::of_class(e, "PhaseTapChangerSymmetrical") {
                    tc.reference("PhaseTapChanger.TransformerEnd").map(|r| r.trim_start_matches('#').to_string())
                } else if let Some(tc) = Fields::of_class(e, "PhaseTapChangerAsymmetrical") {
                    tc.reference("PhaseTapChanger.TransformerEnd").map(|r| r.trim_start_matches('#').to_string())
                } else if let Some(tc) = Fields::of_class(e, "PhaseTapChangerTabular") {
                    tc.reference("PhaseTapChanger.TransformerEnd").map(|r| r.trim_start_matches('#').to_string())
                } else {
                    None
                }
            });
            let energized = te_id.as_deref().is_some_and(is_tc_energized);
            if !energized { continue; }
            let tc_entry = &dataset.entries[mrid];
            if !tap_changer_transformer(dataset, tc_entry).is_some_and(|pt| in_service.contains(pt)) { continue; }
            if !tc_has_sv.contains(mrid) {
                v.push(Violation {
                    object_id:   mrid.clone(),
                    rule_id:     "sm600:SvTapStep-SV__4".into(),
                    name:        "C:600:SV:SvTapStep:SV__4".into(),
                    class:       type_name.to_string(),
                    property:    "rdf:type".into(),
                    message:     "SvTapStep is not instantiated for an energized TapChanger.".into(),
                    severity:    "sh:Violation".into(),
                    description: String::new(),
                });
            }
        }
    }
    v
}

fn check_regulating_control_same_island(dataset: &CimDataset, topo: &Topology) -> Vec<Violation> {
    // Terminal → island
    let term_to_island = |term: &str| topo.term_tn.get(term).and_then(|tn| topo.tn_to_island.get(tn)).copied();

    // RegulatingControl MRID → SynchronousMachines referencing it. Built once instead of
    // rescanning all SynchronousMachine per RegulatingControl below.
    let mut rc_to_sm: HashMap<String, Vec<String>> = HashMap::default();
    for sm_mrid in dataset.by_type.get("SynchronousMachine").into_iter().flatten() {
        let sm_entry = &dataset.entries[sm_mrid];
        if let Some(sm) = Fields::of_class(sm_entry, "SynchronousMachine")
            && let Some(rc_ref) = sm.reference("RegulatingCondEq.RegulatingControl") {
                let rc_id = rc_ref.trim_start_matches('#').to_string();
                rc_to_sm.entry(rc_id).or_default().push(sm_mrid.clone());
            }
    }

    // TapChangerControl (RegulatingControl) MRID → (tap changer MRID, type name, transformer
    // end MRID). Built once instead of rescanning all 5 TapChanger types per RegulatingControl
    // below.
    let tc_types = ["RatioTapChanger", "PhaseTapChangerLinear", "PhaseTapChangerSymmetrical",
                    "PhaseTapChangerAsymmetrical", "PhaseTapChangerTabular"];
    // (tap changer MRID, its type, its TransformerEnd MRID)
    type TapChangers = Vec<(String, &'static str, Option<String>)>;
    let mut tcc_to_tc: HashMap<String, TapChangers> = HashMap::default();
    for tc_type in &tc_types {
        for tc_mrid in dataset.by_type.get(*tc_type).into_iter().flatten() {
            let tc_entry = &dataset.entries[tc_mrid];
            let (tcc_ref, te_id) = if let Some(tc) = Fields::of_class(tc_entry, "RatioTapChanger") {
                (tc.reference("TapChanger.TapChangerControl").map(|r| r.trim_start_matches('#').to_string()),
                 tc.reference("RatioTapChanger.TransformerEnd").map(|r| r.trim_start_matches('#').to_string()))
            } else if let Some(tc) = Fields::of_class(tc_entry, "PhaseTapChangerLinear") {
                (tc.reference("TapChanger.TapChangerControl").map(|r| r.trim_start_matches('#').to_string()),
                 tc.reference("PhaseTapChanger.TransformerEnd").map(|r| r.trim_start_matches('#').to_string()))
            } else if let Some(tc) = Fields::of_class(tc_entry, "PhaseTapChangerSymmetrical") {
                (tc.reference("TapChanger.TapChangerControl").map(|r| r.trim_start_matches('#').to_string()),
                 tc.reference("PhaseTapChanger.TransformerEnd").map(|r| r.trim_start_matches('#').to_string()))
            } else if let Some(tc) = Fields::of_class(tc_entry, "PhaseTapChangerAsymmetrical") {
                (tc.reference("TapChanger.TapChangerControl").map(|r| r.trim_start_matches('#').to_string()),
                 tc.reference("PhaseTapChanger.TransformerEnd").map(|r| r.trim_start_matches('#').to_string()))
            } else if let Some(tc) = Fields::of_class(tc_entry, "PhaseTapChangerTabular") {
                (tc.reference("TapChanger.TapChangerControl").map(|r| r.trim_start_matches('#').to_string()),
                 tc.reference("PhaseTapChanger.TransformerEnd").map(|r| r.trim_start_matches('#').to_string()))
            } else {
                continue
            };
            if let Some(tcc_id) = tcc_ref {
                tcc_to_tc.entry(tcc_id).or_default().push((tc_mrid.clone(), *tc_type, te_id));
            }
        }
    }

    let mut v = Vec::new();
    for mrid in dataset.by_type.get("RegulatingControl").into_iter().flatten() {
        let entry = &dataset.entries[mrid];
        let rc = match Fields::of_class(entry, "RegulatingControl") { Some(r) => r, None => continue };
        if !rc.bool("RegulatingControl.enabled").unwrap_or(false) { continue; }
        let rc_term_id = match rc.reference("RegulatingControl.Terminal") { Some(r) => r.trim_start_matches('#'), None => continue };
        let rc_island = match term_to_island(rc_term_id) { Some(i) => i, None => continue };

        // Check SynchronousMachines referencing this RC
        for sm_mrid in rc_to_sm.get(mrid).into_iter().flatten() {
            for term_id in topo.equip_terms.get(sm_mrid.as_str()).into_iter().flatten() {
                if let Some(sm_island) = term_to_island(term_id)
                    && sm_island != rc_island {
                        v.push(Violation {
                            object_id:   mrid.clone(),
                            rule_id:     "sm6002:RegulatingControl-point".into(),
                            name:        "C:600:EQ:RegulatingControl:point".into(),
                            class:       "RegulatingControl".into(),
                            property:    "rdf:type".into(),
                            message:     format!("The controlled point and the controlling equipment (SynchronousMachine {}) are not located in the same TopologicalIsland.", sm_mrid),
                            severity:    "sh:Violation".into(),
                            description: String::new(),
                        });
                        break;
                    }
            }
        }

        // Check TapChangers referencing this RC
        for (tc_mrid, tc_type, te_id) in tcc_to_tc.get(mrid).into_iter().flatten() {
            let te_entry = match te_id.as_deref().and_then(|id| dataset.entries.get(id)) { Some(e) => e, None => continue };
            let term_id = if let Some(pte) = Fields::of_class(te_entry, "PowerTransformerEnd") {
                pte.reference("TransformerEnd.Terminal").map(|r| r.trim_start_matches('#').to_string())
            } else {
                None
            };
            if let Some(t_id) = term_id
                && let Some(tc_island) = term_to_island(&t_id)
                    && tc_island != rc_island {
                        v.push(Violation {
                            object_id:   mrid.clone(),
                            rule_id:     "sm6002:RegulatingControl-point".into(),
                            name:        "C:600:EQ:RegulatingControl:point".into(),
                            class:       "RegulatingControl".into(),
                            property:    "rdf:type".into(),
                            message:     format!("The controlled point and the controlling equipment ({} {}) are not located in the same TopologicalIsland.", tc_type, tc_mrid),
                            severity:    "sh:Violation".into(),
                            description: String::new(),
                        });
                    }
        }
    }
    v
}
