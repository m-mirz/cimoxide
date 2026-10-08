//! NCP's `sh:sparql` rules, written by hand in `sparql::nc`.
//!
//! The test configurations exercise few of them — relicapgrid's data is clean
//! for all but ClassCount and the dangling references — so each rule gets a
//! dataset that breaks it and the same dataset fixed: a test that only checks
//! for silence cannot tell a working rule from one that never runs.

use cimmodel::CimDataset;
use cimvalidation::sparql::nc::{validate_local, validate_merged};
use cimvalidation::Violation;

fn decode(body: &str) -> CimDataset {
    let xml = format!(
        r#"<?xml version="1.0" encoding="UTF-8"?>
<rdf:RDF xmlns:rdf="http://www.w3.org/1999/02/22-rdf-syntax-ns#"
         xmlns:cim="https://cim.ucaiug.io/ns#"
         xmlns:cim17="http://iec.ch/TC57/CIM100#"
         xmlns:nc="https://cim4.eu/ns/nc#"
         xmlns:dcat="http://www.w3.org/ns/dcat#"
         xmlns:dcatcim="https://cim4.eu/ns/dcatcim#"
         xmlns:dcterms="http://purl.org/dc/terms/"
         xmlns:adms="http://www.w3.org/ns/adms#">
{body}
</rdf:RDF>"#
    );
    CimDataset::decode_str(&xml).expect("fixture did not decode")
}

/// The ids of the elements `rule` reports, sorted.
fn reported(v: &[Violation], rule: &str) -> Vec<String> {
    let mut ids: Vec<String> = v.iter().filter(|x| x.rule_id == rule).map(|x| x.object_id.clone()).collect();
    ids.sort();
    ids
}

fn merged(body: &str, rule: &str) -> Vec<String> {
    reported(&validate_merged(&decode(body)), rule)
}

fn local(body: &str, rule: &str) -> Vec<String> {
    reported(&validate_local(&decode(body)), rule)
}

// ── per dataset ────────────────────────────────────────────────────────────

const HEADER_OPEN: &str = r#"<dcat:Dataset rdf:about="urn:uuid:00000000-0000-0000-0000-000000000001">"#;

#[test]
fn class_count_reports_each_class_of_the_dataset() {
    let v = validate_local(&decode(&format!(
        "{HEADER_OPEN}</dcat:Dataset>
  <nc:BoundaryPoint rdf:ID=\"_bp1\"/><nc:BoundaryPoint rdf:ID=\"_bp2\"/>"
    )));
    let messages: Vec<&str> =
        v.iter().filter(|x| x.rule_id == "cc:ClassCount-property").map(|x| x.message.as_str()).collect();
    assert_eq!(messages, [
        "The class nc:BoundaryPoint appears 2 times in the data graph.",
        "The class nc:Dataset appears 1 times in the data graph.",
    ]);
    assert!(v.iter().filter(|x| x.rule_id == "cc:ClassCount-property").all(|x| x.severity == "sh:Info"));
}

#[test]
fn conforms_to_is_required_outside_the_boundary_model() {
    let rule = "dm:conformsTo-NC-cardinality";
    let with = |spatial: &str, conforms: &str| {
        local(&format!(
            "{HEADER_OPEN}<dcterms:spatial rdf:resource=\"{spatial}\"/>{conforms}</dcat:Dataset>"
        ), rule)
    };
    let frame = "https://energy.referencedata.eu/Test/Frame/X";
    assert_eq!(with(frame, ""), ["urn:uuid:00000000-0000-0000-0000-000000000001"]);
    assert!(with(frame, "<dcterms:conformsTo rdf:resource=\"https://ap.cim4.eu/Contingency/2.5\"/>").is_empty());
    assert!(with("https://energy.referencedata.eu/Frame/BoundaryModel", "").is_empty());
}

#[test]
fn description_and_version_notes_need_an_english_language_tag() {
    let with = |attrs: &str| {
        let v = validate_local(&decode(&format!(
            "{HEADER_OPEN}<dcterms:description{attrs}>d</dcterms:description><adms:versionNotes{attrs}>n</adms:versionNotes></dcat:Dataset>"
        )));
        (reported(&v, "dm:description.langTag-presence").len(), reported(&v, "dm:versionNotes.langTag-presence").len())
    };
    assert_eq!(with(r#" xml:lang="en""#), (0, 0));
    assert_eq!(with(""), (1, 1), "an untagged literal has lang \"\"");
    assert_eq!(with(r#" xml:lang="de""#), (1, 1));
}

#[test]
fn alternative_and_preferred_version_come_together() {
    let rule = "dm:alternativeVersionAndPreferredVersion-dependency";
    let alt = r#"<dcatcim:alternativeVersionOf rdf:resource="urn:uuid:00000000-0000-0000-0000-000000000002"/>"#;
    let pref = r#"<dcatcim:preferredVersion rdf:resource="urn:uuid:00000000-0000-0000-0000-000000000003"/>"#;
    let with = |body: &str| local(&format!("{HEADER_OPEN}{body}</dcat:Dataset>"), rule).len();
    assert_eq!(with(alt), 1);
    assert_eq!(with(pref), 1);
    assert_eq!(with(&format!("{alt}{pref}")), 0);
    assert_eq!(with(""), 0);
}

// ── merged ─────────────────────────────────────────────────────────────────

#[test]
fn a_base_case_current_limit_needs_an_assessed_element_in_the_base_case() {
    let rule = "aec:OperationalLimit.AssessedElement-required";
    let with = |in_base_case: &str| merged(&format!(
        r#"<nc:BaseCaseCurrentLimit rdf:ID="_lim"/>
  <nc:AssessedElement rdf:ID="_ae">
    <nc:AssessedElement.OperationalLimit rdf:resource='#_lim'/>
    <nc:AssessedElement.inBaseCase>{in_base_case}</nc:AssessedElement.inBaseCase>
  </nc:AssessedElement>"#
    ), rule);
    assert!(with("true").is_empty());
    assert_eq!(with("false"), ["_lim"]);
    assert_eq!(merged(r#"<nc:BaseCaseCurrentLimit rdf:ID="_lim"/>"#, rule), ["_lim"]);
}

#[test]
fn an_exclusive_dependency_carries_no_lag_or_overlap() {
    for (class, rule) in [
        ("OutageDependency", "asc:PeerTemporalDependency.constraintKind-exclusive"),
        ("ThermalGeneratingUnitDependency", "erc:PeerTemporalDependency.constraintKind-exclusive"),
        ("PowerBidDependency", "sisc:PeerTemporalDependency.constraintKind-exclusive"),
    ] {
        let with = |kind: &str, extra: &str| merged(&format!(
            r#"<nc:{class} rdf:ID="_d">
    <nc:PeerTemporalDependency.constraintKind rdf:resource="https://cim4.eu/ns/nc#DependencyConstraintKind.{kind}"/>
    {extra}
  </nc:{class}>"#
        ), rule);
        let lag = "<nc:PeerTemporalDependency.startToStartLag>PT1H</nc:PeerTemporalDependency.startToStartLag>";
        assert_eq!(with("exclusive", lag), ["_d"], "{class}");
        assert!(with("exclusive", "").is_empty(), "{class}");
        assert!(with("finishToStart", lag).is_empty(), "{class}");
    }
}

#[test]
fn a_hydro_plant_runs_its_machines_in_one_mode() {
    let rule = "erc:HydroPowerPlant-operatingMode";
    let machine = |id: &str, unit: &str, mode: &str| format!(
        r#"<cim17:SynchronousMachine rdf:ID="{id}">
    <cim17:RotatingMachine.GeneratingUnit rdf:resource='#{unit}'/>
    <cim17:SynchronousMachine.operatingMode rdf:resource="http://iec.ch/TC57/CIM100#SynchronousMachineOperatingMode.{mode}"/>
  </cim17:SynchronousMachine>"#
    );
    let plant = r#"<cim17:HydroPowerPlant rdf:ID="_hpp"/>
  <cim17:HydroGeneratingUnit rdf:ID="_u1"><cim17:HydroGeneratingUnit.HydroPowerPlant rdf:resource='#_hpp'/></cim17:HydroGeneratingUnit>
  <cim17:HydroGeneratingUnit rdf:ID="_u2"><cim17:HydroGeneratingUnit.HydroPowerPlant rdf:resource='#_hpp'/></cim17:HydroGeneratingUnit>"#;
    let same = format!("{plant}{}{}", machine("_m1", "_u1", "generator"), machine("_m2", "_u2", "generator"));
    let mixed = format!("{plant}{}{}", machine("_m1", "_u1", "generator"), machine("_m2", "_u2", "condenser"));
    assert!(merged(&same, rule).is_empty());
    assert_eq!(merged(&mixed, rule), ["_hpp"]);
}

#[test]
fn a_cross_zonal_element_joins_two_zones_in_opposite_directions() {
    let rule = "erc:CrossZonalNetworkElement.BiddingZoneBorder-usage";
    let border = |id: &str, from: &str, to: &str| format!(
        r#"<nc:BiddingZoneBorder rdf:ID="{id}">
    <nc:BiddingZoneBorder.FromBiddingZone rdf:resource='#{from}'/>
    <nc:BiddingZoneBorder.ToBiddingZone rdf:resource='#{to}'/>
  </nc:BiddingZoneBorder>"#
    );
    let line = r#"<nc:CrossZonalLine rdf:ID="_czl">
    <nc:CrossZonalNetworkElement.BiddingZoneBorder rdf:resource='#_b1'/>
    <nc:CrossZonalNetworkElement.BiddingZoneBorder rdf:resource='#_b2'/>
  </nc:CrossZonalLine>"#;
    assert!(merged(&format!("{line}{}{}", border("_b1", "_za", "_zb"), border("_b2", "_zb", "_za")), rule).is_empty());
    assert_eq!(merged(&format!("{line}{}{}", border("_b1", "_za", "_zb"), border("_b2", "_za", "_zb")), rule), ["_czl"]);
    assert_eq!(merged(&format!("{line}{}{}", border("_b1", "_za", "_za"), border("_b2", "_za", "_za")), rule), ["_czl"]);
}

#[test]
fn a_boundary_point_has_a_border_or_a_link_but_not_both() {
    let rule = "erc:BoundaryPoint-requiredAssociation";
    let border = r#"<nc:BoundaryPoint.BoundaryPointBorder rdf:resource='#_bpb'/>"#;
    let link = r#"<nc:BoundaryPointBorderLink rdf:ID="_l"><nc:BoundaryPointBorderLink.BoundaryPoint rdf:resource='#_bp'/></nc:BoundaryPointBorderLink>"#;
    let with = |inner: &str, outer: &str| merged(&format!(r#"<nc:BoundaryPoint rdf:ID="_bp">{inner}</nc:BoundaryPoint>{outer}"#), rule);
    assert!(with(border, "").is_empty());
    assert!(with("", link).is_empty());
    assert_eq!(with("", ""), ["_bp"]);
    assert_eq!(with(border, link), ["_bp"]);
}

#[test]
fn boundary_names_join_the_party_names() {
    for (class, one, two, rule) in [
        ("BoundaryPoint", "partyOneResourceName", "partyTwoResourceName", "erc:BoundaryPoint-naming"),
        ("BoundaryPointBorder", "partyOneName", "partyTwoName", "erc:BoundaryPointBorder-naming"),
    ] {
        let with = |name: &str| merged(&format!(
            r#"<nc:{class} rdf:ID="_x">
    <nc:{class}.{one}>A</nc:{class}.{one}>
    <nc:{class}.{two}>B</nc:{class}.{two}>
    {name}
  </nc:{class}>"#
        ), rule);
        assert!(with("<cim:IdentifiedObject.name>A-B</cim:IdentifiedObject.name>").is_empty(), "{class}");
        assert_eq!(with("<cim:IdentifiedObject.name>B-A</cim:IdentifiedObject.name>"), ["_x"], "{class}");
        assert_eq!(with(""), ["_x"], "{class}: a missing name fails too");
    }
}

#[test]
fn a_responsibility_point_has_one_node() {
    let rule = "erc:CommonResponsibilityPoint-requiredAssociation";
    let cn = r#"<nc:CommonResponsibilityPoint.ConnectivityNode rdf:resource='#_cn'/>"#;
    let dc = r#"<nc:CommonResponsibilityPoint.DCNode rdf:resource='#_dc'/>"#;
    let with = |body: &str| merged(&format!(r#"<nc:GridConnectionPoint rdf:ID="_p">{body}</nc:GridConnectionPoint>"#), rule);
    assert!(with(cn).is_empty());
    assert!(with(dc).is_empty());
    assert_eq!(with(""), ["_p"]);
    assert_eq!(with(&format!("{cn}{dc}")), ["_p"]);
}

#[test]
fn a_border_link_needs_two_borders_for_its_point() {
    let rule = "erc:BoundaryPointBorderLink-usage";
    let link = |id: &str, border: &str| format!(
        r#"<nc:BoundaryPointBorderLink rdf:ID="{id}">
    <nc:BoundaryPointBorderLink.BoundaryPoint rdf:resource='#_bp'/>
    <nc:BoundaryPointBorderLink.BoundaryPointBorder rdf:resource='#{border}'/>
  </nc:BoundaryPointBorderLink>"#
    );
    assert!(merged(&format!("{}{}", link("_l1", "_b1"), link("_l2", "_b2")), rule).is_empty());
    assert_eq!(merged(&format!("{}{}", link("_l1", "_b1"), link("_l2", "_b1")), rule), ["_l1", "_l2"]);
    assert_eq!(merged(&link("_l1", "_b1"), rule), ["_l1"]);
}

#[test]
fn a_dc_tie_corridor_is_scheduled_through_its_area_or_its_poles_not_both() {
    let rule = "psc:DCTieCorridor-powerSchedule";
    let corridor = r#"<nc:DCTieCorridor rdf:ID="_c"><nc:DCTieCorridor.SchedulingArea rdf:resource='#_sa'/></nc:DCTieCorridor>
  <nc:DCPole rdf:ID="_pole"><nc:DCPole.DCTieCorridor rdf:resource='#_c'/></nc:DCPole>"#;
    let by_area = r#"<nc:PowerSchedule rdf:ID="_ps1"><nc:PowerSchedule.SchedulingArea rdf:resource='#_sa'/></nc:PowerSchedule>"#;
    let by_pole = r#"<nc:PowerSchedule rdf:ID="_ps2"><nc:PowerSchedule.DCPole rdf:resource='#_pole'/></nc:PowerSchedule>"#;
    assert!(merged(&format!("{corridor}{by_area}"), rule).is_empty());
    assert!(merged(&format!("{corridor}{by_pole}"), rule).is_empty());
    assert_eq!(merged(&format!("{corridor}{by_area}{by_pole}"), rule), ["_c"]);
}

#[test]
fn a_priced_power_schedule_needs_a_currency() {
    let schedule = |currency: &str| format!(
        r#"<nc:PowerSchedule rdf:ID="_ps">{currency}</nc:PowerSchedule>
  <nc:PowerTimePoint rdf:ID="_tp"><nc:PowerTimePoint.PowerSchedule rdf:resource='#_ps'/><nc:PowerTimePoint.price>10</nc:PowerTimePoint.price></nc:PowerTimePoint>"#
    );
    let eur = r#"<nc:PowerSchedule.currency rdf:resource="https://cim.ucaiug.io/ns#Currency.EUR"/>"#;
    for rule in ["psc:PowerSchedule-currency-property", "rasc:PowerSchedule-currency-property"] {
        assert_eq!(merged(&schedule(""), rule), ["_ps"], "{rule}");
        assert!(merged(&schedule(eur), rule).is_empty(), "{rule}");
    }
    let rule = "rasc:PowerSchedule-currencyConsistency";
    let action = |currency: &str| format!(
        r#"<nc:RedispatchScheduleAction rdf:ID="_a"><nc:PowerScheduleAction.PowerSchedule rdf:resource='#_ps'/>{currency}</nc:RedispatchScheduleAction>"#
    );
    let action_eur = r#"<nc:PowerScheduleAction.currency rdf:resource="https://cim.ucaiug.io/ns#Currency.EUR"/>"#;
    assert_eq!(merged(&format!("{}{}", schedule(""), action("")), rule), ["_ps"]);
    assert!(merged(&format!("{}{}", schedule(""), action(action_eur)), rule).is_empty());
    assert!(merged(&format!("{}{}", schedule(eur), action("")), rule).is_empty());
    assert!(merged(&schedule(""), rule).is_empty(), "no action, no consistency question");
}

#[test]
fn a_priced_schedule_action_needs_a_currency() {
    let rule = "rasc:PowerScheduleAction-currency-property";
    let with = |body: &str| merged(&format!(r#"<nc:CountertradeScheduleAction rdf:ID="_a">{body}</nc:CountertradeScheduleAction>"#), rule);
    let price = "<nc:PowerScheduleAction.energyPrice>5</nc:PowerScheduleAction.energyPrice>";
    let eur = r#"<nc:PowerScheduleAction.currency rdf:resource="https://cim.ucaiug.io/ns#Currency.EUR"/>"#;
    assert_eq!(with(price), ["_a"]);
    assert!(with(&format!("{price}{eur}")).is_empty());
    assert!(with("").is_empty());
}

#[test]
fn schedule_dependencies_have_a_kind_exactly_within_a_group() {
    let group = r#"<nc:RemedialActionScheduleGroup rdf:ID="_g"/>"#;
    let dep = |in_group: bool, kind: bool| format!(
        r#"<nc:RemedialActionScheduleDependency rdf:ID="_d">{}{}</nc:RemedialActionScheduleDependency>"#,
        if in_group { r##"<nc:RemedialActionScheduleDependency.RemedialActionScheduleGroup rdf:resource='#_g'/>"## } else { "" },
        if kind { r#"<nc:RemedialActionScheduleDependency.kind rdf:resource="https://cim4.eu/ns/nc#RemedialActionScheduleDependencyKind.exclusive"/>"# } else { "" },
    );
    let cardinality = "rasc:RemedialActionScheduleDependency.kind-cardinality";
    let applicability = "rasc:RemedialActionScheduleDependency.kind-applicability";
    assert!(merged(&format!("{group}{}", dep(true, true)), cardinality).is_empty());
    assert_eq!(merged(&format!("{group}{}", dep(true, false)), cardinality), ["_g"]);
    assert_eq!(merged(group, cardinality), ["_g"]);
    assert_eq!(merged(&dep(false, true), applicability), ["_d"]);
    assert!(merged(&dep(false, false), applicability).is_empty());
    assert!(merged(&format!("{group}{}", dep(true, true)), applicability).is_empty());
}

#[test]
fn a_refused_schedule_says_why() {
    let response = |body: &str| format!(r#"<nc:RemedialActionScheduleResponse rdf:ID="_r">{body}</nc:RemedialActionScheduleResponse>"#);
    let refused = r#"<nc:RemedialActionScheduleResponse.kind rdf:resource="https://cim4.eu/ns/nc#RemedialActionScheduleResponseKind.refused"/>"#;
    let other = r#"<nc:RemedialActionScheduleResponse.rejectionReasonKind rdf:resource="https://cim4.eu/ns/nc#RejectionReasonKind.other"/>"#;
    let reason = "<nc:RemedialActionScheduleResponse.rejectionReason>too late</nc:RemedialActionScheduleResponse.rejectionReason>";
    let required = "rasc:RemedialActionScheduleResponse.rejectionReasonKind-required";
    let applicability = "rasc:RemedialActionScheduleResponse.rejectionReason-applicability";
    assert_eq!(merged(&response(refused), required), ["_r"]);
    assert!(merged(&response(&format!("{refused}{other}{reason}")), required).is_empty());
    assert_eq!(merged(&response(other), applicability), ["_r"]);
    assert!(merged(&response(&format!("{other}{reason}")), applicability).is_empty());
    let v = validate_merged(&decode(&response(other)));
    assert_eq!(v.iter().find(|x| x.rule_id == applicability).unwrap().severity, "sh:Warning");
}

#[test]
fn a_power_flow_result_reports_the_values_its_limit_needs() {
    let result = |limit_class: &str, values: &str| format!(
        r#"<cim:{limit_class} rdf:ID="_lim"/>
  <nc:BaseCasePowerFlowResult rdf:ID="_pfr">
    <nc:PowerFlowResult.OperationalLimit rdf:resource='#_lim'/>
    <nc:PowerFlowResult.value>1</nc:PowerFlowResult.value>
    <nc:PowerFlowResult.absoluteValue>1</nc:PowerFlowResult.absoluteValue>
    {values}
  </nc:BaseCasePowerFlowResult>"#
    );
    for (limit, value, rule) in [
        ("ApparentPowerLimit", "valueVA", "sarc:PowerFlowResult-ApparentPowerLimit"),
        ("ActivePowerLimit", "valueW", "sarc:PowerFlowResult-ActivePowerLimit"),
        ("CurrentLimit", "valueA", "sarc:PowerFlowResult-CurrentLimit"),
        ("VoltageLimit", "valueV", "sarc:PowerFlowResult-VoltageLimit"),
    ] {
        assert_eq!(merged(&result(limit, ""), rule), ["_pfr"], "{limit}");
        let given = format!("<nc:PowerFlowResult.{value}>1</nc:PowerFlowResult.{value}>");
        assert!(merged(&result(limit, &given), rule).is_empty(), "{limit}");
    }
    // `sarc:PowerFlowResult-ReactivePowerLimit` is not exercised: neither
    // CGMES 3.0 nor NCP defines ReactivePowerLimit, so no such element decodes.
    // A limit the dataset does not hold has no class to read.
    let absent = r#"<nc:BaseCasePowerFlowResult rdf:ID="_pfr"><nc:PowerFlowResult.OperationalLimit rdf:resource='#_gone'/></nc:BaseCasePowerFlowResult>"#;
    assert!(merged(absent, "sarc:PowerFlowResult-CurrentLimit").is_empty());

    let rule = "sarc:PowerFlowResult.value";
    let without_values = r#"<nc:ContingencyPowerFlowResult rdf:ID="_pfr"><nc:PowerFlowResult.OperationalLimit rdf:resource='#_lim'/><nc:PowerFlowResult.value>1</nc:PowerFlowResult.value></nc:ContingencyPowerFlowResult>"#;
    assert_eq!(merged(without_values, rule), ["_pfr"]);
    assert!(merged(&result("CurrentLimit", ""), rule).is_empty());
}

#[test]
fn a_voltage_angle_limit_needs_the_angle() {
    let rule = "sarc:PowerFlowResult-VoltageAngleLimit";
    let with = |angle: &str| merged(&format!(
        r#"<nc:VoltageAngleLimit rdf:ID="_lim"/>
  <nc:BaseCasePowerFlowResult rdf:ID="_pfr"><nc:PowerFlowResult.OperationalLimit rdf:resource='#_lim'/>{angle}</nc:BaseCasePowerFlowResult>"#
    ), rule);
    assert_eq!(with(""), ["_pfr"]);
    assert!(with("<nc:PowerFlowResult.valueAngle>3</nc:PowerFlowResult.valueAngle>").is_empty());
}

#[test]
fn a_stage_is_given_exactly_for_a_scheme_remedial_action() {
    let rule = "sarc:RemedialActionApplied.StageForRemedialActionScheme-cardinality";
    let with = |ra_class: &str, stage: bool| merged(&format!(
        r#"<nc:{ra_class} rdf:ID="_ra"/>
  <nc:RemedialActionApplied rdf:ID="_app"><nc:RemedialActionApplied.RemedialAction rdf:resource='#_ra'/>{}</nc:RemedialActionApplied>"#,
        if stage { r##"<nc:RemedialActionApplied.StageForRemedialActionScheme rdf:resource='#_st'/>"## } else { "" }
    ), rule);
    assert!(with("SchemeRemedialAction", true).is_empty());
    assert_eq!(with("SchemeRemedialAction", false), ["_app"]);
    assert_eq!(with("GridStateAlterationRemedialAction", true), ["_app"]);
    assert!(with("GridStateAlterationRemedialAction", false).is_empty());
}

#[test]
fn power_flow_values_match_what_the_result_is_attached_to() {
    let with = |body: &str, rule: &str| merged(&format!(r#"<nc:BaseCasePowerFlowResult rdf:ID="_pfr">{body}</nc:BaseCasePowerFlowResult>"#), rule);
    let v = "<nc:PowerFlowResult.valueV>400</nc:PowerFlowResult.valueV>";
    let w = "<nc:PowerFlowResult.valueW>10</nc:PowerFlowResult.valueW>";
    let terminal = r#"<nc:PowerFlowResult.ACDCTerminal rdf:resource='#_t'/>"#;
    let node = r#"<nc:PowerFlowResult.TopologicalNode rdf:resource='#_tn'/>"#;
    assert_eq!(with(&format!("{v}{terminal}"), "sarc:PowerFlowResult-voltageAndAngle"), ["_pfr"]);
    assert!(with(&format!("{v}{node}"), "sarc:PowerFlowResult-voltageAndAngle").is_empty());
    assert_eq!(with(&format!("{w}{node}"), "sarc:PowerFlowResult-topologicalNode"), ["_pfr"]);
    assert!(with(&format!("{w}{terminal}"), "sarc:PowerFlowResult-topologicalNode").is_empty());
}

#[test]
fn a_power_bid_time_point_combines_its_costs_and_values_as_allowed() {
    let rule = "sisc:PowerBidScheduleTimePoint-attributes";
    let with = |attrs: &[&str]| {
        let body: String = attrs
            .iter()
            .map(|a| format!("<nc:PowerBidScheduleTimePoint.{a}>1</nc:PowerBidScheduleTimePoint.{a}>"))
            .collect();
        merged(&format!(r#"<nc:PowerBidScheduleTimePoint rdf:ID="_tp">{body}</nc:PowerBidScheduleTimePoint>"#), rule)
    };
    assert!(with(&["activationCost"]).is_empty());
    assert!(with(&["shutdownCost"]).is_empty());
    assert!(with(&["p", "price"]).is_empty());
    assert_eq!(with(&["activationCost", "shutdownCost"]), ["_tp"]);
    assert_eq!(with(&["activationCost", "p"]), ["_tp"]);
}

#[test]
fn a_reference_to_an_object_not_in_the_merged_data_dangles() {
    let rule = "com:All-DanglingReferences";
    let body = r#"<nc:BoundaryPoint rdf:ID="_bp">
    <nc:BoundaryPoint.BoundaryPointBorder rdf:resource='#_bpb'/>
    <nc:BoundaryPoint.ToEndIsoCode rdf:resource="http://publications.europa.eu/resource/authority/country/BEL"/>
  </nc:BoundaryPoint>"#;
    let v = validate_merged(&decode(body));
    let found: Vec<(&str, &str)> =
        v.iter().filter(|x| x.rule_id == rule).map(|x| (x.object_id.as_str(), x.property.as_str())).collect();
    // The reference data IRI is not a CIM identifier.
    assert_eq!(found, [("_bp", "BoundaryPoint.BoundaryPointBorder")]);
    let resolved = format!(r#"{body}<nc:BoundaryPointBorder rdf:ID="_bpb"/>"#);
    assert!(merged(&resolved, rule).is_empty());
}

// ── the merged pass as `validate_crossprofile` runs it ────────────────────

/// The Complex files' table shapes and rules run once on the merged data when
/// an NC profile is in play, and not otherwise: CGMES-only data never loads
/// the NC table.
#[test]
fn the_crossprofile_pass_runs_the_complex_files_for_nc_data() {
    // A contingency on a bay: Contingency-AP-Con-Complex allows equipment
    // classes only, and the bay is in the data, so its class is known.
    let ds = decode(r#"<cim17:Bay rdf:ID="_bay"/>
  <cim:ContingencyEquipment rdf:ID="_ce"><cim:ContingencyEquipment.Equipment rdf:resource='#_bay'/></cim:ContingencyEquipment>"#);
    let rule = "coc:ContingencyElement.Equipment-allowedClasses";
    let run = |profiles: &[&str]| {
        let cfg = cimvalidation::Config { profiles: profiles.iter().map(|p| p.to_string()).collect(), ..Default::default() };
        reported(&cimvalidation::validate_crossprofile(&ds, &cfg), rule)
    };
    assert_eq!(run(&["CO"]), ["_ce"]);
    assert!(run(&["EQ"]).is_empty());
    assert!(run(&[]).is_empty());
}

/// `cimvalidation` names the merged profile without depending on
/// `cimoxide-schema`, which assigns it.
#[test]
fn the_merged_profile_code_matches_the_resolver() {
    assert_eq!(cimvalidation::MERGED_PROFILE, cimschema::shacl::resolve::MERGED_PROFILE);
}
