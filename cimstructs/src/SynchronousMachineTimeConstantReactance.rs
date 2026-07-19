/// Synchronous machine detailed modelling types are defined by the combination of the attributes SynchronousMachineTimeConstantReactance.modelType and SynchronousMachineTimeConstantReactance.rotorType. Parameter details: The “p” in the time-related attribute names is a substitution for a “prime” in the usual parameter notation, e.g. tpdo refers to T'do. The parameters used for models expressed in time constant reactance form include: - RotatingMachine.ratedS (MVAbase); - RotatingMachineDynamics.damping (D); - RotatingMachineDynamics.inertia (H); - RotatingMachineDynamics.saturationFactor (S1); - RotatingMachineDynamics.saturationFactor120 (S12); - RotatingMachineDynamics.statorLeakageReactance (Xl); - RotatingMachineDynamics.statorResistance (Rs); - SynchronousMachineTimeConstantReactance.ks (Ks); - SynchronousMachineDetailed.saturationFactorQAxis (S1q); - SynchronousMachineDetailed.saturationFactor120QAxis (S12q); - SynchronousMachineDetailed.efdBaseRatio; - SynchronousMachineDetailed.ifdBaseType; - .xDirectSync (Xd); - .xDirectTrans (X'd); - .xDirectSubtrans (X''d); - .xQuadSync (Xq); - .xQuadTrans (X'q); - .xQuadSubtrans (X''q); - .tpdo (T'do); - .tppdo (T''do); - .tpqo (T'qo); - .tppqo (T''qo); - .tc.
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct SynchronousMachineTimeConstantReactance {
    #[serde(flatten)]
    pub base: super::SynchronousMachineDetailed,
    /// Saturation loading correction factor (Ks) (>= 0). Used only by type J model. Typical value = 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ks: Option<f64>,
    /// Type of synchronous machine model used in dynamic simulation applications.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_type: Option<super::base::UriRef>,
    /// Type of rotor on physical machine.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rotor_type: Option<super::base::UriRef>,
    /// Damping time constant for “Canay” reactance (>= 0). Typical value = 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tc: Option<f64>,
    /// Direct-axis transient rotor time constant (T'do) (> SynchronousMachineTimeConstantReactance.tppdo). Typical value = 5.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tpdo: Option<f64>,
    /// Direct-axis subtransient rotor time constant (T''do) (> 0). Typical value = 0,03.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tppdo: Option<f64>,
    /// Quadrature-axis subtransient rotor time constant (T''qo) (> 0). Typical value = 0,03.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tppqo: Option<f64>,
    /// Quadrature-axis transient rotor time constant (T'qo) (> SynchronousMachineTimeConstantReactance.tppqo). Typical value = 0,5.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tpqo: Option<f64>,
    /// Direct-axis subtransient reactance (unsaturated) (X''d) (> RotatingMachineDynamics.statorLeakageReactance). Typical value = 0,2.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x_direct_subtrans: Option<f64>,
    /// Direct-axis synchronous reactance (Xd) (>= SynchronousMachineTimeConstantReactance.xDirectTrans). The quotient of a sustained value of that AC component of armature voltage that is produced by the total direct-axis flux due to direct-axis armature current and the value of the AC component of this current, the machine running at rated speed. Typical value = 1,8.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x_direct_sync: Option<f64>,
    /// Direct-axis transient reactance (unsaturated) (X'd) (>= SynchronousMachineTimeConstantReactance.xDirectSubtrans). Typical value = 0,5.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x_direct_trans: Option<f64>,
    /// Quadrature-axis subtransient reactance (X''q) (> RotatingMachineDynamics.statorLeakageReactance). Typical value = 0,2.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x_quad_subtrans: Option<f64>,
    /// Quadrature-axis synchronous reactance (Xq) (>= SynchronousMachineTimeConstantReactance.xQuadTrans). The ratio of the component of reactive armature voltage, due to the quadrature-axis component of armature current, to this component of current, under steady state conditions and at rated frequency. Typical value = 1,6.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x_quad_sync: Option<f64>,
    /// Quadrature-axis transient reactance (X'q) (>= SynchronousMachineTimeConstantReactance.xQuadSubtrans). Typical value = 0,3.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x_quad_trans: Option<f64>,
}
impl crate::base::CimElement for SynchronousMachineTimeConstantReactance {
    fn mrid(&self) -> &str { &self.base.base.base.base.base.id }
    fn type_name(&self) -> &'static str { "SynchronousMachineTimeConstantReactance" }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn to_json_value(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or(serde_json::Value::Null)
    }
    fn to_block(&self) -> crate::base::RdfBlock {
        let mut block = self.base.to_block();
        block.type_name = "SynchronousMachineTimeConstantReactance".to_string();
        if let Some(v) = self.ks {
            block.fields.insert("SynchronousMachineTimeConstantReactance.ks".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(ref v) = self.model_type {
            block.fields.insert("SynchronousMachineTimeConstantReactance.modelType".into(), crate::base::FieldValue::Resource(v.uri.clone()));
        }
        if let Some(ref v) = self.rotor_type {
            block.fields.insert("SynchronousMachineTimeConstantReactance.rotorType".into(), crate::base::FieldValue::Resource(v.uri.clone()));
        }
        if let Some(v) = self.tc {
            block.fields.insert("SynchronousMachineTimeConstantReactance.tc".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.tpdo {
            block.fields.insert("SynchronousMachineTimeConstantReactance.tpdo".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.tppdo {
            block.fields.insert("SynchronousMachineTimeConstantReactance.tppdo".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.tppqo {
            block.fields.insert("SynchronousMachineTimeConstantReactance.tppqo".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.tpqo {
            block.fields.insert("SynchronousMachineTimeConstantReactance.tpqo".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.x_direct_subtrans {
            block.fields.insert("SynchronousMachineTimeConstantReactance.xDirectSubtrans".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.x_direct_sync {
            block.fields.insert("SynchronousMachineTimeConstantReactance.xDirectSync".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.x_direct_trans {
            block.fields.insert("SynchronousMachineTimeConstantReactance.xDirectTrans".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.x_quad_subtrans {
            block.fields.insert("SynchronousMachineTimeConstantReactance.xQuadSubtrans".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.x_quad_sync {
            block.fields.insert("SynchronousMachineTimeConstantReactance.xQuadSync".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.x_quad_trans {
            block.fields.insert("SynchronousMachineTimeConstantReactance.xQuadTrans".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        block
    }
}

impl SynchronousMachineTimeConstantReactance {
    pub fn from_block(b: &crate::base::RdfBlock) -> Self {
        let mut obj = Self::default();
        obj.base.base.base.base.base.id.clone_from(&b.mrid);
        for (key, val) in &b.fields {
            match key.as_str() {
                "SynchronousMachineTimeConstantReactance.ks" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.ks = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.ks = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "SynchronousMachineTimeConstantReactance.modelType" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.model_type = Some(crate::base::UriRef { uri: sv.clone() });
                    }
                }
                "SynchronousMachineTimeConstantReactance.rotorType" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.rotor_type = Some(crate::base::UriRef { uri: sv.clone() });
                    }
                }
                "SynchronousMachineTimeConstantReactance.tc" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.tc = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.tc = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "SynchronousMachineTimeConstantReactance.tpdo" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.tpdo = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.tpdo = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "SynchronousMachineTimeConstantReactance.tppdo" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.tppdo = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.tppdo = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "SynchronousMachineTimeConstantReactance.tppqo" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.tppqo = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.tppqo = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "SynchronousMachineTimeConstantReactance.tpqo" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.tpqo = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.tpqo = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "SynchronousMachineTimeConstantReactance.xDirectSubtrans" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.x_direct_subtrans = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.x_direct_subtrans = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "SynchronousMachineTimeConstantReactance.xDirectSync" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.x_direct_sync = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.x_direct_sync = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "SynchronousMachineTimeConstantReactance.xDirectTrans" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.x_direct_trans = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.x_direct_trans = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "SynchronousMachineTimeConstantReactance.xQuadSubtrans" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.x_quad_subtrans = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.x_quad_subtrans = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "SynchronousMachineTimeConstantReactance.xQuadSync" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.x_quad_sync = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.x_quad_sync = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "SynchronousMachineTimeConstantReactance.xQuadTrans" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.x_quad_trans = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.x_quad_trans = Some(v); } }
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
