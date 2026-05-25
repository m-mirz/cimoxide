use std::collections::HashMap;

pub trait CimElement: Send + Sync {
    fn mrid(&self) -> &str;
    fn type_name(&self) -> &'static str;
    fn as_any(&self) -> &dyn std::any::Any;
    fn to_json_value(&self) -> serde_json::Value;
    fn to_block(&self) -> RdfBlock;
}

#[derive(Debug, Clone)]
pub enum FieldValue {
    Text(String),
    Resource(String),
    ResourceList(Vec<String>),
}

#[derive(Debug, Default, Clone)]
pub struct RdfBlock {
    pub type_name: String,
    pub mrid: String,
    pub fields: HashMap<String, FieldValue>,
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
