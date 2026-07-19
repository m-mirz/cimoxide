/// State variable for transformer tap step.
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct SvTapStep {
    pub id: String,
    /// The tap changer associated with the tap step state.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tap_changer: Option<super::base::MridRef>,
    /// The floating point tap position. This is not the tap ratio, but rather the tap step position as defined by the related tap changer model and normally is constrained to be within the range of minimum and maximum tap positions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<f64>,
}
impl crate::base::CimElement for SvTapStep {
    fn mrid(&self) -> &str { &self.id }
    fn type_name(&self) -> &'static str { "SvTapStep" }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn to_json_value(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or(serde_json::Value::Null)
    }
    fn to_block(&self) -> crate::base::RdfBlock {
        let mut block = crate::base::RdfBlock {
            type_name: "SvTapStep".to_string(),
            mrid: self.id.clone(),
            fields: std::collections::HashMap::new(),
            duplicate_fields: std::collections::HashSet::new(),
        };
        if let Some(ref v) = self.tap_changer {
            block.fields.insert("SvTapStep.TapChanger".into(), crate::base::FieldValue::Resource(v.mrid.clone()));
        }
        if let Some(v) = self.position {
            block.fields.insert("SvTapStep.position".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        block
    }
}

impl SvTapStep {
    pub fn from_block(b: &crate::base::RdfBlock) -> Self {
        let mut obj = Self::default();
        obj.id.clone_from(&b.mrid);
        for (key, val) in &b.fields {
            match key.as_str() {
                "SvTapStep.TapChanger" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.tap_changer = Some(crate::base::MridRef { mrid: sv.clone() });
                    }
                }
                "SvTapStep.position" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.position = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.position = Some(v); } }
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
