//! Parser for the SHACL constraint files that accompany an application profile.
//!
//! [`ttl_import::import_ttl_file`] reads one Turtle file into
//! [`model::FileResults`]; [`simplify::simplify`] then normalises the parsed
//! constraints, dropping the ones a consumer cannot or need not act on and
//! recording why in a [`skip::SkipCollector`].
//!
//! Lives here rather than in `cimgen` so the build-time generator and the
//! runtime shape loader in `cimvalidation` parse with the same code — the same
//! reason the RDFS parser moved out of `cimgen`.

pub mod model;
pub mod simplify;
pub mod resolve;
pub mod skip;
pub mod ttl_import;
