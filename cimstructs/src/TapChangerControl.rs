/// Describes behaviour specific to tap changers, e.g. how the voltage at the end of a line varies with the load level and compensation of the voltage drop by tap adjustment.
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct TapChangerControl {
    #[serde(flatten)]
    pub base: super::RegulatingControl,
}
impl crate::base::CimElement for TapChangerControl {
    fn mrid(&self) -> &str { &self.base.base.base.id }
    fn type_name(&self) -> &'static str { "TapChangerControl" }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn to_json_value(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or(serde_json::Value::Null)
    }
    fn to_block(&self) -> crate::base::RdfBlock {
        let mut block = self.base.to_block();
        block.type_name = "TapChangerControl".to_string();
        block
    }
}

impl TapChangerControl {
    pub fn from_block(b: &crate::base::RdfBlock) -> Self {
        let mut obj = Self::default();
        obj.base.base.base.id.clone_from(&b.mrid);
        for (key, val) in &b.fields {
            match key.as_str() {
                "RegulatingControl.Terminal" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.base.terminal = Some(crate::base::MridRef { mrid: sv.clone() });
                    }
                }
                "RegulatingControl.discrete" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.base.discrete = Some(sv.trim() == "true"); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.base.discrete = Some(sv.trim() == "true"); }
                        }
                        _ => {}
                    }
                }
                "RegulatingControl.enabled" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.base.enabled = Some(sv.trim() == "true"); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.base.enabled = Some(sv.trim() == "true"); }
                        }
                        _ => {}
                    }
                }
                "RegulatingControl.maxAllowedTargetValue" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.base.max_allowed_target_value = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.base.max_allowed_target_value = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "RegulatingControl.minAllowedTargetValue" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.base.min_allowed_target_value = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.base.min_allowed_target_value = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "RegulatingControl.mode" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.base.mode = Some(crate::base::UriRef { uri: sv.clone() });
                    }
                }
                "RegulatingControl.targetDeadband" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.base.target_deadband = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.base.target_deadband = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "RegulatingControl.targetValue" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.base.target_value = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.base.target_value = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "RegulatingControl.targetValueUnitMultiplier" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.base.target_value_unit_multiplier = Some(crate::base::UriRef { uri: sv.clone() });
                    }
                }
                "IdentifiedObject.description" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.base.base.base.description = sv.clone(); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.base.base.base.description = sv.clone(); }
                        }
                        _ => {}
                    }
                }
                "IdentifiedObject.energyIdentCodeEic" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.base.base.base.energy_ident_code_eic = sv.clone(); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.base.base.base.energy_ident_code_eic = sv.clone(); }
                        }
                        _ => {}
                    }
                }
                "IdentifiedObject.mRID" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.base.base.base.m_rid = sv.clone(); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.base.base.base.m_rid = sv.clone(); }
                        }
                        _ => {}
                    }
                }
                "IdentifiedObject.name" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.base.base.base.name = sv.clone(); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.base.base.base.name = sv.clone(); }
                        }
                        _ => {}
                    }
                }
                "IdentifiedObject.shortName" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.base.base.base.short_name = sv.clone(); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.base.base.base.short_name = sv.clone(); }
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
