/// Type UEL1 model which has a circular limit boundary when plotted in terms of machine reactive power vs. real power output. Reference: IEEE UEL1 421.5-2005, 10.1.
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct UnderexcLimIEEE1 {
    #[serde(flatten)]
    pub base: super::UnderexcitationLimiterDynamics,
    /// UEL centre setting (KUC). Typical value = 1,38.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kuc: Option<f64>,
    /// UEL excitation system stabilizer gain (KUF). Typical value = 3,3.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kuf: Option<f64>,
    /// UEL integral gain (KUI). Typical value = 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kui: Option<f64>,
    /// UEL proportional gain (KUL). Typical value = 100.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kul: Option<f64>,
    /// UEL radius setting (KUR). Typical value = 1,95.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kur: Option<f64>,
    /// UEL lead time constant (TU1) (>= 0). Typical value = 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tu1: Option<f64>,
    /// UEL lag time constant (TU2) (>= 0). Typical value = 0,05.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tu2: Option<f64>,
    /// UEL lead time constant (TU3) (>= 0). Typical value = 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tu3: Option<f64>,
    /// UEL lag time constant (TU4) (>= 0). Typical value = 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tu4: Option<f64>,
    /// UEL maximum limit for operating point phasor magnitude (VUCMAX). Typical value = 5,8.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vucmax: Option<f64>,
    /// UEL integrator output maximum limit (VUIMAX) (> UnderexcLimIEEE1.vuimin).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vuimax: Option<f64>,
    /// UEL integrator output minimum limit (VUIMIN) (< UnderexcLimIEEE1.vuimax).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vuimin: Option<f64>,
    /// UEL output maximum limit (VULMAX) (> UnderexcLimIEEE1.vulmin). Typical value = 18.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vulmax: Option<f64>,
    /// UEL output minimum limit (VULMIN) (< UnderexcLimIEEE1.vulmax). Typical value = -18.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vulmin: Option<f64>,
    /// UEL maximum limit for radius phasor magnitude (VURMAX). Typical value = 5,8.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vurmax: Option<f64>,
}
impl crate::base::CimElement for UnderexcLimIEEE1 {
    fn mrid(&self) -> &str { &self.base.base.base.id }
    fn type_name(&self) -> &'static str { "UnderexcLimIEEE1" }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn to_json_value(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or(serde_json::Value::Null)
    }
    fn to_block(&self) -> crate::base::RdfBlock {
        let mut block = self.base.to_block();
        block.type_name = "UnderexcLimIEEE1".to_string();
        if let Some(v) = self.kuc {
            block.fields.insert("UnderexcLimIEEE1.kuc".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.kuf {
            block.fields.insert("UnderexcLimIEEE1.kuf".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.kui {
            block.fields.insert("UnderexcLimIEEE1.kui".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.kul {
            block.fields.insert("UnderexcLimIEEE1.kul".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.kur {
            block.fields.insert("UnderexcLimIEEE1.kur".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.tu1 {
            block.fields.insert("UnderexcLimIEEE1.tu1".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.tu2 {
            block.fields.insert("UnderexcLimIEEE1.tu2".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.tu3 {
            block.fields.insert("UnderexcLimIEEE1.tu3".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.tu4 {
            block.fields.insert("UnderexcLimIEEE1.tu4".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.vucmax {
            block.fields.insert("UnderexcLimIEEE1.vucmax".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.vuimax {
            block.fields.insert("UnderexcLimIEEE1.vuimax".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.vuimin {
            block.fields.insert("UnderexcLimIEEE1.vuimin".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.vulmax {
            block.fields.insert("UnderexcLimIEEE1.vulmax".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.vulmin {
            block.fields.insert("UnderexcLimIEEE1.vulmin".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.vurmax {
            block.fields.insert("UnderexcLimIEEE1.vurmax".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        block
    }
}

impl UnderexcLimIEEE1 {
    pub fn from_block(b: &crate::base::RdfBlock) -> Self {
        let mut obj = Self::default();
        obj.base.base.base.id.clone_from(&b.mrid);
        for (key, val) in &b.fields {
            match key.as_str() {
                "UnderexcLimIEEE1.kuc" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.kuc = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.kuc = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "UnderexcLimIEEE1.kuf" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.kuf = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.kuf = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "UnderexcLimIEEE1.kui" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.kui = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.kui = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "UnderexcLimIEEE1.kul" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.kul = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.kul = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "UnderexcLimIEEE1.kur" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.kur = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.kur = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "UnderexcLimIEEE1.tu1" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.tu1 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.tu1 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "UnderexcLimIEEE1.tu2" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.tu2 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.tu2 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "UnderexcLimIEEE1.tu3" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.tu3 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.tu3 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "UnderexcLimIEEE1.tu4" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.tu4 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.tu4 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "UnderexcLimIEEE1.vucmax" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.vucmax = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.vucmax = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "UnderexcLimIEEE1.vuimax" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.vuimax = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.vuimax = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "UnderexcLimIEEE1.vuimin" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.vuimin = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.vuimin = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "UnderexcLimIEEE1.vulmax" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.vulmax = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.vulmax = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "UnderexcLimIEEE1.vulmin" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.vulmin = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.vulmin = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "UnderexcLimIEEE1.vurmax" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.vurmax = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.vurmax = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "UnderexcitationLimiterDynamics.ExcitationSystemDynamics" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.base.excitation_system_dynamics = Some(crate::base::MridRef { mrid: sv.clone() });
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
