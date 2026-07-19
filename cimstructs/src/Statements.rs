/// It represent a set of Definition and/or Description elements.
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct Statements {
    pub id: String,
    /// Statement object.
    pub object: String,
    /// Statement predicate.
    pub predicate: String,
    /// Statement subject.
    pub subject: String,
}
impl crate::base::CimElement for Statements {
    fn mrid(&self) -> &str { &self.id }
    fn type_name(&self) -> &'static str { "Statements" }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn to_json_value(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or(serde_json::Value::Null)
    }
    fn to_block(&self) -> crate::base::RdfBlock {
        let mut block = crate::base::RdfBlock {
            type_name: "Statements".to_string(),
            mrid: self.id.clone(),
            fields: std::collections::HashMap::new(),
            duplicate_fields: std::collections::HashSet::new(),
        };
        if !self.object.is_empty() {
            block.fields.insert("Statements.object".into(), crate::base::FieldValue::Text(self.object.clone()));
        }
        if !self.predicate.is_empty() {
            block.fields.insert("Statements.predicate".into(), crate::base::FieldValue::Text(self.predicate.clone()));
        }
        if !self.subject.is_empty() {
            block.fields.insert("Statements.subject".into(), crate::base::FieldValue::Text(self.subject.clone()));
        }
        block
    }
}

impl Statements {
    pub fn from_block(b: &crate::base::RdfBlock) -> Self {
        let mut obj = Self::default();
        obj.id.clone_from(&b.mrid);
        for (key, val) in &b.fields {
            match key.as_str() {
                "Statements.object" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.object = sv.clone(); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.object = sv.clone(); }
                        }
                        _ => {}
                    }
                }
                "Statements.predicate" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.predicate = sv.clone(); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.predicate = sv.clone(); }
                        }
                        _ => {}
                    }
                }
                "Statements.subject" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.subject = sv.clone(); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.subject = sv.clone(); }
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
