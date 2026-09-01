//! Mapping between cimoxide's decoded identifiers and RDF IRIs.
//!
//! The decoder is namespace-blind by construction: `local_name()` drops the XML prefix and
//! `strip_fragment()` drops the IRI base, so `RdfBlock.fields` keys are bare
//! `IdentifiedObject.name` strings and `rdf:resource` targets have lost their namespace.
//! Everything needed to put that back is in the `TYPE_NS` / `ATTR_RDF` tables that `cimgen`
//! emits into `cimstructs::profile_meta`; this module is the lookup layer over them.

use std::collections::HashMap;
use std::sync::OnceLock;

use cimstructs::profile_meta::{ATTR_RDF, TYPE_NS};

/// Base for identifiers that are neither absolute IRIs nor UUIDs. Nothing in the CGMES test
/// configurations hits this, but `rdf:ID` is not required to be a UUID.
pub const LOCAL_BASE: &str = "http://cimoxide/id/";

/// Namespace used for predicates whose attribute id is absent from `ATTR_RDF` — a
/// third-party extension, or a CIM version skew between the data and the generated schema.
pub const FALLBACK_NS: &str = "http://iec.ch/TC57/CIM100#";

/// What kind of RDF object an attribute's value denotes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Kind {
    /// A literal, typed by `AttrRdf::range`.
    Literal,
    /// An IRI reference to another CIM object.
    Association,
    /// An IRI naming an enumeration value; `AttrRdf::range` is the enum's namespace.
    Enum,
}

/// One row of `ATTR_RDF`, resolved.
#[derive(Debug, Clone, Copy)]
pub struct AttrRdf {
    pub namespace: &'static str,
    /// XSD datatype IRI for [`Kind::Literal`], the enum namespace for [`Kind::Enum`],
    /// empty for [`Kind::Association`].
    pub range: &'static str,
    pub kind: Kind,
}

// `TYPE_NS` and `ATTR_RDF` are slices, and they are consulted once per field of every
// element — a linear scan would dominate at a million fields, so index them once.
fn type_ns_index() -> &'static HashMap<&'static str, &'static str> {
    static IDX: OnceLock<HashMap<&'static str, &'static str>> = OnceLock::new();
    IDX.get_or_init(|| TYPE_NS.iter().copied().collect())
}

fn attr_index() -> &'static HashMap<&'static str, AttrRdf> {
    static IDX: OnceLock<HashMap<&'static str, AttrRdf>> = OnceLock::new();
    IDX.get_or_init(|| {
        ATTR_RDF
            .iter()
            .map(|&(id, namespace, range, kind)| {
                let kind = match kind {
                    0 => Kind::Literal,
                    2 => Kind::Enum,
                    _ => Kind::Association,
                };
                (id, AttrRdf { namespace, range, kind })
            })
            .collect()
    })
}

/// RDF namespace IRI of a CIM class, e.g. `http://iec.ch/TC57/CIM100-European#` for
/// `BoundaryPoint`. Trailing `#` included.
pub fn type_namespace(type_name: &str) -> Option<&'static str> {
    type_ns_index().get(type_name).copied()
}

/// Full class IRI for a decoded `type_name`.
pub fn type_iri(type_name: &str) -> String {
    format!("{}{type_name}", type_namespace(type_name).unwrap_or(FALLBACK_NS))
}

/// RDF metadata for an `RdfBlock.fields` key such as `"ACLineSegment.b0ch"`.
pub fn attr(attr_id: &str) -> Option<AttrRdf> {
    attr_index().get(attr_id).copied()
}

/// Full predicate IRI for an `RdfBlock.fields` key.
pub fn predicate_iri(attr_id: &str) -> String {
    let ns = attr(attr_id).map_or(FALLBACK_NS, |a| a.namespace);
    format!("{ns}{attr_id}")
}

fn is_absolute_iri(s: &str) -> bool {
    // scheme = ALPHA *( ALPHA / DIGIT / "+" / "-" / "." ) ":"
    let mut chars = s.char_indices();
    match chars.next() {
        Some((_, c)) if c.is_ascii_alphabetic() => {}
        _ => return false,
    }
    for (i, c) in chars {
        match c {
            ':' => return i > 0,
            c if c.is_ascii_alphanumeric() || c == '+' || c == '-' || c == '.' => {}
            _ => return false,
        }
    }
    false
}

fn is_uuid(s: &str) -> bool {
    let b = s.as_bytes();
    if b.len() != 36 {
        return false;
    }
    b.iter().enumerate().all(|(i, &c)| {
        if matches!(i, 8 | 13 | 18 | 23) {
            c == b'-'
        } else {
            c.is_ascii_hexdigit()
        }
    })
}

/// Subject IRI for a decoded MRID.
///
/// The decoder yields identifiers in two shapes: `_bd8b27aa-…` (from `rdf:ID`, and the form
/// `rdf:resource="#_…"` targets are stripped to) and `urn:uuid:2b6b90c0-…` (from `rdf:about`
/// on `md:FullModel`). Both normalise onto the CGMES-canonical `urn:uuid:` form, which is
/// what makes [`iri_to_mrid`] a pure inverse — no side table is held.
pub fn mrid_to_iri(mrid: &str) -> String {
    if is_absolute_iri(mrid) {
        return mrid.to_string();
    }
    let bare = mrid.strip_prefix('_').unwrap_or(mrid);
    if is_uuid(bare) {
        format!("urn:uuid:{bare}")
    } else {
        format!("{LOCAL_BASE}{mrid}")
    }
}

/// Inverse of [`mrid_to_iri`], resolved against the dataset the IRI came from.
///
/// `urn:uuid:X` is ambiguous on its own — the decoder may hold it as `_X` or as `X` — so
/// membership in `entries` picks the right one.
pub fn iri_to_mrid(iri: &str, ds: &cimdecoder::CimDataset) -> Option<String> {
    if let Some(rest) = iri.strip_prefix(LOCAL_BASE) {
        return ds.entries.contains_key(rest).then(|| rest.to_string());
    }
    if let Some(uuid) = iri.strip_prefix("urn:uuid:") {
        let underscored = format!("_{uuid}");
        if ds.entries.contains_key(&underscored) {
            return Some(underscored);
        }
        if ds.entries.contains_key(uuid) {
            return Some(uuid.to_string());
        }
    }
    ds.entries.contains_key(iri).then(|| iri.to_string())
}

/// Object IRI for an association value.
///
/// Association targets are normally MRIDs, but two `eu:`-namespaced enumerations
/// (`LimitKind`, `SVCControlMode`) are generated as marker structs rather than enums —
/// upstream `cims:stereotype` parsing is last-write-wins and their `European` stereotype
/// overwrites the `enumeration` one — so `eu:OperationalLimitType.kind` arrives here as a
/// plain reference holding a fragment-stripped `LimitKind.patl`. A value that names no entry
/// in the dataset but does match a known `Type.value` is that case, and the class namespace
/// rebuilds the IRI the decoder discarded.
pub fn reference_iri(value: &str, ds: &cimdecoder::CimDataset) -> String {
    if !ds.entries.contains_key(value)
        && let Some((type_name, _)) = value.split_once('.')
        && let Some(ns) = type_namespace(type_name)
    {
        return format!("{ns}{value}");
    }
    mrid_to_iri(value)
}

/// Namespace bindings for queries: every CGMES namespace the generator knows, plus `xsd:`.
pub fn sparql_prefixes() -> &'static [(&'static str, &'static str)] {
    static P: OnceLock<Vec<(&'static str, &'static str)>> = OnceLock::new();
    P.get_or_init(|| {
        let mut v: Vec<(&'static str, &'static str)> =
            cimstructs::constants::CIM_NAMESPACES.to_vec();
        v.push(("xsd", "http://www.w3.org/2001/XMLSchema#"));
        v
    })
}
