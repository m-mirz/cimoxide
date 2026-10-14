//! Objects of classes no class table knows — reference data's SKOS, PROV and
//! DCAT — are kept apart from the CIM objects and written back as read.

use cimmodel::CimDataset;
use cimmodel::FieldValue;
use cimmodel::convert::dataset_to_xml;

const RD: &str = r##"<?xml version="1.0" encoding="UTF-8"?>
<rdf:RDF xmlns:rdf="http://www.w3.org/1999/02/22-rdf-syntax-ns#"
         xmlns:cim="http://iec.ch/TC57/CIM100#"
         xmlns:dcat="http://www.w3.org/ns/dcat#"
         xmlns:dcterms="http://purl.org/dc/terms/"
         xmlns:prov="http://www.w3.org/ns/prov#"
         xmlns:skos="http://www.w3.org/2004/02/skos/core#"
         xmlns:qudt="http://qudt.org/schema/qudt/">
  <dcat:Dataset rdf:about="urn:uuid:11111111-1111-1111-1111-111111111111">
    <dcterms:type>Reference data</dcterms:type>
  </dcat:Dataset>
  <prov:Agent rdf:about="https://energy.referencedata.eu/test/party/TSO-Belgovia">
    <skos:prefLabel xml:lang="en">TSO Belgovia</skos:prefLabel>
    <prov:type>TSO</prov:type>
    <prov:hadMember rdf:resource="https://energy.referencedata.eu/NCP/1.0#collection"/>
    <qudt:value rdf:nodeID="A0"/>
  </prov:Agent>
  <qudt:QuantityValue rdf:nodeID="A0">
    <qudt:numericValue>420</qudt:numericValue>
  </qudt:QuantityValue>
  <cim:ACLineSegment rdf:ID="_line">
    <cim:IdentifiedObject.name>L1</cim:IdentifiedObject.name>
  </cim:ACLineSegment>
</rdf:RDF>"##;

const AGENT: &str = "https://energy.referencedata.eu/test/party/TSO-Belgovia";

#[test]
fn unknown_classes_are_kept_apart_by_iri() {
    let ds = CimDataset::decode_str(RD).unwrap();
    let agent = &ds.foreign[AGENT];
    assert_eq!(agent.type_name(), "http://www.w3.org/ns/prov#Agent");
    assert_eq!(agent.get_str("prefLabel"), Some("TSO Belgovia"));
    assert!(!ds.entries.contains_key(AGENT), "not a CIM object");
    assert!(ds.by_type.keys().all(|t| !t.contains("prov#")));
    assert!(ds.entries.contains_key("_line"));
}

#[test]
fn a_blank_node_is_kept_and_referenced_by_its_label() {
    let ds = CimDataset::decode_str(RD).unwrap();
    assert_eq!(ds.foreign["_:A0"].get_str("numericValue"), Some("420"));
    assert!(matches!(ds.foreign[AGENT].get("value"), Some(FieldValue::Resource(r)) if r == "_:A0"));
}

/// One local name in two vocabularies: `dcterms:type` on the dataset,
/// `prov:type` on the agent.
#[test]
fn a_field_keeps_its_namespace_per_class() {
    let xml = dataset_to_xml(&CimDataset::decode_str(RD).unwrap()).unwrap();
    assert!(xml.contains("<dcterms:type>Reference data</dcterms:type>"), "{xml}");
    assert!(xml.contains("<prov:type>TSO</prov:type>"), "{xml}");
}

#[test]
fn the_whole_dataset_is_written_back() {
    let ds = CimDataset::decode_str(RD).unwrap();
    let xml = dataset_to_xml(&ds).unwrap();
    for part in [
        r#"<prov:Agent rdf:about="https://energy.referencedata.eu/test/party/TSO-Belgovia">"#,
        r#"<skos:prefLabel xml:lang="en">TSO Belgovia</skos:prefLabel>"#,
        r#"<prov:hadMember rdf:resource="https://energy.referencedata.eu/NCP/1.0#collection"/>"#,
        r#"<qudt:value rdf:nodeID="A0"/>"#,
        r#"<qudt:QuantityValue rdf:nodeID="A0">"#,
        r#"<dcat:Dataset rdf:about="urn:uuid:11111111-1111-1111-1111-111111111111">"#,
        "ACLineSegment",
    ] {
        assert!(xml.contains(part), "missing {part} in\n{xml}");
    }
    let again = CimDataset::decode_str(&xml).unwrap();
    assert_eq!(again.foreign.len(), ds.foreign.len());
    for (id, e) in &ds.foreign {
        assert_eq!(e.to_json_value(), again.foreign[id].to_json_value(), "{id}");
        assert_eq!(e.type_name(), again.foreign[id].type_name());
    }
}

/// An element written in another namespace than its class's — CGMES files
/// write `cim:BoundaryPoint` for CIM100-European's class — goes back as written.
#[test]
fn an_element_keeps_the_namespace_it_was_written_in() {
    let xml = r##"<?xml version="1.0" encoding="UTF-8"?>
<rdf:RDF xmlns:rdf="http://www.w3.org/1999/02/22-rdf-syntax-ns#" xmlns:cim="http://iec.ch/TC57/CIM100#">
  <cim:BoundaryPoint rdf:ID="_bp"><cim:IdentifiedObject.name>BP</cim:IdentifiedObject.name></cim:BoundaryPoint>
</rdf:RDF>"##;
    let ds = CimDataset::decode_str(xml).unwrap();
    assert_eq!(ds.entries["_bp"].type_name(), "BoundaryPoint");
    let out = dataset_to_xml(&ds).unwrap();
    assert!(out.contains(r##"<cim:BoundaryPoint rdf:about="#_bp">"##), "{out}");
}

/// Merged, a foreign object described in two files joins their fields.
#[test]
fn foreign_objects_merge() {
    let mut a = CimDataset::decode_str(RD).unwrap();
    let b = CimDataset::decode_str(&RD.replace("<prov:type>TSO</prov:type>", "<prov:atLocation>Belgovia</prov:atLocation>")).unwrap();
    a.merge(b);
    assert_eq!(a.foreign[AGENT].get_str("atLocation"), Some("Belgovia"));
    assert_eq!(a.foreign[AGENT].get_str("type"), Some("TSO"));
}
