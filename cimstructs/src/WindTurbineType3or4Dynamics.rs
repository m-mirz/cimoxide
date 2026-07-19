/// Parent class supporting relationships to wind turbines type 3 and type 4 and wind plant including their control models.
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct WindTurbineType3or4Dynamics {
    #[serde(flatten)]
    pub base: super::DynamicsFunctionBlock,
    /// The power electronics connection associated with this wind turbine type 3 or type 4 dynamics model.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub power_electronics_connection: Option<super::base::MridRef>,
    /// Remote input signal used by these wind turbine type 3 or type 4 models.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remote_input_signal: Option<super::base::MridRef>,
    /// The wind plant with which the wind turbines type 3 or type 4 are associated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wind_plant_dynamics: Option<super::base::MridRef>,
}
impl crate::base::CimElement for WindTurbineType3or4Dynamics {
    fn mrid(&self) -> &str { &self.base.base.id }
    fn type_name(&self) -> &'static str { "WindTurbineType3or4Dynamics" }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn to_json_value(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or(serde_json::Value::Null)
    }
    fn to_block(&self) -> crate::base::RdfBlock {
        let mut block = self.base.to_block();
        block.type_name = "WindTurbineType3or4Dynamics".to_string();
        if let Some(ref v) = self.power_electronics_connection {
            block.fields.insert("WindTurbineType3or4Dynamics.PowerElectronicsConnection".into(), crate::base::FieldValue::Resource(v.mrid.clone()));
        }
        if let Some(ref v) = self.remote_input_signal {
            block.fields.insert("WindTurbineType3or4Dynamics.RemoteInputSignal".into(), crate::base::FieldValue::Resource(v.mrid.clone()));
        }
        if let Some(ref v) = self.wind_plant_dynamics {
            block.fields.insert("WindTurbineType3or4Dynamics.WindPlantDynamics".into(), crate::base::FieldValue::Resource(v.mrid.clone()));
        }
        block
    }
}

impl WindTurbineType3or4Dynamics {
    pub fn from_block(b: &crate::base::RdfBlock) -> Self {
        let mut obj = Self::default();
        obj.base.base.id.clone_from(&b.mrid);
        for (key, val) in &b.fields {
            match key.as_str() {
                "WindTurbineType3or4Dynamics.PowerElectronicsConnection" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.power_electronics_connection = Some(crate::base::MridRef { mrid: sv.clone() });
                    }
                }
                "WindTurbineType3or4Dynamics.RemoteInputSignal" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.remote_input_signal = Some(crate::base::MridRef { mrid: sv.clone() });
                    }
                }
                "WindTurbineType3or4Dynamics.WindPlantDynamics" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.wind_plant_dynamics = Some(crate::base::MridRef { mrid: sv.clone() });
                    }
                }
                "DynamicsFunctionBlock.enabled" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.base.enabled = Some(sv.trim() == "true"); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.base.enabled = Some(sv.trim() == "true"); }
                        }
                        _ => {}
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
