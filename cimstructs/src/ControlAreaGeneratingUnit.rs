/// A control area generating unit. This class is needed so that alternate control area definitions may include the same generating unit. It should be noted that only one instance within a control area should reference a specific generating unit.
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct ControlAreaGeneratingUnit {
    #[serde(flatten)]
    pub base: super::IdentifiedObject,
    /// The parent control area for the generating unit specifications.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub control_area: Option<super::base::MridRef>,
    /// The generating unit specified for this control area. Note that a control area should include a GeneratingUnit only once.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generating_unit: Option<super::base::MridRef>,
}
impl crate::base::CimElement for ControlAreaGeneratingUnit {
    fn mrid(&self) -> &str { &self.base.id }
    fn type_name(&self) -> &'static str { "ControlAreaGeneratingUnit" }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn to_json_value(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or(serde_json::Value::Null)
    }
    fn to_block(&self) -> crate::base::RdfBlock {
        let mut block = self.base.to_block();
        block.type_name = "ControlAreaGeneratingUnit".to_string();
        if let Some(ref v) = self.control_area {
            block.fields.insert("ControlAreaGeneratingUnit.ControlArea".into(), crate::base::FieldValue::Resource(v.mrid.clone()));
        }
        if let Some(ref v) = self.generating_unit {
            block.fields.insert("ControlAreaGeneratingUnit.GeneratingUnit".into(), crate::base::FieldValue::Resource(v.mrid.clone()));
        }
        block
    }
}

impl ControlAreaGeneratingUnit {
    pub fn from_block(b: &crate::base::RdfBlock) -> Self {
        let mut obj = Self::default();
        obj.base.id.clone_from(&b.mrid);
        for (key, val) in &b.fields {
            match key.as_str() {
                "ControlAreaGeneratingUnit.ControlArea" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.control_area = Some(crate::base::MridRef { mrid: sv.clone() });
                    }
                }
                "ControlAreaGeneratingUnit.GeneratingUnit" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.generating_unit = Some(crate::base::MridRef { mrid: sv.clone() });
                    }
                }
                "IdentifiedObject.description" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.base.description = sv.clone(); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.base.description = sv.clone(); }
                        }
                        _ => {}
                    }
                }
                "IdentifiedObject.energyIdentCodeEic" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.base.energy_ident_code_eic = sv.clone(); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.base.energy_ident_code_eic = sv.clone(); }
                        }
                        _ => {}
                    }
                }
                "IdentifiedObject.mRID" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.base.m_rid = sv.clone(); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.base.m_rid = sv.clone(); }
                        }
                        _ => {}
                    }
                }
                "IdentifiedObject.name" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.base.name = sv.clone(); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.base.name = sv.clone(); }
                        }
                        _ => {}
                    }
                }
                "IdentifiedObject.shortName" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.base.short_name = sv.clone(); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.base.short_name = sv.clone(); }
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
