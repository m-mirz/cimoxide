/// Town details, in the context of address.
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct TownDetail {
    pub id: String,
    /// Town code.
    pub code: String,
    /// Name of the country.
    pub country: String,
    /// Town name.
    pub name: String,
    /// Town section. For example, it is common for there to be 36 sections per township.
    pub section: String,
    /// Name of the state or province.
    pub state_or_province: String,
}
impl crate::base::CimElement for TownDetail {
    fn mrid(&self) -> &str { &self.id }
    fn type_name(&self) -> &'static str { "TownDetail" }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn to_json_value(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or(serde_json::Value::Null)
    }
    fn to_block(&self) -> crate::base::RdfBlock {
        let mut block = crate::base::RdfBlock {
            type_name: "TownDetail".to_string(),
            mrid: self.id.clone(),
            fields: std::collections::HashMap::new(),
            duplicate_fields: std::collections::HashSet::new(),
        };
        if !self.code.is_empty() {
            block.fields.insert("TownDetail.code".into(), crate::base::FieldValue::Text(self.code.clone()));
        }
        if !self.country.is_empty() {
            block.fields.insert("TownDetail.country".into(), crate::base::FieldValue::Text(self.country.clone()));
        }
        if !self.name.is_empty() {
            block.fields.insert("TownDetail.name".into(), crate::base::FieldValue::Text(self.name.clone()));
        }
        if !self.section.is_empty() {
            block.fields.insert("TownDetail.section".into(), crate::base::FieldValue::Text(self.section.clone()));
        }
        if !self.state_or_province.is_empty() {
            block.fields.insert("TownDetail.stateOrProvince".into(), crate::base::FieldValue::Text(self.state_or_province.clone()));
        }
        block
    }
}

impl TownDetail {
    pub fn from_block(b: &crate::base::RdfBlock) -> Self {
        let mut obj = Self::default();
        obj.id.clone_from(&b.mrid);
        for (key, val) in &b.fields {
            match key.as_str() {
                "TownDetail.code" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.code = sv.clone(); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.code = sv.clone(); }
                        }
                        _ => {}
                    }
                }
                "TownDetail.country" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.country = sv.clone(); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.country = sv.clone(); }
                        }
                        _ => {}
                    }
                }
                "TownDetail.name" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.name = sv.clone(); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.name = sv.clone(); }
                        }
                        _ => {}
                    }
                }
                "TownDetail.section" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.section = sv.clone(); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.section = sv.clone(); }
                        }
                        _ => {}
                    }
                }
                "TownDetail.stateOrProvince" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.state_or_province = sv.clone(); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.state_or_province = sv.clone(); }
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
