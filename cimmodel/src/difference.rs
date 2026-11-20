//! Difference models: a CGMES `dm:DifferenceModel`'s or an NC
//! `dcatcim:DifferenceSet`'s statements, applied to a model, and computed
//! between two.
//!
//! IEC 61970-552 describes a change to a model as statements: the reverse
//! differences, removed from the model; the forward differences, added to it;
//! and preconditions, which must hold in the model first. The decoder reads
//! each block into a dataset of its own ([`CimDataset::differences`]): a
//! typed element states the object's type and its values, an
//! `rdf:Description` only its values, and lands among the block's foreign
//! objects, its keys `Class.attr` as written. A changed value is a statement
//! in each block, the old one reverse, the new one forward; a new object's
//! type and values are forward statements, a removed one's reverse.

use std::error::Error;
use std::fmt::Write;

use crate::base::{foreign_class, ClassDef, Element, FastMap, FastSet, FieldValue, Value};
use crate::convert_nc::Prefixes;
use crate::decode::{CimDataset, FieldForm};
use crate::registry::type_registry;

/// RDF's namespace, whose `Description` an untyped statement is written as.
const RDF: &str = "http://www.w3.org/1999/02/22-rdf-syntax-ns#";
/// A CGMES `dm:DifferenceModel`'s namespace, its blocks' names in it.
pub(crate) const DM: &str = "http://iec.ch/TC57/61970-552/DifferenceModel/1#";
/// An NC `dcatcim:DifferenceSet`'s namespace, its blocks' names in it.
pub(crate) const DCATCIM: &str = "https://cim4.eu/ns/dcatcim#";

/// A block of a difference header.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Block {
    /// Statements added to the model.
    Forward,
    /// Statements removed from the model.
    Reverse,
    /// Statements that must hold in the model first (CGMES only).
    Preconditions,
}

/// The statements of one difference header ([`CimDataset::differences`]).
pub struct Differences {
    /// The header's identifier (`urn:uuid:…`); empty for differences computed
    /// ([`Differences::between`]) until the caller gives them one.
    pub header: String,
    pub forward: CimDataset,
    pub reverse: CimDataset,
    pub preconditions: CimDataset,
    /// The blocks the header held, in the order written, for the encoder.
    pub blocks: Vec<Block>,
}

impl Differences {
    pub(crate) fn new(header: &str) -> Self {
        Self {
            header: header.to_string(),
            forward: CimDataset::new(),
            reverse: CimDataset::new(),
            preconditions: CimDataset::new(),
            blocks: Vec::new(),
        }
    }

    /// The dataset of block `which`, recorded as held.
    pub(crate) fn block_mut(&mut self, which: Block) -> &mut CimDataset {
        if !self.blocks.contains(&which) {
            self.blocks.push(which);
        }
        match which {
            Block::Forward => &mut self.forward,
            Block::Reverse => &mut self.reverse,
            Block::Preconditions => &mut self.preconditions,
        }
    }

    /// The statements that take `before` to `after`: what `after` lacks of
    /// `before` reverse, what it adds forward. An object in one only is
    /// written whole, typed; an object in both, where its values differ, as
    /// an `rdf:Description` of the values that differ, typed only where its
    /// types do. Applied to `before` ([`CimDataset::apply`]), they give
    /// `after`. The header is empty: the caller names it.
    pub fn between(before: &CimDataset, after: &CimDataset) -> Self {
        let mut d = Self::new("");
        d.blocks = vec![Block::Reverse, Block::Forward];
        let ids: FastSet<&str> = before
            .entries
            .keys()
            .chain(before.foreign.keys())
            .chain(after.entries.keys())
            .chain(after.foreign.keys())
            .map(String::as_str)
            .collect();
        let mut ids: Vec<&str> = ids.into_iter().collect();
        ids.sort_unstable();
        for id in ids {
            let (was, now) = (lookup(before, id), lookup(after, id));
            match (was, now) {
                (Some((b, foreign)), None) => put(&mut d.reverse, b.clone(), foreign),
                (None, Some((a, foreign))) => put(&mut d.forward, a.clone(), foreign),
                (Some((b, foreign)), Some((a, _))) => {
                    let gone = without(b, a);
                    let came = without(a, b);
                    let types = |e: &Element| e.types().map(|c| c.qualified).collect::<FastSet<_>>();
                    let (tb, ta) = (types(b), types(a));
                    let lost: Vec<&'static ClassDef> = b.types().filter(|c| !ta.contains(c.qualified)).collect();
                    let gained: Vec<&'static ClassDef> = a.types().filter(|c| !tb.contains(c.qualified)).collect();
                    if !gone.is_empty() || !lost.is_empty() {
                        put(&mut d.reverse, statements(id, &lost, gone), foreign);
                        named(&mut d.reverse, before, id, b);
                    }
                    if !came.is_empty() || !gained.is_empty() {
                        put(&mut d.forward, statements(id, &gained, came), foreign);
                        named(&mut d.forward, after, id, a);
                    }
                }
                (None, None) => {}
            }
        }
        d
    }
}

/// `id`'s object in `ds`, and whether it is foreign: among the CIM objects
/// by mRID (an IRI's text after its last `#`), else among the foreign ones as
/// written.
fn lookup<'a>(ds: &'a CimDataset, id: &str) -> Option<(&'a Element, bool)> {
    ds.entries
        .get(id)
        .or_else(|| ds.entries.get(fragment(id)))
        .map(|e| (e, false))
        .or_else(|| ds.foreign.get(id).map(|e| (e, true)))
}

/// The key a CIM object has for identifier `id`: its text after the last `#`.
fn fragment(id: &str) -> &str {
    id.rsplit_once('#').map_or(id, |(_, f)| if f.is_empty() { id } else { f })
}

/// Into a block: a typed CIM object among its objects, anything else among
/// its foreign ones.
fn put(ds: &mut CimDataset, element: Element, foreign: bool) {
    let id = element.mrid().to_string();
    if foreign || element.class().qualified.starts_with(RDF) {
        ds.foreign.insert(id, element);
    } else {
        ds.set(id, element);
    }
}

/// `a`'s values `b` lacks, by key.
fn without(a: &Element, b: &Element) -> Vec<(&'static str, FieldValue)> {
    let mut out = Vec::new();
    let mut keys: Vec<&'static str> = a.fields().keys().copied().collect();
    keys.sort_unstable();
    for key in keys {
        for v in a.fields()[key].values() {
            if !b.has_value(key, &v.owned()) {
                out.push((key, v.owned()));
            }
        }
    }
    out
}

/// Statements about `id`, an object both models hold: its types `types` and
/// `values`, as a typed element of the first type (the rest beside it), or an
/// `rdf:Description` without one; written `rdf:about`, as they describe an
/// object defined elsewhere.
fn statements(id: &str, types: &[&'static ClassDef], values: Vec<(&'static str, FieldValue)>) -> Element {
    let mut e = Element::new(foreign_class(RDF, "Description"), id.to_string());
    e.set_types(types);
    e.mark_described();
    for (key, value) in values {
        e.add_field(key, value);
    }
    e
}

/// The namespaces the fields of statement `id` in `block` are written in, an
/// `rdf:Description`'s keys being declared by no class of its: as `like`
/// (the object they are about, in `from`) has them, read or declared.
fn named(block: &mut CimDataset, from: &CimDataset, id: &str, like: &Element) {
    let Some(e) = block.foreign.get(id).filter(|e| untyped(e)) else { return };
    let reg = type_registry();
    let class = e.class().qualified;
    for &key in e.fields().keys() {
        let form = from.field_forms.get(&(like.type_name(), key)).copied().unwrap_or_default();
        let ns = form.ns.or_else(|| reg.attr_of(like.class(), key).map(|a| a.ns));
        let resource_ns = form.resource_ns;
        block.field_forms.insert((class, key), FieldForm { ns, datatype: form.datatype, resource_ns });
    }
}

/// A difference header's blocks inside it, in the order read: each a field of
/// the header in namespace `ns` ([`DM`] or [`DCATCIM`]) holding its
/// statements (`rdf:parseType="Statements"`), written as a document's objects
/// are.
pub(crate) fn write_blocks(out: &mut String, prefixes: &mut Prefixes, d: &Differences, ns: &'static str) -> Result<(), Box<dyn Error>> {
    let nc = ns == DCATCIM;
    let prefix = prefixes.of(ns);
    for &which in &d.blocks {
        let (name, ds) = match (which, nc) {
            (Block::Forward, false) => ("forwardDifferences", &d.forward),
            (Block::Forward, true) => ("forwardDifference", &d.forward),
            (Block::Reverse, false) => ("reverseDifferences", &d.reverse),
            (Block::Reverse, true) => ("reverseDifference", &d.reverse),
            (Block::Preconditions, false) => ("preconditions", &d.preconditions),
            (Block::Preconditions, true) => continue,
        };
        let mut inner = String::new();
        crate::convert::write_objects(&mut inner, prefixes, ds)?;
        if inner.is_empty() {
            write!(out, "\n    <{prefix}:{name} rdf:parseType=\"Statements\"/>")?;
        } else {
            write!(out, "\n    <{prefix}:{name} rdf:parseType=\"Statements\">\n{inner}    </{prefix}:{name}>")?;
        }
    }
    Ok(())
}

/// A block's statements, typed and untyped, by subject.
fn sorted(ds: &CimDataset) -> Vec<(&String, &Element)> {
    let mut all: Vec<(&String, &Element)> = ds.entries.iter().chain(&ds.foreign).collect();
    all.sort_unstable_by_key(|(id, _)| id.as_str());
    all
}

/// Whether `e` states no type: an `rdf:Description`.
fn untyped(e: &Element) -> bool {
    e.class().qualified == "http://www.w3.org/1999/02/22-rdf-syntax-ns#Description"
}

/// One statement a difference could not apply.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Statement {
    /// That the object is of a class, by its qualified name.
    Type(&'static str),
    /// That a field holds a value.
    Value(&'static str, FieldValue),
}

/// A statement that does not hold where a difference needs it: a reverse
/// statement or a precondition the model lacks, a forward statement about an
/// object that neither exists nor is given a type.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Unmatched {
    pub block: Block,
    pub subject: String,
    pub statement: Statement,
}

/// A field a difference changed: its value before and after, `None` where
/// it had none.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FieldChange {
    pub key: &'static str,
    pub before: Option<FieldValue>,
    pub after: Option<FieldValue>,
}

/// An object a difference changed, there before and after.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Changed {
    pub id: String,
    /// Its class before and after, where that changed.
    pub retyped: Option<(&'static str, &'static str)>,
    pub fields: Vec<FieldChange>,
}

/// What applying a difference did ([`CimDataset::apply`]). Each list is in
/// identifier order.
#[derive(Debug, Default)]
pub struct Applied {
    /// The objects it created.
    pub added: Vec<String>,
    /// The objects it removed, as they were.
    pub removed: Vec<Element>,
    pub changed: Vec<Changed>,
    /// The statements that did not hold where they had to.
    pub unmatched: Vec<Unmatched>,
    /// Objects whose every type was removed while values of theirs stayed:
    /// kept, with their types, as an object needs a class.
    pub untyped: Vec<String>,
}

impl CimDataset {
    /// Apply a difference: its preconditions checked, its reverse statements
    /// removed, its forward statements added, as IEC 61970-552 describes. An
    /// object left with neither type nor values is removed; one given a type
    /// it lacked is created. A statement that does not hold is reported
    /// ([`Applied::unmatched`]) and passed over, not fatal. Several
    /// differences apply in turn, in the order the caller gives (along their
    /// headers' `Model.Supersedes`, say).
    pub fn apply(&mut self, d: &Differences) -> Applied {
        let mut applied = Applied::default();
        let miss = |applied: &mut Applied, block, subject: &str, statement| {
            applied.unmatched.push(Unmatched { block, subject: subject.to_string(), statement });
        };

        // Preconditions hold in the model as it is.
        for (id, e) in sorted(&d.preconditions) {
            let held = lookup(self, id).map(|(o, _)| o);
            if !untyped(e) {
                for t in e.types() {
                    if !held.is_some_and(|o| o.types().any(|c| c.qualified == t.qualified)) {
                        miss(&mut applied, Block::Preconditions, id, Statement::Type(t.qualified));
                    }
                }
            }
            for (key, fv) in e.fields() {
                for v in fv.values() {
                    if !held.is_some_and(|o| o.has_value(key, &v.owned())) {
                        miss(&mut applied, Block::Preconditions, id, Statement::Value(key, v.owned()));
                    }
                }
            }
        }

        // Each object the difference is about, as it was, and its types as they become.
        let mut touched: Vec<&str> = d
            .reverse
            .entries
            .keys()
            .chain(d.reverse.foreign.keys())
            .chain(d.forward.entries.keys())
            .chain(d.forward.foreign.keys())
            .map(String::as_str)
            .collect::<FastSet<&str>>()
            .into_iter()
            .collect();
        touched.sort_unstable();
        // By the key the object has in the model, or will have.
        let mut was: FastMap<&str, Option<(Element, bool)>> = FastMap::default();
        let mut types: FastMap<&str, Vec<&'static ClassDef>> = FastMap::default();
        for &id in &touched {
            let found = lookup(self, id).map(|(e, f)| (e.clone(), f));
            types.insert(id, found.as_ref().map_or_else(Vec::new, |(e, _)| e.types().collect()));
            was.insert(id, found);
        }

        for (id, e) in sorted(&d.reverse) {
            let id = id.as_str();
            let Some(object) = self.object_mut(id) else {
                if !untyped(e) {
                    for t in e.types() {
                        miss(&mut applied, Block::Reverse, id, Statement::Type(t.qualified));
                    }
                }
                for (key, fv) in e.fields() {
                    for v in fv.values() {
                        miss(&mut applied, Block::Reverse, id, Statement::Value(key, v.owned()));
                    }
                }
                continue;
            };
            let mut missed = Vec::new();
            for (key, fv) in e.fields() {
                for v in fv.values() {
                    if !object.remove_value(key, &v.owned()) {
                        missed.push(Statement::Value(key, v.owned()));
                    }
                }
            }
            if !untyped(e) {
                let held = types.get_mut(id).expect("touched");
                for t in e.types() {
                    match held.iter().position(|c| c.qualified == t.qualified) {
                        Some(i) => {
                            held.remove(i);
                        }
                        None => missed.push(Statement::Type(t.qualified)),
                    }
                }
            }
            for statement in missed {
                miss(&mut applied, Block::Reverse, id, statement);
            }
        }

        for (id, e) in sorted(&d.forward) {
            let id = id.as_str();
            let stated: Vec<&'static ClassDef> = if untyped(e) { Vec::new() } else { e.types().collect() };
            let held = types.get_mut(id).expect("touched");
            for t in &stated {
                if !held.iter().any(|c| c.qualified == t.qualified) {
                    held.push(t);
                }
            }
            if self.object_mut(id).is_none() {
                let Some(&class) = stated.first() else {
                    // Values for an object that neither is nor is given a type.
                    for (key, fv) in e.fields() {
                        for v in fv.values() {
                            miss(&mut applied, Block::Forward, id, Statement::Value(key, v.owned()));
                        }
                    }
                    continue;
                };
                let mut created = Element::new(class, e.mrid().to_string());
                if !e.defined() {
                    created.mark_described();
                }
                let foreign = d.forward.foreign.contains_key(id);
                put(self, created, foreign);
            }
            let object = self.object_mut(id).expect("just made");
            for (key, fv) in e.fields() {
                for v in fv.values() {
                    let value = v.owned();
                    if !object.has_value(key, &value) {
                        object.add_field(key, value);
                    }
                }
            }
        }

        // What became of each object.
        for &id in &touched {
            let now_types = &types[id];
            let Some((key, foreign)) = self.key_of(id) else { continue };
            match &was[id] {
                None => applied.added.push(key),
                Some((before, _)) => {
                    let object = if foreign { &self.foreign[&key] } else { &self.entries[&key] };
                    if now_types.is_empty() {
                        if object.fields().is_empty() {
                            let removed = if foreign { self.foreign.remove(&key) } else { self.remove(&key) };
                            applied.removed.extend(removed.map(|_| before.clone()));
                        } else {
                            applied.untyped.push(key.clone());
                            if let Some(c) = changes(&key, before, object) {
                                applied.changed.push(c);
                            }
                        }
                        continue;
                    }
                    let mut object = object.clone();
                    if !now_types.iter().map(|c| c.qualified).eq(object.types().map(|c| c.qualified)) {
                        object.set_types(now_types);
                    }
                    if let Some(c) = changes(&key, before, &object) {
                        applied.changed.push(c);
                    }
                    if foreign {
                        self.foreign.insert(key, object);
                    } else {
                        self.set(key, object);
                    }
                }
            }
        }
        applied.added.sort_unstable();
        applied.added.dedup();
        applied
    }

    /// The object identifier `id` names, to change.
    fn object_mut(&mut self, id: &str) -> Option<&mut Element> {
        let (key, foreign) = self.key_of(id)?;
        if foreign { self.foreign.get_mut(&key) } else { self.entries.get_mut(&key) }
    }

    /// The key identifier `id`'s object has here, and whether it is foreign.
    fn key_of(&self, id: &str) -> Option<(String, bool)> {
        if self.entries.contains_key(id) {
            Some((id.to_string(), false))
        } else if self.entries.contains_key(fragment(id)) {
            Some((fragment(id).to_string(), false))
        } else if self.foreign.contains_key(id) {
            Some((id.to_string(), true))
        } else {
            None
        }
    }
}

/// How `after` differs from `before`, the same object: its class, and each
/// field whose values differ. `None` where nothing does.
fn changes(id: &str, before: &Element, after: &Element) -> Option<Changed> {
    let retyped = (before.class().qualified != after.class().qualified).then(|| (before.class().qualified, after.class().qualified));
    let mut keys: Vec<&'static str> = before.fields().keys().chain(after.fields().keys()).copied().collect::<FastSet<_>>().into_iter().collect();
    keys.sort_unstable();
    let same = |a: Option<&FieldValue>, b: Option<&FieldValue>| match (a, b) {
        (None, None) => true,
        (Some(a), Some(b)) => {
            let mut x: Vec<Value> = a.values().collect();
            let mut y: Vec<Value> = b.values().collect();
            x.sort_unstable();
            y.sort_unstable();
            x == y
        }
        _ => false,
    };
    let fields: Vec<FieldChange> = keys
        .into_iter()
        .filter(|k| !same(before.fields().get(k), after.fields().get(k)))
        .map(|k| FieldChange { key: k, before: before.fields().get(k).cloned(), after: after.fields().get(k).cloned() })
        .collect();
    (retyped.is_some() || !fields.is_empty()).then(|| Changed { id: id.to_string(), retyped, fields })
}
