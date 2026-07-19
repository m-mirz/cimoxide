/// Different from LimIEEEOEL, LimOEL2 has a fixed pickup threshold and reduces the excitation set-point by means of a non-windup integral regulator. Irated is the rated machine excitation current (calculated from nameplate conditions: Vnom, Pnom, CosPhinom).
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct OverexcLim2 {
    #[serde(flatten)]
    pub base: super::OverexcitationLimiterDynamics,
    /// Limit value of rated field current (IFDLIM). Typical value = 1,05.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ifdlim: Option<f64>,
    /// Gain Over excitation limiter (KOI). Typical value = 0,1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub koi: Option<f64>,
    /// Maximum error signal (VOIMAX) (> OverexcLim2.voimin). Typical value = 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voimax: Option<f64>,
    /// Minimum error signal (VOIMIN) (< OverexcLim2.voimax). Typical value = -9999.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voimin: Option<f64>,
}
impl crate::base::CimElement for OverexcLim2 {
    fn mrid(&self) -> &str { &self.base.base.base.id }
    fn type_name(&self) -> &'static str { "OverexcLim2" }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn to_json_value(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or(serde_json::Value::Null)
    }
    fn to_block(&self) -> crate::base::RdfBlock {
        let mut block = self.base.to_block();
        block.type_name = "OverexcLim2".to_string();
        if let Some(v) = self.ifdlim {
            block.fields.insert("OverexcLim2.ifdlim".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.koi {
            block.fields.insert("OverexcLim2.koi".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.voimax {
            block.fields.insert("OverexcLim2.voimax".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.voimin {
            block.fields.insert("OverexcLim2.voimin".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        block
    }
}

impl OverexcLim2 {
    pub fn from_block(b: &crate::base::RdfBlock) -> Self {
        let mut obj = Self::default();
        obj.base.base.base.id.clone_from(&b.mrid);
        for (key, val) in &b.fields {
            match key.as_str() {
                "OverexcLim2.ifdlim" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.ifdlim = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.ifdlim = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "OverexcLim2.koi" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.koi = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.koi = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "OverexcLim2.voimax" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.voimax = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.voimax = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "OverexcLim2.voimin" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.voimin = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.voimin = Some(v); } }
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
