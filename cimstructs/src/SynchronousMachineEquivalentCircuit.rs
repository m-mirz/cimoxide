/// The electrical equations for all variations of the synchronous models are based on the SynchronousEquivalentCircuit diagram for the direct- and quadrature- axes. Equations for conversion between equivalent circuit and time constant reactance forms: Xd = Xad + Xl X’d = Xl + Xad x Xfd / (Xad + Xfd) X”d = Xl + Xad x Xfd x X1d / (Xad x Xfd + Xad x X1d + Xfd x X1d) Xq = Xaq + Xl X’q = Xl + Xaq x X1q / (Xaq + X1q) X”q = Xl + Xaq x X1q x X2q / (Xaq x X1q + Xaq x X2q + X1q x X2q) T’do = (Xad + Xfd) / (omega0 x Rfd) T”do = (Xad x Xfd + Xad x X1d + Xfd x X1d) / (omega0 x R1d x (Xad + Xfd) T’qo = (Xaq + X1q) / (omega0 x R1q) T”qo = (Xaq x X1q + Xaq x X2q + X1q x X2q) / (omega0 x R2q x (Xaq + X1q) Same equations using CIM attributes from SynchronousMachineTimeConstantReactance class on left of '=' and SynchronousMachineEquivalentCircuit class on right (except as noted): xDirectSync = xad + RotatingMachineDynamics.statorLeakageReactance xDirectTrans = RotatingMachineDynamics.statorLeakageReactance + xad x xfd / (xad + xfd) xDirectSubtrans = RotatingMachineDynamics.statorLeakageReactance + xad x xfd x x1d / (xad x xfd + xad x x1d + xfd x x1d) xQuadSync = xaq + RotatingMachineDynamics.statorLeakageReactance xQuadTrans = RotatingMachineDynamics.statorLeakageReactance + xaq x x1q / (xaq+ x1q) xQuadSubtrans = RotatingMachineDynamics.statorLeakageReactance + xaq x x1q x x2q / (xaq x x1q + xaq x x2q + x1q x x2q) tpdo = (xad + xfd) / (2 x pi x nominal frequency x rfd) tppdo = (xad x xfd + xad x x1d + xfd x x1d) / (2 x pi x nominal frequency x r1d x (xad + xfd) tpqo = (xaq + x1q) / (2 x pi x nominal frequency x r1q) tppqo = (xaq x x1q + xaq x x2q + x1q x x2q) / (2 x pi x nominal frequency x r2q x (xaq + x1q) These are only valid for a simplified model where 'Canay' reactance is zero.
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct SynchronousMachineEquivalentCircuit {
    #[serde(flatten)]
    pub base: super::SynchronousMachineDetailed,
    /// Direct-axis damper 1 winding resistance.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r1d: Option<f64>,
    /// Quadrature-axis damper 1 winding resistance.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r1q: Option<f64>,
    /// Quadrature-axis damper 2 winding resistance.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r2q: Option<f64>,
    /// Field winding resistance.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rfd: Option<f64>,
    /// Direct-axis damper 1 winding leakage reactance.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x1d: Option<f64>,
    /// Quadrature-axis damper 1 winding leakage reactance.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x1q: Option<f64>,
    /// Quadrature-axis damper 2 winding leakage reactance.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x2q: Option<f64>,
    /// Direct-axis mutual reactance.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub xad: Option<f64>,
    /// Quadrature-axis mutual reactance.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub xaq: Option<f64>,
    /// Differential mutual (“Canay”) reactance.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub xf1d: Option<f64>,
    /// Field winding leakage reactance.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub xfd: Option<f64>,
}
impl crate::base::CimElement for SynchronousMachineEquivalentCircuit {
    fn mrid(&self) -> &str { &self.base.base.base.base.base.id }
    fn type_name(&self) -> &'static str { "SynchronousMachineEquivalentCircuit" }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn to_json_value(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or(serde_json::Value::Null)
    }
    fn to_block(&self) -> crate::base::RdfBlock {
        let mut block = self.base.to_block();
        block.type_name = "SynchronousMachineEquivalentCircuit".to_string();
        if let Some(v) = self.r1d {
            block.fields.insert("SynchronousMachineEquivalentCircuit.r1d".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.r1q {
            block.fields.insert("SynchronousMachineEquivalentCircuit.r1q".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.r2q {
            block.fields.insert("SynchronousMachineEquivalentCircuit.r2q".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.rfd {
            block.fields.insert("SynchronousMachineEquivalentCircuit.rfd".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.x1d {
            block.fields.insert("SynchronousMachineEquivalentCircuit.x1d".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.x1q {
            block.fields.insert("SynchronousMachineEquivalentCircuit.x1q".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.x2q {
            block.fields.insert("SynchronousMachineEquivalentCircuit.x2q".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.xad {
            block.fields.insert("SynchronousMachineEquivalentCircuit.xad".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.xaq {
            block.fields.insert("SynchronousMachineEquivalentCircuit.xaq".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.xf1d {
            block.fields.insert("SynchronousMachineEquivalentCircuit.xf1d".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.xfd {
            block.fields.insert("SynchronousMachineEquivalentCircuit.xfd".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        block
    }
}

impl SynchronousMachineEquivalentCircuit {
    pub fn from_block(b: &crate::base::RdfBlock) -> Self {
        let mut obj = Self::default();
        obj.base.base.base.base.base.id.clone_from(&b.mrid);
        for (key, val) in &b.fields {
            match key.as_str() {
                "SynchronousMachineEquivalentCircuit.r1d" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.r1d = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.r1d = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "SynchronousMachineEquivalentCircuit.r1q" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.r1q = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.r1q = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "SynchronousMachineEquivalentCircuit.r2q" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.r2q = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.r2q = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "SynchronousMachineEquivalentCircuit.rfd" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.rfd = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.rfd = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "SynchronousMachineEquivalentCircuit.x1d" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.x1d = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.x1d = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "SynchronousMachineEquivalentCircuit.x1q" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.x1q = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.x1q = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "SynchronousMachineEquivalentCircuit.x2q" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.x2q = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.x2q = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "SynchronousMachineEquivalentCircuit.xad" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.xad = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.xad = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "SynchronousMachineEquivalentCircuit.xaq" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.xaq = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.xaq = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "SynchronousMachineEquivalentCircuit.xf1d" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.xf1d = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.xf1d = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "SynchronousMachineEquivalentCircuit.xfd" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.xfd = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.xfd = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "SynchronousMachineDetailed.efdBaseRatio" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.base.efd_base_ratio = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.base.efd_base_ratio = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "SynchronousMachineDetailed.ifdBaseType" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.base.ifd_base_type = Some(crate::base::UriRef { uri: sv.clone() });
                    }
                }
                "SynchronousMachineDetailed.saturationFactor120QAxis" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.base.saturation_factor120q_axis = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.base.saturation_factor120q_axis = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "SynchronousMachineDetailed.saturationFactorQAxis" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.base.saturation_factor_q_axis = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.base.saturation_factor_q_axis = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "SynchronousMachineDynamics.SynchronousMachine" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.base.base.synchronous_machine = Some(crate::base::MridRef { mrid: sv.clone() });
                    }
                }
                "RotatingMachineDynamics.damping" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.base.base.base.damping = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.base.base.base.damping = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "RotatingMachineDynamics.inertia" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.base.base.base.inertia = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.base.base.base.inertia = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "RotatingMachineDynamics.saturationFactor" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.base.base.base.saturation_factor = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.base.base.base.saturation_factor = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "RotatingMachineDynamics.saturationFactor120" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.base.base.base.saturation_factor120 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.base.base.base.saturation_factor120 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "RotatingMachineDynamics.statorLeakageReactance" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.base.base.base.stator_leakage_reactance = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.base.base.base.stator_leakage_reactance = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "RotatingMachineDynamics.statorResistance" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.base.base.base.stator_resistance = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.base.base.base.stator_resistance = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "DynamicsFunctionBlock.enabled" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.base.base.base.base.enabled = Some(sv.trim() == "true"); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.base.base.base.base.enabled = Some(sv.trim() == "true"); }
                        }
                        _ => {}
                    }
                }
                "IdentifiedObject.description" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.base.base.base.base.base.description = sv.clone(); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.base.base.base.base.base.description = sv.clone(); }
                        }
                        _ => {}
                    }
                }
                "IdentifiedObject.energyIdentCodeEic" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.base.base.base.base.base.energy_ident_code_eic = sv.clone(); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.base.base.base.base.base.energy_ident_code_eic = sv.clone(); }
                        }
                        _ => {}
                    }
                }
                "IdentifiedObject.mRID" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.base.base.base.base.base.m_rid = sv.clone(); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.base.base.base.base.base.m_rid = sv.clone(); }
                        }
                        _ => {}
                    }
                }
                "IdentifiedObject.name" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.base.base.base.base.base.name = sv.clone(); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.base.base.base.base.base.name = sv.clone(); }
                        }
                        _ => {}
                    }
                }
                "IdentifiedObject.shortName" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.base.base.base.base.base.short_name = sv.clone(); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.base.base.base.base.base.short_name = sv.clone(); }
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
