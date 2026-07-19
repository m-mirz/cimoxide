/// Set of spatial coordinates that determine a point, defined in the coordinate system specified in 'Location.CoordinateSystem'. Use a single position point instance to describe a point-oriented location. Use a sequence of position points to describe a line-oriented object (physical location of non-point oriented objects like cables or lines), or area of an object (like a substation or a geographical zone - in this case, have first and last position point with the same values).
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct PositionPoint {
    pub id: String,
    /// Location described by this position point.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<super::base::MridRef>,
    /// Zero-relative sequence number of this point within a series of points.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sequence_number: Option<i64>,
    /// X axis position.
    pub x_position: String,
    /// Y axis position.
    pub y_position: String,
    /// (if applicable) Z axis position.
    pub z_position: String,
}
impl crate::base::CimElement for PositionPoint {
    fn mrid(&self) -> &str { &self.id }
    fn type_name(&self) -> &'static str { "PositionPoint" }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn to_json_value(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or(serde_json::Value::Null)
    }
    fn to_block(&self) -> crate::base::RdfBlock {
        let mut block = crate::base::RdfBlock {
            type_name: "PositionPoint".to_string(),
            mrid: self.id.clone(),
            fields: std::collections::HashMap::new(),
            duplicate_fields: std::collections::HashSet::new(),
        };
        if let Some(ref v) = self.location {
            block.fields.insert("PositionPoint.Location".into(), crate::base::FieldValue::Resource(v.mrid.clone()));
        }
        if let Some(v) = self.sequence_number {
            block.fields.insert("PositionPoint.sequenceNumber".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if !self.x_position.is_empty() {
            block.fields.insert("PositionPoint.xPosition".into(), crate::base::FieldValue::Text(self.x_position.clone()));
        }
        if !self.y_position.is_empty() {
            block.fields.insert("PositionPoint.yPosition".into(), crate::base::FieldValue::Text(self.y_position.clone()));
        }
        if !self.z_position.is_empty() {
            block.fields.insert("PositionPoint.zPosition".into(), crate::base::FieldValue::Text(self.z_position.clone()));
        }
        block
    }
}

impl PositionPoint {
    pub fn from_block(b: &crate::base::RdfBlock) -> Self {
        let mut obj = Self::default();
        obj.id.clone_from(&b.mrid);
        for (key, val) in &b.fields {
            match key.as_str() {
                "PositionPoint.Location" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.location = Some(crate::base::MridRef { mrid: sv.clone() });
                    }
                }
                "PositionPoint.sequenceNumber" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.sequence_number = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.sequence_number = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "PositionPoint.xPosition" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.x_position = sv.clone(); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.x_position = sv.clone(); }
                        }
                        _ => {}
                    }
                }
                "PositionPoint.yPosition" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.y_position = sv.clone(); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.y_position = sv.clone(); }
                        }
                        _ => {}
                    }
                }
                "PositionPoint.zPosition" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.z_position = sv.clone(); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.z_position = sv.clone(); }
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
