/// A point in a given space defined by 3 coordinates and associated to a diagram object. The coordinates may be positive or negative as the origin does not have to be in the corner of a diagram.
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct DiagramObjectPoint {
    pub id: String,
    /// The diagram object with which the points are associated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub diagram_object: Option<super::base::MridRef>,
    /// The 'glue' point to which this point is associated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub diagram_object_glue_point: Option<super::base::MridRef>,
    /// The sequence position of the point, used for defining the order of points for diagram objects acting as a polyline or polygon with more than one point. The attribute shall be a positive value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sequence_number: Option<i64>,
    /// The X coordinate of this point.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x_position: Option<f64>,
    /// The Y coordinate of this point.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub y_position: Option<f64>,
    /// The Z coordinate of this point.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub z_position: Option<f64>,
}
impl crate::base::CimElement for DiagramObjectPoint {
    fn mrid(&self) -> &str { &self.id }
    fn type_name(&self) -> &'static str { "DiagramObjectPoint" }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn to_json_value(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or(serde_json::Value::Null)
    }
    fn to_block(&self) -> crate::base::RdfBlock {
        let mut block = crate::base::RdfBlock {
            type_name: "DiagramObjectPoint".to_string(),
            mrid: self.id.clone(),
            fields: std::collections::HashMap::new(),
            duplicate_fields: std::collections::HashSet::new(),
        };
        if let Some(ref v) = self.diagram_object {
            block.fields.insert("DiagramObjectPoint.DiagramObject".into(), crate::base::FieldValue::Resource(v.mrid.clone()));
        }
        if let Some(ref v) = self.diagram_object_glue_point {
            block.fields.insert("DiagramObjectPoint.DiagramObjectGluePoint".into(), crate::base::FieldValue::Resource(v.mrid.clone()));
        }
        if let Some(v) = self.sequence_number {
            block.fields.insert("DiagramObjectPoint.sequenceNumber".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.x_position {
            block.fields.insert("DiagramObjectPoint.xPosition".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.y_position {
            block.fields.insert("DiagramObjectPoint.yPosition".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.z_position {
            block.fields.insert("DiagramObjectPoint.zPosition".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        block
    }
}

impl DiagramObjectPoint {
    pub fn from_block(b: &crate::base::RdfBlock) -> Self {
        let mut obj = Self::default();
        obj.id.clone_from(&b.mrid);
        for (key, val) in &b.fields {
            match key.as_str() {
                "DiagramObjectPoint.DiagramObject" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.diagram_object = Some(crate::base::MridRef { mrid: sv.clone() });
                    }
                }
                "DiagramObjectPoint.DiagramObjectGluePoint" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.diagram_object_glue_point = Some(crate::base::MridRef { mrid: sv.clone() });
                    }
                }
                "DiagramObjectPoint.sequenceNumber" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.sequence_number = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.sequence_number = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "DiagramObjectPoint.xPosition" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.x_position = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.x_position = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "DiagramObjectPoint.yPosition" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.y_position = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.y_position = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "DiagramObjectPoint.zPosition" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.z_position = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.z_position = Some(v); } }
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
