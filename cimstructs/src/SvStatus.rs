/// State variable for status.
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct SvStatus {
    pub id: String,
    /// The conducting equipment associated with the status state variable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conducting_equipment: Option<super::base::MridRef>,
    /// The in service status as a result of topology processing. It indicates if the equipment is considered as energized by the power flow. It reflects if the equipment is connected within a solvable island. It does not necessarily reflect whether or not the island was solved by the power flow.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub in_service: Option<bool>,
}
impl crate::base::CimElement for SvStatus {
    fn mrid(&self) -> &str { &self.id }
    fn type_name(&self) -> &'static str { "SvStatus" }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn to_json_value(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or(serde_json::Value::Null)
    }
    fn to_block(&self) -> crate::base::RdfBlock {
        let mut block = crate::base::RdfBlock {
            type_name: "SvStatus".to_string(),
            mrid: self.id.clone(),
            fields: std::collections::HashMap::new(),
            duplicate_fields: std::collections::HashSet::new(),
        };
        if let Some(ref v) = self.conducting_equipment {
            block.fields.insert("SvStatus.ConductingEquipment".into(), crate::base::FieldValue::Resource(v.mrid.clone()));
        }
        if let Some(v) = self.in_service {
            block.fields.insert("SvStatus.inService".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        block
    }
}

impl SvStatus {
    pub fn from_block(b: &crate::base::RdfBlock) -> Self {
        let mut obj = Self::default();
        obj.id.clone_from(&b.mrid);
        for (key, val) in &b.fields {
            match key.as_str() {
                "SvStatus.ConductingEquipment" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.conducting_equipment = Some(crate::base::MridRef { mrid: sv.clone() });
                    }
                }
                "SvStatus.inService" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.in_service = Some(sv.trim() == "true"); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.in_service = Some(sv.trim() == "true"); }
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
