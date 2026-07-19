/// This is used for grouping diagram object points from different diagram objects that are considered to be glued together in a diagram even if they are not at the exact same coordinates.
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct DiagramObjectGluePoint {
    pub id: String,
}
impl crate::base::CimElement for DiagramObjectGluePoint {
    fn mrid(&self) -> &str { &self.id }
    fn type_name(&self) -> &'static str { "DiagramObjectGluePoint" }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn to_json_value(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or(serde_json::Value::Null)
    }
    fn to_block(&self) -> crate::base::RdfBlock {
        let block = crate::base::RdfBlock {
            type_name: "DiagramObjectGluePoint".to_string(),
            mrid: self.id.clone(),
            fields: std::collections::HashMap::new(),
            duplicate_fields: std::collections::HashSet::new(),
        };
        block
    }
}

impl DiagramObjectGluePoint {
    pub fn from_block(b: &crate::base::RdfBlock) -> Self {
        let mut obj = Self::default();
        obj.id.clone_from(&b.mrid);
        obj
    }
}
