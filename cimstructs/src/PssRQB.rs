/// Power system stabilizer type RQB. This power system stabilizer is intended to be used together with excitation system type ExcRQB, which is primarily used in nuclear or thermal generating units.
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct PssRQB {
    #[serde(flatten)]
    pub base: super::PowerSystemStabilizerDynamics,
    /// Lead lag gain (KDPM). Typical value = 0,185.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kdpm: Option<f64>,
    /// Speed input gain (Ki2). Typical value = 3,43.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ki2: Option<f64>,
    /// Electrical power input gain (Ki3). Typical value = -11,45.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ki3: Option<f64>,
    /// Mechanical power input gain (Ki4). Typical value = 11,86.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ki4: Option<f64>,
    /// Speed deadband (SIBV). Typical value = 0,006.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sibv: Option<f64>,
    /// Lead lag time constant (T4F) (>= 0). Typical value = 0,045.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub t4f: Option<f64>,
    /// Input time constant (T4M) (>= 0). Typical value = 5.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub t4m: Option<f64>,
    /// Speed time constant (T4MOM) (>= 0). Typical value = 1,27.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub t4mom: Option<f64>,
    /// Speed delay (TOMD) (>= 0). Typical value = 0,02.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tomd: Option<f64>,
    /// Speed time constant (TOMSL) (>= 0). Typical value = 0,04.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tomsl: Option<f64>,
}
impl crate::base::CimElement for PssRQB {
    fn mrid(&self) -> &str { &self.base.base.base.id }
    fn type_name(&self) -> &'static str { "PssRQB" }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn to_json_value(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or(serde_json::Value::Null)
    }
    fn to_block(&self) -> crate::base::RdfBlock {
        let mut block = self.base.to_block();
        block.type_name = "PssRQB".to_string();
        if let Some(v) = self.kdpm {
            block.fields.insert("PssRQB.kdpm".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.ki2 {
            block.fields.insert("PssRQB.ki2".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.ki3 {
            block.fields.insert("PssRQB.ki3".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.ki4 {
            block.fields.insert("PssRQB.ki4".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.sibv {
            block.fields.insert("PssRQB.sibv".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.t4f {
            block.fields.insert("PssRQB.t4f".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.t4m {
            block.fields.insert("PssRQB.t4m".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.t4mom {
            block.fields.insert("PssRQB.t4mom".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.tomd {
            block.fields.insert("PssRQB.tomd".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.tomsl {
            block.fields.insert("PssRQB.tomsl".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        block
    }
}

impl PssRQB {
    pub fn from_block(b: &crate::base::RdfBlock) -> Self {
        let mut obj = Self::default();
        obj.base.base.base.id.clone_from(&b.mrid);
        for (key, val) in &b.fields {
            match key.as_str() {
                "PssRQB.kdpm" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.kdpm = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.kdpm = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "PssRQB.ki2" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.ki2 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.ki2 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "PssRQB.ki3" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.ki3 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.ki3 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "PssRQB.ki4" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.ki4 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.ki4 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "PssRQB.sibv" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.sibv = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.sibv = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "PssRQB.t4f" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.t4f = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.t4f = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "PssRQB.t4m" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.t4m = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.t4m = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "PssRQB.t4mom" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.t4mom = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.t4mom = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "PssRQB.tomd" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.tomd = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.tomd = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "PssRQB.tomsl" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.tomsl = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.tomsl = Some(v); } }
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
