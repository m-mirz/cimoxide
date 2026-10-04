//! The shape interpreter run against typed CGMES elements.
//!
//! The shapes here are written by hand rather than resolved from the CGMES
//! SHACL, so each test pins down one constraint's semantics independently of
//! the resolver. As in `nc_bag.rs`, every test starts from a dataset that
//! validates clean and introduces one defect: a test asserting an empty result
//! cannot tell a working interpreter from one that checks nothing.
//!
//! The semantics being pinned are the generated validators', since the table
//! has to reproduce them: a range or comparison is only checked when the value
//! is present and parses as a number.

use cimdecoder::CimDataset;
use cimvalidation::bag::{validate_shapes, Source};
use cimvalidation::shapes::{Check, Constraint, Path, PropShape, ShapeDef, Target};
use cimvalidation::Violation;

const UNIT: &str = "44444444-4444-4444-4444-444444444444";

/// A GeneratingUnit with the three numbers the shapes below talk about.
fn unit(min_p: &str, max_p: &str, nominal_p: &str) -> CimDataset {
    let field = |name: &str, v: &str| {
        if v.is_empty() {
            String::new()
        } else {
            format!("    <cim:GeneratingUnit.{name}>{v}</cim:GeneratingUnit.{name}>\n")
        }
    };
    let xml = format!(
        r#"<?xml version="1.0" encoding="UTF-8"?>
<rdf:RDF xmlns:rdf="http://www.w3.org/1999/02/22-rdf-syntax-ns#"
         xmlns:cim="http://iec.ch/TC57/CIM100#">
  <cim:GeneratingUnit rdf:about="urn:uuid:{UNIT}">
    <cim:IdentifiedObject.name>G1</cim:IdentifiedObject.name>
{}{}{}  </cim:GeneratingUnit>
</rdf:RDF>"#,
        field("minOperatingP", min_p),
        field("maxOperatingP", max_p),
        field("nominalP", nominal_p),
    );
    let ds = CimDataset::decode_str(&xml).expect("fixture did not decode");
    assert!(
        ds.by_type.contains_key("GeneratingUnit"),
        "fixture must decode as the typed CGMES struct, got {:?}",
        ds.by_type.keys().collect::<Vec<_>>()
    );
    ds
}

const fn check(constraint: Constraint, name: &'static str) -> Check {
    Check {
        constraint,
        rule_id: name,
        name,
        message: "",
        description: "",
        severity: "sh:Violation",
    }
}

const GU: &[Target] = &[Target::Class(&["GeneratingUnit"])];

static NOMINAL: PropShape = PropShape {
    path: Path::Forward("GeneratingUnit.nominalP"),
    checks: &[
        check(Constraint::MinExclusive(0.0), "nominalP-minExclusive"),
        check(Constraint::MaxInclusive(1000.0), "nominalP-maxInclusive"),
    ],
};

static MIN_P: PropShape = PropShape {
    path: Path::Forward("GeneratingUnit.minOperatingP"),
    checks: &[
        check(Constraint::MinInclusive(0.0), "minOperatingP-minInclusive"),
        check(Constraint::LessThanOrEquals("GeneratingUnit.maxOperatingP"), "minOperatingP-lessThanOrEquals"),
    ],
};

static MAX_P: PropShape = PropShape {
    path: Path::Forward("GeneratingUnit.maxOperatingP"),
    checks: &[
        check(Constraint::MaxExclusive(2000.0), "maxOperatingP-maxExclusive"),
        check(Constraint::LessThan("GeneratingUnit.nominalP"), "maxOperatingP-lessThan"),
    ],
};

static SHAPE: ShapeDef = ShapeDef {
    targets: GU,
    props: &[&NOMINAL, &MIN_P, &MAX_P],
    closed: None,
    logic: &[],
    profiles: &["EQ"],
    file: "test",
};

fn run(ds: &CimDataset) -> Vec<Violation> {
    validate_shapes(ds, Source::Typed, &[&SHAPE])
}

fn names(v: &[Violation]) -> Vec<&str> {
    let mut n: Vec<&str> = v.iter().map(|x| x.rule_id.as_str()).collect();
    n.sort_unstable();
    n
}

#[test]
fn a_conforming_unit_reports_nothing() {
    let v = run(&unit("10", "90", "100"));
    assert!(v.is_empty(), "expected no violations, got {v:#?}");
}

#[test]
fn inclusive_bounds_admit_the_bound_and_exclusive_ones_do_not() {
    // minOperatingP = 0 sits on an inclusive bound; nominalP = 1000 on one too.
    assert!(run(&unit("0", "90", "1000")).is_empty());
    // maxOperatingP = 2000 sits on an exclusive bound.
    let v = run(&unit("10", "2000", "1000"));
    assert!(names(&v).contains(&"maxOperatingP-maxExclusive"), "{v:#?}");
}

#[test]
fn a_value_below_the_minimum_is_flagged() {
    let ds = unit("-5", "90", "100");
    let v = run(&ds);
    assert_eq!(names(&v), ["minOperatingP-minInclusive"]);
    assert_eq!(v[0].object_id, ds.by_type["GeneratingUnit"][0]);
    assert_eq!(v[0].class, "GeneratingUnit");
    assert_eq!(v[0].property, "GeneratingUnit.minOperatingP");
}

#[test]
fn a_zero_fails_an_exclusive_minimum() {
    let v = run(&unit("0", "0", ""));
    assert!(v.is_empty(), "nominalP absent: nothing to compare, got {v:#?}");
    let v = run(&unit("10", "90", "0"));
    // nominalP = 0 also makes maxOperatingP < nominalP false.
    assert_eq!(names(&v), ["maxOperatingP-lessThan", "nominalP-minExclusive"]);
}

#[test]
fn less_than_or_equals_compares_two_fields_of_one_element() {
    assert!(run(&unit("90", "90", "100")).is_empty(), "equal is allowed by lessThanOrEquals");
    let v = run(&unit("95", "90", "100"));
    assert_eq!(names(&v), ["minOperatingP-lessThanOrEquals"]);
}

#[test]
fn less_than_is_strict() {
    let v = run(&unit("10", "100", "100"));
    assert_eq!(names(&v), ["maxOperatingP-lessThan"]);
}

/// The generated validators read an `Option<f64>`, which is `None` both when
/// the field is absent and when its text does not parse. Neither is a range
/// violation; a malformed number is `sh:datatype`'s to report.
#[test]
fn absent_or_malformed_numbers_are_not_range_violations() {
    assert!(run(&unit("", "", "")).is_empty());
    assert!(run(&unit("lots", "90", "100")).is_empty());
    assert!(run(&unit("10", "90", "1e9x")).is_empty());
}

static NAMED: PropShape = PropShape {
    path: Path::Forward("IdentifiedObject.name"),
    checks: &[check(Constraint::MaxLength(1), "name-maxLength")],
};

static BY_NAME: ShapeDef = ShapeDef {
    targets: &[Target::SubjectsOf("IdentifiedObject.name")],
    props: &[&NAMED],
    closed: None,
    logic: &[],
    profiles: &["X"],
    file: "test",
};

/// NC has `sh:targetSubjectsOf` shapes on `IdentifiedObject.name`. Run with
/// `Source::Bags`, they must not reach a named CGMES struct, or every typed
/// element in a mixed dataset would be validated against NC rules.
#[test]
fn a_source_reads_only_its_own_family() {
    let ds = unit("10", "90", "100");
    assert_eq!(names(&validate_shapes(&ds, Source::Typed, &[&BY_NAME])), ["name-maxLength"]);
    assert!(validate_shapes(&ds, Source::Bags, &[&BY_NAME]).is_empty());
    assert!(validate_shapes(&ds, Source::Bags, &[&SHAPE]).is_empty());
}

/// After `drop_blocks` a typed element has nothing left to read. Skipping it
/// would report a defective element as valid.
#[test]
#[should_panic(expected = "block was dropped")]
fn validating_after_drop_blocks_is_an_error_not_a_pass() {
    let mut ds = unit("-5", "90", "100");
    ds.drop_blocks();
    run(&ds);
}
