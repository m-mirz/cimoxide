use std::path::Path;

use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;

use crate::base::{intern, AttrDef, ClassDef, Element, FastMap, FieldValue, TypeRegistry};
use crate::registry;

pub struct CimDataset {
    pub entries: FastMap<String, Element>,
    /// Maps `type_name()` → list of MRIDs of that type. Populated on insert, maintained on merge.
    pub by_type: FastMap<String, Vec<String>>,
}

impl Default for CimDataset {
    fn default() -> Self {
        Self::new()
    }
}

impl CimDataset {
    pub fn new() -> Self {
        Self {
            entries: FastMap::default(),
            by_type: FastMap::default(),
        }
    }

    /// Decode an RDF/XML string into a CimDataset, using the provided type registry.
    pub fn decode_str(content: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let mut ds = Self::new();
        parse_rdf(content, registry::type_registry(), &mut ds)?;
        Ok(ds)
    }

    /// Decode an RDF/XML file into a CimDataset, using the provided type registry.
    pub fn decode_file(path: &Path) -> Result<Self, Box<dyn std::error::Error>> {
        Self::decode_str(&std::fs::read_to_string(path)?)
    }

    /// Decode multiple RDF/XML files into a single CimDataset, merging entries with the same MRID.
    pub fn decode_files(paths: &[&Path]) -> Result<Self, Box<dyn std::error::Error>> {
        let mut combined = Self::new();
        for path in paths {
            let file_ds = Self::decode_file(path)?;
            combined.merge(file_ds);
        }
        Ok(combined)
    }

    /// Decode files in parallel using one thread per file, without merging.
    /// Preserves input order. Falls back to sequential decoding for 0–1 paths
    /// to avoid thread-spawn overhead.
    pub fn decode_files_parallel_separate(paths: &[&Path]) -> Result<Vec<Self>, Box<dyn std::error::Error>> {
        if paths.len() <= 1 {
            return paths.iter().map(|p| Self::decode_file(p)).collect();
        }
        let results: Vec<Result<Self, String>> = std::thread::scope(|s| {
            paths
                .iter()
                .map(|p| s.spawn(|| Self::decode_file(p).map_err(|e| e.to_string())))
                .collect::<Vec<_>>()
                .into_iter()
                .map(|h| h.join().expect("decode thread panicked"))
                .collect()
        });
        results
            .into_iter()
            .collect::<Result<_, String>>()
            .map_err(|e| e.into())
    }

    /// Decode files in parallel using one thread per file, then merge sequentially.
    pub fn decode_files_parallel(paths: &[&Path]) -> Result<Self, Box<dyn std::error::Error>> {
        let datasets = Self::decode_files_parallel_separate(paths)?;
        Ok(datasets
            .into_iter()
            .reduce(|mut a, b| {
                a.merge(b);
                a
            })
            .unwrap_or_default())
    }

    /// Merge another dataset into self, combining objects with the same MRID:
    /// a later scalar wins, reference lists are joined. An object keeps the
    /// class it was first decoded as.
    pub fn merge(&mut self, other: CimDataset) {
        for (mrid, incoming) in other.entries {
            if let Some(existing) = self.entries.get_mut(&mrid) {
                existing.merge_from(&incoming);
            } else {
                let type_name = incoming.type_name().to_string();
                self.by_type.entry(type_name).or_default().push(mrid.clone());
                self.entries.insert(mrid, incoming);
            }
        }
    }

    /// Insert or replace the element at `mrid`, keeping `by_type` consistent
    /// (including the case where `mrid` already exists under a different type).
    pub fn set(&mut self, mrid: String, element: Element) {
        let new_type = element.type_name().to_string();
        if let Some(old) = self.entries.get(&mrid) {
            let old_type = old.type_name();
            if old_type != new_type
                && let Some(v) = self.by_type.get_mut(old_type) {
                    v.retain(|m| m != &mrid);
                }
        }
        let already_indexed = self
            .by_type
            .get(&new_type)
            .is_some_and(|v| v.contains(&mrid));
        if !already_indexed {
            self.by_type.entry(new_type).or_default().push(mrid.clone());
        }
        self.entries.insert(mrid, element);
    }

    /// Remove the element at `mrid`, pruning it from `by_type`. Returns the
    /// removed element, or `None` if `mrid` was not present.
    pub fn remove(&mut self, mrid: &str) -> Option<Element> {
        let removed = self.entries.remove(mrid)?;
        if let Some(v) = self.by_type.get_mut(removed.type_name()) {
            v.retain(|m| m != mrid);
        }
        Some(removed)
    }
}

// --- XML streaming parser ---------------------------------------------------

/// Push an MRID onto its type bucket, allocating the key only when the bucket
/// is new rather than once per element.
fn index(by_type: &mut FastMap<String, Vec<String>>, type_name: &'static str, mrid: String) {
    if let Some(bucket) = by_type.get_mut(type_name) {
        bucket.push(mrid);
    } else {
        by_type.insert(type_name.to_string(), vec![mrid]);
    }
}

type Table = FastMap<&'static str, &'static ClassDef>;

/// The document's `xmlns` bindings, resolved once to dispatch tables.
///
/// Resolving a prefix per element rather than a namespace IRI is what keeps
/// this cheap: prefixes are two to four bytes, IRIs are thirty-odd, and a
/// document binds a handful of them and then repeats them on every element.
#[derive(Default)]
struct Scope {
    prefixes: Vec<(Vec<u8>, Option<&'static Table>)>,
    default: Option<&'static Table>,
}

impl Scope {
    /// Collect `xmlns:*` and `xmlns` declarations from an element.
    fn absorb(&mut self, e: &BytesStart, reg: &'static TypeRegistry) {
        for attr in e.attributes().flatten() {
            let key = attr.key.as_ref();
            let Some(rest) = key.strip_prefix(b"xmlns".as_slice()) else {
                continue;
            };
            let Ok(ns) = std::str::from_utf8(&attr.value) else {
                continue;
            };
            // A namespace written without its delimiter still names the same vocabulary.
            let table = reg.ns_table(ns).or_else(|| {
                if ns.ends_with('#') || ns.ends_with('/') {
                    None
                } else {
                    reg.ns_table(&format!("{ns}#"))
                }
            });
            match rest.strip_prefix(b":".as_slice()) {
                Some(prefix) => {
                    let prefix = prefix.to_vec();
                    match self.prefixes.iter_mut().find(|(k, _)| *k == prefix) {
                        Some(slot) => slot.1 = table,
                        None => self.prefixes.push((prefix, table)),
                    }
                }
                None if rest.is_empty() => self.default = table,
                None => {}
            }
        }
    }

    fn table(&self, prefix: Option<&[u8]>) -> Option<&'static Table> {
        match prefix {
            None => self.default,
            Some(p) => self
                .prefixes
                .iter()
                .find(|(k, _)| k.as_slice() == p)
                .and_then(|(_, t)| *t),
        }
    }

}

/// One pass over an element's attributes, yielding both things the parser needs:
/// the MRID and whether the element redeclares any namespace. Scanning twice
/// costs more than the nested-`xmlns` case it would serve, because every element
/// pays for it and almost none declare one.
fn scan_attrs(e: &BytesStart) -> Result<(String, bool), Box<dyn std::error::Error>> {
    let mut mrid = String::new();
    let mut has_xmlns = false;
    for attr in e.attributes().flatten() {
        let key = attr.key.as_ref();
        if key == b"rdf:about" || key == b"rdf:ID" {
            mrid = strip_fragment(std::str::from_utf8(&attr.value)?);
        } else if key.starts_with(b"xmlns") {
            has_xmlns = true;
        }
    }
    Ok((mrid, has_xmlns))
}

/// Split a qualified name into its prefix and local part.
fn split_qname(raw: &[u8]) -> (Option<&[u8]>, &[u8]) {
    match raw.iter().position(|b| *b == b':') {
        Some(i) => (Some(&raw[..i]), &raw[i + 1..]),
        None => (None, raw),
    }
}

/// Resolve an element name to its class.
///
/// The namespace wins when the document binds one we know. Otherwise we fall
/// back to the bare local name, which is what every pre-namespace release did:
/// real files bind prefixes we do not expect (one CGMES test file binds the
/// ModelDescription IRI to `mdc`), and write elements under a namespace their
/// class was not declared in (`<cim:BoundaryPoint>` where `BoundaryPoint` is
/// declared in `CIM100-European#`). The fallback is therefore per element, not
/// per document.
fn resolve(
    scope: &Scope,
    reg: &'static TypeRegistry,
    raw: &[u8],
) -> Result<Option<&'static ClassDef>, Box<dyn std::error::Error>> {
    let (prefix, local_bytes) = split_qname(raw);
    let local = std::str::from_utf8(local_bytes)?;
    if let Some(class) = scope.table(prefix).and_then(|t| t.get(local)) {
        return Ok(Some(*class));
    }
    Ok(reg.bare().get(local).copied())
}

/// Parse an RDF/XML string into a CimDataset, using the provided type registry.
fn parse_rdf(
    content: &str,
    reg: &'static TypeRegistry,
    ds: &mut CimDataset,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut reader = Reader::from_str(content);
    let mut buf = Vec::new();

    let mut depth: u32 = 0;
    let mut scope = Scope::default();
    // Outer scope parked while a nested `xmlns` override is in effect.
    let mut nested: Option<Scope> = None;
    // Resolved at the element's start tag, so an unregistered element never
    // accumulates fields.
    let mut current: Option<Element> = None;
    // The attributes `current`'s class declares, which supply most field keys.
    let mut declared: Option<&'static FastMap<&'static str, &'static AttrDef>> = None;
    let mut pending_key: Option<&'static str> = None;
    // The `xml:lang` of the field element whose text comes next.
    let mut pending_lang: Option<String> = None;
    // A field key as stored: the class table's id when the class declares it,
    // else the interned name.
    let key_of = |declared: Option<&'static FastMap<&'static str, &'static AttrDef>>, local: &str| {
        declared.and_then(|d| d.get(local)).map_or_else(|| intern(local), |a| a.id)
    };

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(ref e)) => {
                depth += 1;

                match depth {
                    1 => scope.absorb(e, reg),
                    2 => {
                        // Every CGMES and NC file declares its namespaces on the
                        // root, but a nested redeclaration is legal RDF/XML, so
                        // honour one when it appears. The End arm restores the
                        // outer scope.
                        let (mrid, has_xmlns) = scan_attrs(e)?;
                        if has_xmlns {
                            let mut inner = Scope {
                                prefixes: scope.prefixes.clone(),
                                default: scope.default,
                            };
                            inner.absorb(e, reg);
                            nested = Some(std::mem::replace(&mut scope, inner));
                        }

                        current = resolve(&scope, reg, e.name().as_ref())?.map(|class| Element::new(class, mrid));
                        declared = current.as_ref().and_then(|c| reg.declared(c.class()));
                    }
                    3 => {
                        if let Some(element) = current.as_mut() {
                            let name = e.name();
                            let (_, local_bytes) = split_qname(name.as_ref());
                            let local = key_of(declared, std::str::from_utf8(local_bytes)?);
                            let (res, lang) = field_attrs(e.attributes())?;
                            if let Some(res) = res {
                                element.add_field(local, FieldValue::Resource(res));
                            } else {
                                pending_key = Some(local);
                                pending_lang = lang;
                            }
                        }
                    }
                    _ => {}
                }
            }

            Ok(Event::Empty(ref e)) => {
                match depth {
                    1 => {
                        // Top-level self-closing element: <cim:Foo rdf:ID="x" />
                        // It can carry its own xmlns; with no children there is
                        // nothing to restore afterwards, so resolve against a
                        // throwaway scope.
                        let (mrid, has_xmlns) = scan_attrs(e)?;
                        let local_scope = if has_xmlns {
                            let mut inner = Scope {
                                prefixes: scope.prefixes.clone(),
                                default: scope.default,
                            };
                            inner.absorb(e, reg);
                            Some(inner)
                        } else {
                            None
                        };
                        let active = local_scope.as_ref().unwrap_or(&scope);
                        if !mrid.is_empty()
                            && let Some(class) = resolve(active, reg, e.name().as_ref())? {
                                index(&mut ds.by_type, class.qualified, mrid.clone());
                                ds.entries.insert(mrid.clone(), Element::new(class, mrid));
                            }
                    }
                    2 => {
                        // Self-closing field element within the current type block.
                        if let Some(element) = current.as_mut()
                            && let Some(res) = find_resource(e.attributes())? {
                                let name = e.name();
                                let (_, local_bytes) = split_qname(name.as_ref());
                                let local = key_of(declared, std::str::from_utf8(local_bytes)?);
                                element.add_field(local, FieldValue::Resource(res));
                            }
                    }
                    _ => {}
                }
            }

            Ok(Event::Text(ref e)) => {
                if depth == 3
                    && let (Some(element), Some(key)) = (current.as_mut(), pending_key.take()) {
                        let text = e.unescape()?.trim().to_string();
                        if !text.is_empty() {
                            element.add_field(key, FieldValue::Text(text));
                            if let Some(lang) = pending_lang.take() {
                                element.add_lang(key, &lang);
                            }
                        }
                    }
            }

            Ok(Event::End(_)) => {
                if depth == 2 {
                    pending_key = None;
                    pending_lang = None;
                    if let Some(element) = current.take()
                        && !element.mrid().is_empty() {
                            index(&mut ds.by_type, element.type_name(), element.mrid().to_string());
                            ds.entries.insert(element.mrid().to_string(), element);
                        }
                    if let Some(outer) = nested.take() {
                        scope = outer;
                    }
                }
                depth = depth.saturating_sub(1);
            }

            Ok(Event::Eof) => break,
            Err(e) => return Err(Box::new(e)),
            _ => {}
        }
        buf.clear();
    }

    Ok(())
}

// --- helpers ----------------------------------------------------------------


/// Strip the fragment from a URI, e.g. "http://example.com#foo" → "foo".
fn strip_fragment(s: &str) -> String {
    if let Some(i) = s.rfind('#') {
        s[i + 1..].to_string()
    } else {
        s.trim_start_matches('#').to_string()
    }
}


/// A field element's `rdf:resource` (as [`find_resource`] gives it) and
/// `xml:lang`, in one pass over its attributes.
fn field_attrs(
    attrs: quick_xml::events::attributes::Attributes<'_>,
) -> Result<(Option<String>, Option<String>), Box<dyn std::error::Error>> {
    let (mut res, mut lang) = (None, None);
    for attr in attrs.flatten() {
        match attr.key.as_ref() {
            b"rdf:resource" => res = Some(strip_fragment(std::str::from_utf8(&attr.value)?)),
            b"xml:lang" => lang = Some(std::str::from_utf8(&attr.value)?.to_string()),
            _ => {}
        }
    }
    Ok((res, lang))
}

/// Find the value of the "rdf:resource" attribute from an element.
fn find_resource(
    attrs: quick_xml::events::attributes::Attributes<'_>,
) -> Result<Option<String>, Box<dyn std::error::Error>> {
    for attr in attrs.flatten() {
        let key = std::str::from_utf8(attr.key.as_ref())?;
        if key == "rdf:resource" {
            return Ok(Some(strip_fragment(std::str::from_utf8(&attr.value)?)));
        }
    }
    Ok(None)
}
