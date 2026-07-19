/// WoodwardTM electric hydro governor. [Footnote: Woodward electric hydro governors are an example of suitable products available commercially. This information is given for the convenience of users of this document and does not constitute an endorsement by IEC of these products.]
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct GovHydroWEH {
    #[serde(flatten)]
    pub base: super::TurbineGovernorDynamics,
    /// Speed deadband (db).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db: Option<f64>,
    /// Value to allow the integral controller to advance beyond the gate limits (Dicn).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dicn: Option<f64>,
    /// Value to allow the pilot valve controller to advance beyond the gate limits (Dpv).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dpv: Option<f64>,
    /// Turbine damping factor (Dturb). Unit = delta P (PU of MWbase) / delta speed (PU).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dturb: Option<f64>,
    /// Feedback signal selection (Sw). true = PID output (if R-Perm-Gate = droop and R-Perm-Pe = 0) false = electrical power (if R-Perm-Gate = 0 and R-Perm-Pe = droop) or false = gate position (if R-Perm-Gate = droop and R-Perm-Pe = 0). Typical value = false.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feedback_signal: Option<bool>,
    /// Flowgate 1 (Fl1). Flow value for gate position point 1 for lookup table representing water flow through the turbine as a function of gate position to produce steady state flow.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fl1: Option<f64>,
    /// Flowgate 2 (Fl2). Flow value for gate position point 2 for lookup table representing water flow through the turbine as a function of gate position to produce steady state flow.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fl2: Option<f64>,
    /// Flowgate 3 (Fl3). Flow value for gate position point 3 for lookup table representing water flow through the turbine as a function of gate position to produce steady state flow.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fl3: Option<f64>,
    /// Flowgate 4 (Fl4). Flow value for gate position point 4 for lookup table representing water flow through the turbine as a function of gate position to produce steady state flow.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fl4: Option<f64>,
    /// Flowgate 5 (Fl5). Flow value for gate position point 5 for lookup table representing water flow through the turbine as a function of gate position to produce steady state flow.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fl5: Option<f64>,
    /// Flow P1 (Fp1). Turbine flow value for point 1 for lookup table representing PU mechanical power on machine MVA rating as a function of turbine flow.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fp1: Option<f64>,
    /// Flow P10 (Fp10). Turbine flow value for point 10 for lookup table representing PU mechanical power on machine MVA rating as a function of turbine flow.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fp10: Option<f64>,
    /// Flow P2 (Fp2). Turbine flow value for point 2 for lookup table representing PU mechanical power on machine MVA rating as a function of turbine flow.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fp2: Option<f64>,
    /// Flow P3 (Fp3). Turbine flow value for point 3 for lookup table representing PU mechanical power on machine MVA rating as a function of turbine flow.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fp3: Option<f64>,
    /// Flow P4 (Fp4). Turbine flow value for point 4 for lookup table representing PU mechanical power on machine MVA rating as a function of turbine flow.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fp4: Option<f64>,
    /// Flow P5 (Fp5). Turbine flow value for point 5 for lookup table representing PU mechanical power on machine MVA rating as a function of turbine flow.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fp5: Option<f64>,
    /// Flow P6 (Fp6). Turbine flow value for point 6 for lookup table representing PU mechanical power on machine MVA rating as a function of turbine flow.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fp6: Option<f64>,
    /// Flow P7 (Fp7). Turbine flow value for point 7 for lookup table representing PU mechanical power on machine MVA rating as a function of turbine flow.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fp7: Option<f64>,
    /// Flow P8 (Fp8). Turbine flow value for point 8 for lookup table representing PU mechanical power on machine MVA rating as a function of turbine flow.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fp8: Option<f64>,
    /// Flow P9 (Fp9). Turbine flow value for point 9 for lookup table representing PU mechanical power on machine MVA rating as a function of turbine flow.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fp9: Option<f64>,
    /// Maximum gate position (Gmax) (> GovHydroWEH.gmin).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gmax: Option<f64>,
    /// Minimum gate position (Gmin) (< GovHydroWEH.gmax).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gmin: Option<f64>,
    /// Maximum gate closing rate (Gtmxcl).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gtmxcl: Option<f64>,
    /// Maximum gate opening rate (Gtmxop).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gtmxop: Option<f64>,
    /// Gate 1 (Gv1). Gate Position value for point 1 for lookup table representing water flow through the turbine as a function of gate position to produce steady state flow.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gv1: Option<f64>,
    /// Gate 2 (Gv2). Gate Position value for point 2 for lookup table representing water flow through the turbine as a function of gate position to produce steady state flow.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gv2: Option<f64>,
    /// Gate 3 (Gv3). Gate Position value for point 3 for lookup table representing water flow through the turbine as a function of gate position to produce steady state flow.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gv3: Option<f64>,
    /// Gate 4 (Gv4). Gate Position value for point 4 for lookup table representing water flow through the turbine as a function of gate position to produce steady state flow.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gv4: Option<f64>,
    /// Gate 5 (Gv5). Gate Position value for point 5 for lookup table representing water flow through the turbine as a function of gate position to produce steady state flow.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gv5: Option<f64>,
    /// Derivative controller derivative gain (Kd).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kd: Option<f64>,
    /// Derivative controller Integral gain (Ki).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ki: Option<f64>,
    /// Derivative control gain (Kp).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kp: Option<f64>,
    /// Base for power values (MWbase) (> 0). Unit = MW.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mwbase: Option<f64>,
    /// Pmss flow P1 (Pmss1). Mechanical power output for turbine flow point 1 for lookup table representing PU mechanical power on machine MVA rating as a function of turbine flow.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pmss1: Option<f64>,
    /// Pmss flow P10 (Pmss10). Mechanical power output for turbine flow point 10 for lookup table representing PU mechanical power on machine MVA rating as a function of turbine flow.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pmss10: Option<f64>,
    /// Pmss flow P2 (Pmss2). Mechanical power output for turbine flow point 2 for lookup table representing PU mechanical power on machine MVA rating as a function of turbine flow.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pmss2: Option<f64>,
    /// Pmss flow P3 (Pmss3). Mechanical power output for turbine flow point 3 for lookup table representing PU mechanical power on machine MVA rating as a function of turbine flow.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pmss3: Option<f64>,
    /// Pmss flow P4 (Pmss4). Mechanical power output for turbine flow point 4 for lookup table representing PU mechanical power on machine MVA rating as a function of turbine flow.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pmss4: Option<f64>,
    /// Pmss flow P5 (Pmss5). Mechanical power output for turbine flow point 5 for lookup table representing PU mechanical power on machine MVA rating as a function of turbine flow.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pmss5: Option<f64>,
    /// Pmss flow P6 (Pmss6). Mechanical power output for turbine flow point 6 for lookup table representing PU mechanical power on machine MVA rating as a function of turbine flow.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pmss6: Option<f64>,
    /// Pmss flow P7 (Pmss7). Mechanical power output for turbine flow point 7 for lookup table representing PU mechanical power on machine MVA rating as a function of turbine flow.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pmss7: Option<f64>,
    /// Pmss flow P8 (Pmss8). Mechanical power output for turbine flow point 8 for lookup table representing PU mechanical power on machine MVA rating as a function of turbine flow.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pmss8: Option<f64>,
    /// Pmss flow P9 (Pmss9). Mechanical power output for turbine flow point 9 for lookup table representing PU mechanical power on machine MVA rating as a function of turbine flow.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pmss9: Option<f64>,
    /// Permanent droop for governor output feedback (R-Perm-Gate).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rpg: Option<f64>,
    /// Permanent droop for electrical power feedback (R-Perm-Pe).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rpp: Option<f64>,
    /// Derivative controller time constant (Td) (>= 0). Limits the derivative characteristic beyond a breakdown frequency to avoid amplification of high-frequency noise.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub td: Option<f64>,
    /// Distributive valve time lag time constant (Tdv) (>= 0).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tdv: Option<f64>,
    /// Value to allow the distribution valve controller to advance beyond the gate movement rate limit (Tg) (>= 0).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tg: Option<f64>,
    /// Pilot valve time lag time constant (Tp) (>= 0).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tp: Option<f64>,
    /// Electrical power droop time constant (Tpe) (>= 0).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tpe: Option<f64>,
    /// Water inertia time constant (Tw) (> 0).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tw: Option<f64>,
}
impl crate::base::CimElement for GovHydroWEH {
    fn mrid(&self) -> &str { &self.base.base.base.id }
    fn type_name(&self) -> &'static str { "GovHydroWEH" }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn to_json_value(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or(serde_json::Value::Null)
    }
    fn to_block(&self) -> crate::base::RdfBlock {
        let mut block = self.base.to_block();
        block.type_name = "GovHydroWEH".to_string();
        if let Some(v) = self.db {
            block.fields.insert("GovHydroWEH.db".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.dicn {
            block.fields.insert("GovHydroWEH.dicn".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.dpv {
            block.fields.insert("GovHydroWEH.dpv".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.dturb {
            block.fields.insert("GovHydroWEH.dturb".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.feedback_signal {
            block.fields.insert("GovHydroWEH.feedbackSignal".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.fl1 {
            block.fields.insert("GovHydroWEH.fl1".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.fl2 {
            block.fields.insert("GovHydroWEH.fl2".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.fl3 {
            block.fields.insert("GovHydroWEH.fl3".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.fl4 {
            block.fields.insert("GovHydroWEH.fl4".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.fl5 {
            block.fields.insert("GovHydroWEH.fl5".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.fp1 {
            block.fields.insert("GovHydroWEH.fp1".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.fp10 {
            block.fields.insert("GovHydroWEH.fp10".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.fp2 {
            block.fields.insert("GovHydroWEH.fp2".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.fp3 {
            block.fields.insert("GovHydroWEH.fp3".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.fp4 {
            block.fields.insert("GovHydroWEH.fp4".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.fp5 {
            block.fields.insert("GovHydroWEH.fp5".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.fp6 {
            block.fields.insert("GovHydroWEH.fp6".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.fp7 {
            block.fields.insert("GovHydroWEH.fp7".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.fp8 {
            block.fields.insert("GovHydroWEH.fp8".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.fp9 {
            block.fields.insert("GovHydroWEH.fp9".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.gmax {
            block.fields.insert("GovHydroWEH.gmax".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.gmin {
            block.fields.insert("GovHydroWEH.gmin".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.gtmxcl {
            block.fields.insert("GovHydroWEH.gtmxcl".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.gtmxop {
            block.fields.insert("GovHydroWEH.gtmxop".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.gv1 {
            block.fields.insert("GovHydroWEH.gv1".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.gv2 {
            block.fields.insert("GovHydroWEH.gv2".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.gv3 {
            block.fields.insert("GovHydroWEH.gv3".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.gv4 {
            block.fields.insert("GovHydroWEH.gv4".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.gv5 {
            block.fields.insert("GovHydroWEH.gv5".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.kd {
            block.fields.insert("GovHydroWEH.kd".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.ki {
            block.fields.insert("GovHydroWEH.ki".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.kp {
            block.fields.insert("GovHydroWEH.kp".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.mwbase {
            block.fields.insert("GovHydroWEH.mwbase".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.pmss1 {
            block.fields.insert("GovHydroWEH.pmss1".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.pmss10 {
            block.fields.insert("GovHydroWEH.pmss10".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.pmss2 {
            block.fields.insert("GovHydroWEH.pmss2".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.pmss3 {
            block.fields.insert("GovHydroWEH.pmss3".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.pmss4 {
            block.fields.insert("GovHydroWEH.pmss4".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.pmss5 {
            block.fields.insert("GovHydroWEH.pmss5".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.pmss6 {
            block.fields.insert("GovHydroWEH.pmss6".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.pmss7 {
            block.fields.insert("GovHydroWEH.pmss7".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.pmss8 {
            block.fields.insert("GovHydroWEH.pmss8".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.pmss9 {
            block.fields.insert("GovHydroWEH.pmss9".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.rpg {
            block.fields.insert("GovHydroWEH.rpg".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.rpp {
            block.fields.insert("GovHydroWEH.rpp".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.td {
            block.fields.insert("GovHydroWEH.td".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.tdv {
            block.fields.insert("GovHydroWEH.tdv".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.tg {
            block.fields.insert("GovHydroWEH.tg".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.tp {
            block.fields.insert("GovHydroWEH.tp".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.tpe {
            block.fields.insert("GovHydroWEH.tpe".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.tw {
            block.fields.insert("GovHydroWEH.tw".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        block
    }
}

impl GovHydroWEH {
    pub fn from_block(b: &crate::base::RdfBlock) -> Self {
        let mut obj = Self::default();
        obj.base.base.base.id.clone_from(&b.mrid);
        for (key, val) in &b.fields {
            match key.as_str() {
                "GovHydroWEH.db" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.db = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.db = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovHydroWEH.dicn" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.dicn = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.dicn = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovHydroWEH.dpv" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.dpv = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.dpv = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovHydroWEH.dturb" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.dturb = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.dturb = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovHydroWEH.feedbackSignal" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.feedback_signal = Some(sv.trim() == "true"); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.feedback_signal = Some(sv.trim() == "true"); }
                        }
                        _ => {}
                    }
                }
                "GovHydroWEH.fl1" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.fl1 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.fl1 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovHydroWEH.fl2" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.fl2 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.fl2 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovHydroWEH.fl3" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.fl3 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.fl3 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovHydroWEH.fl4" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.fl4 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.fl4 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovHydroWEH.fl5" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.fl5 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.fl5 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovHydroWEH.fp1" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.fp1 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.fp1 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovHydroWEH.fp10" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.fp10 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.fp10 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovHydroWEH.fp2" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.fp2 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.fp2 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovHydroWEH.fp3" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.fp3 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.fp3 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovHydroWEH.fp4" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.fp4 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.fp4 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovHydroWEH.fp5" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.fp5 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.fp5 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovHydroWEH.fp6" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.fp6 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.fp6 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovHydroWEH.fp7" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.fp7 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.fp7 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovHydroWEH.fp8" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.fp8 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.fp8 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovHydroWEH.fp9" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.fp9 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.fp9 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovHydroWEH.gmax" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.gmax = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.gmax = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovHydroWEH.gmin" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.gmin = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.gmin = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovHydroWEH.gtmxcl" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.gtmxcl = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.gtmxcl = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovHydroWEH.gtmxop" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.gtmxop = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.gtmxop = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovHydroWEH.gv1" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.gv1 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.gv1 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovHydroWEH.gv2" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.gv2 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.gv2 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovHydroWEH.gv3" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.gv3 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.gv3 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovHydroWEH.gv4" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.gv4 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.gv4 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovHydroWEH.gv5" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.gv5 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.gv5 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovHydroWEH.kd" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.kd = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.kd = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovHydroWEH.ki" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.ki = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.ki = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovHydroWEH.kp" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.kp = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.kp = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovHydroWEH.mwbase" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.mwbase = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.mwbase = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovHydroWEH.pmss1" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.pmss1 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.pmss1 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovHydroWEH.pmss10" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.pmss10 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.pmss10 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovHydroWEH.pmss2" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.pmss2 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.pmss2 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovHydroWEH.pmss3" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.pmss3 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.pmss3 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovHydroWEH.pmss4" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.pmss4 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.pmss4 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovHydroWEH.pmss5" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.pmss5 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.pmss5 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovHydroWEH.pmss6" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.pmss6 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.pmss6 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovHydroWEH.pmss7" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.pmss7 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.pmss7 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovHydroWEH.pmss8" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.pmss8 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.pmss8 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovHydroWEH.pmss9" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.pmss9 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.pmss9 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovHydroWEH.rpg" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.rpg = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.rpg = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovHydroWEH.rpp" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.rpp = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.rpp = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovHydroWEH.td" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.td = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.td = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovHydroWEH.tdv" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.tdv = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.tdv = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovHydroWEH.tg" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.tg = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.tg = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovHydroWEH.tp" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.tp = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.tp = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovHydroWEH.tpe" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.tpe = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.tpe = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovHydroWEH.tw" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.tw = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.tw = Some(v); } }
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
