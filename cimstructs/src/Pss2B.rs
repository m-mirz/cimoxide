/// Modified IEEE PSS2B. Extra lead/lag (or rate) block added at end (up to 4 lead/lags total).
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct Pss2B {
    #[serde(flatten)]
    pub base: super::PowerSystemStabilizerDynamics,
    /// Numerator constant (a). Typical value = 1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub a: Option<f64>,
    /// Stabilizer gain (Ks1). Typical value = 12.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ks1: Option<f64>,
    /// Gain on signal #2 (Ks2). Typical value = 0,2.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ks2: Option<f64>,
    /// Gain on signal #2 input before ramp-tracking filter (Ks3). Typical value = 1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ks3: Option<f64>,
    /// Gain on signal #2 input after ramp-tracking filter (Ks4). Typical value = 1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ks4: Option<f64>,
    /// Denominator order of ramp tracking filter (m). Typical value = 5.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m: Option<i64>,
    /// Order of ramp tracking filter (n). Typical value = 1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub n: Option<i64>,
    /// Lead/lag time constant (T1) (>= 0). Typical value = 0,12.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub t1: Option<f64>,
    /// Lead/lag time constant (T10) (>= 0). Typical value = 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub t10: Option<f64>,
    /// Lead/lag time constant (T11) (>= 0). Typical value = 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub t11: Option<f64>,
    /// Lead/lag time constant (T2) (>= 0). Typical value = 0,02.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub t2: Option<f64>,
    /// Lead/lag time constant (T3) (>= 0). Typical value = 0,3.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub t3: Option<f64>,
    /// Lead/lag time constant (T4) (>= 0). Typical value = 0,02.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub t4: Option<f64>,
    /// Time constant on signal #1 (T6) (>= 0). Typical value = 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub t6: Option<f64>,
    /// Time constant on signal #2 (T7) (>= 0). Typical value = 2.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub t7: Option<f64>,
    /// Lead of ramp tracking filter (T8) (>= 0). Typical value = 0,2.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub t8: Option<f64>,
    /// Lag of ramp tracking filter (T9) (>= 0). Typical value = 0,1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub t9: Option<f64>,
    /// Lead constant (Ta) (>= 0). Typical value = 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ta: Option<f64>,
    /// Lag time constant (Tb) (>= 0). Typical value = 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tb: Option<f64>,
    /// First washout on signal #1 (Tw1) (>= 0). Typical value = 2.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tw1: Option<f64>,
    /// Second washout on signal #1 (Tw2) (>= 0). Typical value = 2.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tw2: Option<f64>,
    /// First washout on signal #2 (Tw3) (>= 0). Typical value = 2.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tw3: Option<f64>,
    /// Second washout on signal #2 (Tw4) (>= 0). Typical value = 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tw4: Option<f64>,
    /// Input signal #1 maximum limit (Vsi1max) (> Pss2B.vsi1min). Typical value = 2.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vsi1max: Option<f64>,
    /// Input signal #1 minimum limit (Vsi1min) (< Pss2B.vsi1max). Typical value = -2.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vsi1min: Option<f64>,
    /// Input signal #2 maximum limit (Vsi2max) (> Pss2B.vsi2min). Typical value = 2.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vsi2max: Option<f64>,
    /// Input signal #2 minimum limit (Vsi2min) (< Pss2B.vsi2max). Typical value = -2.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vsi2min: Option<f64>,
    /// Stabilizer output maximum limit (Vstmax) (> Pss2B.vstmin). Typical value = 0,1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vstmax: Option<f64>,
    /// Stabilizer output minimum limit (Vstmin) (< Pss2B.vstmax). Typical value = -0,1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vstmin: Option<f64>,
}
impl crate::base::CimElement for Pss2B {
    fn mrid(&self) -> &str { &self.base.base.base.id }
    fn type_name(&self) -> &'static str { "Pss2B" }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn to_json_value(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or(serde_json::Value::Null)
    }
    fn to_block(&self) -> crate::base::RdfBlock {
        let mut block = self.base.to_block();
        block.type_name = "Pss2B".to_string();
        if let Some(v) = self.a {
            block.fields.insert("Pss2B.a".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.ks1 {
            block.fields.insert("Pss2B.ks1".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.ks2 {
            block.fields.insert("Pss2B.ks2".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.ks3 {
            block.fields.insert("Pss2B.ks3".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.ks4 {
            block.fields.insert("Pss2B.ks4".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.m {
            block.fields.insert("Pss2B.m".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.n {
            block.fields.insert("Pss2B.n".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.t1 {
            block.fields.insert("Pss2B.t1".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.t10 {
            block.fields.insert("Pss2B.t10".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.t11 {
            block.fields.insert("Pss2B.t11".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.t2 {
            block.fields.insert("Pss2B.t2".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.t3 {
            block.fields.insert("Pss2B.t3".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.t4 {
            block.fields.insert("Pss2B.t4".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.t6 {
            block.fields.insert("Pss2B.t6".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.t7 {
            block.fields.insert("Pss2B.t7".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.t8 {
            block.fields.insert("Pss2B.t8".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.t9 {
            block.fields.insert("Pss2B.t9".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.ta {
            block.fields.insert("Pss2B.ta".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.tb {
            block.fields.insert("Pss2B.tb".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.tw1 {
            block.fields.insert("Pss2B.tw1".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.tw2 {
            block.fields.insert("Pss2B.tw2".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.tw3 {
            block.fields.insert("Pss2B.tw3".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.tw4 {
            block.fields.insert("Pss2B.tw4".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.vsi1max {
            block.fields.insert("Pss2B.vsi1max".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.vsi1min {
            block.fields.insert("Pss2B.vsi1min".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.vsi2max {
            block.fields.insert("Pss2B.vsi2max".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.vsi2min {
            block.fields.insert("Pss2B.vsi2min".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.vstmax {
            block.fields.insert("Pss2B.vstmax".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.vstmin {
            block.fields.insert("Pss2B.vstmin".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        block
    }
}

impl Pss2B {
    pub fn from_block(b: &crate::base::RdfBlock) -> Self {
        let mut obj = Self::default();
        obj.base.base.base.id.clone_from(&b.mrid);
        for (key, val) in &b.fields {
            match key.as_str() {
                "Pss2B.a" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.a = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.a = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "Pss2B.ks1" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.ks1 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.ks1 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "Pss2B.ks2" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.ks2 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.ks2 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "Pss2B.ks3" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.ks3 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.ks3 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "Pss2B.ks4" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.ks4 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.ks4 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "Pss2B.m" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.m = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.m = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "Pss2B.n" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.n = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.n = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "Pss2B.t1" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.t1 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.t1 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "Pss2B.t10" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.t10 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.t10 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "Pss2B.t11" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.t11 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.t11 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "Pss2B.t2" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.t2 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.t2 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "Pss2B.t3" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.t3 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.t3 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "Pss2B.t4" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.t4 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.t4 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "Pss2B.t6" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.t6 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.t6 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "Pss2B.t7" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.t7 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.t7 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "Pss2B.t8" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.t8 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.t8 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "Pss2B.t9" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.t9 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.t9 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "Pss2B.ta" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.ta = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.ta = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "Pss2B.tb" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.tb = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.tb = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "Pss2B.tw1" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.tw1 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.tw1 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "Pss2B.tw2" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.tw2 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.tw2 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "Pss2B.tw3" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.tw3 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.tw3 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "Pss2B.tw4" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.tw4 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.tw4 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "Pss2B.vsi1max" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.vsi1max = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.vsi1max = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "Pss2B.vsi1min" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.vsi1min = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.vsi1min = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "Pss2B.vsi2max" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.vsi2max = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.vsi2max = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "Pss2B.vsi2min" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.vsi2min = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.vsi2min = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "Pss2B.vstmax" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.vstmax = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.vstmax = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "Pss2B.vstmin" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.vstmin = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.vstmin = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "PowerSystemStabilizerDynamics.ExcitationSystemDynamics" => {
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
