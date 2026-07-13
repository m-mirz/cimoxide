use std::path::Path;
use cimdecoder::CimDataset;

#[test]
fn decode_small_file() {
    let path = Path::new("../testdata/test_003.xml");
    let ds = CimDataset::decode_file(path).expect("decode failed");
    assert!(!ds.entries.is_empty(), "expected at least one object");

    let node = ds.entries.get("N0").expect("TopologicalNode N0 not found");
    assert_eq!(node.element.type_name(), "TopologicalNode");
    assert_eq!(node.element.mrid(), "N0");
}

#[test]
fn decode_eq_file_fields() {
    let path = Path::new("../testdata/test_sparql_EQ_001.xml");
    let ds = CimDataset::decode_file(path).expect("decode failed");

    // BaseVoltage.nominalVoltage (f64 field) should be decoded
    let bv = ds.entries.get("BV.110").expect("BaseVoltage BV.110 not found");
    assert_eq!(bv.element.type_name(), "BaseVoltage");
    // Check raw block has the field
    assert!(bv.block.fields.contains_key("BaseVoltage.nominalVoltage"), "missing nominalVoltage field");

    // VoltageLevel → BaseVoltage (MridRef) should be decoded
    let vl = ds.entries.get("VL.110").expect("VoltageLevel VL.110 not found");
    assert_eq!(vl.element.type_name(), "VoltageLevel");
    assert!(vl.block.fields.contains_key("VoltageLevel.BaseVoltage"), "missing BaseVoltage ref");
}

#[test]
fn decode_merge_two_files() {
    let eq = Path::new("../testdata/test_003.xml");
    let tp = Path::new("../testdata/test_004.xml");
    let ds = CimDataset::decode_files(&[eq, tp]).expect("decode_files failed");
    assert!(!ds.entries.is_empty(), "expected objects after merge");
}

#[test]
fn repeated_text_field_keeps_all_values() {
    // A combined EQ+SC header declares md:Model.profile twice; the decoder must
    // keep both (regression: repeated Text fields used to overwrite each other,
    // so profile detection only ever saw the last profile of a file).
    let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<rdf:RDF xmlns:rdf="http://www.w3.org/1999/02/22-rdf-syntax-ns#"
         xmlns:md="http://iec.ch/TC57/61970-552/ModelDescription/1#"
         xmlns:cim="http://iec.ch/TC57/CIM100#">
  <md:FullModel rdf:about="urn:uuid:11111111-1111-1111-1111-111111111111">
    <md:Model.profile>http://iec.ch/TC57/ns/CIM/CoreEquipment-EU/3.0</md:Model.profile>
    <md:Model.profile>http://iec.ch/TC57/ns/CIM/ShortCircuit-EU/3.0</md:Model.profile>
  </md:FullModel>
</rdf:RDF>"#;
    let ds = CimDataset::decode_str(xml).expect("decode failed");
    let fm_mrid = &ds.by_type["FullModel"][0];
    let fm = ds.entries[fm_mrid]
        .element
        .as_any()
        .downcast_ref::<cimstructs::FullModel>()
        .expect("FullModel downcast");
    assert_eq!(
        fm.base.profile,
        vec![
            "http://iec.ch/TC57/ns/CIM/CoreEquipment-EU/3.0".to_string(),
            "http://iec.ch/TC57/ns/CIM/ShortCircuit-EU/3.0".to_string(),
        ],
        "both md:Model.profile values must survive decoding"
    );
}
