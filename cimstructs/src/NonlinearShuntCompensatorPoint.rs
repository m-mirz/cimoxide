/// A non linear shunt compensator bank or section admittance value. The number of NonlinearShuntCompenstorPoint instances associated with a NonlinearShuntCompensator shall be equal to ShuntCompensator.maximumSections. ShuntCompensator.sections shall only be set to one of the NonlinearShuntCompenstorPoint.sectionNumber. There is no interpolation between NonlinearShuntCompenstorPoint-s.
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct NonlinearShuntCompensatorPoint {
    pub id: String,
    /// Non-linear shunt compensator owning this point.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nonlinear_shunt_compensator: Option<super::base::MridRef>,
    /// Positive sequence shunt (charging) susceptance per section.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub b: Option<f64>,
    /// Zero sequence shunt (charging) susceptance per section.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub b0: Option<f64>,
    /// Positive sequence shunt (charging) conductance per section.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub g: Option<f64>,
    /// Zero sequence shunt (charging) conductance per section.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub g0: Option<f64>,
    /// The number of the section.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub section_number: Option<i64>,
}
impl crate::base::CimElement for NonlinearShuntCompensatorPoint {
    fn mrid(&self) -> &str { &self.id }
    fn type_name(&self) -> &'static str { "NonlinearShuntCompensatorPoint" }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn to_json_value(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or(serde_json::Value::Null)
    }
    fn to_block(&self) -> crate::base::RdfBlock {
        let mut block = crate::base::RdfBlock {
            type_name: "NonlinearShuntCompensatorPoint".to_string(),
            mrid: self.id.clone(),
            fields: std::collections::HashMap::new(),
            duplicate_fields: std::collections::HashSet::new(),
        };
        if let Some(ref v) = self.nonlinear_shunt_compensator {
            block.fields.insert("NonlinearShuntCompensatorPoint.NonlinearShuntCompensator".into(), crate::base::FieldValue::Resource(v.mrid.clone()));
        }
        if let Some(v) = self.b {
            block.fields.insert("NonlinearShuntCompensatorPoint.b".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.b0 {
            block.fields.insert("NonlinearShuntCompensatorPoint.b0".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.g {
            block.fields.insert("NonlinearShuntCompensatorPoint.g".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.g0 {
            block.fields.insert("NonlinearShuntCompensatorPoint.g0".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.section_number {
            block.fields.insert("NonlinearShuntCompensatorPoint.sectionNumber".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        block
    }
}

impl NonlinearShuntCompensatorPoint {
    pub fn from_block(b: &crate::base::RdfBlock) -> Self {
        let mut obj = Self::default();
        obj.id.clone_from(&b.mrid);
        for (key, val) in &b.fields {
            match key.as_str() {
                "NonlinearShuntCompensatorPoint.NonlinearShuntCompensator" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.nonlinear_shunt_compensator = Some(crate::base::MridRef { mrid: sv.clone() });
                    }
                }
                "NonlinearShuntCompensatorPoint.b" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.b = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.b = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "NonlinearShuntCompensatorPoint.b0" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.b0 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.b0 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "NonlinearShuntCompensatorPoint.g" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.g = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.g = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "NonlinearShuntCompensatorPoint.g0" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.g0 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.g0 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "NonlinearShuntCompensatorPoint.sectionNumber" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.section_number = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.section_number = Some(v); } }
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
