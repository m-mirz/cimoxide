/// Limit values for Accumulator measurements.
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct AccumulatorLimit {
    #[serde(flatten)]
    pub base: super::Limit,
    /// The set of limits.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit_set: Option<super::base::MridRef>,
    /// The value to supervise against. The value is positive.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<i64>,
}
impl crate::base::CimElement for AccumulatorLimit {
    fn mrid(&self) -> &str { &self.base.base.id }
    fn type_name(&self) -> &'static str { "AccumulatorLimit" }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn to_json_value(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or(serde_json::Value::Null)
    }
    fn to_block(&self) -> crate::base::RdfBlock {
        let mut block = self.base.to_block();
        block.type_name = "AccumulatorLimit".to_string();
        if let Some(ref v) = self.limit_set {
            block.fields.insert("AccumulatorLimit.LimitSet".into(), crate::base::FieldValue::Resource(v.mrid.clone()));
        }
        if let Some(v) = self.value {
            block.fields.insert("AccumulatorLimit.value".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        block
    }
}

impl AccumulatorLimit {
    pub fn from_block(b: &crate::base::RdfBlock) -> Self {
        let mut obj = Self::default();
        obj.base.base.id.clone_from(&b.mrid);
        for (key, val) in &b.fields {
            match key.as_str() {
                "AccumulatorLimit.LimitSet" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.limit_set = Some(crate::base::MridRef { mrid: sv.clone() });
                    }
                }
                "AccumulatorLimit.value" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.value = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.value = Some(v); } }
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
