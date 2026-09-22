use std::collections::HashMap;

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
    pub fields: HashMap<String, FieldValue>,
    /// Field names (local XML element name) that were assigned more than once
    /// within a single parsed element, indicating a MaxCount violation.
    pub duplicate_fields: std::collections::HashSet<String>,
}

impl Default for RdfBlock {
    fn default() -> Self {
        Self {
            type_name: String::new(),
            mrid: String::new(),
            fields: HashMap::new(),
            duplicate_fields: std::collections::HashSet::new(),
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

#[derive(Clone, Copy)]
pub struct TypeEntry {
    pub type_name: &'static str,
    pub parse: ParseFn,
}

/// Namespace-aware type dispatch.
///
/// RDF/XML identifies a class by `(namespace, local name)`, and the two profile
/// families overlap on 164 local names — both even spell the prefix `cim`, so
/// only the xmlns binding distinguishes them. Lookup is therefore keyed on the
/// resolved namespace, with a bare-name fallback that preserves the historical
/// behaviour for files whose prefixes are unbound or unexpected.
pub struct TypeRegistry {
    by_ns: HashMap<&'static str, HashMap<&'static str, TypeEntry>>,
    bare: HashMap<&'static str, TypeEntry>,
    by_type_name: HashMap<&'static str, TypeEntry>,
}

impl TypeRegistry {
    pub fn from_rows(
        rows: &'static [(&'static str, &'static str, &'static str, ParseFn)],
        bare_rows: &'static [(&'static str, &'static str, ParseFn)],
    ) -> Self {
        let mut by_ns: HashMap<&'static str, HashMap<&'static str, TypeEntry>> = HashMap::new();
        let mut by_type_name = HashMap::new();
        for (ns, local, type_name, parse) in rows {
            let entry = TypeEntry { type_name, parse: *parse };
            by_ns.entry(ns).or_default().insert(local, entry);
            by_type_name.insert(*type_name, entry);
        }
        let bare = bare_rows
            .iter()
            .map(|(local, type_name, parse)| (*local, TypeEntry { type_name, parse: *parse }))
            .collect();
        Self { by_ns, bare, by_type_name }
    }

    /// Dispatch table for one namespace, resolved once per XML prefix per file.
    pub fn ns_table(&self, ns: &str) -> Option<&HashMap<&'static str, TypeEntry>> {
        self.by_ns.get(ns)
    }

    pub fn bare(&self) -> &HashMap<&'static str, TypeEntry> {
        &self.bare
    }

    /// Re-dispatch an already-decoded element, used when merging datasets.
    pub fn by_type_name(&self, name: &str) -> Option<TypeEntry> {
        self.by_type_name.get(name).copied()
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
