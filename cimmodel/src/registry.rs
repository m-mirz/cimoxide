//! The class registry the decoder resolves elements against.
//!
//! Built from the families' class tables — the generated ones, or tables read
//! from RDFS at runtime (see [`crate::schema_source`]).

use std::sync::OnceLock;

pub use crate::base::TypeRegistry;

/// Memoized registry of every family. Built once per process.
pub fn type_registry() -> &'static TypeRegistry {
    static R: OnceLock<TypeRegistry> = OnceLock::new();
    R.get_or_init(|| {
        let mut reg = TypeRegistry::new();
        // CGMES alone takes the bare-name fallback: an unbound prefix cannot
        // tell the families apart, and CGMES is the historical default.
        reg.add_family(crate::schema_source::resolve("cgmes", crate::cgmes_classes::CLASSES), true);
        reg.add_family(crate::schema_source::resolve("nc", crate::nc_classes::CLASSES), false);
        reg
    })
}
