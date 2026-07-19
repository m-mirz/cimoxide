/// Simplified type UEL2 underexcitation limiter. This model can be derived from UnderexcLimIEEE2. The limit characteristic (look -up table) is a single straight-line, the same as UnderexcLimIEEE2 (see Figure 10.4 (p 32), IEEE 421.5-2005 Section 10.2).
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct UnderexcLim2Simplified {
    #[serde(flatten)]
    pub base: super::UnderexcitationLimiterDynamics,
    /// Gain Under excitation limiter (KUI). Typical value = 0,1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kui: Option<f64>,
    /// Segment P initial point (P0). Typical value = 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub p0: Option<f64>,
    /// Segment P end point (P1). Typical value = 1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub p1: Option<f64>,
    /// Segment Q initial point (Q0). Typical value = -0,31.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub q0: Option<f64>,
    /// Segment Q end point (Q1). Typical value = -0,1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub q1: Option<f64>,
    /// Maximum error signal (VUIMAX) (> UnderexcLim2Simplified.vuimin). Typical value = 1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vuimax: Option<f64>,
    /// Minimum error signal (VUIMIN) (< UnderexcLim2Simplified.vuimax). Typical value = 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vuimin: Option<f64>,
}
impl crate::base::CimElement for UnderexcLim2Simplified {
    fn mrid(&self) -> &str { &self.base.base.base.id }
    fn type_name(&self) -> &'static str { "UnderexcLim2Simplified" }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn to_json_value(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or(serde_json::Value::Null)
    }
    fn to_block(&self) -> crate::base::RdfBlock {
        let mut block = self.base.to_block();
        block.type_name = "UnderexcLim2Simplified".to_string();
        if let Some(v) = self.kui {
            block.fields.insert("UnderexcLim2Simplified.kui".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.p0 {
            block.fields.insert("UnderexcLim2Simplified.p0".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.p1 {
            block.fields.insert("UnderexcLim2Simplified.p1".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.q0 {
            block.fields.insert("UnderexcLim2Simplified.q0".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.q1 {
            block.fields.insert("UnderexcLim2Simplified.q1".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.vuimax {
            block.fields.insert("UnderexcLim2Simplified.vuimax".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.vuimin {
            block.fields.insert("UnderexcLim2Simplified.vuimin".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        block
    }
}

impl UnderexcLim2Simplified {
    pub fn from_block(b: &crate::base::RdfBlock) -> Self {
        let mut obj = Self::default();
        obj.base.base.base.id.clone_from(&b.mrid);
        for (key, val) in &b.fields {
            match key.as_str() {
                "UnderexcLim2Simplified.kui" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.kui = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.kui = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "UnderexcLim2Simplified.p0" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.p0 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.p0 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "UnderexcLim2Simplified.p1" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.p1 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.p1 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "UnderexcLim2Simplified.q0" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.q0 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.q0 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "UnderexcLim2Simplified.q1" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.q1 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.q1 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "UnderexcLim2Simplified.vuimax" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.vuimax = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.vuimax = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "UnderexcLim2Simplified.vuimin" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.vuimin = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.vuimin = Some(v); } }
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
