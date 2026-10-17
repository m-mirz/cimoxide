//! A literal is kept as the XML wrote it: whitespace included, and an empty
//! element as an empty value rather than no value. RDF/XML gives a property
//! element's text as the literal, and `sh:minLength`, `sh:pattern` or a
//! datatype check judge that text, not a trimmed one.

use cimmodel::CimDataset;
use cimmodel::base::FieldValue;
use cimmodel::convert::dataset_to_xml;

fn decode(fields: &str) -> CimDataset {
    let xml = format!(
        r#"<?xml version="1.0" encoding="UTF-8"?>
<rdf:RDF xmlns:rdf="http://www.w3.org/1999/02/22-rdf-syntax-ns#"
         xmlns:cim="http://iec.ch/TC57/CIM100#">
  <cim:ACLineSegment rdf:ID="_line">
{fields}
  </cim:ACLineSegment>
</rdf:RDF>"#
    );
    CimDataset::decode_str(&xml).expect("decode")
}

fn text<'a>(ds: &'a CimDataset, key: &str) -> Option<&'a str> {
    ds.entries["_line"].get_str(key)
}

#[test]
fn surrounding_whitespace_is_kept() {
    let ds = decode("    <cim:IdentifiedObject.name>  Line 1 </cim:IdentifiedObject.name>");
    assert_eq!(text(&ds, "IdentifiedObject.name"), Some("  Line 1 "));
}

#[test]
fn an_empty_element_is_an_empty_value() {
    let ds = decode("    <cim:IdentifiedObject.name></cim:IdentifiedObject.name>");
    assert_eq!(text(&ds, "IdentifiedObject.name"), Some(""));
    let ds = decode("    <cim:IdentifiedObject.description/>");
    assert_eq!(text(&ds, "IdentifiedObject.description"), Some(""));
}

#[test]
fn a_self_closing_reference_is_still_a_reference() {
    let ds = decode(r##"    <cim:ConductingEquipment.BaseVoltage rdf:resource="#_bv"/>"##);
    assert!(matches!(
        ds.entries["_line"].get("ConductingEquipment.BaseVoltage"),
        Some(FieldValue::Resource(r)) if r == "_bv"
    ));
}

#[test]
fn text_split_by_a_comment_or_cdata_is_one_value() {
    let ds = decode("    <cim:IdentifiedObject.name>Li<!-- x -->ne <![CDATA[<1>]]> &amp; 2</cim:IdentifiedObject.name>");
    assert_eq!(text(&ds, "IdentifiedObject.name"), Some("Line <1> & 2"));
}

#[test]
fn a_language_tag_stays_with_an_empty_value() {
    let ds = decode(r#"    <cim:IdentifiedObject.description xml:lang="en"/>"#);
    assert_eq!(text(&ds, "IdentifiedObject.description"), Some(""));
    assert_eq!(ds.entries["_line"].langs("IdentifiedObject.description").collect::<Vec<_>>(), ["en"]);
}

#[test]
fn literals_survive_an_xml_round_trip() {
    let ds = decode(
        "    <cim:IdentifiedObject.name>  Line 1 </cim:IdentifiedObject.name>
    <cim:IdentifiedObject.description></cim:IdentifiedObject.description>",
    );
    let again = CimDataset::decode_str(&dataset_to_xml(&ds).expect("encode")).expect("decode");
    assert_eq!(text(&again, "IdentifiedObject.name"), Some("  Line 1 "));
    assert_eq!(text(&again, "IdentifiedObject.description"), Some(""));
}

/// `rdf:parseType="Statements"` (a difference model's `forwardDifferences`)
/// holds elements; the indentation between them is no literal.
#[test]
fn a_field_with_child_elements_has_no_text() {
    let ds = decode(
        r#"    <cim:IdentifiedObject.name>
      <cim:Breaker rdf:ID="_inner">
        <cim:IdentifiedObject.name>x</cim:IdentifiedObject.name>
      </cim:Breaker>
    </cim:IdentifiedObject.name>
    <cim:IdentifiedObject.description>
      <cim:Breaker rdf:ID="_empty"/>
    </cim:IdentifiedObject.description>"#,
    );
    assert_eq!(text(&ds, "IdentifiedObject.name"), None);
    assert_eq!(text(&ds, "IdentifiedObject.description"), None);
}

/// An empty `rdf:parseType="Statements"` block holds no statements, not a
/// whitespace literal.
#[test]
fn an_empty_statements_block_has_no_text() {
    let ds = decode(
        r#"    <cim:IdentifiedObject.name rdf:parseType="Statements">
    </cim:IdentifiedObject.name>
    <cim:IdentifiedObject.description rdf:parseType="Statements"/>"#,
    );
    assert_eq!(text(&ds, "IdentifiedObject.name"), None);
    assert_eq!(text(&ds, "IdentifiedObject.description"), None);
}
