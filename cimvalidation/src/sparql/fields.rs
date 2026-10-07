//! An element's attributes read from its decoded field map, with exactly the
//! values the generated structs' `from_block` would hold.
//!
//! The hand-written rules used to downcast to those structs. Reading the field
//! map instead keeps validation independent of the generated types, so the
//! same rules work for any element the decoder produced. The accessors mirror
//! `from_block` rule for rule, including its quirks — a repeated scalar keeps
//! its last value, a boolean is `true` only for the text `true`, a numeric
//! field parses into its own type, and a single-valued reference written twice
//! is absent — because the rules' results must not change.
//!
//! Like the shape-table interpreter, this needs the decoder's blocks: an
//! element whose block was dropped (`CimDataset::drop_blocks`) has nothing to
//! read, and validating it is a caller error rather than a silent pass.

use cimmodel::base::{FieldValue, RdfBlock};
use cimmodel::{CimDataset, CimEntry};

/// One element's fields. Values borrow from the dataset, not from this view,
/// so they outlive it.
#[derive(Clone, Copy)]
pub(crate) struct Fields<'a> {
    block: &'a RdfBlock,
}

impl<'a> Fields<'a> {
    /// The fields of `entry`, whatever its class.
    pub(crate) fn of(entry: &'a CimEntry) -> Self {
        if entry.block.type_name.is_empty() {
            panic!(
                "{} ({}) has no fields to validate: its block was dropped \
                 (CimDataset::drop_blocks) before validation",
                entry.element.mrid(),
                entry.element.type_name()
            );
        }
        Self { block: &entry.block }
    }

    /// The fields of `entry` if it is an instance of exactly `class` — what a
    /// downcast to that struct answered. A subclass is a different struct, so
    /// it does not match.
    pub(crate) fn of_class(entry: &'a CimEntry, class: &str) -> Option<Self> {
        (entry.element.type_name() == class).then(|| Self::of(entry))
    }

    /// The fields of the element `mrid` in `dataset`, if it is exactly `class`.
    pub(crate) fn get(dataset: &'a CimDataset, mrid: &str, class: &str) -> Option<Self> {
        Self::of_class(dataset.entries.get(mrid)?, class)
    }

    /// The element's mRID as the struct holds it (`IdentifiedObject.id`).
    pub(crate) fn id(&self) -> &'a str {
        &self.block.mrid
    }

    fn scalar(&self, key: &str) -> Option<&'a str> {
        match self.block.fields.get(key)? {
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
        match self.block.fields.get(key) {
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
        match self.block.fields.get(key)? {
            FieldValue::Resource(s) => Some(s),
            _ => None,
        }
    }

    /// A many-valued association: every referenced mRID.
    pub(crate) fn references(&self, key: &str) -> &'a [String] {
        match self.block.fields.get(key) {
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
