/// Modified IEEE ST6B static excitation system with PID controller and optional inner feedback loop.
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct ExcST6B {
    #[serde(flatten)]
    pub base: super::ExcitationSystemDynamics,
    /// Exciter output current limit reference (Ilr) (> 0). Typical value = 4,164.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ilr: Option<f64>,
    /// Selector (K1). true = feedback is from Ifd false = feedback is not from Ifd. Typical value = true.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub k1: Option<bool>,
    /// Exciter output current limit adjustment (Kcl) (> 0). Typical value = 1,0577.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kcl: Option<f64>,
    /// Pre-control gain constant of the inner loop field regulator (Kff). Typical value = 1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kff: Option<f64>,
    /// Feedback gain constant of the inner loop field regulator (Kg) (>= 0). Typical value = 1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kg: Option<f64>,
    /// Voltage regulator integral gain (Kia) (> 0). Typical value = 45,094.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kia: Option<f64>,
    /// Exciter output current limit adjustment (Kcl) (> 0). Typical value = 17,33.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub klr: Option<f64>,
    /// Forward gain constant of the inner loop field regulator (Km). Typical value = 1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub km: Option<f64>,
    /// Voltage regulator proportional gain (Kpa) (> 0). Typical value = 18,038.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kpa: Option<f64>,
    /// Voltage regulator derivative gain (Kvd). Typical value = 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kvd: Option<f64>,
    /// OEL input selector (OELin). Typical value = noOELinput (corresponds to OELin = 0 on diagram).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oelin: Option<super::base::UriRef>,
    /// Feedback time constant of inner loop field voltage regulator (Tg) (>= 0). Typical value = 0,02.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tg: Option<f64>,
    /// Rectifier firing time constant (Ts) (>= 0). Typical value = 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ts: Option<f64>,
    /// Voltage regulator derivative gain (Tvd) (>= 0). Typical value = 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tvd: Option<f64>,
    /// Maximum voltage regulator output (Vamax) (> 0). Typical value = 4,81.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vamax: Option<f64>,
    /// Minimum voltage regulator output (Vamin) (< 0). Typical value = -3,85.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vamin: Option<f64>,
    /// Selector (Vilim). true = Vimin-Vimax limiter is active false = Vimin-Vimax limiter is not active. Typical value = true.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vilim: Option<bool>,
    /// Maximum voltage regulator input limit (Vimax) (> ExcST6B.vimin). Typical value = 10.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vimax: Option<f64>,
    /// Minimum voltage regulator input limit (Vimin) (< ExcST6B.vimax). Typical value = -10.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vimin: Option<f64>,
    /// Selector (vmult). true = multiply regulator output by terminal voltage false = do not multiply regulator output by terminal voltage. Typical value = true.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vmult: Option<bool>,
    /// Maximum voltage regulator output (Vrmax) (> 0). Typical value = 4,81.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vrmax: Option<f64>,
    /// Minimum voltage regulator output (Vrmin) (< 0). Typical value = -3,85.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vrmin: Option<f64>,
    /// Excitation source reactance (Xc). Typical value = 0,05.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub xc: Option<f64>,
}
impl crate::base::CimElement for ExcST6B {
    fn mrid(&self) -> &str { &self.base.base.base.id }
    fn type_name(&self) -> &'static str { "ExcST6B" }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn to_json_value(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or(serde_json::Value::Null)
    }
    fn to_block(&self) -> crate::base::RdfBlock {
        let mut block = self.base.to_block();
        block.type_name = "ExcST6B".to_string();
        if let Some(v) = self.ilr {
            block.fields.insert("ExcST6B.ilr".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.k1 {
            block.fields.insert("ExcST6B.k1".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.kcl {
            block.fields.insert("ExcST6B.kcl".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.kff {
            block.fields.insert("ExcST6B.kff".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.kg {
            block.fields.insert("ExcST6B.kg".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.kia {
            block.fields.insert("ExcST6B.kia".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.klr {
            block.fields.insert("ExcST6B.klr".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.km {
            block.fields.insert("ExcST6B.km".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.kpa {
            block.fields.insert("ExcST6B.kpa".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.kvd {
            block.fields.insert("ExcST6B.kvd".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(ref v) = self.oelin {
            block.fields.insert("ExcST6B.oelin".into(), crate::base::FieldValue::Resource(v.uri.clone()));
        }
        if let Some(v) = self.tg {
            block.fields.insert("ExcST6B.tg".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.ts {
            block.fields.insert("ExcST6B.ts".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.tvd {
            block.fields.insert("ExcST6B.tvd".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.vamax {
            block.fields.insert("ExcST6B.vamax".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.vamin {
            block.fields.insert("ExcST6B.vamin".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.vilim {
            block.fields.insert("ExcST6B.vilim".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.vimax {
            block.fields.insert("ExcST6B.vimax".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.vimin {
            block.fields.insert("ExcST6B.vimin".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.vmult {
            block.fields.insert("ExcST6B.vmult".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.vrmax {
            block.fields.insert("ExcST6B.vrmax".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.vrmin {
            block.fields.insert("ExcST6B.vrmin".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.xc {
            block.fields.insert("ExcST6B.xc".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        block
    }
}

impl ExcST6B {
    pub fn from_block(b: &crate::base::RdfBlock) -> Self {
        let mut obj = Self::default();
        obj.base.base.base.id.clone_from(&b.mrid);
        for (key, val) in &b.fields {
            match key.as_str() {
                "ExcST6B.ilr" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.ilr = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.ilr = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcST6B.k1" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.k1 = Some(sv.trim() == "true"); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.k1 = Some(sv.trim() == "true"); }
                        }
                        _ => {}
                    }
                }
                "ExcST6B.kcl" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.kcl = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.kcl = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcST6B.kff" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.kff = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.kff = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcST6B.kg" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.kg = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.kg = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcST6B.kia" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.kia = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.kia = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcST6B.klr" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.klr = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.klr = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcST6B.km" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.km = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.km = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcST6B.kpa" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.kpa = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.kpa = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcST6B.kvd" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.kvd = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.kvd = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcST6B.oelin" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.oelin = Some(crate::base::UriRef { uri: sv.clone() });
                    }
                }
                "ExcST6B.tg" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.tg = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.tg = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcST6B.ts" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.ts = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.ts = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcST6B.tvd" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.tvd = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.tvd = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcST6B.vamax" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.vamax = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.vamax = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcST6B.vamin" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.vamin = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.vamin = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcST6B.vilim" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.vilim = Some(sv.trim() == "true"); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.vilim = Some(sv.trim() == "true"); }
                        }
                        _ => {}
                    }
                }
                "ExcST6B.vimax" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.vimax = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.vimax = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcST6B.vimin" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.vimin = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.vimin = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcST6B.vmult" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.vmult = Some(sv.trim() == "true"); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.vmult = Some(sv.trim() == "true"); }
                        }
                        _ => {}
                    }
                }
                "ExcST6B.vrmax" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.vrmax = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.vrmax = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcST6B.vrmin" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.vrmin = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.vrmin = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcST6B.xc" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.xc = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.xc = Some(v); } }
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
