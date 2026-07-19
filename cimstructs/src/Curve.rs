/// A multi-purpose curve or functional relationship between an independent variable (X-axis) and dependent (Y-axis) variables.
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct Curve {
    #[serde(flatten)]
    pub base: super::IdentifiedObject,
    /// The style or shape of the curve.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub curve_style: Option<super::base::UriRef>,
    /// The X-axis units of measure.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x_unit: Option<super::base::UriRef>,
    /// The Y1-axis units of measure.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub y1unit: Option<super::base::UriRef>,
    /// The Y2-axis units of measure.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub y2unit: Option<super::base::UriRef>,
}
impl crate::base::CimElement for Curve {
    fn mrid(&self) -> &str { &self.base.id }
    fn type_name(&self) -> &'static str { "Curve" }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn to_json_value(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or(serde_json::Value::Null)
    }
    fn to_block(&self) -> crate::base::RdfBlock {
        let mut block = self.base.to_block();
        block.type_name = "Curve".to_string();
        if let Some(ref v) = self.curve_style {
            block.fields.insert("Curve.curveStyle".into(), crate::base::FieldValue::Resource(v.uri.clone()));
        }
        if let Some(ref v) = self.x_unit {
            block.fields.insert("Curve.xUnit".into(), crate::base::FieldValue::Resource(v.uri.clone()));
        }
        if let Some(ref v) = self.y1unit {
            block.fields.insert("Curve.y1Unit".into(), crate::base::FieldValue::Resource(v.uri.clone()));
        }
        if let Some(ref v) = self.y2unit {
            block.fields.insert("Curve.y2Unit".into(), crate::base::FieldValue::Resource(v.uri.clone()));
        }
        block
    }
}

impl Curve {
    pub fn from_block(b: &crate::base::RdfBlock) -> Self {
        let mut obj = Self::default();
        obj.base.id.clone_from(&b.mrid);
        for (key, val) in &b.fields {
            match key.as_str() {
                "Curve.curveStyle" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.curve_style = Some(crate::base::UriRef { uri: sv.clone() });
                    }
                }
                "Curve.xUnit" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.x_unit = Some(crate::base::UriRef { uri: sv.clone() });
                    }
                }
                "Curve.y1Unit" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.y1unit = Some(crate::base::UriRef { uri: sv.clone() });
                    }
                }
                "Curve.y2Unit" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.y2unit = Some(crate::base::UriRef { uri: sv.clone() });
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
