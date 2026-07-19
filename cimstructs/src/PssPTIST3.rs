/// PTI microprocessor-based stabilizer type 3.
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct PssPTIST3 {
    #[serde(flatten)]
    pub base: super::PowerSystemStabilizerDynamics,
    /// Filter coefficient (A0).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub a0: Option<f64>,
    /// Limiter (Al).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub a1: Option<f64>,
    /// Filter coefficient (A2).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub a2: Option<f64>,
    /// Filter coefficient (A3).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub a3: Option<f64>,
    /// Filter coefficient (A4).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub a4: Option<f64>,
    /// Filter coefficient (A5).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub a5: Option<f64>,
    /// Limiter (Al).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub al: Option<f64>,
    /// Threshold value above which output averaging will be bypassed (Athres). Typical value = 0,005.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub athres: Option<f64>,
    /// Filter coefficient (B0).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub b0: Option<f64>,
    /// Filter coefficient (B1).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub b1: Option<f64>,
    /// Filter coefficient (B2).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub b2: Option<f64>,
    /// Filter coefficient (B3).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub b3: Option<f64>,
    /// Filter coefficient (B4).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub b4: Option<f64>,
    /// Filter coefficient (B5).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub b5: Option<f64>,
    /// Limiter (Dl).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dl: Option<f64>,
    /// Time step related to activation of controls (deltatc) (>= 0). Typical value = 0,025 (0,03 for 50 Hz).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dtc: Option<f64>,
    /// Time step frequency calculation (deltatf) (>= 0). Typical value = 0,025 (0,03 for 50 Hz).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dtf: Option<f64>,
    /// Time step active power calculation (deltatp) (>= 0). Typical value = 0,0125 (0,015 for 50 Hz).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dtp: Option<f64>,
    /// Digital/analogue output switch (Isw). true = produce analogue output false = convert to digital output, using tap selection table.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub isw: Option<bool>,
    /// Gain (K). Typical value = 9.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub k: Option<f64>,
    /// Threshold value (Lthres).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lthres: Option<f64>,
    /// (M). M = 2 x H. Typical value = 5.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m: Option<f64>,
    /// Number of control outputs to average (NAV) (1 <= NAV <= 16). Typical value = 4.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nav: Option<f64>,
    /// Number of counts at limit to active limit function (NCL) (> 0).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ncl: Option<f64>,
    /// Number of counts until reset after limit function is triggered (NCR).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ncr: Option<f64>,
    /// (Pmin).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pmin: Option<f64>,
    /// Time constant (T1) (>= 0). Typical value = 0,3.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub t1: Option<f64>,
    /// Time constant (T2) (>= 0). Typical value = 1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub t2: Option<f64>,
    /// Time constant (T3) (>= 0). Typical value = 0,2.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub t3: Option<f64>,
    /// Time constant (T4) (>= 0). Typical value = 0,05.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub t4: Option<f64>,
    /// Time constant (T5) (>= 0).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub t5: Option<f64>,
    /// Time constant (T6) (>= 0).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub t6: Option<f64>,
    /// Time constant (Tf) (>= 0). Typical value = 0,2.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tf: Option<f64>,
    /// Time constant (Tp) (>= 0). Typical value = 0,2.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tp: Option<f64>,
}
impl crate::base::CimElement for PssPTIST3 {
    fn mrid(&self) -> &str { &self.base.base.base.id }
    fn type_name(&self) -> &'static str { "PssPTIST3" }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn to_json_value(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or(serde_json::Value::Null)
    }
    fn to_block(&self) -> crate::base::RdfBlock {
        let mut block = self.base.to_block();
        block.type_name = "PssPTIST3".to_string();
        if let Some(v) = self.a0 {
            block.fields.insert("PssPTIST3.a0".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.a1 {
            block.fields.insert("PssPTIST3.a1".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.a2 {
            block.fields.insert("PssPTIST3.a2".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.a3 {
            block.fields.insert("PssPTIST3.a3".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.a4 {
            block.fields.insert("PssPTIST3.a4".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.a5 {
            block.fields.insert("PssPTIST3.a5".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.al {
            block.fields.insert("PssPTIST3.al".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.athres {
            block.fields.insert("PssPTIST3.athres".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.b0 {
            block.fields.insert("PssPTIST3.b0".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.b1 {
            block.fields.insert("PssPTIST3.b1".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.b2 {
            block.fields.insert("PssPTIST3.b2".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.b3 {
            block.fields.insert("PssPTIST3.b3".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.b4 {
            block.fields.insert("PssPTIST3.b4".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.b5 {
            block.fields.insert("PssPTIST3.b5".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.dl {
            block.fields.insert("PssPTIST3.dl".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.dtc {
            block.fields.insert("PssPTIST3.dtc".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.dtf {
            block.fields.insert("PssPTIST3.dtf".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.dtp {
            block.fields.insert("PssPTIST3.dtp".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.isw {
            block.fields.insert("PssPTIST3.isw".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.k {
            block.fields.insert("PssPTIST3.k".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.lthres {
            block.fields.insert("PssPTIST3.lthres".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.m {
            block.fields.insert("PssPTIST3.m".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.nav {
            block.fields.insert("PssPTIST3.nav".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.ncl {
            block.fields.insert("PssPTIST3.ncl".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.ncr {
            block.fields.insert("PssPTIST3.ncr".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.pmin {
            block.fields.insert("PssPTIST3.pmin".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.t1 {
            block.fields.insert("PssPTIST3.t1".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.t2 {
            block.fields.insert("PssPTIST3.t2".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.t3 {
            block.fields.insert("PssPTIST3.t3".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.t4 {
            block.fields.insert("PssPTIST3.t4".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.t5 {
            block.fields.insert("PssPTIST3.t5".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.t6 {
            block.fields.insert("PssPTIST3.t6".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.tf {
            block.fields.insert("PssPTIST3.tf".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.tp {
            block.fields.insert("PssPTIST3.tp".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        block
    }
}

impl PssPTIST3 {
    pub fn from_block(b: &crate::base::RdfBlock) -> Self {
        let mut obj = Self::default();
        obj.base.base.base.id.clone_from(&b.mrid);
        for (key, val) in &b.fields {
            match key.as_str() {
                "PssPTIST3.a0" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.a0 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.a0 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "PssPTIST3.a1" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.a1 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.a1 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "PssPTIST3.a2" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.a2 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.a2 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "PssPTIST3.a3" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.a3 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.a3 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "PssPTIST3.a4" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.a4 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.a4 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "PssPTIST3.a5" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.a5 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.a5 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "PssPTIST3.al" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.al = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.al = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "PssPTIST3.athres" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.athres = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.athres = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "PssPTIST3.b0" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.b0 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.b0 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "PssPTIST3.b1" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.b1 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.b1 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "PssPTIST3.b2" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.b2 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.b2 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "PssPTIST3.b3" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.b3 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.b3 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "PssPTIST3.b4" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.b4 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.b4 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "PssPTIST3.b5" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.b5 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.b5 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "PssPTIST3.dl" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.dl = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.dl = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "PssPTIST3.dtc" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.dtc = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.dtc = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "PssPTIST3.dtf" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.dtf = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.dtf = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "PssPTIST3.dtp" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.dtp = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.dtp = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "PssPTIST3.isw" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.isw = Some(sv.trim() == "true"); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.isw = Some(sv.trim() == "true"); }
                        }
                        _ => {}
                    }
                }
                "PssPTIST3.k" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.k = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.k = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "PssPTIST3.lthres" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.lthres = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.lthres = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "PssPTIST3.m" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.m = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.m = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "PssPTIST3.nav" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.nav = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.nav = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "PssPTIST3.ncl" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.ncl = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.ncl = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "PssPTIST3.ncr" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.ncr = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.ncr = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "PssPTIST3.pmin" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.pmin = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.pmin = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "PssPTIST3.t1" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.t1 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.t1 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "PssPTIST3.t2" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.t2 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.t2 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "PssPTIST3.t3" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.t3 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.t3 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "PssPTIST3.t4" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.t4 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.t4 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "PssPTIST3.t5" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.t5 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.t5 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "PssPTIST3.t6" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.t6 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.t6 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "PssPTIST3.tf" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.tf = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.tf = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "PssPTIST3.tp" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.tp = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.tp = Some(v); } }
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
