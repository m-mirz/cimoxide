/// Analog represents an analog Measurement.
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct Analog {
    #[serde(flatten)]
    pub base: super::Measurement,
    /// If true then this measurement is an active power, reactive power or current with the convention that a positive value measured at the Terminal means power is flowing into the related PowerSystemResource.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub positive_flow_in: Option<bool>,
}
impl crate::base::CimElement for Analog {
    fn mrid(&self) -> &str { &self.base.base.id }
    fn type_name(&self) -> &'static str { "Analog" }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn to_json_value(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or(serde_json::Value::Null)
    }
    fn to_block(&self) -> crate::base::RdfBlock {
        let mut block = self.base.to_block();
        block.type_name = "Analog".to_string();
        if let Some(v) = self.positive_flow_in {
            block.fields.insert("Analog.positiveFlowIn".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        block
    }
}

impl Analog {
    pub fn from_block(b: &crate::base::RdfBlock) -> Self {
        let mut obj = Self::default();
        obj.base.base.id.clone_from(&b.mrid);
        for (key, val) in &b.fields {
            match key.as_str() {
                "Analog.positiveFlowIn" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.positive_flow_in = Some(sv.trim() == "true"); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.positive_flow_in = Some(sv.trim() == "true"); }
                        }
                        _ => {}
                    }
                }
                "Measurement.PowerSystemResource" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.base.power_system_resource = Some(crate::base::MridRef { mrid: sv.clone() });
                    }
                }
                "Measurement.Terminal" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.base.terminal = Some(crate::base::MridRef { mrid: sv.clone() });
                    }
                }
                "Measurement.measurementType" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.base.measurement_type = sv.clone(); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.base.measurement_type = sv.clone(); }
                        }
                        _ => {}
                    }
                }
                "Measurement.phases" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.base.phases = Some(crate::base::UriRef { uri: sv.clone() });
                    }
                }
                "Measurement.unitMultiplier" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.base.unit_multiplier = Some(crate::base::UriRef { uri: sv.clone() });
                    }
                }
                "Measurement.unitSymbol" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.base.unit_symbol = Some(crate::base::UriRef { uri: sv.clone() });
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
