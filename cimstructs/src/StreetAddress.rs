/// General purpose street and postal address information.
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct StreetAddress {
    pub id: String,
    /// The language in which the address is specified, using ISO 639-1 two digit language code.
    pub language: String,
    /// Post office box.
    pub po_box: String,
    /// Postal code for the address.
    pub postal_code: String,
    /// Status of this address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<super::base::MridRef>,
    /// Street detail.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub street_detail: Option<super::base::MridRef>,
    /// Town detail.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub town_detail: Option<super::base::MridRef>,
}
impl crate::base::CimElement for StreetAddress {
    fn mrid(&self) -> &str { &self.id }
    fn type_name(&self) -> &'static str { "StreetAddress" }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn to_json_value(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or(serde_json::Value::Null)
    }
    fn to_block(&self) -> crate::base::RdfBlock {
        let mut block = crate::base::RdfBlock {
            type_name: "StreetAddress".to_string(),
            mrid: self.id.clone(),
            fields: std::collections::HashMap::new(),
            duplicate_fields: std::collections::HashSet::new(),
        };
        if !self.language.is_empty() {
            block.fields.insert("StreetAddress.language".into(), crate::base::FieldValue::Text(self.language.clone()));
        }
        if !self.po_box.is_empty() {
            block.fields.insert("StreetAddress.poBox".into(), crate::base::FieldValue::Text(self.po_box.clone()));
        }
        if !self.postal_code.is_empty() {
            block.fields.insert("StreetAddress.postalCode".into(), crate::base::FieldValue::Text(self.postal_code.clone()));
        }
        if let Some(ref v) = self.status {
            block.fields.insert("StreetAddress.status".into(), crate::base::FieldValue::Resource(v.mrid.clone()));
        }
        if let Some(ref v) = self.street_detail {
            block.fields.insert("StreetAddress.streetDetail".into(), crate::base::FieldValue::Resource(v.mrid.clone()));
        }
        if let Some(ref v) = self.town_detail {
            block.fields.insert("StreetAddress.townDetail".into(), crate::base::FieldValue::Resource(v.mrid.clone()));
        }
        block
    }
}

impl StreetAddress {
    pub fn from_block(b: &crate::base::RdfBlock) -> Self {
        let mut obj = Self::default();
        obj.id.clone_from(&b.mrid);
        for (key, val) in &b.fields {
            match key.as_str() {
                "StreetAddress.language" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.language = sv.clone(); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.language = sv.clone(); }
                        }
                        _ => {}
                    }
                }
                "StreetAddress.poBox" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.po_box = sv.clone(); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.po_box = sv.clone(); }
                        }
                        _ => {}
                    }
                }
                "StreetAddress.postalCode" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.postal_code = sv.clone(); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.postal_code = sv.clone(); }
                        }
                        _ => {}
                    }
                }
                "StreetAddress.status" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.status = Some(crate::base::MridRef { mrid: sv.clone() });
                    }
                }
                "StreetAddress.streetDetail" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.street_detail = Some(crate::base::MridRef { mrid: sv.clone() });
                    }
                }
                "StreetAddress.townDetail" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.town_detail = Some(crate::base::MridRef { mrid: sv.clone() });
                    }
                }
                _ => {}
            }
        }
        obj
    }
}
