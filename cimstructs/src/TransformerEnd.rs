/// A conducting connection point of a power transformer. It corresponds to a physical transformer winding terminal. In earlier CIM versions, the TransformerWinding class served a similar purpose, but this class is more flexible because it associates to terminal but is not a specialization of ConductingEquipment.
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct TransformerEnd {
    #[serde(flatten)]
    pub base: super::IdentifiedObject,
    /// Base voltage of the transformer end. This is essential for PU calculation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_voltage: Option<super::base::MridRef>,
    /// Terminal of the power transformer to which this transformer end belongs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub terminal: Option<super::base::MridRef>,
    /// Number for this transformer end, corresponding to the end's order in the power transformer vector group or phase angle clock number. Highest voltage winding should be 1. Each end within a power transformer should have a unique subsequent end number. Note the transformer end number need not match the terminal sequence number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_number: Option<i64>,
    /// (for Yn and Zn connections) True if the neutral is solidly grounded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grounded: Option<bool>,
    /// (for Yn and Zn connections) Resistance part of neutral impedance where 'grounded' is true.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rground: Option<f64>,
    /// (for Yn and Zn connections) Reactive part of neutral impedance where 'grounded' is true.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub xground: Option<f64>,
}
impl crate::base::CimElement for TransformerEnd {
    fn mrid(&self) -> &str { &self.base.id }
    fn type_name(&self) -> &'static str { "TransformerEnd" }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn to_json_value(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or(serde_json::Value::Null)
    }
    fn to_block(&self) -> crate::base::RdfBlock {
        let mut block = self.base.to_block();
        block.type_name = "TransformerEnd".to_string();
        if let Some(ref v) = self.base_voltage {
            block.fields.insert("TransformerEnd.BaseVoltage".into(), crate::base::FieldValue::Resource(v.mrid.clone()));
        }
        if let Some(ref v) = self.terminal {
            block.fields.insert("TransformerEnd.Terminal".into(), crate::base::FieldValue::Resource(v.mrid.clone()));
        }
        if let Some(v) = self.end_number {
            block.fields.insert("TransformerEnd.endNumber".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.grounded {
            block.fields.insert("TransformerEnd.grounded".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.rground {
            block.fields.insert("TransformerEnd.rground".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.xground {
            block.fields.insert("TransformerEnd.xground".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        block
    }
}

impl TransformerEnd {
    pub fn from_block(b: &crate::base::RdfBlock) -> Self {
        let mut obj = Self::default();
        obj.base.id.clone_from(&b.mrid);
        for (key, val) in &b.fields {
            match key.as_str() {
                "TransformerEnd.BaseVoltage" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.base_voltage = Some(crate::base::MridRef { mrid: sv.clone() });
                    }
                }
                "TransformerEnd.Terminal" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.terminal = Some(crate::base::MridRef { mrid: sv.clone() });
                    }
                }
                "TransformerEnd.endNumber" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.end_number = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.end_number = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "TransformerEnd.grounded" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.grounded = Some(sv.trim() == "true"); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.grounded = Some(sv.trim() == "true"); }
                        }
                        _ => {}
                    }
                }
                "TransformerEnd.rground" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.rground = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.rground = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "TransformerEnd.xground" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.xground = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.xground = Some(v); } }
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
