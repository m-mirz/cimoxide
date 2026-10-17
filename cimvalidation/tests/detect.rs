//! Profile detection from a CGMES `md:FullModel` header, or a DCAT header's
//! `dcterms:conformsTo`, through the index generated from `CGMES/PROF`.

use cimmodel::CimDataset;

fn header(profiles: &[&str]) -> String {
    let profiles: String = profiles
        .iter()
        .map(|p| format!("    <md:Model.profile>{p}</md:Model.profile>\n"))
        .collect();
    format!(
        r#"<?xml version="1.0" encoding="UTF-8"?>
<rdf:RDF xmlns:rdf="http://www.w3.org/1999/02/22-rdf-syntax-ns#"
         xmlns:cim="http://iec.ch/TC57/CIM100#"
         xmlns:md="http://iec.ch/TC57/61970-552/ModelDescription/1#">
  <md:FullModel rdf:about="urn:uuid:00000000-0000-0000-0000-000000000001">
    <md:Model.created>2026-01-01T00:00:00Z</md:Model.created>
{profiles}  </md:FullModel>
</rdf:RDF>"#
    )
}

fn detect(profiles: &[&str]) -> cimvalidation::Config {
    let ds = CimDataset::decode_str(&header(profiles)).expect("decode");
    cimvalidation::detect_config(&ds)
}

#[test]
fn declared_profiles_map_to_their_codes() {
    let cfg = detect(&[
        "http://iec.ch/TC57/ns/CIM/CoreEquipment-EU/3.0",
        "http://iec.ch/TC57/ns/CIM/SteadyStateHypothesis-EU/3.0",
        "http://iec.ch/TC57/ns/CIM/Dynamics-EU/1.0",
    ]);
    assert_eq!(cfg.profiles, ["DY", "EQ", "SSH"]);
    assert!(cfg.not_solved && !cfg.solved);
}

#[test]
fn state_variables_make_the_set_solved() {
    let cfg = detect(&["http://iec.ch/TC57/ns/CIM/StateVariables-EU/3.0"]);
    assert_eq!(cfg.profiles, ["SV"]);
    assert!(cfg.solved && !cfg.not_solved);
}

#[test]
fn an_unknown_profile_iri_is_ignored() {
    let cfg = detect(&[
        "http://example.com/NotAProfile/1.0",
        "http://iec.ch/TC57/ns/CIM/Topology-EU/3.0",
    ]);
    assert_eq!(cfg.profiles, ["TP"]);
}

/// Every code the generated index can report is one validation dispatches on
/// or `FH`, the file header, whose rules run for every file anyway.
#[test]
fn the_index_covers_the_cgmes_profiles() {
    let (iris, codes) = cimvalidation::cgmes_profile_index();
    assert_eq!(codes, ["DL", "DY", "EQ", "EQBD", "FH", "GL", "OP", "SC", "SSH", "SV", "TP"]);
    assert!(iris.iter().all(|(_, code)| codes.contains(code)));
}

// ── DCAT headers ───────────────────────────────────────────────────────────

/// A CGMES file with the DCAT header NC uses, as ReliCapGrid's boundary files
/// carry: `conforms_to` are its `dcterms:conformsTo` IRIs.
fn dcat_file(conforms_to: &[&str], body: &str) -> CimDataset {
    let conforms: String = conforms_to
        .iter()
        .map(|c| format!("    <dcterms:conformsTo rdf:resource=\"{c}\"/>\n"))
        .collect();
    let xml = format!(
        r#"<?xml version="1.0" encoding="UTF-8"?>
<rdf:RDF xmlns:rdf="http://www.w3.org/1999/02/22-rdf-syntax-ns#"
         xmlns:cim="http://iec.ch/TC57/CIM100#"
         xmlns:dcat="http://www.w3.org/ns/dcat#"
         xmlns:dcterms="http://purl.org/dc/terms/">
  <dcat:Dataset rdf:about="urn:uuid:00000000-0000-0000-0000-000000000002">
{conforms}  </dcat:Dataset>
{body}
</rdf:RDF>"#
    );
    CimDataset::decode_str(&xml).expect("decode")
}

#[test]
fn a_dcat_header_declares_cgmes_profiles() {
    let ds = dcat_file(&[
        "http://iec.ch/TC57/CIM100#",
        "http://iec.ch/TC57/ns/CIM/CoreEquipment-EU/3.0",
        "http://iec.ch/TC57/ns/CIM/EquipmentBoundary-EU/3.0",
    ], "");
    assert_eq!(cimvalidation::detect_config(&ds).profiles, ["EQ", "EQBD"]);
    assert!(cimvalidation::detect_nc_profiles(&ds).is_empty());
}

/// ReliCapGrid's boundary headers name vocabularies only.
#[test]
fn a_vocabulary_is_not_a_profile() {
    let ds = dcat_file(&["http://iec.ch/TC57/CIM100#", "https://cim4.eu/ns/nc/2.4#"], "");
    assert!(cimvalidation::detect_config(&ds).profiles.is_empty());
}

/// The detected profile's shapes run: a line without its required length.
#[test]
fn shapes_run_on_a_file_with_a_dcat_header() {
    let line = r#"  <cim:ACLineSegment rdf:ID="_line"/>"#;
    let run = |conforms_to: &[&str]| {
        let files = vec![dcat_file(conforms_to, line)];
        let cfg = cimvalidation::combined_config(&files, None, None, false, false, Vec::new());
        cimvalidation::validate_files(files, &cfg)
            .into_iter()
            .filter(|v| v.object_id == "_line" && v.rule_id.starts_with("eq:"))
            .count()
    };
    assert!(run(&["http://iec.ch/TC57/ns/CIM/CoreEquipment-EU/3.0"]) > 0);
    assert_eq!(run(&["http://iec.ch/TC57/CIM100#"]), 0, "no profile declared, nothing runs");
}
