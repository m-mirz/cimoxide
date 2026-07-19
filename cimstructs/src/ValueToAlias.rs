/// Describes the translation of one particular value into a name, e.g. 1 as 'Open'.
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct ValueToAlias {
    #[serde(flatten)]
    pub base: super::IdentifiedObject,
    /// The ValueAliasSet having the ValueToAlias mappings.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value_alias_set: Option<super::base::MridRef>,
    /// The value that is mapped.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<i64>,
}
impl crate::base::CimElement for ValueToAlias {
    fn mrid(&self) -> &str { &self.base.id }
    fn type_name(&self) -> &'static str { "ValueToAlias" }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn to_json_value(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or(serde_json::Value::Null)
    }
    fn to_block(&self) -> crate::base::RdfBlock {
        let mut block = self.base.to_block();
        block.type_name = "ValueToAlias".to_string();
        if let Some(ref v) = self.value_alias_set {
            block.fields.insert("ValueToAlias.ValueAliasSet".into(), crate::base::FieldValue::Resource(v.mrid.clone()));
        }
        if let Some(v) = self.value {
            block.fields.insert("ValueToAlias.value".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        block
    }
}

impl ValueToAlias {
    pub fn from_block(b: &crate::base::RdfBlock) -> Self {
        let mut obj = Self::default();
        obj.base.id.clone_from(&b.mrid);
        for (key, val) in &b.fields {
            match key.as_str() {
                "ValueToAlias.ValueAliasSet" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.value_alias_set = Some(crate::base::MridRef { mrid: sv.clone() });
                    }
                }
                "ValueToAlias.value" => {
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
