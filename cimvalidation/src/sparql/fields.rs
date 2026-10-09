//! An element's attributes read by type: a number, a flag, a reference.
//!
//! The values are those of the CIM attribute types — the same the generated
//! structs held while there were any, rule for rule, including their quirks:
//! a repeated scalar keeps its last value, a boolean is `true` only for the
//! text `true`, a numeric field parses into its own type, and a single-valued
//! reference written twice is absent.

use cimmodel::base::FieldValue;
use cimmodel::{CimDataset, Element};

/// One element's fields. Values borrow from the dataset, not from this view,
/// so they outlive it.
#[derive(Clone, Copy)]
pub(crate) struct Fields<'a> {
    element: &'a Element,
}

impl<'a> Fields<'a> {
    /// The fields of `entry`, whatever its class.
    pub(crate) fn of(entry: &'a Element) -> Self {
        Self { element: entry }
    }

    /// The fields of `entry` if it is an instance of exactly `class`. A
    /// subclass does not match: the rules name each class they apply to.
    pub(crate) fn of_class(entry: &'a Element, class: &str) -> Option<Self> {
        (entry.type_name() == class).then(|| Self::of(entry))
    }

    /// The fields of the element `mrid` in `dataset`, if it is exactly `class`.
    pub(crate) fn get(dataset: &'a CimDataset, mrid: &str, class: &str) -> Option<Self> {
        Self::of_class(dataset.entries.get(mrid)?, class)
    }

    /// The element's mRID (its `rdf:ID` / `rdf:about`).
    pub(crate) fn id(&self) -> &'a str {
        self.element.mrid()
    }

    /// Whether the element carries `key` at all, with any value: the SPARQL's
    /// `EXISTS { $this p ?o }`, or a bound `OPTIONAL`.
    pub(crate) fn has(&self, key: &str) -> bool {
        self.element.fields().contains_key(key)
    }

    fn scalar(&self, key: &str) -> Option<&'a str> {
        match self.element.fields().get(key)? {
            FieldValue::Text(s) => Some(s),
            FieldValue::TextList(v) => v.last().map(String::as_str),
            _ => None,
        }
    }

    /// A `String` attribute: the text as written, `""` when absent.
    pub(crate) fn text(&self, key: &str) -> &'a str {
        self.scalar(key).unwrap_or("")
    }

    /// A list-valued text attribute (`Vec<String>`), each value trimmed.
    pub(crate) fn texts(&self, key: &str) -> Vec<&'a str> {
        match self.element.fields().get(key) {
            Some(FieldValue::Text(s)) => vec![s.trim()],
            Some(FieldValue::TextList(v)) => v.iter().map(|s| s.trim()).collect(),
            _ => Vec::new(),
        }
    }

    /// A floating-point attribute; `None` when absent or unparsable.
    pub(crate) fn f64(&self, key: &str) -> Option<f64> {
        self.scalar(key)?.trim().parse().ok()
    }

    /// An integer attribute; `None` when absent or not an integer (`1.0` is not).
    pub(crate) fn i64(&self, key: &str) -> Option<i64> {
        self.scalar(key)?.trim().parse().ok()
    }

    /// A boolean attribute: `Some(true)` only for the text `true`, any other
    /// text is `Some(false)`.
    pub(crate) fn bool(&self, key: &str) -> Option<bool> {
        Some(self.scalar(key)?.trim() == "true")
    }

    /// A single-valued association: the referenced mRID, or `None` when absent,
    /// written as text, or given twice.
    pub(crate) fn reference(&self, key: &str) -> Option<&'a str> {
        match self.element.fields().get(key)? {
            FieldValue::Resource(s) => Some(s),
            _ => None,
        }
    }

    /// A many-valued association: every referenced mRID.
    pub(crate) fn references(&self, key: &str) -> &'a [String] {
        match self.element.fields().get(key) {
            Some(FieldValue::Resource(s)) => std::slice::from_ref(s),
            Some(FieldValue::ResourceList(v)) => v,
            _ => &[],
        }
    }

    /// An enumeration attribute: the value as stored (`PhaseCode.ABC`), or
    /// `None` when absent, written as text, or given twice.
    pub(crate) fn enumeration(&self, key: &str) -> Option<&'a str> {
        self.reference(key)
    }
}
