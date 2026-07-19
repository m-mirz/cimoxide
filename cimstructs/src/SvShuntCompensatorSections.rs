/// State variable for the number of sections in service for a shunt compensator.
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct SvShuntCompensatorSections {
    pub id: String,
    /// The shunt compensator for which the state applies.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shunt_compensator: Option<super::base::MridRef>,
    /// The number of sections in service as a continuous variable. The attribute shall be a positive value or zero. To get integer value scale with ShuntCompensator.bPerSection.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sections: Option<f64>,
}
impl crate::base::CimElement for SvShuntCompensatorSections {
    fn mrid(&self) -> &str { &self.id }
    fn type_name(&self) -> &'static str { "SvShuntCompensatorSections" }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn to_json_value(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or(serde_json::Value::Null)
    }
    fn to_block(&self) -> crate::base::RdfBlock {
        let mut block = crate::base::RdfBlock {
            type_name: "SvShuntCompensatorSections".to_string(),
            mrid: self.id.clone(),
            fields: std::collections::HashMap::new(),
            duplicate_fields: std::collections::HashSet::new(),
        };
        if let Some(ref v) = self.shunt_compensator {
            block.fields.insert("SvShuntCompensatorSections.ShuntCompensator".into(), crate::base::FieldValue::Resource(v.mrid.clone()));
        }
        if let Some(v) = self.sections {
            block.fields.insert("SvShuntCompensatorSections.sections".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        block
    }
}

impl SvShuntCompensatorSections {
    pub fn from_block(b: &crate::base::RdfBlock) -> Self {
        let mut obj = Self::default();
        obj.id.clone_from(&b.mrid);
        for (key, val) in &b.fields {
            match key.as_str() {
                "SvShuntCompensatorSections.ShuntCompensator" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.shunt_compensator = Some(crate::base::MridRef { mrid: sv.clone() });
                    }
                }
                "SvShuntCompensatorSections.sections" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.sections = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.sections = Some(v); } }
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
