/// Current status information relevant to an entity.
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct Status {
    pub id: String,
    /// Date and time for which status 'value' applies.
    pub date_time: String,
    /// Reason code or explanation for why an object went to the current status 'value'.
    pub reason: String,
    /// Pertinent information regarding the current 'value', as free form text.
    pub remark: String,
    /// Status value at 'dateTime'; prior status changes may have been kept in instances of activity records associated with the object to which this status applies.
    pub value: String,
}
impl crate::base::CimElement for Status {
    fn mrid(&self) -> &str { &self.id }
    fn type_name(&self) -> &'static str { "Status" }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn to_json_value(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or(serde_json::Value::Null)
    }
    fn to_block(&self) -> crate::base::RdfBlock {
        let mut block = crate::base::RdfBlock {
            type_name: "Status".to_string(),
            mrid: self.id.clone(),
            fields: std::collections::HashMap::new(),
            duplicate_fields: std::collections::HashSet::new(),
        };
        if !self.date_time.is_empty() {
            block.fields.insert("Status.dateTime".into(), crate::base::FieldValue::Text(self.date_time.clone()));
        }
        if !self.reason.is_empty() {
            block.fields.insert("Status.reason".into(), crate::base::FieldValue::Text(self.reason.clone()));
        }
        if !self.remark.is_empty() {
            block.fields.insert("Status.remark".into(), crate::base::FieldValue::Text(self.remark.clone()));
        }
        if !self.value.is_empty() {
            block.fields.insert("Status.value".into(), crate::base::FieldValue::Text(self.value.clone()));
        }
        block
    }
}

impl Status {
    pub fn from_block(b: &crate::base::RdfBlock) -> Self {
        let mut obj = Self::default();
        obj.id.clone_from(&b.mrid);
        for (key, val) in &b.fields {
            match key.as_str() {
                "Status.dateTime" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.date_time = sv.clone(); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.date_time = sv.clone(); }
                        }
                        _ => {}
                    }
                }
                "Status.reason" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.reason = sv.clone(); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.reason = sv.clone(); }
                        }
                        _ => {}
                    }
                }
                "Status.remark" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.remark = sv.clone(); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.remark = sv.clone(); }
                        }
                        _ => {}
                    }
                }
                "Status.value" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.value = sv.clone(); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.value = sv.clone(); }
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
