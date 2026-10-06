//! Decoding the NC profile family alongside CGMES.

use std::path::Path;

use cimmodel::CimDataset;
use cimmodel::base::{CimElement, GenericElement};

fn nc_fixture() -> CimDataset {
    CimDataset::decode_file(Path::new("../testdata/test_nc_CO_001.xml")).unwrap()
}

fn bag<'a>(ds: &'a CimDataset, mrid: &str) -> &'a GenericElement {
    ds.entries[mrid]
        .element
        .as_any()
        .downcast_ref::<GenericElement>()
        .expect("NC elements decode to property bags")
}

const CONTINGENCY: &str = "urn:uuid:11111111-1111-1111-1111-111111111111";
const EQUIPMENT: &str = "urn:uuid:33333333-3333-3333-3333-333333333333";

#[test]
fn nc_elements_bucket_under_qualified_names() {
    let ds = nc_fixture();
    let mut keys: Vec<&str> = ds.by_type.keys().map(String::as_str).collect();
    keys.sort();
    assert_eq!(
        keys,
        ["nc:ContingencyEquipment", "nc:Equipment", "nc:OrdinaryContingency"]
    );
    // The bare CGMES keys must not appear: a consumer looking for CGMES
    // `Equipment` must not see NC's.
    assert!(!ds.by_type.contains_key("Equipment"));
    assert!(!ds.by_type.contains_key("Contingency"));
}

#[test]
fn nc_namespaces_resolve_per_element() {
    let ds = nc_fixture();
    // Same document, two namespaces, both spelled with a prefix bound at the root.
    assert_eq!(bag(&ds, CONTINGENCY).type_ns(), "https://cim4.eu/ns/nc#");
    assert_eq!(bag(&ds, EQUIPMENT).type_ns(), "https://cim.ucaiug.io/ns#");
    assert_eq!(bag(&ds, CONTINGENCY).local_name(), "OrdinaryContingency");
}

#[test]
fn nc_fields_decode() {
    let ds = nc_fixture();
    let c = bag(&ds, CONTINGENCY);
    assert_eq!(
        c.get_str("IdentifiedObject.mRID"),
        Some("11111111-1111-1111-1111-111111111111")
    );
    assert_eq!(c.get_str("IdentifiedObject.name"), Some("N-1 line outage"));
    assert_eq!(c.get_bool("Contingency.normalMustStudy"), Some(true));

    let ce = bag(&ds, "urn:uuid:22222222-2222-2222-2222-222222222222");
    assert_eq!(
        ce.get_ref("ContingencyElement.Contingency"),
        Some(CONTINGENCY)
    );
    // Associations always read back as a slice, even when singular.
    assert_eq!(ce.get_refs("ContingencyEquipment.Equipment").len(), 1);
    assert_eq!(ce.get_refs("ContingencyEquipment.NoSuchAttr").len(), 0);
}

#[test]
fn nc_inherits_across_namespaces() {
    let ds = nc_fixture();
    // nc:OrdinaryContingency extends cim:Contingency, declared in the other
    // NCP namespace.
    let def = bag(&ds, CONTINGENCY).class_def();
    let super_idx = def.super_class.expect("OrdinaryContingency has a super class");
    let parent = &cimmodel::nc_classes::CLASSES[super_idx];
    assert_eq!(parent.local, "Contingency");
    assert_eq!(parent.ns, "https://cim.ucaiug.io/ns#");
}

#[test]
fn nc_equipment_is_not_cgmes_equipment() {
    let ds = nc_fixture();
    let e = bag(&ds, EQUIPMENT);
    // The attribute CGMES does not have at all.
    assert_eq!(e.get_bool("Equipment.networkAnalysisEnabled"), Some(true));
    assert_eq!(e.type_name(), "nc:Equipment");

    // ...and the CGMES struct of the same bare name is a different type with a
    // different namespace.
    let cgmes = cimmodel::Equipment::default();
    assert_eq!(cgmes.type_name(), "Equipment");
    assert_eq!(cgmes.type_ns(), "http://iec.ch/TC57/CIM100#");
}

#[test]
fn cgmes_and_nc_coexist_in_one_dataset() {
    let mut ds = nc_fixture();
    ds.merge(CimDataset::decode_file(Path::new("../testdata/test_shacl_EQ_001.xml")).unwrap());
    assert!(ds.by_type.contains_key("nc:Equipment"));
    assert!(ds.by_type.keys().any(|k| !k.starts_with("nc:")));
}

#[test]
fn unknown_prefix_falls_back_to_bare_name() {
    // An unbound namespace, and no xmlns at all: both must keep decoding as
    // CGMES, which is what every release before namespace resolution did.
    let unbound = r#"<?xml version="1.0" encoding="UTF-8"?>
<rdf:RDF xmlns:rdf="http://www.w3.org/1999/02/22-rdf-syntax-ns#"
         xmlns:foo="http://example.com/nope#">
  <foo:Terminal rdf:ID="_t1"><foo:IdentifiedObject.name>T</foo:IdentifiedObject.name></foo:Terminal>
</rdf:RDF>"#;
    let ds = CimDataset::decode_str(unbound).unwrap();
    assert_eq!(ds.by_type.get("Terminal").map(Vec::len), Some(1));

    let undeclared = r#"<?xml version="1.0" encoding="UTF-8"?>
<rdf:RDF xmlns:rdf="http://www.w3.org/1999/02/22-rdf-syntax-ns#">
  <Terminal rdf:ID="_t2"><IdentifiedObject.name>T</IdentifiedObject.name></Terminal>
</rdf:RDF>"#;
    let ds = CimDataset::decode_str(undeclared).unwrap();
    assert_eq!(ds.by_type.get("Terminal").map(Vec::len), Some(1));
}

#[test]
fn nested_xmlns_redeclaration_is_honoured() {
    // Legal RDF/XML that no ENTSO-E file writes: the element rebinds `cim` to
    // the NC vocabulary. Resolution must follow the inner binding, then revert.
    let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<rdf:RDF xmlns:rdf="http://www.w3.org/1999/02/22-rdf-syntax-ns#"
         xmlns:cim="http://iec.ch/TC57/CIM100#">
  <cim:Equipment xmlns:cim="https://cim.ucaiug.io/ns#" rdf:about="urn:uuid:a"/>
  <cim:Equipment rdf:about="urn:uuid:b"/>
</rdf:RDF>"#;
    let ds = CimDataset::decode_str(xml).unwrap();
    assert_eq!(ds.by_type.get("nc:Equipment").map(Vec::len), Some(1));
    assert_eq!(ds.by_type.get("Equipment").map(Vec::len), Some(1));
}
