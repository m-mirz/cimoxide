/// The over excitation limiter model is intended to represent the significant features of OELs necessary for some large-scale system studies. It is the result of a pragmatic approach to obtain a model that can be widely applied with attainable data from generator owners. An attempt to include all variations in the functionality of OELs and duplicate how they interact with the rest of the excitation systems would likely result in a level of application insufficient for the studies for which they are intended. Reference: IEEE OEL 421.5-2005, 9.
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct OverexcLimIEEE {
    #[serde(flatten)]
    pub base: super::OverexcitationLimiterDynamics,
    /// OEL pickup/drop-out hysteresis (HYST). Typical value = 0,03.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hyst: Option<f64>,
    /// OEL timed field current limit (IFDLIM). Typical value = 1,05.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ifdlim: Option<f64>,
    /// OEL instantaneous field current limit (IFDMAX). Typical value = 1,5.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ifdmax: Option<f64>,
    /// OEL timed field current limiter pickup level (ITFPU). Typical value = 1,05.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub itfpu: Option<f64>,
    /// OEL cooldown gain (KCD). Typical value = 1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kcd: Option<f64>,
    /// OEL ramped limit rate (KRAMP). Unit = PU / s. Typical value = 10.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kramp: Option<f64>,
}
impl crate::base::CimElement for OverexcLimIEEE {
    fn mrid(&self) -> &str { &self.base.base.base.id }
    fn type_name(&self) -> &'static str { "OverexcLimIEEE" }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn to_json_value(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or(serde_json::Value::Null)
    }
    fn to_block(&self) -> crate::base::RdfBlock {
        let mut block = self.base.to_block();
        block.type_name = "OverexcLimIEEE".to_string();
        if let Some(v) = self.hyst {
            block.fields.insert("OverexcLimIEEE.hyst".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.ifdlim {
            block.fields.insert("OverexcLimIEEE.ifdlim".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.ifdmax {
            block.fields.insert("OverexcLimIEEE.ifdmax".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.itfpu {
            block.fields.insert("OverexcLimIEEE.itfpu".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.kcd {
            block.fields.insert("OverexcLimIEEE.kcd".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.kramp {
            block.fields.insert("OverexcLimIEEE.kramp".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        block
    }
}

impl OverexcLimIEEE {
    pub fn from_block(b: &crate::base::RdfBlock) -> Self {
        let mut obj = Self::default();
        obj.base.base.base.id.clone_from(&b.mrid);
        for (key, val) in &b.fields {
            match key.as_str() {
                "OverexcLimIEEE.hyst" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.hyst = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.hyst = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "OverexcLimIEEE.ifdlim" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.ifdlim = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.ifdlim = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "OverexcLimIEEE.ifdmax" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.ifdmax = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.ifdmax = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "OverexcLimIEEE.itfpu" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.itfpu = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.itfpu = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "OverexcLimIEEE.kcd" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.kcd = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.kcd = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "OverexcLimIEEE.kramp" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.kramp = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.kramp = Some(v); } }
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
