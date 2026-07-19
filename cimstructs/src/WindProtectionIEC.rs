/// The grid protection model includes protection against over- and under-voltage, and against over- and under-frequency. Reference: IEC 61400-27-1:2015, 5.6.6.
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct WindProtectionIEC {
    #[serde(flatten)]
    pub base: super::IdentifiedObject,
    /// Maximum rate of change of frequency (dFmax). It is a type-dependent parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dfimax: Option<f64>,
    /// Wind turbine over frequency protection activation threshold (fover). It is a project-dependent parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fover: Option<f64>,
    /// Wind turbine under frequency protection activation threshold (funder). It is a project-dependent parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub funder: Option<f64>,
    /// Zero crossing measurement mode (Mzc). It is a type-dependent parameter. true = WT protection system uses zero crossings to detect frequency (1 in the IEC model) false = WT protection system does not use zero crossings to detect frequency (0 in the IEC model).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mzc: Option<bool>,
    /// Time interval of moving average window (TfMA) (>= 0). It is a type-dependent parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tfma: Option<f64>,
    /// Wind turbine over voltage protection activation threshold (uover). It is a project-dependent parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uover: Option<f64>,
    /// Wind turbine under voltage protection activation threshold (uunder). It is a project-dependent parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uunder: Option<f64>,
}
impl crate::base::CimElement for WindProtectionIEC {
    fn mrid(&self) -> &str { &self.base.id }
    fn type_name(&self) -> &'static str { "WindProtectionIEC" }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn to_json_value(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or(serde_json::Value::Null)
    }
    fn to_block(&self) -> crate::base::RdfBlock {
        let mut block = self.base.to_block();
        block.type_name = "WindProtectionIEC".to_string();
        if let Some(v) = self.dfimax {
            block.fields.insert("WindProtectionIEC.dfimax".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.fover {
            block.fields.insert("WindProtectionIEC.fover".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.funder {
            block.fields.insert("WindProtectionIEC.funder".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.mzc {
            block.fields.insert("WindProtectionIEC.mzc".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.tfma {
            block.fields.insert("WindProtectionIEC.tfma".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.uover {
            block.fields.insert("WindProtectionIEC.uover".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.uunder {
            block.fields.insert("WindProtectionIEC.uunder".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        block
    }
}

impl WindProtectionIEC {
    pub fn from_block(b: &crate::base::RdfBlock) -> Self {
        let mut obj = Self::default();
        obj.base.id.clone_from(&b.mrid);
        for (key, val) in &b.fields {
            match key.as_str() {
                "WindProtectionIEC.dfimax" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.dfimax = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.dfimax = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "WindProtectionIEC.fover" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.fover = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.fover = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "WindProtectionIEC.funder" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.funder = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.funder = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "WindProtectionIEC.mzc" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.mzc = Some(sv.trim() == "true"); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.mzc = Some(sv.trim() == "true"); }
                        }
                        _ => {}
                    }
                }
                "WindProtectionIEC.tfma" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.tfma = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.tfma = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "WindProtectionIEC.uover" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.uover = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.uover = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "WindProtectionIEC.uunder" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.uunder = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.uunder = Some(v); } }
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
