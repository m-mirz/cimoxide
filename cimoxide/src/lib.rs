//! Facade over the `cimoxide-*` crates.
//!
//! Every item lives in one of the underlying crates; this crate only re-exports them
//! under a single dependency so downstream code can write `cimoxide = "0.2"` instead of
//! listing the model, validation and SPARQL crates. Depend on the individual crates directly when you only
//! need part of the pipeline (or want to avoid pulling in oxigraph).
//!
//! ```no_run
//! use cimoxide::model::CimDataset;
//! use std::path::Path;
//!
//! let ds = CimDataset::decode_file(Path::new("EQ.xml"))?;
//! println!("{} elements", ds.entries.len());
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```

/// The CIM data model: generated typed structs, one per RDF type, the
/// streaming RDF/XML decoder producing a `CimDataset`, and conversion to
/// RDF/XML and JSON (`model::convert`).
pub use cimmodel as model;

/// SHACL validation against the ENTSO-E profile constraints.
pub use cimvalidation as validation;

/// SPARQL 1.1 querying over a decoded dataset (enabled by the `sparql` feature).
#[cfg(feature = "sparql")]
pub use cimsparql as sparql;
