/// State variable for voltage.
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct SvVoltage {
    pub id: String,
    /// The topological node associated with the voltage state.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topological_node: Option<super::base::MridRef>,
    /// The voltage angle of the topological node complex voltage with respect to system reference.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub angle: Option<f64>,
    /// The voltage magnitude at the topological node. The attribute shall be a positive value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub v: Option<f64>,
}
impl crate::base::CimElement for SvVoltage {
    fn mrid(&self) -> &str { &self.id }
    fn type_name(&self) -> &'static str { "SvVoltage" }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn to_json_value(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or(serde_json::Value::Null)
    }
    fn to_block(&self) -> crate::base::RdfBlock {
        let mut block = crate::base::RdfBlock {
            type_name: "SvVoltage".to_string(),
            mrid: self.id.clone(),
            fields: std::collections::HashMap::new(),
            duplicate_fields: std::collections::HashSet::new(),
        };
        if let Some(ref v) = self.topological_node {
            block.fields.insert("SvVoltage.TopologicalNode".into(), crate::base::FieldValue::Resource(v.mrid.clone()));
        }
        if let Some(v) = self.angle {
            block.fields.insert("SvVoltage.angle".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.v {
            block.fields.insert("SvVoltage.v".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        block
    }
}

impl SvVoltage {
    pub fn from_block(b: &crate::base::RdfBlock) -> Self {
        let mut obj = Self::default();
        obj.id.clone_from(&b.mrid);
        for (key, val) in &b.fields {
            match key.as_str() {
                "SvVoltage.TopologicalNode" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.topological_node = Some(crate::base::MridRef { mrid: sv.clone() });
                    }
                }
                "SvVoltage.angle" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.angle = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.angle = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "SvVoltage.v" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.v = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.v = Some(v); } }
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        }
        obj
    }
}
