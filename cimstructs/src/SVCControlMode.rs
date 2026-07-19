/// Static VAr Compensator control mode.
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct SVCControlMode {
    pub id: String,
}
impl crate::base::CimElement for SVCControlMode {
    fn mrid(&self) -> &str { &self.id }
    fn type_name(&self) -> &'static str { "SVCControlMode" }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn to_json_value(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or(serde_json::Value::Null)
    }
    fn to_block(&self) -> crate::base::RdfBlock {
        let block = crate::base::RdfBlock {
            type_name: "SVCControlMode".to_string(),
            mrid: self.id.clone(),
            fields: std::collections::HashMap::new(),
            duplicate_fields: std::collections::HashSet::new(),
        };
        block
    }
}

impl SVCControlMode {
    pub fn from_block(b: &crate::base::RdfBlock) -> Self {
        let mut obj = Self::default();
        obj.id.clone_from(&b.mrid);
        obj
    }
}
