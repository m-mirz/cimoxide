/// Limit on active power flow.
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct ActivePowerLimit {
    #[serde(flatten)]
    pub base: super::OperationalLimit,
    /// The normal value of active power limit. The attribute shall be a positive value or zero.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub normal_value: Option<f64>,
    /// Value of active power limit. The attribute shall be a positive value or zero.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<f64>,
}
impl crate::base::CimElement for ActivePowerLimit {
    fn mrid(&self) -> &str { &self.base.base.id }
    fn type_name(&self) -> &'static str { "ActivePowerLimit" }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn to_json_value(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or(serde_json::Value::Null)
    }
    fn to_block(&self) -> crate::base::RdfBlock {
        let mut block = self.base.to_block();
        block.type_name = "ActivePowerLimit".to_string();
        if let Some(v) = self.normal_value {
            block.fields.insert("ActivePowerLimit.normalValue".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.value {
            block.fields.insert("ActivePowerLimit.value".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        block
    }
}

impl ActivePowerLimit {
    pub fn from_block(b: &crate::base::RdfBlock) -> Self {
        let mut obj = Self::default();
        obj.base.base.id.clone_from(&b.mrid);
        for (key, val) in &b.fields {
            match key.as_str() {
                "ActivePowerLimit.normalValue" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.normal_value = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.normal_value = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ActivePowerLimit.value" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.value = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.value = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "OperationalLimit.OperationalLimitSet" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.base.operational_limit_set = Some(crate::base::MridRef { mrid: sv.clone() });
                    }
                }
                "OperationalLimit.OperationalLimitType" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.base.operational_limit_type = Some(crate::base::MridRef { mrid: sv.clone() });
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
