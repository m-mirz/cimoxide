/// Rotor resistance control model. Reference: IEC 61400-27-1:2015, 5.6.5.3.
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct WindContRotorRIEC {
    #[serde(flatten)]
    pub base: super::IdentifiedObject,
    /// Integral gain in rotor resistance PI controller (KIrr). It is a type-dependent parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kirr: Option<f64>,
    /// Filter gain for generator speed measurement (Komegafilt). It is a type-dependent parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub komegafilt: Option<f64>,
    /// Filter gain for power measurement (Kpfilt). It is a type-dependent parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kpfilt: Option<f64>,
    /// Proportional gain in rotor resistance PI controller (KPrr). It is a type-dependent parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kprr: Option<f64>,
    /// Maximum rotor resistance (rmax) (> WindContRotorRIEC.rmin). It is a type-dependent parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rmax: Option<f64>,
    /// Minimum rotor resistance (rmin) (< WindContRotorRIEC.rmax). It is a type-dependent parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rmin: Option<f64>,
    /// Filter time constant for generator speed measurement (Tomegafiltrr) (>= 0). It is a type-dependent parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tomegafiltrr: Option<f64>,
    /// Filter time constant for power measurement (Tpfiltrr) (>= 0). It is a type-dependent parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tpfiltrr: Option<f64>,
}
impl crate::base::CimElement for WindContRotorRIEC {
    fn mrid(&self) -> &str { &self.base.id }
    fn type_name(&self) -> &'static str { "WindContRotorRIEC" }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn to_json_value(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or(serde_json::Value::Null)
    }
    fn to_block(&self) -> crate::base::RdfBlock {
        let mut block = self.base.to_block();
        block.type_name = "WindContRotorRIEC".to_string();
        if let Some(v) = self.kirr {
            block.fields.insert("WindContRotorRIEC.kirr".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.komegafilt {
            block.fields.insert("WindContRotorRIEC.komegafilt".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.kpfilt {
            block.fields.insert("WindContRotorRIEC.kpfilt".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.kprr {
            block.fields.insert("WindContRotorRIEC.kprr".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.rmax {
            block.fields.insert("WindContRotorRIEC.rmax".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.rmin {
            block.fields.insert("WindContRotorRIEC.rmin".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.tomegafiltrr {
            block.fields.insert("WindContRotorRIEC.tomegafiltrr".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.tpfiltrr {
            block.fields.insert("WindContRotorRIEC.tpfiltrr".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        block
    }
}

impl WindContRotorRIEC {
    pub fn from_block(b: &crate::base::RdfBlock) -> Self {
        let mut obj = Self::default();
        obj.base.id.clone_from(&b.mrid);
        for (key, val) in &b.fields {
            match key.as_str() {
                "WindContRotorRIEC.kirr" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.kirr = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.kirr = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "WindContRotorRIEC.komegafilt" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.komegafilt = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.komegafilt = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "WindContRotorRIEC.kpfilt" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.kpfilt = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.kpfilt = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "WindContRotorRIEC.kprr" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.kprr = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.kprr = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "WindContRotorRIEC.rmax" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.rmax = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.rmax = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "WindContRotorRIEC.rmin" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.rmin = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.rmin = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "WindContRotorRIEC.tomegafiltrr" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.tomegafiltrr = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.tomegafiltrr = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "WindContRotorRIEC.tpfiltrr" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.tpfiltrr = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.tpfiltrr = Some(v); } }
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
