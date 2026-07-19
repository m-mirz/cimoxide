/// Hydro turbine and governor. Represents plants with straight-forward penstock configurations and hydraulic governors of the traditional 'dashpot' type. This model can be used to represent simple, Francis/Pelton or Kaplan turbines.
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct GovHydro4 {
    #[serde(flatten)]
    pub base: super::TurbineGovernorDynamics,
    /// Turbine gain (At). Typical value = 1,2.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub at: Option<f64>,
    /// Kaplan blade servo point 0 (Bgv0) (= 0 for simple, = 0 for Francis/Pelton). Typical value for Kaplan = 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bgv0: Option<f64>,
    /// Kaplan blade servo point 1 (Bgv1) (= 0 for simple, = 0 for Francis/Pelton). Typical value for Kaplan = 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bgv1: Option<f64>,
    /// Kaplan blade servo point 2 (Bgv2) (= 0 for simple, = 0 for Francis/Pelton). Typical value for Kaplan = 0,1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bgv2: Option<f64>,
    /// Kaplan blade servo point 3 (Bgv3) (= 0 for simple, = 0 for Francis/Pelton). Typical value for Kaplan = 0,667.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bgv3: Option<f64>,
    /// Kaplan blade servo point 4 (Bgv4) (= 0 for simple, = 0 for Francis/Pelton). Typical value for Kaplan = 0,9.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bgv4: Option<f64>,
    /// Kaplan blade servo point 5 (Bgv5) (= 0 for simple, = 0 for Francis/Pelton). Typical value for Kaplan = 1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bgv5: Option<f64>,
    /// Maximum blade adjustment factor (Bmax) (= 0 for simple, = 0 for Francis/Pelton). Typical value for Kaplan = 1,1276.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bmax: Option<f64>,
    /// Intentional deadband width (db1). Unit = Hz. Typical value = 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db1: Option<f64>,
    /// Unintentional dead-band (db2). Unit = MW. Typical value = 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db2: Option<f64>,
    /// Turbine damping factor (Dturb). Unit = delta P (PU of MWbase) / delta speed (PU). Typical value for simple = 0,5, Francis/Pelton = 1,1, Kaplan = 1,1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dturb: Option<f64>,
    /// Intentional db hysteresis (eps). Unit = Hz. Typical value = 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eps: Option<f64>,
    /// Maximum gate opening, PU of MWbase (Gmax) (> GovHydro4.gmin). Typical value = 1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gmax: Option<f64>,
    /// Minimum gate opening, PU of MWbase (Gmin) (< GovHydro4.gmax). Typical value = 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gmin: Option<f64>,
    /// Nonlinear gain point 0, PU gv (Gv0) (= 0 for simple). Typical for Francis/Pelton = 0,1, Kaplan = 0,1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gv0: Option<f64>,
    /// Nonlinear gain point 1, PU gv (Gv1) (= 0 for simple, > GovHydro4.gv0 for Francis/Pelton and Kaplan). Typical value for Francis/Pelton = 0,4, Kaplan = 0,4.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gv1: Option<f64>,
    /// Nonlinear gain point 2, PU gv (Gv2) (= 0 for simple, > GovHydro4.gv1 for Francis/Pelton and Kaplan). Typical value for Francis/Pelton = 0,5, Kaplan = 0,5.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gv2: Option<f64>,
    /// Nonlinear gain point 3, PU gv (Gv3) (= 0 for simple, > GovHydro4.gv2 for Francis/Pelton and Kaplan). Typical value for Francis/Pelton = 0,7, Kaplan = 0,7.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gv3: Option<f64>,
    /// Nonlinear gain point 4, PU gv (Gv4) (= 0 for simple, > GovHydro4.gv3 for Francis/Pelton and Kaplan). Typical value for Francis/Pelton = 0,8, Kaplan = 0,8.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gv4: Option<f64>,
    /// Nonlinear gain point 5, PU gv (Gv5) (= 0 for simple, < 1 and > GovHydro4.gv4 for Francis/Pelton and Kaplan). Typical value for Francis/Pelton = 0,9, Kaplan = 0,9.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gv5: Option<f64>,
    /// Head available at dam (hdam). Typical value = 1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hdam: Option<f64>,
    /// The kind of model being represented (simple, Francis/Pelton or Kaplan).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<super::base::UriRef>,
    /// Base for power values (MWbase) (> 0). Unit = MW.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mwbase: Option<f64>,
    /// Nonlinear gain point 0, PU power (Pgv0) (= 0 for simple). Typical value = 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pgv0: Option<f64>,
    /// Nonlinear gain point 1, PU power (Pgv1) (= 0 for simple). Typical value for Francis/Pelton = 0,42, Kaplan = 0,35.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pgv1: Option<f64>,
    /// Nonlinear gain point 2, PU power (Pgv2) (= 0 for simple). Typical value for Francis/Pelton = 0,56, Kaplan = 0,468.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pgv2: Option<f64>,
    /// Nonlinear gain point 3, PU power (Pgv3) (= 0 for simple). Typical value for Francis/Pelton = 0,8, Kaplan = 0,796.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pgv3: Option<f64>,
    /// Nonlinear gain point 4, PU power (Pgv4) (= 0 for simple). Typical value for Francis/Pelton = 0,9, Kaplan = 0,917.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pgv4: Option<f64>,
    /// Nonlinear gain point 5, PU power (Pgv5) (= 0 for simple). Typical value for Francis/Pelton = 0,97, Kaplan = 0,99.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pgv5: Option<f64>,
    /// No-load flow at nominal head (Qnl). Typical value for simple = 0,08, Francis/Pelton = 0, Kaplan = 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qnl: Option<f64>,
    /// Permanent droop (Rperm) (>= 0). Typical value = 0,05.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rperm: Option<f64>,
    /// Temporary droop (Rtemp) (>= 0). Typical value = 0,3.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rtemp: Option<f64>,
    /// Blade servo time constant (Tblade) (>= 0). Typical value = 100.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tblade: Option<f64>,
    /// Gate servo time constant (Tg) (> 0). Typical value = 0,5.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tg: Option<f64>,
    /// Pilot servo time constant (Tp) (>= 0). Typical value = 0,1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tp: Option<f64>,
    /// Dashpot time constant (Tr) (>= 0). Typical value = 5.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tr: Option<f64>,
    /// Water inertia time constant (Tw) (> 0). Typical value = 1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tw: Option<f64>,
    /// Max gate closing velocity (Uc). Typical value = 0,2.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uc: Option<f64>,
    /// Max gate opening velocity (Uo). Typical value = 0,2.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uo: Option<f64>,
}
impl crate::base::CimElement for GovHydro4 {
    fn mrid(&self) -> &str { &self.base.base.base.id }
    fn type_name(&self) -> &'static str { "GovHydro4" }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn to_json_value(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or(serde_json::Value::Null)
    }
    fn to_block(&self) -> crate::base::RdfBlock {
        let mut block = self.base.to_block();
        block.type_name = "GovHydro4".to_string();
        if let Some(v) = self.at {
            block.fields.insert("GovHydro4.at".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.bgv0 {
            block.fields.insert("GovHydro4.bgv0".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.bgv1 {
            block.fields.insert("GovHydro4.bgv1".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.bgv2 {
            block.fields.insert("GovHydro4.bgv2".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.bgv3 {
            block.fields.insert("GovHydro4.bgv3".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.bgv4 {
            block.fields.insert("GovHydro4.bgv4".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.bgv5 {
            block.fields.insert("GovHydro4.bgv5".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.bmax {
            block.fields.insert("GovHydro4.bmax".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.db1 {
            block.fields.insert("GovHydro4.db1".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.db2 {
            block.fields.insert("GovHydro4.db2".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.dturb {
            block.fields.insert("GovHydro4.dturb".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.eps {
            block.fields.insert("GovHydro4.eps".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.gmax {
            block.fields.insert("GovHydro4.gmax".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.gmin {
            block.fields.insert("GovHydro4.gmin".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.gv0 {
            block.fields.insert("GovHydro4.gv0".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.gv1 {
            block.fields.insert("GovHydro4.gv1".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.gv2 {
            block.fields.insert("GovHydro4.gv2".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.gv3 {
            block.fields.insert("GovHydro4.gv3".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.gv4 {
            block.fields.insert("GovHydro4.gv4".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.gv5 {
            block.fields.insert("GovHydro4.gv5".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.hdam {
            block.fields.insert("GovHydro4.hdam".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(ref v) = self.model {
            block.fields.insert("GovHydro4.model".into(), crate::base::FieldValue::Resource(v.uri.clone()));
        }
        if let Some(v) = self.mwbase {
            block.fields.insert("GovHydro4.mwbase".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.pgv0 {
            block.fields.insert("GovHydro4.pgv0".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.pgv1 {
            block.fields.insert("GovHydro4.pgv1".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.pgv2 {
            block.fields.insert("GovHydro4.pgv2".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.pgv3 {
            block.fields.insert("GovHydro4.pgv3".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.pgv4 {
            block.fields.insert("GovHydro4.pgv4".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.pgv5 {
            block.fields.insert("GovHydro4.pgv5".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.qnl {
            block.fields.insert("GovHydro4.qnl".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.rperm {
            block.fields.insert("GovHydro4.rperm".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.rtemp {
            block.fields.insert("GovHydro4.rtemp".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.tblade {
            block.fields.insert("GovHydro4.tblade".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.tg {
            block.fields.insert("GovHydro4.tg".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.tp {
            block.fields.insert("GovHydro4.tp".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.tr {
            block.fields.insert("GovHydro4.tr".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.tw {
            block.fields.insert("GovHydro4.tw".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.uc {
            block.fields.insert("GovHydro4.uc".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.uo {
            block.fields.insert("GovHydro4.uo".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        block
    }
}

impl GovHydro4 {
    pub fn from_block(b: &crate::base::RdfBlock) -> Self {
        let mut obj = Self::default();
        obj.base.base.base.id.clone_from(&b.mrid);
        for (key, val) in &b.fields {
            match key.as_str() {
                "GovHydro4.at" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.at = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.at = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovHydro4.bgv0" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.bgv0 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.bgv0 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovHydro4.bgv1" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.bgv1 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.bgv1 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovHydro4.bgv2" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.bgv2 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.bgv2 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovHydro4.bgv3" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.bgv3 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.bgv3 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovHydro4.bgv4" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.bgv4 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.bgv4 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovHydro4.bgv5" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.bgv5 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.bgv5 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovHydro4.bmax" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.bmax = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.bmax = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovHydro4.db1" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.db1 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.db1 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovHydro4.db2" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.db2 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.db2 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovHydro4.dturb" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.dturb = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.dturb = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovHydro4.eps" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.eps = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.eps = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovHydro4.gmax" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.gmax = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.gmax = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovHydro4.gmin" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.gmin = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.gmin = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovHydro4.gv0" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.gv0 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.gv0 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovHydro4.gv1" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.gv1 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.gv1 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovHydro4.gv2" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.gv2 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.gv2 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovHydro4.gv3" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.gv3 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.gv3 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovHydro4.gv4" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.gv4 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.gv4 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovHydro4.gv5" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.gv5 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.gv5 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovHydro4.hdam" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.hdam = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.hdam = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovHydro4.model" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.model = Some(crate::base::UriRef { uri: sv.clone() });
                    }
                }
                "GovHydro4.mwbase" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.mwbase = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.mwbase = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovHydro4.pgv0" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.pgv0 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.pgv0 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovHydro4.pgv1" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.pgv1 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.pgv1 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovHydro4.pgv2" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.pgv2 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.pgv2 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovHydro4.pgv3" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.pgv3 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.pgv3 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovHydro4.pgv4" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.pgv4 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.pgv4 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovHydro4.pgv5" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.pgv5 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.pgv5 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovHydro4.qnl" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.qnl = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.qnl = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovHydro4.rperm" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.rperm = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.rperm = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovHydro4.rtemp" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.rtemp = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.rtemp = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovHydro4.tblade" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.tblade = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.tblade = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovHydro4.tg" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.tg = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.tg = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovHydro4.tp" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.tp = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.tp = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovHydro4.tr" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.tr = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.tr = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovHydro4.tw" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.tw = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.tw = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovHydro4.uc" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.uc = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.uc = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovHydro4.uo" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.uo = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.uo = Some(v); } }
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
