/// Turbine load controller model developed by WECC. This model represents a supervisory turbine load controller that acts to maintain turbine power at a set value by continuous adjustment of the turbine governor speed-load reference. This model is intended to represent slow reset 'outer loop' controllers managing the action of the turbine governor.
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct TurbLCFB1 {
    #[serde(flatten)]
    pub base: super::TurbineLoadControllerDynamics,
    /// Controller deadband (db). Typical value = 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db: Option<f64>,
    /// Maximum control error (Emax) (see parameter detail 4). Typical value = 0,02.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emax: Option<f64>,
    /// Frequency bias gain (Fb). Typical value = 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fb: Option<f64>,
    /// Frequency bias flag (Fbf). true = enable frequency bias false = disable frequency bias. Typical value = false.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fbf: Option<bool>,
    /// Maximum turbine speed/load reference bias (Irmax) (see parameter detail 3). Typical value = 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub irmax: Option<f64>,
    /// Integral gain (Ki). Typical value = 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ki: Option<f64>,
    /// Proportional gain (Kp). Typical value = 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kp: Option<f64>,
    /// Base for power values (MWbase) (> 0). Unit = MW.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mwbase: Option<f64>,
    /// Power controller flag (Pbf). true = enable load controller false = disable load controller. Typical value = false.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pbf: Option<bool>,
    /// Power controller setpoint (Pmwset) (see parameter detail 1). Unit = MW. Typical value = 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pmwset: Option<f64>,
    /// Type of turbine governor reference (Type). true = speed reference governor false = load reference governor. Typical value = true.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub speed_reference_governor: Option<bool>,
    /// Power transducer time constant (Tpelec) (>= 0). Typical value = 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tpelec: Option<f64>,
}
impl crate::base::CimElement for TurbLCFB1 {
    fn mrid(&self) -> &str { &self.base.base.base.id }
    fn type_name(&self) -> &'static str { "TurbLCFB1" }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn to_json_value(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or(serde_json::Value::Null)
    }
    fn to_block(&self) -> crate::base::RdfBlock {
        let mut block = self.base.to_block();
        block.type_name = "TurbLCFB1".to_string();
        if let Some(v) = self.db {
            block.fields.insert("TurbLCFB1.db".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.emax {
            block.fields.insert("TurbLCFB1.emax".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.fb {
            block.fields.insert("TurbLCFB1.fb".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.fbf {
            block.fields.insert("TurbLCFB1.fbf".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.irmax {
            block.fields.insert("TurbLCFB1.irmax".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.ki {
            block.fields.insert("TurbLCFB1.ki".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.kp {
            block.fields.insert("TurbLCFB1.kp".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.mwbase {
            block.fields.insert("TurbLCFB1.mwbase".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.pbf {
            block.fields.insert("TurbLCFB1.pbf".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.pmwset {
            block.fields.insert("TurbLCFB1.pmwset".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.speed_reference_governor {
            block.fields.insert("TurbLCFB1.speedReferenceGovernor".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.tpelec {
            block.fields.insert("TurbLCFB1.tpelec".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        block
    }
}

impl TurbLCFB1 {
    pub fn from_block(b: &crate::base::RdfBlock) -> Self {
        let mut obj = Self::default();
        obj.base.base.base.id.clone_from(&b.mrid);
        for (key, val) in &b.fields {
            match key.as_str() {
                "TurbLCFB1.db" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.db = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.db = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "TurbLCFB1.emax" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.emax = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.emax = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "TurbLCFB1.fb" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.fb = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.fb = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "TurbLCFB1.fbf" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.fbf = Some(sv.trim() == "true"); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.fbf = Some(sv.trim() == "true"); }
                        }
                        _ => {}
                    }
                }
                "TurbLCFB1.irmax" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.irmax = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.irmax = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "TurbLCFB1.ki" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.ki = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.ki = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "TurbLCFB1.kp" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.kp = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.kp = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "TurbLCFB1.mwbase" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.mwbase = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.mwbase = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "TurbLCFB1.pbf" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.pbf = Some(sv.trim() == "true"); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.pbf = Some(sv.trim() == "true"); }
                        }
                        _ => {}
                    }
                }
                "TurbLCFB1.pmwset" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.pmwset = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.pmwset = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "TurbLCFB1.speedReferenceGovernor" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.speed_reference_governor = Some(sv.trim() == "true"); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.speed_reference_governor = Some(sv.trim() == "true"); }
                        }
                        _ => {}
                    }
                }
                "TurbLCFB1.tpelec" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.tpelec = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.tpelec = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "TurbineLoadControllerDynamics.TurbineGovernorDynamics" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.base.turbine_governor_dynamics = Some(crate::base::MridRef { mrid: sv.clone() });
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
