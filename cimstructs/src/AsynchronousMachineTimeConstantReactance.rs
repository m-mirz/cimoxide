/// Parameter details: If X'' = X', a single cage (one equivalent rotor winding per axis) is modelled. The “p” in the attribute names is a substitution for a “prime” in the usual parameter notation, e.g. tpo refers to T'o. The parameters used for models expressed in time constant reactance form include: - RotatingMachine.ratedS (MVAbase); - RotatingMachineDynamics.damping (D); - RotatingMachineDynamics.inertia (H); - RotatingMachineDynamics.saturationFactor (S1); - RotatingMachineDynamics.saturationFactor120 (S12); - RotatingMachineDynamics.statorLeakageReactance (Xl); - RotatingMachineDynamics.statorResistance (Rs); - .xs (Xs); - .xp (X'); - .xpp (X''); - .tpo (T'o); - .tppo (T''o).
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct AsynchronousMachineTimeConstantReactance {
    #[serde(flatten)]
    pub base: super::AsynchronousMachineDynamics,
    /// Transient rotor time constant (T'o) (> AsynchronousMachineTimeConstantReactance.tppo). Typical value = 5.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tpo: Option<f64>,
    /// Subtransient rotor time constant (T''o) (> 0). Typical value = 0,03.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tppo: Option<f64>,
    /// Transient reactance (unsaturated) (X') (>= AsynchronousMachineTimeConstantReactance.xpp). Typical value = 0,5.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub xp: Option<f64>,
    /// Subtransient reactance (unsaturated) (X'') (> RotatingMachineDynamics.statorLeakageReactance). Typical value = 0,2.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub xpp: Option<f64>,
    /// Synchronous reactance (Xs) (>= AsynchronousMachineTimeConstantReactance.xp). Typical value = 1,8.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub xs: Option<f64>,
}
impl crate::base::CimElement for AsynchronousMachineTimeConstantReactance {
    fn mrid(&self) -> &str { &self.base.base.base.base.id }
    fn type_name(&self) -> &'static str { "AsynchronousMachineTimeConstantReactance" }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn to_json_value(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or(serde_json::Value::Null)
    }
    fn to_block(&self) -> crate::base::RdfBlock {
        let mut block = self.base.to_block();
        block.type_name = "AsynchronousMachineTimeConstantReactance".to_string();
        if let Some(v) = self.tpo {
            block.fields.insert("AsynchronousMachineTimeConstantReactance.tpo".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.tppo {
            block.fields.insert("AsynchronousMachineTimeConstantReactance.tppo".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.xp {
            block.fields.insert("AsynchronousMachineTimeConstantReactance.xp".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.xpp {
            block.fields.insert("AsynchronousMachineTimeConstantReactance.xpp".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.xs {
            block.fields.insert("AsynchronousMachineTimeConstantReactance.xs".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        block
    }
}

impl AsynchronousMachineTimeConstantReactance {
    pub fn from_block(b: &crate::base::RdfBlock) -> Self {
        let mut obj = Self::default();
        obj.base.base.base.base.id.clone_from(&b.mrid);
        for (key, val) in &b.fields {
            match key.as_str() {
                "AsynchronousMachineTimeConstantReactance.tpo" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.tpo = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.tpo = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "AsynchronousMachineTimeConstantReactance.tppo" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.tppo = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.tppo = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "AsynchronousMachineTimeConstantReactance.xp" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.xp = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.xp = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "AsynchronousMachineTimeConstantReactance.xpp" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.xpp = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.xpp = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "AsynchronousMachineTimeConstantReactance.xs" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.xs = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.xs = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "AsynchronousMachineDynamics.AsynchronousMachine" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.base.asynchronous_machine = Some(crate::base::MridRef { mrid: sv.clone() });
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
