/// Supports definition of one or more parameters of several different datatypes for use by proprietary user-defined models. This class does not inherit from IdentifiedObject since it is not intended that a single instance of it be referenced by more than one proprietary user-defined model instance.
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct ProprietaryParameterDynamics {
    pub id: String,
    /// Proprietary user-defined model with which this parameter is associated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asynchronous_machine_user_defined: Option<super::base::MridRef>,
    /// Proprietary user-defined model with which this parameter is associated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub csc_user_defined: Option<super::base::MridRef>,
    /// Proprietary user-defined model with which this parameter is associated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discontinuous_excitation_control_user_defined: Option<super::base::MridRef>,
    /// Proprietary user-defined model with which this parameter is associated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub excitation_system_user_defined: Option<super::base::MridRef>,
    /// Proprietary user-defined model with which this parameter is associated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub load_user_defined: Option<super::base::MridRef>,
    /// Proprietary user-defined model with which this parameter is associated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mechanical_load_user_defined: Option<super::base::MridRef>,
    /// Proprietary user-defined model with which this parameter is associated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overexcitation_limiter_user_defined: Option<super::base::MridRef>,
    /// Proprietary user-defined model with which this parameter is associated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pfv_ar_controller_type1user_defined: Option<super::base::MridRef>,
    /// Proprietary user-defined model with which this parameter is associated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pfv_ar_controller_type2user_defined: Option<super::base::MridRef>,
    /// Proprietary user-defined model with which this parameter is associated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub power_system_stabilizer_user_defined: Option<super::base::MridRef>,
    /// Proprietary user-defined model with which this parameter is associated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub svc_user_defined: Option<super::base::MridRef>,
    /// Proprietary user-defined model with which this parameter is associated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub synchronous_machine_user_defined: Option<super::base::MridRef>,
    /// Proprietary user-defined model with which this parameter is associated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub turbine_governor_user_defined: Option<super::base::MridRef>,
    /// Proprietary user-defined model with which this parameter is associated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub turbine_load_controller_user_defined: Option<super::base::MridRef>,
    /// Proprietary user-defined model with which this parameter is associated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub underexcitation_limiter_user_defined: Option<super::base::MridRef>,
    /// Proprietary user-defined model with which this parameter is associated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vsc_user_defined: Option<super::base::MridRef>,
    /// Proprietary user-defined model with which this parameter is associated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voltage_adjuster_user_defined: Option<super::base::MridRef>,
    /// Proprietary user-defined model with which this parameter is associated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voltage_compensator_user_defined: Option<super::base::MridRef>,
    /// Proprietary user-defined model with which this parameter is associated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wind_plant_user_defined: Option<super::base::MridRef>,
    /// Proprietary user-defined model with which this parameter is associated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wind_type1or2user_defined: Option<super::base::MridRef>,
    /// Proprietary user-defined model with which this parameter is associated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wind_type3or4user_defined: Option<super::base::MridRef>,
    /// Boolean parameter value. If this attribute is populated, integerParameterValue and floatParameterValue will not be.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub boolean_parameter_value: Option<bool>,
    /// Floating point parameter value. If this attribute is populated, booleanParameterValue and integerParameterValue will not be.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub float_parameter_value: Option<f64>,
    /// Integer parameter value. If this attribute is populated, booleanParameterValue and floatParameterValue will not be.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integer_parameter_value: Option<i64>,
    /// Sequence number of the parameter among the set of parameters associated with the related proprietary user-defined model.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_number: Option<i64>,
}
impl crate::base::CimElement for ProprietaryParameterDynamics {
    fn mrid(&self) -> &str { &self.id }
    fn type_name(&self) -> &'static str { "ProprietaryParameterDynamics" }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn to_json_value(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or(serde_json::Value::Null)
    }
    fn to_block(&self) -> crate::base::RdfBlock {
        let mut block = crate::base::RdfBlock {
            type_name: "ProprietaryParameterDynamics".to_string(),
            mrid: self.id.clone(),
            fields: std::collections::HashMap::new(),
            duplicate_fields: std::collections::HashSet::new(),
        };
        if let Some(ref v) = self.asynchronous_machine_user_defined {
            block.fields.insert("ProprietaryParameterDynamics.AsynchronousMachineUserDefined".into(), crate::base::FieldValue::Resource(v.mrid.clone()));
        }
        if let Some(ref v) = self.csc_user_defined {
            block.fields.insert("ProprietaryParameterDynamics.CSCUserDefined".into(), crate::base::FieldValue::Resource(v.mrid.clone()));
        }
        if let Some(ref v) = self.discontinuous_excitation_control_user_defined {
            block.fields.insert("ProprietaryParameterDynamics.DiscontinuousExcitationControlUserDefined".into(), crate::base::FieldValue::Resource(v.mrid.clone()));
        }
        if let Some(ref v) = self.excitation_system_user_defined {
            block.fields.insert("ProprietaryParameterDynamics.ExcitationSystemUserDefined".into(), crate::base::FieldValue::Resource(v.mrid.clone()));
        }
        if let Some(ref v) = self.load_user_defined {
            block.fields.insert("ProprietaryParameterDynamics.LoadUserDefined".into(), crate::base::FieldValue::Resource(v.mrid.clone()));
        }
        if let Some(ref v) = self.mechanical_load_user_defined {
            block.fields.insert("ProprietaryParameterDynamics.MechanicalLoadUserDefined".into(), crate::base::FieldValue::Resource(v.mrid.clone()));
        }
        if let Some(ref v) = self.overexcitation_limiter_user_defined {
            block.fields.insert("ProprietaryParameterDynamics.OverexcitationLimiterUserDefined".into(), crate::base::FieldValue::Resource(v.mrid.clone()));
        }
        if let Some(ref v) = self.pfv_ar_controller_type1user_defined {
            block.fields.insert("ProprietaryParameterDynamics.PFVArControllerType1UserDefined".into(), crate::base::FieldValue::Resource(v.mrid.clone()));
        }
        if let Some(ref v) = self.pfv_ar_controller_type2user_defined {
            block.fields.insert("ProprietaryParameterDynamics.PFVArControllerType2UserDefined".into(), crate::base::FieldValue::Resource(v.mrid.clone()));
        }
        if let Some(ref v) = self.power_system_stabilizer_user_defined {
            block.fields.insert("ProprietaryParameterDynamics.PowerSystemStabilizerUserDefined".into(), crate::base::FieldValue::Resource(v.mrid.clone()));
        }
        if let Some(ref v) = self.svc_user_defined {
            block.fields.insert("ProprietaryParameterDynamics.SVCUserDefined".into(), crate::base::FieldValue::Resource(v.mrid.clone()));
        }
        if let Some(ref v) = self.synchronous_machine_user_defined {
            block.fields.insert("ProprietaryParameterDynamics.SynchronousMachineUserDefined".into(), crate::base::FieldValue::Resource(v.mrid.clone()));
        }
        if let Some(ref v) = self.turbine_governor_user_defined {
            block.fields.insert("ProprietaryParameterDynamics.TurbineGovernorUserDefined".into(), crate::base::FieldValue::Resource(v.mrid.clone()));
        }
        if let Some(ref v) = self.turbine_load_controller_user_defined {
            block.fields.insert("ProprietaryParameterDynamics.TurbineLoadControllerUserDefined".into(), crate::base::FieldValue::Resource(v.mrid.clone()));
        }
        if let Some(ref v) = self.underexcitation_limiter_user_defined {
            block.fields.insert("ProprietaryParameterDynamics.UnderexcitationLimiterUserDefined".into(), crate::base::FieldValue::Resource(v.mrid.clone()));
        }
        if let Some(ref v) = self.vsc_user_defined {
            block.fields.insert("ProprietaryParameterDynamics.VSCUserDefined".into(), crate::base::FieldValue::Resource(v.mrid.clone()));
        }
        if let Some(ref v) = self.voltage_adjuster_user_defined {
            block.fields.insert("ProprietaryParameterDynamics.VoltageAdjusterUserDefined".into(), crate::base::FieldValue::Resource(v.mrid.clone()));
        }
        if let Some(ref v) = self.voltage_compensator_user_defined {
            block.fields.insert("ProprietaryParameterDynamics.VoltageCompensatorUserDefined".into(), crate::base::FieldValue::Resource(v.mrid.clone()));
        }
        if let Some(ref v) = self.wind_plant_user_defined {
            block.fields.insert("ProprietaryParameterDynamics.WindPlantUserDefined".into(), crate::base::FieldValue::Resource(v.mrid.clone()));
        }
        if let Some(ref v) = self.wind_type1or2user_defined {
            block.fields.insert("ProprietaryParameterDynamics.WindType1or2UserDefined".into(), crate::base::FieldValue::Resource(v.mrid.clone()));
        }
        if let Some(ref v) = self.wind_type3or4user_defined {
            block.fields.insert("ProprietaryParameterDynamics.WindType3or4UserDefined".into(), crate::base::FieldValue::Resource(v.mrid.clone()));
        }
        if let Some(v) = self.boolean_parameter_value {
            block.fields.insert("ProprietaryParameterDynamics.booleanParameterValue".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.float_parameter_value {
            block.fields.insert("ProprietaryParameterDynamics.floatParameterValue".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.integer_parameter_value {
            block.fields.insert("ProprietaryParameterDynamics.integerParameterValue".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.parameter_number {
            block.fields.insert("ProprietaryParameterDynamics.parameterNumber".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        block
    }
}

impl ProprietaryParameterDynamics {
    pub fn from_block(b: &crate::base::RdfBlock) -> Self {
        let mut obj = Self::default();
        obj.id.clone_from(&b.mrid);
        for (key, val) in &b.fields {
            match key.as_str() {
                "ProprietaryParameterDynamics.AsynchronousMachineUserDefined" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.asynchronous_machine_user_defined = Some(crate::base::MridRef { mrid: sv.clone() });
                    }
                }
                "ProprietaryParameterDynamics.CSCUserDefined" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.csc_user_defined = Some(crate::base::MridRef { mrid: sv.clone() });
                    }
                }
                "ProprietaryParameterDynamics.DiscontinuousExcitationControlUserDefined" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.discontinuous_excitation_control_user_defined = Some(crate::base::MridRef { mrid: sv.clone() });
                    }
                }
                "ProprietaryParameterDynamics.ExcitationSystemUserDefined" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.excitation_system_user_defined = Some(crate::base::MridRef { mrid: sv.clone() });
                    }
                }
                "ProprietaryParameterDynamics.LoadUserDefined" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.load_user_defined = Some(crate::base::MridRef { mrid: sv.clone() });
                    }
                }
                "ProprietaryParameterDynamics.MechanicalLoadUserDefined" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.mechanical_load_user_defined = Some(crate::base::MridRef { mrid: sv.clone() });
                    }
                }
                "ProprietaryParameterDynamics.OverexcitationLimiterUserDefined" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.overexcitation_limiter_user_defined = Some(crate::base::MridRef { mrid: sv.clone() });
                    }
                }
                "ProprietaryParameterDynamics.PFVArControllerType1UserDefined" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.pfv_ar_controller_type1user_defined = Some(crate::base::MridRef { mrid: sv.clone() });
                    }
                }
                "ProprietaryParameterDynamics.PFVArControllerType2UserDefined" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.pfv_ar_controller_type2user_defined = Some(crate::base::MridRef { mrid: sv.clone() });
                    }
                }
                "ProprietaryParameterDynamics.PowerSystemStabilizerUserDefined" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.power_system_stabilizer_user_defined = Some(crate::base::MridRef { mrid: sv.clone() });
                    }
                }
                "ProprietaryParameterDynamics.SVCUserDefined" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.svc_user_defined = Some(crate::base::MridRef { mrid: sv.clone() });
                    }
                }
                "ProprietaryParameterDynamics.SynchronousMachineUserDefined" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.synchronous_machine_user_defined = Some(crate::base::MridRef { mrid: sv.clone() });
                    }
                }
                "ProprietaryParameterDynamics.TurbineGovernorUserDefined" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.turbine_governor_user_defined = Some(crate::base::MridRef { mrid: sv.clone() });
                    }
                }
                "ProprietaryParameterDynamics.TurbineLoadControllerUserDefined" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.turbine_load_controller_user_defined = Some(crate::base::MridRef { mrid: sv.clone() });
                    }
                }
                "ProprietaryParameterDynamics.UnderexcitationLimiterUserDefined" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.underexcitation_limiter_user_defined = Some(crate::base::MridRef { mrid: sv.clone() });
                    }
                }
                "ProprietaryParameterDynamics.VSCUserDefined" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.vsc_user_defined = Some(crate::base::MridRef { mrid: sv.clone() });
                    }
                }
                "ProprietaryParameterDynamics.VoltageAdjusterUserDefined" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.voltage_adjuster_user_defined = Some(crate::base::MridRef { mrid: sv.clone() });
                    }
                }
                "ProprietaryParameterDynamics.VoltageCompensatorUserDefined" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.voltage_compensator_user_defined = Some(crate::base::MridRef { mrid: sv.clone() });
                    }
                }
                "ProprietaryParameterDynamics.WindPlantUserDefined" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.wind_plant_user_defined = Some(crate::base::MridRef { mrid: sv.clone() });
                    }
                }
                "ProprietaryParameterDynamics.WindType1or2UserDefined" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.wind_type1or2user_defined = Some(crate::base::MridRef { mrid: sv.clone() });
                    }
                }
                "ProprietaryParameterDynamics.WindType3or4UserDefined" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.wind_type3or4user_defined = Some(crate::base::MridRef { mrid: sv.clone() });
                    }
                }
                "ProprietaryParameterDynamics.booleanParameterValue" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.boolean_parameter_value = Some(sv.trim() == "true"); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.boolean_parameter_value = Some(sv.trim() == "true"); }
                        }
                        _ => {}
                    }
                }
                "ProprietaryParameterDynamics.floatParameterValue" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.float_parameter_value = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.float_parameter_value = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ProprietaryParameterDynamics.integerParameterValue" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.integer_parameter_value = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.integer_parameter_value = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ProprietaryParameterDynamics.parameterNumber" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.parameter_number = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.parameter_number = Some(v); } }
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
