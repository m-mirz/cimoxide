/// Simplified IEC type plant level model. Reference: IEC 61400-27-1:2015, Annex D.
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct WindPlantIEC {
    #[serde(flatten)]
    pub base: super::WindPlantDynamics,
    /// Wind plant frequency and active power control model associated with this wind plant.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wind_plant_freq_pcontrol_iec: Option<super::base::MridRef>,
    /// Wind plant model with which this wind reactive control is associated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wind_plant_reactive_control_iec: Option<super::base::MridRef>,
}
impl crate::base::CimElement for WindPlantIEC {
    fn mrid(&self) -> &str { &self.base.base.base.id }
    fn type_name(&self) -> &'static str { "WindPlantIEC" }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn to_json_value(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or(serde_json::Value::Null)
    }
    fn to_block(&self) -> crate::base::RdfBlock {
        let mut block = self.base.to_block();
        block.type_name = "WindPlantIEC".to_string();
        if let Some(ref v) = self.wind_plant_freq_pcontrol_iec {
            block.fields.insert("WindPlantIEC.WindPlantFreqPcontrolIEC".into(), crate::base::FieldValue::Resource(v.mrid.clone()));
        }
        if let Some(ref v) = self.wind_plant_reactive_control_iec {
            block.fields.insert("WindPlantIEC.WindPlantReactiveControlIEC".into(), crate::base::FieldValue::Resource(v.mrid.clone()));
        }
        block
    }
}

impl WindPlantIEC {
    pub fn from_block(b: &crate::base::RdfBlock) -> Self {
        let mut obj = Self::default();
        obj.base.base.base.id.clone_from(&b.mrid);
        for (key, val) in &b.fields {
            match key.as_str() {
                "WindPlantIEC.WindPlantFreqPcontrolIEC" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.wind_plant_freq_pcontrol_iec = Some(crate::base::MridRef { mrid: sv.clone() });
                    }
                }
                "WindPlantIEC.WindPlantReactiveControlIEC" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.wind_plant_reactive_control_iec = Some(crate::base::MridRef { mrid: sv.clone() });
                    }
                }
                "WindPlantDynamics.RemoteInputSignal" => {
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
