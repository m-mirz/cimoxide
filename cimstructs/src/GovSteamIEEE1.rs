/// IEEE steam turbine governor model. Reference: IEEE Transactions on Power Apparatus and Systems, November/December 1973, Volume PAS-92, Number 6, Dynamic Models for Steam and Hydro Turbines in Power System Studies, page 1904.
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct GovSteamIEEE1 {
    #[serde(flatten)]
    pub base: super::TurbineGovernorDynamics,
    /// Governor gain (reciprocal of droop) (K) (> 0). Typical value = 25.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub k: Option<f64>,
    /// Fraction of HP shaft power after first boiler pass (K1). Typical value = 0,2.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub k1: Option<f64>,
    /// Fraction of LP shaft power after first boiler pass (K2). Typical value = 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub k2: Option<f64>,
    /// Fraction of HP shaft power after second boiler pass (K3). Typical value = 0,3.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub k3: Option<f64>,
    /// Fraction of LP shaft power after second boiler pass (K4). Typical value = 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub k4: Option<f64>,
    /// Fraction of HP shaft power after third boiler pass (K5). Typical value = 0,5.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub k5: Option<f64>,
    /// Fraction of LP shaft power after third boiler pass (K6). Typical value = 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub k6: Option<f64>,
    /// Fraction of HP shaft power after fourth boiler pass (K7). Typical value = 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub k7: Option<f64>,
    /// Fraction of LP shaft power after fourth boiler pass (K8). Typical value = 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub k8: Option<f64>,
    /// Base for power values (MWbase) (> 0). Unit = MW.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mwbase: Option<f64>,
    /// Maximum valve opening (Pmax) (> GovSteamIEEE1.pmin). Typical value = 1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pmax: Option<f64>,
    /// Minimum valve opening (Pmin) (>= 0 and < GovSteamIEEE1.pmax). Typical value = 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pmin: Option<f64>,
    /// Governor lag time constant (T1) (>= 0). Typical value = 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub t1: Option<f64>,
    /// Governor lead time constant (T2) (>= 0). Typical value = 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub t2: Option<f64>,
    /// Valve positioner time constant (T3) (> 0). Typical value = 0,1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub t3: Option<f64>,
    /// Inlet piping/steam bowl time constant (T4) (>= 0). Typical value = 0,3.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub t4: Option<f64>,
    /// Time constant of second boiler pass (T5) (>= 0). Typical value = 5.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub t5: Option<f64>,
    /// Time constant of third boiler pass (T6) (>= 0). Typical value = 0,5.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub t6: Option<f64>,
    /// Time constant of fourth boiler pass (T7) (>= 0). Typical value = 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub t7: Option<f64>,
    /// Maximum valve closing velocity (Uc) (< 0). Unit = PU / s. Typical value = -10.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uc: Option<f64>,
    /// Maximum valve opening velocity (Uo) (> 0). Unit = PU / s. Typical value = 1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uo: Option<f64>,
}
impl crate::base::CimElement for GovSteamIEEE1 {
    fn mrid(&self) -> &str { &self.base.base.base.id }
    fn type_name(&self) -> &'static str { "GovSteamIEEE1" }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn to_json_value(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or(serde_json::Value::Null)
    }
    fn to_block(&self) -> crate::base::RdfBlock {
        let mut block = self.base.to_block();
        block.type_name = "GovSteamIEEE1".to_string();
        if let Some(v) = self.k {
            block.fields.insert("GovSteamIEEE1.k".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.k1 {
            block.fields.insert("GovSteamIEEE1.k1".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.k2 {
            block.fields.insert("GovSteamIEEE1.k2".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.k3 {
            block.fields.insert("GovSteamIEEE1.k3".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.k4 {
            block.fields.insert("GovSteamIEEE1.k4".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.k5 {
            block.fields.insert("GovSteamIEEE1.k5".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.k6 {
            block.fields.insert("GovSteamIEEE1.k6".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.k7 {
            block.fields.insert("GovSteamIEEE1.k7".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.k8 {
            block.fields.insert("GovSteamIEEE1.k8".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.mwbase {
            block.fields.insert("GovSteamIEEE1.mwbase".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.pmax {
            block.fields.insert("GovSteamIEEE1.pmax".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.pmin {
            block.fields.insert("GovSteamIEEE1.pmin".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.t1 {
            block.fields.insert("GovSteamIEEE1.t1".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.t2 {
            block.fields.insert("GovSteamIEEE1.t2".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.t3 {
            block.fields.insert("GovSteamIEEE1.t3".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.t4 {
            block.fields.insert("GovSteamIEEE1.t4".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.t5 {
            block.fields.insert("GovSteamIEEE1.t5".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.t6 {
            block.fields.insert("GovSteamIEEE1.t6".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.t7 {
            block.fields.insert("GovSteamIEEE1.t7".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.uc {
            block.fields.insert("GovSteamIEEE1.uc".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.uo {
            block.fields.insert("GovSteamIEEE1.uo".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        block
    }
}

impl GovSteamIEEE1 {
    pub fn from_block(b: &crate::base::RdfBlock) -> Self {
        let mut obj = Self::default();
        obj.base.base.base.id.clone_from(&b.mrid);
        for (key, val) in &b.fields {
            match key.as_str() {
                "GovSteamIEEE1.k" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.k = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.k = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovSteamIEEE1.k1" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.k1 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.k1 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovSteamIEEE1.k2" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.k2 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.k2 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovSteamIEEE1.k3" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.k3 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.k3 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovSteamIEEE1.k4" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.k4 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.k4 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovSteamIEEE1.k5" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.k5 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.k5 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovSteamIEEE1.k6" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.k6 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.k6 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovSteamIEEE1.k7" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.k7 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.k7 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovSteamIEEE1.k8" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.k8 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.k8 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovSteamIEEE1.mwbase" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.mwbase = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.mwbase = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovSteamIEEE1.pmax" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.pmax = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.pmax = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovSteamIEEE1.pmin" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.pmin = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.pmin = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovSteamIEEE1.t1" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.t1 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.t1 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovSteamIEEE1.t2" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.t2 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.t2 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovSteamIEEE1.t3" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.t3 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.t3 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovSteamIEEE1.t4" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.t4 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.t4 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovSteamIEEE1.t5" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.t5 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.t5 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovSteamIEEE1.t6" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.t6 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.t6 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovSteamIEEE1.t7" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.t7 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.t7 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovSteamIEEE1.uc" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.uc = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.uc = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovSteamIEEE1.uo" => {
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
