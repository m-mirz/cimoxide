//! Facade over the `cimoxide-*` crates.
//!
//! Every item lives in one of the underlying crates; this crate only re-exports them
//! under a single dependency so downstream code can write `cimoxide = "0.2"` instead of
//! listing four or five crates. Depend on the individual crates directly when you only
//! need part of the pipeline (or want to avoid pulling in oxigraph).
//!
//! ```no_run
//! use cimoxide::decoder::CimDataset;
//! use std::path::Path;
//!
//! let ds = CimDataset::decode_file(Path::new("EQ.xml"))?;
//! println!("{} elements", ds.entries.len());
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```

/// Generated typed structs for the CIM classes, one per RDF type.
pub use cimstructs as structs;

/// Streaming RDF/XML decoder producing a [`structs`]-typed `CimDataset`.
pub use cimdecoder as decoder;

/// SHACL validation against the ENTSO-E profile constraints.
pub use cimvalidation as validation;

/// Conversion between RDF/XML and JSON.
pub use cimconvert as convert;

/// SPARQL 1.1 querying over a decoded dataset (enabled by the `sparql` feature).
#[cfg(feature = "sparql")]
pub use cimsparql as sparql;
