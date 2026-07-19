/// Describes each tap step in the ratio tap changer tabular curve.
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct RatioTapChangerTablePoint {
    #[serde(flatten)]
    pub base: super::TapChangerTablePoint,
    /// Table of this point.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ratio_tap_changer_table: Option<super::base::MridRef>,
}
impl crate::base::CimElement for RatioTapChangerTablePoint {
    fn mrid(&self) -> &str { &self.base.id }
    fn type_name(&self) -> &'static str { "RatioTapChangerTablePoint" }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn to_json_value(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or(serde_json::Value::Null)
    }
    fn to_block(&self) -> crate::base::RdfBlock {
        let mut block = self.base.to_block();
        block.type_name = "RatioTapChangerTablePoint".to_string();
        if let Some(ref v) = self.ratio_tap_changer_table {
            block.fields.insert("RatioTapChangerTablePoint.RatioTapChangerTable".into(), crate::base::FieldValue::Resource(v.mrid.clone()));
        }
        block
    }
}

impl RatioTapChangerTablePoint {
    pub fn from_block(b: &crate::base::RdfBlock) -> Self {
        let mut obj = Self::default();
        obj.base.id.clone_from(&b.mrid);
        for (key, val) in &b.fields {
            match key.as_str() {
                "RatioTapChangerTablePoint.RatioTapChangerTable" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.ratio_tap_changer_table = Some(crate::base::MridRef { mrid: sv.clone() });
                    }
                }
                "TapChangerTablePoint.b" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.base.b = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.base.b = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "TapChangerTablePoint.g" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.base.g = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.base.g = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "TapChangerTablePoint.r" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.base.r = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.base.r = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "TapChangerTablePoint.ratio" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.base.ratio = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.base.ratio = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "TapChangerTablePoint.step" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.base.step = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.base.step = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "TapChangerTablePoint.x" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.base.x = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.base.x = Some(v); } }
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
