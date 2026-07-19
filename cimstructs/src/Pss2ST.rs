/// PTI microprocessor-based stabilizer type 1.
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct Pss2ST {
    #[serde(flatten)]
    pub base: super::PowerSystemStabilizerDynamics,
    /// Type of input signal #1 (rotorAngularFrequencyDeviation, busFrequencyDeviation, generatorElectricalPower, generatorAcceleratingPower, busVoltage, or busVoltageDerivative - shall be different than Pss2ST.inputSignal2Type). Typical value = rotorAngularFrequencyDeviation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_signal1type: Option<super::base::UriRef>,
    /// Type of input signal #2 (rotorAngularFrequencyDeviation, busFrequencyDeviation, generatorElectricalPower, generatorAcceleratingPower, busVoltage, or busVoltageDerivative - shall be different than Pss2ST.inputSignal1Type). Typical value = busVoltageDerivative.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_signal2type: Option<super::base::UriRef>,
    /// Gain (K1).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub k1: Option<f64>,
    /// Gain (K2).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub k2: Option<f64>,
    /// Limiter (LSMAX) (> Pss2ST.lsmin).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lsmax: Option<f64>,
    /// Limiter (LSMIN) (< Pss2ST.lsmax).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lsmin: Option<f64>,
    /// Time constant (T1) (>= 0).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub t1: Option<f64>,
    /// Time constant (T10) (>= 0).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub t10: Option<f64>,
    /// Time constant (T2) (>= 0).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub t2: Option<f64>,
    /// Time constant (T3) (>= 0).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub t3: Option<f64>,
    /// Time constant (T4) (>= 0).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub t4: Option<f64>,
    /// Time constant (T5) (>= 0).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub t5: Option<f64>,
    /// Time constant (T6) (>= 0).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub t6: Option<f64>,
    /// Time constant (T7) (>= 0).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub t7: Option<f64>,
    /// Time constant (T8) (>= 0).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub t8: Option<f64>,
    /// Time constant (T9) (>= 0).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub t9: Option<f64>,
    /// Cutoff limiter (VCL).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vcl: Option<f64>,
    /// Cutoff limiter (VCU).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vcu: Option<f64>,
}
impl crate::base::CimElement for Pss2ST {
    fn mrid(&self) -> &str { &self.base.base.base.id }
    fn type_name(&self) -> &'static str { "Pss2ST" }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn to_json_value(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or(serde_json::Value::Null)
    }
    fn to_block(&self) -> crate::base::RdfBlock {
        let mut block = self.base.to_block();
        block.type_name = "Pss2ST".to_string();
        if let Some(ref v) = self.input_signal1type {
            block.fields.insert("Pss2ST.inputSignal1Type".into(), crate::base::FieldValue::Resource(v.uri.clone()));
        }
        if let Some(ref v) = self.input_signal2type {
            block.fields.insert("Pss2ST.inputSignal2Type".into(), crate::base::FieldValue::Resource(v.uri.clone()));
        }
        if let Some(v) = self.k1 {
            block.fields.insert("Pss2ST.k1".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.k2 {
            block.fields.insert("Pss2ST.k2".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.lsmax {
            block.fields.insert("Pss2ST.lsmax".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.lsmin {
            block.fields.insert("Pss2ST.lsmin".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.t1 {
            block.fields.insert("Pss2ST.t1".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.t10 {
            block.fields.insert("Pss2ST.t10".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.t2 {
            block.fields.insert("Pss2ST.t2".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.t3 {
            block.fields.insert("Pss2ST.t3".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.t4 {
            block.fields.insert("Pss2ST.t4".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.t5 {
            block.fields.insert("Pss2ST.t5".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.t6 {
            block.fields.insert("Pss2ST.t6".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.t7 {
            block.fields.insert("Pss2ST.t7".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.t8 {
            block.fields.insert("Pss2ST.t8".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.t9 {
            block.fields.insert("Pss2ST.t9".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.vcl {
            block.fields.insert("Pss2ST.vcl".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.vcu {
            block.fields.insert("Pss2ST.vcu".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        block
    }
}

impl Pss2ST {
    pub fn from_block(b: &crate::base::RdfBlock) -> Self {
        let mut obj = Self::default();
        obj.base.base.base.id.clone_from(&b.mrid);
        for (key, val) in &b.fields {
            match key.as_str() {
                "Pss2ST.inputSignal1Type" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.input_signal1type = Some(crate::base::UriRef { uri: sv.clone() });
                    }
                }
                "Pss2ST.inputSignal2Type" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.input_signal2type = Some(crate::base::UriRef { uri: sv.clone() });
                    }
                }
                "Pss2ST.k1" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.k1 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.k1 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "Pss2ST.k2" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.k2 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.k2 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "Pss2ST.lsmax" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.lsmax = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.lsmax = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "Pss2ST.lsmin" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.lsmin = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.lsmin = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "Pss2ST.t1" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.t1 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.t1 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "Pss2ST.t10" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.t10 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.t10 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "Pss2ST.t2" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.t2 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.t2 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "Pss2ST.t3" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.t3 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.t3 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "Pss2ST.t4" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.t4 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.t4 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "Pss2ST.t5" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.t5 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.t5 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "Pss2ST.t6" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.t6 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.t6 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "Pss2ST.t7" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.t7 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.t7 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "Pss2ST.t8" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.t8 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.t8 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "Pss2ST.t9" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.t9 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.t9 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "Pss2ST.vcl" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.vcl = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.vcl = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "Pss2ST.vcu" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.vcu = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.vcu = Some(v); } }
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
