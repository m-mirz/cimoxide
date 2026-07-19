/// Basic hydro turbine governor.
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct GovHydro1 {
    #[serde(flatten)]
    pub base: super::TurbineGovernorDynamics,
    /// Turbine gain (At) (> 0). Typical value = 1,2.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub at: Option<f64>,
    /// Turbine damping factor (Dturb) (>= 0). Typical value = 0,5.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dturb: Option<f64>,
    /// Maximum gate opening (Gmax) (> 0 and > GovHydro.gmin). Typical value = 1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gmax: Option<f64>,
    /// Minimum gate opening (Gmin) (>= 0 and < GovHydro1.gmax). Typical value = 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gmin: Option<f64>,
    /// Turbine nominal head (hdam). Typical value = 1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hdam: Option<f64>,
    /// Base for power values (MWbase) (> 0). Unit = MW.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mwbase: Option<f64>,
    /// No-load flow at nominal head (qnl) (>= 0). Typical value = 0,08.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qnl: Option<f64>,
    /// Permanent droop (R) (> 0). Typical value = 0,04.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rperm: Option<f64>,
    /// Temporary droop (r) (> GovHydro1.rperm). Typical value = 0,3.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rtemp: Option<f64>,
    /// Filter time constant (Tf) (> 0). Typical value = 0,05.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tf: Option<f64>,
    /// Gate servo time constant (Tg) (> 0). Typical value = 0,5.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tg: Option<f64>,
    /// Washout time constant (Tr) (> 0). Typical value = 5.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tr: Option<f64>,
    /// Water inertia time constant (Tw) (> 0). Typical value = 1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tw: Option<f64>,
    /// Maximum gate velocity (Vlem) (> 0). Typical value = 0,2.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub velm: Option<f64>,
}
impl crate::base::CimElement for GovHydro1 {
    fn mrid(&self) -> &str { &self.base.base.base.id }
    fn type_name(&self) -> &'static str { "GovHydro1" }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn to_json_value(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or(serde_json::Value::Null)
    }
    fn to_block(&self) -> crate::base::RdfBlock {
        let mut block = self.base.to_block();
        block.type_name = "GovHydro1".to_string();
        if let Some(v) = self.at {
            block.fields.insert("GovHydro1.at".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.dturb {
            block.fields.insert("GovHydro1.dturb".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.gmax {
            block.fields.insert("GovHydro1.gmax".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.gmin {
            block.fields.insert("GovHydro1.gmin".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.hdam {
            block.fields.insert("GovHydro1.hdam".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.mwbase {
            block.fields.insert("GovHydro1.mwbase".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.qnl {
            block.fields.insert("GovHydro1.qnl".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.rperm {
            block.fields.insert("GovHydro1.rperm".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.rtemp {
            block.fields.insert("GovHydro1.rtemp".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.tf {
            block.fields.insert("GovHydro1.tf".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.tg {
            block.fields.insert("GovHydro1.tg".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.tr {
            block.fields.insert("GovHydro1.tr".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.tw {
            block.fields.insert("GovHydro1.tw".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.velm {
            block.fields.insert("GovHydro1.velm".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        block
    }
}

impl GovHydro1 {
    pub fn from_block(b: &crate::base::RdfBlock) -> Self {
        let mut obj = Self::default();
        obj.base.base.base.id.clone_from(&b.mrid);
        for (key, val) in &b.fields {
            match key.as_str() {
                "GovHydro1.at" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.at = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.at = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovHydro1.dturb" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.dturb = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.dturb = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovHydro1.gmax" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.gmax = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.gmax = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovHydro1.gmin" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.gmin = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.gmin = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovHydro1.hdam" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.hdam = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.hdam = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovHydro1.mwbase" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.mwbase = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.mwbase = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovHydro1.qnl" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.qnl = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.qnl = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovHydro1.rperm" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.rperm = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.rperm = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovHydro1.rtemp" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.rtemp = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.rtemp = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovHydro1.tf" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.tf = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.tf = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovHydro1.tg" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.tg = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.tg = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovHydro1.tr" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.tr = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.tr = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovHydro1.tw" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.tw = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.tw = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovHydro1.velm" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.velm = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.velm = Some(v); } }
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
