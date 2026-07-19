/// Describes each tap step in the tabular curve.
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct TapChangerTablePoint {
    pub id: String,
    /// The magnetizing branch susceptance deviation as a percentage of nominal value. The actual susceptance is calculated as follows: calculated magnetizing susceptance = b(nominal) * (1 + b(from this class)/100). The b(nominal) is defined as the static magnetizing susceptance on the associated power transformer end or ends. This model assumes the star impedance (pi model) form.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub b: Option<f64>,
    /// The magnetizing branch conductance deviation as a percentage of nominal value. The actual conductance is calculated as follows: calculated magnetizing conductance = g(nominal) * (1 + g(from this class)/100). The g(nominal) is defined as the static magnetizing conductance on the associated power transformer end or ends. This model assumes the star impedance (pi model) form.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub g: Option<f64>,
    /// The resistance deviation as a percentage of nominal value. The actual reactance is calculated as follows: calculated resistance = r(nominal) * (1 + r(from this class)/100). The r(nominal) is defined as the static resistance on the associated power transformer end or ends. This model assumes the star impedance (pi model) form.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r: Option<f64>,
    /// The voltage at the tap step divided by rated voltage of the transformer end having the tap changer. Hence this is a value close to one. For example, if the ratio at step 1 is 1.01, and the rated voltage of the transformer end is 110kV, then the voltage obtained by setting the tap changer to step 1 to is 111.1kV.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ratio: Option<f64>,
    /// The tap step.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub step: Option<i64>,
    /// The series reactance deviation as a percentage of nominal value. The actual reactance is calculated as follows: calculated reactance = x(nominal) * (1 + x(from this class)/100). The x(nominal) is defined as the static series reactance on the associated power transformer end or ends. This model assumes the star impedance (pi model) form.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x: Option<f64>,
}
impl crate::base::CimElement for TapChangerTablePoint {
    fn mrid(&self) -> &str { &self.id }
    fn type_name(&self) -> &'static str { "TapChangerTablePoint" }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn to_json_value(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or(serde_json::Value::Null)
    }
    fn to_block(&self) -> crate::base::RdfBlock {
        let mut block = crate::base::RdfBlock {
            type_name: "TapChangerTablePoint".to_string(),
            mrid: self.id.clone(),
            fields: std::collections::HashMap::new(),
            duplicate_fields: std::collections::HashSet::new(),
        };
        if let Some(v) = self.b {
            block.fields.insert("TapChangerTablePoint.b".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.g {
            block.fields.insert("TapChangerTablePoint.g".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.r {
            block.fields.insert("TapChangerTablePoint.r".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.ratio {
            block.fields.insert("TapChangerTablePoint.ratio".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.step {
            block.fields.insert("TapChangerTablePoint.step".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.x {
            block.fields.insert("TapChangerTablePoint.x".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        block
    }
}

impl TapChangerTablePoint {
    pub fn from_block(b: &crate::base::RdfBlock) -> Self {
        let mut obj = Self::default();
        obj.id.clone_from(&b.mrid);
        for (key, val) in &b.fields {
            match key.as_str() {
                "TapChangerTablePoint.b" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.b = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.b = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "TapChangerTablePoint.g" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.g = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.g = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "TapChangerTablePoint.r" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.r = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.r = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "TapChangerTablePoint.ratio" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.ratio = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.ratio = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "TapChangerTablePoint.step" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.step = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.step = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "TapChangerTablePoint.x" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.x = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.x = Some(v); } }
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
