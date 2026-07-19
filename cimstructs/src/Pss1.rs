/// Italian PSS with three inputs (speed, frequency, power).
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct Pss1 {
    #[serde(flatten)]
    pub base: super::PowerSystemStabilizerDynamics,
    /// Frequency power input gain (KF). Typical value = 5.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kf: Option<f64>,
    /// Shaft speed power input gain (Komega). Typical value = 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub komega: Option<f64>,
    /// Electric power input gain (KPE). Typical value = 0,3.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kpe: Option<f64>,
    /// PSS gain (Ks). Typical value = 1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ks: Option<f64>,
    /// Minimum power PSS enabling (Pmin). Typical value = 0,25.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pmin: Option<f64>,
    /// Lead/lag time constant (T10) (>= 0). Typical value = 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub t10: Option<f64>,
    /// Washout (T5) (>= 0). Typical value = 3,5.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub t5: Option<f64>,
    /// Filter time constant (T6) (>= 0). Typical value = 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub t6: Option<f64>,
    /// Lead/lag time constant (T7) (>= 0). If = 0, both blocks are bypassed. Typical value = 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub t7: Option<f64>,
    /// Lead/lag time constant (T8) (>= 0). Typical value = 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub t8: Option<f64>,
    /// Lead/lag time constant (T9) (>= 0). If = 0, both blocks are bypassed. Typical value = 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub t9: Option<f64>,
    /// Electric power filter time constant (TPE) (>= 0). Typical value = 0,05.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tpe: Option<f64>,
    /// Signal selector (VADAT). true = closed (generator power is greater than Pmin) false = open (Pe is smaller than Pmin). Typical value = true.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vadat: Option<bool>,
    /// Stabilizer output maximum limit (VSMN). Typical value = -0,06.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vsmn: Option<f64>,
    /// Stabilizer output minimum limit (VSMX). Typical value = 0,06.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vsmx: Option<f64>,
}
impl crate::base::CimElement for Pss1 {
    fn mrid(&self) -> &str { &self.base.base.base.id }
    fn type_name(&self) -> &'static str { "Pss1" }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn to_json_value(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or(serde_json::Value::Null)
    }
    fn to_block(&self) -> crate::base::RdfBlock {
        let mut block = self.base.to_block();
        block.type_name = "Pss1".to_string();
        if let Some(v) = self.kf {
            block.fields.insert("Pss1.kf".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.komega {
            block.fields.insert("Pss1.komega".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.kpe {
            block.fields.insert("Pss1.kpe".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.ks {
            block.fields.insert("Pss1.ks".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.pmin {
            block.fields.insert("Pss1.pmin".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.t10 {
            block.fields.insert("Pss1.t10".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.t5 {
            block.fields.insert("Pss1.t5".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.t6 {
            block.fields.insert("Pss1.t6".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.t7 {
            block.fields.insert("Pss1.t7".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.t8 {
            block.fields.insert("Pss1.t8".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.t9 {
            block.fields.insert("Pss1.t9".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.tpe {
            block.fields.insert("Pss1.tpe".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.vadat {
            block.fields.insert("Pss1.vadat".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.vsmn {
            block.fields.insert("Pss1.vsmn".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.vsmx {
            block.fields.insert("Pss1.vsmx".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        block
    }
}

impl Pss1 {
    pub fn from_block(b: &crate::base::RdfBlock) -> Self {
        let mut obj = Self::default();
        obj.base.base.base.id.clone_from(&b.mrid);
        for (key, val) in &b.fields {
            match key.as_str() {
                "Pss1.kf" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.kf = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.kf = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "Pss1.komega" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.komega = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.komega = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "Pss1.kpe" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.kpe = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.kpe = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "Pss1.ks" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.ks = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.ks = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "Pss1.pmin" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.pmin = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.pmin = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "Pss1.t10" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.t10 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.t10 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "Pss1.t5" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.t5 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.t5 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "Pss1.t6" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.t6 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.t6 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "Pss1.t7" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.t7 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.t7 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "Pss1.t8" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.t8 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.t8 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "Pss1.t9" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.t9 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.t9 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "Pss1.tpe" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.tpe = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.tpe = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "Pss1.vadat" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.vadat = Some(sv.trim() == "true"); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.vadat = Some(sv.trim() == "true"); }
                        }
                        _ => {}
                    }
                }
                "Pss1.vsmn" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.vsmn = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.vsmn = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "Pss1.vsmx" => {
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
