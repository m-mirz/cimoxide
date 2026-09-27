//! The SHACL components NCP uses and CGMES does not.
//!
//! `sh:closed`, `sh:ignoredProperties`, `sh:alternativePath`,
//! `sh:qualifiedValueShape` and `sh:deactivated` appear nowhere in the CGMES
//! constraint files, so the CGMES codegen hash cannot detect a regression in
//! any of them. These tests are the only thing that can.

use std::path::{Path, PathBuf};

use cimschema::family;
use cimschema::shacl::model::{FileResults, ShapeInfo};
use cimschema::shacl::{simplify, ttl_import};

fn ncp_shacl(name: &str) -> PathBuf {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap();
    root.join("application-profiles-library/NCP/SHACL").join(name)
}

fn parse(name: &str) -> FileResults {
    ttl_import::import_ttl_file(&ncp_shacl(name))
        .unwrap_or_else(|e| panic!("cannot parse {name}: {e}"))
}

/// Parse inline Turtle, for constructs the shipped NCP files do not expose in
/// a reachable position.
fn parse_str(name: &str, ttl: &str) -> FileResults {
    let path = std::env::temp_dir().join(format!("cimoxide-shacl-{name}.ttl"));
    std::fs::write(&path, ttl).expect("cannot write fixture");
    let fr = ttl_import::import_ttl_file(&path).expect("cannot parse fixture");
    let _ = std::fs::remove_file(&path);
    fr
}

fn shape<'a>(fr: &'a FileResults, id: &str) -> &'a ShapeInfo {
    fr.shapes
        .iter()
        .find(|s| s.id == id)
        .unwrap_or_else(|| panic!("no shape {id} in {}", fr.file_name))
}

/// `sh:closed` is the constraint a property bag can express and generated
/// structs cannot: unknown properties are dropped at decode, so there is
/// nothing left to flag. The allowed set has to come off the NodeShape's
/// constraint-less `[ sh:path X ]` blank nodes, which the property-shape
/// builder discards for having nothing to check.
#[test]
fn closed_shape_collects_its_allowed_properties() {
    let fr = parse("Contingency-AP-Con-Simple-SHACL.ttl");
    let s = shape(&fr, "co:OrdinaryContingency-AllowedProperties");

    let allowed = s.closed.as_ref().expect("sh:closed true was not picked up");
    assert_eq!(
        allowed,
        &[
            "cim:IdentifiedObject.description",
            "cim:IdentifiedObject.mRID",
            "cim:IdentifiedObject.name",
            "nc:Contingency.EquipmentOperator",
            "nc:Contingency.SimulationEvents",
            "nc:Contingency.normalMustStudy",
            "nc:Contingency.normalProbability",
            // sh:ignoredProperties
            "rdf:type",
        ]
    );
    assert_eq!(
        s.targets.iter().map(|t| t.value.as_str()).collect::<Vec<_>>(),
        ["nc:OrdinaryContingency"]
    );
}

#[test]
fn a_shape_without_sh_closed_has_no_allowed_set() {
    let fr = parse("Contingency-AP-Con-Complex-SHACL.ttl");
    assert!(shape(&fr, "coc:ContingencyEquipment").closed.is_none());
}

/// Encoded as one `"|a|b|c"` segment. Branch order follows the file, because
/// the shape's own message enumerates the alternatives in that order.
#[test]
fn alternative_path_is_one_segment_per_shape() {
    let fr = parse("AssessedElement-AP-Con-Complex-SHACL.ttl");
    let alt = fr
        .shapes
        .iter()
        .flat_map(|s| &s.properties)
        .find_map(|p| p.path.first().filter(|seg| seg.starts_with('|')))
        .expect("no alternativePath segment found");

    let branches: Vec<&str> = alt.trim_start_matches('|').split('|').collect();
    assert_eq!(
        branches,
        [
            "nc:AssessedElement.AssessedPowerTransferCorridor",
            "nc:AssessedElement.ConductingEquipment",
            "nc:AssessedElement.DCTieCorridor",
            "nc:AssessedElement.Line",
            "nc:AssessedElement.OperationalLimit",
        ]
    );
}

/// A branch of an alternative path can itself be an inverse path, so the two
/// encodings nest: `"|^nc:A.b|nc:C.d"`.
#[test]
fn an_alternative_branch_can_be_an_inverse_path() {
    let fr = parse("StateInstructionSchedule-AP-Con-Complex-SHACL.ttl");
    let nested = fr
        .shapes
        .iter()
        .flat_map(|s| &s.properties)
        .filter_map(|p| p.path.first())
        .find(|seg| seg.starts_with('|') && seg.contains("|^"))
        .expect("no alternativePath with an inverse branch");

    assert_eq!(
        nested,
        "|^nc:PowerShiftKeyDistribution.PowerShiftKeySchedule\
         |nc:PowerShiftKeySchedule.ParticipationFactorTimePoint"
    );
}

/// Written against a fixture rather than the NCP file, because NCP's three
/// `sh:qualifiedValueShape` shapes are **unreachable** — see
/// [`qualified_shapes_in_ncp_are_unreachable_by_design`].
#[test]
fn qualified_min_count_carries_its_nested_shape() {
    let fr = parse_str(
        "qualified",
        r#"
@prefix sh:   <http://www.w3.org/ns/shacl#> .
@prefix rdf:  <http://www.w3.org/1999/02/22-rdf-syntax-ns#> .
@prefix dct:  <http://purl.org/dc/terms/> .
@prefix ex:   <http://example.org/> .

ex:Shape  a  sh:NodeShape ;
        sh:targetClass  ex:Dataset ;
        sh:property     ex:conformsToOneOf .

ex:conformsToOneOf
        a                       sh:PropertyShape ;
        sh:path                 dct:conformsTo ;
        sh:qualifiedValueShape  [ sh:in ( <https://ap.cim4.eu/Contingency/2.3>
                                          <https://ap.cim4.eu/MonitoringArea/2.3> ) ] ;
        sh:qualifiedMinCount    1 .
"#,
    );

    let c = fr
        .shapes
        .iter()
        .flat_map(|s| &s.properties)
        .flat_map(|p| &p.constraints)
        .find(|c| c.component == "sh:QualifiedMinCountConstraintComponent")
        .expect("no sh:qualifiedMinCount constraint");

    assert_eq!(c.payload["qualifiedMinCount"].as_int(), Some(1));
    let branch = c.payload["shape"].as_shapes().expect("nested shape missing");
    let values = branch[0]
        .iter()
        .find(|b| b.component == "sh:InConstraintComponent")
        .and_then(|b| b.payload.get("in"))
        .and_then(|v| v.as_list())
        .expect("nested sh:in missing");
    assert!(
        values.iter().any(|v| v.contains("ap.cim4.eu/Contingency")),
        "expected the profile IRI list, got {values:?}"
    );
}

/// All three of NCP's `sh:qualifiedValueShape` shapes are reached only through
/// `sh:or ( [ sh:not <named shape> ] [ … ] )` — material implication: *if* the
/// dataset conforms to one of these profiles, *then* the other branch's
/// property is required.
///
/// The parser does not follow `sh:not` to a named shape, so these stay
/// unreachable, and that is deliberate. Reaching them without implication
/// semantics would turn the antecedent into a standalone rule and report
/// "this dataset must conform to an NC profile" against every dataset that
/// does not — strictly worse than not checking. Honouring them properly needs
/// shape references in the IR, which is its own change.
///
/// This test pins the gap so it is a recorded decision rather than a surprise.
#[test]
fn qualified_shapes_in_ncp_are_unreachable_by_design() {
    let fr = parse("DatasetMetadata-AP-Con-SHACL.ttl");
    assert!(
        !fr.shapes
            .iter()
            .flat_map(|s| &s.properties)
            .flat_map(|p| &p.constraints)
            .any(|c| c.component == "sh:QualifiedMinCountConstraintComponent"),
        "NCP's qualified shapes became reachable — the interpreter must now \
         implement or(not(P), Q) as an implication, or it will report the \
         antecedent as a violation"
    );
}

/// Two shapes carry `sh:deactivated true`, and one of them would otherwise
/// contribute a live `sh:in` check on `rdf:type`. Ignoring the flag enforces a
/// rule the schema explicitly switched off.
#[test]
fn deactivated_shapes_are_dropped_and_accounted_for() {
    let mut results = vec![parse("DatasetMetadata-AP-Con-SHACL.ttl")];

    let before: usize = results[0]
        .shapes
        .iter()
        .flat_map(|s| &s.properties)
        .filter(|p| p.deactivated)
        .flat_map(|p| &p.constraints)
        .count();
    assert!(before > 0, "fixture no longer contains a deactivated shape");

    let skips = simplify::simplify(&mut results, &family::NC);

    assert!(
        !results[0]
            .shapes
            .iter()
            .flat_map(|s| &s.properties)
            .any(|p| p.deactivated),
        "a deactivated shape survived simplification"
    );
    let reported = skips
        .iter()
        .flat_map(|(_, entries)| entries)
        .filter(|e| e.reason.starts_with("sh:deactivated"))
        .count();
    assert!(reported > 0, "deactivated drops were not reported as skips");
}

/// The family gate. `sh:nodeKind` and `sh:datatype` are tautologies against a
/// generated struct and real checks against a property bag, so the same file
/// must simplify differently for the two families.
#[test]
fn type_system_rules_apply_to_typed_families_only() {
    let count = |family| {
        let mut results = vec![parse("Contingency-AP-Con-Simple-SHACL.ttl")];
        simplify::simplify(&mut results, family);
        results[0]
            .shapes
            .iter()
            .flat_map(|s| &s.properties)
            .flat_map(|p| &p.constraints)
            .filter(|c| {
                c.component == "sh:NodeKindConstraintComponent"
                    || c.component == "sh:DatatypeConstraintComponent"
            })
            .count()
    };

    let bag = count(&family::NC);
    let typed = count(&family::CGMES);
    assert!(
        bag > typed,
        "bag family kept {bag} nodeKind/datatype constraints, typed kept {typed} — \
         the gate is not doing anything"
    );
    assert_eq!(typed, 0, "typed family should drop all of them on this file");
}

/// The manifests are the authority on which shapes apply to which profile.
/// CGMES has no equivalent, so this mapping is the one part of NC validation
/// that replaces hand-written dispatch with data.
#[test]
fn manifests_map_profiles_to_constraint_files() {
    let dir = ncp_shacl("Validation");
    let mut manifests: Vec<ttl_import::Manifest> = std::fs::read_dir(&dir)
        .expect("no Validation directory")
        .filter_map(Result::ok)
        .map(|e| e.path())
        .filter(|p| p.extension().and_then(|x| x.to_str()) == Some("ttl"))
        .map(|p| ttl_import::import_manifest(&p).unwrap_or_else(|e| panic!("{p:?}: {e}")))
        .collect();
    manifests.sort_by(|a, b| a.profile.cmp(&b.profile));

    assert_eq!(manifests.len(), 18, "expected one manifest per NC profile");

    let co = manifests
        .iter()
        .find(|m| m.profile == "CO")
        .expect("no Contingency manifest");
    assert_eq!(
        co.imports,
        [
            "Contingency-AP-Con-Simple-SHACL",
            "DatasetMetadata-AP-Con-SHACL",
            "NC-AP-Con-ClassCount-Complex-SHACL",
            "NC-AP-Con-Complex-IdentifiedObjecStringLength-SHACL",
            "NC-AP-Con-PrefixDeclaration-Complex-SHACL",
        ]
    );

    // The shared files are why a shape belongs to many profiles, not one.
    let shared = manifests
        .iter()
        .filter(|m| m.imports.iter().any(|i| i == "DatasetMetadata-AP-Con-SHACL"))
        .count();
    assert!(shared >= 17, "expected DatasetMetadata to be near-universal, got {shared}");
}

/// `NCP/PROF` is the authority on profile identity: a dataset declares
/// `dcterms:conformsTo <some IRI>`, and only these descriptors say which short
/// code that IRI means.
#[test]
fn profile_descriptors_map_iris_to_codes() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap();
    let profiles = cimschema::import::import_profile_index(
        &root.join("application-profiles-library/NCP/PROF"),
    )
    .expect("cannot read NCP/PROF");

    assert_eq!(profiles.len(), 18, "expected one descriptor per NC profile");

    let co = profiles
        .iter()
        .find(|p| p.keyword == "CO")
        .expect("no Contingency descriptor");
    assert!(co.iris.contains(&"https://ap.cim4.eu/Contingency".to_string()));
    assert!(co.iris.contains(&"https://ap.cim4.eu/Contingency/2.3".to_string()));

    // Codes must be unique, or a conformsTo IRI would map ambiguously.
    let mut codes: Vec<&str> = profiles.iter().map(|p| p.keyword.as_str()).collect();
    codes.sort_unstable();
    let before = codes.len();
    codes.dedup();
    assert_eq!(codes.len(), before, "duplicate profile codes: {codes:?}");
}
