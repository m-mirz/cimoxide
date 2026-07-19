/// Combined static load and induction motor load effects. The dynamics of the motor are simplified by linearizing the induction machine equations.
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct LoadComposite {
    #[serde(flatten)]
    pub base: super::LoadDynamics,
    /// Active load-frequency dependence index (dynamic) (Epfd). Typical value = 1,5.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub epfd: Option<f64>,
    /// Active load-frequency dependence index (static) (Epfs). Typical value = 1,5.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub epfs: Option<f64>,
    /// Active load-voltage dependence index (dynamic) (Epvd). Typical value = 0,7.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub epvd: Option<f64>,
    /// Active load-voltage dependence index (static) (Epvs). Typical value = 0,7.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub epvs: Option<f64>,
    /// Reactive load-frequency dependence index (dynamic) (Eqfd). Typical value = 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eqfd: Option<f64>,
    /// Reactive load-frequency dependence index (static) (Eqfs). Typical value = 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eqfs: Option<f64>,
    /// Reactive load-voltage dependence index (dynamic) (Eqvd). Typical value = 2.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eqvd: Option<f64>,
    /// Reactive load-voltage dependence index (static) (Eqvs). Typical value = 2.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eqvs: Option<f64>,
    /// Inertia constant (H) (>= 0). Typical value = 2,5.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub h: Option<f64>,
    /// Loading factor (Lfac). The ratio of initial P to motor MVA base. Typical value = 0,8.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lfac: Option<f64>,
    /// Fraction of constant-power load to be represented by this motor model (PFRAC) (>= 0,0 and <= 1,0). Typical value = 0,5.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pfrac: Option<f64>,
}
impl crate::base::CimElement for LoadComposite {
    fn mrid(&self) -> &str { &self.base.base.id }
    fn type_name(&self) -> &'static str { "LoadComposite" }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn to_json_value(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or(serde_json::Value::Null)
    }
    fn to_block(&self) -> crate::base::RdfBlock {
        let mut block = self.base.to_block();
        block.type_name = "LoadComposite".to_string();
        if let Some(v) = self.epfd {
            block.fields.insert("LoadComposite.epfd".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.epfs {
            block.fields.insert("LoadComposite.epfs".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.epvd {
            block.fields.insert("LoadComposite.epvd".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.epvs {
            block.fields.insert("LoadComposite.epvs".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.eqfd {
            block.fields.insert("LoadComposite.eqfd".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.eqfs {
            block.fields.insert("LoadComposite.eqfs".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.eqvd {
            block.fields.insert("LoadComposite.eqvd".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.eqvs {
            block.fields.insert("LoadComposite.eqvs".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.h {
            block.fields.insert("LoadComposite.h".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.lfac {
            block.fields.insert("LoadComposite.lfac".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.pfrac {
            block.fields.insert("LoadComposite.pfrac".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        block
    }
}

impl LoadComposite {
    pub fn from_block(b: &crate::base::RdfBlock) -> Self {
        let mut obj = Self::default();
        obj.base.base.id.clone_from(&b.mrid);
        for (key, val) in &b.fields {
            match key.as_str() {
                "LoadComposite.epfd" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.epfd = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.epfd = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "LoadComposite.epfs" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.epfs = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.epfs = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "LoadComposite.epvd" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.epvd = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.epvd = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "LoadComposite.epvs" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.epvs = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.epvs = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "LoadComposite.eqfd" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.eqfd = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.eqfd = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "LoadComposite.eqfs" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.eqfs = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.eqfs = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "LoadComposite.eqvd" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.eqvd = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.eqvd = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "LoadComposite.eqvs" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.eqvs = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.eqvs = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "LoadComposite.h" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.h = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.h = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "LoadComposite.lfac" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.lfac = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.lfac = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "LoadComposite.pfrac" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.pfrac = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.pfrac = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "IdentifiedObject.description" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.base.base.description = sv.clone(); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.base.base.description = sv.clone(); }
                        }
                        _ => {}
                    }
                }
                "IdentifiedObject.energyIdentCodeEic" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.base.base.energy_ident_code_eic = sv.clone(); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.base.base.energy_ident_code_eic = sv.clone(); }
                        }
                        _ => {}
                    }
                }
                "IdentifiedObject.mRID" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.base.base.m_rid = sv.clone(); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.base.base.m_rid = sv.clone(); }
                        }
                        _ => {}
                    }
                }
                "IdentifiedObject.name" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.base.base.name = sv.clone(); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.base.base.name = sv.clone(); }
                        }
                        _ => {}
                    }
                }
                "IdentifiedObject.shortName" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.base.base.short_name = sv.clone(); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.base.base.short_name = sv.clone(); }
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
