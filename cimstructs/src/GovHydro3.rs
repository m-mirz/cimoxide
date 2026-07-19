/// Modified IEEE hydro governor-turbine. This model differs from that defined in the IEEE modelling guideline paper in that the limits on gate position and velocity do not permit 'wind up' of the upstream signals.
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct GovHydro3 {
    #[serde(flatten)]
    pub base: super::TurbineGovernorDynamics,
    /// Turbine gain (At) (>0). Typical value = 1,2.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub at: Option<f64>,
    /// Intentional dead-band width (db1). Unit = Hz. Typical value = 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db1: Option<f64>,
    /// Unintentional dead-band (db2). Unit = MW. Typical value = 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db2: Option<f64>,
    /// Turbine damping factor (Dturb). Typical value = 0,2.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dturb: Option<f64>,
    /// Intentional db hysteresis (eps). Unit = Hz. Typical value = 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eps: Option<f64>,
    /// Governor control flag (Cflag). true = PID control is active false = double derivative control is active. Typical value = true.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub governor_control: Option<bool>,
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
    /// Turbine nominal head (H0). Typical value = 1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub h0: Option<f64>,
    /// Derivative gain (K1). Typical value = 0,01.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub k1: Option<f64>,
    /// Double derivative gain, if Cflag = -1 (K2). Typical value = 2,5.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub k2: Option<f64>,
    /// Gate servo gain (Kg). Typical value = 2.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kg: Option<f64>,
    /// Integral gain (Ki). Typical value = 0,5.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ki: Option<f64>,
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
    /// Maximum gate opening, PU of MWbase (Pmax) (> GovHydro3.pmin). Typical value = 1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pmax: Option<f64>,
    /// Minimum gate opening, PU of MWbase (Pmin) (< GovHydro3.pmax). Typical value = 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pmin: Option<f64>,
    /// No-load turbine flow at nominal head (Qnl). Typical value = 0,08.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qnl: Option<f64>,
    /// Steady-state droop, PU, for electrical power feedback (Relec). Typical value = 0,05.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relec: Option<f64>,
    /// Steady-state droop, PU, for governor output feedback (Rgate). Typical value = 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rgate: Option<f64>,
    /// Input filter time constant (Td) (>= 0). Typical value = 0,05.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub td: Option<f64>,
    /// Washout time constant (Tf) (>= 0). Typical value = 0,1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tf: Option<f64>,
    /// Gate servo time constant (Tp) (>= 0). Typical value = 0,05.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tp: Option<f64>,
    /// Power feedback time constant (Tt) (>= 0). Typical value = 0,2.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tt: Option<f64>,
    /// Water inertia time constant (Tw) (>= 0). If = 0, block is bypassed. Typical value = 1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tw: Option<f64>,
    /// Maximum gate closing velocity (Velcl). Unit = PU / s. Typical value = -0,2.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub velcl: Option<f64>,
    /// Maximum gate opening velocity (Velop). Unit = PU / s. Typical value = 0,2.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub velop: Option<f64>,
}
impl crate::base::CimElement for GovHydro3 {
    fn mrid(&self) -> &str { &self.base.base.base.id }
    fn type_name(&self) -> &'static str { "GovHydro3" }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn to_json_value(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or(serde_json::Value::Null)
    }
    fn to_block(&self) -> crate::base::RdfBlock {
        let mut block = self.base.to_block();
        block.type_name = "GovHydro3".to_string();
        if let Some(v) = self.at {
            block.fields.insert("GovHydro3.at".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.db1 {
            block.fields.insert("GovHydro3.db1".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.db2 {
            block.fields.insert("GovHydro3.db2".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.dturb {
            block.fields.insert("GovHydro3.dturb".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.eps {
            block.fields.insert("GovHydro3.eps".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.governor_control {
            block.fields.insert("GovHydro3.governorControl".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.gv1 {
            block.fields.insert("GovHydro3.gv1".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.gv2 {
            block.fields.insert("GovHydro3.gv2".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.gv3 {
            block.fields.insert("GovHydro3.gv3".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.gv4 {
            block.fields.insert("GovHydro3.gv4".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.gv5 {
            block.fields.insert("GovHydro3.gv5".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.gv6 {
            block.fields.insert("GovHydro3.gv6".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.h0 {
            block.fields.insert("GovHydro3.h0".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.k1 {
            block.fields.insert("GovHydro3.k1".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.k2 {
            block.fields.insert("GovHydro3.k2".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.kg {
            block.fields.insert("GovHydro3.kg".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.ki {
            block.fields.insert("GovHydro3.ki".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.mwbase {
            block.fields.insert("GovHydro3.mwbase".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.pgv1 {
            block.fields.insert("GovHydro3.pgv1".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.pgv2 {
            block.fields.insert("GovHydro3.pgv2".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.pgv3 {
            block.fields.insert("GovHydro3.pgv3".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.pgv4 {
            block.fields.insert("GovHydro3.pgv4".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.pgv5 {
            block.fields.insert("GovHydro3.pgv5".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.pgv6 {
            block.fields.insert("GovHydro3.pgv6".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.pmax {
            block.fields.insert("GovHydro3.pmax".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.pmin {
            block.fields.insert("GovHydro3.pmin".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.qnl {
            block.fields.insert("GovHydro3.qnl".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.relec {
            block.fields.insert("GovHydro3.relec".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.rgate {
            block.fields.insert("GovHydro3.rgate".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.td {
            block.fields.insert("GovHydro3.td".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.tf {
            block.fields.insert("GovHydro3.tf".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.tp {
            block.fields.insert("GovHydro3.tp".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.tt {
            block.fields.insert("GovHydro3.tt".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.tw {
            block.fields.insert("GovHydro3.tw".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.velcl {
            block.fields.insert("GovHydro3.velcl".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.velop {
            block.fields.insert("GovHydro3.velop".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        block
    }
}

impl GovHydro3 {
    pub fn from_block(b: &crate::base::RdfBlock) -> Self {
        let mut obj = Self::default();
        obj.base.base.base.id.clone_from(&b.mrid);
        for (key, val) in &b.fields {
            match key.as_str() {
                "GovHydro3.at" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.at = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.at = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovHydro3.db1" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.db1 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.db1 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovHydro3.db2" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.db2 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.db2 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovHydro3.dturb" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.dturb = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.dturb = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovHydro3.eps" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.eps = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.eps = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovHydro3.governorControl" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.governor_control = Some(sv.trim() == "true"); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.governor_control = Some(sv.trim() == "true"); }
                        }
                        _ => {}
                    }
                }
                "GovHydro3.gv1" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.gv1 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.gv1 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovHydro3.gv2" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.gv2 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.gv2 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovHydro3.gv3" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.gv3 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.gv3 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovHydro3.gv4" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.gv4 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.gv4 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovHydro3.gv5" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.gv5 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.gv5 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovHydro3.gv6" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.gv6 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.gv6 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovHydro3.h0" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.h0 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.h0 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovHydro3.k1" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.k1 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.k1 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovHydro3.k2" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.k2 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.k2 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovHydro3.kg" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.kg = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.kg = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovHydro3.ki" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.ki = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.ki = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovHydro3.mwbase" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.mwbase = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.mwbase = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovHydro3.pgv1" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.pgv1 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.pgv1 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovHydro3.pgv2" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.pgv2 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.pgv2 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovHydro3.pgv3" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.pgv3 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.pgv3 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovHydro3.pgv4" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.pgv4 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.pgv4 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovHydro3.pgv5" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.pgv5 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.pgv5 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovHydro3.pgv6" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.pgv6 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.pgv6 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovHydro3.pmax" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.pmax = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.pmax = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovHydro3.pmin" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.pmin = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.pmin = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovHydro3.qnl" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.qnl = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.qnl = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovHydro3.relec" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.relec = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.relec = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovHydro3.rgate" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.rgate = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.rgate = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovHydro3.td" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.td = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.td = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovHydro3.tf" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.tf = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.tf = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovHydro3.tp" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.tp = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.tp = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovHydro3.tt" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.tt = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.tt = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovHydro3.tw" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.tw = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.tw = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovHydro3.velcl" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.velcl = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.velcl = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovHydro3.velop" => {
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
