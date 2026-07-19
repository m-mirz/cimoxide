/// Woodward™ gas turbine governor. [Footnote: Woodward gas turbines are an example of suitable products available commercially. This information is given for the convenience of users of this document and does not constitute an endorsement by IEC of these products.]
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct GovGASTWD {
    #[serde(flatten)]
    pub base: super::TurbineGovernorDynamics,
    /// Valve positioner (A).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub a: Option<f64>,
    /// Exhaust temperature parameter (Af1).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub af1: Option<f64>,
    /// Coefficient equal to 0,5(1-speed) (Af2).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub af2: Option<f64>,
    /// Valve positioner (B).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub b: Option<f64>,
    /// (Bf1). Bf1 = E(1-w) where E (speed sensitivity coefficient) is 0,55 to 0,65 x Tr.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bf1: Option<f64>,
    /// Turbine torque coefficient Khhv (depends on heating value of fuel stream in combustion chamber) (Bf2).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bf2: Option<f64>,
    /// Valve positioner (C).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub c: Option<f64>,
    /// Coefficient defining fuel flow where power output is 0 % (Cf2). Synchronous but no output. Typically 0,23 x Khhv (23 % fuel flow).
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
    /// Drop governor gain (Kd).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kd: Option<f64>,
    /// (Kdroop) (>= 0).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kdroop: Option<f64>,
    /// Fuel system feedback (Kf).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kf: Option<f64>,
    /// Isochronous Governor Gain (Ki).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ki: Option<f64>,
    /// PID proportional gain (Kp).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kp: Option<f64>,
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
    /// Temperature control (Tc).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tc: Option<f64>,
    /// Compressor discharge time constant (Tcd) (>= 0).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tcd: Option<f64>,
    /// Power transducer time constant (Td) (>= 0).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub td: Option<f64>,
    /// Fuel system time constant (Tf) (>= 0).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tf: Option<f64>,
    /// Maximum Turbine limit (Tmax) (> GovGASTWD.tmin).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tmax: Option<f64>,
    /// Minimum turbine limit (Tmin) (< GovGASTWD.tmax).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tmin: Option<f64>,
    /// Rated temperature (Tr).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tr: Option<f64>,
    /// Turbine rating (Trate). Unit = MW.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trate: Option<f64>,
    /// Temperature controller integration rate (Tt) (>= 0).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tt: Option<f64>,
}
impl crate::base::CimElement for GovGASTWD {
    fn mrid(&self) -> &str { &self.base.base.base.id }
    fn type_name(&self) -> &'static str { "GovGASTWD" }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn to_json_value(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or(serde_json::Value::Null)
    }
    fn to_block(&self) -> crate::base::RdfBlock {
        let mut block = self.base.to_block();
        block.type_name = "GovGASTWD".to_string();
        if let Some(v) = self.a {
            block.fields.insert("GovGASTWD.a".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.af1 {
            block.fields.insert("GovGASTWD.af1".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.af2 {
            block.fields.insert("GovGASTWD.af2".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.b {
            block.fields.insert("GovGASTWD.b".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.bf1 {
            block.fields.insert("GovGASTWD.bf1".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.bf2 {
            block.fields.insert("GovGASTWD.bf2".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.c {
            block.fields.insert("GovGASTWD.c".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.cf2 {
            block.fields.insert("GovGASTWD.cf2".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.ecr {
            block.fields.insert("GovGASTWD.ecr".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.etd {
            block.fields.insert("GovGASTWD.etd".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.k3 {
            block.fields.insert("GovGASTWD.k3".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.k4 {
            block.fields.insert("GovGASTWD.k4".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.k5 {
            block.fields.insert("GovGASTWD.k5".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.k6 {
            block.fields.insert("GovGASTWD.k6".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.kd {
            block.fields.insert("GovGASTWD.kd".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.kdroop {
            block.fields.insert("GovGASTWD.kdroop".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.kf {
            block.fields.insert("GovGASTWD.kf".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.ki {
            block.fields.insert("GovGASTWD.ki".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.kp {
            block.fields.insert("GovGASTWD.kp".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.mwbase {
            block.fields.insert("GovGASTWD.mwbase".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.t {
            block.fields.insert("GovGASTWD.t".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.t3 {
            block.fields.insert("GovGASTWD.t3".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.t4 {
            block.fields.insert("GovGASTWD.t4".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.t5 {
            block.fields.insert("GovGASTWD.t5".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.tc {
            block.fields.insert("GovGASTWD.tc".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.tcd {
            block.fields.insert("GovGASTWD.tcd".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.td {
            block.fields.insert("GovGASTWD.td".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.tf {
            block.fields.insert("GovGASTWD.tf".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.tmax {
            block.fields.insert("GovGASTWD.tmax".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.tmin {
            block.fields.insert("GovGASTWD.tmin".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.tr {
            block.fields.insert("GovGASTWD.tr".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.trate {
            block.fields.insert("GovGASTWD.trate".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.tt {
            block.fields.insert("GovGASTWD.tt".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        block
    }
}

impl GovGASTWD {
    pub fn from_block(b: &crate::base::RdfBlock) -> Self {
        let mut obj = Self::default();
        obj.base.base.base.id.clone_from(&b.mrid);
        for (key, val) in &b.fields {
            match key.as_str() {
                "GovGASTWD.a" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.a = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.a = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovGASTWD.af1" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.af1 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.af1 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovGASTWD.af2" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.af2 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.af2 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovGASTWD.b" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.b = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.b = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovGASTWD.bf1" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.bf1 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.bf1 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovGASTWD.bf2" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.bf2 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.bf2 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovGASTWD.c" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.c = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.c = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovGASTWD.cf2" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.cf2 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.cf2 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovGASTWD.ecr" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.ecr = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.ecr = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovGASTWD.etd" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.etd = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.etd = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovGASTWD.k3" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.k3 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.k3 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovGASTWD.k4" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.k4 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.k4 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovGASTWD.k5" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.k5 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.k5 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovGASTWD.k6" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.k6 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.k6 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovGASTWD.kd" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.kd = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.kd = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovGASTWD.kdroop" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.kdroop = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.kdroop = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovGASTWD.kf" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.kf = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.kf = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovGASTWD.ki" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.ki = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.ki = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovGASTWD.kp" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.kp = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.kp = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovGASTWD.mwbase" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.mwbase = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.mwbase = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovGASTWD.t" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.t = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.t = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovGASTWD.t3" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.t3 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.t3 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovGASTWD.t4" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.t4 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.t4 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovGASTWD.t5" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.t5 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.t5 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovGASTWD.tc" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.tc = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.tc = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovGASTWD.tcd" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.tcd = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.tcd = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovGASTWD.td" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.td = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.td = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovGASTWD.tf" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.tf = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.tf = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovGASTWD.tmax" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.tmax = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.tmax = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovGASTWD.tmin" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.tmin = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.tmin = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovGASTWD.tr" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.tr = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.tr = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovGASTWD.trate" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.trate = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.trate = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovGASTWD.tt" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.tt = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.tt = Some(v); } }
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
