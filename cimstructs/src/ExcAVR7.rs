/// IVO excitation system.
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct ExcAVR7 {
    #[serde(flatten)]
    pub base: super::ExcitationSystemDynamics,
    /// Lead coefficient (A1). Typical value = 0,5.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub a1: Option<f64>,
    /// Lag coefficient (A2). Typical value = 0,5.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub a2: Option<f64>,
    /// Lead coefficient (A3). Typical value = 0,5.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub a3: Option<f64>,
    /// Lag coefficient (A4). Typical value = 0,5.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub a4: Option<f64>,
    /// Lead coefficient (A5). Typical value = 0,5.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub a5: Option<f64>,
    /// Lag coefficient (A6). Typical value = 0,5.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub a6: Option<f64>,
    /// Gain (K1). Typical value = 1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub k1: Option<f64>,
    /// Gain (K3). Typical value = 3.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub k3: Option<f64>,
    /// Gain (K5). Typical value = 1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub k5: Option<f64>,
    /// Lead time constant (T1) (>= 0). Typical value = 0,05.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub t1: Option<f64>,
    /// Lag time constant (T2) (>= 0). Typical value = 0,1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub t2: Option<f64>,
    /// Lead time constant (T3) (>= 0). Typical value = 0,1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub t3: Option<f64>,
    /// Lag time constant (T4) (>= 0). Typical value = 0,1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub t4: Option<f64>,
    /// Lead time constant (T5) (>= 0). Typical value = 0,1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub t5: Option<f64>,
    /// Lag time constant (T6) (>= 0). Typical value = 0,1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub t6: Option<f64>,
    /// Lead-lag maximum limit (Vmax1) (> ExcAVR7.vmin1). Typical value = 5.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vmax1: Option<f64>,
    /// Lead-lag maximum limit (Vmax3) (> ExcAVR7.vmin3). Typical value = 5.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vmax3: Option<f64>,
    /// Lead-lag maximum limit (Vmax5) (> ExcAVR7.vmin5). Typical value = 5.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vmax5: Option<f64>,
    /// Lead-lag minimum limit (Vmin1) (< ExcAVR7.vmax1). Typical value = -5.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vmin1: Option<f64>,
    /// Lead-lag minimum limit (Vmin3) (< ExcAVR7.vmax3). Typical value = -5.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vmin3: Option<f64>,
    /// Lead-lag minimum limit (Vmin5) (< ExcAVR7.vmax5). Typical value = -2.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vmin5: Option<f64>,
}
impl crate::base::CimElement for ExcAVR7 {
    fn mrid(&self) -> &str { &self.base.base.base.id }
    fn type_name(&self) -> &'static str { "ExcAVR7" }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn to_json_value(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or(serde_json::Value::Null)
    }
    fn to_block(&self) -> crate::base::RdfBlock {
        let mut block = self.base.to_block();
        block.type_name = "ExcAVR7".to_string();
        if let Some(v) = self.a1 {
            block.fields.insert("ExcAVR7.a1".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.a2 {
            block.fields.insert("ExcAVR7.a2".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.a3 {
            block.fields.insert("ExcAVR7.a3".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.a4 {
            block.fields.insert("ExcAVR7.a4".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.a5 {
            block.fields.insert("ExcAVR7.a5".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.a6 {
            block.fields.insert("ExcAVR7.a6".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.k1 {
            block.fields.insert("ExcAVR7.k1".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.k3 {
            block.fields.insert("ExcAVR7.k3".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.k5 {
            block.fields.insert("ExcAVR7.k5".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.t1 {
            block.fields.insert("ExcAVR7.t1".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.t2 {
            block.fields.insert("ExcAVR7.t2".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.t3 {
            block.fields.insert("ExcAVR7.t3".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.t4 {
            block.fields.insert("ExcAVR7.t4".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.t5 {
            block.fields.insert("ExcAVR7.t5".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.t6 {
            block.fields.insert("ExcAVR7.t6".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.vmax1 {
            block.fields.insert("ExcAVR7.vmax1".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.vmax3 {
            block.fields.insert("ExcAVR7.vmax3".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.vmax5 {
            block.fields.insert("ExcAVR7.vmax5".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.vmin1 {
            block.fields.insert("ExcAVR7.vmin1".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.vmin3 {
            block.fields.insert("ExcAVR7.vmin3".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.vmin5 {
            block.fields.insert("ExcAVR7.vmin5".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        block
    }
}

impl ExcAVR7 {
    pub fn from_block(b: &crate::base::RdfBlock) -> Self {
        let mut obj = Self::default();
        obj.base.base.base.id.clone_from(&b.mrid);
        for (key, val) in &b.fields {
            match key.as_str() {
                "ExcAVR7.a1" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.a1 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.a1 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcAVR7.a2" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.a2 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.a2 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcAVR7.a3" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.a3 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.a3 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcAVR7.a4" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.a4 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.a4 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcAVR7.a5" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.a5 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.a5 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcAVR7.a6" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.a6 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.a6 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcAVR7.k1" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.k1 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.k1 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcAVR7.k3" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.k3 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.k3 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcAVR7.k5" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.k5 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.k5 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcAVR7.t1" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.t1 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.t1 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcAVR7.t2" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.t2 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.t2 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcAVR7.t3" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.t3 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.t3 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcAVR7.t4" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.t4 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.t4 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcAVR7.t5" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.t5 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.t5 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcAVR7.t6" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.t6 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.t6 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcAVR7.vmax1" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.vmax1 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.vmax1 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcAVR7.vmax3" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.vmax3 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.vmax3 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcAVR7.vmax5" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.vmax5 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.vmax5 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcAVR7.vmin1" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.vmin1 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.vmin1 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcAVR7.vmin3" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.vmin3 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.vmin3 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcAVR7.vmin5" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.vmin5 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.vmin5 = Some(v); } }
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
