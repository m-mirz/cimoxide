/// Pitch control power model. Reference: IEC 61400-27-1:2015, 5.6.5.1.
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct WindPitchContPowerIEC {
    #[serde(flatten)]
    pub base: super::IdentifiedObject,
    /// Rate limit for increasing power (dpmax) (> WindPitchContPowerIEC.dpmin). It is a type-dependent parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dpmax: Option<f64>,
    /// Rate limit for decreasing power (dpmin) (< WindPitchContPowerIEC.dpmax). It is a type-dependent parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dpmin: Option<f64>,
    /// Minimum power setting (pmin). It is a type-dependent parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pmin: Option<f64>,
    /// If pinit < pset then power will be ramped down to pmin. It is (pset) in the IEC 61400-27-1:2015. It is a type-dependent parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pset: Option<f64>,
    /// Lag time constant (T1) (>= 0). It is a type-dependent parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub t1: Option<f64>,
    /// Voltage measurement time constant (Tr) (>= 0). It is a type-dependent parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tr: Option<f64>,
    /// Dip detection threshold (uUVRT). It is a type-dependent parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uuvrt: Option<f64>,
}
impl crate::base::CimElement for WindPitchContPowerIEC {
    fn mrid(&self) -> &str { &self.base.id }
    fn type_name(&self) -> &'static str { "WindPitchContPowerIEC" }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn to_json_value(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or(serde_json::Value::Null)
    }
    fn to_block(&self) -> crate::base::RdfBlock {
        let mut block = self.base.to_block();
        block.type_name = "WindPitchContPowerIEC".to_string();
        if let Some(v) = self.dpmax {
            block.fields.insert("WindPitchContPowerIEC.dpmax".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.dpmin {
            block.fields.insert("WindPitchContPowerIEC.dpmin".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.pmin {
            block.fields.insert("WindPitchContPowerIEC.pmin".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.pset {
            block.fields.insert("WindPitchContPowerIEC.pset".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.t1 {
            block.fields.insert("WindPitchContPowerIEC.t1".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.tr {
            block.fields.insert("WindPitchContPowerIEC.tr".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.uuvrt {
            block.fields.insert("WindPitchContPowerIEC.uuvrt".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        block
    }
}

impl WindPitchContPowerIEC {
    pub fn from_block(b: &crate::base::RdfBlock) -> Self {
        let mut obj = Self::default();
        obj.base.id.clone_from(&b.mrid);
        for (key, val) in &b.fields {
            match key.as_str() {
                "WindPitchContPowerIEC.dpmax" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.dpmax = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.dpmax = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "WindPitchContPowerIEC.dpmin" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.dpmin = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.dpmin = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "WindPitchContPowerIEC.pmin" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.pmin = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.pmin = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "WindPitchContPowerIEC.pset" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.pset = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.pset = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "WindPitchContPowerIEC.t1" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.t1 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.t1 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "WindPitchContPowerIEC.tr" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.tr = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.tr = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "WindPitchContPowerIEC.uuvrt" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.uuvrt = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.uuvrt = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "IdentifiedObject.description" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.base.description = sv.clone(); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.base.description = sv.clone(); }
                        }
                        _ => {}
                    }
                }
                "IdentifiedObject.energyIdentCodeEic" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.base.energy_ident_code_eic = sv.clone(); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.base.energy_ident_code_eic = sv.clone(); }
                        }
                        _ => {}
                    }
                }
                "IdentifiedObject.mRID" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.base.m_rid = sv.clone(); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.base.m_rid = sv.clone(); }
                        }
                        _ => {}
                    }
                }
                "IdentifiedObject.name" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.base.name = sv.clone(); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.base.name = sv.clone(); }
                        }
                        _ => {}
                    }
                }
                "IdentifiedObject.shortName" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.base.short_name = sv.clone(); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.base.short_name = sv.clone(); }
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
