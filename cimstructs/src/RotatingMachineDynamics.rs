/// Abstract parent class for all synchronous and asynchronous machine standard models.
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct RotatingMachineDynamics {
    #[serde(flatten)]
    pub base: super::DynamicsFunctionBlock,
    /// Damping torque coefficient (D) (>= 0). A proportionality constant that, when multiplied by the angular velocity of the rotor poles with respect to the magnetic field (frequency), results in the damping torque. This value is often zero when the sources of damping torques (generator damper windings, load damping effects, etc.) are modelled in detail. Typical value = 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub damping: Option<f64>,
    /// Inertia constant of generator or motor and mechanical load (H) (> 0). This is the specification for the stored energy in the rotating mass when operating at rated speed. For a generator, this includes the generator plus all other elements (turbine, exciter) on the same shaft and has units of MW x s. For a motor, it includes the motor plus its mechanical load. Conventional units are PU on the generator MVA base, usually expressed as MW x s / MVA or just s. This value is used in the accelerating power reference frame for operator training simulator solutions. Typical value = 3.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inertia: Option<f64>,
    /// Saturation factor at rated terminal voltage (S1) (>= 0). Not used by simplified model. Defined by defined by S(E1) in the SynchronousMachineSaturationParameters diagram. Typical value = 0,02.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub saturation_factor: Option<f64>,
    /// Saturation factor at 120% of rated terminal voltage (S12) (>= RotatingMachineDynamics.saturationFactor). Not used by the simplified model, defined by S(E2) in the SynchronousMachineSaturationParameters diagram. Typical value = 0,12.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub saturation_factor120: Option<f64>,
    /// Stator leakage reactance (Xl) (>= 0). Typical value = 0,15.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stator_leakage_reactance: Option<f64>,
    /// Stator (armature) resistance (Rs) (>= 0). Typical value = 0,005.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stator_resistance: Option<f64>,
}
impl crate::base::CimElement for RotatingMachineDynamics {
    fn mrid(&self) -> &str { &self.base.base.id }
    fn type_name(&self) -> &'static str { "RotatingMachineDynamics" }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn to_json_value(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or(serde_json::Value::Null)
    }
    fn to_block(&self) -> crate::base::RdfBlock {
        let mut block = self.base.to_block();
        block.type_name = "RotatingMachineDynamics".to_string();
        if let Some(v) = self.damping {
            block.fields.insert("RotatingMachineDynamics.damping".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.inertia {
            block.fields.insert("RotatingMachineDynamics.inertia".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.saturation_factor {
            block.fields.insert("RotatingMachineDynamics.saturationFactor".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.saturation_factor120 {
            block.fields.insert("RotatingMachineDynamics.saturationFactor120".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.stator_leakage_reactance {
            block.fields.insert("RotatingMachineDynamics.statorLeakageReactance".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.stator_resistance {
            block.fields.insert("RotatingMachineDynamics.statorResistance".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        block
    }
}

impl RotatingMachineDynamics {
    pub fn from_block(b: &crate::base::RdfBlock) -> Self {
        let mut obj = Self::default();
        obj.base.base.id.clone_from(&b.mrid);
        for (key, val) in &b.fields {
            match key.as_str() {
                "RotatingMachineDynamics.damping" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.damping = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.damping = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "RotatingMachineDynamics.inertia" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.inertia = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.inertia = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "RotatingMachineDynamics.saturationFactor" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.saturation_factor = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.saturation_factor = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "RotatingMachineDynamics.saturationFactor120" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.saturation_factor120 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.saturation_factor120 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "RotatingMachineDynamics.statorLeakageReactance" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.stator_leakage_reactance = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.stator_leakage_reactance = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "RotatingMachineDynamics.statorResistance" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.stator_resistance = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.stator_resistance = Some(v); } }
                        }
                        _ => {}
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
