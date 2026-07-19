/// PID governor and turbine.
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct GovHydroPID {
    #[serde(flatten)]
    pub base: super::TurbineGovernorDynamics,
    /// Turbine numerator multiplier (Aturb) (see parameter detail 3). Typical value -1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aturb: Option<f64>,
    /// Turbine denominator multiplier (Bturb) (see parameter detail 3). Typical value = 0,5.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bturb: Option<f64>,
    /// Intentional dead-band width (db1). Unit = Hz. Typical value = 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db1: Option<f64>,
    /// Unintentional dead-band (db2). Unit = MW. Typical value = 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db2: Option<f64>,
    /// Intentional db hysteresis (eps). Unit = Hz. Typical value = 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eps: Option<f64>,
    /// Nonlinear gain point 1, PU gv (Gv1). Typical value = 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gv1: Option<f64>,
    /// Nonlinear gain point 2, PU gv (Gv2). Typical value = 0.
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
    /// Input signal switch (Flag). true = Pe input is used false = feedback is received from CV. Flag is normally dependent on Tt. If Tt is zero, Flag is set to false. If Tt is not zero, Flag is set to true. Typical value = true.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_signal: Option<bool>,
    /// Derivative gain (Kd). Typical value = 1,11.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kd: Option<f64>,
    /// Gate servo gain (Kg). Typical value = 2,5.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kg: Option<f64>,
    /// Integral gain (Ki). Typical value = 0,36.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ki: Option<f64>,
    /// Proportional gain (Kp). Typical value = 0,1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kp: Option<f64>,
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
    /// Maximum gate opening, PU of MWbase (Pmax) (> GovHydroPID.pmin). Typical value = 1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pmax: Option<f64>,
    /// Minimum gate opening, PU of MWbase (Pmin) (< GovHydroPID.pmax). Typical value = 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pmin: Option<f64>,
    /// Steady state droop (R). Typical value = 0,05.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r: Option<f64>,
    /// Input filter time constant (Td) (>= 0). If = 0, block is bypassed. Typical value = 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub td: Option<f64>,
    /// Washout time constant (Tf) (>= 0). Typical value = 0,1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tf: Option<f64>,
    /// Gate servo time constant (Tp) (>= 0). If = 0, block is bypassed. Typical value = 0,35.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tp: Option<f64>,
    /// Power feedback time constant (Tt) (>= 0). If = 0, block is bypassed. Typical value = 0,02.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tt: Option<f64>,
    /// Turbine time constant (Tturb) (>= 0). See Parameter detail 3. Typical value = 0,8.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tturb: Option<f64>,
    /// Maximum gate closing velocity (Velcl). Unit = PU / s. Typical value = -0,14.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub velcl: Option<f64>,
    /// Maximum gate opening velocity (Velop). Unit = PU / s. Typical value = 0,09.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub velop: Option<f64>,
}
impl crate::base::CimElement for GovHydroPID {
    fn mrid(&self) -> &str { &self.base.base.base.id }
    fn type_name(&self) -> &'static str { "GovHydroPID" }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn to_json_value(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or(serde_json::Value::Null)
    }
    fn to_block(&self) -> crate::base::RdfBlock {
        let mut block = self.base.to_block();
        block.type_name = "GovHydroPID".to_string();
        if let Some(v) = self.aturb {
            block.fields.insert("GovHydroPID.aturb".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.bturb {
            block.fields.insert("GovHydroPID.bturb".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.db1 {
            block.fields.insert("GovHydroPID.db1".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.db2 {
            block.fields.insert("GovHydroPID.db2".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.eps {
            block.fields.insert("GovHydroPID.eps".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.gv1 {
            block.fields.insert("GovHydroPID.gv1".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.gv2 {
            block.fields.insert("GovHydroPID.gv2".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.gv3 {
            block.fields.insert("GovHydroPID.gv3".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.gv4 {
            block.fields.insert("GovHydroPID.gv4".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.gv5 {
            block.fields.insert("GovHydroPID.gv5".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.gv6 {
            block.fields.insert("GovHydroPID.gv6".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.input_signal {
            block.fields.insert("GovHydroPID.inputSignal".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.kd {
            block.fields.insert("GovHydroPID.kd".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.kg {
            block.fields.insert("GovHydroPID.kg".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.ki {
            block.fields.insert("GovHydroPID.ki".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.kp {
            block.fields.insert("GovHydroPID.kp".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.mwbase {
            block.fields.insert("GovHydroPID.mwbase".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.pgv1 {
            block.fields.insert("GovHydroPID.pgv1".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.pgv2 {
            block.fields.insert("GovHydroPID.pgv2".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.pgv3 {
            block.fields.insert("GovHydroPID.pgv3".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.pgv4 {
            block.fields.insert("GovHydroPID.pgv4".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.pgv5 {
            block.fields.insert("GovHydroPID.pgv5".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.pgv6 {
            block.fields.insert("GovHydroPID.pgv6".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.pmax {
            block.fields.insert("GovHydroPID.pmax".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.pmin {
            block.fields.insert("GovHydroPID.pmin".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.r {
            block.fields.insert("GovHydroPID.r".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.td {
            block.fields.insert("GovHydroPID.td".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.tf {
            block.fields.insert("GovHydroPID.tf".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.tp {
            block.fields.insert("GovHydroPID.tp".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.tt {
            block.fields.insert("GovHydroPID.tt".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.tturb {
            block.fields.insert("GovHydroPID.tturb".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.velcl {
            block.fields.insert("GovHydroPID.velcl".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.velop {
            block.fields.insert("GovHydroPID.velop".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        block
    }
}

impl GovHydroPID {
    pub fn from_block(b: &crate::base::RdfBlock) -> Self {
        let mut obj = Self::default();
        obj.base.base.base.id.clone_from(&b.mrid);
        for (key, val) in &b.fields {
            match key.as_str() {
                "GovHydroPID.aturb" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.aturb = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.aturb = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovHydroPID.bturb" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.bturb = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.bturb = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovHydroPID.db1" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.db1 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.db1 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovHydroPID.db2" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.db2 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.db2 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovHydroPID.eps" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.eps = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.eps = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovHydroPID.gv1" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.gv1 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.gv1 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovHydroPID.gv2" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.gv2 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.gv2 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovHydroPID.gv3" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.gv3 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.gv3 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovHydroPID.gv4" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.gv4 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.gv4 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovHydroPID.gv5" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.gv5 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.gv5 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovHydroPID.gv6" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.gv6 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.gv6 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovHydroPID.inputSignal" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.input_signal = Some(sv.trim() == "true"); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.input_signal = Some(sv.trim() == "true"); }
                        }
                        _ => {}
                    }
                }
                "GovHydroPID.kd" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.kd = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.kd = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovHydroPID.kg" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.kg = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.kg = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovHydroPID.ki" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.ki = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.ki = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovHydroPID.kp" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.kp = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.kp = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovHydroPID.mwbase" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.mwbase = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.mwbase = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovHydroPID.pgv1" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.pgv1 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.pgv1 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovHydroPID.pgv2" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.pgv2 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.pgv2 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovHydroPID.pgv3" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.pgv3 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.pgv3 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovHydroPID.pgv4" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.pgv4 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.pgv4 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovHydroPID.pgv5" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.pgv5 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.pgv5 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovHydroPID.pgv6" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.pgv6 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.pgv6 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovHydroPID.pmax" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.pmax = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.pmax = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovHydroPID.pmin" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.pmin = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.pmin = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovHydroPID.r" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.r = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.r = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovHydroPID.td" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.td = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.td = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovHydroPID.tf" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.tf = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.tf = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovHydroPID.tp" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.tp = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.tp = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovHydroPID.tt" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.tt = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.tt = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovHydroPID.tturb" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.tturb = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.tturb = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovHydroPID.velcl" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.velcl = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.velcl = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovHydroPID.velop" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.velop = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.velop = Some(v); } }
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
