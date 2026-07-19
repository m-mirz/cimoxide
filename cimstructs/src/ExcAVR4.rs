/// Italian excitation system. It represents a static exciter and electric voltage regulator.
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct ExcAVR4 {
    #[serde(flatten)]
    pub base: super::ExcitationSystemDynamics,
    /// AVR output voltage dependency selector (IMUL). true = selector is connected false = selector is not connected. Typical value = true.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub imul: Option<bool>,
    /// AVR gain (KA). Typical value = 300.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ka: Option<f64>,
    /// Exciter gain (KE). Typical value = 1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ke: Option<f64>,
    /// Exciter internal reactance (KIF). Typical value = 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kif: Option<f64>,
    /// AVR time constant (T1) (>= 0). Typical value = 4,8.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub t1: Option<f64>,
    /// Exciter current feedback time constant (T1IF) (>= 0). Typical value = 60.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub t1if: Option<f64>,
    /// AVR time constant (T2) (>= 0). Typical value = 1,5.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub t2: Option<f64>,
    /// AVR time constant (T3) (>= 0). Typical value = 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub t3: Option<f64>,
    /// AVR time constant (T4) (>= 0). Typical value = 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub t4: Option<f64>,
    /// Exciter current feedback time constant (TIF) (>= 0). Typical value = 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tif: Option<f64>,
    /// Minimum exciter output (VFMN). Typical value = 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vfmn: Option<f64>,
    /// Maximum exciter output (VFMX). Typical value = 5.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vfmx: Option<f64>,
    /// Minimum AVR output (VRMN). Typical value = 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vrmn: Option<f64>,
    /// Maximum AVR output (VRMX). Typical value = 5.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vrmx: Option<f64>,
}
impl crate::base::CimElement for ExcAVR4 {
    fn mrid(&self) -> &str { &self.base.base.base.id }
    fn type_name(&self) -> &'static str { "ExcAVR4" }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn to_json_value(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or(serde_json::Value::Null)
    }
    fn to_block(&self) -> crate::base::RdfBlock {
        let mut block = self.base.to_block();
        block.type_name = "ExcAVR4".to_string();
        if let Some(v) = self.imul {
            block.fields.insert("ExcAVR4.imul".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.ka {
            block.fields.insert("ExcAVR4.ka".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.ke {
            block.fields.insert("ExcAVR4.ke".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.kif {
            block.fields.insert("ExcAVR4.kif".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.t1 {
            block.fields.insert("ExcAVR4.t1".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.t1if {
            block.fields.insert("ExcAVR4.t1if".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.t2 {
            block.fields.insert("ExcAVR4.t2".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.t3 {
            block.fields.insert("ExcAVR4.t3".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.t4 {
            block.fields.insert("ExcAVR4.t4".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.tif {
            block.fields.insert("ExcAVR4.tif".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.vfmn {
            block.fields.insert("ExcAVR4.vfmn".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.vfmx {
            block.fields.insert("ExcAVR4.vfmx".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.vrmn {
            block.fields.insert("ExcAVR4.vrmn".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.vrmx {
            block.fields.insert("ExcAVR4.vrmx".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        block
    }
}

impl ExcAVR4 {
    pub fn from_block(b: &crate::base::RdfBlock) -> Self {
        let mut obj = Self::default();
        obj.base.base.base.id.clone_from(&b.mrid);
        for (key, val) in &b.fields {
            match key.as_str() {
                "ExcAVR4.imul" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.imul = Some(sv.trim() == "true"); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.imul = Some(sv.trim() == "true"); }
                        }
                        _ => {}
                    }
                }
                "ExcAVR4.ka" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.ka = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.ka = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcAVR4.ke" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.ke = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.ke = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcAVR4.kif" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.kif = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.kif = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcAVR4.t1" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.t1 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.t1 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcAVR4.t1if" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.t1if = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.t1if = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcAVR4.t2" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.t2 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.t2 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcAVR4.t3" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.t3 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.t3 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcAVR4.t4" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.t4 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.t4 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcAVR4.tif" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.tif = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.tif = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcAVR4.vfmn" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.vfmn = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.vfmn = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcAVR4.vfmx" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.vfmx = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.vfmx = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcAVR4.vrmn" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.vrmn = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.vrmn = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcAVR4.vrmx" => {
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
