/// P control model type 4A. Reference: IEC 61400-27-1:2015, 5.6.5.5.
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct WindContPType4aIEC {
    #[serde(flatten)]
    pub base: super::IdentifiedObject,
    /// Maximum wind turbine power ramp rate (dpmaxp4A). It is a project-dependent parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dpmaxp4a: Option<f64>,
    /// Time constant in power order lag (Tpordp4A) (>= 0). It is a type-dependent parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tpordp4a: Option<f64>,
    /// Voltage measurement filter time constant (Tufiltp4A) (>= 0). It is a type-dependent parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tufiltp4a: Option<f64>,
}
impl crate::base::CimElement for WindContPType4aIEC {
    fn mrid(&self) -> &str { &self.base.id }
    fn type_name(&self) -> &'static str { "WindContPType4aIEC" }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn to_json_value(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or(serde_json::Value::Null)
    }
    fn to_block(&self) -> crate::base::RdfBlock {
        let mut block = self.base.to_block();
        block.type_name = "WindContPType4aIEC".to_string();
        if let Some(v) = self.dpmaxp4a {
            block.fields.insert("WindContPType4aIEC.dpmaxp4a".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.tpordp4a {
            block.fields.insert("WindContPType4aIEC.tpordp4a".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.tufiltp4a {
            block.fields.insert("WindContPType4aIEC.tufiltp4a".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        block
    }
}

impl WindContPType4aIEC {
    pub fn from_block(b: &crate::base::RdfBlock) -> Self {
        let mut obj = Self::default();
        obj.base.id.clone_from(&b.mrid);
        for (key, val) in &b.fields {
            match key.as_str() {
                "WindContPType4aIEC.dpmaxp4a" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.dpmaxp4a = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.dpmaxp4a = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "WindContPType4aIEC.tpordp4a" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.tpordp4a = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.tpordp4a = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "WindContPType4aIEC.tufiltp4a" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.tufiltp4a = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.tufiltp4a = Some(v); } }
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
