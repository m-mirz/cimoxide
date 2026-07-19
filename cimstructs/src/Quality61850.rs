/// Quality flags in this class are as defined in IEC 61850, except for estimatorReplaced, which has been included in this class for convenience.
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct Quality61850 {
    pub id: String,
    /// Measurement value may be incorrect due to a reference being out of calibration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bad_reference: Option<bool>,
    /// Value has been replaced by State Estimator. estimatorReplaced is not an IEC61850 quality bit but has been put in this class for convenience.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimator_replaced: Option<bool>,
    /// This identifier indicates that a supervision function has detected an internal or external failure, e.g. communication failure.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure: Option<bool>,
    /// Measurement value is old and possibly invalid, as it has not been successfully updated during a specified time interval.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub old_data: Option<bool>,
    /// Measurement value is blocked and hence unavailable for transmission.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operator_blocked: Option<bool>,
    /// To prevent some overload of the communication it is sensible to detect and suppress oscillating (fast changing) binary inputs. If a signal changes in a defined time twice in the same direction (from 0 to 1 or from 1 to 0) then oscillation is detected and the detail quality identifier 'oscillatory' is set. If it is detected a configured numbers of transient changes could be passed by. In this time the validity status 'questionable' is set. If after this defined numbers of changes the signal is still in the oscillating state the value shall be set either to the opposite state of the previous stable value or to a defined default value. In this case the validity status 'questionable' is reset and 'invalid' is set as long as the signal is oscillating. If it is configured such that no transient changes should be passed by then the validity status 'invalid' is set immediately in addition to the detail quality identifier 'oscillatory' (used for status information only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oscillatory: Option<bool>,
    /// Measurement value is beyond a predefined range of value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub out_of_range: Option<bool>,
    /// Measurement value is beyond the capability of being represented properly. For example, a counter value overflows from maximum count back to a value of zero.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub over_flow: Option<bool>,
    /// Source gives information related to the origin of a value. The value may be acquired from the process, defaulted or substituted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<super::base::UriRef>,
    /// A correlation function has detected that the value is not consistent with other values. Typically set by a network State Estimator.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suspect: Option<bool>,
    /// Measurement value is transmitted for test purposes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub test: Option<bool>,
    /// Validity of the measurement value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validity: Option<super::base::UriRef>,
}
impl crate::base::CimElement for Quality61850 {
    fn mrid(&self) -> &str { &self.id }
    fn type_name(&self) -> &'static str { "Quality61850" }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn to_json_value(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or(serde_json::Value::Null)
    }
    fn to_block(&self) -> crate::base::RdfBlock {
        let mut block = crate::base::RdfBlock {
            type_name: "Quality61850".to_string(),
            mrid: self.id.clone(),
            fields: std::collections::HashMap::new(),
            duplicate_fields: std::collections::HashSet::new(),
        };
        if let Some(v) = self.bad_reference {
            block.fields.insert("Quality61850.badReference".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.estimator_replaced {
            block.fields.insert("Quality61850.estimatorReplaced".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.failure {
            block.fields.insert("Quality61850.failure".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.old_data {
            block.fields.insert("Quality61850.oldData".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.operator_blocked {
            block.fields.insert("Quality61850.operatorBlocked".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.oscillatory {
            block.fields.insert("Quality61850.oscillatory".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.out_of_range {
            block.fields.insert("Quality61850.outOfRange".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.over_flow {
            block.fields.insert("Quality61850.overFlow".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(ref v) = self.source {
            block.fields.insert("Quality61850.source".into(), crate::base::FieldValue::Resource(v.uri.clone()));
        }
        if let Some(v) = self.suspect {
            block.fields.insert("Quality61850.suspect".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.test {
            block.fields.insert("Quality61850.test".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(ref v) = self.validity {
            block.fields.insert("Quality61850.validity".into(), crate::base::FieldValue::Resource(v.uri.clone()));
        }
        block
    }
}

impl Quality61850 {
    pub fn from_block(b: &crate::base::RdfBlock) -> Self {
        let mut obj = Self::default();
        obj.id.clone_from(&b.mrid);
        for (key, val) in &b.fields {
            match key.as_str() {
                "Quality61850.badReference" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.bad_reference = Some(sv.trim() == "true"); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.bad_reference = Some(sv.trim() == "true"); }
                        }
                        _ => {}
                    }
                }
                "Quality61850.estimatorReplaced" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.estimator_replaced = Some(sv.trim() == "true"); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.estimator_replaced = Some(sv.trim() == "true"); }
                        }
                        _ => {}
                    }
                }
                "Quality61850.failure" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.failure = Some(sv.trim() == "true"); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.failure = Some(sv.trim() == "true"); }
                        }
                        _ => {}
                    }
                }
                "Quality61850.oldData" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.old_data = Some(sv.trim() == "true"); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.old_data = Some(sv.trim() == "true"); }
                        }
                        _ => {}
                    }
                }
                "Quality61850.operatorBlocked" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.operator_blocked = Some(sv.trim() == "true"); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.operator_blocked = Some(sv.trim() == "true"); }
                        }
                        _ => {}
                    }
                }
                "Quality61850.oscillatory" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.oscillatory = Some(sv.trim() == "true"); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.oscillatory = Some(sv.trim() == "true"); }
                        }
                        _ => {}
                    }
                }
                "Quality61850.outOfRange" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.out_of_range = Some(sv.trim() == "true"); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.out_of_range = Some(sv.trim() == "true"); }
                        }
                        _ => {}
                    }
                }
                "Quality61850.overFlow" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.over_flow = Some(sv.trim() == "true"); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.over_flow = Some(sv.trim() == "true"); }
                        }
                        _ => {}
                    }
                }
                "Quality61850.source" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.source = Some(crate::base::UriRef { uri: sv.clone() });
                    }
                }
                "Quality61850.suspect" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.suspect = Some(sv.trim() == "true"); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.suspect = Some(sv.trim() == "true"); }
                        }
                        _ => {}
                    }
                }
                "Quality61850.test" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.test = Some(sv.trim() == "true"); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.test = Some(sv.trim() == "true"); }
                        }
                        _ => {}
                    }
                }
                "Quality61850.validity" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.validity = Some(crate::base::UriRef { uri: sv.clone() });
                    }
                }
                _ => {}
            }
        }
        obj
    }
}
