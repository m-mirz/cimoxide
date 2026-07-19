/// State variable for power flow. Load convention is used for flow direction. This means flow out from the TopologicalNode into the equipment is positive.
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct SvPowerFlow {
    pub id: String,
    /// The terminal associated with the power flow state variable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub terminal: Option<super::base::MridRef>,
    /// The active power flow. Load sign convention is used, i.e. positive sign means flow out from a TopologicalNode (bus) into the conducting equipment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub p: Option<f64>,
    /// The reactive power flow. Load sign convention is used, i.e. positive sign means flow out from a TopologicalNode (bus) into the conducting equipment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub q: Option<f64>,
}
impl crate::base::CimElement for SvPowerFlow {
    fn mrid(&self) -> &str { &self.id }
    fn type_name(&self) -> &'static str { "SvPowerFlow" }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn to_json_value(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or(serde_json::Value::Null)
    }
    fn to_block(&self) -> crate::base::RdfBlock {
        let mut block = crate::base::RdfBlock {
            type_name: "SvPowerFlow".to_string(),
            mrid: self.id.clone(),
            fields: std::collections::HashMap::new(),
            duplicate_fields: std::collections::HashSet::new(),
        };
        if let Some(ref v) = self.terminal {
            block.fields.insert("SvPowerFlow.Terminal".into(), crate::base::FieldValue::Resource(v.mrid.clone()));
        }
        if let Some(v) = self.p {
            block.fields.insert("SvPowerFlow.p".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.q {
            block.fields.insert("SvPowerFlow.q".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        block
    }
}

impl SvPowerFlow {
    pub fn from_block(b: &crate::base::RdfBlock) -> Self {
        let mut obj = Self::default();
        obj.id.clone_from(&b.mrid);
        for (key, val) in &b.fields {
            match key.as_str() {
                "SvPowerFlow.Terminal" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.terminal = Some(crate::base::MridRef { mrid: sv.clone() });
                    }
                }
                "SvPowerFlow.p" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.p = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.p = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "SvPowerFlow.q" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.q = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.q = Some(v); } }
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
