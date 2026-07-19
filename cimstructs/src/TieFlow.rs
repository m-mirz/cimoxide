/// Defines the structure (in terms of location and direction) of the net interchange constraint for a control area. This constraint may be used by either AGC or power flow.
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct TieFlow {
    #[serde(flatten)]
    pub base: super::IdentifiedObject,
    /// The control area of the tie flows.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub control_area: Option<super::base::MridRef>,
    /// The terminal to which this tie flow belongs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub terminal: Option<super::base::MridRef>,
    /// Specifies the sign of the tie flow associated with a control area. True if positive flow into the terminal (load convention) is also positive flow into the control area. See the description of ControlArea for further explanation of how TieFlow.positiveFlowIn is used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub positive_flow_in: Option<bool>,
}
impl crate::base::CimElement for TieFlow {
    fn mrid(&self) -> &str { &self.base.id }
    fn type_name(&self) -> &'static str { "TieFlow" }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn to_json_value(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or(serde_json::Value::Null)
    }
    fn to_block(&self) -> crate::base::RdfBlock {
        let mut block = self.base.to_block();
        block.type_name = "TieFlow".to_string();
        if let Some(ref v) = self.control_area {
            block.fields.insert("TieFlow.ControlArea".into(), crate::base::FieldValue::Resource(v.mrid.clone()));
        }
        if let Some(ref v) = self.terminal {
            block.fields.insert("TieFlow.Terminal".into(), crate::base::FieldValue::Resource(v.mrid.clone()));
        }
        if let Some(v) = self.positive_flow_in {
            block.fields.insert("TieFlow.positiveFlowIn".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        block
    }
}

impl TieFlow {
    pub fn from_block(b: &crate::base::RdfBlock) -> Self {
        let mut obj = Self::default();
        obj.base.id.clone_from(&b.mrid);
        for (key, val) in &b.fields {
            match key.as_str() {
                "TieFlow.ControlArea" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.control_area = Some(crate::base::MridRef { mrid: sv.clone() });
                    }
                }
                "TieFlow.Terminal" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.terminal = Some(crate::base::MridRef { mrid: sv.clone() });
                    }
                }
                "TieFlow.positiveFlowIn" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.positive_flow_in = Some(sv.trim() == "true"); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.positive_flow_in = Some(sv.trim() == "true"); }
                        }
                        _ => {}
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
