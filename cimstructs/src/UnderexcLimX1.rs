/// Allis-Chalmers minimum excitation limiter.
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct UnderexcLimX1 {
    #[serde(flatten)]
    pub base: super::UnderexcitationLimiterDynamics,
    /// Minimum excitation limit slope (K) (> 0).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub k: Option<f64>,
    /// Differential gain (KF2).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kf2: Option<f64>,
    /// Minimum excitation limit gain (KM).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub km: Option<f64>,
    /// Minimum excitation limit value (MELMAX).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub melmax: Option<f64>,
    /// Differential time constant (TF2) (>= 0).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tf2: Option<f64>,
    /// Minimum excitation limit time constant (TM) (>= 0).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tm: Option<f64>,
}
impl crate::base::CimElement for UnderexcLimX1 {
    fn mrid(&self) -> &str { &self.base.base.base.id }
    fn type_name(&self) -> &'static str { "UnderexcLimX1" }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn to_json_value(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or(serde_json::Value::Null)
    }
    fn to_block(&self) -> crate::base::RdfBlock {
        let mut block = self.base.to_block();
        block.type_name = "UnderexcLimX1".to_string();
        if let Some(v) = self.k {
            block.fields.insert("UnderexcLimX1.k".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.kf2 {
            block.fields.insert("UnderexcLimX1.kf2".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.km {
            block.fields.insert("UnderexcLimX1.km".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.melmax {
            block.fields.insert("UnderexcLimX1.melmax".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.tf2 {
            block.fields.insert("UnderexcLimX1.tf2".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.tm {
            block.fields.insert("UnderexcLimX1.tm".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        block
    }
}

impl UnderexcLimX1 {
    pub fn from_block(b: &crate::base::RdfBlock) -> Self {
        let mut obj = Self::default();
        obj.base.base.base.id.clone_from(&b.mrid);
        for (key, val) in &b.fields {
            match key.as_str() {
                "UnderexcLimX1.k" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.k = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.k = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "UnderexcLimX1.kf2" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.kf2 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.kf2 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "UnderexcLimX1.km" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.km = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.km = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "UnderexcLimX1.melmax" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.melmax = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.melmax = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "UnderexcLimX1.tf2" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.tf2 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.tf2 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "UnderexcLimX1.tm" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.tm = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.tm = Some(v); } }
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
