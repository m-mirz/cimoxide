/// Two-dimensional aerodynamic model. Reference: IEC 61400-27-1:2015, 5.6.1.3.
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct WindAeroTwoDimIEC {
    #[serde(flatten)]
    pub base: super::IdentifiedObject,
    /// Partial derivative of aerodynamic power with respect to changes in WTR speed (dpomega). It is a type-dependent parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dpomega: Option<f64>,
    /// Partial derivative of aerodynamic power with respect to changes in pitch angle (dptheta). It is a type-dependent parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dptheta: Option<f64>,
    /// Partial derivative (dpv1). It is a type-dependent parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dpv1: Option<f64>,
    /// Rotor speed if the wind turbine is not derated (omega0). It is a type-dependent parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub omegazero: Option<f64>,
    /// Available aerodynamic power (pavail). It is a case-dependent parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pavail: Option<f64>,
    /// Blade angle at twice rated wind speed (thetav2). It is a type-dependent parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thetav2: Option<f64>,
    /// Pitch angle if the wind turbine is not derated (theta0). It is a case-dependent parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thetazero: Option<f64>,
}
impl crate::base::CimElement for WindAeroTwoDimIEC {
    fn mrid(&self) -> &str { &self.base.id }
    fn type_name(&self) -> &'static str { "WindAeroTwoDimIEC" }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn to_json_value(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or(serde_json::Value::Null)
    }
    fn to_block(&self) -> crate::base::RdfBlock {
        let mut block = self.base.to_block();
        block.type_name = "WindAeroTwoDimIEC".to_string();
        if let Some(v) = self.dpomega {
            block.fields.insert("WindAeroTwoDimIEC.dpomega".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.dptheta {
            block.fields.insert("WindAeroTwoDimIEC.dptheta".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.dpv1 {
            block.fields.insert("WindAeroTwoDimIEC.dpv1".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.omegazero {
            block.fields.insert("WindAeroTwoDimIEC.omegazero".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.pavail {
            block.fields.insert("WindAeroTwoDimIEC.pavail".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.thetav2 {
            block.fields.insert("WindAeroTwoDimIEC.thetav2".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.thetazero {
            block.fields.insert("WindAeroTwoDimIEC.thetazero".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        block
    }
}

impl WindAeroTwoDimIEC {
    pub fn from_block(b: &crate::base::RdfBlock) -> Self {
        let mut obj = Self::default();
        obj.base.id.clone_from(&b.mrid);
        for (key, val) in &b.fields {
            match key.as_str() {
                "WindAeroTwoDimIEC.dpomega" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.dpomega = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.dpomega = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "WindAeroTwoDimIEC.dptheta" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.dptheta = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.dptheta = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "WindAeroTwoDimIEC.dpv1" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.dpv1 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.dpv1 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "WindAeroTwoDimIEC.omegazero" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.omegazero = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.omegazero = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "WindAeroTwoDimIEC.pavail" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.pavail = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.pavail = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "WindAeroTwoDimIEC.thetav2" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.thetav2 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.thetav2 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "WindAeroTwoDimIEC.thetazero" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.thetazero = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.thetazero = Some(v); } }
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
