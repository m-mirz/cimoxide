//! The profiles' `sh:sparql` constraints, read with what they need to run.

use std::path::{Path, PathBuf};

use cimschema::shacl::model::{SparqlConstraint, SparqlTarget};
use cimschema::shacl::ttl_import::import_ttl_file;

fn root() -> &'static Path {
    Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap()
}

fn ttl(dir: &str) -> Vec<PathBuf> {
    let mut paths: Vec<PathBuf> = std::fs::read_dir(root().join(dir))
        .unwrap()
        .filter_map(|e| e.ok().map(|e| e.path()))
        .filter(|p| p.extension().is_some_and(|x| x == "ttl"))
        .collect();
    paths.sort();
    paths
}

/// `a sh:SPARQLConstraint` written in a file, not commented out.
fn declared(path: &Path) -> usize {
    std::fs::read_to_string(path)
        .unwrap()
        .lines()
        .filter(|l| !l.trim_start().starts_with('#') && l.contains("sh:SPARQLConstraint"))
        .count()
}

fn constraints(dir: &str) -> Vec<(PathBuf, SparqlConstraint)> {
    ttl(dir)
        .into_iter()
        .flat_map(|path| {
            let file = import_ttl_file(&path).unwrap();
            assert_eq!(file.sparql.len(), declared(&path), "{}", path.display());
            file.sparql.into_iter().map(move |c| (path.clone(), c))
        })
        .collect()
}

#[test]
fn every_constraint_is_read_with_its_query_targets_and_prefixes() {
    for (dir, expected) in [("CGMES-AP/CGMES/SHACL", 185), ("NC-AP/NCP/SHACL", 35)] {
        let all = constraints(dir);
        assert_eq!(all.len(), expected, "{dir}");
        for (path, c) in &all {
            let at = format!("{} {}", path.display(), c.constraint_id);
            assert!(c.query.select.contains("$this"), "{at}: no $this");
            assert!(!c.targets.is_empty(), "{at}: no targets");
            assert!(
                c.query.prefixes.iter().any(|(p, _)| p == "cim"),
                "{at}: cim not declared"
            );
            assert!(
                !c.query.select.contains("$PATH") || !c.path.is_empty(),
                "{at}: $PATH without a path"
            );
            assert!(!c.rule_id.starts_with("_:"), "{at}: anonymous rule");
        }
        let undeclared: std::collections::BTreeSet<String> = all
            .iter()
            .filter(|(_, c)| !c.query.declared)
            .map(|(p, _)| p.file_name().unwrap().to_string_lossy().into_owned())
            .collect();
        eprintln!("{dir}: prefixes not declared in {undeclared:?}");
    }
}

/// A constraint reports under its shape's id, the one cimoxide's hand-written rule uses.
#[test]
fn a_constraint_reports_under_its_shape() {
    let all = constraints("CGMES-AP/CGMES/SHACL");
    let (_, c) = all
        .iter()
        .find(|(_, c)| c.rule_id == "equ:ShuntCompensator.maximumSections-numberOfInstances")
        .expect("the non-linear shunt point count");
    assert_eq!(
        c.constraint_id,
        "equ:ShuntCompensator.maximumSections-numberOfInstancesSparql"
    );
    assert_eq!(c.path, ["cim:ShuntCompensator.maximumSections"]);
    assert!(
        c.targets
            .contains(&SparqlTarget::Class("cim:NonlinearShuntCompensator".into())),
        "{:?}",
        c.targets
    );
    assert_eq!(
        c.query.prefixes.first(),
        Some(&("cim".to_string(), "http://iec.ch/TC57/CIM100#".to_string()))
    );
    assert!(
        c.message
            .starts_with("The number of NonlinearShuntCompensatorPoint")
    );
    assert_eq!(c.severity, "sh:Violation");
    assert!(c.query.declared);
}

/// Every CGMES file holding a constraint has tags, so the runner knows when it applies.
#[test]
fn every_cgmes_file_with_a_constraint_is_tagged() {
    use cimschema::shacl::cgmes_manifest::sparql_tags;
    for path in ttl("CGMES-AP/CGMES/SHACL") {
        if declared(&path) > 0 {
            let stem = path.file_stem().unwrap().to_string_lossy();
            assert!(!sparql_tags(&stem).is_empty(), "{stem} has no tag");
        }
    }
}
