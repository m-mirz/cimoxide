/// An analog control that issues a set point value.
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct SetPoint {
    #[serde(flatten)]
    pub base: super::AnalogControl,
    /// Normal value for Control.value e.g. used for percentage scaling.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub normal_value: Option<f64>,
    /// The value representing the actuator output.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<f64>,
}
impl crate::base::CimElement for SetPoint {
    fn mrid(&self) -> &str { &self.base.base.base.base.id }
    fn type_name(&self) -> &'static str { "SetPoint" }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn to_json_value(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or(serde_json::Value::Null)
    }
    fn to_block(&self) -> crate::base::RdfBlock {
        let mut block = self.base.to_block();
        block.type_name = "SetPoint".to_string();
        if let Some(v) = self.normal_value {
            block.fields.insert("SetPoint.normalValue".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.value {
            block.fields.insert("SetPoint.value".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        block
    }
}

impl SetPoint {
    pub fn from_block(b: &crate::base::RdfBlock) -> Self {
        let mut obj = Self::default();
        obj.base.base.base.base.id.clone_from(&b.mrid);
        for (key, val) in &b.fields {
            match key.as_str() {
                "SetPoint.normalValue" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.normal_value = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.normal_value = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "SetPoint.value" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.value = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.value = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "AnalogControl.AnalogValue" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.base.analog_value = Some(crate::base::MridRef { mrid: sv.clone() });
                    }
                }
                "AnalogControl.maxValue" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.base.max_value = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.base.max_value = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "AnalogControl.minValue" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.base.min_value = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.base.min_value = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "Control.PowerSystemResource" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.base.base.power_system_resource = Some(crate::base::MridRef { mrid: sv.clone() });
                    }
                }
                "Control.controlType" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.base.base.control_type = sv.clone(); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.base.base.control_type = sv.clone(); }
                        }
                        _ => {}
                    }
                }
                "Control.operationInProgress" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.base.base.operation_in_progress = Some(sv.trim() == "true"); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.base.base.operation_in_progress = Some(sv.trim() == "true"); }
                        }
                        _ => {}
                    }
                }
                "Control.timeStamp" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.base.base.time_stamp = sv.clone(); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.base.base.time_stamp = sv.clone(); }
                        }
                        _ => {}
                    }
                }
                "Control.unitMultiplier" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.base.base.unit_multiplier = Some(crate::base::UriRef { uri: sv.clone() });
                    }
                }
                "Control.unitSymbol" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.base.base.unit_symbol = Some(crate::base::UriRef { uri: sv.clone() });
                    }
                }
                "IdentifiedObject.description" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.base.base.base.base.description = sv.clone(); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.base.base.base.base.description = sv.clone(); }
                        }
                        _ => {}
                    }
                }
                "IdentifiedObject.energyIdentCodeEic" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.base.base.base.base.energy_ident_code_eic = sv.clone(); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.base.base.base.base.energy_ident_code_eic = sv.clone(); }
                        }
                        _ => {}
                    }
                }
                "IdentifiedObject.mRID" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.base.base.base.base.m_rid = sv.clone(); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.base.base.base.base.m_rid = sv.clone(); }
                        }
                        _ => {}
                    }
                }
                "IdentifiedObject.name" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.base.base.base.base.name = sv.clone(); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.base.base.base.base.name = sv.clone(); }
                        }
                        _ => {}
                    }
                }
                "IdentifiedObject.shortName" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.base.base.base.base.short_name = sv.clone(); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.base.base.base.base.short_name = sv.clone(); }
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
