/// This command resets the counter value to zero.
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct AccumulatorReset {
    #[serde(flatten)]
    pub base: super::Control,
    /// The accumulator value that is reset by the command.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accumulator_value: Option<super::base::MridRef>,
}
impl crate::base::CimElement for AccumulatorReset {
    fn mrid(&self) -> &str { &self.base.base.base.id }
    fn type_name(&self) -> &'static str { "AccumulatorReset" }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn to_json_value(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or(serde_json::Value::Null)
    }
    fn to_block(&self) -> crate::base::RdfBlock {
        let mut block = self.base.to_block();
        block.type_name = "AccumulatorReset".to_string();
        if let Some(ref v) = self.accumulator_value {
            block.fields.insert("AccumulatorReset.AccumulatorValue".into(), crate::base::FieldValue::Resource(v.mrid.clone()));
        }
        block
    }
}

impl AccumulatorReset {
    pub fn from_block(b: &crate::base::RdfBlock) -> Self {
        let mut obj = Self::default();
        obj.base.base.base.id.clone_from(&b.mrid);
        for (key, val) in &b.fields {
            match key.as_str() {
                "AccumulatorReset.AccumulatorValue" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.accumulator_value = Some(crate::base::MridRef { mrid: sv.clone() });
                    }
                }
                "Control.PowerSystemResource" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.base.power_system_resource = Some(crate::base::MridRef { mrid: sv.clone() });
                    }
                }
                "Control.controlType" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.base.control_type = sv.clone(); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.base.control_type = sv.clone(); }
                        }
                        _ => {}
                    }
                }
                "Control.operationInProgress" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.base.operation_in_progress = Some(sv.trim() == "true"); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.base.operation_in_progress = Some(sv.trim() == "true"); }
                        }
                        _ => {}
                    }
                }
                "Control.timeStamp" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.base.time_stamp = sv.clone(); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.base.time_stamp = sv.clone(); }
                        }
                        _ => {}
                    }
                }
                "Control.unitMultiplier" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.base.unit_multiplier = Some(crate::base::UriRef { uri: sv.clone() });
                    }
                }
                "Control.unitSymbol" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.base.unit_symbol = Some(crate::base::UriRef { uri: sv.clone() });
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
