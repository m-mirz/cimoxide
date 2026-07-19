/// Type UEL2 underexcitation limiter which has either a straight-line or multi-segment characteristic when plotted in terms of machine reactive power output vs. real power output. Reference: IEEE UEL2 421.5-2005, 10.2 (limit characteristic lookup table shown in Figure 10.4 (p 32)).
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct UnderexcLimIEEE2 {
    #[serde(flatten)]
    pub base: super::UnderexcitationLimiterDynamics,
    /// UEL terminal voltage exponent applied to real power input to UEL limit look-up table (k1). Typical value = 2.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub k1: Option<f64>,
    /// UEL terminal voltage exponent applied to reactive power output from UEL limit look-up table (k2). Typical value = 2.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub k2: Option<f64>,
    /// Gain associated with optional integrator feedback input signal to UEL (KFB). Typical value = 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kfb: Option<f64>,
    /// UEL excitation system stabilizer gain (KUF). Typical value = 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kuf: Option<f64>,
    /// UEL integral gain (KUI). Typical value = 0,5.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kui: Option<f64>,
    /// UEL proportional gain (KUL). Typical value = 0,8.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kul: Option<f64>,
    /// Real power values for endpoints (P0). Typical value = 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub p0: Option<f64>,
    /// Real power values for endpoints (P1). Typical value = 0,3.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub p1: Option<f64>,
    /// Real power values for endpoints (P10).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub p10: Option<f64>,
    /// Real power values for endpoints (P2). Typical value = 0,6.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub p2: Option<f64>,
    /// Real power values for endpoints (P3). Typical value = 0,9.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub p3: Option<f64>,
    /// Real power values for endpoints (P4). Typical value = 1,02.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub p4: Option<f64>,
    /// Real power values for endpoints (P5).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub p5: Option<f64>,
    /// Real power values for endpoints (P6).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub p6: Option<f64>,
    /// Real power values for endpoints (P7).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub p7: Option<f64>,
    /// Real power values for endpoints (P8).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub p8: Option<f64>,
    /// Real power values for endpoints (P9).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub p9: Option<f64>,
    /// Reactive power values for endpoints (Q0). Typical value = -0,31.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub q0: Option<f64>,
    /// Reactive power values for endpoints (Q1). Typical value = -0,31.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub q1: Option<f64>,
    /// Reactive power values for endpoints (Q10).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub q10: Option<f64>,
    /// Reactive power values for endpoints (Q2). Typical value = -0,28.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub q2: Option<f64>,
    /// Reactive power values for endpoints (Q3). Typical value = -0,21.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub q3: Option<f64>,
    /// Reactive power values for endpoints (Q4). Typical value = 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub q4: Option<f64>,
    /// Reactive power values for endpoints (Q5).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub q5: Option<f64>,
    /// Reactive power values for endpoints (Q6).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub q6: Option<f64>,
    /// Reactive power values for endpoints (Q7).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub q7: Option<f64>,
    /// Reactive power values for endpoints (Q8).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub q8: Option<f64>,
    /// Reactive power values for endpoints (Q9).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub q9: Option<f64>,
    /// UEL lead time constant (TU1) (>= 0). Typical value = 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tu1: Option<f64>,
    /// UEL lag time constant (TU2) (>= 0). Typical value = 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tu2: Option<f64>,
    /// UEL lead time constant (TU3) (>= 0). Typical value = 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tu3: Option<f64>,
    /// UEL lag time constant (TU4) (>= 0). Typical value = 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tu4: Option<f64>,
    /// Time constant associated with optional integrator feedback input signal to UEL (TUL) (>= 0). Typical value = 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tul: Option<f64>,
    /// Real power filter time constant (TUP) (>= 0). Typical value = 5.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tup: Option<f64>,
    /// Reactive power filter time constant (TUQ) (>= 0). Typical value = 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tuq: Option<f64>,
    /// Voltage filter time constant (TUV) (>= 0). Typical value = 5.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tuv: Option<f64>,
    /// UEL integrator output maximum limit (VUIMAX) (> UnderexcLimIEEE2.vuimin). Typical value = 0,25.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vuimax: Option<f64>,
    /// UEL integrator output minimum limit (VUIMIN) (< UnderexcLimIEEE2.vuimax). Typical value = 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vuimin: Option<f64>,
    /// UEL output maximum limit (VULMAX) (> UnderexcLimIEEE2.vulmin). Typical value = 0,25.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vulmax: Option<f64>,
    /// UEL output minimum limit (VULMIN) (< UnderexcLimIEEE2.vulmax). Typical value = 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vulmin: Option<f64>,
}
impl crate::base::CimElement for UnderexcLimIEEE2 {
    fn mrid(&self) -> &str { &self.base.base.base.id }
    fn type_name(&self) -> &'static str { "UnderexcLimIEEE2" }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn to_json_value(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or(serde_json::Value::Null)
    }
    fn to_block(&self) -> crate::base::RdfBlock {
        let mut block = self.base.to_block();
        block.type_name = "UnderexcLimIEEE2".to_string();
        if let Some(v) = self.k1 {
            block.fields.insert("UnderexcLimIEEE2.k1".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.k2 {
            block.fields.insert("UnderexcLimIEEE2.k2".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.kfb {
            block.fields.insert("UnderexcLimIEEE2.kfb".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.kuf {
            block.fields.insert("UnderexcLimIEEE2.kuf".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.kui {
            block.fields.insert("UnderexcLimIEEE2.kui".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.kul {
            block.fields.insert("UnderexcLimIEEE2.kul".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.p0 {
            block.fields.insert("UnderexcLimIEEE2.p0".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.p1 {
            block.fields.insert("UnderexcLimIEEE2.p1".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.p10 {
            block.fields.insert("UnderexcLimIEEE2.p10".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.p2 {
            block.fields.insert("UnderexcLimIEEE2.p2".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.p3 {
            block.fields.insert("UnderexcLimIEEE2.p3".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.p4 {
            block.fields.insert("UnderexcLimIEEE2.p4".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.p5 {
            block.fields.insert("UnderexcLimIEEE2.p5".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.p6 {
            block.fields.insert("UnderexcLimIEEE2.p6".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.p7 {
            block.fields.insert("UnderexcLimIEEE2.p7".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.p8 {
            block.fields.insert("UnderexcLimIEEE2.p8".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.p9 {
            block.fields.insert("UnderexcLimIEEE2.p9".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.q0 {
            block.fields.insert("UnderexcLimIEEE2.q0".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.q1 {
            block.fields.insert("UnderexcLimIEEE2.q1".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.q10 {
            block.fields.insert("UnderexcLimIEEE2.q10".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.q2 {
            block.fields.insert("UnderexcLimIEEE2.q2".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.q3 {
            block.fields.insert("UnderexcLimIEEE2.q3".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.q4 {
            block.fields.insert("UnderexcLimIEEE2.q4".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.q5 {
            block.fields.insert("UnderexcLimIEEE2.q5".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.q6 {
            block.fields.insert("UnderexcLimIEEE2.q6".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.q7 {
            block.fields.insert("UnderexcLimIEEE2.q7".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.q8 {
            block.fields.insert("UnderexcLimIEEE2.q8".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.q9 {
            block.fields.insert("UnderexcLimIEEE2.q9".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.tu1 {
            block.fields.insert("UnderexcLimIEEE2.tu1".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.tu2 {
            block.fields.insert("UnderexcLimIEEE2.tu2".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.tu3 {
            block.fields.insert("UnderexcLimIEEE2.tu3".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.tu4 {
            block.fields.insert("UnderexcLimIEEE2.tu4".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.tul {
            block.fields.insert("UnderexcLimIEEE2.tul".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.tup {
            block.fields.insert("UnderexcLimIEEE2.tup".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.tuq {
            block.fields.insert("UnderexcLimIEEE2.tuq".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.tuv {
            block.fields.insert("UnderexcLimIEEE2.tuv".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.vuimax {
            block.fields.insert("UnderexcLimIEEE2.vuimax".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.vuimin {
            block.fields.insert("UnderexcLimIEEE2.vuimin".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.vulmax {
            block.fields.insert("UnderexcLimIEEE2.vulmax".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.vulmin {
            block.fields.insert("UnderexcLimIEEE2.vulmin".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        block
    }
}

impl UnderexcLimIEEE2 {
    pub fn from_block(b: &crate::base::RdfBlock) -> Self {
        let mut obj = Self::default();
        obj.base.base.base.id.clone_from(&b.mrid);
        for (key, val) in &b.fields {
            match key.as_str() {
                "UnderexcLimIEEE2.k1" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.k1 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.k1 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "UnderexcLimIEEE2.k2" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.k2 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.k2 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "UnderexcLimIEEE2.kfb" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.kfb = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.kfb = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "UnderexcLimIEEE2.kuf" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.kuf = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.kuf = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "UnderexcLimIEEE2.kui" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.kui = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.kui = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "UnderexcLimIEEE2.kul" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.kul = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.kul = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "UnderexcLimIEEE2.p0" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.p0 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.p0 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "UnderexcLimIEEE2.p1" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.p1 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.p1 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "UnderexcLimIEEE2.p10" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.p10 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.p10 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "UnderexcLimIEEE2.p2" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.p2 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.p2 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "UnderexcLimIEEE2.p3" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.p3 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.p3 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "UnderexcLimIEEE2.p4" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.p4 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.p4 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "UnderexcLimIEEE2.p5" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.p5 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.p5 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "UnderexcLimIEEE2.p6" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.p6 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.p6 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "UnderexcLimIEEE2.p7" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.p7 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.p7 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "UnderexcLimIEEE2.p8" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.p8 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.p8 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "UnderexcLimIEEE2.p9" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.p9 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.p9 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "UnderexcLimIEEE2.q0" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.q0 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.q0 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "UnderexcLimIEEE2.q1" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.q1 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.q1 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "UnderexcLimIEEE2.q10" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.q10 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.q10 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "UnderexcLimIEEE2.q2" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.q2 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.q2 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "UnderexcLimIEEE2.q3" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.q3 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.q3 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "UnderexcLimIEEE2.q4" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.q4 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.q4 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "UnderexcLimIEEE2.q5" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.q5 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.q5 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "UnderexcLimIEEE2.q6" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.q6 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.q6 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "UnderexcLimIEEE2.q7" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.q7 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.q7 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "UnderexcLimIEEE2.q8" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.q8 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.q8 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "UnderexcLimIEEE2.q9" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.q9 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.q9 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "UnderexcLimIEEE2.tu1" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.tu1 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.tu1 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "UnderexcLimIEEE2.tu2" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.tu2 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.tu2 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "UnderexcLimIEEE2.tu3" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.tu3 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.tu3 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "UnderexcLimIEEE2.tu4" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.tu4 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.tu4 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "UnderexcLimIEEE2.tul" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.tul = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.tul = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "UnderexcLimIEEE2.tup" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.tup = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.tup = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "UnderexcLimIEEE2.tuq" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.tuq = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.tuq = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "UnderexcLimIEEE2.tuv" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.tuv = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.tuv = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "UnderexcLimIEEE2.vuimax" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.vuimax = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.vuimax = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "UnderexcLimIEEE2.vuimin" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.vuimin = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.vuimin = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "UnderexcLimIEEE2.vulmax" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.vulmax = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.vulmax = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "UnderexcLimIEEE2.vulmin" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.vulmin = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.vulmin = Some(v); } }
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
