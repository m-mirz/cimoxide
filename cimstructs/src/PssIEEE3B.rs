/// IEEE 421.5-2005 type PSS3B power system stabilizer model. The PSS model PSS3B has dual inputs of electrical power and rotor angular frequency deviation. The signals are used to derive an equivalent mechanical power signal. This model has 2 input signals. They have the following fixed types (expressed in terms of InputSignalKind values): the first one is of rotorAngleFrequencyDeviation type and the second one is of generatorElectricalPower type. Reference: IEEE 3B 421.5-2005, 8.3.
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct PssIEEE3B {
    #[serde(flatten)]
    pub base: super::PowerSystemStabilizerDynamics,
    /// Notch filter parameter (A1). Typical value = 0,359.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub a1: Option<f64>,
    /// Notch filter parameter (A2). Typical value = 0,586.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub a2: Option<f64>,
    /// Notch filter parameter (A3). Typical value = 0,429.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub a3: Option<f64>,
    /// Notch filter parameter (A4). Typical value = 0,564.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub a4: Option<f64>,
    /// Notch filter parameter (A5). Typical value = 0,001.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub a5: Option<f64>,
    /// Notch filter parameter (A6). Typical value = 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub a6: Option<f64>,
    /// Notch filter parameter (A7). Typical value = 0,031.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub a7: Option<f64>,
    /// Notch filter parameter (A8). Typical value = 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub a8: Option<f64>,
    /// Gain on signal # 1 (Ks1). Typical value = -0,602.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ks1: Option<f64>,
    /// Gain on signal # 2 (Ks2). Typical value = 30,12.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ks2: Option<f64>,
    /// Transducer time constant (T1) (>= 0). Typical value = 0,012.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub t1: Option<f64>,
    /// Transducer time constant (T2) (>= 0). Typical value = 0,012.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub t2: Option<f64>,
    /// Washout time constant (Tw1) (>= 0). Typical value = 0,3.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tw1: Option<f64>,
    /// Washout time constant (Tw2) (>= 0). Typical value = 0,3.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tw2: Option<f64>,
    /// Washout time constant (Tw3) (>= 0). Typical value = 0,6.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tw3: Option<f64>,
    /// Stabilizer output maximum limit (Vstmax) (> PssIEEE3B.vstmin). Typical value = 0,1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vstmax: Option<f64>,
    /// Stabilizer output minimum limit (Vstmin) (< PssIEEE3B.vstmax). Typical value = -0,1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vstmin: Option<f64>,
}
impl crate::base::CimElement for PssIEEE3B {
    fn mrid(&self) -> &str { &self.base.base.base.id }
    fn type_name(&self) -> &'static str { "PssIEEE3B" }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn to_json_value(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or(serde_json::Value::Null)
    }
    fn to_block(&self) -> crate::base::RdfBlock {
        let mut block = self.base.to_block();
        block.type_name = "PssIEEE3B".to_string();
        if let Some(v) = self.a1 {
            block.fields.insert("PssIEEE3B.a1".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.a2 {
            block.fields.insert("PssIEEE3B.a2".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.a3 {
            block.fields.insert("PssIEEE3B.a3".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.a4 {
            block.fields.insert("PssIEEE3B.a4".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.a5 {
            block.fields.insert("PssIEEE3B.a5".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.a6 {
            block.fields.insert("PssIEEE3B.a6".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.a7 {
            block.fields.insert("PssIEEE3B.a7".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.a8 {
            block.fields.insert("PssIEEE3B.a8".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.ks1 {
            block.fields.insert("PssIEEE3B.ks1".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.ks2 {
            block.fields.insert("PssIEEE3B.ks2".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.t1 {
            block.fields.insert("PssIEEE3B.t1".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.t2 {
            block.fields.insert("PssIEEE3B.t2".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.tw1 {
            block.fields.insert("PssIEEE3B.tw1".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.tw2 {
            block.fields.insert("PssIEEE3B.tw2".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.tw3 {
            block.fields.insert("PssIEEE3B.tw3".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.vstmax {
            block.fields.insert("PssIEEE3B.vstmax".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.vstmin {
            block.fields.insert("PssIEEE3B.vstmin".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        block
    }
}

impl PssIEEE3B {
    pub fn from_block(b: &crate::base::RdfBlock) -> Self {
        let mut obj = Self::default();
        obj.base.base.base.id.clone_from(&b.mrid);
        for (key, val) in &b.fields {
            match key.as_str() {
                "PssIEEE3B.a1" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.a1 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.a1 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "PssIEEE3B.a2" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.a2 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.a2 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "PssIEEE3B.a3" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.a3 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.a3 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "PssIEEE3B.a4" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.a4 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.a4 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "PssIEEE3B.a5" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.a5 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.a5 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "PssIEEE3B.a6" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.a6 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.a6 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "PssIEEE3B.a7" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.a7 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.a7 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "PssIEEE3B.a8" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.a8 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.a8 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "PssIEEE3B.ks1" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.ks1 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.ks1 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "PssIEEE3B.ks2" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.ks2 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.ks2 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "PssIEEE3B.t1" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.t1 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.t1 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "PssIEEE3B.t2" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.t2 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.t2 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "PssIEEE3B.tw1" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.tw1 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.tw1 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "PssIEEE3B.tw2" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.tw2 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.tw2 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "PssIEEE3B.tw3" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.tw3 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.tw3 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "PssIEEE3B.vstmax" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.vstmax = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.vstmax = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "PssIEEE3B.vstmin" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.vstmin = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.vstmin = Some(v); } }
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
