/// Parent class supporting relationships to IEC wind turbines type 3 generator models of IEC type 3A and 3B.
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct WindGenType3IEC {
    #[serde(flatten)]
    pub base: super::IdentifiedObject,
    /// Maximum active current ramp rate (dipmax). It is a project-dependent parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dipmax: Option<f64>,
    /// Maximum reactive current ramp rate (diqmax). It is a project-dependent parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub diqmax: Option<f64>,
    /// Electromagnetic transient reactance (xS). It is a type-dependent parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub xs: Option<f64>,
}
impl crate::base::CimElement for WindGenType3IEC {
    fn mrid(&self) -> &str { &self.base.id }
    fn type_name(&self) -> &'static str { "WindGenType3IEC" }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn to_json_value(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or(serde_json::Value::Null)
    }
    fn to_block(&self) -> crate::base::RdfBlock {
        let mut block = self.base.to_block();
        block.type_name = "WindGenType3IEC".to_string();
        if let Some(v) = self.dipmax {
            block.fields.insert("WindGenType3IEC.dipmax".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.diqmax {
            block.fields.insert("WindGenType3IEC.diqmax".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.xs {
            block.fields.insert("WindGenType3IEC.xs".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        block
    }
}

impl WindGenType3IEC {
    pub fn from_block(b: &crate::base::RdfBlock) -> Self {
        let mut obj = Self::default();
        obj.base.id.clone_from(&b.mrid);
        for (key, val) in &b.fields {
            match key.as_str() {
                "WindGenType3IEC.dipmax" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.dipmax = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.dipmax = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "WindGenType3IEC.diqmax" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.diqmax = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.diqmax = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "WindGenType3IEC.xs" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.xs = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.xs = Some(v); } }
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
