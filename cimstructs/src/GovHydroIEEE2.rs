/// IEEE hydro turbine governor model represents plants with straightforward penstock configurations and hydraulic-dashpot governors. Reference: IEEE Transactions on Power Apparatus and Systems, November/December 1973, Volume PAS-92, Number 6, Dynamic Models for Steam and Hydro Turbines in Power System Studies, page 1904.
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct GovHydroIEEE2 {
    #[serde(flatten)]
    pub base: super::TurbineGovernorDynamics,
    /// Turbine numerator multiplier (Aturb). Typical value = -1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aturb: Option<f64>,
    /// Turbine denominator multiplier (Bturb) (> 0). Typical value = 0,5.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bturb: Option<f64>,
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
    /// Turbine gain (Kturb). Typical value = 1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kturb: Option<f64>,
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
    /// Maximum gate opening (Pmax) (> GovHydroIEEE2.pmin). Typical value = 1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pmax: Option<f64>,
    /// Minimum gate opening (Pmin) (<GovHydroIEEE2.pmax). Typical value = 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pmin: Option<f64>,
    /// Permanent droop (Rperm). Typical value = 0,05.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rperm: Option<f64>,
    /// Temporary droop (Rtemp). Typical value = 0,5.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rtemp: Option<f64>,
    /// Gate servo time constant (Tg) (>= 0). Typical value = 0,5.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tg: Option<f64>,
    /// Pilot servo valve time constant (Tp) (>= 0). Typical value = 0,03.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tp: Option<f64>,
    /// Dashpot time constant (Tr) (>= 0). Typical value = 12.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tr: Option<f64>,
    /// Water inertia time constant (Tw) (>= 0). Typical value = 2.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tw: Option<f64>,
    /// Maximum gate closing velocity (Uc) (<0). Typical value = -0,1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uc: Option<f64>,
    /// Maximum gate opening velocity (Uo). Unit = PU / s. Typical value = 0,1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uo: Option<f64>,
}
impl crate::base::CimElement for GovHydroIEEE2 {
    fn mrid(&self) -> &str { &self.base.base.base.id }
    fn type_name(&self) -> &'static str { "GovHydroIEEE2" }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn to_json_value(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or(serde_json::Value::Null)
    }
    fn to_block(&self) -> crate::base::RdfBlock {
        let mut block = self.base.to_block();
        block.type_name = "GovHydroIEEE2".to_string();
        if let Some(v) = self.aturb {
            block.fields.insert("GovHydroIEEE2.aturb".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.bturb {
            block.fields.insert("GovHydroIEEE2.bturb".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.gv1 {
            block.fields.insert("GovHydroIEEE2.gv1".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.gv2 {
            block.fields.insert("GovHydroIEEE2.gv2".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.gv3 {
            block.fields.insert("GovHydroIEEE2.gv3".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.gv4 {
            block.fields.insert("GovHydroIEEE2.gv4".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.gv5 {
            block.fields.insert("GovHydroIEEE2.gv5".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.gv6 {
            block.fields.insert("GovHydroIEEE2.gv6".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.kturb {
            block.fields.insert("GovHydroIEEE2.kturb".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.mwbase {
            block.fields.insert("GovHydroIEEE2.mwbase".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.pgv1 {
            block.fields.insert("GovHydroIEEE2.pgv1".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.pgv2 {
            block.fields.insert("GovHydroIEEE2.pgv2".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.pgv3 {
            block.fields.insert("GovHydroIEEE2.pgv3".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.pgv4 {
            block.fields.insert("GovHydroIEEE2.pgv4".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.pgv5 {
            block.fields.insert("GovHydroIEEE2.pgv5".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.pgv6 {
            block.fields.insert("GovHydroIEEE2.pgv6".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.pmax {
            block.fields.insert("GovHydroIEEE2.pmax".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.pmin {
            block.fields.insert("GovHydroIEEE2.pmin".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.rperm {
            block.fields.insert("GovHydroIEEE2.rperm".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.rtemp {
            block.fields.insert("GovHydroIEEE2.rtemp".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.tg {
            block.fields.insert("GovHydroIEEE2.tg".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.tp {
            block.fields.insert("GovHydroIEEE2.tp".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.tr {
            block.fields.insert("GovHydroIEEE2.tr".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.tw {
            block.fields.insert("GovHydroIEEE2.tw".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.uc {
            block.fields.insert("GovHydroIEEE2.uc".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.uo {
            block.fields.insert("GovHydroIEEE2.uo".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        block
    }
}

impl GovHydroIEEE2 {
    pub fn from_block(b: &crate::base::RdfBlock) -> Self {
        let mut obj = Self::default();
        obj.base.base.base.id.clone_from(&b.mrid);
        for (key, val) in &b.fields {
            match key.as_str() {
                "GovHydroIEEE2.aturb" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.aturb = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.aturb = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovHydroIEEE2.bturb" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.bturb = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.bturb = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovHydroIEEE2.gv1" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.gv1 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.gv1 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovHydroIEEE2.gv2" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.gv2 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.gv2 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovHydroIEEE2.gv3" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.gv3 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.gv3 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovHydroIEEE2.gv4" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.gv4 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.gv4 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovHydroIEEE2.gv5" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.gv5 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.gv5 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovHydroIEEE2.gv6" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.gv6 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.gv6 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovHydroIEEE2.kturb" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.kturb = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.kturb = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovHydroIEEE2.mwbase" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.mwbase = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.mwbase = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovHydroIEEE2.pgv1" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.pgv1 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.pgv1 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovHydroIEEE2.pgv2" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.pgv2 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.pgv2 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovHydroIEEE2.pgv3" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.pgv3 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.pgv3 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovHydroIEEE2.pgv4" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.pgv4 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.pgv4 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovHydroIEEE2.pgv5" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.pgv5 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.pgv5 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovHydroIEEE2.pgv6" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.pgv6 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.pgv6 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovHydroIEEE2.pmax" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.pmax = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.pmax = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovHydroIEEE2.pmin" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.pmin = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.pmin = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovHydroIEEE2.rperm" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.rperm = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.rperm = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovHydroIEEE2.rtemp" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.rtemp = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.rtemp = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovHydroIEEE2.tg" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.tg = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.tg = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovHydroIEEE2.tp" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.tp = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.tp = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovHydroIEEE2.tr" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.tr = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.tr = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovHydroIEEE2.tw" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.tw = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.tw = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovHydroIEEE2.uc" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.uc = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.uc = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovHydroIEEE2.uo" => {
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
