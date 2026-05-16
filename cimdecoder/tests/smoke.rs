use std::path::Path;
use cimdecoder::CimDataset;

#[test]
fn decode_small_file() {
    let path = Path::new("../cimgo/testdata/test_003.xml");
    let ds = CimDataset::decode_file(path).expect("decode failed");
    assert!(!ds.entries.is_empty(), "expected at least one object");

    let node = ds.entries.get("N0").expect("TopologicalNode N0 not found");
    assert_eq!(node.element.type_name(), "TopologicalNode");
    assert_eq!(node.element.mrid(), "N0");
}

#[test]
fn decode_eq_file_fields() {
    let path = Path::new("../cimgo/testdata/test_sparql_EQ_001.xml");
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
    let eq = Path::new("../cimgo/testdata/test_003.xml");
    let tp = Path::new("../cimgo/testdata/test_004.xml");
    let ds = CimDataset::decode_files(&[eq, tp]).expect("decode_files failed");
    assert!(!ds.entries.is_empty(), "expected objects after merge");
}
