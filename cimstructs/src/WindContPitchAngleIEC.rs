/// Pitch angle control model. Reference: IEC 61400-27-1:2015, 5.6.5.2.
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct WindContPitchAngleIEC {
    #[serde(flatten)]
    pub base: super::IdentifiedObject,
    /// Maximum pitch positive ramp rate (dthetamax) (> WindContPitchAngleIEC.dthetamin). It is a type-dependent parameter. Unit = degrees / s.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dthetamax: Option<f64>,
    /// Maximum pitch negative ramp rate (dthetamin) (< WindContPitchAngleIEC.dthetamax). It is a type-dependent parameter. Unit = degrees / s.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dthetamin: Option<f64>,
    /// Power PI controller integration gain (KIc). It is a type-dependent parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kic: Option<f64>,
    /// Speed PI controller integration gain (KIomega). It is a type-dependent parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kiomega: Option<f64>,
    /// Power PI controller proportional gain (KPc). It is a type-dependent parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kpc: Option<f64>,
    /// Speed PI controller proportional gain (KPomega). It is a type-dependent parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kpomega: Option<f64>,
    /// Pitch cross coupling gain (KPX). It is a type-dependent parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kpx: Option<f64>,
    /// Maximum pitch angle (thetamax) (> WindContPitchAngleIEC.thetamin). It is a type-dependent parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thetamax: Option<f64>,
    /// Minimum pitch angle (thetamin) (< WindContPitchAngleIEC.thetamax). It is a type-dependent parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thetamin: Option<f64>,
    /// Pitch time constant (ttheta) (>= 0). It is a type-dependent parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ttheta: Option<f64>,
}
impl crate::base::CimElement for WindContPitchAngleIEC {
    fn mrid(&self) -> &str { &self.base.id }
    fn type_name(&self) -> &'static str { "WindContPitchAngleIEC" }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn to_json_value(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or(serde_json::Value::Null)
    }
    fn to_block(&self) -> crate::base::RdfBlock {
        let mut block = self.base.to_block();
        block.type_name = "WindContPitchAngleIEC".to_string();
        if let Some(v) = self.dthetamax {
            block.fields.insert("WindContPitchAngleIEC.dthetamax".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.dthetamin {
            block.fields.insert("WindContPitchAngleIEC.dthetamin".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.kic {
            block.fields.insert("WindContPitchAngleIEC.kic".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.kiomega {
            block.fields.insert("WindContPitchAngleIEC.kiomega".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.kpc {
            block.fields.insert("WindContPitchAngleIEC.kpc".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.kpomega {
            block.fields.insert("WindContPitchAngleIEC.kpomega".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.kpx {
            block.fields.insert("WindContPitchAngleIEC.kpx".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.thetamax {
            block.fields.insert("WindContPitchAngleIEC.thetamax".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.thetamin {
            block.fields.insert("WindContPitchAngleIEC.thetamin".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.ttheta {
            block.fields.insert("WindContPitchAngleIEC.ttheta".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        block
    }
}

impl WindContPitchAngleIEC {
    pub fn from_block(b: &crate::base::RdfBlock) -> Self {
        let mut obj = Self::default();
        obj.base.id.clone_from(&b.mrid);
        for (key, val) in &b.fields {
            match key.as_str() {
                "WindContPitchAngleIEC.dthetamax" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.dthetamax = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.dthetamax = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "WindContPitchAngleIEC.dthetamin" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.dthetamin = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.dthetamin = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "WindContPitchAngleIEC.kic" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.kic = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.kic = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "WindContPitchAngleIEC.kiomega" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.kiomega = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.kiomega = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "WindContPitchAngleIEC.kpc" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.kpc = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.kpc = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "WindContPitchAngleIEC.kpomega" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.kpomega = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.kpomega = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "WindContPitchAngleIEC.kpx" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.kpx = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.kpx = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "WindContPitchAngleIEC.thetamax" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.thetamax = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.thetamax = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "WindContPitchAngleIEC.thetamin" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.thetamin = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.thetamin = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "WindContPitchAngleIEC.ttheta" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.ttheta = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.ttheta = Some(v); } }
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
