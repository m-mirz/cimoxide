//! SPARQL 1.1 querying over a decoded CGMES dataset.
//!
//! [`CimStore`] materialises a [`cimdecoder::CimDataset`] into an in-memory RDF store and
//! runs SPARQL against it:
//!
//! ```no_run
//! use cimdecoder::CimDataset;
//! use cimsparql::{CimStore, QueryResults};
//!
//! let ds = CimDataset::decode_file(std::path::Path::new("MicroGrid_EQ.xml"))?;
//! let store = CimStore::from_dataset(&ds)?;
//! if let QueryResults::Solutions(solutions) = store.query(
//!     "SELECT ?s ?name WHERE { ?s a cim:ACLineSegment ; cim:IdentifiedObject.name ?name }",
//! )? {
//!     for s in solutions {
//!         println!("{:?}", s?);
//!     }
//! }
//! # Ok::<_, Box<dyn std::error::Error>>(())
//! ```
//!
//! Queries are run against the default graph. Per-profile named graphs are not possible:
//! `CimEntry` records no source-file provenance.

pub mod format;
pub mod iri;
pub mod triples;

use cimdecoder::CimDataset;
use oxigraph::sparql::SparqlEvaluator;
use oxigraph::store::Store;

pub use oxigraph::model::{
    GraphName, Literal, NamedNode, NamedOrBlankNode, Quad, Term, Triple, Variable,
};
pub use oxigraph::sparql::{QueryResults, QuerySolution};
pub use triples::{GraphOptions, Stats, quads};

/// Anything that can go wrong building or querying the store.
#[derive(Debug)]
pub enum CimSparqlError {
    /// The in-memory store rejected a load or a read.
    Storage(oxigraph::store::StorageError),
    /// The query failed to parse.
    Parse(oxigraph::sparql::SparqlSyntaxError),
    /// The query parsed but failed while evaluating.
    Evaluation(oxigraph::sparql::QueryEvaluationError),
    /// Serialising results failed.
    Format(std::io::Error),
    /// A namespace from the generated tables is not a valid IRI.
    Iri(oxigraph::model::IriParseError),
}

impl std::fmt::Display for CimSparqlError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Storage(e) => write!(f, "store error: {e}"),
            Self::Parse(e) => write!(f, "invalid SPARQL: {e}"),
            Self::Evaluation(e) => write!(f, "query evaluation failed: {e}"),
            Self::Format(e) => write!(f, "could not write results: {e}"),
            Self::Iri(e) => write!(f, "invalid namespace IRI: {e}"),
        }
    }
}

impl std::error::Error for CimSparqlError {}

impl From<oxigraph::store::StorageError> for CimSparqlError {
    fn from(e: oxigraph::store::StorageError) -> Self {
        Self::Storage(e)
    }
}
impl From<oxigraph::sparql::SparqlSyntaxError> for CimSparqlError {
    fn from(e: oxigraph::sparql::SparqlSyntaxError) -> Self {
        Self::Parse(e)
    }
}
impl From<oxigraph::sparql::QueryEvaluationError> for CimSparqlError {
    fn from(e: oxigraph::sparql::QueryEvaluationError) -> Self {
        Self::Evaluation(e)
    }
}
impl From<oxigraph::model::IriParseError> for CimSparqlError {
    fn from(e: oxigraph::model::IriParseError) -> Self {
        Self::Iri(e)
    }
}
impl From<std::io::Error> for CimSparqlError {
    fn from(e: std::io::Error) -> Self {
        Self::Format(e)
    }
}

/// An in-memory RDF view of a [`CimDataset`], queryable with SPARQL.
pub struct CimStore {
    store: Store,
    stats: Stats,
}

impl CimStore {
    /// Materialise the whole dataset.
    pub fn from_dataset(ds: &CimDataset) -> Result<Self, CimSparqlError> {
        Self::from_dataset_with(ds, &GraphOptions::new())
    }

    /// Materialise the dataset under [`GraphOptions`].
    pub fn from_dataset_with(
        ds: &CimDataset,
        opts: &GraphOptions,
    ) -> Result<Self, CimSparqlError> {
        let store = Store::new()?;
        let mut stats = Stats::default();
        let mut loader = store.bulk_loader();
        loader.load_quads(quads(ds, opts, &mut stats))?;
        // The bulk loader buffers; without commit() it drops everything silently.
        loader.commit()?;
        Ok(Self { store, stats })
    }

    /// Run a query with the CGMES namespaces pre-bound, so `cim:`, `eu:`, `md:`, `dm:`,
    /// `rdf:` and `xsd:` resolve without a prologue. This is what lets an ENTSO-E
    /// `sh:select` body be pasted in unchanged. A query declaring a prefix itself wins.
    pub fn query(&self, sparql: &str) -> Result<QueryResults<'static>, CimSparqlError> {
        self.evaluate(sparql, true)
    }

    /// Run a query with no prefixes pre-bound.
    pub fn query_raw(&self, sparql: &str) -> Result<QueryResults<'static>, CimSparqlError> {
        self.evaluate(sparql, false)
    }

    fn evaluate(
        &self,
        sparql: &str,
        bind_prefixes: bool,
    ) -> Result<QueryResults<'static>, CimSparqlError> {
        let mut evaluator = SparqlEvaluator::new();
        if bind_prefixes {
            for (prefix, ns) in iri::sparql_prefixes() {
                evaluator = evaluator.with_prefix(*prefix, *ns)?;
            }
        }
        Ok(evaluator.parse_query(sparql)?.on_store(&self.store).execute()?)
    }

    /// Number of quads in the store.
    pub fn len(&self) -> Result<usize, CimSparqlError> {
        Ok(self.store.len()?)
    }

    pub fn is_empty(&self) -> Result<bool, CimSparqlError> {
        Ok(self.store.is_empty()?)
    }

    /// What materialisation saw — unmapped predicates, rebuilt blocks, totals.
    pub fn stats(&self) -> Stats {
        self.stats
    }

    /// The underlying store, for SPARQL UPDATE, dumping, or anything else oxigraph offers.
    pub fn store(&self) -> &Store {
        &self.store
    }
}

/// The namespace bindings [`CimStore::query`] pre-binds: every CGMES namespace, plus `xsd:`.
pub fn prefixes() -> &'static [(&'static str, &'static str)] {
    iri::sparql_prefixes()
}
