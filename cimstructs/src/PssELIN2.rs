/// Power system stabilizer typically associated with ExcELIN2 (though PssIEEE2B or Pss2B can also be used).
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct PssELIN2 {
    #[serde(flatten)]
    pub base: super::PowerSystemStabilizerDynamics,
    /// Coefficient (a_PSS). Typical value = 0,1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apss: Option<f64>,
    /// Gain (Ks1). Typical value = 1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ks1: Option<f64>,
    /// Gain (Ks2). Typical value = 0,1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ks2: Option<f64>,
    /// Coefficient (p_PSS) (>= 0 and <= 4). Typical value = 0,1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ppss: Option<f64>,
    /// PSS limiter (psslim). Typical value = 0,1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub psslim: Option<f64>,
    /// Time constant (Ts1) (>= 0). Typical value = 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ts1: Option<f64>,
    /// Time constant (Ts2) (>= 0). Typical value = 1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ts2: Option<f64>,
    /// Time constant (Ts3) (>= 0). Typical value = 1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ts3: Option<f64>,
    /// Time constant (Ts4) (>= 0). Typical value = 0,1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ts4: Option<f64>,
    /// Time constant (Ts5) (>= 0). Typical value = 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ts5: Option<f64>,
    /// Time constant (Ts6) (>= 0). Typical value = 1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ts6: Option<f64>,
}
impl crate::base::CimElement for PssELIN2 {
    fn mrid(&self) -> &str { &self.base.base.base.id }
    fn type_name(&self) -> &'static str { "PssELIN2" }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn to_json_value(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or(serde_json::Value::Null)
    }
    fn to_block(&self) -> crate::base::RdfBlock {
        let mut block = self.base.to_block();
        block.type_name = "PssELIN2".to_string();
        if let Some(v) = self.apss {
            block.fields.insert("PssELIN2.apss".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.ks1 {
            block.fields.insert("PssELIN2.ks1".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.ks2 {
            block.fields.insert("PssELIN2.ks2".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.ppss {
            block.fields.insert("PssELIN2.ppss".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.psslim {
            block.fields.insert("PssELIN2.psslim".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.ts1 {
            block.fields.insert("PssELIN2.ts1".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.ts2 {
            block.fields.insert("PssELIN2.ts2".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.ts3 {
            block.fields.insert("PssELIN2.ts3".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.ts4 {
            block.fields.insert("PssELIN2.ts4".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.ts5 {
            block.fields.insert("PssELIN2.ts5".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.ts6 {
            block.fields.insert("PssELIN2.ts6".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        block
    }
}

impl PssELIN2 {
    pub fn from_block(b: &crate::base::RdfBlock) -> Self {
        let mut obj = Self::default();
        obj.base.base.base.id.clone_from(&b.mrid);
        for (key, val) in &b.fields {
            match key.as_str() {
                "PssELIN2.apss" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.apss = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.apss = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "PssELIN2.ks1" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.ks1 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.ks1 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "PssELIN2.ks2" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.ks2 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.ks2 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "PssELIN2.ppss" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.ppss = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.ppss = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "PssELIN2.psslim" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.psslim = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.psslim = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "PssELIN2.ts1" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.ts1 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.ts1 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "PssELIN2.ts2" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.ts2 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.ts2 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "PssELIN2.ts3" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.ts3 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.ts3 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "PssELIN2.ts4" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.ts4 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.ts4 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "PssELIN2.ts5" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.ts5 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.ts5 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "PssELIN2.ts6" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.ts6 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.ts6 = Some(v); } }
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
