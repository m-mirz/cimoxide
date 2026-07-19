/// Reference frame rotation model. Reference: IEC 61400-27-1:2015, 5.6.3.5.
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct WindRefFrameRotIEC {
    #[serde(flatten)]
    pub base: super::IdentifiedObject,
    /// Time constant for PLL first order filter model (TPLL) (>= 0). It is a type-dependent parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tpll: Option<f64>,
    /// Voltage below which the angle of the voltage is filtered and possibly also frozen (uPLL1). It is a type-dependent parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upll1: Option<f64>,
    /// Voltage (uPLL2) below which the angle of the voltage is frozen if uPLL2 is smaller or equal to uPLL1 . It is a type-dependent parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upll2: Option<f64>,
}
impl crate::base::CimElement for WindRefFrameRotIEC {
    fn mrid(&self) -> &str { &self.base.id }
    fn type_name(&self) -> &'static str { "WindRefFrameRotIEC" }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn to_json_value(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or(serde_json::Value::Null)
    }
    fn to_block(&self) -> crate::base::RdfBlock {
        let mut block = self.base.to_block();
        block.type_name = "WindRefFrameRotIEC".to_string();
        if let Some(v) = self.tpll {
            block.fields.insert("WindRefFrameRotIEC.tpll".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.upll1 {
            block.fields.insert("WindRefFrameRotIEC.upll1".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.upll2 {
            block.fields.insert("WindRefFrameRotIEC.upll2".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        block
    }
}

impl WindRefFrameRotIEC {
    pub fn from_block(b: &crate::base::RdfBlock) -> Self {
        let mut obj = Self::default();
        obj.base.id.clone_from(&b.mrid);
        for (key, val) in &b.fields {
            match key.as_str() {
                "WindRefFrameRotIEC.tpll" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.tpll = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.tpll = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "WindRefFrameRotIEC.upll1" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.upll1 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.upll1 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "WindRefFrameRotIEC.upll2" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.upll2 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.upll2 = Some(v); } }
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
