//! The hand-written CGMES rules read an absent value as absent, as their SPARQL
//! does: `$this p ?v` binds nothing, `OPTIONAL` leaves the variable unbound,
//! and a comparison with an unbound variable is false. They used to read it as
//! 0 or false, which both reported elements that only lacked an optional value
//! and missed elements whose value was present but 0.
//!
//! Each test sets one rule's values present, absent and zero.

use cimvalidation::{Config, Violation};

fn run(profile: &str, body: &str) -> Vec<Violation> {
    let xml = format!(
        r##"<?xml version="1.0" encoding="UTF-8"?>
<rdf:RDF xmlns:rdf="http://www.w3.org/1999/02/22-rdf-syntax-ns#" xmlns:cim="http://iec.ch/TC57/CIM100#">
{body}
</rdf:RDF>"##
    );
    let ds = cimmodel::CimDataset::decode_str(&xml).unwrap();
    let cfg = Config { profiles: vec![profile.into()], ..Default::default() };
    cimvalidation::sparql::validate_profile_local(&ds, profile, &cfg)
}

fn count(profile: &str, body: &str, rule: &str) -> usize {
    run(profile, body).iter().filter(|v| v.rule_id == rule).count()
}

fn element(class: &str, fields: &[(&str, &str)]) -> String {
    let inner: String = fields
        .iter()
        .map(|(k, v)| {
            if v.starts_with('#') || v.starts_with("http") {
                format!("<cim:{k} rdf:resource=\"{v}\"/>")
            } else {
                format!("<cim:{k}>{v}</cim:{k}>")
            }
        })
        .collect();
    format!("<cim:{class} rdf:ID=\"_x\">{inner}</cim:{class}>")
}

// ── short circuit ──────────────────────────────────────────────────────────

#[test]
fn varistor_values_follow_varistor_present() {
    let usage = "scu:SeriesCompensator.varistorRatedCurrent-usage";
    let required = "sc600:SeriesCompensator.varistorRatedCurrent-required";
    let sc = |fields: &[(&str, &str)]| element("SeriesCompensator", fields);
    let present = |v: &'static str| ("SeriesCompensator.varistorPresent", v);
    let current = |v: &'static str| ("SeriesCompensator.varistorRatedCurrent", v);
    // A value given while varistorPresent is false — 0 is a value.
    assert_eq!(count("SC", &sc(&[present("false"), current("0")]), usage), 1);
    // varistorPresent absent is not false.
    assert_eq!(count("SC", &sc(&[current("5")]), usage), 0);
    // Required when true; 0 counts as given.
    assert_eq!(count("SC", &sc(&[present("true")]), required), 1);
    assert_eq!(count("SC", &sc(&[present("true"), current("0")]), required), 0);
}

#[test]
fn grounding_needs_both_impedances() {
    let rule = "sc452:TransformerEnd-grounding";
    let end = |fields: &[(&str, &str)]| element("PowerTransformerEnd", fields);
    assert_eq!(count("SC", &end(&[("TransformerEnd.grounded", "true"), ("TransformerEnd.rground", "1")]), rule), 1);
    assert_eq!(count("SC", &end(&[("TransformerEnd.grounded", "true"), ("TransformerEnd.rground", "0"), ("TransformerEnd.xground", "0")]), rule), 0);
}

// ── dynamics ───────────────────────────────────────────────────────────────

#[test]
fn excitation_gains_need_their_values_given() {
    assert_eq!(count("DY", &element("ExcBBC", &[]), "dyu:ExcBBC.k-valueRange"), 0);
    assert_eq!(count("DY", &element("ExcBBC", &[("ExcBBC.k", "0")]), "dyu:ExcBBC.k-valueRange"), 1);
    let rule = "dyu:ExcAC8B.kpr-valueRange";
    assert_eq!(count("DY", &element("ExcAC8B", &[]), rule), 0);
    assert_eq!(count("DY", &element("ExcAC8B", &[("ExcAC8B.kir", "0")]), rule), 0);
    assert_eq!(count("DY", &element("ExcAC8B", &[("ExcAC8B.kir", "0"), ("ExcAC8B.kpr", "-1")]), rule), 1);
}

#[test]
fn gov_hydro4_points_compare_only_given_values() {
    let gov = |model: &str, fields: &[(&str, &str)]| {
        let mut f = vec![("GovHydro4.model", format!("http://iec.ch/TC57/CIM100#GovHydro4ModelKind.{model}"))];
        f.extend(fields.iter().map(|(k, v)| (*k, v.to_string())));
        let f: Vec<(&str, &str)> = f.iter().map(|(k, v)| (*k, v.as_str())).collect();
        element("GovHydro4", &f)
    };
    let rule = "dyu:GovHydro4.gv1-valueRange";
    assert_eq!(count("DY", &gov("kaplan", &[("GovHydro4.gv0", "0.5")]), rule), 0, "gv1 absent");
    assert_eq!(count("DY", &gov("kaplan", &[("GovHydro4.gv0", "0.5"), ("GovHydro4.gv1", "0.4")]), rule), 1);
    assert_eq!(count("DY", &gov("kaplan", &[("GovHydro4.gv0", "0.5"), ("GovHydro4.gv1", "0.6")]), rule), 0);
    assert_eq!(count("DY", &gov("simple", &[("GovHydro4.bgv0", "0.1")]), "dyu:GovHydro4.bgv0-valueRange"), 1);
    assert_eq!(count("DY", &gov("simple", &[]), "dyu:GovHydro4.bgv0-valueRange"), 0);
}

#[test]
fn load_static_models_check_attributes_by_presence() {
    let load = |model: &str, keys: &[&str]| {
        let mut f = vec![("LoadStatic.staticLoadModelType".to_string(), format!("http://iec.ch/TC57/CIM100#StaticLoadModelKind.{model}"))];
        f.extend(keys.iter().map(|k| (format!("LoadStatic.{k}"), "0".to_string())));
        let f: Vec<(&str, &str)> = f.iter().map(|(k, v)| (k.as_str(), v.as_str())).collect();
        element("LoadStatic", &f)
    };
    let all14 = ["kp1", "kp2", "kp3", "kpf", "kq1", "kq2", "kq3", "kqf", "ep1", "ep2", "ep3", "eq1", "eq2", "eq3"];
    let exp = "dyu:LoadStatic.staticLoadModelType-exponental";
    assert_eq!(count("DY", &load("exponential", &all14), exp), 0);
    assert_eq!(count("DY", &load("exponential", &all14[1..]), exp), 1, "kp1 missing");
    assert_eq!(count("DY", &load("exponential", &[&all14[..], &["kp4"]].concat()), exp), 1, "kp4 given");
    let cz = "dyu:LoadStatic.staticLoadModelType-constantZ";
    assert_eq!(count("DY", &load("constantZ", &[]), cz), 0);
    assert_eq!(count("DY", &load("constantZ", &["kp1"]), cz), 1, "a coefficient of 0 is given");
}

#[test]
fn simplified_machines_carry_no_saturation_at_all() {
    let rule = "dyu:SynchronousMachineSimplified-requiredAttributes";
    assert_eq!(count("DY", &element("SynchronousMachineSimplified", &[]), rule), 0);
    assert_eq!(count("DY", &element("SynchronousMachineSimplified", &[("RotatingMachineDynamics.saturationFactor", "0")]), rule), 1);
}

#[test]
fn subtransient_round_rotor_needs_every_parameter() {
    let rule = "dy457:SynchronousMachineTimeConstantReactance-modelType-SubtransientRoundRotor";
    let all = [
        ("SynchronousMachineDetailed.saturationFactorQAxis", "0"),
        ("SynchronousMachineDetailed.saturationFactor120QAxis", "0"),
        ("RotatingMachineDynamics.saturationFactor", "0"),
        ("RotatingMachineDynamics.saturationFactor120", "0"),
        ("SynchronousMachineTimeConstantReactance.xQuadTrans", "0"),
        ("SynchronousMachineTimeConstantReactance.tpqo", "0"),
    ];
    let with = |fields: &[(&str, &str)]| {
        let mut f = vec![
            ("SynchronousMachineTimeConstantReactance.modelType", "http://iec.ch/TC57/CIM100#SynchronousMachineModelKind.subtransient"),
            ("SynchronousMachineTimeConstantReactance.rotorType", "http://iec.ch/TC57/CIM100#RotorKind.roundRotor"),
        ];
        f.extend_from_slice(fields);
        element("SynchronousMachineTimeConstantReactance", &f)
    };
    assert_eq!(count("DY", &with(&all), rule), 0, "zeros are values");
    assert_eq!(count("DY", &with(&all[..5]), rule), 1, "tpqo missing");
}

#[test]
fn a_governor_with_no_machine_is_reported_once() {
    let body = element("GovCT1", &[]);
    assert_eq!(count("DY", &body, "dyu:TurbineGovernorDynamics"), 0, "the SPARQL rules no longer report it");
    let cfg = Config { profiles: vec!["DY".into()], ..Default::default() };
    let xml = format!(
        r##"<?xml version="1.0" encoding="UTF-8"?>
<rdf:RDF xmlns:rdf="http://www.w3.org/1999/02/22-rdf-syntax-ns#" xmlns:cim="http://iec.ch/TC57/CIM100#">{body}</rdf:RDF>"##
    );
    let ds = cimmodel::CimDataset::decode_str(&xml).unwrap();
    let all = cimvalidation::validate_profile_local(&ds, "DY", &cfg);
    assert_eq!(all.iter().filter(|v| v.rule_id == "dyu:TurbineGovernorDynamics").count(), 1, "the table reports it");
}

#[test]
fn mwbase_reports_missing_ratings_on_a_machine_the_dataset_holds() {
    let rule = "dyn457:TurbineGovernorDynamics-mbaseEquation";
    let gov = r#"<cim:GovCT1 rdf:ID="_g"><cim:GovCT1.mwbase>90</cim:GovCT1.mwbase><cim:TurbineGovernorDynamics.SynchronousMachineDynamics rdf:resource='#_smd'/></cim:GovCT1>
  <cim:SynchronousMachineTimeConstantReactance rdf:ID="_smd"><cim:SynchronousMachineDynamics.SynchronousMachine rdf:resource='#_sm'/></cim:SynchronousMachineTimeConstantReactance>"#;
    assert_eq!(count("DY", gov, rule), 0, "machine not in the dataset");
    let sm = |fields: &str| format!(r#"{gov}<cim:SynchronousMachine rdf:ID="_sm">{fields}</cim:SynchronousMachine>"#);
    assert_eq!(count("DY", &sm("<cim:RotatingMachine.ratedS>100</cim:RotatingMachine.ratedS>"), rule), 1, "ratedPowerFactor missing");
    assert_eq!(count("DY", &sm("<cim:RotatingMachine.ratedS>100</cim:RotatingMachine.ratedS><cim:RotatingMachine.ratedPowerFactor>0.9</cim:RotatingMachine.ratedPowerFactor>"), rule), 0);
}

// ── solved model, cross-profile ────────────────────────────────────────────

fn solved(body: &str, rule: &str) -> Vec<String> {
    let xml = format!(
        r##"<?xml version="1.0" encoding="UTF-8"?>
<rdf:RDF xmlns:rdf="http://www.w3.org/1999/02/22-rdf-syntax-ns#" xmlns:cim="http://iec.ch/TC57/CIM100#">
{body}
</rdf:RDF>"##
    );
    let ds = cimmodel::CimDataset::decode_str(&xml).unwrap();
    let cfg = Config { common: true, solved: true, ..Default::default() };
    let mut ids: Vec<String> = cimvalidation::sparql::validate_crossprofile(&ds, &cfg)
        .into_iter()
        .filter(|v| v.rule_id == rule)
        .map(|v| v.object_id)
        .collect();
    ids.sort();
    ids
}

/// One energized node in an island, and a piece of equipment on it.
fn energized(class: &str, extra: &str, in_service: &str) -> String {
    format!(
        r##"<cim:TopologicalIsland rdf:ID="_ti"><cim:TopologicalIsland.TopologicalNodes rdf:resource="#_tn"/></cim:TopologicalIsland>
  <cim:TopologicalNode rdf:ID="_tn"/>
  <cim:SvVoltage rdf:ID="_sv"><cim:SvVoltage.TopologicalNode rdf:resource="#_tn"/></cim:SvVoltage>
  <cim:Terminal rdf:ID="_t"><cim:Terminal.ConductingEquipment rdf:resource="#_eq"/><cim:Terminal.TopologicalNode rdf:resource="#_tn"/></cim:Terminal>
  <cim:{class} rdf:ID="_eq">{extra}</cim:{class}>
  <cim:SvStatus rdf:ID="_st"><cim:SvStatus.ConductingEquipment rdf:resource="#_eq"/><cim:SvStatus.inService>{in_service}</cim:SvStatus.inService></cim:SvStatus>"##
    )
}

/// SvSwitch is required for every switch class the shape targets, in service as
/// the SV states it.
#[test]
fn sv_switch_covers_every_switch_class_in_service() {
    let rule = "sm600:SvSwitch-SV__4";
    let retained = "<cim:Switch.retained>true</cim:Switch.retained>";
    assert_eq!(solved(&energized("Breaker", retained, "true"), rule), ["_eq"]);
    assert!(solved(&energized("Breaker", retained, "false"), rule).is_empty());
    assert!(solved(&energized("Breaker", "", "true"), rule).is_empty(), "not retained");
}

/// An out-of-service compensator needs no section count.
#[test]
fn sv_shunt_sections_are_required_in_service_only() {
    let rule = "sm600:SvShuntCompensatorSections-SV__4";
    assert_eq!(solved(&energized("LinearShuntCompensator", "", "true"), rule), ["_eq"]);
    assert!(solved(&energized("LinearShuntCompensator", "", "false"), rule).is_empty());
}

/// Equipment without an SvStatus is reported once.
#[test]
fn missing_sv_status_is_reported_once() {
    let body = energized("Breaker", "", "true").replace(
        r##"<cim:SvStatus rdf:ID="_st"><cim:SvStatus.ConductingEquipment rdf:resource="#_eq"/><cim:SvStatus.inService>true</cim:SvStatus.inService></cim:SvStatus>"##,
        "",
    );
    assert_eq!(solved(&body, "sm600:SvStatus-SV__4"), ["_eq"]);
}

/// Enabled controls of one mode on one TopologicalNode — not one Terminal —
/// must agree, and every control on the node is reported.
#[test]
fn controls_on_one_node_must_agree() {
    let rc = |id: &str, term: &str, target: &str| format!(
        r##"<cim:RegulatingControl rdf:ID="{id}"><cim:RegulatingControl.Terminal rdf:resource="#{term}"/>
    <cim:RegulatingControl.enabled>true</cim:RegulatingControl.enabled>
    <cim:RegulatingControl.mode rdf:resource="http://iec.ch/TC57/CIM100#RegulatingControlModeKind.voltage"/>
    <cim:RegulatingControl.targetValue>{target}</cim:RegulatingControl.targetValue></cim:RegulatingControl>"##
    );
    let nodes = r##"<cim:Terminal rdf:ID="_t1"><cim:Terminal.TopologicalNode rdf:resource="#_tn"/></cim:Terminal>
  <cim:Terminal rdf:ID="_t2"><cim:Terminal.TopologicalNode rdf:resource="#_tn"/></cim:Terminal>"##;
    let rule = "sm6002:RegulatingControl-samePoint";
    assert_eq!(solved(&format!("{nodes}{}{}", rc("_a", "_t1", "400"), rc("_b", "_t2", "410")), rule), ["_a", "_b"]);
    assert!(solved(&format!("{nodes}{}{}", rc("_a", "_t1", "400"), rc("_b", "_t2", "400")), rule).is_empty());
}
