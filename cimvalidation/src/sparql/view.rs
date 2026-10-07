//! An element as its generated struct held it, for the three rules written
//! against that view: float special values and mRID uniqueness
//! (`common.rs`), and dangling references (`common_solved_mas.rs`).
//!
//! The view keeps only the attributes the element's class declares, each in
//! the form its Rust type had: a text attribute's last value as written, a
//! number or flag parsed and written back, a single reference only when it
//! was given once. The rules' results depend on exactly that — an undeclared
//! field, a value that did not parse, or a reference given twice was not in
//! the view — so it is rebuilt here from the class table and the attribute
//! ranges in `profile_meta::ATTR_RDF`.
//!
//! An NC element's view was its fields unchanged: NC never had structs.
//!
//! The order matters too. The rules walked the view's map, and a struct built
//! that map afresh, inserting the root class's attributes first; a map's
//! iteration order follows from its insertion sequence. [`struct_order`]
//! rebuilds that sequence, so findings come out in the order they always did.

use std::borrow::Cow;
use std::sync::OnceLock;

use cimmodel::base::{AttrDef, AttrKind, FastMap, FieldValue};
use cimmodel::Element;

/// The Rust type a CGMES literal attribute had.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Literal {
    Text,
    Float,
    Integer,
    Boolean,
}

/// From the attribute's XSD range, the way the generator mapped it: double
/// and decimal were `f64`, integer `i64`, boolean `bool`, everything else —
/// string, the date types, anyURI, an unknown range — `String`.
fn literal_type(attr_id: &str) -> Literal {
    static TYPES: OnceLock<FastMap<&'static str, Literal>> = OnceLock::new();
    let types = TYPES.get_or_init(|| {
        cimmodel::profile_meta::ATTR_RDF
            .iter()
            .filter(|(_, _, _, kind)| *kind == 0)
            .map(|(id, _, range, _)| {
                let t = match range.rsplit('#').next().unwrap_or("") {
                    "double" | "decimal" => Literal::Float,
                    "integer" => Literal::Integer,
                    "boolean" => Literal::Boolean,
                    _ => Literal::Text,
                };
                (*id, t)
            })
            .collect()
    });
    types.get(attr_id).copied().unwrap_or(Literal::Text)
}

fn is_nc(e: &Element) -> bool {
    e.type_name().starts_with("nc:")
}

fn declared(e: &Element, key: &str) -> Option<&'static AttrDef> {
    cimmodel::registry::type_registry().attr(e.class(), key)
}

/// Whether the struct held a value for `attr`, by the generated `from_block`'s
/// rules.
fn in_view(e: &Element, attr: &AttrDef) -> bool {
    let Some(value) = e.fields().get(attr.id) else { return false };
    match attr.kind {
        AttrKind::Literal => {
            let parses = |s: &str| match literal_type(attr.id) {
                Literal::Float => s.trim().parse::<f64>().is_ok(),
                Literal::Integer => s.trim().parse::<i64>().is_ok(),
                Literal::Text | Literal::Boolean => true,
            };
            match (value, attr.is_list) {
                (FieldValue::Text(s), _) => parses(s),
                (FieldValue::TextList(vs), true) => vs.iter().any(|s| parses(s)),
                (FieldValue::TextList(vs), false) => vs.last().is_some_and(|s| parses(s)),
                _ => false,
            }
        }
        _ => matches!(
            (value, attr.is_list),
            (FieldValue::Resource(_), _) | (FieldValue::ResourceList(_), true)
        ),
    }
}

/// The keys of a CGMES element's view, in the iteration order of the map its
/// struct's `to_block` built: the root class's attributes inserted first, each
/// class's in declaration order, into a map with the same hasher.
fn struct_order(e: &Element) -> FastMap<&'static str, usize> {
    let reg = cimmodel::registry::type_registry();
    let mut map: FastMap<&'static str, ()> = FastMap::default();
    for class in reg.chain(e.class()) {
        for attr in class.attrs {
            if in_view(e, attr) {
                map.insert(attr.id, ());
            }
        }
    }
    map.keys().enumerate().map(|(i, k)| (*k, i)).collect()
}

/// `items` in the order the struct's view iterated them.
fn ordered<'a, T>(e: &Element, mut items: Vec<(&'a str, T)>) -> Vec<(&'a str, T)> {
    if is_nc(e) || items.len() < 2 {
        return items;
    }
    let order = struct_order(e);
    items.sort_by_key(|(k, _)| order.get(k).copied().unwrap_or(usize::MAX));
    items
}

/// A scalar's value: the last one when it was repeated.
fn last_text(v: &FieldValue) -> Option<&str> {
    match v {
        FieldValue::Text(s) => Some(s),
        FieldValue::TextList(vs) => vs.last().map(String::as_str),
        _ => None,
    }
}

/// `IdentifiedObject.mRID` as the view held it: absent or empty is `None`.
pub(crate) fn mrid(e: &Element) -> Option<&str> {
    const KEY: &str = "IdentifiedObject.mRID";
    let value = e.fields().get(KEY)?;
    let m = if is_nc(e) {
        match value {
            FieldValue::Text(s) => s.as_str(),
            _ => return None,
        }
    } else {
        declared(e, KEY)?;
        last_text(value)?
    };
    (!m.is_empty()).then_some(m)
}

/// Every single-valued text in the view, keyed by attribute: what the
/// float-special-values rule parses.
pub(crate) fn texts(e: &Element) -> Vec<(&str, Cow<'_, str>)> {
    let mut out = Vec::new();
    for (key, value) in e.fields() {
        if is_nc(e) {
            if let FieldValue::Text(s) = value {
                out.push((*key, Cow::Borrowed(s.as_str())));
            }
            continue;
        }
        let Some(attr) = declared(e, key) else { continue };
        if attr.kind != AttrKind::Literal || attr.is_list {
            continue;
        }
        let Some(raw) = last_text(value) else { continue };
        let shown = match literal_type(key) {
            Literal::Text => Some(Cow::Borrowed(raw)),
            Literal::Float => raw.trim().parse::<f64>().ok().map(|v| Cow::Owned(v.to_string())),
            Literal::Integer => raw.trim().parse::<i64>().ok().map(|v| Cow::Owned(v.to_string())),
            Literal::Boolean => Some(Cow::Owned((raw.trim() == "true").to_string())),
        };
        if let Some(s) = shown {
            out.push((*key, s));
        }
    }
    ordered(e, out)
}

/// Every reference in the view, keyed by attribute.
pub(crate) fn references(e: &Element) -> Vec<(&str, &[String])> {
    let mut out = Vec::new();
    for (key, value) in e.fields() {
        let refs: &[String] = match value {
            FieldValue::Resource(r) => std::slice::from_ref(r),
            FieldValue::ResourceList(rs) => rs,
            _ => continue,
        };
        if !is_nc(e) {
            let Some(attr) = declared(e, key) else { continue };
            if attr.kind == AttrKind::Literal {
                continue;
            }
            // A single-valued reference given twice was absent from the view.
            if !attr.is_list && refs.len() != 1 {
                continue;
            }
        }
        out.push((*key, refs));
    }
    ordered(e, out)
}
