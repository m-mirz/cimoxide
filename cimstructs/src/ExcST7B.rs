/// Modified IEEE ST7B static excitation system without stator current limiter (SCL) and current compensator (DROOP) inputs.
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct ExcST7B {
    #[serde(flatten)]
    pub base: super::ExcitationSystemDynamics,
    /// High-value gate feedback gain (Kh) (>= 0). Typical value = 1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kh: Option<f64>,
    /// Voltage regulator integral gain (Kia) (>= 0). Typical value = 1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kia: Option<f64>,
    /// Low-value gate feedback gain (Kl) (>= 0). Typical value = 1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kl: Option<f64>,
    /// Voltage regulator proportional gain (Kpa) (> 0). Typical value = 40.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kpa: Option<f64>,
    /// OEL input selector (OELin). Typical value = noOELinput.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oelin: Option<super::base::UriRef>,
    /// Regulator lag time constant (Tb) (>= 0). Typical value = 1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tb: Option<f64>,
    /// Regulator lead time constant (Tc) (>= 0). Typical value = 1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tc: Option<f64>,
    /// Excitation control system stabilizer time constant (Tf) (>= 0). Typical value = 1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tf: Option<f64>,
    /// Feedback time constant of inner loop field voltage regulator (Tg) (>= 0). Typical value = 1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tg: Option<f64>,
    /// Feedback time constant (Tia) (>= 0). Typical value = 3.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tia: Option<f64>,
    /// Rectifier firing time constant (Ts) (>= 0). Typical value = 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ts: Option<f64>,
    /// UEL input selector (UELin). Typical value = noUELinput.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uelin: Option<super::base::UriRef>,
    /// Maximum voltage reference signal (Vmax) (> 0 and > ExcST7B.vmin)). Typical value = 1,1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vmax: Option<f64>,
    /// Minimum voltage reference signal (Vmin) (> 0 and < ExcST7B.vmax). Typical value = 0,9.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vmin: Option<f64>,
    /// Maximum voltage regulator output (Vrmax) (> 0). Typical value = 5.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vrmax: Option<f64>,
    /// Minimum voltage regulator output (Vrmin) (< 0). Typical value = -4,5.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vrmin: Option<f64>,
}
impl crate::base::CimElement for ExcST7B {
    fn mrid(&self) -> &str { &self.base.base.base.id }
    fn type_name(&self) -> &'static str { "ExcST7B" }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn to_json_value(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or(serde_json::Value::Null)
    }
    fn to_block(&self) -> crate::base::RdfBlock {
        let mut block = self.base.to_block();
        block.type_name = "ExcST7B".to_string();
        if let Some(v) = self.kh {
            block.fields.insert("ExcST7B.kh".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.kia {
            block.fields.insert("ExcST7B.kia".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.kl {
            block.fields.insert("ExcST7B.kl".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.kpa {
            block.fields.insert("ExcST7B.kpa".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(ref v) = self.oelin {
            block.fields.insert("ExcST7B.oelin".into(), crate::base::FieldValue::Resource(v.uri.clone()));
        }
        if let Some(v) = self.tb {
            block.fields.insert("ExcST7B.tb".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.tc {
            block.fields.insert("ExcST7B.tc".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.tf {
            block.fields.insert("ExcST7B.tf".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.tg {
            block.fields.insert("ExcST7B.tg".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.tia {
            block.fields.insert("ExcST7B.tia".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.ts {
            block.fields.insert("ExcST7B.ts".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(ref v) = self.uelin {
            block.fields.insert("ExcST7B.uelin".into(), crate::base::FieldValue::Resource(v.uri.clone()));
        }
        if let Some(v) = self.vmax {
            block.fields.insert("ExcST7B.vmax".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.vmin {
            block.fields.insert("ExcST7B.vmin".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.vrmax {
            block.fields.insert("ExcST7B.vrmax".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.vrmin {
            block.fields.insert("ExcST7B.vrmin".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        block
    }
}

impl ExcST7B {
    pub fn from_block(b: &crate::base::RdfBlock) -> Self {
        let mut obj = Self::default();
        obj.base.base.base.id.clone_from(&b.mrid);
        for (key, val) in &b.fields {
            match key.as_str() {
                "ExcST7B.kh" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.kh = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.kh = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcST7B.kia" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.kia = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.kia = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcST7B.kl" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.kl = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.kl = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcST7B.kpa" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.kpa = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.kpa = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcST7B.oelin" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.oelin = Some(crate::base::UriRef { uri: sv.clone() });
                    }
                }
                "ExcST7B.tb" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.tb = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.tb = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcST7B.tc" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.tc = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.tc = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcST7B.tf" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.tf = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.tf = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcST7B.tg" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.tg = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.tg = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcST7B.tia" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.tia = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.tia = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcST7B.ts" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.ts = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.ts = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcST7B.uelin" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.uelin = Some(crate::base::UriRef { uri: sv.clone() });
                    }
                }
                "ExcST7B.vmax" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.vmax = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.vmax = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcST7B.vmin" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.vmin = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.vmin = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcST7B.vrmax" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.vrmax = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.vrmax = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcST7B.vrmin" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.vrmin = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.vrmin = Some(v); } }
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
