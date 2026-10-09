//! An object written under a superclass in one file and its own class in
//! another takes its own class in the merge, whichever file comes first.

use cimmodel::CimDataset;

fn file(class: &str, field: &str) -> CimDataset {
    CimDataset::decode_str(&format!(
        r##"<?xml version="1.0" encoding="UTF-8"?>
<rdf:RDF xmlns:rdf="http://www.w3.org/1999/02/22-rdf-syntax-ns#" xmlns:cim="http://iec.ch/TC57/CIM100#">
  <cim:{class} rdf:about="#_x">{field}</cim:{class}>
</rdf:RDF>"##
    ))
    .unwrap()
}

fn eq() -> CimDataset {
    file("ACLineSegment", "<cim:ACLineSegment.r>1</cim:ACLineSegment.r>")
}

fn ssh() -> CimDataset {
    file("Equipment", "<cim:Equipment.inService>true</cim:Equipment.inService>")
}

fn check(ds: &CimDataset) {
    let e = &ds.entries["_x"];
    assert_eq!(e.type_name(), "ACLineSegment");
    let types: Vec<&str> = e.types().map(|c| c.qualified).collect();
    assert_eq!(types, ["ACLineSegment", "Equipment"], "both types kept");
    assert!(e.fields().contains_key("ACLineSegment.r") && e.fields().contains_key("Equipment.inService"));
    assert_eq!(ds.by_type.get("ACLineSegment").map(Vec::len), Some(1));
    assert!(ds.by_type.get("Equipment").is_none_or(Vec::is_empty), "{:?}", ds.by_type);
}

#[test]
fn the_subclass_wins_whichever_file_comes_first() {
    let mut a = ssh();
    a.merge(eq());
    check(&a);
    let mut b = eq();
    b.merge(ssh());
    check(&b);
}

#[test]
fn unrelated_classes_keep_the_first() {
    let mut a = file("Breaker", "");
    a.merge(file("ACLineSegment", ""));
    assert_eq!(a.entries["_x"].type_name(), "Breaker");
    assert_eq!(a.entries["_x"].types().count(), 2);
}
