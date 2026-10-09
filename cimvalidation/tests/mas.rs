//! The NotSolvedMAS and SolvedMAS rules read across a model authority set's
//! profiles, so they run on the merged dataset: on one file there is nothing
//! to compare.

use cimmodel::CimDataset;

fn file(profile: &str, body: &str) -> CimDataset {
    let xml = format!(
        r##"<?xml version="1.0" encoding="UTF-8"?>
<rdf:RDF xmlns:rdf="http://www.w3.org/1999/02/22-rdf-syntax-ns#"
         xmlns:cim="http://iec.ch/TC57/CIM100#"
         xmlns:md="http://iec.ch/TC57/61970-552/ModelDescription/1#">
  <md:FullModel rdf:about="urn:uuid:00000000-0000-0000-0000-00000000000{n}">
    <md:Model.created>2026-01-01T00:00:00Z</md:Model.created>
    <md:Model.profile>http://iec.ch/TC57/ns/CIM/{profile}/3.0</md:Model.profile>
  </md:FullModel>
{body}
</rdf:RDF>"##,
        n = profile.len() % 10,
    );
    CimDataset::decode_str(&xml).expect("decode")
}

const RULE: &str = "sshn456:RotatingMachine.p-limits";

fn eq() -> CimDataset {
    file("CoreEquipment-EU", r##"
  <cim:ThermalGeneratingUnit rdf:ID="_gu">
    <cim:GeneratingUnit.minOperatingP>0</cim:GeneratingUnit.minOperatingP>
    <cim:GeneratingUnit.maxOperatingP>100</cim:GeneratingUnit.maxOperatingP>
  </cim:ThermalGeneratingUnit>
  <cim:SynchronousMachine rdf:ID="_sm">
    <cim:RotatingMachine.GeneratingUnit rdf:resource="#_gu"/>
  </cim:SynchronousMachine>"##)
}

fn ssh() -> CimDataset {
    // p = 50 in the load convention: the unit consumes 50 MW, below its minimum of 0.
    file("SteadyStateHypothesis-EU", r##"
  <cim:SynchronousMachine rdf:about="#_sm">
    <cim:Equipment.inService>true</cim:Equipment.inService>
    <cim:RotatingMachine.p>50</cim:RotatingMachine.p>
  </cim:SynchronousMachine>"##)
}

#[test]
fn a_rule_reading_eq_and_ssh_sees_both_files() {
    let files = vec![eq(), ssh()];
    let cfg = cimvalidation::combined_config(&files, None, None, false, false, Vec::new());
    assert!(cfg.not_solved);

    let ssh_alone = cimvalidation::validate_profile_local(&files[1], "SSH", &cfg);
    assert_eq!(ssh_alone.iter().filter(|v| v.rule_id == RULE).count(), 0, "the SSH file holds no limits");

    let all = cimvalidation::validate_files(files, &cfg);
    assert_eq!(all.iter().filter(|v| v.rule_id == RULE).count(), 1, "{all:#?}");
}

/// Merged, the equipment is typed both `ACLineSegment` (EQ) and `Equipment`
/// (SSH), whichever file comes first, and `sh:class ConductingEquipment`
/// holds: one of its types is one.
#[test]
fn a_merged_object_carries_every_type_it_was_written_under() {
    let eq = file("CoreEquipment-EU", r##"
  <cim:ACLineSegment rdf:ID="_line"/>"##);
    let ssh = file("SteadyStateHypothesis-EU", r##"
  <cim:Equipment rdf:about="#_line">
    <cim:Equipment.inService>true</cim:Equipment.inService>
  </cim:Equipment>"##);
    let sv = file("StateVariables-EU", r##"
  <cim:SvStatus rdf:ID="_status">
    <cim:SvStatus.ConductingEquipment rdf:resource="#_line"/>
    <cim:SvStatus.inService>true</cim:SvStatus.inService>
  </cim:SvStatus>"##);
    let mut merged = CimDataset::new();
    for ds in [ssh, eq, sv] {
        merged.merge(ds);
    }
    let cfg = cimvalidation::Config { profiles: vec!["SV".into()], solved: true, ..Default::default() };
    let v = cimvalidation::validate_crossprofile_shacl(&merged, &cfg);
    let class_rule = "sv456cpi:SvStatus.ConductingEquipment-valueType";
    assert_eq!(v.iter().filter(|v| v.rule_id == class_rule).count(), 0, "{v:#?}");
}
