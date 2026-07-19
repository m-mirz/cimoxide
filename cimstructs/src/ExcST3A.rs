/// Modified IEEE ST3A static excitation system with added speed multiplier.
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct ExcST3A {
    #[serde(flatten)]
    pub base: super::ExcitationSystemDynamics,
    /// Maximum AVR output (Efdmax) (>= 0). Typical value = 6,9.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub efdmax: Option<f64>,
    /// Rectifier loading factor proportional to commutating reactance (Kc) (>= 0). Typical value = 1,1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kc: Option<f64>,
    /// Feedback gain constant of the inner loop field regulator (Kg) (>= 0). Typical value = 1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kg: Option<f64>,
    /// Potential circuit gain coefficient (Ki) (>= 0). Typical value = 4,83.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ki: Option<f64>,
    /// AVR gain (Kj) (> 0). Typical value = 200.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kj: Option<f64>,
    /// Forward gain constant of the inner loop field regulator (Km) (> 0). Typical value = 7,04.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub km: Option<f64>,
    /// Potential source gain (Kp) (> 0). Typical value = 4,37.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kp: Option<f64>,
    /// Coefficient to allow different usage of the model-speed coefficient (Ks). Typical value = 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ks: Option<f64>,
    /// Coefficient to allow different usage of the model-speed coefficient (Ks1). Typical value = 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ks1: Option<f64>,
    /// Voltage regulator time constant (Tb) (>= 0). Typical value = 6,67.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tb: Option<f64>,
    /// Voltage regulator time constant (Tc) (>= 0). Typical value = 1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tc: Option<f64>,
    /// Potential circuit phase angle (thetap). Typical value = 20.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thetap: Option<f64>,
    /// Forward time constant of inner loop field regulator (Tm) (> 0). Typical value = 1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tm: Option<f64>,
    /// Maximum excitation voltage (Vbmax) (> 0). Typical value = 8,63.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vbmax: Option<f64>,
    /// Maximum inner loop feedback voltage (Vgmax) (>= 0). Typical value = 6,53.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vgmax: Option<f64>,
    /// Maximum voltage regulator input limit (Vimax) (> 0). Typical value = 0,2.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vimax: Option<f64>,
    /// Minimum voltage regulator input limit (Vimin) (< 0). Typical value = -0,2.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vimin: Option<f64>,
    /// Maximum voltage regulator output (Vrmax) (> 0). Typical value = 1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vrmax: Option<f64>,
    /// Minimum voltage regulator output (Vrmin) (< 0). Typical value = -1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vrmin: Option<f64>,
    /// Reactance associated with potential source (Xl) (>= 0). Typical value = 0,09.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub xl: Option<f64>,
}
impl crate::base::CimElement for ExcST3A {
    fn mrid(&self) -> &str { &self.base.base.base.id }
    fn type_name(&self) -> &'static str { "ExcST3A" }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn to_json_value(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or(serde_json::Value::Null)
    }
    fn to_block(&self) -> crate::base::RdfBlock {
        let mut block = self.base.to_block();
        block.type_name = "ExcST3A".to_string();
        if let Some(v) = self.efdmax {
            block.fields.insert("ExcST3A.efdmax".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.kc {
            block.fields.insert("ExcST3A.kc".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.kg {
            block.fields.insert("ExcST3A.kg".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.ki {
            block.fields.insert("ExcST3A.ki".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.kj {
            block.fields.insert("ExcST3A.kj".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.km {
            block.fields.insert("ExcST3A.km".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.kp {
            block.fields.insert("ExcST3A.kp".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.ks {
            block.fields.insert("ExcST3A.ks".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.ks1 {
            block.fields.insert("ExcST3A.ks1".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.tb {
            block.fields.insert("ExcST3A.tb".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.tc {
            block.fields.insert("ExcST3A.tc".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.thetap {
            block.fields.insert("ExcST3A.thetap".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.tm {
            block.fields.insert("ExcST3A.tm".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.vbmax {
            block.fields.insert("ExcST3A.vbmax".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.vgmax {
            block.fields.insert("ExcST3A.vgmax".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.vimax {
            block.fields.insert("ExcST3A.vimax".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.vimin {
            block.fields.insert("ExcST3A.vimin".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.vrmax {
            block.fields.insert("ExcST3A.vrmax".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.vrmin {
            block.fields.insert("ExcST3A.vrmin".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.xl {
            block.fields.insert("ExcST3A.xl".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        block
    }
}

impl ExcST3A {
    pub fn from_block(b: &crate::base::RdfBlock) -> Self {
        let mut obj = Self::default();
        obj.base.base.base.id.clone_from(&b.mrid);
        for (key, val) in &b.fields {
            match key.as_str() {
                "ExcST3A.efdmax" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.efdmax = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.efdmax = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcST3A.kc" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.kc = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.kc = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcST3A.kg" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.kg = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.kg = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcST3A.ki" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.ki = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.ki = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcST3A.kj" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.kj = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.kj = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcST3A.km" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.km = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.km = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcST3A.kp" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.kp = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.kp = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcST3A.ks" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.ks = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.ks = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcST3A.ks1" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.ks1 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.ks1 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcST3A.tb" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.tb = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.tb = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcST3A.tc" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.tc = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.tc = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcST3A.thetap" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.thetap = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.thetap = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcST3A.tm" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.tm = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.tm = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcST3A.vbmax" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.vbmax = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.vbmax = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcST3A.vgmax" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.vgmax = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.vgmax = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcST3A.vimax" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.vimax = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.vimax = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcST3A.vimin" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.vimin = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.vimin = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcST3A.vrmax" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.vrmax = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.vrmax = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcST3A.vrmin" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.vrmin = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.vrmin = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcST3A.xl" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.xl = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.xl = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcitationSystemDynamics.SynchronousMachineDynamics" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.base.synchronous_machine_dynamics = Some(crate::base::MridRef { mrid: sv.clone() });
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
