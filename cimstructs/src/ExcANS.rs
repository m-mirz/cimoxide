/// Italian excitation system. It represents static field voltage or excitation current feedback excitation system.
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct ExcANS {
    #[serde(flatten)]
    pub base: super::ExcitationSystemDynamics,
    /// Governor control flag (BLINT). 0 = lead-lag regulator 1 = proportional integral regulator. Typical value = 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blint: Option<i64>,
    /// Minimum exciter current (IFMN). Typical value = -5,2.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ifmn: Option<f64>,
    /// Maximum exciter current (IFMX). Typical value = 6,5.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ifmx: Option<f64>,
    /// Exciter gain (K2). Typical value = 20.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub k2: Option<f64>,
    /// AVR gain (K3). Typical value = 1000.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub k3: Option<f64>,
    /// Ceiling factor (KCE). Typical value = 1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kce: Option<f64>,
    /// Feedback enabling (KRVECC). 0 = open loop control 1 = closed loop control. Typical value = 1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub krvecc: Option<i64>,
    /// Rate feedback signal flag (KVFIF). 0 = output voltage of the exciter 1 = exciter field current. Typical value = 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kvfif: Option<i64>,
    /// Time constant (T1) (>= 0). Typical value = 20.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub t1: Option<f64>,
    /// Time constant (T2) (>= 0). Typical value = 0,05.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub t2: Option<f64>,
    /// Time constant (T3) (>= 0). Typical value = 1,6.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub t3: Option<f64>,
    /// Exciter time constant (TB) (>= 0). Typical value = 0,04.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tb: Option<f64>,
    /// Minimum AVR output (VRMN). Typical value = -5,2.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vrmn: Option<f64>,
    /// Maximum AVR output (VRMX). Typical value = 6,5.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vrmx: Option<f64>,
}
impl crate::base::CimElement for ExcANS {
    fn mrid(&self) -> &str { &self.base.base.base.id }
    fn type_name(&self) -> &'static str { "ExcANS" }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn to_json_value(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or(serde_json::Value::Null)
    }
    fn to_block(&self) -> crate::base::RdfBlock {
        let mut block = self.base.to_block();
        block.type_name = "ExcANS".to_string();
        if let Some(v) = self.blint {
            block.fields.insert("ExcANS.blint".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.ifmn {
            block.fields.insert("ExcANS.ifmn".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.ifmx {
            block.fields.insert("ExcANS.ifmx".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.k2 {
            block.fields.insert("ExcANS.k2".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.k3 {
            block.fields.insert("ExcANS.k3".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.kce {
            block.fields.insert("ExcANS.kce".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.krvecc {
            block.fields.insert("ExcANS.krvecc".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.kvfif {
            block.fields.insert("ExcANS.kvfif".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.t1 {
            block.fields.insert("ExcANS.t1".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.t2 {
            block.fields.insert("ExcANS.t2".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.t3 {
            block.fields.insert("ExcANS.t3".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.tb {
            block.fields.insert("ExcANS.tb".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.vrmn {
            block.fields.insert("ExcANS.vrmn".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.vrmx {
            block.fields.insert("ExcANS.vrmx".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        block
    }
}

impl ExcANS {
    pub fn from_block(b: &crate::base::RdfBlock) -> Self {
        let mut obj = Self::default();
        obj.base.base.base.id.clone_from(&b.mrid);
        for (key, val) in &b.fields {
            match key.as_str() {
                "ExcANS.blint" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.blint = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.blint = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcANS.ifmn" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.ifmn = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.ifmn = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcANS.ifmx" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.ifmx = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.ifmx = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcANS.k2" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.k2 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.k2 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcANS.k3" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.k3 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.k3 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcANS.kce" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.kce = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.kce = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcANS.krvecc" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.krvecc = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.krvecc = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcANS.kvfif" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.kvfif = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.kvfif = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcANS.t1" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.t1 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.t1 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcANS.t2" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.t2 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.t2 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcANS.t3" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.t3 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.t3 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcANS.tb" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.tb = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.tb = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcANS.vrmn" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.vrmn = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.vrmn = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcANS.vrmx" => {
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
