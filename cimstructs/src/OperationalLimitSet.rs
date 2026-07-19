/// A set of limits associated with equipment. Sets of limits might apply to a specific temperature, or season for example. A set of limits may contain different severities of limit levels that would apply to the same equipment. The set may contain limits of different types such as apparent power and current limits or high and low voltage limits that are logically applied together as a set.
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct OperationalLimitSet {
    #[serde(flatten)]
    pub base: super::IdentifiedObject,
    /// The equipment to which the limit set applies.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub equipment: Option<super::base::MridRef>,
    /// The terminal where the operational limit set apply.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub terminal: Option<super::base::MridRef>,
}
impl crate::base::CimElement for OperationalLimitSet {
    fn mrid(&self) -> &str { &self.base.id }
    fn type_name(&self) -> &'static str { "OperationalLimitSet" }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn to_json_value(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or(serde_json::Value::Null)
    }
    fn to_block(&self) -> crate::base::RdfBlock {
        let mut block = self.base.to_block();
        block.type_name = "OperationalLimitSet".to_string();
        if let Some(ref v) = self.equipment {
            block.fields.insert("OperationalLimitSet.Equipment".into(), crate::base::FieldValue::Resource(v.mrid.clone()));
        }
        if let Some(ref v) = self.terminal {
            block.fields.insert("OperationalLimitSet.Terminal".into(), crate::base::FieldValue::Resource(v.mrid.clone()));
        }
        block
    }
}

impl OperationalLimitSet {
    pub fn from_block(b: &crate::base::RdfBlock) -> Self {
        let mut obj = Self::default();
        obj.base.id.clone_from(&b.mrid);
        for (key, val) in &b.fields {
            match key.as_str() {
                "OperationalLimitSet.Equipment" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.equipment = Some(crate::base::MridRef { mrid: sv.clone() });
                    }
                }
                "OperationalLimitSet.Terminal" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.terminal = Some(crate::base::MridRef { mrid: sv.clone() });
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
