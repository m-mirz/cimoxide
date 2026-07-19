/// State variable for switch.
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct SvSwitch {
    pub id: String,
    /// The switch associated with the switch state.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub switch: Option<super::base::MridRef>,
    /// The attribute tells if the computed state of the switch is considered open.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub open: Option<bool>,
}
impl crate::base::CimElement for SvSwitch {
    fn mrid(&self) -> &str { &self.id }
    fn type_name(&self) -> &'static str { "SvSwitch" }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn to_json_value(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or(serde_json::Value::Null)
    }
    fn to_block(&self) -> crate::base::RdfBlock {
        let mut block = crate::base::RdfBlock {
            type_name: "SvSwitch".to_string(),
            mrid: self.id.clone(),
            fields: std::collections::HashMap::new(),
            duplicate_fields: std::collections::HashSet::new(),
        };
        if let Some(ref v) = self.switch {
            block.fields.insert("SvSwitch.Switch".into(), crate::base::FieldValue::Resource(v.mrid.clone()));
        }
        if let Some(v) = self.open {
            block.fields.insert("SvSwitch.open".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        block
    }
}

impl SvSwitch {
    pub fn from_block(b: &crate::base::RdfBlock) -> Self {
        let mut obj = Self::default();
        obj.id.clone_from(&b.mrid);
        for (key, val) in &b.fields {
            match key.as_str() {
                "SvSwitch.Switch" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.switch = Some(crate::base::MridRef { mrid: sv.clone() });
                    }
                }
                "SvSwitch.open" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.open = Some(sv.trim() == "true"); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.open = Some(sv.trim() == "true"); }
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
