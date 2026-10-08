//! The shape interpreter run against CGMES elements.
//!
//! The shapes here are written by hand rather than resolved from the CGMES
//! SHACL, so each test pins down one constraint's semantics independently of
//! the resolver. As in `nc_bag.rs`, every test starts from a dataset that
//! validates clean and introduces one defect: a test asserting an empty result
//! cannot tell a working interpreter from one that checks nothing.
//!
//! The semantics being pinned are the former generated validators', which the
//! table reproduces: a range or comparison is only checked when the value
//! is present and parses as a number.

use cimmodel::CimDataset;
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
        "fixture must decode as a CGMES GeneratingUnit, got {:?}",
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
    validate_shapes(ds, Source::Cgmes, &[&SHAPE])
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
/// `Source::Nc`, they must not reach a named CGMES element, or every CGMES
/// element in a mixed dataset would be validated against NC rules.
#[test]
fn a_source_reads_only_its_own_family() {
    let ds = unit("10", "90", "100");
    assert_eq!(names(&validate_shapes(&ds, Source::Cgmes, &[&BY_NAME])), ["name-maxLength"]);
    assert!(validate_shapes(&ds, Source::Nc, &[&BY_NAME]).is_empty());
    assert!(validate_shapes(&ds, Source::Nc, &[&SHAPE]).is_empty());
}

/// The classes the generated CGMES shape whose check is `rule` targets.
fn cgmes_targets(rule: &str) -> Vec<&'static str> {
    let shape = cimvalidation::cgmes_shapes::SHAPES
        .iter()
        .find(|s| s.props.iter().any(|p| p.checks.iter().any(|c| c.rule_id == rule)))
        .unwrap_or_else(|| panic!("no CGMES shape checks {rule}"));
    shape
        .targets
        .iter()
        .flat_map(|t| match t {
            Target::Class(c) => c.to_vec(),
            _ => Vec::new(),
        })
        .collect()
}

/// A concrete target class matches itself only: SHACL reaches subclass
/// instances through `rdfs:subClassOf` in the data graph, which CGMES data
/// lacks, and the APL is written that way — this shape's description exempts
/// `TapChangerControl`, a subclass of its target.
#[test]
fn a_concrete_target_class_does_not_reach_its_subclasses() {
    assert_eq!(cgmes_targets("eq452:RegulatingControl-RegulatingEquipment"), ["RegulatingControl"]);
}

/// An abstract target has no instances of its own, so it still expands to its
/// concrete descendants; a literal match would check nothing.
#[test]
fn an_abstract_target_class_expands_to_its_concrete_descendants() {
    let shape = cimvalidation::cgmes_shapes::SHAPES
        .iter()
        .find(|s| s.targets.iter().any(|t| matches!(t, Target::Class(c) if c.contains(&"Analog") && c.contains(&"Discrete"))))
        .expect("no shape on abstract Measurement");
    let classes: Vec<&str> = shape.targets.iter().flat_map(|t| match t { Target::Class(c) => c.to_vec(), _ => Vec::new() }).collect();
    assert!(classes.contains(&"Discrete") && !classes.contains(&"Measurement"), "{classes:?}");
}
