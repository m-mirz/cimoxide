/// Multi-purpose data points for defining a curve. The use of this generic class is discouraged if a more specific class can be used to specify the X and Y axis values along with their specific data types.
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct CurveData {
    pub id: String,
    /// The curve of this curve data point.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub curve: Option<super::base::MridRef>,
    /// The data value of the X-axis variable, depending on the X-axis units.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub xvalue: Option<f64>,
    /// The data value of the first Y-axis variable, depending on the Y-axis units.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub y1value: Option<f64>,
    /// The data value of the second Y-axis variable (if present), depending on the Y-axis units.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub y2value: Option<f64>,
}
impl crate::base::CimElement for CurveData {
    fn mrid(&self) -> &str { &self.id }
    fn type_name(&self) -> &'static str { "CurveData" }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn to_json_value(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or(serde_json::Value::Null)
    }
    fn to_block(&self) -> crate::base::RdfBlock {
        let mut block = crate::base::RdfBlock {
            type_name: "CurveData".to_string(),
            mrid: self.id.clone(),
            fields: std::collections::HashMap::new(),
            duplicate_fields: std::collections::HashSet::new(),
        };
        if let Some(ref v) = self.curve {
            block.fields.insert("CurveData.Curve".into(), crate::base::FieldValue::Resource(v.mrid.clone()));
        }
        if let Some(v) = self.xvalue {
            block.fields.insert("CurveData.xvalue".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.y1value {
            block.fields.insert("CurveData.y1value".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.y2value {
            block.fields.insert("CurveData.y2value".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        block
    }
}

impl CurveData {
    pub fn from_block(b: &crate::base::RdfBlock) -> Self {
        let mut obj = Self::default();
        obj.id.clone_from(&b.mrid);
        for (key, val) in &b.fields {
            match key.as_str() {
                "CurveData.Curve" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.curve = Some(crate::base::MridRef { mrid: sv.clone() });
                    }
                }
                "CurveData.xvalue" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.xvalue = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.xvalue = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "CurveData.y1value" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.y1value = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.y1value = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "CurveData.y2value" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.y2value = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.y2value = Some(v); } }
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
