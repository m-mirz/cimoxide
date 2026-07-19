/// Parent class supporting relationships to IEC wind turbines type 1 and type 2 including their control models. Generator model for wind turbine of IEC type 1 or type 2 is a standard asynchronous generator model. Reference: IEC 61400-27-1:2015, 5.5.2 and 5.5.3.
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct WindTurbineType1or2IEC {
    #[serde(flatten)]
    pub base: super::WindTurbineType1or2Dynamics,
    /// Wind mechanical model associated with this wind generator type 1 or type 2 model.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wind_mech_iec: Option<super::base::MridRef>,
    /// Wind turbune protection model associated with this wind generator type 1 or type 2 model.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wind_protection_iec: Option<super::base::MridRef>,
}
impl crate::base::CimElement for WindTurbineType1or2IEC {
    fn mrid(&self) -> &str { &self.base.base.base.id }
    fn type_name(&self) -> &'static str { "WindTurbineType1or2IEC" }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn to_json_value(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or(serde_json::Value::Null)
    }
    fn to_block(&self) -> crate::base::RdfBlock {
        let mut block = self.base.to_block();
        block.type_name = "WindTurbineType1or2IEC".to_string();
        if let Some(ref v) = self.wind_mech_iec {
            block.fields.insert("WindTurbineType1or2IEC.WindMechIEC".into(), crate::base::FieldValue::Resource(v.mrid.clone()));
        }
        if let Some(ref v) = self.wind_protection_iec {
            block.fields.insert("WindTurbineType1or2IEC.WindProtectionIEC".into(), crate::base::FieldValue::Resource(v.mrid.clone()));
        }
        block
    }
}

impl WindTurbineType1or2IEC {
    pub fn from_block(b: &crate::base::RdfBlock) -> Self {
        let mut obj = Self::default();
        obj.base.base.base.id.clone_from(&b.mrid);
        for (key, val) in &b.fields {
            match key.as_str() {
                "WindTurbineType1or2IEC.WindMechIEC" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.wind_mech_iec = Some(crate::base::MridRef { mrid: sv.clone() });
                    }
                }
                "WindTurbineType1or2IEC.WindProtectionIEC" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.wind_protection_iec = Some(crate::base::MridRef { mrid: sv.clone() });
                    }
                }
                "WindTurbineType1or2Dynamics.AsynchronousMachineDynamics" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.base.asynchronous_machine_dynamics = Some(crate::base::MridRef { mrid: sv.clone() });
                    }
                }
                "WindTurbineType1or2Dynamics.RemoteInputSignal" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.base.remote_input_signal = Some(crate::base::MridRef { mrid: sv.clone() });
                    }
                }
                "DynamicsFunctionBlock.enabled" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.base.base.enabled = Some(sv.trim() == "true"); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.base.base.enabled = Some(sv.trim() == "true"); }
                        }
                        _ => {}
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
