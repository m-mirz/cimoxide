/// A value and normal value associated with a specific kind of limit. The sub class value and normalValue attributes vary inversely to the associated OperationalLimitType.acceptableDuration (acceptableDuration for short). If a particular piece of equipment has multiple operational limits of the same kind (apparent power, current, etc.), the limit with the greatest acceptableDuration shall have the smallest limit value and the limit with the smallest acceptableDuration shall have the largest limit value. Note: A large current can only be allowed to flow through a piece of equipment for a short duration without causing damage, but a lesser current can be allowed to flow for a longer duration.
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct OperationalLimit {
    #[serde(flatten)]
    pub base: super::IdentifiedObject,
    /// The limit set to which the limit values belong.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operational_limit_set: Option<super::base::MridRef>,
    /// The limit type associated with this limit.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operational_limit_type: Option<super::base::MridRef>,
}
impl crate::base::CimElement for OperationalLimit {
    fn mrid(&self) -> &str { &self.base.id }
    fn type_name(&self) -> &'static str { "OperationalLimit" }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn to_json_value(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or(serde_json::Value::Null)
    }
    fn to_block(&self) -> crate::base::RdfBlock {
        let mut block = self.base.to_block();
        block.type_name = "OperationalLimit".to_string();
        if let Some(ref v) = self.operational_limit_set {
            block.fields.insert("OperationalLimit.OperationalLimitSet".into(), crate::base::FieldValue::Resource(v.mrid.clone()));
        }
        if let Some(ref v) = self.operational_limit_type {
            block.fields.insert("OperationalLimit.OperationalLimitType".into(), crate::base::FieldValue::Resource(v.mrid.clone()));
        }
        block
    }
}

impl OperationalLimit {
    pub fn from_block(b: &crate::base::RdfBlock) -> Self {
        let mut obj = Self::default();
        obj.base.id.clone_from(&b.mrid);
        for (key, val) in &b.fields {
            match key.as_str() {
                "OperationalLimit.OperationalLimitSet" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.operational_limit_set = Some(crate::base::MridRef { mrid: sv.clone() });
                    }
                }
                "OperationalLimit.OperationalLimitType" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.operational_limit_type = Some(crate::base::MridRef { mrid: sv.clone() });
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
