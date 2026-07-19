/// Control is used for supervisory/device control. It represents control outputs that are used to change the state in a process, e.g. close or open breaker, a set point value or a raise lower command.
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct Control {
    #[serde(flatten)]
    pub base: super::IOPoint,
    /// Regulating device governed by this control output.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub power_system_resource: Option<super::base::MridRef>,
    /// Specifies the type of Control. For example, this specifies if the Control represents BreakerOpen, BreakerClose, GeneratorVoltageSetPoint, GeneratorRaise, GeneratorLower, etc.
    pub control_type: String,
    /// Indicates that a client is currently sending control commands that has not completed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation_in_progress: Option<bool>,
    /// The last time a control output was sent.
    pub time_stamp: String,
    /// The unit multiplier of the controlled quantity.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_multiplier: Option<super::base::UriRef>,
    /// The unit of measure of the controlled quantity.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_symbol: Option<super::base::UriRef>,
}
impl crate::base::CimElement for Control {
    fn mrid(&self) -> &str { &self.base.base.id }
    fn type_name(&self) -> &'static str { "Control" }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn to_json_value(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or(serde_json::Value::Null)
    }
    fn to_block(&self) -> crate::base::RdfBlock {
        let mut block = self.base.to_block();
        block.type_name = "Control".to_string();
        if let Some(ref v) = self.power_system_resource {
            block.fields.insert("Control.PowerSystemResource".into(), crate::base::FieldValue::Resource(v.mrid.clone()));
        }
        if !self.control_type.is_empty() {
            block.fields.insert("Control.controlType".into(), crate::base::FieldValue::Text(self.control_type.clone()));
        }
        if let Some(v) = self.operation_in_progress {
            block.fields.insert("Control.operationInProgress".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if !self.time_stamp.is_empty() {
            block.fields.insert("Control.timeStamp".into(), crate::base::FieldValue::Text(self.time_stamp.clone()));
        }
        if let Some(ref v) = self.unit_multiplier {
            block.fields.insert("Control.unitMultiplier".into(), crate::base::FieldValue::Resource(v.uri.clone()));
        }
        if let Some(ref v) = self.unit_symbol {
            block.fields.insert("Control.unitSymbol".into(), crate::base::FieldValue::Resource(v.uri.clone()));
        }
        block
    }
}

impl Control {
    pub fn from_block(b: &crate::base::RdfBlock) -> Self {
        let mut obj = Self::default();
        obj.base.base.id.clone_from(&b.mrid);
        for (key, val) in &b.fields {
            match key.as_str() {
                "Control.PowerSystemResource" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.power_system_resource = Some(crate::base::MridRef { mrid: sv.clone() });
                    }
                }
                "Control.controlType" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.control_type = sv.clone(); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.control_type = sv.clone(); }
                        }
                        _ => {}
                    }
                }
                "Control.operationInProgress" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.operation_in_progress = Some(sv.trim() == "true"); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.operation_in_progress = Some(sv.trim() == "true"); }
                        }
                        _ => {}
                    }
                }
                "Control.timeStamp" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.time_stamp = sv.clone(); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.time_stamp = sv.clone(); }
                        }
                        _ => {}
                    }
                }
                "Control.unitMultiplier" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.unit_multiplier = Some(crate::base::UriRef { uri: sv.clone() });
                    }
                }
                "Control.unitSymbol" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.unit_symbol = Some(crate::base::UriRef { uri: sv.clone() });
                    }
                }
                "IdentifiedObject.description" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.base.base.description = sv.clone(); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.base.base.description = sv.clone(); }
                        }
                        _ => {}
                    }
                }
                "IdentifiedObject.energyIdentCodeEic" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.base.base.energy_ident_code_eic = sv.clone(); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.base.base.energy_ident_code_eic = sv.clone(); }
                        }
                        _ => {}
                    }
                }
                "IdentifiedObject.mRID" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.base.base.m_rid = sv.clone(); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.base.base.m_rid = sv.clone(); }
                        }
                        _ => {}
                    }
                }
                "IdentifiedObject.name" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.base.base.name = sv.clone(); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.base.base.name = sv.clone(); }
                        }
                        _ => {}
                    }
                }
                "IdentifiedObject.shortName" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.base.base.short_name = sv.clone(); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.base.base.short_name = sv.clone(); }
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
