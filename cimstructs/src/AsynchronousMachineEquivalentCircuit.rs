/// The electrical equations of all variations of the asynchronous model are based on the AsynchronousEquivalentCircuit diagram for the direct- and quadrature- axes, with two equivalent rotor windings in each axis. Equations for conversion between equivalent circuit and time constant reactance forms: Xs = Xm + Xl X' = Xl + Xm x Xlr1 / (Xm + Xlr1) X'' = Xl + Xm x Xlr1 x Xlr2 / (Xm x Xlr1 + Xm x Xlr2 + Xlr1 x Xlr2) T'o = (Xm + Xlr1) / (omega0 x Rr1) T''o = (Xm x Xlr1 + Xm x Xlr2 + Xlr1 x Xlr2) / (omega0 x Rr2 x (Xm + Xlr1) Same equations using CIM attributes from AsynchronousMachineTimeConstantReactance class on left of '=' and AsynchronousMachineEquivalentCircuit class on right (except as noted): xs = xm + RotatingMachineDynamics.statorLeakageReactance xp = RotatingMachineDynamics.statorLeakageReactance + xm x xlr1 / (xm + xlr1) xpp = RotatingMachineDynamics.statorLeakageReactance + xm x xlr1 x xlr2 / (xm x xlr1 + xm x xlr2 + xlr1 x xlr2) tpo = (xm + xlr1) / (2 x pi x nominal frequency x rr1) tppo = (xm x xlr1 + xm x xlr2 + xlr1 x xlr2) / (2 x pi x nominal frequency x rr2 x (xm + xlr1).
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct AsynchronousMachineEquivalentCircuit {
    #[serde(flatten)]
    pub base: super::AsynchronousMachineDynamics,
    /// Damper 1 winding resistance.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rr1: Option<f64>,
    /// Damper 2 winding resistance.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rr2: Option<f64>,
    /// Damper 1 winding leakage reactance.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub xlr1: Option<f64>,
    /// Damper 2 winding leakage reactance.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub xlr2: Option<f64>,
    /// Magnetizing reactance.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub xm: Option<f64>,
}
impl crate::base::CimElement for AsynchronousMachineEquivalentCircuit {
    fn mrid(&self) -> &str { &self.base.base.base.base.id }
    fn type_name(&self) -> &'static str { "AsynchronousMachineEquivalentCircuit" }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn to_json_value(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or(serde_json::Value::Null)
    }
    fn to_block(&self) -> crate::base::RdfBlock {
        let mut block = self.base.to_block();
        block.type_name = "AsynchronousMachineEquivalentCircuit".to_string();
        if let Some(v) = self.rr1 {
            block.fields.insert("AsynchronousMachineEquivalentCircuit.rr1".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.rr2 {
            block.fields.insert("AsynchronousMachineEquivalentCircuit.rr2".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.xlr1 {
            block.fields.insert("AsynchronousMachineEquivalentCircuit.xlr1".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.xlr2 {
            block.fields.insert("AsynchronousMachineEquivalentCircuit.xlr2".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.xm {
            block.fields.insert("AsynchronousMachineEquivalentCircuit.xm".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        block
    }
}

impl AsynchronousMachineEquivalentCircuit {
    pub fn from_block(b: &crate::base::RdfBlock) -> Self {
        let mut obj = Self::default();
        obj.base.base.base.base.id.clone_from(&b.mrid);
        for (key, val) in &b.fields {
            match key.as_str() {
                "AsynchronousMachineEquivalentCircuit.rr1" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.rr1 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.rr1 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "AsynchronousMachineEquivalentCircuit.rr2" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.rr2 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.rr2 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "AsynchronousMachineEquivalentCircuit.xlr1" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.xlr1 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.xlr1 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "AsynchronousMachineEquivalentCircuit.xlr2" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.xlr2 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.xlr2 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "AsynchronousMachineEquivalentCircuit.xm" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.xm = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.xm = Some(v); } }
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
