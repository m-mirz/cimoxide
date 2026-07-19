/// Hydro turbine and governor. Represents plants with straightforward penstock configurations and 'three term' electro-hydraulic governors (i.e. WoodwardTM electronic). [Footnote: Woodward electronic governors are an example of suitable products available commercially. This information is given for the convenience of users of this document and does not constitute an endorsement by IEC of these products.]
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct GovHydroPID2 {
    #[serde(flatten)]
    pub base: super::TurbineGovernorDynamics,
    /// Factor multiplying Tw (Atw). Typical value = 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub atw: Option<f64>,
    /// Turbine damping factor (D). Unit = delta P / delta speed. Typical value = 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d: Option<f64>,
    /// Feedback signal type flag (Flag). true = use gate position feedback signal false = use Pe.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feedback_signal: Option<bool>,
    /// Gate opening at speed no load (G0). Typical value = 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub g0: Option<f64>,
    /// Intermediate gate opening (G1). Typical value = 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub g1: Option<f64>,
    /// Intermediate gate opening (G2). Typical value = 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub g2: Option<f64>,
    /// Maximum gate opening (Gmax) (> GovHydroPID2.gmin). Typical value = 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gmax: Option<f64>,
    /// Minimum gate opening (Gmin) (> GovHydroPID2.gmax). Typical value = 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gmin: Option<f64>,
    /// Derivative gain (Kd). Typical value = 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kd: Option<f64>,
    /// Reset gain (Ki). Unit = PU/s. Typical value = 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ki: Option<f64>,
    /// Proportional gain (Kp). Typical value = 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kp: Option<f64>,
    /// Base for power values (MWbase) (>0). Unit = MW.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mwbase: Option<f64>,
    /// Power at gate opening G1 (P1). Typical value = 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub p1: Option<f64>,
    /// Power at gate opening G2 (P2). Typical value = 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub p2: Option<f64>,
    /// Power at full opened gate (P3). Typical value = 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub p3: Option<f64>,
    /// Permanent drop (Rperm). Typical value = 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rperm: Option<f64>,
    /// Controller time constant (Ta) (>= 0). Typical value = 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ta: Option<f64>,
    /// Gate servo time constant (Tb) (> 0).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tb: Option<f64>,
    /// Speed detector time constant (Treg) (>= 0). Typical value = 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub treg: Option<f64>,
    /// Water inertia time constant (Tw) (>= 0). Typical value = 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tw: Option<f64>,
    /// Maximum gate opening velocity (Velmax) (< GovHydroPID2.velmin). Unit = PU / s. Typical value = 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub velmax: Option<f64>,
    /// Maximum gate closing velocity (Velmin) (> GovHydroPID2.velmax). Unit = PU / s. Typical value = 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub velmin: Option<f64>,
}
impl crate::base::CimElement for GovHydroPID2 {
    fn mrid(&self) -> &str { &self.base.base.base.id }
    fn type_name(&self) -> &'static str { "GovHydroPID2" }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn to_json_value(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or(serde_json::Value::Null)
    }
    fn to_block(&self) -> crate::base::RdfBlock {
        let mut block = self.base.to_block();
        block.type_name = "GovHydroPID2".to_string();
        if let Some(v) = self.atw {
            block.fields.insert("GovHydroPID2.atw".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.d {
            block.fields.insert("GovHydroPID2.d".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.feedback_signal {
            block.fields.insert("GovHydroPID2.feedbackSignal".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.g0 {
            block.fields.insert("GovHydroPID2.g0".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.g1 {
            block.fields.insert("GovHydroPID2.g1".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.g2 {
            block.fields.insert("GovHydroPID2.g2".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.gmax {
            block.fields.insert("GovHydroPID2.gmax".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.gmin {
            block.fields.insert("GovHydroPID2.gmin".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.kd {
            block.fields.insert("GovHydroPID2.kd".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.ki {
            block.fields.insert("GovHydroPID2.ki".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.kp {
            block.fields.insert("GovHydroPID2.kp".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.mwbase {
            block.fields.insert("GovHydroPID2.mwbase".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.p1 {
            block.fields.insert("GovHydroPID2.p1".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.p2 {
            block.fields.insert("GovHydroPID2.p2".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.p3 {
            block.fields.insert("GovHydroPID2.p3".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.rperm {
            block.fields.insert("GovHydroPID2.rperm".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.ta {
            block.fields.insert("GovHydroPID2.ta".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.tb {
            block.fields.insert("GovHydroPID2.tb".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.treg {
            block.fields.insert("GovHydroPID2.treg".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.tw {
            block.fields.insert("GovHydroPID2.tw".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.velmax {
            block.fields.insert("GovHydroPID2.velmax".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.velmin {
            block.fields.insert("GovHydroPID2.velmin".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        block
    }
}

impl GovHydroPID2 {
    pub fn from_block(b: &crate::base::RdfBlock) -> Self {
        let mut obj = Self::default();
        obj.base.base.base.id.clone_from(&b.mrid);
        for (key, val) in &b.fields {
            match key.as_str() {
                "GovHydroPID2.atw" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.atw = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.atw = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovHydroPID2.d" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.d = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.d = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovHydroPID2.feedbackSignal" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.feedback_signal = Some(sv.trim() == "true"); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.feedback_signal = Some(sv.trim() == "true"); }
                        }
                        _ => {}
                    }
                }
                "GovHydroPID2.g0" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.g0 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.g0 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovHydroPID2.g1" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.g1 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.g1 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovHydroPID2.g2" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.g2 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.g2 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovHydroPID2.gmax" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.gmax = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.gmax = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovHydroPID2.gmin" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.gmin = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.gmin = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovHydroPID2.kd" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.kd = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.kd = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovHydroPID2.ki" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.ki = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.ki = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovHydroPID2.kp" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.kp = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.kp = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovHydroPID2.mwbase" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.mwbase = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.mwbase = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovHydroPID2.p1" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.p1 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.p1 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovHydroPID2.p2" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.p2 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.p2 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovHydroPID2.p3" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.p3 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.p3 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovHydroPID2.rperm" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.rperm = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.rperm = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovHydroPID2.ta" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.ta = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.ta = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovHydroPID2.tb" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.tb = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.tb = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovHydroPID2.treg" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.treg = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.treg = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovHydroPID2.tw" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.tw = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.tw = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovHydroPID2.velmax" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.velmax = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.velmax = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovHydroPID2.velmin" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.velmin = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.velmin = Some(v); } }
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
