//! Parser for ENTSO-E RDFS application profile schemas.
//!
//! Reads the RDFS vocabularies of a profile family (CGMES, NCP) into a
//! [`model::CimSpecification`]. `cimgen` uses this at build time to generate
//! Rust sources; `cimstructs` uses it at runtime to load a class table from
//! RDFS instead of the generated one.
//!
//! [`import::import_schema_files`] parses; [`processing::postprocess`] then
//! fills in everything derived — attribute classification, profile origins and
//! namespaces — and is **not** optional for either consumer.
//!
//! [`shacl`] does the same for the SHACL constraint files: `cimgen` generates
//! validators from them, `cimvalidation` can load a shape table from them at
//! runtime.

pub mod family;
pub mod import;
pub mod model;
pub mod processing;
pub mod shacl;
