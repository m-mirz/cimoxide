/// IEEE PF controller type 2 which is a summing point type controller making up the outside loop of a two-loop system. This controller is implemented as a slow PI type controller. The voltage regulator forms the inner loop and is implemented as a fast controller. Reference: IEEE 421.5-2005, 11.4.
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct PFVArType2IEEEPFController {
    #[serde(flatten)]
    pub base: super::PFVArControllerType2Dynamics,
    /// Overexcitation or under excitation flag (EXLON) true = 1 (not in the overexcitation or underexcitation state, integral action is active) false = 0 (in the overexcitation or underexcitation state, so integral action is disabled to allow the limiter to play its role).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exlon: Option<bool>,
    /// Integral gain of the pf controller (KI). Typical value = 1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ki: Option<f64>,
    /// Proportional gain of the pf controller (KP). Typical value = 1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kp: Option<f64>,
    /// Power factor reference (PFREF).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pfref: Option<f64>,
    /// Maximum output of the pf controller (VCLMT). Typical value = 0,1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vclmt: Option<f64>,
    /// Voltage regulator reference (VREF).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vref: Option<f64>,
    /// Generator sensing voltage (VS).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vs: Option<f64>,
}
impl crate::base::CimElement for PFVArType2IEEEPFController {
    fn mrid(&self) -> &str { &self.base.base.base.id }
    fn type_name(&self) -> &'static str { "PFVArType2IEEEPFController" }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn to_json_value(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or(serde_json::Value::Null)
    }
    fn to_block(&self) -> crate::base::RdfBlock {
        let mut block = self.base.to_block();
        block.type_name = "PFVArType2IEEEPFController".to_string();
        if let Some(v) = self.exlon {
            block.fields.insert("PFVArType2IEEEPFController.exlon".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.ki {
            block.fields.insert("PFVArType2IEEEPFController.ki".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.kp {
            block.fields.insert("PFVArType2IEEEPFController.kp".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.pfref {
            block.fields.insert("PFVArType2IEEEPFController.pfref".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.vclmt {
            block.fields.insert("PFVArType2IEEEPFController.vclmt".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.vref {
            block.fields.insert("PFVArType2IEEEPFController.vref".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.vs {
            block.fields.insert("PFVArType2IEEEPFController.vs".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        block
    }
}

impl PFVArType2IEEEPFController {
    pub fn from_block(b: &crate::base::RdfBlock) -> Self {
        let mut obj = Self::default();
        obj.base.base.base.id.clone_from(&b.mrid);
        for (key, val) in &b.fields {
            match key.as_str() {
                "PFVArType2IEEEPFController.exlon" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.exlon = Some(sv.trim() == "true"); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.exlon = Some(sv.trim() == "true"); }
                        }
                        _ => {}
                    }
                }
                "PFVArType2IEEEPFController.ki" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.ki = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.ki = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "PFVArType2IEEEPFController.kp" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.kp = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.kp = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "PFVArType2IEEEPFController.pfref" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.pfref = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.pfref = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "PFVArType2IEEEPFController.vclmt" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.vclmt = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.vclmt = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "PFVArType2IEEEPFController.vref" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.vref = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.vref = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "PFVArType2IEEEPFController.vs" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.vs = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.vs = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "PFVArControllerType2Dynamics.ExcitationSystemDynamics" => {
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
