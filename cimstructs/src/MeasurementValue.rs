/// The current state for a measurement. A state value is an instance of a measurement from a specific source. Measurements can be associated with many state values, each representing a different source for the measurement.
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct MeasurementValue {
    #[serde(flatten)]
    pub base: super::IOPoint,
    /// A reference to the type of source that updates the MeasurementValue, e.g. SCADA, CCLink, manual, etc. User conventions for the names of sources are contained in the introduction to IEC 61970-301.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub measurement_value_source: Option<super::base::MridRef>,
    /// The limit, expressed as a percentage of the sensor maximum, that errors will not exceed when the sensor is used under reference conditions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sensor_accuracy: Option<f64>,
    /// The time when the value was last updated.
    pub time_stamp: String,
}
impl crate::base::CimElement for MeasurementValue {
    fn mrid(&self) -> &str { &self.base.base.id }
    fn type_name(&self) -> &'static str { "MeasurementValue" }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn to_json_value(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or(serde_json::Value::Null)
    }
    fn to_block(&self) -> crate::base::RdfBlock {
        let mut block = self.base.to_block();
        block.type_name = "MeasurementValue".to_string();
        if let Some(ref v) = self.measurement_value_source {
            block.fields.insert("MeasurementValue.MeasurementValueSource".into(), crate::base::FieldValue::Resource(v.mrid.clone()));
        }
        if let Some(v) = self.sensor_accuracy {
            block.fields.insert("MeasurementValue.sensorAccuracy".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if !self.time_stamp.is_empty() {
            block.fields.insert("MeasurementValue.timeStamp".into(), crate::base::FieldValue::Text(self.time_stamp.clone()));
        }
        block
    }
}

impl MeasurementValue {
    pub fn from_block(b: &crate::base::RdfBlock) -> Self {
        let mut obj = Self::default();
        obj.base.base.id.clone_from(&b.mrid);
        for (key, val) in &b.fields {
            match key.as_str() {
                "MeasurementValue.MeasurementValueSource" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.measurement_value_source = Some(crate::base::MridRef { mrid: sv.clone() });
                    }
                }
                "MeasurementValue.sensorAccuracy" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.sensor_accuracy = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.sensor_accuracy = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "MeasurementValue.timeStamp" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.time_stamp = sv.clone(); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.time_stamp = sv.clone(); }
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
