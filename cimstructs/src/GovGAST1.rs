/// Modified single shaft gas turbine.
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct GovGAST1 {
    #[serde(flatten)]
    pub base: super::TurbineGovernorDynamics,
    /// Turbine power time constant numerator scale factor (a). Typical value = 0,8.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub a: Option<f64>,
    /// Turbine power time constant denominator scale factor (b) (>0). Typical value = 1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub b: Option<f64>,
    /// Intentional dead-band width (db1). Unit = Hz. Typical value = 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db1: Option<f64>,
    /// Unintentional dead-band (db2). Unit = MW. Typical value = 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db2: Option<f64>,
    /// Intentional db hysteresis (eps). Unit = Hz. Typical value = 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eps: Option<f64>,
    /// Fuel flow at zero power output (Fidle). Typical value = 0,18.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fidle: Option<f64>,
    /// Nonlinear gain point 1, PU gv (Gv1). Typical value = 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gv1: Option<f64>,
    /// Nonlinear gain point 2,PU gv (Gv2). Typical value = 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gv2: Option<f64>,
    /// Nonlinear gain point 3, PU gv (Gv3). Typical value = 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gv3: Option<f64>,
    /// Nonlinear gain point 4, PU gv (Gv4). Typical value = 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gv4: Option<f64>,
    /// Nonlinear gain point 5, PU gv (Gv5). Typical value = 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gv5: Option<f64>,
    /// Nonlinear gain point 6, PU gv (Gv6). Typical value = 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gv6: Option<f64>,
    /// Governor gain (Ka). Typical value = 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ka: Option<f64>,
    /// Temperature limiter gain (Kt). Typical value = 3.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kt: Option<f64>,
    /// Ambient temperature load limit (Lmax). Lmax is the turbine power output corresponding to the limiting exhaust gas temperature. Typical value = 1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lmax: Option<f64>,
    /// Valve position change allowed at fast rate (Loadinc). Typical value = 0,05.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub loadinc: Option<f64>,
    /// Maximum long term fuel valve opening rate (Ltrate). Typical value = 0,02.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ltrate: Option<f64>,
    /// Base for power values (MWbase) (> 0). Unit = MW.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mwbase: Option<f64>,
    /// Nonlinear gain point 1, PU power (Pgv1). Typical value = 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pgv1: Option<f64>,
    /// Nonlinear gain point 2, PU power (Pgv2). Typical value = 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pgv2: Option<f64>,
    /// Nonlinear gain point 3, PU power (Pgv3). Typical value = 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pgv3: Option<f64>,
    /// Nonlinear gain point 4, PU power (Pgv4). Typical value = 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pgv4: Option<f64>,
    /// Nonlinear gain point 5, PU power (Pgv5). Typical value = 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pgv5: Option<f64>,
    /// Nonlinear gain point 6, PU power (Pgv6). Typical value = 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pgv6: Option<f64>,
    /// Permanent droop (R) (>0). Typical value = 0,04.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r: Option<f64>,
    /// Maximum fuel valve opening rate (Rmax). Unit = PU / s. Typical value = 1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rmax: Option<f64>,
    /// Governor mechanism time constant (T1) (>= 0). T1 represents the natural valve positioning time constant of the governor for small disturbances, as seen when rate limiting is not in effect. Typical value = 0,5.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub t1: Option<f64>,
    /// Turbine power time constant (T2) (>= 0). T2 represents delay due to internal energy storage of the gas turbine engine. T2 can be used to give a rough approximation to the delay associated with acceleration of the compressor spool of a multi-shaft engine, or with the compressibility of gas in the plenum of the free power turbine of an aero-derivative unit, for example. Typical value = 0,5.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub t2: Option<f64>,
    /// Turbine exhaust temperature time constant (T3) (>= 0). T3 represents delay in the exhaust temperature and load limiting system. Typical value = 3.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub t3: Option<f64>,
    /// Governor lead time constant (T4) (>= 0). Typical value = 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub t4: Option<f64>,
    /// Governor lag time constant (T5) (>= 0). If = 0, entire gain and lead-lag block is bypassed. Typical value = 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub t5: Option<f64>,
    /// Valve position averaging time constant (Tltr) (>= 0). Typical value = 10.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tltr: Option<f64>,
    /// Maximum turbine power, PU of MWbase (Vmax) (> GovGAST1.vmin). Typical value = 1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vmax: Option<f64>,
    /// Minimum turbine power, PU of MWbase (Vmin) (< GovGAST1.vmax). Typical value = 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vmin: Option<f64>,
}
impl crate::base::CimElement for GovGAST1 {
    fn mrid(&self) -> &str { &self.base.base.base.id }
    fn type_name(&self) -> &'static str { "GovGAST1" }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn to_json_value(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or(serde_json::Value::Null)
    }
    fn to_block(&self) -> crate::base::RdfBlock {
        let mut block = self.base.to_block();
        block.type_name = "GovGAST1".to_string();
        if let Some(v) = self.a {
            block.fields.insert("GovGAST1.a".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.b {
            block.fields.insert("GovGAST1.b".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.db1 {
            block.fields.insert("GovGAST1.db1".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.db2 {
            block.fields.insert("GovGAST1.db2".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.eps {
            block.fields.insert("GovGAST1.eps".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.fidle {
            block.fields.insert("GovGAST1.fidle".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.gv1 {
            block.fields.insert("GovGAST1.gv1".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.gv2 {
            block.fields.insert("GovGAST1.gv2".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.gv3 {
            block.fields.insert("GovGAST1.gv3".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.gv4 {
            block.fields.insert("GovGAST1.gv4".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.gv5 {
            block.fields.insert("GovGAST1.gv5".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.gv6 {
            block.fields.insert("GovGAST1.gv6".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.ka {
            block.fields.insert("GovGAST1.ka".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.kt {
            block.fields.insert("GovGAST1.kt".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.lmax {
            block.fields.insert("GovGAST1.lmax".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.loadinc {
            block.fields.insert("GovGAST1.loadinc".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.ltrate {
            block.fields.insert("GovGAST1.ltrate".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.mwbase {
            block.fields.insert("GovGAST1.mwbase".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.pgv1 {
            block.fields.insert("GovGAST1.pgv1".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.pgv2 {
            block.fields.insert("GovGAST1.pgv2".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.pgv3 {
            block.fields.insert("GovGAST1.pgv3".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.pgv4 {
            block.fields.insert("GovGAST1.pgv4".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.pgv5 {
            block.fields.insert("GovGAST1.pgv5".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.pgv6 {
            block.fields.insert("GovGAST1.pgv6".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.r {
            block.fields.insert("GovGAST1.r".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.rmax {
            block.fields.insert("GovGAST1.rmax".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.t1 {
            block.fields.insert("GovGAST1.t1".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.t2 {
            block.fields.insert("GovGAST1.t2".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.t3 {
            block.fields.insert("GovGAST1.t3".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.t4 {
            block.fields.insert("GovGAST1.t4".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.t5 {
            block.fields.insert("GovGAST1.t5".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.tltr {
            block.fields.insert("GovGAST1.tltr".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.vmax {
            block.fields.insert("GovGAST1.vmax".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.vmin {
            block.fields.insert("GovGAST1.vmin".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        block
    }
}

impl GovGAST1 {
    pub fn from_block(b: &crate::base::RdfBlock) -> Self {
        let mut obj = Self::default();
        obj.base.base.base.id.clone_from(&b.mrid);
        for (key, val) in &b.fields {
            match key.as_str() {
                "GovGAST1.a" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.a = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.a = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovGAST1.b" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.b = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.b = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovGAST1.db1" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.db1 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.db1 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovGAST1.db2" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.db2 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.db2 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovGAST1.eps" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.eps = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.eps = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovGAST1.fidle" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.fidle = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.fidle = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovGAST1.gv1" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.gv1 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.gv1 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovGAST1.gv2" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.gv2 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.gv2 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovGAST1.gv3" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.gv3 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.gv3 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovGAST1.gv4" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.gv4 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.gv4 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovGAST1.gv5" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.gv5 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.gv5 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovGAST1.gv6" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.gv6 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.gv6 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovGAST1.ka" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.ka = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.ka = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovGAST1.kt" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.kt = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.kt = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovGAST1.lmax" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.lmax = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.lmax = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovGAST1.loadinc" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.loadinc = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.loadinc = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovGAST1.ltrate" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.ltrate = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.ltrate = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovGAST1.mwbase" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.mwbase = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.mwbase = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovGAST1.pgv1" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.pgv1 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.pgv1 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovGAST1.pgv2" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.pgv2 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.pgv2 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovGAST1.pgv3" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.pgv3 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.pgv3 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovGAST1.pgv4" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.pgv4 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.pgv4 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovGAST1.pgv5" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.pgv5 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.pgv5 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovGAST1.pgv6" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.pgv6 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.pgv6 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovGAST1.r" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.r = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.r = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovGAST1.rmax" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.rmax = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.rmax = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovGAST1.t1" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.t1 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.t1 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovGAST1.t2" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.t2 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.t2 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovGAST1.t3" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.t3 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.t3 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovGAST1.t4" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.t4 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.t4 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovGAST1.t5" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.t5 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.t5 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovGAST1.tltr" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.tltr = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.tltr = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovGAST1.vmax" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.vmax = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.vmax = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovGAST1.vmin" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.vmin = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.vmin = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "TurbineGovernorDynamics.AsynchronousMachineDynamics" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.base.asynchronous_machine_dynamics = Some(crate::base::MridRef { mrid: sv.clone() });
                    }
                }
                "TurbineGovernorDynamics.SynchronousMachineDynamics" => {
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
