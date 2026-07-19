/// AccumulatorValue represents an accumulated (counted) MeasurementValue.
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct AccumulatorValue {
    #[serde(flatten)]
    pub base: super::MeasurementValue,
    /// Measurement to which this value is connected.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accumulator: Option<super::base::MridRef>,
}
impl crate::base::CimElement for AccumulatorValue {
    fn mrid(&self) -> &str { &self.base.base.base.id }
    fn type_name(&self) -> &'static str { "AccumulatorValue" }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn to_json_value(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or(serde_json::Value::Null)
    }
    fn to_block(&self) -> crate::base::RdfBlock {
        let mut block = self.base.to_block();
        block.type_name = "AccumulatorValue".to_string();
        if let Some(ref v) = self.accumulator {
            block.fields.insert("AccumulatorValue.Accumulator".into(), crate::base::FieldValue::Resource(v.mrid.clone()));
        }
        block
    }
}

impl AccumulatorValue {
    pub fn from_block(b: &crate::base::RdfBlock) -> Self {
        let mut obj = Self::default();
        obj.base.base.base.id.clone_from(&b.mrid);
        for (key, val) in &b.fields {
            match key.as_str() {
                "AccumulatorValue.Accumulator" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.accumulator = Some(crate::base::MridRef { mrid: sv.clone() });
                    }
                }
                "MeasurementValue.MeasurementValueSource" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.base.measurement_value_source = Some(crate::base::MridRef { mrid: sv.clone() });
                    }
                }
                "MeasurementValue.sensorAccuracy" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.base.sensor_accuracy = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.base.sensor_accuracy = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "MeasurementValue.timeStamp" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.base.time_stamp = sv.clone(); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.base.time_stamp = sv.clone(); }
                        }
                        _ => {}
                    }
                }
                "IdentifiedObject.description" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.base.base.base.description = sv.clone(); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.base.base.base.description = sv.clone(); }
                        }
                        _ => {}
                    }
                }
                "IdentifiedObject.energyIdentCodeEic" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.base.base.base.energy_ident_code_eic = sv.clone(); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.base.base.base.energy_ident_code_eic = sv.clone(); }
                        }
                        _ => {}
                    }
                }
                "IdentifiedObject.mRID" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.base.base.base.m_rid = sv.clone(); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.base.base.base.m_rid = sv.clone(); }
                        }
                        _ => {}
                    }
                }
                "IdentifiedObject.name" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.base.base.base.name = sv.clone(); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.base.base.base.name = sv.clone(); }
                        }
                        _ => {}
                    }
                }
                "IdentifiedObject.shortName" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.base.base.base.short_name = sv.clone(); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.base.base.base.short_name = sv.clone(); }
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
