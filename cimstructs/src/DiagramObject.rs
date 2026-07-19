/// An object that defines one or more points in a given space. This object can be associated with anything that specializes IdentifiedObject. For single line diagrams such objects typically include such items as analog values, breakers, disconnectors, power transformers, and transmission lines.
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct DiagramObject {
    #[serde(flatten)]
    pub base: super::IdentifiedObject,
    /// A diagram object is part of a diagram.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub diagram: Option<super::base::MridRef>,
    /// A diagram object has a style associated that provides a reference for the style used in the originating system.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub diagram_object_style: Option<super::base::MridRef>,
    /// The domain object to which this diagram object is associated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identified_object_: Option<super::base::MridRef>,
    /// The drawing order of this element. The higher the number, the later the element is drawn in sequence. This is used to ensure that elements that overlap are rendered in the correct order.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub drawing_order: Option<i64>,
    /// Defines whether or not the diagram objects points define the boundaries of a polygon or the routing of a polyline. If this value is true then a receiving application should consider the first and last points to be connected.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_polygon: Option<bool>,
    /// The offset in the X direction. This is used for defining the offset from centre for rendering an icon (the default is that a single point specifies the centre of the icon). The offset is in per-unit with 0 indicating there is no offset from the horizontal centre of the icon. -0.5 indicates it is offset by 50% to the left and 0.5 indicates an offset of 50% to the right.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset_x: Option<f64>,
    /// The offset in the Y direction. This is used for defining the offset from centre for rendering an icon (the default is that a single point specifies the centre of the icon). The offset is in per-unit with 0 indicating there is no offset from the vertical centre of the icon. The offset direction is dependent on the orientation of the diagram, with -0.5 and 0.5 indicating an offset of +/- 50% on the vertical axis.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset_y: Option<f64>,
    /// Sets the angle of rotation of the diagram object. Zero degrees is pointing to the top of the diagram. Rotation is clockwise. DiagramObject.rotation=0 has the following meaning: The connection point of an element which has one terminal is pointing to the top side of the diagram. The connection point 'From side' of an element which has more than one terminal is pointing to the top side of the diagram. DiagramObject.rotation=90 has the following meaning: The connection point of an element which has one terminal is pointing to the right hand side of the diagram. The connection point 'From side' of an element which has more than one terminal is pointing to the right hand side of the diagram.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rotation: Option<f64>,
}
impl crate::base::CimElement for DiagramObject {
    fn mrid(&self) -> &str { &self.base.id }
    fn type_name(&self) -> &'static str { "DiagramObject" }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn to_json_value(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or(serde_json::Value::Null)
    }
    fn to_block(&self) -> crate::base::RdfBlock {
        let mut block = self.base.to_block();
        block.type_name = "DiagramObject".to_string();
        if let Some(ref v) = self.diagram {
            block.fields.insert("DiagramObject.Diagram".into(), crate::base::FieldValue::Resource(v.mrid.clone()));
        }
        if let Some(ref v) = self.diagram_object_style {
            block.fields.insert("DiagramObject.DiagramObjectStyle".into(), crate::base::FieldValue::Resource(v.mrid.clone()));
        }
        if let Some(ref v) = self.identified_object_ {
            block.fields.insert("DiagramObject.IdentifiedObject".into(), crate::base::FieldValue::Resource(v.mrid.clone()));
        }
        if let Some(v) = self.drawing_order {
            block.fields.insert("DiagramObject.drawingOrder".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.is_polygon {
            block.fields.insert("DiagramObject.isPolygon".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.offset_x {
            block.fields.insert("DiagramObject.offsetX".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.offset_y {
            block.fields.insert("DiagramObject.offsetY".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.rotation {
            block.fields.insert("DiagramObject.rotation".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        block
    }
}

impl DiagramObject {
    pub fn from_block(b: &crate::base::RdfBlock) -> Self {
        let mut obj = Self::default();
        obj.base.id.clone_from(&b.mrid);
        for (key, val) in &b.fields {
            match key.as_str() {
                "DiagramObject.Diagram" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.diagram = Some(crate::base::MridRef { mrid: sv.clone() });
                    }
                }
                "DiagramObject.DiagramObjectStyle" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.diagram_object_style = Some(crate::base::MridRef { mrid: sv.clone() });
                    }
                }
                "DiagramObject.IdentifiedObject" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.identified_object_ = Some(crate::base::MridRef { mrid: sv.clone() });
                    }
                }
                "DiagramObject.drawingOrder" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.drawing_order = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.drawing_order = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "DiagramObject.isPolygon" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.is_polygon = Some(sv.trim() == "true"); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.is_polygon = Some(sv.trim() == "true"); }
                        }
                        _ => {}
                    }
                }
                "DiagramObject.offsetX" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.offset_x = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.offset_x = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "DiagramObject.offsetY" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.offset_y = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.offset_y = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "DiagramObject.rotation" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.rotation = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.rotation = Some(v); } }
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
