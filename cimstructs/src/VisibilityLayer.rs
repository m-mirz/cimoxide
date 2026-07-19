/// Layers are typically used for grouping diagram objects according to themes and scales. Themes are used to display or hide certain information (e.g., lakes, borders), while scales are used for hiding or displaying information depending on the current zoom level (hide text when it is too small to be read, or when it exceeds the screen size). This is also called de-cluttering. CIM based graphics exchange supports an m:n relationship between diagram objects and layers. The importing system shall convert an m:n case into an appropriate 1:n representation if the importing system does not support m:n.
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct VisibilityLayer {
    #[serde(flatten)]
    pub base: super::IdentifiedObject,
    /// A visibility layer can contain one or more diagram objects.
    pub visible_objects: Vec<super::base::MridRef>,
    /// The drawing order for this layer. The higher the number, the later the layer and the objects within it are rendered.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub drawing_order: Option<i64>,
}
impl crate::base::CimElement for VisibilityLayer {
    fn mrid(&self) -> &str { &self.base.id }
    fn type_name(&self) -> &'static str { "VisibilityLayer" }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn to_json_value(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or(serde_json::Value::Null)
    }
    fn to_block(&self) -> crate::base::RdfBlock {
        let mut block = self.base.to_block();
        block.type_name = "VisibilityLayer".to_string();
        if !self.visible_objects.is_empty() {
            block.fields.insert("VisibilityLayer.VisibleObjects".into(), crate::base::FieldValue::ResourceList(self.visible_objects.iter().map(|r| r.mrid.clone()).collect()));
        }
        if let Some(v) = self.drawing_order {
            block.fields.insert("VisibilityLayer.drawingOrder".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        block
    }
}

impl VisibilityLayer {
    pub fn from_block(b: &crate::base::RdfBlock) -> Self {
        let mut obj = Self::default();
        obj.base.id.clone_from(&b.mrid);
        for (key, val) in &b.fields {
            match key.as_str() {
                "VisibilityLayer.VisibleObjects" => {
                    match val {
                        crate::base::FieldValue::Resource(sv) => obj.visible_objects.push(crate::base::MridRef { mrid: sv.clone() }),
                        crate::base::FieldValue::ResourceList(svs) => {
                            for sv in svs { obj.visible_objects.push(crate::base::MridRef { mrid: sv.clone() }); }
                        }
                        _ => {}
                    }
                }
                "VisibilityLayer.drawingOrder" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.drawing_order = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.drawing_order = Some(v); } }
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
