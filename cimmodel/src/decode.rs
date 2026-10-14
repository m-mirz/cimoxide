use std::path::Path;

use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;

use crate::base::{intern, AttrDef, AttrKind, ClassDef, Element, FastMap, FastSet, FieldValue, TypeRegistry};
use crate::registry;

/// Whether `text` looks like CGMES or NC RDF/XML: it binds one of the two
/// CIM namespaces near the top.
pub fn is_cim(text: &str) -> bool {
    let mut end = text.len().min(8192);
    while !text.is_char_boundary(end) {
        end -= 1;
    }
    let head = &text[..end];
    head.contains("http://iec.ch/TC57/") || head.contains("https://cim.ucaiug.io/ns#")
}

/// How a dataset's files wrote one field key ([`CimDataset::field_forms`]).
/// Each part is the first the files showed; one key written two ways keeps
/// the first.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct FieldForm {
    /// The key's namespace IRI, for a key the class tables do not declare
    /// (`conformsTo` → `http://purl.org/dc/terms/`); a declared attribute's is
    /// its [`AttrDef::ns`].
    pub ns: Option<&'static str>,
    /// The `rdf:datatype` its literals carried (`dcat:temporalResolution` →
    /// `xsd:duration`).
    pub datatype: Option<&'static str>,
    /// The namespace its references were cut from, when not the document's
    /// own (`#_x`) and not an enumeration's, whose namespace the class table
    /// holds: a `PropertyReference` to `http://iec.ch/TC57/CIM100#Switch.open`
    /// decodes as `Switch.open`.
    pub resource_ns: Option<&'static str>,
}

impl FieldForm {
    fn fill_from(&mut self, other: FieldForm) {
        self.ns = self.ns.or(other.ns);
        self.datatype = self.datatype.or(other.datatype);
        self.resource_ns = self.resource_ns.or(other.resource_ns);
    }
}

pub struct CimDataset {
    pub entries: FastMap<String, Element>,
    /// Maps `type_name()` → list of MRIDs of that type. Populated on insert, maintained on merge.
    pub by_type: FastMap<String, Vec<String>>,
    /// How the files wrote each field key, where the class tables cannot tell
    /// and the decoded value no longer shows: what the encoder needs to write
    /// the field back as it was read.
    pub field_forms: FastMap<&'static str, FieldForm>,
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
            field_forms: FastMap::default(),
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
    ///
    /// Where the files type an object differently — an SSH file writing
    /// `cim:Equipment` for what the EQ file wrote as `cim:ACLineSegment` — the
    /// object keeps every type, as the RDF union does ([`Element::types`]),
    /// and its class (and `by_type` bucket) is the most specific, whichever
    /// file came first. Of unrelated classes the first stays the class.
    pub fn merge(&mut self, other: CimDataset) {
        for (key, form) in other.field_forms {
            self.field_forms.entry(key).or_default().fill_from(form);
        }
        let reg = registry::type_registry();
        let mut moved: FastMap<&'static str, FastSet<String>> = FastMap::default();
        for (mrid, incoming) in other.entries {
            if let Some(existing) = self.entries.get_mut(&mrid) {
                existing.merge_from(&incoming);
                let old = existing.class();
                let mut changed = false;
                for t in incoming.types() {
                    changed |= existing.add_type(t, reg);
                }
                if changed {
                    moved.entry(old.qualified).or_default().insert(mrid.clone());
                    self.by_type.entry(existing.type_name().to_string()).or_default().push(mrid);
                }
            } else {
                let type_name = incoming.type_name().to_string();
                self.by_type.entry(type_name).or_default().push(mrid.clone());
                self.entries.insert(mrid, incoming);
            }
        }
        // One pass per bucket left, however many of its objects moved.
        for (old, mrids) in moved {
            if let Some(bucket) = self.by_type.get_mut(old) {
                bucket.retain(|m| !mrids.contains(m));
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
    /// Prefix → its namespace IRI (interned) and that namespace's table.
    prefixes: Vec<(Vec<u8>, &'static str, Option<&'static Table>)>,
    default: Option<&'static Table>,
    default_ns: &'static str,
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
            let ns = intern(ns);
            match rest.strip_prefix(b":".as_slice()) {
                Some(prefix) => {
                    let prefix = prefix.to_vec();
                    match self.prefixes.iter_mut().find(|(k, _, _)| *k == prefix) {
                        Some(slot) => (slot.1, slot.2) = (ns, table),
                        None => self.prefixes.push((prefix, ns, table)),
                    }
                }
                None if rest.is_empty() => (self.default, self.default_ns) = (table, ns),
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
                .find(|(k, _, _)| k.as_slice() == p)
                .and_then(|(_, _, t)| *t),
        }
    }

    /// The namespace IRI `prefix` is bound to, `""` if none.
    fn ns(&self, prefix: Option<&[u8]>) -> &'static str {
        match prefix {
            None => self.default_ns,
            Some(p) => self.prefixes.iter().find(|(k, _, _)| k.as_slice() == p).map_or("", |(_, ns, _)| ns),
        }
    }

}

/// One pass over an element's attributes, yielding what the parser needs: the
/// MRID, whether the element only describes the object (`rdf:about`, no
/// `rdf:ID`) and whether it redeclares any namespace. Scanning twice
/// costs more than the nested-`xmlns` case it would serve, because every element
/// pays for it and almost none declare one.
fn scan_attrs(e: &BytesStart) -> Result<(String, bool, bool), Box<dyn std::error::Error>> {
    let mut mrid = String::new();
    let mut described = false;
    let mut defined = false;
    let mut has_xmlns = false;
    for attr in e.attributes().flatten() {
        let key = attr.key.as_ref();
        if key == b"rdf:about" || key == b"rdf:ID" {
            mrid = strip_fragment(std::str::from_utf8(&attr.value)?);
            described |= key == b"rdf:about";
            defined |= key == b"rdf:ID";
        } else if key.starts_with(b"xmlns") {
            has_xmlns = true;
        }
    }
    Ok((mrid, described && !defined, has_xmlns))
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
    // The literal field's text so far. Stored at its end tag, as written —
    // untrimmed, and empty for `<cim:X></cim:X>` — since a comment or CDATA
    // section splits it into several events.
    let mut pending_text = String::new();
    // A field element's key as stored — the class table's id when the class
    // declares it, else the interned name — and its attributes. What the key
    // and value lose of how it was written goes to `field_forms`: an
    // undeclared key's namespace, a datatype, the namespace a reference was
    // cut from.
    let field = |declared: Option<&'static FastMap<&'static str, &'static AttrDef>>,
                 e: &BytesStart,
                 scope: &Scope,
                 forms: &mut FastMap<&'static str, FieldForm>|
     -> Result<(&'static str, FieldAttrs), Box<dyn std::error::Error>> {
        let name = e.name();
        let (prefix, local_bytes) = split_qname(name.as_ref());
        let local = std::str::from_utf8(local_bytes)?;
        let attr = declared.and_then(|d| d.get(local));
        let key = attr.map_or_else(|| intern(local), |a| a.id);
        let attrs = field_attrs(e.attributes())?;
        let ns = scope.ns(prefix);
        let undeclared_ns = (attr.is_none() && !ns.is_empty()).then_some(ns);
        let resource_ns = attrs.resource_ns.filter(|_| attr.is_none_or(|a| a.kind != AttrKind::Enum));
        if undeclared_ns.is_some() || attrs.datatype.is_some() || resource_ns.is_some() {
            forms.entry(key).or_default().fill_from(FieldForm {
                ns: undeclared_ns,
                datatype: attrs.datatype,
                resource_ns,
            });
        }
        Ok((key, attrs))
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
                        let (mrid, described, has_xmlns) = scan_attrs(e)?;
                        if has_xmlns {
                            let mut inner = Scope {
                                prefixes: scope.prefixes.clone(),
                                default: scope.default,
                                default_ns: scope.default_ns,
                            };
                            inner.absorb(e, reg);
                            nested = Some(std::mem::replace(&mut scope, inner));
                        }

                        current = resolve(&scope, reg, e.name().as_ref())?.map(|class| {
                            let mut element = Element::new(class, mrid);
                            if described {
                                element.mark_described();
                            }
                            element
                        });
                        declared = current.as_ref().and_then(|c| reg.declared(c.class()));
                    }
                    3 => {
                        if let Some(element) = current.as_mut() {
                            let (key, attrs) = field(declared, e, &scope, &mut ds.field_forms)?;
                            if let Some(res) = attrs.resource {
                                element.add_field(key, FieldValue::Resource(res));
                            } else if !attrs.nodes {
                                pending_key = Some(key);
                                pending_lang = attrs.lang;
                                pending_text.clear();
                            }
                        }
                    }
                    // A child element: the field holds statements or XML
                    // (`rdf:parseType`), not a literal.
                    _ => pending_key = None,
                }
            }

            Ok(Event::Empty(ref e)) => {
                match depth {
                    1 => {
                        // Top-level self-closing element: <cim:Foo rdf:ID="x" />
                        // It can carry its own xmlns; with no children there is
                        // nothing to restore afterwards, so resolve against a
                        // throwaway scope.
                        let (mrid, described, has_xmlns) = scan_attrs(e)?;
                        let local_scope = if has_xmlns {
                            let mut inner = Scope {
                                prefixes: scope.prefixes.clone(),
                                default: scope.default,
                                default_ns: scope.default_ns,
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
                                let mut element = Element::new(class, mrid.clone());
                                if described {
                                    element.mark_described();
                                }
                                ds.entries.insert(mrid, element);
                            }
                    }
                    2 => {
                        // Self-closing field element within the current type
                        // block: a reference, or else an empty literal.
                        if let Some(element) = current.as_mut() {
                            let (key, attrs) = field(declared, e, &scope, &mut ds.field_forms)?;
                            if let Some(res) = attrs.resource {
                                element.add_field(key, FieldValue::Resource(res));
                            } else if !attrs.nodes {
                                element.add_field(key, FieldValue::Text(String::new()));
                                if let Some(lang) = attrs.lang {
                                    element.add_lang(key, &lang);
                                }
                            }
                        }
                    }
                    _ => pending_key = None,
                }
            }

            Ok(Event::Text(ref e)) if depth == 3 && pending_key.is_some() => {
                pending_text.push_str(&e.unescape()?);
            }

            Ok(Event::CData(ref e)) if depth == 3 && pending_key.is_some() => {
                pending_text.push_str(std::str::from_utf8(e)?);
            }

            Ok(Event::End(_)) => {
                if depth == 3
                    && let (Some(element), Some(key)) = (current.as_mut(), pending_key.take()) {
                        element.add_field(key, FieldValue::Text(std::mem::take(&mut pending_text)));
                        if let Some(lang) = pending_lang.take() {
                            element.add_lang(key, &lang);
                        }
                    }
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
///
/// An IRI whose fragment is empty names a vocabulary, not an object
/// (`dcterms:conformsTo rdf:resource="http://iec.ch/TC57/CIM100#"`), and is
/// kept whole: stripped, it was the empty string, which no encoder can write
/// back.
fn strip_fragment(s: &str) -> String {
    match s.rfind('#') {
        Some(i) if i + 1 == s.len() && i > 0 => s.to_string(),
        Some(i) => s[i + 1..].to_string(),
        None => s.to_string(),
    }
}


/// What a field element's attributes say about its value.
struct FieldAttrs {
    /// `rdf:resource`, as [`strip_fragment`] keeps it.
    resource: Option<String>,
    /// The part of `rdf:resource` that was cut off, when the IRI has one
    /// before its `#` (not a reference into the document itself).
    resource_ns: Option<&'static str>,
    lang: Option<String>,
    datatype: Option<&'static str>,
    /// Its `rdf:parseType` makes its content nodes rather than a literal
    /// (`Resource`, `Collection`, a difference model's `Statements`; not
    /// `Literal`).
    nodes: bool,
}

/// A field element's attributes, in one pass.
fn field_attrs(attrs: quick_xml::events::attributes::Attributes<'_>) -> Result<FieldAttrs, Box<dyn std::error::Error>> {
    let mut out = FieldAttrs { resource: None, resource_ns: None, lang: None, datatype: None, nodes: false };
    for attr in attrs.flatten() {
        match attr.key.as_ref() {
            b"rdf:resource" => {
                let iri = std::str::from_utf8(&attr.value)?;
                let kept = strip_fragment(iri);
                if kept.len() < iri.len() && kept.len() + 1 < iri.len() {
                    out.resource_ns = Some(intern(&iri[..iri.len() - kept.len()]));
                }
                out.resource = Some(kept);
            }
            b"xml:lang" => out.lang = Some(std::str::from_utf8(&attr.value)?.to_string()),
            b"rdf:datatype" => out.datatype = Some(intern(std::str::from_utf8(&attr.value)?)),
            b"rdf:parseType" => out.nodes = attr.value.as_ref() != b"Literal",
            _ => {}
        }
    }
    Ok(out)
}
