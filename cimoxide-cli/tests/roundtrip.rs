use std::path::Path;

use cimdecoder::CimDataset;
use cimoxide_cli_convert::{dataset_from_json, dataset_to_json, dataset_to_xml};

// Re-export convert module for tests
mod cimoxide_cli_convert {
    include!("../src/convert.rs");
}

fn test_xml_path() -> &'static Path {
    Path::new("../testdata/test_003.xml")
}

#[test]
fn json_shape() {
    let ds = CimDataset::decode_file(test_xml_path()).expect("decode failed");
    let json = dataset_to_json(&ds);
    let map = json.as_object().expect("root is object");

    // N0 is a TopologicalNode with name "N0"
    let n0 = &map["N0"];
    assert_eq!(n0["_type"].as_str().unwrap(), "TopologicalNode");
    assert_eq!(n0["name"].as_str().unwrap(), "N0");

    // Terminal.N0 MridRef field should be a plain string
    let t = &map["Terminal.N0"];
    assert_eq!(t["_type"].as_str().unwrap(), "Terminal");
    assert!(
        t["topological_node"].is_string(),
        "MridRef should serialize as string"
    );
    assert_eq!(t["topological_node"].as_str().unwrap(), "N0");
}

#[test]
fn json_to_dataset_mrid_set() {
    let ds = CimDataset::decode_file(test_xml_path()).expect("decode failed");
    let original_mrids: std::collections::HashSet<&str> =
        ds.entries.keys().map(String::as_str).collect();

    let json = serde_json::to_string(&dataset_to_json(&ds)).expect("serialize");
    let ds2 = dataset_from_json(&json).expect("from_json failed");

    let round_trip_mrids: std::collections::HashSet<&str> =
        ds2.entries.keys().map(String::as_str).collect();

    assert_eq!(
        original_mrids, round_trip_mrids,
        "MRID set must survive JSON round-trip"
    );
}

#[test]
fn xml_round_trip_mrid_count() {
    let ds = CimDataset::decode_file(test_xml_path()).expect("decode failed");
    let original_count = ds.entries.len();

    let json = serde_json::to_string(&dataset_to_json(&ds)).expect("serialize");
    let ds2 = dataset_from_json(&json).expect("from_json failed");
    let xml = dataset_to_xml(&ds2).expect("to_xml failed");

    let ds3 = CimDataset::decode_str(&xml).expect("decode round-trip XML failed");
    assert_eq!(
        ds3.entries.len(),
        original_count,
        "entry count must survive XML round-trip"
    );
}

#[test]
fn xml_output_structure() {
    let ds = CimDataset::decode_file(test_xml_path()).expect("decode failed");
    let json = serde_json::to_string(&dataset_to_json(&ds)).expect("serialize");
    let ds2 = dataset_from_json(&json).expect("from_json");
    let xml = dataset_to_xml(&ds2).expect("to_xml");

    assert!(xml.contains("xmlns:cim="), "must declare cim namespace");
    assert!(xml.contains("rdf:about="), "must use rdf:about for MRIDs");
    assert!(
        xml.contains("rdf:resource="),
        "must use rdf:resource for references"
    );
}

#[test]
fn numeric_fields_are_numbers() {
    let eq_path = Path::new("../testdata/test_sparql_EQ_001.xml");
    if !eq_path.exists() {
        return; // skip if testdata not present
    }
    let ds = CimDataset::decode_file(eq_path).expect("decode failed");
    let json = dataset_to_json(&ds);
    let map = json.as_object().expect("root is object");

    // BaseVoltage should have a numeric nominalVoltage field
    for (_mrid, obj) in map {
        if obj["_type"].as_str() == Some("BaseVoltage") {
            if let Some(v) = obj.get("nominal_voltage") {
                assert!(v.is_number(), "nominalVoltage must be a number, got {v}");
            }
            break;
        }
    }
}
