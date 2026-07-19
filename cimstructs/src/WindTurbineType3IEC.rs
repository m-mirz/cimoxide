/// Parent class supporting relationships to IEC wind turbines type 3 including their control models.
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct WindTurbineType3IEC {
    #[serde(flatten)]
    pub base: super::WindTurbineType3or4IEC,
    /// Wind aerodynamic model associated with this wind generator type 3 model.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wind_aero_one_dim_iec: Option<super::base::MridRef>,
    /// Wind aerodynamic model associated with this wind turbine type 3 model.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wind_aero_two_dim_iec: Option<super::base::MridRef>,
    /// Wind control P type 3 model associated with this wind turbine type 3 model.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wind_cont_p_type3iec: Option<super::base::MridRef>,
    /// Wind control pitch angle model associated with this wind turbine type 3.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wind_cont_pitch_angle_iec: Option<super::base::MridRef>,
    /// Wind generator type 3 model associated with this wind turbine type 3 model.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wind_gen_type3iec: Option<super::base::MridRef>,
    /// Wind mechanical model associated with this wind turbine type 3 model.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wind_mech_iec: Option<super::base::MridRef>,
}
impl crate::base::CimElement for WindTurbineType3IEC {
    fn mrid(&self) -> &str { &self.base.base.base.base.id }
    fn type_name(&self) -> &'static str { "WindTurbineType3IEC" }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn to_json_value(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or(serde_json::Value::Null)
    }
    fn to_block(&self) -> crate::base::RdfBlock {
        let mut block = self.base.to_block();
        block.type_name = "WindTurbineType3IEC".to_string();
        if let Some(ref v) = self.wind_aero_one_dim_iec {
            block.fields.insert("WindTurbineType3IEC.WindAeroOneDimIEC".into(), crate::base::FieldValue::Resource(v.mrid.clone()));
        }
        if let Some(ref v) = self.wind_aero_two_dim_iec {
            block.fields.insert("WindTurbineType3IEC.WindAeroTwoDimIEC".into(), crate::base::FieldValue::Resource(v.mrid.clone()));
        }
        if let Some(ref v) = self.wind_cont_p_type3iec {
            block.fields.insert("WindTurbineType3IEC.WindContPType3IEC".into(), crate::base::FieldValue::Resource(v.mrid.clone()));
        }
        if let Some(ref v) = self.wind_cont_pitch_angle_iec {
            block.fields.insert("WindTurbineType3IEC.WindContPitchAngleIEC".into(), crate::base::FieldValue::Resource(v.mrid.clone()));
        }
        if let Some(ref v) = self.wind_gen_type3iec {
            block.fields.insert("WindTurbineType3IEC.WindGenType3IEC".into(), crate::base::FieldValue::Resource(v.mrid.clone()));
        }
        if let Some(ref v) = self.wind_mech_iec {
            block.fields.insert("WindTurbineType3IEC.WindMechIEC".into(), crate::base::FieldValue::Resource(v.mrid.clone()));
        }
        block
    }
}

impl WindTurbineType3IEC {
    pub fn from_block(b: &crate::base::RdfBlock) -> Self {
        let mut obj = Self::default();
        obj.base.base.base.base.id.clone_from(&b.mrid);
        for (key, val) in &b.fields {
            match key.as_str() {
                "WindTurbineType3IEC.WindAeroOneDimIEC" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.wind_aero_one_dim_iec = Some(crate::base::MridRef { mrid: sv.clone() });
                    }
                }
                "WindTurbineType3IEC.WindAeroTwoDimIEC" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.wind_aero_two_dim_iec = Some(crate::base::MridRef { mrid: sv.clone() });
                    }
                }
                "WindTurbineType3IEC.WindContPType3IEC" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.wind_cont_p_type3iec = Some(crate::base::MridRef { mrid: sv.clone() });
                    }
                }
                "WindTurbineType3IEC.WindContPitchAngleIEC" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.wind_cont_pitch_angle_iec = Some(crate::base::MridRef { mrid: sv.clone() });
                    }
                }
                "WindTurbineType3IEC.WindGenType3IEC" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.wind_gen_type3iec = Some(crate::base::MridRef { mrid: sv.clone() });
                    }
                }
                "WindTurbineType3IEC.WindMechIEC" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.wind_mech_iec = Some(crate::base::MridRef { mrid: sv.clone() });
                    }
                }
                "WindTurbineType3or4IEC.WIndContQIEC" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.base.w_ind_cont_qiec = Some(crate::base::MridRef { mrid: sv.clone() });
                    }
                }
                "WindTurbineType3or4IEC.WindContCurrLimIEC" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.base.wind_cont_curr_lim_iec = Some(crate::base::MridRef { mrid: sv.clone() });
                    }
                }
                "WindTurbineType3or4IEC.WindContQLimIEC" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.base.wind_cont_q_lim_iec = Some(crate::base::MridRef { mrid: sv.clone() });
                    }
                }
                "WindTurbineType3or4IEC.WindContQPQULimIEC" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.base.wind_cont_qpqu_lim_iec = Some(crate::base::MridRef { mrid: sv.clone() });
                    }
                }
                "WindTurbineType3or4IEC.WindProtectionIEC" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.base.wind_protection_iec = Some(crate::base::MridRef { mrid: sv.clone() });
                    }
                }
                "WindTurbineType3or4IEC.WindRefFrameRotIEC" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.base.wind_ref_frame_rot_iec = Some(crate::base::MridRef { mrid: sv.clone() });
                    }
                }
                "WindTurbineType3or4Dynamics.PowerElectronicsConnection" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.base.base.power_electronics_connection = Some(crate::base::MridRef { mrid: sv.clone() });
                    }
                }
                "WindTurbineType3or4Dynamics.RemoteInputSignal" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.base.base.remote_input_signal = Some(crate::base::MridRef { mrid: sv.clone() });
                    }
                }
                "WindTurbineType3or4Dynamics.WindPlantDynamics" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.base.base.wind_plant_dynamics = Some(crate::base::MridRef { mrid: sv.clone() });
                    }
                }
                "DynamicsFunctionBlock.enabled" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.base.base.base.enabled = Some(sv.trim() == "true"); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.base.base.base.enabled = Some(sv.trim() == "true"); }
                        }
                        _ => {}
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
