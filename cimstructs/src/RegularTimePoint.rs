/// Time point for a schedule where the time between the consecutive points is constant.
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct RegularTimePoint {
    pub id: String,
    /// Regular interval schedule containing this time point.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval_schedule: Option<super::base::MridRef>,
    /// The position of the regular time point in the sequence. Note that time points don't have to be sequential, i.e. time points may be omitted. The actual time for a RegularTimePoint is computed by multiplying the associated regular interval schedule's time step with the regular time point sequence number and adding the associated schedules start time. To specify values for the start time, use sequence number 0. The sequence number cannot be negative.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sequence_number: Option<i64>,
    /// The first value at the time. The meaning of the value is defined by the derived type of the associated schedule.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value1: Option<f64>,
    /// The second value at the time. The meaning of the value is defined by the derived type of the associated schedule.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value2: Option<f64>,
}
impl crate::base::CimElement for RegularTimePoint {
    fn mrid(&self) -> &str { &self.id }
    fn type_name(&self) -> &'static str { "RegularTimePoint" }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn to_json_value(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or(serde_json::Value::Null)
    }
    fn to_block(&self) -> crate::base::RdfBlock {
        let mut block = crate::base::RdfBlock {
            type_name: "RegularTimePoint".to_string(),
            mrid: self.id.clone(),
            fields: std::collections::HashMap::new(),
            duplicate_fields: std::collections::HashSet::new(),
        };
        if let Some(ref v) = self.interval_schedule {
            block.fields.insert("RegularTimePoint.IntervalSchedule".into(), crate::base::FieldValue::Resource(v.mrid.clone()));
        }
        if let Some(v) = self.sequence_number {
            block.fields.insert("RegularTimePoint.sequenceNumber".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.value1 {
            block.fields.insert("RegularTimePoint.value1".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.value2 {
            block.fields.insert("RegularTimePoint.value2".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        block
    }
}

impl RegularTimePoint {
    pub fn from_block(b: &crate::base::RdfBlock) -> Self {
        let mut obj = Self::default();
        obj.id.clone_from(&b.mrid);
        for (key, val) in &b.fields {
            match key.as_str() {
                "RegularTimePoint.IntervalSchedule" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.interval_schedule = Some(crate::base::MridRef { mrid: sv.clone() });
                    }
                }
                "RegularTimePoint.sequenceNumber" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.sequence_number = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.sequence_number = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "RegularTimePoint.value1" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.value1 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.value1 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "RegularTimePoint.value2" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.value2 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.value2 = Some(v); } }
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
