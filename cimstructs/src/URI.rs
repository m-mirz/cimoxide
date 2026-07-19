/// URI is a string following the rules defined by the W3C/IETF URI Planning Interest Group in a set of RFCs of which one is RFC 3305.
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct URI {
    pub id: String,
}
impl crate::base::CimElement for URI {
    fn mrid(&self) -> &str { &self.id }
    fn type_name(&self) -> &'static str { "URI" }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn to_json_value(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or(serde_json::Value::Null)
    }
    fn to_block(&self) -> crate::base::RdfBlock {
        let block = crate::base::RdfBlock {
            type_name: "URI".to_string(),
            mrid: self.id.clone(),
            fields: std::collections::HashMap::new(),
            duplicate_fields: std::collections::HashSet::new(),
        };
        block
    }
}

impl URI {
    pub fn from_block(b: &crate::base::RdfBlock) -> Self {
        let mut obj = Self::default();
        obj.id.clone_from(&b.mrid);
        obj
    }
}
