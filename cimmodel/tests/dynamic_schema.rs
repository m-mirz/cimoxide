//! The runtime RDFS loader must reproduce the generated class table exactly.
#![cfg(feature = "dynamic-schema")]

use std::path::Path;

use cimmodel::base::ClassDef;
use cimmodel::schema_source;

fn rdfs_dir() -> &'static Path {
    Path::new("../application-profiles-library/NCP/RDFS")
}

fn cgmes_rdfs_dir() -> &'static Path {
    Path::new("../application-profiles-library/CGMES/RDFS")
}

/// The loader and `cimgen::generator::classes_gen` build the same table by
/// separate code paths. Nothing but this test stops them drifting apart.
#[test]
fn runtime_table_matches_generated() {
    compare(schema_source::load_table("nc", rdfs_dir()).unwrap(), cimmodel::nc_classes::CLASSES);
}

/// The same for CGMES, whose class table decoding has used since the
/// generated structs went away.
#[test]
fn cgmes_runtime_table_matches_generated() {
    compare(schema_source::load_table("cgmes", cgmes_rdfs_dir()).unwrap(), cimmodel::cgmes_classes::CLASSES);
}

fn compare(dynamic: &[ClassDef], generated: &[ClassDef]) {
    assert_eq!(
        dynamic.len(),
        generated.len(),
        "class count differs: {} from RDFS, {} generated",
        dynamic.len(),
        generated.len()
    );

    for (d, g) in dynamic.iter().zip(generated.iter()) {
        assert_eq!(d.qualified, g.qualified, "class ordering differs");
        assert_eq!(d.ns, g.ns, "{}: namespace", g.qualified);
        assert_eq!(d.local, g.local, "{}: local name", g.qualified);
        // The index, not a name — a mismatch here means the class ids diverged
        // and every super_class link points somewhere else.
        assert_eq!(d.super_class, g.super_class, "{}: super class", g.qualified);
        assert_eq!(d.concrete, g.concrete, "{}: concrete", g.qualified);
        assert_eq!(d.origins, g.origins, "{}: origins", g.qualified);

        assert_eq!(
            d.attrs.len(),
            g.attrs.len(),
            "{}: attribute count",
            g.qualified
        );
        for (da, ga) in d.attrs.iter().zip(g.attrs.iter()) {
            assert_eq!(da.id, ga.id, "{}: attribute id", g.qualified);
            assert_eq!(da.ns, ga.ns, "{}.{}: namespace", g.qualified, ga.id);
            assert_eq!(da.kind, ga.kind, "{}.{}: kind", g.qualified, ga.id);
            assert_eq!(da.range, ga.range, "{}.{}: range", g.qualified, ga.id);
            assert_eq!(da.is_list, ga.is_list, "{}.{}: is_list", g.qualified, ga.id);
            assert_eq!(da.origins, ga.origins, "{}.{}: origins", g.qualified, ga.id);
        }
    }
}

#[test]
fn unknown_family_is_rejected() {
    assert!(schema_source::load_table("nope", rdfs_dir()).is_err());
}

/// A family's vocabularies are not in another family's directory.
#[test]
fn a_family_does_not_load_from_another_familys_directory() {
    assert!(schema_source::load_table("cgmes", rdfs_dir()).is_err());
}

/// `CIMOXIDE_RDFS_DIR` may list both directories; each serves its own family,
/// although NC's file pattern also matches CGMES's file names.
#[test]
fn a_directory_is_matched_to_its_family() {
    use schema_source::family_of_dir;
    assert_eq!(family_of_dir(rdfs_dir()).map(|f| f.id), Some("nc"));
    assert_eq!(family_of_dir(cgmes_rdfs_dir()).map(|f| f.id), Some("cgmes"));
    assert!(family_of_dir(Path::new("../testdata")).is_none());
}

#[test]
fn missing_directory_is_an_error_not_a_panic() {
    assert!(schema_source::load_table("nc", Path::new("/nonexistent/nope")).is_err());
}
