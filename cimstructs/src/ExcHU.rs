/// Hungarian excitation system, with built-in voltage transducer.
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct ExcHU {
    #[serde(flatten)]
    pub base: super::ExcitationSystemDynamics,
    /// Major loop PI tag gain factor (Ae). Typical value = 3.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ae: Option<f64>,
    /// Minor loop PI tag gain factor (Ai). Typical value = 22.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ai: Option<f64>,
    /// AVR constant (Atr). Typical value = 2,19.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub atr: Option<f64>,
    /// Field voltage control signal upper limit on AVR base (Emax) (> ExcHU.emin). Typical value = 0,996.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emax: Option<f64>,
    /// Field voltage control signal lower limit on AVR base (Emin) (< ExcHU.emax). Typical value = -0,866.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emin: Option<f64>,
    /// Major loop PI tag output signal upper limit (Imax) (> ExcHU.imin). Typical value = 2,19.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub imax: Option<f64>,
    /// Major loop PI tag output signal lower limit (Imin) (< ExcHU.imax). Typical value = 0,1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub imin: Option<f64>,
    /// Voltage base conversion constant (Ke). Typical value = 4,666.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ke: Option<f64>,
    /// Current base conversion constant (Ki). Typical value = 0,21428.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ki: Option<f64>,
    /// Major loop PI tag integration time constant (Te) (>= 0). Typical value = 0,154.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub te: Option<f64>,
    /// Minor loop PI control tag integration time constant (Ti) (>= 0). Typical value = 0,01333.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ti: Option<f64>,
    /// Filter time constant (Tr) (>= 0). If a voltage compensator is used in conjunction with this excitation system model, Tr should be set to 0. Typical value = 0,01.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tr: Option<f64>,
}
impl crate::base::CimElement for ExcHU {
    fn mrid(&self) -> &str { &self.base.base.base.id }
    fn type_name(&self) -> &'static str { "ExcHU" }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn to_json_value(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or(serde_json::Value::Null)
    }
    fn to_block(&self) -> crate::base::RdfBlock {
        let mut block = self.base.to_block();
        block.type_name = "ExcHU".to_string();
        if let Some(v) = self.ae {
            block.fields.insert("ExcHU.ae".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.ai {
            block.fields.insert("ExcHU.ai".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.atr {
            block.fields.insert("ExcHU.atr".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.emax {
            block.fields.insert("ExcHU.emax".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.emin {
            block.fields.insert("ExcHU.emin".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.imax {
            block.fields.insert("ExcHU.imax".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.imin {
            block.fields.insert("ExcHU.imin".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.ke {
            block.fields.insert("ExcHU.ke".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.ki {
            block.fields.insert("ExcHU.ki".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.te {
            block.fields.insert("ExcHU.te".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.ti {
            block.fields.insert("ExcHU.ti".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.tr {
            block.fields.insert("ExcHU.tr".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        block
    }
}

impl ExcHU {
    pub fn from_block(b: &crate::base::RdfBlock) -> Self {
        let mut obj = Self::default();
        obj.base.base.base.id.clone_from(&b.mrid);
        for (key, val) in &b.fields {
            match key.as_str() {
                "ExcHU.ae" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.ae = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.ae = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcHU.ai" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.ai = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.ai = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcHU.atr" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.atr = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.atr = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcHU.emax" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.emax = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.emax = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcHU.emin" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.emin = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.emin = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcHU.imax" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.imax = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.imax = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcHU.imin" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.imin = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.imin = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcHU.ke" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.ke = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.ke = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcHU.ki" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.ki = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.ki = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcHU.te" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.te = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.te = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcHU.ti" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.ti = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.ti = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcHU.tr" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.tr = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.tr = Some(v); } }
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
