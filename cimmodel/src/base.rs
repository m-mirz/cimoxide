use std::collections::{HashMap, HashSet};
use std::hash::{BuildHasherDefault, Hasher};

// --- Hashing -----------------------------------------------------------------

/// A fast, non-cryptographic hasher for the decoder's maps.
///
/// Every field read during validation and every element lookup goes through a
/// `String`-keyed map, and std's SipHash made those lookups a measurable share
/// of a validation run. This is the multiply-rotate scheme of FxHash (as in
/// `rustc-hash` 2): a word at a time, with the high bits rotated down at the
/// end because hashbrown takes its bucket index from the low bits.
///
/// Written out rather than imported so the core crates carry no dependency for
/// it. Not resistant to deliberately colliding keys, which is acceptable for a
/// tool that reads grid model files rather than serving untrusted requests.
#[derive(Default, Clone, Copy)]
pub struct FastHasher {
    hash: u64,
}

const SEED: u64 = 0xf135_7aea_2e62_a9c5;

impl FastHasher {
    #[inline]
    fn add(&mut self, word: u64) {
        self.hash = self.hash.wrapping_add(word).wrapping_mul(SEED);
    }
}

impl Hasher for FastHasher {
    #[inline]
    fn write(&mut self, bytes: &[u8]) {
        let mut chunks = bytes.chunks_exact(8);
        for c in &mut chunks {
            self.add(u64::from_le_bytes(c.try_into().expect("chunk of 8")));
        }
        let rest = chunks.remainder();
        if !rest.is_empty() {
            let mut buf = [0u8; 8];
            buf[..rest.len()].copy_from_slice(rest);
            self.add(u64::from_le_bytes(buf));
        }
    }
    #[inline]
    fn write_u8(&mut self, i: u8) {
        self.add(i as u64);
    }
    #[inline]
    fn write_u32(&mut self, i: u32) {
        self.add(i as u64);
    }
    #[inline]
    fn write_u64(&mut self, i: u64) {
        self.add(i);
    }
    #[inline]
    fn write_usize(&mut self, i: usize) {
        self.add(i as u64);
    }
    #[inline]
    fn finish(&self) -> u64 {
        self.hash.rotate_left(26)
    }
}

pub type FastBuildHasher = BuildHasherDefault<FastHasher>;
pub type FastMap<K, V> = HashMap<K, V, FastBuildHasher>;
pub type FastSet<T> = HashSet<T, FastBuildHasher>;
/// An element's fields, keyed by `Class.attr`. Keys are `&'static`: a declared
/// attribute's key is its id in the class table, any other goes through
/// [`intern`], so a key is allocated once per process rather than per field.
pub type FieldMap = FastMap<&'static str, FieldValue>;

/// The one `&'static` copy of a field key the class tables do not hold — an
/// attribute the element's class does not declare, as written in the file or
/// set from JSON. Each distinct key is leaked once and kept for the process;
/// there are as many as distinct undeclared attribute names, not fields.
pub fn intern(key: &str) -> &'static str {
    use std::cell::RefCell;
    use std::sync::{Mutex, OnceLock};
    thread_local! {
        static LOCAL: RefCell<FastSet<&'static str>> = RefCell::new(FastSet::default());
    }
    static GLOBAL: OnceLock<Mutex<FastSet<&'static str>>> = OnceLock::new();
    if let Some(k) = LOCAL.with(|l| l.borrow().get(key).copied()) {
        return k;
    }
    let k = {
        let mut global = GLOBAL.get_or_init(Default::default).lock().unwrap();
        match global.get(key) {
            Some(k) => *k,
            None => {
                let k: &'static str = Box::leak(key.to_owned().into_boxed_str());
                global.insert(k);
                k
            }
        }
    };
    LOCAL.with(|l| l.borrow_mut().insert(k));
    k
}

#[derive(Debug, Clone)]
pub enum FieldValue {
    Text(String),
    /// Repeated text element within one parsed element (e.g. the multiple
    /// `md:Model.profile` entries of a combined EQ+SC file header).
    TextList(Vec<String>),
    Resource(String),
    ResourceList(Vec<String>),
}

// --- Elements ----------------------------------------------------------------

/// One decoded CIM object: its class, its mRID, and its fields as written.
///
/// Every family decodes into this one type. The fields are the document's
/// own, keyed by `Class.attr` (the XML element's local name), with values
/// unparsed: what a class declares lives in its [`ClassDef`], not in a
/// generated type, so an attribute the class does not declare and a value
/// that does not parse are kept rather than dropped.
#[derive(Debug, Clone)]
pub struct Element {
    class: &'static ClassDef,
    mrid: String,
    fields: FieldMap,
    /// Fields assigned more than once within the element, which `sh:maxCount`
    /// reads.
    duplicate_fields: FastSet<&'static str>,
    /// The `xml:lang` of each text value written with one, by field, in the
    /// order read. `None` until a value carries one, so an element without
    /// language tags — nearly all of them — pays a pointer.
    langs: Option<Box<Langs>>,
    /// The other classes files wrote the object under, when merged datasets
    /// disagree — an SSH file's `cim:Equipment` for an EQ file's
    /// `cim:ACLineSegment`. In RDF the object has every one of those types;
    /// `class` is the most specific. `None` for nearly every element, so it
    /// costs a pointer, like `langs`.
    also: Option<Box<Types>>,
}

/// `(field key, xml:lang)` for each tagged text value of an element.
type Langs = Vec<(&'static str, Box<str>)>;

/// The classes besides its own a merged element was written under.
type Types = Vec<&'static ClassDef>;

impl Element {
    pub fn new(class: &'static ClassDef, mrid: String) -> Self {
        Self { class, mrid, fields: FieldMap::default(), duplicate_fields: FastSet::default(), langs: None, also: None }
    }

    pub fn with_fields(class: &'static ClassDef, mrid: String, fields: FieldMap) -> Self {
        Self { class, mrid, fields, duplicate_fields: FastSet::default(), langs: None, also: None }
    }

    pub fn class(&self) -> &'static ClassDef {
        self.class
    }

    /// Every class the object is typed as: its class, then any other a merged
    /// file wrote it under ([`CimDataset::merge`](crate::CimDataset::merge)).
    pub fn types(&self) -> impl Iterator<Item = &'static ClassDef> + '_ {
        std::iter::once(self.class).chain(self.also.iter().flat_map(|v| v.iter().copied()))
    }

    /// Record that a file also typed the object as `other`. Becomes its class
    /// when it is a subclass of the current one (field keys name each
    /// attribute's declaring class, so they stay valid); otherwise it is kept
    /// beside it. Returns whether the class changed.
    pub(crate) fn add_type(&mut self, other: &'static ClassDef, reg: &TypeRegistry) -> bool {
        if self.types().any(|c| c.qualified == other.qualified) {
            return false;
        }
        let promote = reg.chain(other).iter().any(|c| c.qualified == self.class.qualified);
        let kept = if promote { std::mem::replace(&mut self.class, other) } else { other };
        self.also.get_or_insert_with(Default::default).push(kept);
        promote
    }

    pub fn mrid(&self) -> &str {
        &self.mrid
    }

    /// Family-qualified class name, e.g. `"Terminal"` (CGMES) or
    /// `"nc:Contingency"`. This is the `CimDataset::by_type` key.
    pub fn type_name(&self) -> &'static str {
        self.class.qualified
    }

    /// Bare CIM class name, without the family qualifier.
    pub fn local_name(&self) -> &'static str {
        self.class.local
    }

    /// RDF namespace IRI of the declaring class, trailing delimiter included.
    pub fn type_ns(&self) -> &'static str {
        self.class.ns
    }

    pub fn fields(&self) -> &FieldMap {
        &self.fields
    }

    pub fn duplicate_fields(&self) -> &FastSet<&'static str> {
        &self.duplicate_fields
    }

    /// The `xml:lang` tags of `key`'s text values, one per value that had
    /// one. A value written without a tag has no entry, so a field whose
    /// values outnumber its tags has an untagged value.
    pub fn langs(&self, key: &str) -> impl Iterator<Item = &str> {
        self.langs.iter().flat_map(|l| l.iter()).filter(move |(k, _)| *k == key).map(|(_, l)| &**l)
    }

    /// Record the `xml:lang` of a text value just added to `key`.
    pub fn add_lang(&mut self, key: &'static str, lang: &str) {
        self.langs.get_or_insert_with(Default::default).push((key, lang.into()));
    }

    pub fn get(&self, attr: &str) -> Option<&FieldValue> {
        self.fields.get(attr)
    }

    pub fn get_str(&self, attr: &str) -> Option<&str> {
        match self.fields.get(attr) {
            Some(FieldValue::Text(s)) => Some(s.as_str()),
            Some(FieldValue::TextList(v)) => v.first().map(String::as_str),
            _ => None,
        }
    }

    pub fn get_f64(&self, attr: &str) -> Option<f64> {
        self.get_str(attr)?.parse().ok()
    }

    pub fn get_bool(&self, attr: &str) -> Option<bool> {
        match self.get_str(attr)? {
            "true" | "1" => Some(true),
            "false" | "0" => Some(false),
            _ => None,
        }
    }

    /// Referenced MRIDs.
    ///
    /// Always a slice, even where the schema says the association is 1:1 — CGMES
    /// has widened 1:1 to 1:many between versions, and a caller written against
    /// a slice does not have to change when that happens.
    pub fn get_refs(&self, attr: &str) -> &[String] {
        match self.fields.get(attr) {
            Some(FieldValue::Resource(s)) => std::slice::from_ref(s),
            Some(FieldValue::ResourceList(v)) => v.as_slice(),
            _ => &[],
        }
    }

    /// First referenced MRID, for associations known to be single-valued.
    pub fn get_ref(&self, attr: &str) -> Option<&str> {
        self.get_refs(attr).first().map(String::as_str)
    }

    /// Add one value as the decoder reads it: a repeated key becomes a list
    /// and is recorded in [`Element::duplicate_fields`].
    pub fn add_field(&mut self, key: &'static str, val: FieldValue) {
        match &val {
            FieldValue::Resource(new_ref) => match self.fields.get_mut(key) {
                Some(FieldValue::ResourceList(list)) => {
                    list.push(new_ref.clone());
                    self.duplicate_fields.insert(key);
                    return;
                }
                Some(existing @ FieldValue::Resource(_)) => {
                    let FieldValue::Resource(old) = std::mem::replace(existing, FieldValue::ResourceList(Vec::new())) else {
                        unreachable!()
                    };
                    *existing = FieldValue::ResourceList(vec![old, new_ref.clone()]);
                    self.duplicate_fields.insert(key);
                    return;
                }
                _ => {}
            },
            FieldValue::Text(new_text) => match self.fields.get_mut(key) {
                Some(FieldValue::TextList(list)) => {
                    list.push(new_text.clone());
                    self.duplicate_fields.insert(key);
                    return;
                }
                Some(existing @ FieldValue::Text(_)) => {
                    let FieldValue::Text(old) = std::mem::replace(existing, FieldValue::TextList(Vec::new())) else {
                        unreachable!()
                    };
                    *existing = FieldValue::TextList(vec![old, new_text.clone()]);
                    self.duplicate_fields.insert(key);
                    return;
                }
                _ => {}
            },
            _ => {
                if self.fields.contains_key(key) {
                    self.duplicate_fields.insert(key);
                }
            }
        }
        self.fields.insert(key, val);
    }

    /// Combine another file's view of the same object: a later scalar wins,
    /// reference lists are joined. Duplicate tracking stays per file.
    pub fn merge_from(&mut self, other: &Element) {
        for (k, v) in &other.fields {
            match v {
                FieldValue::ResourceList(new_list) => match self.fields.get_mut(k) {
                    Some(FieldValue::ResourceList(existing)) => existing.extend(new_list.iter().cloned()),
                    _ => {
                        self.fields.insert(*k, v.clone());
                    }
                },
                _ => {
                    self.fields.insert(*k, v.clone());
                    // The later file's text replaced ours, tags included.
                    if let Some(l) = self.langs.as_mut() {
                        l.retain(|(key, _)| key != k);
                    }
                    for lang in other.langs(k) {
                        self.add_lang(k, lang);
                    }
                }
            }
        }
    }

    /// `{"id": mRID, "Class.attr": value, ...}`: a value is a string, a
    /// repeated one an array of strings.
    pub fn to_json_value(&self) -> serde_json::Value {
        let mut map = serde_json::Map::with_capacity(self.fields.len() + 1);
        map.insert("id".to_string(), serde_json::Value::String(self.mrid.clone()));
        for (k, v) in &self.fields {
            map.insert(k.to_string(), field_to_json(v));
        }
        serde_json::Value::Object(map)
    }

    /// The inverse of [`Element::to_json_value`]. Whether a string is a
    /// reference or text comes from the attribute's declaration; an attribute
    /// the class does not declare is read as text.
    pub fn from_json(
        class: &'static ClassDef,
        reg: &TypeRegistry,
        value: &serde_json::Value,
    ) -> Result<Self, String> {
        let obj = value.as_object().ok_or("element is not a JSON object")?;
        let mrid = obj.get("id").and_then(|v| v.as_str()).ok_or("element has no \"id\"")?.to_string();
        let mut fields = FieldMap::default();
        for (k, v) in obj {
            if k == "id" || k == "_type" {
                continue;
            }
            let is_ref = reg.attr(class, k).is_some_and(|a| a.kind != AttrKind::Literal);
            let value = match v {
                serde_json::Value::String(s) if is_ref => FieldValue::Resource(s.clone()),
                serde_json::Value::String(s) => FieldValue::Text(s.clone()),
                serde_json::Value::Array(items) => {
                    let items: Vec<String> = items
                        .iter()
                        .map(|i| i.as_str().map(str::to_string).ok_or(format!("{k}: list item is not a string")))
                        .collect::<Result<_, _>>()?;
                    if is_ref { FieldValue::ResourceList(items) } else { FieldValue::TextList(items) }
                }
                other => return Err(format!("{k}: expected a string or an array, got {other}")),
            };
            fields.insert(reg.field_key(class, k), value);
        }
        Ok(Self::with_fields(class, mrid, fields))
    }
}

fn field_to_json(v: &FieldValue) -> serde_json::Value {
    match v {
        FieldValue::Text(s) | FieldValue::Resource(s) => serde_json::Value::String(s.clone()),
        FieldValue::TextList(v) | FieldValue::ResourceList(v) => {
            serde_json::Value::Array(v.iter().cloned().map(serde_json::Value::String).collect())
        }
    }
}

// --- Classes -----------------------------------------------------------------

/// How an attribute's value is carried in RDF/XML.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AttrKind {
    /// A literal, written as element text.
    Literal,
    /// A reference to another object, written as `rdf:resource`.
    Association,
    /// An enumeration value, written as `rdf:resource` to a vocabulary IRI.
    Enum,
}

/// One attribute of a class, as the schema declares it.
#[derive(Debug)]
pub struct AttrDef {
    /// `Class.attr`, matching the key in [`Element::fields`].
    pub id: &'static str,
    /// The attribute's RDF namespace: its predicate IRI is `ns` + `id`.
    pub ns: &'static str,
    pub kind: AttrKind,
    /// The CIM datatype of a literal; the class an association or enumeration
    /// points to.
    pub range: &'static str,
    pub is_list: bool,
    /// Whether a profile exchanges this end of the association — `false` for the
    /// inverse ends the vocabularies declare but no dataset writes. Literals and
    /// enumerations are always used.
    pub used: bool,
    /// The XSD datatype of a literal, as the local name after
    /// `http://www.w3.org/2001/XMLSchema#` (`"double"`); `""` otherwise.
    pub xsd: &'static str,
    /// The namespace of an enumeration's values (the decoder keeps a value after
    /// its `#`); `""` otherwise.
    pub value_ns: &'static str,
    /// Profile codes that carry this attribute.
    pub origins: &'static [&'static str],
    /// The vocabulary's `rdfs:comment`, HTML removed and on one line; `""`
    /// if it has none.
    pub comment: &'static str,
}

/// What one family's schema says, as a table: its classes, the profiles it
/// defines and the namespaces its vocabularies bind. Generated by `cimgen`, or
/// read from RDFS at runtime (`schema_source`).
#[derive(Debug)]
pub struct Schema {
    pub classes: &'static [ClassDef],
    /// Profile code → profile URI, the value a dataset's header declares; sorted
    /// by code.
    pub profiles: &'static [(&'static str, &'static str)],
    /// XML prefix → namespace IRI; sorted by prefix.
    pub namespaces: &'static [(&'static str, &'static str)],
    /// Enumerations, sorted by name.
    pub enums: &'static [EnumDef],
}

impl Schema {
    /// The enumeration called `name` (`WindingConnection`).
    pub fn enumeration(&self, name: &str) -> Option<&'static EnumDef> {
        let enums: &'static [EnumDef] = self.enums;
        enums.binary_search_by_key(&name, |e| e.local).ok().map(|i| &enums[i])
    }

    /// The enumeration value `id` names, as the decoder keeps it
    /// (`WindingConnection.D`), with its enumeration.
    pub fn enum_value(&self, id: &str) -> Option<(&'static EnumDef, &'static EnumValueDef)> {
        let e = self.enumeration(id.split_once('.')?.0)?;
        Some((e, e.values.iter().find(|v| v.id == id)?))
    }
}

/// An enumeration of the schema and the values it allows.
#[derive(Debug)]
pub struct EnumDef {
    pub ns: &'static str,
    pub local: &'static str,
    /// The vocabulary's `rdfs:comment`, as for [`ClassDef::comment`].
    pub comment: &'static str,
    /// In vocabulary order.
    pub values: &'static [EnumValueDef],
}

/// One value of an enumeration.
#[derive(Debug)]
pub struct EnumValueDef {
    /// `Enum.value`, as the decoder keeps a value after its `#`.
    pub id: &'static str,
    /// The vocabulary's `rdfs:comment`, as for [`ClassDef::comment`].
    pub comment: &'static str,
}

/// A class of the schema: what it is called and what it declares.
#[derive(Debug)]
pub struct ClassDef {
    pub ns: &'static str,
    pub local: &'static str,
    /// Family-qualified name; the `CimDataset::by_type` key.
    pub qualified: &'static str,
    /// Index of the super class within the same table.
    pub super_class: Option<usize>,
    pub concrete: bool,
    /// The attributes this class declares itself, unused association ends
    /// included ([`AttrDef::used`]); see [`TypeRegistry::attr`] for inherited
    /// ones.
    pub attrs: &'static [AttrDef],
    pub origins: &'static [&'static str],
    /// The vocabulary's `rdfs:comment`, HTML removed and on one line; `""`
    /// if it has none.
    pub comment: &'static str,
}

impl ClassDef {
    /// An attribute this class declares itself (not an inherited one).
    pub fn attr(&self, id: &str) -> Option<&'static AttrDef> {
        self.attrs.iter().find(|a| a.id == id)
    }
}

/// Namespace-aware class lookup, and every attribute each class declares.
///
/// RDF/XML identifies a class by `(namespace, local name)`, and the two profile
/// families overlap on 164 local names — both even spell the prefix `cim`, so
/// only the xmlns binding distinguishes them. Lookup is therefore keyed on the
/// resolved namespace, with a bare-name fallback (CGMES only) that preserves
/// the historical behaviour for files whose prefixes are unbound or unexpected.
#[derive(Default)]
pub struct TypeRegistry {
    by_ns: FastMap<&'static str, FastMap<&'static str, &'static ClassDef>>,
    bare: FastMap<&'static str, &'static ClassDef>,
    by_type_name: FastMap<&'static str, &'static ClassDef>,
    /// Qualified class name → every attribute it declares, inherited ones
    /// included. Built once from the class tables.
    declared: FastMap<&'static str, FastMap<&'static str, &'static AttrDef>>,
    /// Qualified class name → the class and its ancestors, root first.
    chain: FastMap<&'static str, Vec<&'static ClassDef>>,
    /// Each registered family, in registration order.
    families: Vec<FamilyTable>,
}

/// One family as registered: its schema, the `by_type` prefix its classes
/// carry (`""` for CGMES, `"nc:"`), and its attributes by id.
struct FamilyTable {
    id: &'static str,
    prefix: &'static str,
    schema: &'static Schema,
    /// Attribute id → its first declaration in table order. An id names one
    /// predicate, whichever class declares it.
    attrs: FastMap<&'static str, &'static AttrDef>,
}

impl TypeRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    /// Register a family's schema under `id`. `bare_fallback` adds its classes
    /// to the bare-name fallback: an unbound prefix means the families cannot
    /// be told apart, so only the default family (CGMES) takes it.
    pub fn add_family(&mut self, id: &'static str, schema: &'static Schema, bare_fallback: bool) {
        let classes = schema.classes;
        let prefix = classes
            .first()
            .and_then(|c| c.qualified.strip_suffix(c.local))
            .unwrap_or("");
        let mut by_id = FastMap::default();
        for class in classes {
            for a in class.attrs {
                by_id.entry(a.id).or_insert(a);
            }
        }
        self.families.push(FamilyTable { id, prefix, schema, attrs: by_id });
        for class in classes {
            self.by_ns.entry(class.ns).or_default().insert(class.local, class);
            self.by_type_name.insert(class.qualified, class);
            if bare_fallback {
                self.bare.insert(class.local, class);
            }
            let mut attrs = FastMap::default();
            let mut chain = Vec::new();
            let mut next = Some(class);
            while let Some(c) = next {
                for a in c.attrs {
                    attrs.entry(a.id).or_insert(a);
                }
                chain.push(c);
                next = c.super_class.map(|i| &classes[i]);
            }
            chain.reverse();
            self.declared.insert(class.qualified, attrs);
            self.chain.insert(class.qualified, chain);
        }
    }

    /// Lookup table for one namespace, resolved once per XML prefix per file.
    pub fn ns_table(&self, ns: &str) -> Option<&FastMap<&'static str, &'static ClassDef>> {
        self.by_ns.get(ns)
    }

    pub fn bare(&self) -> &FastMap<&'static str, &'static ClassDef> {
        &self.bare
    }

    pub fn by_type_name(&self, name: &str) -> Option<&'static ClassDef> {
        self.by_type_name.get(name).copied()
    }

    /// The attribute `id` if `class` declares it, itself or by inheritance.
    pub fn attr(&self, class: &ClassDef, id: &str) -> Option<&'static AttrDef> {
        self.declared.get(class.qualified)?.get(id).copied()
    }

    /// The attributes `class` declares, inherited ones included, by id — for a
    /// caller that resolves many keys of one element.
    pub fn declared(&self, class: &ClassDef) -> Option<&FastMap<&'static str, &'static AttrDef>> {
        self.declared.get(class.qualified)
    }

    /// The `&'static` key a field `id` of `class` is stored under.
    pub fn field_key(&self, class: &ClassDef, id: &str) -> &'static str {
        self.attr(class, id).map_or_else(|| intern(id), |a| a.id)
    }

    /// `class` and its ancestors, root first.
    pub fn chain(&self, class: &ClassDef) -> &[&'static ClassDef] {
        self.chain.get(class.qualified).map_or(&[], Vec::as_slice)
    }

    /// The `by_type` keys of the concrete classes at or below `qualified`,
    /// sorted: what instances of a class can be typed as.
    pub fn concrete_descendants(&self, qualified: &str) -> Vec<&'static str> {
        let mut out: Vec<&'static str> = self
            .chain
            .iter()
            .filter(|(_, chain)| chain.iter().any(|c| c.qualified == qualified))
            .filter_map(|(q, _)| self.by_type_name.get(q).filter(|c| c.concrete).map(|c| c.qualified))
            .collect();
        out.sort_unstable();
        out
    }

    /// Every attribute `class` declares, inherited ones included.
    pub fn attrs(&self, class: &ClassDef) -> impl Iterator<Item = &'static AttrDef> + '_ {
        self.declared.get(class.qualified).into_iter().flat_map(|m| m.values().copied())
    }

    /// The schema registered as `family` (`"cgmes"`, `"nc"`).
    pub fn schema(&self, family: &str) -> Option<&'static Schema> {
        self.family(family).map(|f| f.schema)
    }

    fn family(&self, id: &str) -> Option<&FamilyTable> {
        self.families.iter().find(|f| f.id == id)
    }

    /// The enumeration value `id` (`WindingConnection.D`) in the family of
    /// `class`, which an element of `class` uses it on.
    pub fn enum_value(&self, class: &ClassDef, id: &str) -> Option<(&'static EnumDef, &'static EnumValueDef)> {
        self.schema(self.family_of(class.qualified)?)?.enum_value(id)
    }

    /// The family a `by_type` key belongs to, by its prefix.
    pub fn family_of(&self, type_name: &str) -> Option<&'static str> {
        // The longest matching prefix, so `nc:X` is not taken for CGMES's "".
        self.families
            .iter()
            .filter(|f| type_name.starts_with(f.prefix))
            .max_by_key(|f| f.prefix.len())
            .map(|f| f.id)
    }

    /// The attribute `id` as `family` declares it on any class — what a field
    /// key names when the element's own class does not declare it.
    pub fn family_attr(&self, family: &str, id: &str) -> Option<&'static AttrDef> {
        self.family(family)?.attrs.get(id).copied()
    }

    /// The attribute a field `key` of an element of `class` stands for: the
    /// class's own declaration, else the family's.
    pub fn attr_of(&self, class: &ClassDef, key: &str) -> Option<&'static AttrDef> {
        self.attr(class, key)
            .or_else(|| self.family_attr(self.family_of(class.qualified)?, key))
    }
}
