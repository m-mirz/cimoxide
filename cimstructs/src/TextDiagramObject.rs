/// A diagram object for placing free-text or text derived from an associated domain object.
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct TextDiagramObject {
    #[serde(flatten)]
    pub base: super::DiagramObject,
    /// The text that is displayed by this text diagram object.
    pub text: String,
}
impl crate::base::CimElement for TextDiagramObject {
    fn mrid(&self) -> &str { &self.base.base.id }
    fn type_name(&self) -> &'static str { "TextDiagramObject" }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn to_json_value(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or(serde_json::Value::Null)
    }
    fn to_block(&self) -> crate::base::RdfBlock {
        let mut block = self.base.to_block();
        block.type_name = "TextDiagramObject".to_string();
        if !self.text.is_empty() {
            block.fields.insert("TextDiagramObject.text".into(), crate::base::FieldValue::Text(self.text.clone()));
        }
        block
    }
}

impl TextDiagramObject {
    pub fn from_block(b: &crate::base::RdfBlock) -> Self {
        let mut obj = Self::default();
        obj.base.base.id.clone_from(&b.mrid);
        for (key, val) in &b.fields {
            match key.as_str() {
                "TextDiagramObject.text" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.text = sv.clone(); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.text = sv.clone(); }
                        }
                        _ => {}
                    }
                }
                "DiagramObject.Diagram" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.base.diagram = Some(crate::base::MridRef { mrid: sv.clone() });
                    }
                }
                "DiagramObject.DiagramObjectStyle" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.base.diagram_object_style = Some(crate::base::MridRef { mrid: sv.clone() });
                    }
                }
                "DiagramObject.IdentifiedObject" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.base.identified_object_ = Some(crate::base::MridRef { mrid: sv.clone() });
                    }
                }
                "DiagramObject.drawingOrder" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.base.drawing_order = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.base.drawing_order = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "DiagramObject.isPolygon" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.base.is_polygon = Some(sv.trim() == "true"); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.base.is_polygon = Some(sv.trim() == "true"); }
                        }
                        _ => {}
                    }
                }
                "DiagramObject.offsetX" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.base.offset_x = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.base.offset_x = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "DiagramObject.offsetY" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.base.offset_y = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.base.offset_y = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "DiagramObject.rotation" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.base.rotation = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.base.rotation = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "IdentifiedObject.description" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.base.base.description = sv.clone(); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.base.base.description = sv.clone(); }
                        }
                        _ => {}
                    }
                }
                "IdentifiedObject.energyIdentCodeEic" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.base.base.energy_ident_code_eic = sv.clone(); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.base.base.energy_ident_code_eic = sv.clone(); }
                        }
                        _ => {}
                    }
                }
                "IdentifiedObject.mRID" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.base.base.m_rid = sv.clone(); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.base.base.m_rid = sv.clone(); }
                        }
                        _ => {}
                    }
                }
                "IdentifiedObject.name" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.base.base.name = sv.clone(); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.base.base.name = sv.clone(); }
                        }
                        _ => {}
                    }
                }
                "IdentifiedObject.shortName" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.base.base.short_name = sv.clone(); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.base.base.short_name = sv.clone(); }
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
