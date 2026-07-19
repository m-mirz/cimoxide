/// The operational meaning of a category of limits.
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct OperationalLimitType {
    #[serde(flatten)]
    pub base: super::IdentifiedObject,
    /// The nominal acceptable duration of the limit. Limits are commonly expressed in terms of the time limit for which the limit is normally acceptable. The actual acceptable duration of a specific limit may depend on other local factors such as temperature or wind speed. The attribute has meaning only if the flag isInfiniteDuration is set to false, hence it shall not be exchanged when isInfiniteDuration is set to true.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acceptable_duration: Option<f64>,
    /// The direction of the limit.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direction: Option<super::base::UriRef>,
    /// Defines if the operational limit type has infinite duration. If true, the limit has infinite duration. If false, the limit has definite duration which is defined by the attribute acceptableDuration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_infinite_duration: Option<bool>,
    /// Types of limits defined in the ENTSO-E Operational Handbook Policy 3.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<super::base::MridRef>,
}
impl crate::base::CimElement for OperationalLimitType {
    fn mrid(&self) -> &str { &self.base.id }
    fn type_name(&self) -> &'static str { "OperationalLimitType" }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn to_json_value(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or(serde_json::Value::Null)
    }
    fn to_block(&self) -> crate::base::RdfBlock {
        let mut block = self.base.to_block();
        block.type_name = "OperationalLimitType".to_string();
        if let Some(v) = self.acceptable_duration {
            block.fields.insert("OperationalLimitType.acceptableDuration".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(ref v) = self.direction {
            block.fields.insert("OperationalLimitType.direction".into(), crate::base::FieldValue::Resource(v.uri.clone()));
        }
        if let Some(v) = self.is_infinite_duration {
            block.fields.insert("OperationalLimitType.isInfiniteDuration".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(ref v) = self.kind {
            block.fields.insert("OperationalLimitType.kind".into(), crate::base::FieldValue::Resource(v.mrid.clone()));
        }
        block
    }
}

impl OperationalLimitType {
    pub fn from_block(b: &crate::base::RdfBlock) -> Self {
        let mut obj = Self::default();
        obj.base.id.clone_from(&b.mrid);
        for (key, val) in &b.fields {
            match key.as_str() {
                "OperationalLimitType.acceptableDuration" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.acceptable_duration = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.acceptable_duration = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "OperationalLimitType.direction" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.direction = Some(crate::base::UriRef { uri: sv.clone() });
                    }
                }
                "OperationalLimitType.isInfiniteDuration" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.is_infinite_duration = Some(sv.trim() == "true"); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.is_infinite_duration = Some(sv.trim() == "true"); }
                        }
                        _ => {}
                    }
                }
                "OperationalLimitType.kind" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.kind = Some(crate::base::MridRef { mrid: sv.clone() });
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
