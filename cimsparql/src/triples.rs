//! Turning a decoded [`CimDataset`] into RDF quads.

use std::collections::HashSet;

use cimmodel::CimDataset;
use cimmodel::base::FieldValue;
use oxigraph::model::{GraphName, Literal, NamedNode, NamedNodeRef, Quad, Term};

use crate::iri;

const RDF_TYPE: NamedNodeRef<'_> =
    NamedNodeRef::new_unchecked("http://www.w3.org/1999/02/22-rdf-syntax-ns#type");

/// Controls what [`quads`] emits.
#[derive(Debug, Clone)]
pub struct GraphOptions {
    /// Restrict materialisation to these type names. `None` emits everything.
    ///
    /// The merged RealGrid configuration is on the order of a million triples, held *in
    /// addition to* the decoded elements it was built from, so scoping the graph is the
    /// main lever on memory.
    pub include_types: Option<HashSet<String>>,
    /// Emit an `rdf:type` triple per element. On by default.
    pub emit_rdf_type: bool,
}

// Hand-written rather than derived: `bool::default()` is `false`, which would silently turn
// off `rdf:type` for anyone reaching for `GraphOptions::default()`.
impl Default for GraphOptions {
    fn default() -> Self {
        Self { include_types: None, emit_rdf_type: true }
    }
}

impl GraphOptions {
    pub fn new() -> Self {
        Self::default()
    }

    /// Restrict materialisation to the named types.
    pub fn with_types<I, S>(mut self, types: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.include_types = Some(types.into_iter().map(Into::into).collect());
        self
    }
}

/// Counts collected while materialising, so callers can tell how faithful the graph is.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Stats {
    pub quads: usize,
    pub elements: usize,
    /// Occurrences (not distinct keys) of a field key absent from `ATTR_RDF`, emitted under
    /// the fallback namespace as a plain literal. Non-zero means the generated tables and the
    /// data have drifted apart — a CIM version skew, or a third-party extension.
    pub unmapped_predicates: usize,
}

/// Streams the dataset as RDF quads in the default graph, without building a store.
///
/// Every field an element carries becomes a quad, including those its class does not
/// declare.
pub fn quads<'a>(
    ds: &'a CimDataset,
    opts: &'a GraphOptions,
    stats: &'a mut Stats,
) -> impl Iterator<Item = Quad> + 'a {
    ds.entries.iter().flat_map(move |(mrid, entry)| {
        let type_name = entry.type_name();
        if let Some(want) = &opts.include_types
            && !want.contains(type_name)
        {
            return Vec::new().into_iter();
        }

        let subject = match NamedNode::new(iri::mrid_to_iri(mrid)) {
            Ok(s) => s,
            Err(_) => return Vec::new().into_iter(),
        };

        let mut out = Vec::with_capacity(entry.fields().len() + 1);
        stats.elements += 1;

        if opts.emit_rdf_type
            && let Ok(class) = NamedNode::new(iri::type_iri(type_name))
        {
            out.push(Quad::new(subject.clone(), RDF_TYPE, class, GraphName::DefaultGraph));
        }

        for (key, value) in entry.fields() {
            let meta = iri::attr(key);
            if meta.is_none() {
                stats.unmapped_predicates += 1;
            }
            let Ok(predicate) = NamedNode::new(iri::predicate_iri(key)) else {
                continue;
            };
            for text in field_values(value) {
                let Some(object) = object_term(meta, text, ds) else {
                    continue;
                };
                out.push(Quad::new(
                    subject.clone(),
                    predicate.clone(),
                    object,
                    GraphName::DefaultGraph,
                ));
            }
        }

        stats.quads += out.len();
        out.into_iter()
    })
}

fn field_values(value: &FieldValue) -> &[String] {
    match value {
        FieldValue::Text(s) | FieldValue::Resource(s) => std::slice::from_ref(s),
        FieldValue::TextList(v) | FieldValue::ResourceList(v) => v.as_slice(),
    }
}

fn object_term(meta: Option<iri::AttrRdf>, text: &str, ds: &CimDataset) -> Option<Term> {
    match meta {
        // Typing matters: an untyped literal makes `FILTER(?r > 0.1)` compare lexically.
        Some(m) if m.kind == iri::Kind::Literal => Some(if m.range.is_empty() {
            Literal::new_simple_literal(text).into()
        } else {
            Literal::new_typed_literal(text, NamedNode::new(m.range).ok()?).into()
        }),
        // The decoder stripped the fragment off enum values; the enum namespace rebuilds it.
        Some(m) if m.kind == iri::Kind::Enum => {
            NamedNode::new(format!("{}{text}", m.range)).ok().map(Into::into)
        }
        Some(_) => NamedNode::new(iri::reference_iri(text, ds)).ok().map(Into::into),
        // Unknown predicate: keep the value rather than dropping it.
        None => Some(Literal::new_simple_literal(text).into()),
    }
}
