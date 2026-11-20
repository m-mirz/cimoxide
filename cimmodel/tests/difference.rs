//! Difference models: their statements decoded, applied to a model, computed
//! between two, and encoded back.

use cimmodel::convert::dataset_to_xml;
use cimmodel::{Block, CimDataset, Differences, FieldValue, Statement};

const CIM: &str = r#"xmlns:cim="http://iec.ch/TC57/CIM100#" xmlns:md="http://iec.ch/TC57/61970-552/ModelDescription/1#" xmlns:dm="http://iec.ch/TC57/61970-552/DifferenceModel/1#" xmlns:rdf="http://www.w3.org/1999/02/22-rdf-syntax-ns#""#;

/// A model: two breakers, A open, B closed, and a current limit.
fn model() -> CimDataset {
    CimDataset::decode_str(&format!(
        r#"<?xml version="1.0" encoding="UTF-8"?>
<rdf:RDF {CIM}>
  <cim:Breaker rdf:ID="_a">
    <cim:IdentifiedObject.name>Breaker A</cim:IdentifiedObject.name>
    <cim:Switch.normalOpen>true</cim:Switch.normalOpen>
  </cim:Breaker>
  <cim:Breaker rdf:ID="_b">
    <cim:IdentifiedObject.name>Breaker B</cim:IdentifiedObject.name>
  </cim:Breaker>
  <cim:CurrentLimit rdf:ID="_limit">
    <cim:IdentifiedObject.name>PATL</cim:IdentifiedObject.name>
    <cim:CurrentLimit.normalValue>500</cim:CurrentLimit.normalValue>
  </cim:CurrentLimit>
</rdf:RDF>"#
    ))
    .expect("decodes")
}

/// A difference model over [`model`], its blocks `body`.
fn difference(body: &str) -> CimDataset {
    CimDataset::decode_str(&format!(
        r#"<?xml version="1.0" encoding="UTF-8"?>
<rdf:RDF {CIM}>
  <dm:DifferenceModel rdf:about="urn:uuid:00000000-0000-4000-8000-0000000000d1">
    <md:Model.created>2026-01-01T00:00:00Z</md:Model.created>
{body}
  </dm:DifferenceModel>
</rdf:RDF>"#
    ))
    .expect("decodes")
}

/// Breaker A renamed and closed, the limit removed, a breaker C added.
const CHANGE: &str = r##"    <dm:reverseDifferences rdf:parseType="Statements">
      <rdf:Description rdf:about="#_a">
        <cim:IdentifiedObject.name>Breaker A</cim:IdentifiedObject.name>
        <cim:Switch.normalOpen>true</cim:Switch.normalOpen>
      </rdf:Description>
      <cim:CurrentLimit rdf:about="#_limit">
        <cim:IdentifiedObject.name>PATL</cim:IdentifiedObject.name>
        <cim:CurrentLimit.normalValue>500</cim:CurrentLimit.normalValue>
      </cim:CurrentLimit>
    </dm:reverseDifferences>
    <dm:forwardDifferences rdf:parseType="Statements">
      <rdf:Description rdf:about="#_a">
        <cim:IdentifiedObject.name>Breaker A1</cim:IdentifiedObject.name>
        <cim:Switch.normalOpen>false</cim:Switch.normalOpen>
      </rdf:Description>
      <cim:Breaker rdf:ID="_c">
        <cim:IdentifiedObject.name>Breaker C</cim:IdentifiedObject.name>
      </cim:Breaker>
    </dm:forwardDifferences>"##;

fn text(s: &str) -> FieldValue {
    FieldValue::Text(s.to_string())
}

#[test]
fn a_difference_models_statements_are_read_into_blocks_of_their_own() {
    let dm = difference(CHANGE);
    // The header alone among the objects: none of its statements.
    assert_eq!(dm.entries.len(), 1, "{:?}", dm.entries.keys().collect::<Vec<_>>());
    assert!(dm.foreign.is_empty());
    let [d] = dm.differences.as_slice() else { panic!("one header's differences") };
    assert_eq!(d.header, "urn:uuid:00000000-0000-4000-8000-0000000000d1");
    assert_eq!(d.blocks, [Block::Reverse, Block::Forward]);
    // Typed statements as objects of their class; an rdf:Description among the foreign ones.
    assert_eq!(d.reverse.entries["_limit"].type_name(), "CurrentLimit");
    assert_eq!(d.forward.entries["_c"].type_name(), "Breaker");
    assert!(d.forward.entries["_c"].defined());
    assert_eq!(d.forward.foreign["_a"].get_str("IdentifiedObject.name"), Some("Breaker A1"));
    assert_eq!(d.reverse.foreign["_a"].get_str("Switch.normalOpen"), Some("true"));
}

#[test]
fn an_empty_block_is_kept_and_states_nothing() {
    let dm = difference(r#"    <dm:reverseDifferences rdf:parseType="Statements"/>
    <dm:forwardDifferences rdf:parseType="Statements">
    </dm:forwardDifferences>"#);
    let d = &dm.differences[0];
    assert_eq!(d.blocks, [Block::Reverse, Block::Forward]);
    assert!(d.forward.entries.is_empty() && d.forward.foreign.is_empty());
    assert!(d.reverse.entries.is_empty() && d.reverse.foreign.is_empty());
}

#[test]
fn an_nc_difference_sets_statements_are_read_too() {
    let ds = CimDataset::decode_str(
        r##"<?xml version="1.0" encoding="UTF-8"?>
<rdf:RDF xmlns:cim="https://cim.ucaiug.io/ns#" xmlns:dcatcim="https://cim4.eu/ns/dcatcim#" xmlns:rdf="http://www.w3.org/1999/02/22-rdf-syntax-ns#">
  <dcatcim:DifferenceSet rdf:about="urn:uuid:00000000-0000-4000-8000-0000000000d2">
    <dcatcim:forwardDifference rdf:parseType="Statements">
      <rdf:Description rdf:about="#_x">
        <cim:IdentifiedObject.name>New</cim:IdentifiedObject.name>
      </rdf:Description>
    </dcatcim:forwardDifference>
    <dcatcim:reverseDifference rdf:parseType="Statements">
      <rdf:Description rdf:about="#_x">
        <cim:IdentifiedObject.name>Old</cim:IdentifiedObject.name>
      </rdf:Description>
    </dcatcim:reverseDifference>
  </dcatcim:DifferenceSet>
</rdf:RDF>"##,
    )
    .expect("decodes");
    let [d] = ds.differences.as_slice() else { panic!("one header's differences") };
    assert_eq!(d.header, "urn:uuid:00000000-0000-4000-8000-0000000000d2");
    assert_eq!(d.blocks, [Block::Forward, Block::Reverse]);
    assert_eq!(d.forward.foreign["_x"].get_str("IdentifiedObject.name"), Some("New"));
    assert_eq!(d.reverse.foreign["_x"].get_str("IdentifiedObject.name"), Some("Old"));
    // Written back inside its header, and read again alike.
    let again = CimDataset::decode_str(&dataset_to_xml(&ds).expect("encodes")).expect("decodes");
    let e = &again.differences[0];
    assert_eq!(e.blocks, d.blocks);
    assert_eq!(e.forward.foreign["_x"].get_str("IdentifiedObject.name"), Some("New"));
}

#[test]
fn applied_it_adds_removes_and_changes_and_says_what() {
    let mut m = model();
    let applied = m.apply(&difference(CHANGE).differences[0]);
    assert_eq!(applied.added, ["_c"]);
    assert_eq!(applied.removed.iter().map(|e| (e.mrid(), e.type_name())).collect::<Vec<_>>(), [("_limit", "CurrentLimit")]);
    assert_eq!(applied.removed[0].get_str("CurrentLimit.normalValue"), Some("500"));
    let [changed] = applied.changed.as_slice() else { panic!("{:?}", applied.changed) };
    assert_eq!(changed.id, "_a");
    assert_eq!(changed.retyped, None);
    let fields: Vec<_> = changed.fields.iter().map(|f| (f.key, f.before.clone(), f.after.clone())).collect();
    assert_eq!(
        fields,
        [
            ("IdentifiedObject.name", Some(text("Breaker A")), Some(text("Breaker A1"))),
            ("Switch.normalOpen", Some(text("true")), Some(text("false"))),
        ]
    );
    assert!(applied.unmatched.is_empty() && applied.untyped.is_empty());
    // The model after: as the difference says, its type index kept.
    assert_eq!(m.entries["_a"].get_str("IdentifiedObject.name"), Some("Breaker A1"));
    assert_eq!(m.entries["_c"].type_name(), "Breaker");
    assert!(!m.entries.contains_key("_limit"));
    let mut breakers = m.by_type["Breaker"].clone();
    breakers.sort();
    assert_eq!(breakers, ["_a", "_b", "_c"]);
    assert!(m.by_type.get("CurrentLimit").is_none_or(Vec::is_empty));
}

#[test]
fn what_does_not_hold_is_said_and_the_rest_applied() {
    let mut m = model();
    let d = difference(
        r##"    <dm:preconditions rdf:parseType="Statements">
      <rdf:Description rdf:about="#_b">
        <cim:IdentifiedObject.name>Breaker Z</cim:IdentifiedObject.name>
      </rdf:Description>
    </dm:preconditions>
    <dm:reverseDifferences rdf:parseType="Statements">
      <rdf:Description rdf:about="#_b">
        <cim:IdentifiedObject.name>Breaker Y</cim:IdentifiedObject.name>
      </rdf:Description>
      <cim:Breaker rdf:about="#_gone"/>
    </dm:reverseDifferences>
    <dm:forwardDifferences rdf:parseType="Statements">
      <rdf:Description rdf:about="#_b">
        <cim:IdentifiedObject.description>closed</cim:IdentifiedObject.description>
      </rdf:Description>
      <rdf:Description rdf:about="#_nowhere">
        <cim:IdentifiedObject.name>Untyped</cim:IdentifiedObject.name>
      </rdf:Description>
    </dm:forwardDifferences>"##,
    );
    let applied = m.apply(&d.differences[0]);
    let missed: Vec<_> = applied.unmatched.iter().map(|u| (u.block, u.subject.as_str(), u.statement.clone())).collect();
    assert_eq!(
        missed,
        [
            (Block::Preconditions, "_b", Statement::Value("IdentifiedObject.name", text("Breaker Z"))),
            (Block::Reverse, "_b", Statement::Value("IdentifiedObject.name", text("Breaker Y"))),
            (Block::Reverse, "_gone", Statement::Type("Breaker")),
            (Block::Forward, "_nowhere", Statement::Value("IdentifiedObject.name", text("Untyped"))),
        ]
    );
    // What did hold, applied.
    assert_eq!(m.entries["_b"].get_str("IdentifiedObject.description"), Some("closed"));
    assert_eq!(m.entries["_b"].get_str("IdentifiedObject.name"), Some("Breaker B"));
    assert!(applied.added.is_empty() && applied.removed.is_empty());
    assert!(!m.entries.contains_key("_nowhere") && !m.foreign.contains_key("_nowhere"));
}

#[test]
fn a_type_changed_retypes_the_object() {
    let mut m = model();
    let d = difference(
        r##"    <dm:reverseDifferences rdf:parseType="Statements">
      <cim:Breaker rdf:about="#_b"/>
    </dm:reverseDifferences>
    <dm:forwardDifferences rdf:parseType="Statements">
      <cim:Disconnector rdf:about="#_b"/>
    </dm:forwardDifferences>"##,
    );
    let applied = m.apply(&d.differences[0]);
    assert_eq!(applied.changed.len(), 1);
    assert_eq!(applied.changed[0].retyped, Some(("Breaker", "Disconnector")));
    assert_eq!(m.entries["_b"].type_name(), "Disconnector");
    assert_eq!(m.entries["_b"].get_str("IdentifiedObject.name"), Some("Breaker B"));
    assert!(m.by_type["Disconnector"].contains(&"_b".to_string()));
    assert!(!m.by_type["Breaker"].contains(&"_b".to_string()));
}

#[test]
fn an_object_stripped_of_every_type_and_value_is_removed_and_of_its_type_alone_kept() {
    let mut m = model();
    let d = difference(
        r##"    <dm:reverseDifferences rdf:parseType="Statements">
      <cim:Breaker rdf:about="#_b">
        <cim:IdentifiedObject.name>Breaker B</cim:IdentifiedObject.name>
      </cim:Breaker>
      <cim:Breaker rdf:about="#_a"/>
    </dm:reverseDifferences>"##,
    );
    let applied = m.apply(&d.differences[0]);
    assert_eq!(applied.removed.iter().map(|e| e.mrid()).collect::<Vec<_>>(), ["_b"]);
    assert_eq!(applied.untyped, ["_a"]);
    assert_eq!(m.entries["_a"].type_name(), "Breaker");
}

#[test]
fn the_difference_between_two_models_applied_to_the_first_gives_the_second() {
    let before = model();
    let mut after = model();
    after.apply(&difference(CHANGE).differences[0]);
    let d = Differences::between(&before, &after);
    assert_eq!(d.blocks, [Block::Reverse, Block::Forward]);
    // A changed object as an rdf:Description of what changed; a new and a gone one whole.
    assert_eq!(d.forward.foreign["_a"].get_str("IdentifiedObject.name"), Some("Breaker A1"));
    assert!(d.forward.entries.contains_key("_c"));
    assert!(d.reverse.entries.contains_key("_limit"));
    let mut again = model();
    let applied = again.apply(&d);
    assert!(applied.unmatched.is_empty());
    let left = Differences::between(&again, &after);
    assert!(left.forward.entries.is_empty() && left.forward.foreign.is_empty());
    assert!(left.reverse.entries.is_empty() && left.reverse.foreign.is_empty());
    // Nothing between a model and itself.
    let none = Differences::between(&before, &model());
    assert!(none.forward.entries.is_empty() && none.reverse.foreign.is_empty());
}

#[test]
fn a_difference_computed_is_written_and_read_back_alike() {
    let before = model();
    let mut after = model();
    after.apply(&difference(CHANGE).differences[0]);
    let mut d = Differences::between(&before, &after);
    d.header = "urn:uuid:00000000-0000-4000-8000-0000000000d3".to_string();
    // A header to hold it.
    let mut doc = difference("");
    doc.differences = vec![d];
    let header = doc.entries.keys().next().unwrap().clone();
    let entry = doc.entries.remove(&header).unwrap();
    doc.set("urn:uuid:00000000-0000-4000-8000-0000000000d3".to_string(), entry);
    let xml = dataset_to_xml(&doc).expect("encodes");
    assert!(xml.contains(r#"<dm:forwardDifferences rdf:parseType="Statements">"#), "{xml}");
    assert!(xml.contains(r##"<rdf:Description rdf:about="#_a">"##), "{xml}");
    let read = CimDataset::decode_str(&xml).expect("decodes");
    let mut m = model();
    let applied = m.apply(&read.differences[0]);
    assert!(applied.unmatched.is_empty(), "{:?}", applied.unmatched);
    assert!(Differences::between(&m, &after).forward.foreign.is_empty());
}

mod real {
    use super::*;

    const DIR: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../CGMES-Test-Configurations/v3.0/MicroGrid/MicroGid-BaseCase");

    fn read(path: &str) -> CimDataset {
        CimDataset::decode_file(format!("{DIR}/{path}").as_ref()).expect("decodes")
    }

    /// MicroGrid's NL difference models, each applied to the file it supersedes.
    #[test]
    fn microgrids_nl_differences_apply_to_their_models_and_round_trip() {
        for (base, diff, added, removed, changed) in [
            ("MicroGrid-NL-MAS/20210325T1530Z_1D_NL_EQ_001.xml", "MicroGrid-NL-MAS-EQ_diff/MicroGrid-NL-MAS-EQ_diff.xml", 91, 41, 88),
            ("MicroGrid-NL-MAS/20210325T1530Z_1D_NL_GL_001.xml", "MicroGrid-NL-MAS-GL_diff/MicroGrid-NL-MAS-GL_diff.xml", 0, 0, 0),
        ] {
            let dm = read(diff);
            let d = &dm.differences[0];
            let mut m = read(base);
            let original = read(base);
            let applied = m.apply(d);
            assert!(applied.unmatched.is_empty(), "{diff}: {:?}", &applied.unmatched[..applied.unmatched.len().min(5)]);
            if added + removed + changed > 0 {
                assert_eq!((applied.added.len(), applied.removed.len(), applied.changed.len()), (added, removed, changed), "{diff}");
            }
            // Written and read back, it gives the same model.
            let again = CimDataset::decode_str(&dataset_to_xml(&dm).expect("encodes")).expect("decodes");
            let mut n = read(base);
            assert!(n.apply(&again.differences[0]).unmatched.is_empty(), "{diff}");
            let left = Differences::between(&m, &n);
            assert!(left.forward.entries.is_empty() && left.forward.foreign.is_empty(), "{diff}");
            assert!(left.reverse.entries.is_empty() && left.reverse.foreign.is_empty(), "{diff}");
            // And computed again from the models, it gives the same model too.
            let mut o = read(base);
            assert!(o.apply(&Differences::between(&original, &m)).unmatched.is_empty(), "{diff}");
            let left = Differences::between(&o, &m);
            assert!(left.forward.entries.is_empty() && left.reverse.entries.is_empty(), "{diff}");
        }
    }
}
