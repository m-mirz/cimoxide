/// Gas turbine.
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct GovGAST2 {
    #[serde(flatten)]
    pub base: super::TurbineGovernorDynamics,
    /// Valve positioner (A).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub a: Option<f64>,
    /// Exhaust temperature parameter (Af1). Unit = PU temperature. Based on temperature in degrees C.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub af1: Option<f64>,
    /// Coefficient equal to 0,5(1-speed) (Af2).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub af2: Option<f64>,
    /// Valve positioner (B).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub b: Option<f64>,
    /// (Bf1). Bf1 = E(1 - W) where E (speed sensitivity coefficient) is 0,55 to 0,65 x Tr. Unit = PU temperature. Based on temperature in degrees C.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bf1: Option<f64>,
    /// Turbine torque coefficient Khhv (depends on heating value of fuel stream in combustion chamber) (Bf2).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bf2: Option<f64>,
    /// Valve positioner (C).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub c: Option<f64>,
    /// Coefficient defining fuel flow where power output is 0% (Cf2). Synchronous but no output. Typically 0,23 x Khhv (23% fuel flow).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cf2: Option<f64>,
    /// Combustion reaction time delay (Ecr) (>= 0).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ecr: Option<f64>,
    /// Turbine and exhaust delay (Etd) (>= 0).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub etd: Option<f64>,
    /// Ratio of fuel adjustment (K3).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub k3: Option<f64>,
    /// Gain of radiation shield (K4).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub k4: Option<f64>,
    /// Gain of radiation shield (K5).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub k5: Option<f64>,
    /// Minimum fuel flow (K6).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub k6: Option<f64>,
    /// Fuel system feedback (Kf).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kf: Option<f64>,
    /// Base for power values (MWbase) (> 0). Unit = MW.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mwbase: Option<f64>,
    /// Fuel control time constant (T) (>= 0).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub t: Option<f64>,
    /// Radiation shield time constant (T3) (>= 0).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub t3: Option<f64>,
    /// Thermocouple time constant (T4) (>= 0).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub t4: Option<f64>,
    /// Temperature control time constant (T5) (>= 0).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub t5: Option<f64>,
    /// Temperature control (Tc). Unit = °F or °C depending on parameters Af1 and Bf1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tc: Option<f64>,
    /// Compressor discharge time constant (Tcd) (>= 0).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tcd: Option<f64>,
    /// Fuel system time constant (Tf) (>= 0).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tf: Option<f64>,
    /// Maximum turbine limit (Tmax) (> GovGAST2.tmin).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tmax: Option<f64>,
    /// Minimum turbine limit (Tmin) (< GovGAST2.tmax).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tmin: Option<f64>,
    /// Rated temperature (Tr). Unit = °C depending on parameters Af1 and Bf1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tr: Option<f64>,
    /// Turbine rating (Trate). Unit = MW.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trate: Option<f64>,
    /// Temperature controller integration rate (Tt) (>= 0).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tt: Option<f64>,
    /// Governor gain (1/droop) on turbine rating (W).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub w: Option<f64>,
    /// Governor lead time constant (X) (>= 0).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x: Option<f64>,
    /// Governor lag time constant (Y) (> 0).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub y: Option<f64>,
    /// Governor mode (Z). 1 = droop 0 = isochronous.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub z: Option<i64>,
}
impl crate::base::CimElement for GovGAST2 {
    fn mrid(&self) -> &str { &self.base.base.base.id }
    fn type_name(&self) -> &'static str { "GovGAST2" }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn to_json_value(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or(serde_json::Value::Null)
    }
    fn to_block(&self) -> crate::base::RdfBlock {
        let mut block = self.base.to_block();
        block.type_name = "GovGAST2".to_string();
        if let Some(v) = self.a {
            block.fields.insert("GovGAST2.a".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.af1 {
            block.fields.insert("GovGAST2.af1".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.af2 {
            block.fields.insert("GovGAST2.af2".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.b {
            block.fields.insert("GovGAST2.b".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.bf1 {
            block.fields.insert("GovGAST2.bf1".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.bf2 {
            block.fields.insert("GovGAST2.bf2".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.c {
            block.fields.insert("GovGAST2.c".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.cf2 {
            block.fields.insert("GovGAST2.cf2".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.ecr {
            block.fields.insert("GovGAST2.ecr".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.etd {
            block.fields.insert("GovGAST2.etd".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.k3 {
            block.fields.insert("GovGAST2.k3".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.k4 {
            block.fields.insert("GovGAST2.k4".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.k5 {
            block.fields.insert("GovGAST2.k5".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.k6 {
            block.fields.insert("GovGAST2.k6".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.kf {
            block.fields.insert("GovGAST2.kf".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.mwbase {
            block.fields.insert("GovGAST2.mwbase".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.t {
            block.fields.insert("GovGAST2.t".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.t3 {
            block.fields.insert("GovGAST2.t3".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.t4 {
            block.fields.insert("GovGAST2.t4".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.t5 {
            block.fields.insert("GovGAST2.t5".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.tc {
            block.fields.insert("GovGAST2.tc".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.tcd {
            block.fields.insert("GovGAST2.tcd".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.tf {
            block.fields.insert("GovGAST2.tf".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.tmax {
            block.fields.insert("GovGAST2.tmax".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.tmin {
            block.fields.insert("GovGAST2.tmin".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.tr {
            block.fields.insert("GovGAST2.tr".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.trate {
            block.fields.insert("GovGAST2.trate".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.tt {
            block.fields.insert("GovGAST2.tt".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.w {
            block.fields.insert("GovGAST2.w".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.x {
            block.fields.insert("GovGAST2.x".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.y {
            block.fields.insert("GovGAST2.y".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.z {
            block.fields.insert("GovGAST2.z".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        block
    }
}

impl GovGAST2 {
    pub fn from_block(b: &crate::base::RdfBlock) -> Self {
        let mut obj = Self::default();
        obj.base.base.base.id.clone_from(&b.mrid);
        for (key, val) in &b.fields {
            match key.as_str() {
                "GovGAST2.a" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.a = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.a = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovGAST2.af1" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.af1 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.af1 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovGAST2.af2" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.af2 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.af2 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovGAST2.b" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.b = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.b = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovGAST2.bf1" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.bf1 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.bf1 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovGAST2.bf2" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.bf2 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.bf2 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovGAST2.c" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.c = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.c = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovGAST2.cf2" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.cf2 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.cf2 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovGAST2.ecr" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.ecr = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.ecr = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovGAST2.etd" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.etd = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.etd = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovGAST2.k3" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.k3 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.k3 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovGAST2.k4" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.k4 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.k4 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovGAST2.k5" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.k5 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.k5 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovGAST2.k6" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.k6 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.k6 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovGAST2.kf" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.kf = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.kf = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovGAST2.mwbase" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.mwbase = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.mwbase = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovGAST2.t" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.t = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.t = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovGAST2.t3" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.t3 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.t3 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovGAST2.t4" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.t4 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.t4 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovGAST2.t5" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.t5 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.t5 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovGAST2.tc" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.tc = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.tc = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovGAST2.tcd" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.tcd = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.tcd = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovGAST2.tf" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.tf = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.tf = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovGAST2.tmax" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.tmax = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.tmax = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovGAST2.tmin" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.tmin = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.tmin = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovGAST2.tr" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.tr = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.tr = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovGAST2.trate" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.trate = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.trate = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovGAST2.tt" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.tt = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.tt = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovGAST2.w" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.w = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.w = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovGAST2.x" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.x = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.x = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovGAST2.y" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.y = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.y = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovGAST2.z" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.z = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.z = Some(v); } }
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
