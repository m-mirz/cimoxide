//! The runtime RDFS loader must reproduce the generated class table exactly.
#![cfg(feature = "dynamic-schema")]

use std::path::Path;

use cimmodel::base::ClassDef;
use cimmodel::schema_source;

fn rdfs_dir() -> &'static Path {
    Path::new("../application-profiles-library/NCP/RDFS")
}

/// The loader and `cimgen::generator::classes_gen` build the same table by
/// separate code paths. Nothing but this test stops them drifting apart.
#[test]
fn runtime_table_matches_generated() {
    let dynamic: &[ClassDef] = schema_source::load_table("nc", rdfs_dir()).unwrap();
    let generated: &[ClassDef] = cimmodel::nc_classes::CLASSES;

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
    assert!(schema_source::load_table("cgmes", rdfs_dir()).is_err());
    assert!(schema_source::load_table("nope", rdfs_dir()).is_err());
}

#[test]
fn missing_directory_is_an_error_not_a_panic() {
    assert!(schema_source::load_table("nc", Path::new("/nonexistent/nope")).is_err());
}
