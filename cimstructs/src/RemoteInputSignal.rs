/// Supports connection to a terminal associated with a remote bus from which an input signal of a specific type is coming.
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct RemoteInputSignal {
    #[serde(flatten)]
    pub base: super::IdentifiedObject,
    /// Discontinuous excitation control model using this remote input signal.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discontinuous_excitation_control_dynamics: Option<super::base::MridRef>,
    /// Power factor or VAr controller type 1 model using this remote input signal.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pfv_ar_controller_type1dynamics: Option<super::base::MridRef>,
    /// Power system stabilizer model using this remote input signal.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub power_system_stabilizer_dynamics: Option<super::base::MridRef>,
    /// Remote terminal with which this input signal is associated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub terminal: Option<super::base::MridRef>,
    /// Underexcitation limiter model using this remote input signal.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub underexcitation_limiter_dynamics: Option<super::base::MridRef>,
    /// Voltage compensator model using this remote input signal.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voltage_compensator_dynamics: Option<super::base::MridRef>,
    /// Type of input signal.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remote_signal_type: Option<super::base::UriRef>,
}
impl crate::base::CimElement for RemoteInputSignal {
    fn mrid(&self) -> &str { &self.base.id }
    fn type_name(&self) -> &'static str { "RemoteInputSignal" }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn to_json_value(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or(serde_json::Value::Null)
    }
    fn to_block(&self) -> crate::base::RdfBlock {
        let mut block = self.base.to_block();
        block.type_name = "RemoteInputSignal".to_string();
        if let Some(ref v) = self.discontinuous_excitation_control_dynamics {
            block.fields.insert("RemoteInputSignal.DiscontinuousExcitationControlDynamics".into(), crate::base::FieldValue::Resource(v.mrid.clone()));
        }
        if let Some(ref v) = self.pfv_ar_controller_type1dynamics {
            block.fields.insert("RemoteInputSignal.PFVArControllerType1Dynamics".into(), crate::base::FieldValue::Resource(v.mrid.clone()));
        }
        if let Some(ref v) = self.power_system_stabilizer_dynamics {
            block.fields.insert("RemoteInputSignal.PowerSystemStabilizerDynamics".into(), crate::base::FieldValue::Resource(v.mrid.clone()));
        }
        if let Some(ref v) = self.terminal {
            block.fields.insert("RemoteInputSignal.Terminal".into(), crate::base::FieldValue::Resource(v.mrid.clone()));
        }
        if let Some(ref v) = self.underexcitation_limiter_dynamics {
            block.fields.insert("RemoteInputSignal.UnderexcitationLimiterDynamics".into(), crate::base::FieldValue::Resource(v.mrid.clone()));
        }
        if let Some(ref v) = self.voltage_compensator_dynamics {
            block.fields.insert("RemoteInputSignal.VoltageCompensatorDynamics".into(), crate::base::FieldValue::Resource(v.mrid.clone()));
        }
        if let Some(ref v) = self.remote_signal_type {
            block.fields.insert("RemoteInputSignal.remoteSignalType".into(), crate::base::FieldValue::Resource(v.uri.clone()));
        }
        block
    }
}

impl RemoteInputSignal {
    pub fn from_block(b: &crate::base::RdfBlock) -> Self {
        let mut obj = Self::default();
        obj.base.id.clone_from(&b.mrid);
        for (key, val) in &b.fields {
            match key.as_str() {
                "RemoteInputSignal.DiscontinuousExcitationControlDynamics" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.discontinuous_excitation_control_dynamics = Some(crate::base::MridRef { mrid: sv.clone() });
                    }
                }
                "RemoteInputSignal.PFVArControllerType1Dynamics" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.pfv_ar_controller_type1dynamics = Some(crate::base::MridRef { mrid: sv.clone() });
                    }
                }
                "RemoteInputSignal.PowerSystemStabilizerDynamics" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.power_system_stabilizer_dynamics = Some(crate::base::MridRef { mrid: sv.clone() });
                    }
                }
                "RemoteInputSignal.Terminal" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.terminal = Some(crate::base::MridRef { mrid: sv.clone() });
                    }
                }
                "RemoteInputSignal.UnderexcitationLimiterDynamics" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.underexcitation_limiter_dynamics = Some(crate::base::MridRef { mrid: sv.clone() });
                    }
                }
                "RemoteInputSignal.VoltageCompensatorDynamics" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.voltage_compensator_dynamics = Some(crate::base::MridRef { mrid: sv.clone() });
                    }
                }
                "RemoteInputSignal.remoteSignalType" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.remote_signal_type = Some(crate::base::UriRef { uri: sv.clone() });
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
