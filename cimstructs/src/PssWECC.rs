/// Dual input power system stabilizer, based on IEEE type 2, with modified output limiter defined by WECC (Western Electricity Coordinating Council, USA).
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct PssWECC {
    #[serde(flatten)]
    pub base: super::PowerSystemStabilizerDynamics,
    /// Type of input signal #1 (rotorAngularFrequencyDeviation, busFrequencyDeviation, generatorElectricalPower, generatorAcceleratingPower, busVoltage, or busVoltageDerivative - shall be different than PssWECC.inputSignal2Type). Typical value = rotorAngularFrequencyDeviation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_signal1type: Option<super::base::UriRef>,
    /// Type of input signal #2 (rotorAngularFrequencyDeviation, busFrequencyDeviation, generatorElectricalPower, generatorAcceleratingPower, busVoltage, busVoltageDerivative - shall be different than PssWECC.inputSignal1Type). Typical value = busVoltageDerivative.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_signal2type: Option<super::base::UriRef>,
    /// Input signal 1 gain (K1). Typical value = 1,13.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub k1: Option<f64>,
    /// Input signal 2 gain (K2). Typical value = 0,0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub k2: Option<f64>,
    /// Input signal 1 transducer time constant (T1) (>= 0). Typical value = 0,037.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub t1: Option<f64>,
    /// Lag time constant (T10) (>= 0). Typical value = 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub t10: Option<f64>,
    /// Input signal 2 transducer time constant (T2) (>= 0). Typical value = 0,0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub t2: Option<f64>,
    /// Stabilizer washout time constant (T3) (>= 0). Typical value = 9,5.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub t3: Option<f64>,
    /// Stabilizer washout time lag constant (T4) (>= 0). Typical value = 9,5.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub t4: Option<f64>,
    /// Lead time constant (T5) (>= 0). Typical value = 1,7.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub t5: Option<f64>,
    /// Lag time constant (T6) (>= 0). Typical value = 1,5.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub t6: Option<f64>,
    /// Lead time constant (T7) (>= 0). Typical value = 1,7.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub t7: Option<f64>,
    /// Lag time constant (T8) (>= 0). Typical value = 1,5.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub t8: Option<f64>,
    /// Lead time constant (T9) (>= 0). Typical value = 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub t9: Option<f64>,
    /// Minimum value for voltage compensator output (VCL). Typical value = 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vcl: Option<f64>,
    /// Maximum value for voltage compensator output (VCU). Typical value = 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vcu: Option<f64>,
    /// Maximum output signal (Vsmax) (> PssWECC.vsmin). Typical value = 0,05.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vsmax: Option<f64>,
    /// Minimum output signal (Vsmin) (< PssWECC.vsmax). Typical value = -0,05.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vsmin: Option<f64>,
}
impl crate::base::CimElement for PssWECC {
    fn mrid(&self) -> &str { &self.base.base.base.id }
    fn type_name(&self) -> &'static str { "PssWECC" }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn to_json_value(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or(serde_json::Value::Null)
    }
    fn to_block(&self) -> crate::base::RdfBlock {
        let mut block = self.base.to_block();
        block.type_name = "PssWECC".to_string();
        if let Some(ref v) = self.input_signal1type {
            block.fields.insert("PssWECC.inputSignal1Type".into(), crate::base::FieldValue::Resource(v.uri.clone()));
        }
        if let Some(ref v) = self.input_signal2type {
            block.fields.insert("PssWECC.inputSignal2Type".into(), crate::base::FieldValue::Resource(v.uri.clone()));
        }
        if let Some(v) = self.k1 {
            block.fields.insert("PssWECC.k1".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.k2 {
            block.fields.insert("PssWECC.k2".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.t1 {
            block.fields.insert("PssWECC.t1".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.t10 {
            block.fields.insert("PssWECC.t10".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.t2 {
            block.fields.insert("PssWECC.t2".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.t3 {
            block.fields.insert("PssWECC.t3".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.t4 {
            block.fields.insert("PssWECC.t4".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.t5 {
            block.fields.insert("PssWECC.t5".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.t6 {
            block.fields.insert("PssWECC.t6".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.t7 {
            block.fields.insert("PssWECC.t7".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.t8 {
            block.fields.insert("PssWECC.t8".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.t9 {
            block.fields.insert("PssWECC.t9".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.vcl {
            block.fields.insert("PssWECC.vcl".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.vcu {
            block.fields.insert("PssWECC.vcu".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.vsmax {
            block.fields.insert("PssWECC.vsmax".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.vsmin {
            block.fields.insert("PssWECC.vsmin".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        block
    }
}

impl PssWECC {
    pub fn from_block(b: &crate::base::RdfBlock) -> Self {
        let mut obj = Self::default();
        obj.base.base.base.id.clone_from(&b.mrid);
        for (key, val) in &b.fields {
            match key.as_str() {
                "PssWECC.inputSignal1Type" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.input_signal1type = Some(crate::base::UriRef { uri: sv.clone() });
                    }
                }
                "PssWECC.inputSignal2Type" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.input_signal2type = Some(crate::base::UriRef { uri: sv.clone() });
                    }
                }
                "PssWECC.k1" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.k1 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.k1 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "PssWECC.k2" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.k2 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.k2 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "PssWECC.t1" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.t1 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.t1 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "PssWECC.t10" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.t10 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.t10 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "PssWECC.t2" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.t2 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.t2 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "PssWECC.t3" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.t3 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.t3 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "PssWECC.t4" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.t4 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.t4 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "PssWECC.t5" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.t5 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.t5 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "PssWECC.t6" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.t6 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.t6 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "PssWECC.t7" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.t7 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.t7 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "PssWECC.t8" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.t8 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.t8 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "PssWECC.t9" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.t9 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.t9 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "PssWECC.vcl" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.vcl = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.vcl = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "PssWECC.vcu" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.vcu = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.vcu = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "PssWECC.vsmax" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.vsmax = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.vsmax = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "PssWECC.vsmin" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.vsmin = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.vsmin = Some(v); } }
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
