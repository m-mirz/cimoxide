/// Generic non-linear dynamic (GNLD) load. This model can be used in mid-term and long-term voltage stability simulations (i.e., to study voltage collapse), as it can replace a more detailed representation of aggregate load, including induction motors, thermostatically controlled and static loads.
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct LoadGenericNonLinear {
    #[serde(flatten)]
    pub base: super::LoadDynamics,
    /// Steady state voltage index for reactive power (BS).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bs: Option<f64>,
    /// Transient voltage index for reactive power (BT).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bt: Option<f64>,
    /// Type of generic non-linear load model.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generic_non_linear_load_model_type: Option<super::base::UriRef>,
    /// Steady state voltage index for active power (LS).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ls: Option<f64>,
    /// Transient voltage index for active power (LT).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lt: Option<f64>,
    /// Time constant of lag function of active power (TP) (> 0).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tp: Option<f64>,
    /// Time constant of lag function of reactive power (TQ) (> 0).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tq: Option<f64>,
}
impl crate::base::CimElement for LoadGenericNonLinear {
    fn mrid(&self) -> &str { &self.base.base.id }
    fn type_name(&self) -> &'static str { "LoadGenericNonLinear" }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn to_json_value(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or(serde_json::Value::Null)
    }
    fn to_block(&self) -> crate::base::RdfBlock {
        let mut block = self.base.to_block();
        block.type_name = "LoadGenericNonLinear".to_string();
        if let Some(v) = self.bs {
            block.fields.insert("LoadGenericNonLinear.bs".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.bt {
            block.fields.insert("LoadGenericNonLinear.bt".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(ref v) = self.generic_non_linear_load_model_type {
            block.fields.insert("LoadGenericNonLinear.genericNonLinearLoadModelType".into(), crate::base::FieldValue::Resource(v.uri.clone()));
        }
        if let Some(v) = self.ls {
            block.fields.insert("LoadGenericNonLinear.ls".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.lt {
            block.fields.insert("LoadGenericNonLinear.lt".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.tp {
            block.fields.insert("LoadGenericNonLinear.tp".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.tq {
            block.fields.insert("LoadGenericNonLinear.tq".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        block
    }
}

impl LoadGenericNonLinear {
    pub fn from_block(b: &crate::base::RdfBlock) -> Self {
        let mut obj = Self::default();
        obj.base.base.id.clone_from(&b.mrid);
        for (key, val) in &b.fields {
            match key.as_str() {
                "LoadGenericNonLinear.bs" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.bs = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.bs = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "LoadGenericNonLinear.bt" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.bt = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.bt = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "LoadGenericNonLinear.genericNonLinearLoadModelType" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.generic_non_linear_load_model_type = Some(crate::base::UriRef { uri: sv.clone() });
                    }
                }
                "LoadGenericNonLinear.ls" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.ls = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.ls = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "LoadGenericNonLinear.lt" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.lt = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.lt = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "LoadGenericNonLinear.tp" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.tp = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.tp = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "LoadGenericNonLinear.tq" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.tq = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.tq = Some(v); } }
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
