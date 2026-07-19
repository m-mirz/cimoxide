/// Italian excitation system corresponding to IEEE (1968) type 2 model. It represents an alternator and rotating diodes and electromechanic voltage regulators.
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct ExcAVR2 {
    #[serde(flatten)]
    pub base: super::ExcitationSystemDynamics,
    /// Field voltage value 1 (E1). Typical value = 4,18.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub e1: Option<f64>,
    /// Field voltage value 2 (E2). Typical value = 3,14.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub e2: Option<f64>,
    /// AVR gain (KA). Typical value = 500.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ka: Option<f64>,
    /// Rate feedback gain (KF). Typical value = 0,12.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kf: Option<f64>,
    /// Saturation factor at E1 (S[E1]). Typical value = 0.1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub se1: Option<f64>,
    /// Saturation factor at E2 (S[E2]). Typical value = 0,03.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub se2: Option<f64>,
    /// AVR time constant (TA) (>= 0). Typical value = 0,02.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ta: Option<f64>,
    /// AVR time constant (TB) (>= 0). Typical value = 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tb: Option<f64>,
    /// Exciter time constant (TE) (>= 0). Typical value = 1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub te: Option<f64>,
    /// Rate feedback time constant (TF1) (>= 0). Typical value = 1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tf1: Option<f64>,
    /// Rate feedback time constant (TF2) (>= 0). Typical value = 1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tf2: Option<f64>,
    /// Minimum AVR output (VRMN). Typical value = -6.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vrmn: Option<f64>,
    /// Maximum AVR output (VRMX). Typical value = 7.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vrmx: Option<f64>,
}
impl crate::base::CimElement for ExcAVR2 {
    fn mrid(&self) -> &str { &self.base.base.base.id }
    fn type_name(&self) -> &'static str { "ExcAVR2" }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn to_json_value(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or(serde_json::Value::Null)
    }
    fn to_block(&self) -> crate::base::RdfBlock {
        let mut block = self.base.to_block();
        block.type_name = "ExcAVR2".to_string();
        if let Some(v) = self.e1 {
            block.fields.insert("ExcAVR2.e1".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.e2 {
            block.fields.insert("ExcAVR2.e2".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.ka {
            block.fields.insert("ExcAVR2.ka".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.kf {
            block.fields.insert("ExcAVR2.kf".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.se1 {
            block.fields.insert("ExcAVR2.se1".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.se2 {
            block.fields.insert("ExcAVR2.se2".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.ta {
            block.fields.insert("ExcAVR2.ta".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.tb {
            block.fields.insert("ExcAVR2.tb".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.te {
            block.fields.insert("ExcAVR2.te".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.tf1 {
            block.fields.insert("ExcAVR2.tf1".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.tf2 {
            block.fields.insert("ExcAVR2.tf2".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.vrmn {
            block.fields.insert("ExcAVR2.vrmn".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.vrmx {
            block.fields.insert("ExcAVR2.vrmx".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        block
    }
}

impl ExcAVR2 {
    pub fn from_block(b: &crate::base::RdfBlock) -> Self {
        let mut obj = Self::default();
        obj.base.base.base.id.clone_from(&b.mrid);
        for (key, val) in &b.fields {
            match key.as_str() {
                "ExcAVR2.e1" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.e1 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.e1 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcAVR2.e2" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.e2 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.e2 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcAVR2.ka" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.ka = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.ka = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcAVR2.kf" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.kf = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.kf = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcAVR2.se1" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.se1 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.se1 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcAVR2.se2" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.se2 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.se2 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcAVR2.ta" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.ta = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.ta = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcAVR2.tb" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.tb = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.tb = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcAVR2.te" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.te = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.te = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcAVR2.tf1" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.tf1 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.tf1 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcAVR2.tf2" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.tf2 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.tf2 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcAVR2.vrmn" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.vrmn = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.vrmn = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcAVR2.vrmx" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.vrmx = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.vrmx = Some(v); } }
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
