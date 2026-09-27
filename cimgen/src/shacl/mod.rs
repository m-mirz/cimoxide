//! Turns parsed SHACL shapes into generated Rust validators.
//!
//! Parsing and simplification live in `cimschema::shacl`; re-exported here so
//! the generator's call sites read as one pipeline.

pub use cimschema::shacl::{model, simplify, ttl_import};

pub mod codegen;
pub mod skip;
pub mod sparql_report;
pub mod ttl_report;
