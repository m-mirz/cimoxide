/// Detailed Italian PSS.
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct Pss5 {
    #[serde(flatten)]
    pub base: super::PowerSystemStabilizerDynamics,
    /// Selector for second washout enabling (CTW2). true = second washout filter is bypassed false = second washout filter in use. Typical value = true.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ctw2: Option<bool>,
    /// Stabilizer output deadband (DEADBAND). Typical value = 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deadband: Option<f64>,
    /// Selector for frequency/shaft speed input (isFreq). true = speed (same meaning as InputSignaKind.rotorSpeed) false = frequency (same meaning as InputSignalKind.busFrequency). Typical value = true (same meaning as InputSignalKind.rotorSpeed).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub isfreq: Option<bool>,
    /// Frequency/shaft speed input gain (KF). Typical value = 5.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kf: Option<f64>,
    /// Electric power input gain (KPE). Typical value = 0,3.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kpe: Option<f64>,
    /// PSS gain (KPSS). Typical value = 1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kpss: Option<f64>,
    /// Minimum power PSS enabling (Pmin). Typical value = 0,25.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pmin: Option<f64>,
    /// Lead/lag time constant (TL1) (>= 0). Typical value = 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tl1: Option<f64>,
    /// Lead/lag time constant (TL2) (>= 0). If = 0, both blocks are bypassed. Typical value = 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tl2: Option<f64>,
    /// Lead/lag time constant (TL3) (>= 0). Typical value = 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tl3: Option<f64>,
    /// Lead/lag time constant (TL4) (>= 0). If = 0, both blocks are bypassed. Typical value = 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tl4: Option<f64>,
    /// Electric power filter time constant (TPE) (>= 0). Typical value = 0,05.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tpe: Option<f64>,
    /// First washout (TW1) (>= 0). Typical value = 3,5.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tw1: Option<f64>,
    /// Second washout (TW2) (>= 0). Typical value = 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tw2: Option<f64>,
    /// Signal selector (VadAtt). true = closed (generator power is greater than Pmin) false = open (Pe is smaller than Pmin). Typical value = true.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vadat: Option<bool>,
    /// Stabilizer output maximum limit (VSMN). Typical value = -0,1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vsmn: Option<f64>,
    /// Stabilizer output minimum limit (VSMX). Typical value = 0,1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vsmx: Option<f64>,
}
impl crate::base::CimElement for Pss5 {
    fn mrid(&self) -> &str { &self.base.base.base.id }
    fn type_name(&self) -> &'static str { "Pss5" }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn to_json_value(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or(serde_json::Value::Null)
    }
    fn to_block(&self) -> crate::base::RdfBlock {
        let mut block = self.base.to_block();
        block.type_name = "Pss5".to_string();
        if let Some(v) = self.ctw2 {
            block.fields.insert("Pss5.ctw2".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.deadband {
            block.fields.insert("Pss5.deadband".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.isfreq {
            block.fields.insert("Pss5.isfreq".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.kf {
            block.fields.insert("Pss5.kf".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.kpe {
            block.fields.insert("Pss5.kpe".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.kpss {
            block.fields.insert("Pss5.kpss".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.pmin {
            block.fields.insert("Pss5.pmin".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.tl1 {
            block.fields.insert("Pss5.tl1".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.tl2 {
            block.fields.insert("Pss5.tl2".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.tl3 {
            block.fields.insert("Pss5.tl3".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.tl4 {
            block.fields.insert("Pss5.tl4".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.tpe {
            block.fields.insert("Pss5.tpe".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.tw1 {
            block.fields.insert("Pss5.tw1".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.tw2 {
            block.fields.insert("Pss5.tw2".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.vadat {
            block.fields.insert("Pss5.vadat".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.vsmn {
            block.fields.insert("Pss5.vsmn".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.vsmx {
            block.fields.insert("Pss5.vsmx".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        block
    }
}

impl Pss5 {
    pub fn from_block(b: &crate::base::RdfBlock) -> Self {
        let mut obj = Self::default();
        obj.base.base.base.id.clone_from(&b.mrid);
        for (key, val) in &b.fields {
            match key.as_str() {
                "Pss5.ctw2" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.ctw2 = Some(sv.trim() == "true"); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.ctw2 = Some(sv.trim() == "true"); }
                        }
                        _ => {}
                    }
                }
                "Pss5.deadband" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.deadband = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.deadband = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "Pss5.isfreq" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.isfreq = Some(sv.trim() == "true"); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.isfreq = Some(sv.trim() == "true"); }
                        }
                        _ => {}
                    }
                }
                "Pss5.kf" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.kf = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.kf = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "Pss5.kpe" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.kpe = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.kpe = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "Pss5.kpss" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.kpss = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.kpss = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "Pss5.pmin" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.pmin = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.pmin = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "Pss5.tl1" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.tl1 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.tl1 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "Pss5.tl2" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.tl2 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.tl2 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "Pss5.tl3" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.tl3 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.tl3 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "Pss5.tl4" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.tl4 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.tl4 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "Pss5.tpe" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.tpe = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.tpe = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "Pss5.tw1" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.tw1 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.tw1 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "Pss5.tw2" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.tw2 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.tw2 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "Pss5.vadat" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.vadat = Some(sv.trim() == "true"); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.vadat = Some(sv.trim() == "true"); }
                        }
                        _ => {}
                    }
                }
                "Pss5.vsmn" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.vsmn = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.vsmn = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "Pss5.vsmx" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.vsmx = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.vsmx = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "PowerSystemStabilizerDynamics.ExcitationSystemDynamics" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.base.excitation_system_dynamics = Some(crate::base::MridRef { mrid: sv.clone() });
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
