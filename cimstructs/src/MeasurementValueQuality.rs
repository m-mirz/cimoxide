/// Measurement quality flags. Bits 0-10 are defined for substation automation in IEC 61850-7-3. Bits 11-15 are reserved for future expansion by that document. Bits 16-31 are reserved for EMS applications.
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct MeasurementValueQuality {
    #[serde(flatten)]
    pub base: super::Quality61850,
    /// A MeasurementValue has a MeasurementValueQuality associated with it.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub measurement_value: Option<super::base::MridRef>,
}
impl crate::base::CimElement for MeasurementValueQuality {
    fn mrid(&self) -> &str { &self.base.id }
    fn type_name(&self) -> &'static str { "MeasurementValueQuality" }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn to_json_value(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or(serde_json::Value::Null)
    }
    fn to_block(&self) -> crate::base::RdfBlock {
        let mut block = self.base.to_block();
        block.type_name = "MeasurementValueQuality".to_string();
        if let Some(ref v) = self.measurement_value {
            block.fields.insert("MeasurementValueQuality.MeasurementValue".into(), crate::base::FieldValue::Resource(v.mrid.clone()));
        }
        block
    }
}

impl MeasurementValueQuality {
    pub fn from_block(b: &crate::base::RdfBlock) -> Self {
        let mut obj = Self::default();
        obj.base.id.clone_from(&b.mrid);
        for (key, val) in &b.fields {
            match key.as_str() {
                "MeasurementValueQuality.MeasurementValue" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.measurement_value = Some(crate::base::MridRef { mrid: sv.clone() });
                    }
                }
                "Quality61850.badReference" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.base.bad_reference = Some(sv.trim() == "true"); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.base.bad_reference = Some(sv.trim() == "true"); }
                        }
                        _ => {}
                    }
                }
                "Quality61850.estimatorReplaced" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.base.estimator_replaced = Some(sv.trim() == "true"); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.base.estimator_replaced = Some(sv.trim() == "true"); }
                        }
                        _ => {}
                    }
                }
                "Quality61850.failure" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.base.failure = Some(sv.trim() == "true"); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.base.failure = Some(sv.trim() == "true"); }
                        }
                        _ => {}
                    }
                }
                "Quality61850.oldData" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.base.old_data = Some(sv.trim() == "true"); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.base.old_data = Some(sv.trim() == "true"); }
                        }
                        _ => {}
                    }
                }
                "Quality61850.operatorBlocked" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.base.operator_blocked = Some(sv.trim() == "true"); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.base.operator_blocked = Some(sv.trim() == "true"); }
                        }
                        _ => {}
                    }
                }
                "Quality61850.oscillatory" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.base.oscillatory = Some(sv.trim() == "true"); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.base.oscillatory = Some(sv.trim() == "true"); }
                        }
                        _ => {}
                    }
                }
                "Quality61850.outOfRange" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.base.out_of_range = Some(sv.trim() == "true"); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.base.out_of_range = Some(sv.trim() == "true"); }
                        }
                        _ => {}
                    }
                }
                "Quality61850.overFlow" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.base.over_flow = Some(sv.trim() == "true"); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.base.over_flow = Some(sv.trim() == "true"); }
                        }
                        _ => {}
                    }
                }
                "Quality61850.source" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.base.source = Some(crate::base::UriRef { uri: sv.clone() });
                    }
                }
                "Quality61850.suspect" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.base.suspect = Some(sv.trim() == "true"); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.base.suspect = Some(sv.trim() == "true"); }
                        }
                        _ => {}
                    }
                }
                "Quality61850.test" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.base.test = Some(sv.trim() == "true"); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.base.test = Some(sv.trim() == "true"); }
                        }
                        _ => {}
                    }
                }
                "Quality61850.validity" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.base.validity = Some(crate::base::UriRef { uri: sv.clone() });
                    }
                }
                _ => {}
            }
        }
        obj
    }
}
