//! Editing through the JSON form, as cimoxide-py's `__setitem__` does: what the
//! class table cannot say about a field, the dataset can.

use cimmodel::CimDataset;
use cimmodel::FieldValue;
use cimmodel::registry::type_registry;

const RAS: &str = r##"<?xml version="1.0" encoding="UTF-8"?>
<rdf:RDF xmlns:rdf="http://www.w3.org/1999/02/22-rdf-syntax-ns#"
         xmlns:dcat="http://www.w3.org/ns/dcat#"
         xmlns:dcterms="http://purl.org/dc/terms/">
  <dcat:Dataset rdf:about="urn:uuid:11111111-1111-1111-1111-111111111111">
    <dcterms:conformsTo rdf:resource="https://ap.cim4.eu/RemedialActionSchedule/2.4"/>
    <dcterms:description xml:lang="en">Schedules</dcterms:description>
    <dcat:version>1.0.0</dcat:version>
  </dcat:Dataset>
</rdf:RDF>"##;

const HEADER: &str = "urn:uuid:11111111-1111-1111-1111-111111111111";
const COPY: &str = "urn:uuid:22222222-2222-2222-2222-222222222222";

/// A copy of the header under a new id, through JSON.
fn copy_header(ds: &CimDataset) -> cimmodel::Element {
    let mut json = ds.entries[HEADER].to_json_value();
    json["id"] = COPY.into();
    let class = type_registry().by_type_name("nc:Dataset").unwrap();
    ds.element_from_json(class, &json).unwrap()
}

#[test]
fn an_undeclared_field_the_dataset_holds_as_a_reference_stays_one() {
    let ds = CimDataset::decode_str(RAS).unwrap();
    assert!(matches!(copy_header(&ds).get("conformsTo"), Some(FieldValue::Resource(_))));
}

/// The NC table declares `dcat:version` an association; the data writes text.
#[test]
fn a_declared_field_the_dataset_holds_as_text_stays_text() {
    let ds = CimDataset::decode_str(RAS).unwrap();
    assert!(matches!(copy_header(&ds).get("version"), Some(FieldValue::Text(t)) if t == "1.0.0"));
}

#[test]
fn language_tags_survive_json() {
    let ds = CimDataset::decode_str(RAS).unwrap();
    assert_eq!(ds.entries[HEADER].to_json_value()["_lang"]["description"][0], "en");
    assert_eq!(copy_header(&ds).langs("description").collect::<Vec<_>>(), ["en"]);
}

#[test]
fn an_edited_copy_exports_like_the_original() {
    let mut ds = CimDataset::decode_str(RAS).unwrap();
    let copy = copy_header(&ds);
    ds.set(COPY.into(), copy);
    ds.remove(HEADER);
    let xml = cimmodel::convert::dataset_to_xml_for_profile(&ds, "RAS").unwrap();
    for part in [
        r#"<dcat:Dataset rdf:about="urn:uuid:22222222-2222-2222-2222-222222222222">"#,
        r#"<dcterms:conformsTo rdf:resource="https://ap.cim4.eu/RemedialActionSchedule/2.4"/>"#,
        r#"<dcterms:description xml:lang="en">Schedules</dcterms:description>"#,
        "<dcat:version>1.0.0</dcat:version>",
    ] {
        assert!(xml.contains(part), "missing {part} in\n{xml}");
    }
}
