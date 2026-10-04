//! Reports over the SHACL constraints: what the shape tables skip, and how
//! much of the `sh:sparql` side the hand-written rules cover.
//!
//! Parsing, simplification and resolution live in `cimschema::shacl`;
//! re-exported here so the generator's call sites read as one pipeline.

pub use cimschema::shacl::{model, simplify, ttl_import};

pub mod skip;
pub mod sparql_report;
pub mod ttl_report;
