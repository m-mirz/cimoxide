/// Slovakian excitation system. UEL and secondary voltage control are included in this model. When this model is used, there cannot be a separate underexcitation limiter or VAr controller model.
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct ExcSK {
    #[serde(flatten)]
    pub base: super::ExcitationSystemDynamics,
    /// Field voltage clipping upper level limit (Efdmax) (> ExcSK.efdmin).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub efdmax: Option<f64>,
    /// Field voltage clipping lower level limit (Efdmin) (< ExcSK.efdmax).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub efdmin: Option<f64>,
    /// Maximum field voltage output (Emax) (> ExcSK.emin). Typical value = 20.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emax: Option<f64>,
    /// Minimum field voltage output (Emin) (< ExcSK.emax). Typical value = -20.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emin: Option<f64>,
    /// Gain (K). Typical value = 1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub k: Option<f64>,
    /// Parameter of underexcitation limit (K1). Typical value = 0,1364.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub k1: Option<f64>,
    /// Parameter of underexcitation limit (K2). Typical value = -0,3861.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub k2: Option<f64>,
    /// PI controller gain (Kc). Typical value = 70.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kc: Option<f64>,
    /// Rectifier regulation factor (Kce). Typical value = 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kce: Option<f64>,
    /// Exciter internal reactance (Kd). Typical value = 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kd: Option<f64>,
    /// P controller gain (Kgob). Typical value = 10.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kgob: Option<f64>,
    /// PI controller gain (Kp). Typical value = 1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kp: Option<f64>,
    /// PI controller gain of integral component (Kqi). Typical value = 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kqi: Option<f64>,
    /// Rate of rise of the reactive power (Kqob).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kqob: Option<f64>,
    /// PI controller gain (Kqp). Typical value = 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kqp: Option<f64>,
    /// Deadband of reactive power (nq). Determines the range of sensitivity. Typical value = 0,001.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nq: Option<f64>,
    /// Secondary voltage control state (Qc_on_off). true = secondary voltage control is on false = secondary voltage control is off. Typical value = false.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qconoff: Option<bool>,
    /// Desired value (setpoint) of reactive power, manual setting (Qz).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qz: Option<f64>,
    /// Selector to apply automatic calculation in secondary controller model (remote). true = automatic calculation is activated false = manual set is active; the use of desired value of reactive power (Qz) is required. Typical value = true.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remote: Option<bool>,
    /// Apparent power of the unit (Sbase) (> 0). Unit = MVA. Typical value = 259.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sbase: Option<f64>,
    /// PI controller phase lead time constant (Tc) (>= 0). Typical value = 8.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tc: Option<f64>,
    /// Time constant of gain block (Te) (>= 0). Typical value = 0,1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub te: Option<f64>,
    /// PI controller phase lead time constant (Ti) (>= 0). Typical value = 2.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ti: Option<f64>,
    /// Time constant (Tp) (>= 0). Typical value = 0,1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tp: Option<f64>,
    /// Voltage transducer time constant (Tr) (>= 0). Typical value = 0,01.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tr: Option<f64>,
    /// Maximum error (UImax) (> ExcSK.uimin). Typical value = 10.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uimax: Option<f64>,
    /// Minimum error (UImin) (< ExcSK.uimax). Typical value = -10.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uimin: Option<f64>,
    /// Maximum controller output (URmax) (> ExcSK.urmin). Typical value = 10.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub urmax: Option<f64>,
    /// Minimum controller output (URmin) (< ExcSK.urmax). Typical value = -10.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub urmin: Option<f64>,
    /// Maximum terminal voltage input (Vtmax) (> ExcSK.vtmin). Determines the range of voltage deadband. Typical value = 1,05.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vtmax: Option<f64>,
    /// Minimum terminal voltage input (Vtmin) (< ExcSK.vtmax). Determines the range of voltage deadband. Typical value = 0,95.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vtmin: Option<f64>,
    /// Maximum output (Yp). Typical value = 1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub yp: Option<f64>,
}
impl crate::base::CimElement for ExcSK {
    fn mrid(&self) -> &str { &self.base.base.base.id }
    fn type_name(&self) -> &'static str { "ExcSK" }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn to_json_value(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or(serde_json::Value::Null)
    }
    fn to_block(&self) -> crate::base::RdfBlock {
        let mut block = self.base.to_block();
        block.type_name = "ExcSK".to_string();
        if let Some(v) = self.efdmax {
            block.fields.insert("ExcSK.efdmax".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.efdmin {
            block.fields.insert("ExcSK.efdmin".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.emax {
            block.fields.insert("ExcSK.emax".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.emin {
            block.fields.insert("ExcSK.emin".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.k {
            block.fields.insert("ExcSK.k".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.k1 {
            block.fields.insert("ExcSK.k1".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.k2 {
            block.fields.insert("ExcSK.k2".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.kc {
            block.fields.insert("ExcSK.kc".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.kce {
            block.fields.insert("ExcSK.kce".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.kd {
            block.fields.insert("ExcSK.kd".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.kgob {
            block.fields.insert("ExcSK.kgob".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.kp {
            block.fields.insert("ExcSK.kp".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.kqi {
            block.fields.insert("ExcSK.kqi".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.kqob {
            block.fields.insert("ExcSK.kqob".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.kqp {
            block.fields.insert("ExcSK.kqp".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.nq {
            block.fields.insert("ExcSK.nq".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.qconoff {
            block.fields.insert("ExcSK.qconoff".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.qz {
            block.fields.insert("ExcSK.qz".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.remote {
            block.fields.insert("ExcSK.remote".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.sbase {
            block.fields.insert("ExcSK.sbase".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.tc {
            block.fields.insert("ExcSK.tc".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.te {
            block.fields.insert("ExcSK.te".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.ti {
            block.fields.insert("ExcSK.ti".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.tp {
            block.fields.insert("ExcSK.tp".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.tr {
            block.fields.insert("ExcSK.tr".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.uimax {
            block.fields.insert("ExcSK.uimax".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.uimin {
            block.fields.insert("ExcSK.uimin".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.urmax {
            block.fields.insert("ExcSK.urmax".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.urmin {
            block.fields.insert("ExcSK.urmin".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.vtmax {
            block.fields.insert("ExcSK.vtmax".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.vtmin {
            block.fields.insert("ExcSK.vtmin".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.yp {
            block.fields.insert("ExcSK.yp".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        block
    }
}

impl ExcSK {
    pub fn from_block(b: &crate::base::RdfBlock) -> Self {
        let mut obj = Self::default();
        obj.base.base.base.id.clone_from(&b.mrid);
        for (key, val) in &b.fields {
            match key.as_str() {
                "ExcSK.efdmax" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.efdmax = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.efdmax = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcSK.efdmin" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.efdmin = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.efdmin = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcSK.emax" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.emax = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.emax = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcSK.emin" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.emin = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.emin = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcSK.k" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.k = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.k = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcSK.k1" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.k1 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.k1 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcSK.k2" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.k2 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.k2 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcSK.kc" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.kc = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.kc = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcSK.kce" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.kce = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.kce = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcSK.kd" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.kd = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.kd = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcSK.kgob" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.kgob = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.kgob = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcSK.kp" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.kp = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.kp = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcSK.kqi" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.kqi = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.kqi = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcSK.kqob" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.kqob = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.kqob = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcSK.kqp" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.kqp = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.kqp = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcSK.nq" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.nq = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.nq = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcSK.qconoff" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.qconoff = Some(sv.trim() == "true"); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.qconoff = Some(sv.trim() == "true"); }
                        }
                        _ => {}
                    }
                }
                "ExcSK.qz" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.qz = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.qz = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcSK.remote" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.remote = Some(sv.trim() == "true"); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.remote = Some(sv.trim() == "true"); }
                        }
                        _ => {}
                    }
                }
                "ExcSK.sbase" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.sbase = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.sbase = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcSK.tc" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.tc = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.tc = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcSK.te" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.te = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.te = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcSK.ti" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.ti = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.ti = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcSK.tp" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.tp = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.tp = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcSK.tr" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.tr = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.tr = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcSK.uimax" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.uimax = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.uimax = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcSK.uimin" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.uimin = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.uimin = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcSK.urmax" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.urmax = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.urmax = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcSK.urmin" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.urmin = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.urmin = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcSK.vtmax" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.vtmax = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.vtmax = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcSK.vtmin" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.vtmin = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.vtmin = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcSK.yp" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.yp = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.yp = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcitationSystemDynamics.SynchronousMachineDynamics" => {
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
