/// The SvInjection reports the calculated bus injection minus the sum of the terminal flows. The terminal flow is positive out from the bus (load sign convention) and bus injection has positive flow into the bus. SvInjection may have the remainder after state estimation or slack after power flow calculation.
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct SvInjection {
    pub id: String,
    /// The topological node associated with the flow injection state variable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topological_node: Option<super::base::MridRef>,
    /// The active power mismatch between calculated injection and initial injection. Positive sign means injection into the TopologicalNode (bus).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub p_injection: Option<f64>,
    /// The reactive power mismatch between calculated injection and initial injection. Positive sign means injection into the TopologicalNode (bus).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub q_injection: Option<f64>,
}
impl crate::base::CimElement for SvInjection {
    fn mrid(&self) -> &str { &self.id }
    fn type_name(&self) -> &'static str { "SvInjection" }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn to_json_value(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or(serde_json::Value::Null)
    }
    fn to_block(&self) -> crate::base::RdfBlock {
        let mut block = crate::base::RdfBlock {
            type_name: "SvInjection".to_string(),
            mrid: self.id.clone(),
            fields: std::collections::HashMap::new(),
            duplicate_fields: std::collections::HashSet::new(),
        };
        if let Some(ref v) = self.topological_node {
            block.fields.insert("SvInjection.TopologicalNode".into(), crate::base::FieldValue::Resource(v.mrid.clone()));
        }
        if let Some(v) = self.p_injection {
            block.fields.insert("SvInjection.pInjection".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.q_injection {
            block.fields.insert("SvInjection.qInjection".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        block
    }
}

impl SvInjection {
    pub fn from_block(b: &crate::base::RdfBlock) -> Self {
        let mut obj = Self::default();
        obj.id.clone_from(&b.mrid);
        for (key, val) in &b.fields {
            match key.as_str() {
                "SvInjection.TopologicalNode" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.topological_node = Some(crate::base::MridRef { mrid: sv.clone() });
                    }
                }
                "SvInjection.pInjection" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.p_injection = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.p_injection = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "SvInjection.qInjection" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.q_injection = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.q_injection = Some(v); } }
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
