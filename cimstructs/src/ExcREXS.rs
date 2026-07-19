/// General purpose rotating excitation system. This model can be used to represent a wide range of excitation systems whose DC power source is an AC or DC generator. It encompasses IEEE type AC1, AC2, DC1, and DC2 excitation system models.
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct ExcREXS {
    #[serde(flatten)]
    pub base: super::ExcitationSystemDynamics,
    /// Field voltage value 1 (E1). Typical value = 3.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub e1: Option<f64>,
    /// Field voltage value 2 (E2). Typical value = 4.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub e2: Option<f64>,
    /// Rate feedback signal flag (fbf). Typical value = fieldCurrent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fbf: Option<super::base::UriRef>,
    /// Limit type flag (Flimf). Typical value = 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flimf: Option<f64>,
    /// Rectifier regulation factor (Kc). Typical value = 0,05.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kc: Option<f64>,
    /// Exciter regulation factor (Kd). Typical value = 2.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kd: Option<f64>,
    /// Exciter field proportional constant (Ke). Typical value = 1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ke: Option<f64>,
    /// Field voltage feedback gain (Kefd). Typical value = 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kefd: Option<f64>,
    /// Rate feedback gain (Kf) (>= 0). Typical value = 0,05.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kf: Option<f64>,
    /// Field voltage controller feedback gain (Kh). Typical value = 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kh: Option<f64>,
    /// Field current regulator integral gain (Kii). Typical value = 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kii: Option<f64>,
    /// Field current regulator proportional gain (Kip). Typical value = 1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kip: Option<f64>,
    /// Coefficient to allow different usage of the model-speed coefficient (Ks). Typical value = 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ks: Option<f64>,
    /// Voltage regulator integral gain (Kvi). Typical value = 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kvi: Option<f64>,
    /// Voltage regulator proportional gain (Kvp). Typical value = 2800.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kvp: Option<f64>,
    /// V/Hz limiter gain (Kvphz). Typical value = 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kvphz: Option<f64>,
    /// Pickup speed of V/Hz limiter (Nvphz). Typical value = 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nvphz: Option<f64>,
    /// Saturation factor at E1 (Se1). Typical value = 0,0001.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub se1: Option<f64>,
    /// Saturation factor at E2 (Se2). Typical value = 0,001.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub se2: Option<f64>,
    /// Voltage regulator time constant (Ta) (>= 0). If = 0, block is bypassed. Typical value = 0,01.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ta: Option<f64>,
    /// Lag time constant (Tb1) (>= 0). If = 0, block is bypassed. Typical value = 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tb1: Option<f64>,
    /// Lag time constant (Tb2) (>= 0). If = 0, block is bypassed. Typical value = 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tb2: Option<f64>,
    /// Lead time constant (Tc1) (>= 0). Typical value = 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tc1: Option<f64>,
    /// Lead time constant (Tc2) (>= 0). Typical value = 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tc2: Option<f64>,
    /// Exciter field time constant (Te) (> 0). Typical value = 1,2.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub te: Option<f64>,
    /// Rate feedback time constant (Tf) (>= 0). If = 0, the feedback path is not used. Typical value = 1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tf: Option<f64>,
    /// Feedback lead time constant (Tf1) (>= 0). Typical value = 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tf1: Option<f64>,
    /// Feedback lag time constant (Tf2) (>= 0). If = 0, block is bypassed. Typical value = 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tf2: Option<f64>,
    /// Field current bridge time constant (Tp) (>= 0). Typical value = 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tp: Option<f64>,
    /// Maximum compounding voltage (Vcmax). Typical value = 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vcmax: Option<f64>,
    /// Maximum exciter field current (Vfmax) (> ExcREXS.vfmin). Typical value = 47.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vfmax: Option<f64>,
    /// Minimum exciter field current (Vfmin) (< ExcREXS.vfmax). Typical value = -20.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vfmin: Option<f64>,
    /// Voltage regulator input limit (Vimax). Typical value = 0,1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vimax: Option<f64>,
    /// Maximum controller output (Vrmax) (> ExcREXS.vrmin). Typical value = 47.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vrmax: Option<f64>,
    /// Minimum controller output (Vrmin) (< ExcREXS.vrmax). Typical value = -20.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vrmin: Option<f64>,
    /// Exciter compounding reactance (Xc). Typical value = 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub xc: Option<f64>,
}
impl crate::base::CimElement for ExcREXS {
    fn mrid(&self) -> &str { &self.base.base.base.id }
    fn type_name(&self) -> &'static str { "ExcREXS" }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn to_json_value(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or(serde_json::Value::Null)
    }
    fn to_block(&self) -> crate::base::RdfBlock {
        let mut block = self.base.to_block();
        block.type_name = "ExcREXS".to_string();
        if let Some(v) = self.e1 {
            block.fields.insert("ExcREXS.e1".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.e2 {
            block.fields.insert("ExcREXS.e2".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(ref v) = self.fbf {
            block.fields.insert("ExcREXS.fbf".into(), crate::base::FieldValue::Resource(v.uri.clone()));
        }
        if let Some(v) = self.flimf {
            block.fields.insert("ExcREXS.flimf".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.kc {
            block.fields.insert("ExcREXS.kc".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.kd {
            block.fields.insert("ExcREXS.kd".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.ke {
            block.fields.insert("ExcREXS.ke".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.kefd {
            block.fields.insert("ExcREXS.kefd".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.kf {
            block.fields.insert("ExcREXS.kf".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.kh {
            block.fields.insert("ExcREXS.kh".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.kii {
            block.fields.insert("ExcREXS.kii".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.kip {
            block.fields.insert("ExcREXS.kip".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.ks {
            block.fields.insert("ExcREXS.ks".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.kvi {
            block.fields.insert("ExcREXS.kvi".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.kvp {
            block.fields.insert("ExcREXS.kvp".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.kvphz {
            block.fields.insert("ExcREXS.kvphz".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.nvphz {
            block.fields.insert("ExcREXS.nvphz".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.se1 {
            block.fields.insert("ExcREXS.se1".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.se2 {
            block.fields.insert("ExcREXS.se2".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.ta {
            block.fields.insert("ExcREXS.ta".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.tb1 {
            block.fields.insert("ExcREXS.tb1".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.tb2 {
            block.fields.insert("ExcREXS.tb2".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.tc1 {
            block.fields.insert("ExcREXS.tc1".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.tc2 {
            block.fields.insert("ExcREXS.tc2".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.te {
            block.fields.insert("ExcREXS.te".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.tf {
            block.fields.insert("ExcREXS.tf".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.tf1 {
            block.fields.insert("ExcREXS.tf1".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.tf2 {
            block.fields.insert("ExcREXS.tf2".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.tp {
            block.fields.insert("ExcREXS.tp".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.vcmax {
            block.fields.insert("ExcREXS.vcmax".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.vfmax {
            block.fields.insert("ExcREXS.vfmax".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.vfmin {
            block.fields.insert("ExcREXS.vfmin".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.vimax {
            block.fields.insert("ExcREXS.vimax".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.vrmax {
            block.fields.insert("ExcREXS.vrmax".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.vrmin {
            block.fields.insert("ExcREXS.vrmin".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.xc {
            block.fields.insert("ExcREXS.xc".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        block
    }
}

impl ExcREXS {
    pub fn from_block(b: &crate::base::RdfBlock) -> Self {
        let mut obj = Self::default();
        obj.base.base.base.id.clone_from(&b.mrid);
        for (key, val) in &b.fields {
            match key.as_str() {
                "ExcREXS.e1" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.e1 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.e1 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcREXS.e2" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.e2 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.e2 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcREXS.fbf" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.fbf = Some(crate::base::UriRef { uri: sv.clone() });
                    }
                }
                "ExcREXS.flimf" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.flimf = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.flimf = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcREXS.kc" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.kc = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.kc = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcREXS.kd" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.kd = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.kd = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcREXS.ke" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.ke = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.ke = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcREXS.kefd" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.kefd = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.kefd = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcREXS.kf" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.kf = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.kf = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcREXS.kh" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.kh = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.kh = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcREXS.kii" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.kii = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.kii = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcREXS.kip" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.kip = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.kip = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcREXS.ks" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.ks = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.ks = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcREXS.kvi" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.kvi = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.kvi = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcREXS.kvp" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.kvp = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.kvp = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcREXS.kvphz" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.kvphz = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.kvphz = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcREXS.nvphz" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.nvphz = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.nvphz = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcREXS.se1" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.se1 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.se1 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcREXS.se2" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.se2 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.se2 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcREXS.ta" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.ta = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.ta = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcREXS.tb1" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.tb1 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.tb1 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcREXS.tb2" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.tb2 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.tb2 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcREXS.tc1" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.tc1 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.tc1 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcREXS.tc2" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.tc2 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.tc2 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcREXS.te" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.te = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.te = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcREXS.tf" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.tf = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.tf = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcREXS.tf1" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.tf1 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.tf1 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcREXS.tf2" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.tf2 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.tf2 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcREXS.tp" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.tp = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.tp = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcREXS.vcmax" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.vcmax = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.vcmax = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcREXS.vfmax" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.vfmax = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.vfmax = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcREXS.vfmin" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.vfmin = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.vfmin = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcREXS.vimax" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.vimax = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.vimax = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcREXS.vrmax" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.vrmax = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.vrmax = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcREXS.vrmin" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.vrmin = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.vrmin = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcREXS.xc" => {
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
