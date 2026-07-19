/// A Command is a discrete control used for supervisory control.
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct Command {
    #[serde(flatten)]
    pub base: super::Control,
    /// The MeasurementValue that is controlled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discrete_value: Option<super::base::MridRef>,
    /// The ValueAliasSet used for translation of a Control value to a name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value_alias_set: Option<super::base::MridRef>,
    /// Normal value for Control.value e.g. used for percentage scaling.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub normal_value: Option<i64>,
    /// The value representing the actuator output.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<i64>,
}
impl crate::base::CimElement for Command {
    fn mrid(&self) -> &str { &self.base.base.base.id }
    fn type_name(&self) -> &'static str { "Command" }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn to_json_value(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or(serde_json::Value::Null)
    }
    fn to_block(&self) -> crate::base::RdfBlock {
        let mut block = self.base.to_block();
        block.type_name = "Command".to_string();
        if let Some(ref v) = self.discrete_value {
            block.fields.insert("Command.DiscreteValue".into(), crate::base::FieldValue::Resource(v.mrid.clone()));
        }
        if let Some(ref v) = self.value_alias_set {
            block.fields.insert("Command.ValueAliasSet".into(), crate::base::FieldValue::Resource(v.mrid.clone()));
        }
        if let Some(v) = self.normal_value {
            block.fields.insert("Command.normalValue".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.value {
            block.fields.insert("Command.value".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        block
    }
}

impl Command {
    pub fn from_block(b: &crate::base::RdfBlock) -> Self {
        let mut obj = Self::default();
        obj.base.base.base.id.clone_from(&b.mrid);
        for (key, val) in &b.fields {
            match key.as_str() {
                "Command.DiscreteValue" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.discrete_value = Some(crate::base::MridRef { mrid: sv.clone() });
                    }
                }
                "Command.ValueAliasSet" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.value_alias_set = Some(crate::base::MridRef { mrid: sv.clone() });
                    }
                }
                "Command.normalValue" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.normal_value = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.normal_value = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "Command.value" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.value = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.value = Some(v); } }
                        }
                        _ => {}
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
