/// The diagram being exchanged. The coordinate system is a standard Cartesian coordinate system and the orientation attribute defines the orientation. The initial view related attributes can be used to specify an initial view with the x,y coordinates of the diagonal points.
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct Diagram {
    #[serde(flatten)]
    pub base: super::IdentifiedObject,
    /// A Diagram may have a DiagramStyle.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub diagram_style: Option<super::base::MridRef>,
    /// Coordinate system orientation of the diagram. A positive orientation gives standard “right-hand” orientation, with negative orientation indicating a “left-hand” orientation. For 2D diagrams, a positive orientation will result in X values increasing from left to right and Y values increasing from bottom to top. A negative orientation gives the “left-hand” orientation (favoured by computer graphics displays) with X values increasing from left to right and Y values increasing from top to bottom.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub orientation: Option<super::base::UriRef>,
    /// X coordinate of the first corner of the initial view.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x1initial_view: Option<f64>,
    /// X coordinate of the second corner of the initial view.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x2initial_view: Option<f64>,
    /// Y coordinate of the first corner of the initial view.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub y1initial_view: Option<f64>,
    /// Y coordinate of the second corner of the initial view.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub y2initial_view: Option<f64>,
}
impl crate::base::CimElement for Diagram {
    fn mrid(&self) -> &str { &self.base.id }
    fn type_name(&self) -> &'static str { "Diagram" }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn to_json_value(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or(serde_json::Value::Null)
    }
    fn to_block(&self) -> crate::base::RdfBlock {
        let mut block = self.base.to_block();
        block.type_name = "Diagram".to_string();
        if let Some(ref v) = self.diagram_style {
            block.fields.insert("Diagram.DiagramStyle".into(), crate::base::FieldValue::Resource(v.mrid.clone()));
        }
        if let Some(ref v) = self.orientation {
            block.fields.insert("Diagram.orientation".into(), crate::base::FieldValue::Resource(v.uri.clone()));
        }
        if let Some(v) = self.x1initial_view {
            block.fields.insert("Diagram.x1InitialView".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.x2initial_view {
            block.fields.insert("Diagram.x2InitialView".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.y1initial_view {
            block.fields.insert("Diagram.y1InitialView".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.y2initial_view {
            block.fields.insert("Diagram.y2InitialView".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        block
    }
}

impl Diagram {
    pub fn from_block(b: &crate::base::RdfBlock) -> Self {
        let mut obj = Self::default();
        obj.base.id.clone_from(&b.mrid);
        for (key, val) in &b.fields {
            match key.as_str() {
                "Diagram.DiagramStyle" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.diagram_style = Some(crate::base::MridRef { mrid: sv.clone() });
                    }
                }
                "Diagram.orientation" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.orientation = Some(crate::base::UriRef { uri: sv.clone() });
                    }
                }
                "Diagram.x1InitialView" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.x1initial_view = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.x1initial_view = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "Diagram.x2InitialView" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.x2initial_view = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.x2initial_view = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "Diagram.y1InitialView" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.y1initial_view = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.y1initial_view = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "Diagram.y2InitialView" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.y2initial_view = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.y2initial_view = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "IdentifiedObject.description" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.base.description = sv.clone(); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.base.description = sv.clone(); }
                        }
                        _ => {}
                    }
                }
                "IdentifiedObject.energyIdentCodeEic" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.base.energy_ident_code_eic = sv.clone(); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.base.energy_ident_code_eic = sv.clone(); }
                        }
                        _ => {}
                    }
                }
                "IdentifiedObject.mRID" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.base.m_rid = sv.clone(); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.base.m_rid = sv.clone(); }
                        }
                        _ => {}
                    }
                }
                "IdentifiedObject.name" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.base.name = sv.clone(); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.base.name = sv.clone(); }
                        }
                        _ => {}
                    }
                }
                "IdentifiedObject.shortName" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.base.short_name = sv.clone(); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.base.short_name = sv.clone(); }
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
