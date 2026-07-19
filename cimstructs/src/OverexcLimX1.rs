/// Field voltage over excitation limiter.
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct OverexcLimX1 {
    #[serde(flatten)]
    pub base: super::OverexcitationLimiterDynamics,
    /// Low voltage point on the inverse time characteristic (EFD1). Typical value = 1,1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub efd1: Option<f64>,
    /// Mid voltage point on the inverse time characteristic (EFD2). Typical value = 1,2.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub efd2: Option<f64>,
    /// High voltage point on the inverse time characteristic (EFD3). Typical value = 1,5.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub efd3: Option<f64>,
    /// Desired field voltage (EFDDES). Typical value = 0,9.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub efddes: Option<f64>,
    /// Rated field voltage (EFDRATED). Typical value = 1,05.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub efdrated: Option<f64>,
    /// Gain (KMX). Typical value = 0,01.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kmx: Option<f64>,
    /// Time to trip the exciter at the low voltage point on the inverse time characteristic (TIME1) (>= 0). Typical value = 120.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub t1: Option<f64>,
    /// Time to trip the exciter at the mid voltage point on the inverse time characteristic (TIME2) (>= 0). Typical value = 40.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub t2: Option<f64>,
    /// Time to trip the exciter at the high voltage point on the inverse time characteristic (TIME3) (>= 0). Typical value = 15.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub t3: Option<f64>,
    /// Low voltage limit (VLOW) (> 0).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vlow: Option<f64>,
}
impl crate::base::CimElement for OverexcLimX1 {
    fn mrid(&self) -> &str { &self.base.base.base.id }
    fn type_name(&self) -> &'static str { "OverexcLimX1" }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn to_json_value(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or(serde_json::Value::Null)
    }
    fn to_block(&self) -> crate::base::RdfBlock {
        let mut block = self.base.to_block();
        block.type_name = "OverexcLimX1".to_string();
        if let Some(v) = self.efd1 {
            block.fields.insert("OverexcLimX1.efd1".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.efd2 {
            block.fields.insert("OverexcLimX1.efd2".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.efd3 {
            block.fields.insert("OverexcLimX1.efd3".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.efddes {
            block.fields.insert("OverexcLimX1.efddes".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.efdrated {
            block.fields.insert("OverexcLimX1.efdrated".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.kmx {
            block.fields.insert("OverexcLimX1.kmx".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.t1 {
            block.fields.insert("OverexcLimX1.t1".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.t2 {
            block.fields.insert("OverexcLimX1.t2".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.t3 {
            block.fields.insert("OverexcLimX1.t3".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.vlow {
            block.fields.insert("OverexcLimX1.vlow".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        block
    }
}

impl OverexcLimX1 {
    pub fn from_block(b: &crate::base::RdfBlock) -> Self {
        let mut obj = Self::default();
        obj.base.base.base.id.clone_from(&b.mrid);
        for (key, val) in &b.fields {
            match key.as_str() {
                "OverexcLimX1.efd1" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.efd1 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.efd1 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "OverexcLimX1.efd2" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.efd2 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.efd2 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "OverexcLimX1.efd3" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.efd3 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.efd3 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "OverexcLimX1.efddes" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.efddes = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.efddes = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "OverexcLimX1.efdrated" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.efdrated = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.efdrated = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "OverexcLimX1.kmx" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.kmx = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.kmx = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "OverexcLimX1.t1" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.t1 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.t1 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "OverexcLimX1.t2" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.t2 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.t2 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "OverexcLimX1.t3" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.t3 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.t3 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "OverexcLimX1.vlow" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.vlow = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.vlow = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "OverexcitationLimiterDynamics.ExcitationSystemDynamics" => {
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
