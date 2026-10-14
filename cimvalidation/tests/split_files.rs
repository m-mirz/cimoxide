//! A model split over files, the way it is exchanged: one file per profile.
//! The per-file phase still checks each file alone, but an NC value-type
//! check finds a referenced object's type in whichever file of the set holds
//! it, and the hand-written rules that read another profile's data run on the
//! merged dataset.

use cimmodel::CimDataset;
use cimvalidation::Violation;

fn file(profile: &str, n: u8, body: &str) -> CimDataset {
    let xml = format!(
        r##"<?xml version="1.0" encoding="UTF-8"?>
<rdf:RDF xmlns:rdf="http://www.w3.org/1999/02/22-rdf-syntax-ns#"
         xmlns:cim="http://iec.ch/TC57/CIM100#"
         xmlns:md="http://iec.ch/TC57/61970-552/ModelDescription/1#">
  <md:FullModel rdf:about="urn:uuid:00000000-0000-0000-0000-0000000000{n:02}">
    <md:Model.created>2026-01-01T00:00:00Z</md:Model.created>
    <md:Model.profile>http://iec.ch/TC57/ns/CIM/{profile}/3.0</md:Model.profile>
  </md:FullModel>
{body}
</rdf:RDF>"##
    );
    CimDataset::decode_str(&xml).expect("decode")
}

fn validate(files: Vec<CimDataset>) -> Vec<Violation> {
    let cfg = cimvalidation::combined_config(&files, None, None, false, false, Vec::new());
    cimvalidation::validate_files(files, &cfg)
}

fn count(v: &[Violation], rule: &str) -> usize {
    v.iter().filter(|v| v.rule_id == rule).count()
}

// ── NC value types across files ────────────────────────────────────────────

fn nc_file(profile: &str, n: u8, body: &str) -> CimDataset {
    let xml = format!(
        r##"<?xml version="1.0" encoding="UTF-8"?>
<rdf:RDF xmlns:rdf="http://www.w3.org/1999/02/22-rdf-syntax-ns#"
         xmlns:cim="https://cim.ucaiug.io/ns#"
         xmlns:nc="https://cim4.eu/ns/nc#"
         xmlns:dcat="http://www.w3.org/ns/dcat#"
         xmlns:dcterms="http://purl.org/dc/terms/">
  <dcat:Dataset rdf:about="urn:uuid:00000000-0000-0000-0000-0000000000{n:02}">
    <dcterms:conformsTo rdf:resource="https://ap.cim4.eu/{profile}/2.5"/>
  </dcat:Dataset>
{body}
</rdf:RDF>"##
    );
    CimDataset::decode_str(&xml).expect("decode")
}

const NC_VALUE_TYPE: &str = "ps:PowerSchedule.GeneratingUnit-valueType";

fn schedule() -> CimDataset {
    nc_file("PowerSchedule", 1, r##"
  <nc:PowerSchedule rdf:about="urn:uuid:11111111-1111-1111-1111-111111111111">
    <nc:PowerSchedule.GeneratingUnit rdf:resource="urn:uuid:22222222-2222-2222-2222-222222222222"/>
  </nc:PowerSchedule>"##)
}

/// `class` is defined in another NC file, as an NC dataset would hold it.
fn nc_unit(class: &str) -> CimDataset {
    nc_file("EquipmentReliability", 2, &format!(r##"
  <{class} rdf:about="urn:uuid:22222222-2222-2222-2222-222222222222"/>"##))
}

#[test]
fn an_nc_value_type_reads_the_type_from_another_file() {
    let v = validate(vec![schedule(), nc_unit("cim:GeneratingUnit")]);
    assert_eq!(count(&v, NC_VALUE_TYPE), 1, "the abstract unit is not in the list: {v:#?}");
    assert_eq!(count(&validate(vec![schedule(), nc_unit("nc:IndustryUnit")]), NC_VALUE_TYPE), 0);
}

#[test]
fn an_nc_value_type_is_silent_on_an_object_no_file_holds() {
    assert_eq!(count(&validate(vec![schedule()]), NC_VALUE_TYPE), 0);
}

#[test]
fn an_nc_file_alone_still_reads_only_itself() {
    let cfg = cimvalidation::Config { profiles: vec!["PS".into()], ..Default::default() };
    let v = cimvalidation::validate_profile_local(&schedule(), "PS", &cfg);
    assert_eq!(count(&v, NC_VALUE_TYPE), 0);
}

// ── CGMES value types stay per file ────────────────────────────────────────

/// CGMES's per-profile value types list only their own profile's classes:
/// `sv:SvStatus.ConductingEquipment-valueType` the two converters. They are
/// written for one file, where the EQ equipment has no type; read across the
/// set, every line would fail. The cross-profile shape checks the type.
#[test]
fn a_cgmes_value_type_does_not_read_other_files() {
    let eq = file("CoreEquipment-EU", 1, r##"
  <cim:ACLineSegment rdf:ID="_line"/>
  <cim:Breaker rdf:ID="_brk"/>"##);
    let sv = file("StateVariables-EU", 2, r##"
  <cim:SvStatus rdf:ID="_s1">
    <cim:SvStatus.ConductingEquipment rdf:resource="#_line"/>
    <cim:SvStatus.inService>true</cim:SvStatus.inService>
  </cim:SvStatus>"##);
    let v = validate(vec![eq, sv]);
    assert_eq!(count(&v, "sv:SvStatus.ConductingEquipment-valueType"), 0, "{v:#?}");
}

// ── rules reading another profile ──────────────────────────────────────────

fn converter_eq(max_alpha: &str) -> CimDataset {
    file("CoreEquipment-EU", 1, &format!(r##"
  <cim:CsConverter rdf:ID="_csc">
    <cim:CsConverter.maxAlpha>{max_alpha}</cim:CsConverter.maxAlpha>
  </cim:CsConverter>"##))
}

fn converter_ssh() -> CimDataset {
    file("SteadyStateHypothesis-EU", 2, r##"
  <cim:CsConverter rdf:about="#_csc">
    <cim:CsConverter.operatingMode rdf:resource="http://iec.ch/TC57/CIM100#CsOperatingModeKind.rectifier"/>
  </cim:CsConverter>"##)
}

#[test]
fn a_converter_range_compares_ssh_mode_with_eq_limits() {
    let rule = "sshu:CsConverter.maxAlpha-valueRangeTypical";
    assert_eq!(count(&validate(vec![converter_eq("180"), converter_ssh()]), rule), 1);
    assert_eq!(count(&validate(vec![converter_eq("15"), converter_ssh()]), rule), 0);
    assert_eq!(count(&validate(vec![converter_eq("180")]), rule), 0, "no SSH, no mode");
}

#[test]
fn a_measured_terminal_is_found_in_eq() {
    let rule = "opn452:Measurement.Terminal-requiredCases";
    let eq = |equipment: &str| file("CoreEquipment-EU", 1, &format!(r##"
  <cim:ACLineSegment rdf:ID="_line"/>
  <cim:ACLineSegment rdf:ID="_other"/>
  <cim:Terminal rdf:ID="_t">
    <cim:Terminal.ConductingEquipment rdf:resource="#{equipment}"/>
  </cim:Terminal>"##));
    let op = || file("Operation-EU", 2, r##"
  <cim:Analog rdf:ID="_m">
    <cim:Measurement.measurementType>ThreePhaseActivePower</cim:Measurement.measurementType>
    <cim:Measurement.PowerSystemResource rdf:resource="#_line"/>
    <cim:Measurement.Terminal rdf:resource="#_t"/>
  </cim:Analog>"##);
    assert_eq!(count(&validate(vec![eq("_line"), op()]), rule), 0, "the terminal is the line's");
    assert_eq!(count(&validate(vec![eq("_other"), op()]), rule), 1, "the terminal is another line's");
}
