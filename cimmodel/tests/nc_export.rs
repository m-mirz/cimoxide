//! NC export: one profile's header, elements and fields, written as read.

use cimmodel::CimDataset;
use cimmodel::convert::dataset_to_xml_for_profile;

const CO: &str = r##"<?xml version="1.0" encoding="UTF-8"?>
<rdf:RDF xmlns:rdf="http://www.w3.org/1999/02/22-rdf-syntax-ns#"
         xmlns:cim="https://cim.ucaiug.io/ns#"
         xmlns:nc="https://cim4.eu/ns/nc#"
         xmlns:dcat="http://www.w3.org/ns/dcat#"
         xmlns:dcterms="http://purl.org/dc/terms/"
         xmlns:prov="http://www.w3.org/ns/prov#">
  <dcat:Dataset rdf:about="urn:uuid:8a51756c-9f13-4294-a022-781f1abde5c2">
    <dcterms:conformsTo rdf:resource="https://ap.cim4.eu/Contingency/2.3"/>
    <dcterms:conformsTo rdf:resource="http://iec.ch/TC57/CIM100#"/>
    <dcterms:description xml:lang="en">Contingencies</dcterms:description>
    <prov:generatedAtTime rdf:datatype="http://www.w3.org/2001/XMLSchema#dateTime">2026-03-20T15:52:36Z</prov:generatedAtTime>
    <dcat:keyword>CO</dcat:keyword>
  </dcat:Dataset>
  <nc:OrdinaryContingency rdf:ID="_c1">
    <cim:IdentifiedObject.mRID>c1</cim:IdentifiedObject.mRID>
    <cim:IdentifiedObject.name> N-1 </cim:IdentifiedObject.name>
    <nc:Contingency.normalMustStudy>true</nc:Contingency.normalMustStudy>
  </nc:OrdinaryContingency>
  <cim:ContingencyEquipment rdf:about="urn:uuid:22222222-2222-2222-2222-222222222222">
    <cim:ContingencyElement.Contingency rdf:resource="#_c1"/>
    <cim:ContingencyEquipment.Equipment rdf:resource="#_line"/>
    <cim:ContingencyEquipment.contingentStatus rdf:resource="https://cim.ucaiug.io/ns#ContingencyEquipmentStatusKind.outOfService"/>
  </cim:ContingencyEquipment>
</rdf:RDF>"##;

fn export(xml: &str, code: &str) -> String {
    dataset_to_xml_for_profile(&CimDataset::decode_str(xml).expect("decode"), code).expect("export")
}

#[test]
fn an_unknown_profile_code_is_an_error() {
    let ds = CimDataset::decode_str(CO).unwrap();
    assert!(dataset_to_xml_for_profile(&ds, "NOPE").is_err());
}

#[test]
fn the_header_is_written_with_its_namespaces() {
    let xml = export(CO, "CO");
    for part in [
        r#"<dcat:Dataset rdf:about="urn:uuid:8a51756c-9f13-4294-a022-781f1abde5c2">"#,
        r#"<dcterms:conformsTo rdf:resource="https://ap.cim4.eu/Contingency/2.3"/>"#,
        r#"<dcterms:conformsTo rdf:resource="http://iec.ch/TC57/CIM100#"/>"#,
        r#"<dcterms:description xml:lang="en">Contingencies</dcterms:description>"#,
        r#"<prov:generatedAtTime rdf:datatype="http://www.w3.org/2001/XMLSchema#dateTime">2026-03-20T15:52:36Z</prov:generatedAtTime>"#,
        r#"xmlns:prov="http://www.w3.org/ns/prov#""#,
    ] {
        assert!(xml.contains(part), "missing {part} in\n{xml}");
    }
}

#[test]
fn identifiers_and_values_keep_their_form() {
    let xml = export(CO, "CO");
    for part in [
        r#"<nc:OrdinaryContingency rdf:ID="_c1">"#,
        r#"<cim:IdentifiedObject.name> N-1 </cim:IdentifiedObject.name>"#,
        r#"<cim:ContingencyEquipment rdf:about="urn:uuid:22222222-2222-2222-2222-222222222222">"#,
        r##"<cim:ContingencyElement.Contingency rdf:resource="#_c1"/>"##,
        r##"<cim:ContingencyEquipment.Equipment rdf:resource="#_line"/>"##,
        r#"rdf:resource="https://cim.ucaiug.io/ns#ContingencyEquipmentStatusKind.outOfService""#,
    ] {
        assert!(xml.contains(part), "missing {part} in\n{xml}");
    }
}

#[test]
fn an_export_decodes_to_the_same_dataset() {
    let ds = CimDataset::decode_str(CO).unwrap();
    let again = CimDataset::decode_str(&dataset_to_xml_for_profile(&ds, "CO").unwrap()).unwrap();
    assert_eq!(ds.entries.len(), again.entries.len());
    for (mrid, e) in &ds.entries {
        let f = &again.entries[mrid];
        assert_eq!(e.type_name(), f.type_name());
        assert_eq!(e.to_json_value(), f.to_json_value(), "{mrid}");
        assert_eq!(e.defined(), f.defined(), "{mrid}");
    }
    assert_eq!(ds.field_forms, again.field_forms);
}

/// Another profile's export of the same data holds only its header.
#[test]
fn elements_of_another_profile_are_left_out() {
    let xml = export(CO, "AE");
    assert!(!xml.contains("Contingency rdf:"), "{xml}");
    assert!(xml.contains(r#"<dcterms:conformsTo rdf:resource="https://ap.cim4.eu/AssessedElement/2.4"/>"#), "{xml}");
}

/// A reference into another vocabulary is written back in it: the decoder
/// keeps `Switch.open` and where it was cut from.
#[test]
fn a_property_reference_keeps_its_namespace() {
    let xml = format!(
        "{}{}",
        CO.trim_end_matches("</rdf:RDF>"),
        r#"  <nc:StaticPropertyRange rdf:ID="_r">
    <nc:StaticPropertyRange.PropertyReference rdf:resource="http://iec.ch/TC57/CIM100#Switch.open"/>
  </nc:StaticPropertyRange>
</rdf:RDF>"#
    );
    let out = export(&xml, "RA");
    assert!(out.contains(r#"rdf:resource="http://iec.ch/TC57/CIM100#Switch.open""#), "{out}");
}

/// CGMES content in the dataset belongs to the CGMES profiles.
#[test]
fn cgmes_elements_are_not_written() {
    let xml = format!(
        "{}{}",
        CO.trim_end_matches("</rdf:RDF>"),
        r#"  <cim17:ACLineSegment xmlns:cim17="http://iec.ch/TC57/CIM100#" rdf:ID="_line">
    <cim17:IdentifiedObject.name>L1</cim17:IdentifiedObject.name>
  </cim17:ACLineSegment>
</rdf:RDF>"#
    );
    assert!(!export(&xml, "CO").contains("ACLineSegment"));
}
