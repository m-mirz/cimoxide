//! NC validation against the shape table.
//!
//! Every test works from a dataset that validates clean, then introduces one
//! deliberate defect. Asserting that correct data produces nothing cannot tell
//! a working interpreter from one that checks nothing — which is the failure
//! mode this design is most exposed to, since a resolution mistake yields a
//! rule that runs and matches nothing rather than a compile error. So the clean
//! case is the baseline and every other test names the rule it expects.

use cimdecoder::CimDataset;
use cimvalidation::{validate_nc_profile, Config, Violation};

/// A conforming Contingency dataset: a contingency, the contingency element
/// that points at it (an `sh:inversePath` rule requires at least one), and the
/// equipment that element names.
///
/// The breaker is bound to the CGMES namespace on purpose. NCP's value-type
/// list names `cim17:Breaker`, which is `http://iec.ch/TC57/CIM100#` — a CGMES
/// class — so a conforming CO file references equipment across families.
fn dataset(contingency_body: &str, element_body: &str) -> CimDataset {
    let xml = format!(
        r#"<?xml version="1.0" encoding="UTF-8"?>
<rdf:RDF xmlns:rdf="http://www.w3.org/1999/02/22-rdf-syntax-ns#"
         xmlns:cim="https://cim.ucaiug.io/ns#"
         xmlns:cim17="http://iec.ch/TC57/CIM100#"
         xmlns:nc="https://cim4.eu/ns/nc#">
  <nc:OrdinaryContingency rdf:about="urn:uuid:11111111-1111-1111-1111-111111111111">
    <cim:IdentifiedObject.mRID>11111111-1111-1111-1111-111111111111</cim:IdentifiedObject.mRID>
{contingency_body}
  </nc:OrdinaryContingency>
  <cim:ContingencyEquipment rdf:about="urn:uuid:22222222-2222-2222-2222-222222222222">
    <cim:IdentifiedObject.mRID>22222222-2222-2222-2222-222222222222</cim:IdentifiedObject.mRID>
    <cim:ContingencyElement.Contingency rdf:resource="urn:uuid:11111111-1111-1111-1111-111111111111"/>
{element_body}
  </cim:ContingencyEquipment>
  <cim17:Breaker rdf:about="urn:uuid:33333333-3333-3333-3333-333333333333">
    <cim17:IdentifiedObject.mRID>33333333-3333-3333-3333-333333333333</cim17:IdentifiedObject.mRID>
  </cim17:Breaker>
</rdf:RDF>"#
    );
    CimDataset::decode_str(&xml).expect("fixture did not decode")
}

const CONTINGENCY: &str = r#"    <cim:IdentifiedObject.name>N-1 line outage</cim:IdentifiedObject.name>
    <nc:Contingency.normalMustStudy>true</nc:Contingency.normalMustStudy>"#;

const ELEMENT: &str = r#"    <cim:ContingencyEquipment.Equipment rdf:resource="urn:uuid:33333333-3333-3333-3333-333333333333"/>
    <cim:ContingencyEquipment.contingentStatus rdf:resource="https://cim.ucaiug.io/ns#ContingencyEquipmentStatusKind.outOfService"/>"#;

fn check(contingency_body: &str, element_body: &str) -> Vec<Violation> {
    validate_nc_profile(
        &dataset(contingency_body, element_body),
        "CO",
        &Config::default(),
    )
}

/// Violations of one named rule.
fn by_name<'a>(v: &'a [Violation], name: &str) -> Vec<&'a Violation> {
    v.iter().filter(|x| x.name == name).collect()
}

/// The baseline every other test departs from. If this ever starts reporting,
/// the tests below stop proving that their *own* defect is what was caught.
#[test]
fn a_conforming_dataset_reports_nothing() {
    let v = check(CONTINGENCY, ELEMENT);
    assert!(v.is_empty(), "expected no violations, got {v:#?}");
}

/// The check that only a property bag can make. Generated structs drop unknown
/// properties at decode, so by validation time there is nothing left to flag.
#[test]
fn closed_shapes_flag_a_property_outside_the_profile() {
    let v = check(
        &format!("{CONTINGENCY}\n    <nc:Contingency.notInThisProfile>7</nc:Contingency.notInThisProfile>"),
        ELEMENT,
    );
    let flagged = by_name(&v, "PropertyNotInProfile");
    assert_eq!(flagged.len(), 1, "{v:#?}");
    assert_eq!(flagged[0].property, "Contingency.notInThisProfile");
    // NC leans on advisory severity where CGMES would not: 842 sh:Info
    // occurrences against CGMES's 7.
    assert_eq!(flagged[0].severity, "sh:Info");
}

/// `sh:datatype` is a tautology against an `f64` and a real check against a
/// bag's `FieldValue::Text`. 930 NCP constraints were discarded on the typed
/// reasoning before the family gate in `simplify`.
#[test]
fn datatype_is_checked_because_a_bag_holds_text() {
    let v = check(
        r#"    <nc:Contingency.normalMustStudy>perhaps</nc:Contingency.normalMustStudy>"#,
        ELEMENT,
    );
    let flagged = by_name(&v, "Contingency.normalMustStudy-datatype");
    assert_eq!(flagged.len(), 1, "expected a boolean datatype violation, got {v:#?}");
    assert_eq!(flagged[0].severity, "sh:Violation");
}

#[test]
fn min_count_flags_a_missing_required_property() {
    let v = check(r#"    <cim:IdentifiedObject.name>N-1</cim:IdentifiedObject.name>"#, ELEMENT);
    assert_eq!(by_name(&v, "Contingency.normalMustStudy-cardinality").len(), 1, "{v:#?}");
}

/// A bag keeps repeated elements as a `TextList`, so the count is directly
/// available. The generated validators consult `duplicate_fields` instead,
/// because a typed struct collapses the repeats before they can be counted.
#[test]
fn max_count_flags_a_repeated_property() {
    let v = check(
        r#"    <cim:IdentifiedObject.name>first</cim:IdentifiedObject.name>
    <cim:IdentifiedObject.name>second</cim:IdentifiedObject.name>
    <nc:Contingency.normalMustStudy>true</nc:Contingency.normalMustStudy>"#,
        ELEMENT,
    );
    assert_eq!(by_name(&v, "IdentifiedObject.name-cardinality").len(), 1, "{v:#?}");
}

/// An `sh:inversePath` rule: a contingency needs at least one element pointing
/// at it, which no forward field on the contingency can answer.
#[test]
fn inverse_path_counts_what_points_at_an_element() {
    let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<rdf:RDF xmlns:rdf="http://www.w3.org/1999/02/22-rdf-syntax-ns#"
         xmlns:cim="https://cim.ucaiug.io/ns#"
         xmlns:nc="https://cim4.eu/ns/nc#">
  <nc:OrdinaryContingency rdf:about="urn:uuid:11111111-1111-1111-1111-111111111111">
    <cim:IdentifiedObject.mRID>11111111-1111-1111-1111-111111111111</cim:IdentifiedObject.mRID>
    <nc:Contingency.normalMustStudy>true</nc:Contingency.normalMustStudy>
  </nc:OrdinaryContingency>
</rdf:RDF>"#;
    let ds = CimDataset::decode_str(xml).unwrap();
    let v = validate_nc_profile(&ds, "CO", &Config::default());
    let flagged = by_name(&v, "Contingency.ContingencyElement-cardinality");
    assert_eq!(flagged.len(), 1, "a lone contingency should be flagged, got {v:#?}");

    // And the same contingency *with* an element is not flagged.
    assert!(by_name(&check(CONTINGENCY, ELEMENT), "Contingency.ContingencyElement-cardinality").is_empty());
}

#[test]
fn enum_values_outside_the_allowed_set_are_flagged() {
    let v = check(
        CONTINGENCY,
        r#"    <cim:ContingencyEquipment.Equipment rdf:resource="urn:uuid:33333333-3333-3333-3333-333333333333"/>
    <cim:ContingencyEquipment.contingentStatus rdf:resource="https://cim.ucaiug.io/ns#ContingencyEquipmentStatusKind.mothballed"/>"#,
    );
    // The enum rule is spelled `-datatype` by the profile: an out-of-profile
    // enumerated value and a non-IRI value share one message there.
    assert_eq!(
        by_name(&v, "ContingencyEquipment.contingentStatus-datatype").len(),
        1,
        "{v:#?}"
    );
    assert!(
        by_name(&check(CONTINGENCY, ELEMENT), "ContingencyEquipment.contingentStatus-datatype")
            .is_empty(),
        "the allowed enum value must not be flagged"
    );
}

/// `sh:path ( nc:X.y rdf:type )` — follow the association, then read the
/// referenced element's class. The allowed list mixes CGMES classes with NC
/// ones and `CimElement::type_name` answers for both, so this is the one place
/// NC validation legitimately reaches across families.
#[test]
fn ref_type_checks_the_class_of_the_referenced_element() {
    // nc:Equipment is the abstract base; the profile lists 119 concrete
    // equipment classes and this is not one of them.
    let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<rdf:RDF xmlns:rdf="http://www.w3.org/1999/02/22-rdf-syntax-ns#"
         xmlns:cim="https://cim.ucaiug.io/ns#"
         xmlns:nc="https://cim4.eu/ns/nc#">
  <nc:OrdinaryContingency rdf:about="urn:uuid:11111111-1111-1111-1111-111111111111">
    <cim:IdentifiedObject.mRID>11111111-1111-1111-1111-111111111111</cim:IdentifiedObject.mRID>
    <nc:Contingency.normalMustStudy>true</nc:Contingency.normalMustStudy>
  </nc:OrdinaryContingency>
  <cim:ContingencyEquipment rdf:about="urn:uuid:22222222-2222-2222-2222-222222222222">
    <cim:IdentifiedObject.mRID>22222222-2222-2222-2222-222222222222</cim:IdentifiedObject.mRID>
    <cim:ContingencyElement.Contingency rdf:resource="urn:uuid:11111111-1111-1111-1111-111111111111"/>
    <cim:ContingencyEquipment.Equipment rdf:resource="urn:uuid:33333333-3333-3333-3333-333333333333"/>
    <cim:ContingencyEquipment.contingentStatus rdf:resource="https://cim.ucaiug.io/ns#ContingencyEquipmentStatusKind.outOfService"/>
  </cim:ContingencyEquipment>
  <cim:Equipment rdf:about="urn:uuid:33333333-3333-3333-3333-333333333333">
    <cim:IdentifiedObject.mRID>33333333-3333-3333-3333-333333333333</cim:IdentifiedObject.mRID>
  </cim:Equipment>
</rdf:RDF>"#;
    let ds = CimDataset::decode_str(xml).unwrap();
    let v = validate_nc_profile(&ds, "CO", &Config::default());
    assert_eq!(by_name(&v, "ContingencyEquipment.Equipment-valueType").len(), 1, "{v:#?}");

    // The conforming dataset points at a CGMES Breaker, which *is* allowed.
    assert!(by_name(&check(CONTINGENCY, ELEMENT), "ContingencyEquipment.Equipment-valueType").is_empty());
}

/// Phase 1 validates one file at a time, so an association pointing out of the
/// file is normal. Reporting it here would make every cross-file reference a
/// value-type violation.
#[test]
fn a_reference_outside_the_dataset_is_not_a_value_type_violation() {
    let v = check(
        CONTINGENCY,
        r#"    <cim:ContingencyEquipment.Equipment rdf:resource="urn:uuid:99999999-9999-9999-9999-999999999999"/>
    <cim:ContingencyEquipment.contingentStatus rdf:resource="https://cim.ucaiug.io/ns#ContingencyEquipmentStatusKind.outOfService"/>"#,
    );
    assert!(by_name(&v, "ContingencyEquipment.Equipment-valueType").is_empty(), "{v:#?}");
}

/// A profile that does not import a constraint file must not run its shapes,
/// or the four shared files — imported by 17 or 18 of the 18 manifests — would
/// cost every profile everything.
#[test]
fn shapes_run_only_for_their_own_profiles() {
    let ds = dataset(CONTINGENCY, ELEMENT);
    let cfg = Config::default();
    // SensitivityMatrix does not import the Contingency constraints.
    assert!(validate_nc_profile(&ds, "SM", &cfg).is_empty());
    // An unknown code selects nothing rather than everything.
    assert!(validate_nc_profile(&ds, "NOT_A_PROFILE", &cfg).is_empty());
}

/// CGMES elements must never reach the bag interpreter: their NC-only fields
/// were dropped at decode, so anything asked of those fields would report
/// absent values that the XML did carry.
#[test]
fn cgmes_elements_are_not_validated_as_bags() {
    let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<rdf:RDF xmlns:rdf="http://www.w3.org/1999/02/22-rdf-syntax-ns#"
         xmlns:cim="http://iec.ch/TC57/CIM100#">
  <cim:Breaker rdf:about="urn:uuid:44444444-4444-4444-4444-444444444444">
    <cim:IdentifiedObject.mRID>44444444-4444-4444-4444-444444444444</cim:IdentifiedObject.mRID>
  </cim:Breaker>
</rdf:RDF>"#;
    let ds = CimDataset::decode_str(xml).unwrap();
    assert!(ds.by_type.contains_key("Breaker"), "fixture should decode as CGMES");
    for profile in ["CO", "ER", "RA"] {
        assert!(
            validate_nc_profile(&ds, profile, &Config::default()).is_empty(),
            "CGMES element was validated under NC profile {profile}"
        );
    }
}

/// The table is the deliverable; a table that quietly shrank would make every
/// test above pass while checking less.
#[test]
fn the_shape_table_is_populated() {
    let shapes = cimvalidation::nc_shapes::SHAPES;
    assert!(shapes.len() > 1500, "only {} shapes", shapes.len());
    let closed = shapes.iter().filter(|s| s.closed.is_some()).count();
    assert!(closed > 600, "only {closed} closed shapes");
    let checks: usize = shapes.iter().flat_map(|s| s.props.iter()).map(|p| p.checks.len()).sum();
    assert!(checks > 10_000, "only {checks} checks");
}

// ── profile detection ──────────────────────────────────────────────────────

/// CGMES announces its profiles in an `md:FullModel` header. NC uses DCAT: a
/// `dcat:Dataset` with `dcterms:conformsTo` naming profile IRIs, which the
/// `NCP/PROF` descriptors map to short codes.
#[test]
fn a_dcat_header_selects_the_profile() {
    let ds = CimDataset::decode_file(std::path::Path::new("../testdata/test_nc_CO_002.xml"))
        .expect("fixture did not decode");
    assert_eq!(cimvalidation::detect_nc_profiles(&ds), ["CO"]);
}

/// Without a header there is nothing to say which profile applies, so nothing
/// runs. This is why `test_nc_CO_001.xml` — written for the decoder — reports
/// no violations through the CLI.
#[test]
fn no_header_selects_no_profile() {
    let ds = CimDataset::decode_file(std::path::Path::new("../testdata/test_nc_CO_001.xml"))
        .expect("fixture did not decode");
    assert!(cimvalidation::detect_nc_profiles(&ds).is_empty());
}

/// An IRI the descriptors do not list is not an NC profile — silently mapping
/// an unknown IRI to some profile would run the wrong shapes.
#[test]
fn an_unknown_conforms_to_iri_selects_nothing() {
    let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<rdf:RDF xmlns:rdf="http://www.w3.org/1999/02/22-rdf-syntax-ns#"
         xmlns:dcat="http://www.w3.org/ns/dcat#"
         xmlns:dcterms="http://purl.org/dc/terms/">
  <dcat:Dataset rdf:about="urn:uuid:00000000-0000-0000-0000-000000000000">
    <dcterms:conformsTo rdf:resource="https://example.invalid/NotAProfile/1.0"/>
  </dcat:Dataset>
</rdf:RDF>"#;
    let ds = CimDataset::decode_str(xml).unwrap();
    assert!(cimvalidation::detect_nc_profiles(&ds).is_empty());
}

/// Both the base IRI and a version IRI identify the same profile, because a
/// dataset may declare either.
#[test]
fn base_and_version_iris_map_to_the_same_code() {
    use cimvalidation::detect::nc_profile_code;
    assert_eq!(nc_profile_code("https://ap.cim4.eu/Contingency"), Some("CO"));
    assert_eq!(nc_profile_code("https://ap.cim4.eu/Contingency/2.3"), Some("CO"));
    assert_eq!(nc_profile_code("https://ap.cim4.eu/RemedialAction/2.5"), Some("RA"));
    assert_eq!(nc_profile_code("https://example.invalid/x"), None);
}

/// The conforming fixture must stay conforming: it is the one end-to-end
/// example of a valid NC dataset in the repo.
#[test]
fn the_conforming_fixture_validates_clean() {
    let ds = CimDataset::decode_file(std::path::Path::new("../testdata/test_nc_CO_002.xml"))
        .expect("fixture did not decode");
    let v = validate_nc_profile(&ds, "CO", &Config::default());
    assert!(v.is_empty(), "{v:#?}");
}

/// DatasetMetadata states several requirements as material implication:
/// `sh:or ( [ sh:not dm:conformsToNCProfile ] [ sh:path P ; sh:minCount 1 ] )`
/// — a dataset declaring conformance to an NC profile must carry P.
fn fixture_without(property: &str) -> String {
    let xml = std::fs::read_to_string("../testdata/test_nc_CO_002.xml").expect("fixture");
    xml.lines().filter(|l| !l.contains(property)).collect::<Vec<_>>().join("\n")
}

#[test]
fn an_nc_dataset_missing_a_required_metadata_property_is_flagged() {
    let ds = CimDataset::decode_str(&fixture_without("dcterms:spatial")).expect("decode");
    let v = validate_nc_profile(&ds, "CO", &Config::default());
    // Both dcat:Dataset elements lost it.
    assert_eq!(by_name(&v, "spatial-NC-cardinality").len(), 2, "{v:#?}");
    assert_eq!(v.len(), 2, "only the removed property should be reported: {v:#?}");
}

/// The other branch of the implication: without conformance to an NC profile
/// the property is not required. Swapping the profile IRI also stops CO
/// detection, so the CO shapes are run explicitly.
#[test]
fn the_requirement_applies_only_to_nc_profiles() {
    let xml = fixture_without("dcterms:spatial")
        .replace("https://ap.cim4.eu/Contingency/2.3", "https://example.invalid/NotAProfile/1.0");
    let ds = CimDataset::decode_str(&xml).expect("decode");
    let v = validate_nc_profile(&ds, "CO", &Config::default());
    assert!(by_name(&v, "spatial-NC-cardinality").is_empty(), "{v:#?}");
}
