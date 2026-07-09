// TTL-side half of the SPARQL Check Coverage report: classifies each CGMES SHACL TTL file
// into the same profile groups as sparql_report.rs's GROUPS, and counts the distinct
// sh:SPARQLConstraintComponent shapes each group actually defines — the denominator for "how
// many of the SPARQL constraints in the standard are implemented", as opposed to how many
// hand-written Rust functions exist. Combined with sparql_report::GroupReport's names
// (matched against sh:name) in combine_coverage to produce a genuine
// Implemented/Total/Coverage figure per group.

use super::model::{FileResults, ShapeInfo};
use super::sparql_report::GroupReport;
use std::collections::{HashMap, HashSet};

/// Classifies a SHACL TTL file's base name into one of GROUPS' labels, by simple substring
/// match on the CGMES profile name embedded in the filename (e.g.
/// "61970-452_Equipment-AP-Con-Complex-SHACL.ttl"). Order matters: check the more specific
/// profile names first. "CIMdesk quality" has no TTL backing and is never returned here.
/// "Equipment" also matches "EquipmentBoundary" files, mirroring how equipment_boundary.rs is
/// grouped under Equipment (EQ) on the hand-written side. Used by both the "Generated SHACL
/// Rules by Profile" table (via the caller in main.rs) and the SPARQL Check Coverage table
/// (via ttl_sparql_names below), so their rows line up: "Common / AllProfiles" absorbs "C:600
/// conformance" (both are cross-cutting, not tied to one profile) as well as
/// GeographicalLocation/the plain Header file (neither has its own profile group on the
/// hand-written side either), and Topology/DiagramLayout/Operation each get their own row
/// instead of being bundled into a generic "Others".
pub fn ttl_group_label(filename: &str) -> &'static str {
    if filename.contains("Equipment") {
        "Equipment (EQ)"
    } else if filename.contains("SteadyStateHypothesis") {
        "Steady State Hypothesis (SSH)"
    } else if filename.contains("Dynamics") {
        "Dynamics (DY)"
    } else if filename.contains("StateVariables") {
        "State Variables (SV)"
    } else if filename.contains("ShortCircuit") {
        "Short Circuit (SC)"
    } else if filename.contains("Topology") {
        "Topology (TP)"
    } else if filename.contains("DiagramLayout") {
        "DiagramLayout (DL)"
    } else if filename.contains("Operation") {
        "Operation (OP)"
    } else {
        // Prof10, AllProfiles, IdentifiedObjectCommon, GeographicalLocation, the plain
        // Header file, ... -- everything cross-cutting or without its own hand-written
        // profile group folds into the catch-all "Common / AllProfiles" bucket.
        "Common / AllProfiles"
    }
}

/// Every label `ttl_group_label` can return, in the fixed display order used for
/// per-profile report tables (HashMap iteration order is randomized, so callers printing
/// one row per group need this).
pub const TTL_GROUP_LABEL_ORDER: &[&str] = &[
    "Equipment (EQ)",
    "Steady State Hypothesis (SSH)",
    "Dynamics (DY)",
    "State Variables (SV)",
    "Short Circuit (SC)",
    "Common / AllProfiles",
    "Topology (TP)",
    "DiagramLayout (DL)",
    "Operation (OP)",
];

/// Splits `s` on "|" and inserts each non-empty part into `out`. A single shape's sh:name can
/// itself be a "|"-joined compound of several rule names when one SPARQL constraint enforces
/// multiple named conformance rules at once (e.g. one shape in
/// IdentifiedObjectCommon_AP-Con-Complex-SHACL.ttl covers
/// "C:301:EQ:IdentifiedObject.shortName:stringLength|C:301:EQBD:...|...") -- this is how the
/// standard itself expresses it, not a formatting quirk, and the hand-written implementation
/// may legitimately cover only one of the joined names in a single Violation. Empty parts (an
/// upstream authoring gap in at least one shape) are skipped.
fn add_name(out: &mut HashSet<String>, s: &str) {
    for part in s.split('|') {
        if !part.is_empty() {
            out.insert(part.to_string());
        }
    }
}

fn collect(group: &'static str, shape: &ShapeInfo, out: &mut HashMap<&'static str, HashSet<String>>) {
    for c in &shape.constraints {
        if c.component == "sh:SPARQLConstraintComponent" {
            add_name(out.entry(group).or_default(), &shape.name);
            break;
        }
    }
    for p in &shape.properties {
        collect(group, p, out);
    }
}

/// Walks every already-parsed TTL file's shapes and collects the sh:name of every shape
/// carrying at least one sh:SPARQLConstraintComponent constraint, grouped by ttl_group_label.
/// A shape can appear more than once in the parsed tree (once per resolved concrete target
/// class); the returned sets dedupe by name. Matching on sh:name (a plain string) rather than
/// the shape's IRI (`rule_id`) sidesteps namespace-prefix normalization entirely -- the
/// hand-written Rust rule_id's prefix can legitimately differ from ttl_import.rs's per-file
/// canonicalization in edge cases, but sh:name is copied verbatim into Violation.name on both
/// sides.
pub fn ttl_sparql_names(results: &[FileResults]) -> HashMap<&'static str, HashSet<String>> {
    let mut out: HashMap<&'static str, HashSet<String>> = HashMap::new();
    for fr in results {
        let group = ttl_group_label(&fr.file_name);
        for shape in &fr.shapes {
            collect(group, shape, &mut out);
        }
    }
    out
}

/// One row of the combined TTL-vs-hand-written report.
pub struct CoverageRow {
    pub label: &'static str,
    /// Distinct sh:names implemented that are also defined in the TTL.
    pub implemented: usize,
    /// Distinct SPARQL constraint shapes defined in the TTL for this group; `None` if the
    /// group has no TTL backing (CIMdesk quality).
    pub ttl_total: Option<usize>,
    /// TTL sh:names with no matching implemented name, sorted.
    pub missing: Vec<String>,
}

/// Matches each profile group's implemented sh:names against the TTL-derived constraint sets,
/// producing the Implemented/Total/Coverage figures that replace the old hand-maintained
/// "SPARQL Constraints | Implemented | Coverage" table.
pub fn combine_coverage(
    groups: &[GroupReport],
    ttl: &HashMap<&'static str, HashSet<String>>,
) -> Vec<CoverageRow> {
    groups
        .iter()
        .map(|g| {
            let implemented: HashSet<String> = g.names.iter().cloned().collect();
            match ttl.get(g.label) {
                None => CoverageRow {
                    label: g.label,
                    implemented: implemented.len(),
                    ttl_total: None,
                    missing: Vec::new(),
                },
                Some(set) => {
                    let mut missing: Vec<String> =
                        set.difference(&implemented).cloned().collect();
                    missing.sort();
                    let implemented_count = set.intersection(&implemented).count();
                    CoverageRow {
                        label: g.label,
                        implemented: implemented_count,
                        ttl_total: Some(set.len()),
                        missing,
                    }
                }
            }
        })
        .collect()
}
