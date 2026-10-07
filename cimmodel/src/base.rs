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
/// An element's fields, keyed by `Class.attr`.
pub type FieldMap = FastMap<String, FieldValue>;

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
    duplicate_fields: FastSet<String>,
}

impl Element {
    pub fn new(class: &'static ClassDef, mrid: String) -> Self {
        Self { class, mrid, fields: FieldMap::default(), duplicate_fields: FastSet::default() }
    }

    pub fn with_fields(class: &'static ClassDef, mrid: String, fields: FieldMap) -> Self {
        Self { class, mrid, fields, duplicate_fields: FastSet::default() }
    }

    pub fn class(&self) -> &'static ClassDef {
        self.class
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

    pub fn duplicate_fields(&self) -> &FastSet<String> {
        &self.duplicate_fields
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
    pub fn add_field(&mut self, key: &str, val: FieldValue) {
        match &val {
            FieldValue::Resource(new_ref) => match self.fields.get_mut(key) {
                Some(FieldValue::ResourceList(list)) => {
                    list.push(new_ref.clone());
                    self.duplicate_fields.insert(key.to_string());
                    return;
                }
                Some(existing @ FieldValue::Resource(_)) => {
                    let FieldValue::Resource(old) = std::mem::replace(existing, FieldValue::ResourceList(Vec::new())) else {
                        unreachable!()
                    };
                    *existing = FieldValue::ResourceList(vec![old, new_ref.clone()]);
                    self.duplicate_fields.insert(key.to_string());
                    return;
                }
                _ => {}
            },
            FieldValue::Text(new_text) => match self.fields.get_mut(key) {
                Some(FieldValue::TextList(list)) => {
                    list.push(new_text.clone());
                    self.duplicate_fields.insert(key.to_string());
                    return;
                }
                Some(existing @ FieldValue::Text(_)) => {
                    let FieldValue::Text(old) = std::mem::replace(existing, FieldValue::TextList(Vec::new())) else {
                        unreachable!()
                    };
                    *existing = FieldValue::TextList(vec![old, new_text.clone()]);
                    self.duplicate_fields.insert(key.to_string());
                    return;
                }
                _ => {}
            },
            _ => {
                if self.fields.contains_key(key) {
                    self.duplicate_fields.insert(key.to_string());
                }
            }
        }
        self.fields.insert(key.to_string(), val);
    }

    /// Combine another file's view of the same object: a later scalar wins,
    /// reference lists are joined. Duplicate tracking stays per file.
    pub fn merge_from(&mut self, other: &Element) {
        for (k, v) in &other.fields {
            match v {
                FieldValue::ResourceList(new_list) => match self.fields.get_mut(k) {
                    Some(FieldValue::ResourceList(existing)) => existing.extend(new_list.iter().cloned()),
                    _ => {
                        self.fields.insert(k.clone(), v.clone());
                    }
                },
                _ => {
                    self.fields.insert(k.clone(), v.clone());
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
            map.insert(k.clone(), field_to_json(v));
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
            fields.insert(k.clone(), value);
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
    pub ns: &'static str,
    pub kind: AttrKind,
    /// xsd type for literals, target class for associations and enums.
    pub range: &'static str,
    pub is_list: bool,
    /// Profile codes that carry this attribute.
    pub origins: &'static [&'static str],
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
    /// The attributes this class declares itself; see
    /// [`TypeRegistry::attr`] for inherited ones.
    pub attrs: &'static [AttrDef],
    pub origins: &'static [&'static str],
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
}

impl TypeRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    /// Register a family's class table. `bare_fallback` adds its classes to
    /// the bare-name fallback: an unbound prefix means the families cannot be
    /// told apart, so only the default family (CGMES) takes it.
    pub fn add_family(&mut self, classes: &'static [ClassDef], bare_fallback: bool) {
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

    /// `class` and its ancestors, root first.
    pub fn chain(&self, class: &ClassDef) -> &[&'static ClassDef] {
        self.chain.get(class.qualified).map_or(&[], Vec::as_slice)
    }

    /// Every attribute `class` declares, inherited ones included.
    pub fn attrs(&self, class: &ClassDef) -> impl Iterator<Item = &'static AttrDef> + '_ {
        self.declared.get(class.qualified).into_iter().flat_map(|m| m.values().copied())
    }
}
