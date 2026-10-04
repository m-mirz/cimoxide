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

pub trait CimElement: Send + Sync {
    fn mrid(&self) -> &str;
    /// Family-qualified name, e.g. `"Terminal"` (CGMES) or `"nc:Contingency"`.
    /// This is the `CimDataset::by_type` key.
    fn type_name(&self) -> &'static str;
    /// RDF namespace IRI of the declaring class, trailing delimiter included.
    fn type_ns(&self) -> &'static str {
        ""
    }
    /// Bare CIM class name, without the family qualifier.
    fn local_name(&self) -> &'static str {
        self.type_name()
    }
    fn as_any(&self) -> &dyn std::any::Any;
    fn to_json_value(&self) -> serde_json::Value;
    fn to_block(&self) -> RdfBlock;
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

#[derive(Debug, Clone)]
pub struct RdfBlock {
    /// Family-qualified type name, matching `CimElement::type_name`. Unique
    /// across families, so it is enough to re-dispatch a block on merge.
    pub type_name: String,
    pub mrid: String,
    pub fields: FieldMap,
    /// Field names (local XML element name) that were assigned more than once
    /// within a single parsed element, indicating a MaxCount violation.
    pub duplicate_fields: FastSet<String>,
}

impl Default for RdfBlock {
    fn default() -> Self {
        Self {
            type_name: String::new(),
            mrid: String::new(),
            fields: FieldMap::default(),
            duplicate_fields: FastSet::default(),
        }
    }
}

impl RdfBlock {
    pub fn merge_from(&mut self, other: &RdfBlock) {
        for (k, v) in &other.fields {
            match v {
                FieldValue::ResourceList(new_list) => {
                    match self.fields.get_mut(k) {
                        Some(FieldValue::ResourceList(existing)) => {
                            existing.extend(new_list.iter().cloned())
                        }
                        _ => {
                            self.fields.insert(k.clone(), v.clone());
                        }
                    }
                }
                _ => {
                    self.fields.insert(k.clone(), v.clone());
                }
            }
        }
    }
}

pub type ParseFn = fn(&RdfBlock) -> Box<dyn CimElement>;

/// How a resolved class turns a block into an element.
///
/// A generated struct has a compiled constructor. A class described only by
/// data cannot: `ParseFn` is a bare function pointer, and the generated rows
/// only work because each is a distinct non-capturing closure with a constant
/// index baked in. A table built at runtime has no such closures, so the
/// `ClassDef` travels in the entry instead.
#[derive(Clone, Copy)]
pub enum Dispatch {
    Typed(ParseFn),
    Bag(&'static ClassDef),
}

#[derive(Clone, Copy)]
pub struct TypeEntry {
    pub type_name: &'static str,
    pub dispatch: Dispatch,
}

impl TypeEntry {
    /// Build the element. The branch lives here so the decoder's hot path has
    /// one call site rather than a match at each of its two constructors.
    pub fn parse(&self, b: &RdfBlock) -> Box<dyn CimElement> {
        match self.dispatch {
            Dispatch::Typed(f) => f(b),
            Dispatch::Bag(class) => Box::new(GenericElement::from_block(class, b)),
        }
    }
}

/// Namespace-aware type dispatch.
///
/// RDF/XML identifies a class by `(namespace, local name)`, and the two profile
/// families overlap on 164 local names — both even spell the prefix `cim`, so
/// only the xmlns binding distinguishes them. Lookup is therefore keyed on the
/// resolved namespace, with a bare-name fallback that preserves the historical
/// behaviour for files whose prefixes are unbound or unexpected.
pub struct TypeRegistry {
    by_ns: FastMap<&'static str, FastMap<&'static str, TypeEntry>>,
    bare: FastMap<&'static str, TypeEntry>,
    by_type_name: FastMap<&'static str, TypeEntry>,
}

impl TypeRegistry {
    pub fn from_rows(
        rows: &'static [(&'static str, &'static str, &'static str, ParseFn)],
        bare_rows: &'static [(&'static str, &'static str, ParseFn)],
    ) -> Self {
        let mut by_ns: FastMap<&'static str, FastMap<&'static str, TypeEntry>> = FastMap::default();
        let mut by_type_name = FastMap::default();
        for (ns, local, type_name, parse) in rows {
            let entry = TypeEntry { type_name, dispatch: Dispatch::Typed(*parse) };
            by_ns.entry(ns).or_default().insert(local, entry);
            by_type_name.insert(*type_name, entry);
        }
        let bare = bare_rows
            .iter()
            .map(|(local, type_name, parse)| {
                (*local, TypeEntry { type_name, dispatch: Dispatch::Typed(*parse) })
            })
            .collect();
        Self { by_ns, bare, by_type_name }
    }

    /// Register a family described by data rather than generated structs.
    ///
    /// Deliberately not added to the bare-name fallback: an unbound prefix
    /// means the families cannot be told apart, and the historical
    /// default-family guess is the safer one.
    pub fn add_bag_family(&mut self, classes: &'static [ClassDef]) {
        for class in classes {
            let entry = TypeEntry {
                type_name: class.qualified,
                dispatch: Dispatch::Bag(class),
            };
            self.by_ns
                .entry(class.ns)
                .or_default()
                .insert(class.local, entry);
            self.by_type_name.insert(class.qualified, entry);
        }
    }

    /// Dispatch table for one namespace, resolved once per XML prefix per file.
    pub fn ns_table(&self, ns: &str) -> Option<&FastMap<&'static str, TypeEntry>> {
        self.by_ns.get(ns)
    }

    pub fn bare(&self) -> &FastMap<&'static str, TypeEntry> {
        &self.bare
    }

    /// Re-dispatch an already-decoded element, used when merging datasets.
    pub fn by_type_name(&self, name: &str) -> Option<TypeEntry> {
        self.by_type_name.get(name).copied()
    }
}

// --- Property-bag elements ---------------------------------------------------

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
    /// `Class.attr`, matching the key in [`RdfBlock::fields`].
    pub id: &'static str,
    pub ns: &'static str,
    pub kind: AttrKind,
    /// xsd type for literals, target class for associations and enums.
    pub range: &'static str,
    pub is_list: bool,
    /// Profile codes that carry this attribute.
    pub origins: &'static [&'static str],
}

/// A class represented as a property bag rather than a generated struct.
#[derive(Debug)]
pub struct ClassDef {
    pub ns: &'static str,
    pub local: &'static str,
    /// Family-qualified name; the `CimDataset::by_type` key.
    pub qualified: &'static str,
    /// Index of the super class within the same table.
    pub super_class: Option<usize>,
    pub concrete: bool,
    pub attrs: &'static [AttrDef],
    pub origins: &'static [&'static str],
}

impl ClassDef {
    pub fn attr(&self, id: &str) -> Option<&'static AttrDef> {
        self.attrs.iter().find(|a| a.id == id)
    }
}

/// An element of a family that is not generated as typed structs.
///
/// Attributes are addressed by their RDF id (`"IdentifiedObject.mRID"`), the
/// same key [`RdfBlock::fields`] uses.
#[derive(Debug, Clone)]
pub struct GenericElement {
    class: &'static ClassDef,
    mrid: String,
    fields: FieldMap,
}

impl GenericElement {
    pub fn from_block(class: &'static ClassDef, b: &RdfBlock) -> Self {
        Self { class, mrid: b.mrid.clone(), fields: b.fields.clone() }
    }

    pub fn class_def(&self) -> &'static ClassDef {
        self.class
    }

    pub fn fields(&self) -> &FieldMap {
        &self.fields
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
}

fn field_to_json(v: &FieldValue) -> serde_json::Value {
    match v {
        FieldValue::Text(s) | FieldValue::Resource(s) => serde_json::Value::String(s.clone()),
        FieldValue::TextList(v) | FieldValue::ResourceList(v) => {
            serde_json::Value::Array(v.iter().cloned().map(serde_json::Value::String).collect())
        }
    }
}

impl CimElement for GenericElement {
    fn mrid(&self) -> &str {
        &self.mrid
    }
    fn type_name(&self) -> &'static str {
        self.class.qualified
    }
    fn type_ns(&self) -> &'static str {
        self.class.ns
    }
    fn local_name(&self) -> &'static str {
        self.class.local
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn to_json_value(&self) -> serde_json::Value {
        // `id` matches the identity field the generated structs serialise, so
        // both representations look the same to a JSON or Python consumer.
        let mut map = serde_json::Map::with_capacity(self.fields.len() + 1);
        map.insert("id".to_string(), serde_json::Value::String(self.mrid.clone()));
        for (k, v) in &self.fields {
            map.insert(k.clone(), field_to_json(v));
        }
        serde_json::Value::Object(map)
    }
    fn to_block(&self) -> RdfBlock {
        RdfBlock {
            type_name: self.class.qualified.to_string(),
            mrid: self.mrid.clone(),
            fields: self.fields.clone(),
            duplicate_fields: FastSet::default(),
        }
    }
}

/// A reference to another CIM object by MRID.
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct MridRef {
    pub mrid: String,
}

/// A reference to a CIM enum value by URI.
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct UriRef {
    pub uri: String,
}
