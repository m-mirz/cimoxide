//! Loading the NC and CGMES shape tables from SHACL at runtime.
//!
//! Unlike `cimmodel`'s equivalent, there is no second implementation to guard
//! against: the generator and this loader both call
//! `cimschema::shacl::resolve`, so they cannot resolve differently. What these
//! tests check is the part that *is* duplicated — rendering the resolved model
//! as Rust source versus interning it into `&'static` — plus the precedence
//! and failure behaviour.
//!
//! Only one test may call `load_from`: it is gated on a process-global flag
//! that the first `nc_shapes()` call sets, and cargo runs these in threads of
//! one process. The rest use `load_table`, which installs nothing.

#![cfg(feature = "dynamic-shapes")]

use std::path::{Path, PathBuf};

use cimvalidation::shape_source::{SHACL_DIR_ENV, ShapeError, load_table};
use cimvalidation::shapes::ShapeDef;

fn shacl_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .join("NC-AP/NCP/SHACL")
}

/// A shape rendered as text, for comparing the two tables without depending on
/// pointer identity or on the string pool's numbering.
fn describe(s: &ShapeDef) -> String {
    format!(
        "{:?}|{:?}|{:?}|{:?}|{:?}|{}",
        s.targets, s.props, s.closed, s.logic, s.profiles, s.file
    )
}

/// The runtime table must match the generated one shape for shape. They share
/// their resolution, so a difference here is a rendering or interning bug —
/// the one place the two paths really do diverge.
#[test]
fn the_runtime_table_matches_the_generated_one() {
    let loaded = load_table("nc", &shacl_dir()).expect("could not load NC shapes");
    let generated = cimvalidation::nc_shapes::SHAPES;

    assert_eq!(
        loaded.len(),
        generated.len(),
        "loaded {} shapes, generated has {}",
        loaded.len(),
        generated.len()
    );

    for (i, (a, b)) in loaded.iter().zip(generated.iter()).enumerate() {
        assert_eq!(describe(a), describe(b), "shape {i} differs");
    }
}

/// The same comparison for CGMES, through the same runtime path.
#[test]
fn the_cgmes_table_matches_its_runtime_load() {
    let dir = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .join("CGMES-AP/CGMES/SHACL");
    let loaded = load_table("cgmes", &dir).expect("could not load CGMES shapes");
    let generated = cimvalidation::cgmes_shapes::SHAPES;

    assert_eq!(
        loaded.len(),
        generated.len(),
        "loaded {} shapes, generated has {}",
        loaded.len(),
        generated.len()
    );
    for (i, (a, b)) in loaded.iter().zip(generated.iter()).enumerate() {
        assert_eq!(describe(a), describe(b), "shape {i} differs");
    }
}

/// `CIMOXIDE_SHACL_DIR` may list several directories; each serves the family
/// whose files it holds, so pointing it at NC's directory alone never makes
/// CGMES try to load from there.
#[test]
fn a_directory_is_matched_to_its_family() {
    use cimvalidation::shape_source::family_of_dir;
    let repos = Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap();
    assert_eq!(
        family_of_dir(&repos.join("NC-AP/NCP/SHACL")).map(|f| f.id),
        Some("nc")
    );
    assert_eq!(
        family_of_dir(&repos.join("CGMES-AP/CGMES/SHACL")).map(|f| f.id),
        Some("cgmes")
    );
    assert_eq!(
        family_of_dir(&repos.join("CGMES-AP/CGMES/RDFS")).map(|f| f.id),
        None
    );
}

/// Copies every file of `dirs` (relative to the workspace) into `to`, flat.
fn copy_flat(dirs: &[&str], to: &Path) {
    let repos = Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap();
    let _ = std::fs::remove_dir_all(to);
    std::fs::create_dir_all(to).unwrap();
    for dir in dirs {
        for entry in std::fs::read_dir(repos.join(dir)).unwrap() {
            let path = entry.unwrap().path();
            if path.is_file() {
                std::fs::copy(&path, to.join(path.file_name().unwrap())).unwrap();
            }
        }
    }
}

/// The files say what they are, so their layout doesn't matter: NC's shapes, manifests,
/// descriptors and vocabulary in one directory, with CGMES's vocabulary beside them, load
/// to the same table as ENTSO-E's tree.
#[test]
fn a_flat_copy_loads_the_same_table() {
    let flat = Path::new(env!("CARGO_TARGET_TMPDIR")).join("flat-profiles/all");
    copy_flat(
        &[
            "NC-AP/NCP/SHACL",
            "NC-AP/NCP/SHACL/Validation",
            "NC-AP/NCP/PROF",
            "NC-AP/NCP/RDFS",
            "CGMES-AP/CGMES/RDFS",
        ],
        &flat,
    );
    assert_eq!(
        cimvalidation::shape_source::family_of_dir(&flat).map(|f| f.id),
        Some("nc")
    );
    let loaded = load_table("nc", &flat).expect("could not load the flat copy");
    let tree = load_table("nc", &shacl_dir()).expect("could not load NC shapes");
    assert_eq!(loaded.len(), tree.len());
    for (i, (a, b)) in loaded.iter().zip(tree.iter()).enumerate() {
        assert_eq!(describe(a), describe(b), "shape {i} differs");
    }
}

/// A `Validation/` directory is no mark of NC: it used to be how a directory was taken for
/// NC's, and a CGMES directory holding one was then read as NC.
#[test]
fn a_validation_directory_does_not_make_a_family() {
    let dir = Path::new(env!("CARGO_TARGET_TMPDIR")).join("cgmes-with-validation/SHACL");
    copy_flat(&["CGMES-AP/CGMES/SHACL"], &dir);
    std::fs::create_dir_all(dir.join("Validation")).unwrap();
    assert_eq!(
        cimvalidation::shape_source::family_of_dir(&dir).map(|f| f.id),
        Some("cgmes")
    );
}

/// A directory holding both families' shapes is neither's alone.
#[test]
fn a_directory_of_both_families_is_neither_alone() {
    let dir = Path::new(env!("CARGO_TARGET_TMPDIR")).join("both-families/SHACL");
    copy_flat(&["CGMES-AP/CGMES/SHACL", "NC-AP/NCP/SHACL"], &dir);
    assert!(cimvalidation::shape_source::family_of_dir(&dir).is_none());
}

#[test]
fn an_unknown_family_is_rejected() {
    assert!(matches!(
        load_table("not-a-family", &shacl_dir()),
        Err(ShapeError::UnknownFamily(_))
    ));
}

/// A bad path must be an error the caller can see, not a panic and not a
/// silent fallback at this level — `resolve` is where the warn-and-fall-back
/// policy lives, and it needs something to warn about.
#[test]
fn a_missing_directory_is_an_error_not_a_panic() {
    let err = load_table("nc", Path::new("/definitely/not/here"))
        .expect_err("a missing directory should not succeed");
    assert!(
        matches!(err, ShapeError::Parse(_)),
        "unexpected error: {err}"
    );
}

/// A directory of the wrong thing parses to nothing rather than to a partial
/// table. Serving half a profile's rules would be worse than serving none.
#[test]
fn a_directory_without_shapes_is_an_error() {
    let rdfs = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .join("NC-AP/NCP/RDFS");
    assert!(
        load_table("nc", &rdfs).is_err(),
        "a directory with no .ttl files should not produce a table"
    );
}

/// The env var names the same directory the explicit call takes, so the two
/// must agree about what is in it.
#[test]
fn the_env_var_names_the_same_directory() {
    assert_eq!(SHACL_DIR_ENV, "CIMOXIDE_SHACL_DIR");
    assert!(shacl_dir().is_dir(), "the NCP SHACL directory should exist");
}

/// The ordering guard. `nc_shapes()` memoizes, so a `load_from` afterwards
/// would be quietly ignored; it reports instead.
///
/// This is the only test that may call `load_from` — see the module comment.
#[test]
fn loading_after_the_table_is_resolved_is_reported() {
    // Force resolution, which is what a first validation would do.
    let _ = cimvalidation::nc_shapes();

    let err = cimvalidation::shape_source::load_from("nc", &shacl_dir())
        .expect_err("a load after resolution should not silently succeed");
    assert!(
        matches!(err, ShapeError::TooLate),
        "unexpected error: {err}"
    );
}
