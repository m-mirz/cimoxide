/// All synchronous machine detailed types use a subset of the same data parameters and input/output variables. The several variations differ in the following ways: - the number of equivalent windings that are included; - the way in which saturation is incorporated into the model; - whether or not “subtransient saliency” (X''q not = X''d) is represented. It is not necessary for each simulation tool to have separate models for each of the model types. The same model can often be used for several types by alternative logic within the model. Also, differences in saturation representation might not result in significant model performance differences so model substitutions are often acceptable.
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct SynchronousMachineDetailed {
    #[serde(flatten)]
    pub base: super::SynchronousMachineDynamics,
    /// Ratio (exciter voltage/generator voltage) of Efd bases of exciter and generator models (> 0). Typical value = 1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub efd_base_ratio: Option<f64>,
    /// Excitation base system mode. It should be equal to the value of WLMDV given by the user. WLMDV is the PU ratio between the field voltage and the excitation current: Efd = WLMDV x Ifd. Typical value = ifag.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ifd_base_type: Option<super::base::UriRef>,
    /// Quadrature-axis saturation factor at 120% of rated terminal voltage (S12q) (>= SynchonousMachineDetailed.saturationFactorQAxis). Typical value = 0,12.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub saturation_factor120q_axis: Option<f64>,
    /// Quadrature-axis saturation factor at rated terminal voltage (S1q) (>= 0). Typical value = 0,02.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub saturation_factor_q_axis: Option<f64>,
}
impl crate::base::CimElement for SynchronousMachineDetailed {
    fn mrid(&self) -> &str { &self.base.base.base.base.id }
    fn type_name(&self) -> &'static str { "SynchronousMachineDetailed" }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn to_json_value(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or(serde_json::Value::Null)
    }
    fn to_block(&self) -> crate::base::RdfBlock {
        let mut block = self.base.to_block();
        block.type_name = "SynchronousMachineDetailed".to_string();
        if let Some(v) = self.efd_base_ratio {
            block.fields.insert("SynchronousMachineDetailed.efdBaseRatio".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(ref v) = self.ifd_base_type {
            block.fields.insert("SynchronousMachineDetailed.ifdBaseType".into(), crate::base::FieldValue::Resource(v.uri.clone()));
        }
        if let Some(v) = self.saturation_factor120q_axis {
            block.fields.insert("SynchronousMachineDetailed.saturationFactor120QAxis".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.saturation_factor_q_axis {
            block.fields.insert("SynchronousMachineDetailed.saturationFactorQAxis".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        block
    }
}

impl SynchronousMachineDetailed {
    pub fn from_block(b: &crate::base::RdfBlock) -> Self {
        let mut obj = Self::default();
        obj.base.base.base.base.id.clone_from(&b.mrid);
        for (key, val) in &b.fields {
            match key.as_str() {
                "SynchronousMachineDetailed.efdBaseRatio" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.efd_base_ratio = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.efd_base_ratio = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "SynchronousMachineDetailed.ifdBaseType" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.ifd_base_type = Some(crate::base::UriRef { uri: sv.clone() });
                    }
                }
                "SynchronousMachineDetailed.saturationFactor120QAxis" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.saturation_factor120q_axis = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.saturation_factor120q_axis = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "SynchronousMachineDetailed.saturationFactorQAxis" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.saturation_factor_q_axis = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.saturation_factor_q_axis = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "SynchronousMachineDynamics.SynchronousMachine" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.base.synchronous_machine = Some(crate::base::MridRef { mrid: sv.clone() });
                    }
                }
                "RotatingMachineDynamics.damping" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.base.base.damping = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.base.base.damping = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "RotatingMachineDynamics.inertia" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.base.base.inertia = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.base.base.inertia = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "RotatingMachineDynamics.saturationFactor" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.base.base.saturation_factor = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.base.base.saturation_factor = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "RotatingMachineDynamics.saturationFactor120" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.base.base.saturation_factor120 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.base.base.saturation_factor120 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "RotatingMachineDynamics.statorLeakageReactance" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.base.base.stator_leakage_reactance = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.base.base.stator_leakage_reactance = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "RotatingMachineDynamics.statorResistance" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.base.base.stator_resistance = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.base.base.stator_resistance = Some(v); } }
                        }
                        _ => {}
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
