/// This is a root class to provide common identification for all classes needing identification and naming attributes.
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct IdentifiedObject {
    pub id: String,
    /// The description is a free human readable text describing or naming the object. It may be non unique and may not correlate to a naming hierarchy.
    pub description: String,
    /// The attribute is used for an exchange of the EIC code (Energy identification Code). The length of the string is 16 characters as defined by the EIC code. For details on EIC scheme please refer to ENTSO-E web site.
    pub energy_ident_code_eic: String,
    /// Master resource identifier issued by a model authority. The mRID is unique within an exchange context. Global uniqueness is easily achieved by using a UUID, as specified in RFC 4122, for the mRID. The use of UUID is strongly recommended. For CIMXML data files in RDF syntax conforming to IEC 61970-552, the mRID is mapped to rdf:ID or rdf:about attributes that identify CIM object elements.
    pub m_rid: String,
    /// The name is any free human readable and possibly non unique text naming the object.
    pub name: String,
    /// The attribute is used for an exchange of a human readable short name with length of the string 12 characters maximum.
    pub short_name: String,
}
impl crate::base::CimElement for IdentifiedObject {
    fn mrid(&self) -> &str { &self.id }
    fn type_name(&self) -> &'static str { "IdentifiedObject" }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn to_json_value(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or(serde_json::Value::Null)
    }
    fn to_block(&self) -> crate::base::RdfBlock {
        let mut block = crate::base::RdfBlock {
            type_name: "IdentifiedObject".to_string(),
            mrid: self.id.clone(),
            fields: std::collections::HashMap::new(),
            duplicate_fields: std::collections::HashSet::new(),
        };
        if !self.description.is_empty() {
            block.fields.insert("IdentifiedObject.description".into(), crate::base::FieldValue::Text(self.description.clone()));
        }
        if !self.energy_ident_code_eic.is_empty() {
            block.fields.insert("IdentifiedObject.energyIdentCodeEic".into(), crate::base::FieldValue::Text(self.energy_ident_code_eic.clone()));
        }
        if !self.m_rid.is_empty() {
            block.fields.insert("IdentifiedObject.mRID".into(), crate::base::FieldValue::Text(self.m_rid.clone()));
        }
        if !self.name.is_empty() {
            block.fields.insert("IdentifiedObject.name".into(), crate::base::FieldValue::Text(self.name.clone()));
        }
        if !self.short_name.is_empty() {
            block.fields.insert("IdentifiedObject.shortName".into(), crate::base::FieldValue::Text(self.short_name.clone()));
        }
        block
    }
}

impl IdentifiedObject {
    pub fn from_block(b: &crate::base::RdfBlock) -> Self {
        let mut obj = Self::default();
        obj.id.clone_from(&b.mrid);
        for (key, val) in &b.fields {
            match key.as_str() {
                "IdentifiedObject.description" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.description = sv.clone(); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.description = sv.clone(); }
                        }
                        _ => {}
                    }
                }
                "IdentifiedObject.energyIdentCodeEic" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.energy_ident_code_eic = sv.clone(); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.energy_ident_code_eic = sv.clone(); }
                        }
                        _ => {}
                    }
                }
                "IdentifiedObject.mRID" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.m_rid = sv.clone(); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.m_rid = sv.clone(); }
                        }
                        _ => {}
                    }
                }
                "IdentifiedObject.name" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.name = sv.clone(); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.name = sv.clone(); }
                        }
                        _ => {}
                    }
                }
                "IdentifiedObject.shortName" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.short_name = sv.clone(); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.short_name = sv.clone(); }
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
