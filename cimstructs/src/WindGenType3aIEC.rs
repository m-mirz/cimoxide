/// IEC type 3A generator set model. Reference: IEC 61400-27-1:2015, 5.6.3.2.
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct WindGenType3aIEC {
    #[serde(flatten)]
    pub base: super::WindGenType3IEC,
    /// Current PI controller proportional gain (KPc). It is a type-dependent parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kpc: Option<f64>,
    /// Current PI controller integration time constant (TIc) (>= 0). It is a type-dependent parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tic: Option<f64>,
}
impl crate::base::CimElement for WindGenType3aIEC {
    fn mrid(&self) -> &str { &self.base.base.id }
    fn type_name(&self) -> &'static str { "WindGenType3aIEC" }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn to_json_value(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or(serde_json::Value::Null)
    }
    fn to_block(&self) -> crate::base::RdfBlock {
        let mut block = self.base.to_block();
        block.type_name = "WindGenType3aIEC".to_string();
        if let Some(v) = self.kpc {
            block.fields.insert("WindGenType3aIEC.kpc".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.tic {
            block.fields.insert("WindGenType3aIEC.tic".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        block
    }
}

impl WindGenType3aIEC {
    pub fn from_block(b: &crate::base::RdfBlock) -> Self {
        let mut obj = Self::default();
        obj.base.base.id.clone_from(&b.mrid);
        for (key, val) in &b.fields {
            match key.as_str() {
                "WindGenType3aIEC.kpc" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.kpc = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.kpc = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "WindGenType3aIEC.tic" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.tic = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.tic = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "WindGenType3IEC.dipmax" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.base.dipmax = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.base.dipmax = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "WindGenType3IEC.diqmax" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.base.diqmax = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.base.diqmax = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "WindGenType3IEC.xs" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.base.xs = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.base.xs = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "IdentifiedObject.description" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.base.base.description = sv.clone(); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.base.base.description = sv.clone(); }
                        }
                        _ => {}
                    }
                }
                "IdentifiedObject.energyIdentCodeEic" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.base.base.energy_ident_code_eic = sv.clone(); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.base.base.energy_ident_code_eic = sv.clone(); }
                        }
                        _ => {}
                    }
                }
                "IdentifiedObject.mRID" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.base.base.m_rid = sv.clone(); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.base.base.m_rid = sv.clone(); }
                        }
                        _ => {}
                    }
                }
                "IdentifiedObject.name" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.base.base.name = sv.clone(); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.base.base.name = sv.clone(); }
                        }
                        _ => {}
                    }
                }
                "IdentifiedObject.shortName" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.base.base.short_name = sv.clone(); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.base.base.short_name = sv.clone(); }
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
