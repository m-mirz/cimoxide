/// Two mass model. Reference: IEC 61400-27-1:2015, 5.6.2.1.
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct WindMechIEC {
    #[serde(flatten)]
    pub base: super::IdentifiedObject,
    /// Drive train damping (cdrt). It is a type-dependent parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cdrt: Option<f64>,
    /// Inertia constant of generator (Hgen) (>= 0). It is a type-dependent parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hgen: Option<f64>,
    /// Inertia constant of wind turbine rotor (HWTR) (>= 0). It is a type-dependent parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hwtr: Option<f64>,
    /// Drive train stiffness (kdrt). It is a type-dependent parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kdrt: Option<f64>,
}
impl crate::base::CimElement for WindMechIEC {
    fn mrid(&self) -> &str { &self.base.id }
    fn type_name(&self) -> &'static str { "WindMechIEC" }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn to_json_value(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or(serde_json::Value::Null)
    }
    fn to_block(&self) -> crate::base::RdfBlock {
        let mut block = self.base.to_block();
        block.type_name = "WindMechIEC".to_string();
        if let Some(v) = self.cdrt {
            block.fields.insert("WindMechIEC.cdrt".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.hgen {
            block.fields.insert("WindMechIEC.hgen".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.hwtr {
            block.fields.insert("WindMechIEC.hwtr".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.kdrt {
            block.fields.insert("WindMechIEC.kdrt".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        block
    }
}

impl WindMechIEC {
    pub fn from_block(b: &crate::base::RdfBlock) -> Self {
        let mut obj = Self::default();
        obj.base.id.clone_from(&b.mrid);
        for (key, val) in &b.fields {
            match key.as_str() {
                "WindMechIEC.cdrt" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.cdrt = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.cdrt = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "WindMechIEC.hgen" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.hgen = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.hgen = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "WindMechIEC.hwtr" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.hwtr = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.hwtr = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "WindMechIEC.kdrt" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.kdrt = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.kdrt = Some(v); } }
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
