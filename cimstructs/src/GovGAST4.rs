/// Generic turbogas.
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct GovGAST4 {
    #[serde(flatten)]
    pub base: super::TurbineGovernorDynamics,
    /// Droop (bp). Typical value = 0,05.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bp: Option<f64>,
    /// Compressor gain (Ktm). Typical value = 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ktm: Option<f64>,
    /// Fuel flow maximum negative error value (MNef). Typical value = -0,05.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mnef: Option<f64>,
    /// Fuel flow maximum positive error value (MXef). Typical value = 0,05.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mxef: Option<f64>,
    /// Minimum valve opening (RYMN). Typical value = 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rymn: Option<f64>,
    /// Maximum valve opening (RYMX). Typical value = 1,1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rymx: Option<f64>,
    /// Maximum gate opening velocity (TA) (>= 0). Typical value = 3.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ta: Option<f64>,
    /// Maximum gate closing velocity (TC) (>= 0). Typical value = 0,5.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tc: Option<f64>,
    /// Fuel control time constant (Tcm) (>= 0). Typical value = 0,1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tcm: Option<f64>,
    /// Compressor discharge volume time constant (Tm) (>= 0). Typical value = 0,2.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tm: Option<f64>,
    /// Time constant of fuel valve positioner (Ty) (>= 0). Typical value = 0,1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ty: Option<f64>,
}
impl crate::base::CimElement for GovGAST4 {
    fn mrid(&self) -> &str { &self.base.base.base.id }
    fn type_name(&self) -> &'static str { "GovGAST4" }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn to_json_value(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or(serde_json::Value::Null)
    }
    fn to_block(&self) -> crate::base::RdfBlock {
        let mut block = self.base.to_block();
        block.type_name = "GovGAST4".to_string();
        if let Some(v) = self.bp {
            block.fields.insert("GovGAST4.bp".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.ktm {
            block.fields.insert("GovGAST4.ktm".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.mnef {
            block.fields.insert("GovGAST4.mnef".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.mxef {
            block.fields.insert("GovGAST4.mxef".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.rymn {
            block.fields.insert("GovGAST4.rymn".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.rymx {
            block.fields.insert("GovGAST4.rymx".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.ta {
            block.fields.insert("GovGAST4.ta".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.tc {
            block.fields.insert("GovGAST4.tc".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.tcm {
            block.fields.insert("GovGAST4.tcm".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.tm {
            block.fields.insert("GovGAST4.tm".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.ty {
            block.fields.insert("GovGAST4.ty".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        block
    }
}

impl GovGAST4 {
    pub fn from_block(b: &crate::base::RdfBlock) -> Self {
        let mut obj = Self::default();
        obj.base.base.base.id.clone_from(&b.mrid);
        for (key, val) in &b.fields {
            match key.as_str() {
                "GovGAST4.bp" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.bp = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.bp = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovGAST4.ktm" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.ktm = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.ktm = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovGAST4.mnef" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.mnef = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.mnef = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovGAST4.mxef" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.mxef = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.mxef = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovGAST4.rymn" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.rymn = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.rymn = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovGAST4.rymx" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.rymx = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.rymx = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovGAST4.ta" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.ta = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.ta = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovGAST4.tc" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.tc = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.tc = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovGAST4.tcm" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.tcm = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.tcm = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovGAST4.tm" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.tm = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.tm = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovGAST4.ty" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.ty = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.ty = Some(v); } }
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
