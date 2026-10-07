//! Profile detection from a CGMES `md:FullModel` header, through the index
//! generated from `CGMES/PROF`.

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
