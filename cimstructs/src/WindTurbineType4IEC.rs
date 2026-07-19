/// Parent class supporting relationships to IEC wind turbines type 4 including their control models.
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct WindTurbineType4IEC {
    #[serde(flatten)]
    pub base: super::WindTurbineType3or4IEC,
    /// Wind generator type 3A model associated with this wind turbine type 4 model.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wind_gen_type3a_iec: Option<super::base::MridRef>,
}
impl crate::base::CimElement for WindTurbineType4IEC {
    fn mrid(&self) -> &str { &self.base.base.base.base.id }
    fn type_name(&self) -> &'static str { "WindTurbineType4IEC" }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn to_json_value(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or(serde_json::Value::Null)
    }
    fn to_block(&self) -> crate::base::RdfBlock {
        let mut block = self.base.to_block();
        block.type_name = "WindTurbineType4IEC".to_string();
        if let Some(ref v) = self.wind_gen_type3a_iec {
            block.fields.insert("WindTurbineType4IEC.WindGenType3aIEC".into(), crate::base::FieldValue::Resource(v.mrid.clone()));
        }
        block
    }
}

impl WindTurbineType4IEC {
    pub fn from_block(b: &crate::base::RdfBlock) -> Self {
        let mut obj = Self::default();
        obj.base.base.base.base.id.clone_from(&b.mrid);
        for (key, val) in &b.fields {
            match key.as_str() {
                "WindTurbineType4IEC.WindGenType3aIEC" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.wind_gen_type3a_iec = Some(crate::base::MridRef { mrid: sv.clone() });
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
