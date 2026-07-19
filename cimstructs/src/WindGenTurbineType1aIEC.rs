/// Wind turbine IEC type 1A. Reference: IEC 61400-27-1:2015, 5.5.2.2.
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct WindGenTurbineType1aIEC {
    #[serde(flatten)]
    pub base: super::WindTurbineType1or2IEC,
    /// Wind aerodynamic model associated with this wind turbine type 1A model.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wind_aero_const_iec: Option<super::base::MridRef>,
}
impl crate::base::CimElement for WindGenTurbineType1aIEC {
    fn mrid(&self) -> &str { &self.base.base.base.base.id }
    fn type_name(&self) -> &'static str { "WindGenTurbineType1aIEC" }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn to_json_value(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or(serde_json::Value::Null)
    }
    fn to_block(&self) -> crate::base::RdfBlock {
        let mut block = self.base.to_block();
        block.type_name = "WindGenTurbineType1aIEC".to_string();
        if let Some(ref v) = self.wind_aero_const_iec {
            block.fields.insert("WindGenTurbineType1aIEC.WindAeroConstIEC".into(), crate::base::FieldValue::Resource(v.mrid.clone()));
        }
        block
    }
}

impl WindGenTurbineType1aIEC {
    pub fn from_block(b: &crate::base::RdfBlock) -> Self {
        let mut obj = Self::default();
        obj.base.base.base.base.id.clone_from(&b.mrid);
        for (key, val) in &b.fields {
            match key.as_str() {
                "WindGenTurbineType1aIEC.WindAeroConstIEC" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.wind_aero_const_iec = Some(crate::base::MridRef { mrid: sv.clone() });
                    }
                }
                "WindTurbineType1or2IEC.WindMechIEC" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.base.wind_mech_iec = Some(crate::base::MridRef { mrid: sv.clone() });
                    }
                }
                "WindTurbineType1or2IEC.WindProtectionIEC" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.base.wind_protection_iec = Some(crate::base::MridRef { mrid: sv.clone() });
                    }
                }
                "WindTurbineType1or2Dynamics.AsynchronousMachineDynamics" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.base.base.asynchronous_machine_dynamics = Some(crate::base::MridRef { mrid: sv.clone() });
                    }
                }
                "WindTurbineType1or2Dynamics.RemoteInputSignal" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.base.base.remote_input_signal = Some(crate::base::MridRef { mrid: sv.clone() });
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
