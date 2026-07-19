/// A schedule of switch positions. If RegularTimePoint.value1 is 0, the switch is open. If 1, the switch is closed.
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct SwitchSchedule {
    #[serde(flatten)]
    pub base: super::SeasonDayTypeSchedule,
    /// A SwitchSchedule is associated with a Switch.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub switch: Option<super::base::MridRef>,
}
impl crate::base::CimElement for SwitchSchedule {
    fn mrid(&self) -> &str { &self.base.base.base.base.id }
    fn type_name(&self) -> &'static str { "SwitchSchedule" }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn to_json_value(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or(serde_json::Value::Null)
    }
    fn to_block(&self) -> crate::base::RdfBlock {
        let mut block = self.base.to_block();
        block.type_name = "SwitchSchedule".to_string();
        if let Some(ref v) = self.switch {
            block.fields.insert("SwitchSchedule.Switch".into(), crate::base::FieldValue::Resource(v.mrid.clone()));
        }
        block
    }
}

impl SwitchSchedule {
    pub fn from_block(b: &crate::base::RdfBlock) -> Self {
        let mut obj = Self::default();
        obj.base.base.base.base.id.clone_from(&b.mrid);
        for (key, val) in &b.fields {
            match key.as_str() {
                "SwitchSchedule.Switch" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.switch = Some(crate::base::MridRef { mrid: sv.clone() });
                    }
                }
                "SeasonDayTypeSchedule.DayType" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.base.day_type = Some(crate::base::MridRef { mrid: sv.clone() });
                    }
                }
                "SeasonDayTypeSchedule.Season" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.base.season = Some(crate::base::MridRef { mrid: sv.clone() });
                    }
                }
                "RegularIntervalSchedule.endTime" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.base.base.end_time = sv.clone(); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.base.base.end_time = sv.clone(); }
                        }
                        _ => {}
                    }
                }
                "RegularIntervalSchedule.timeStep" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.base.base.time_step = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.base.base.time_step = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "BasicIntervalSchedule.startTime" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.base.base.base.start_time = sv.clone(); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.base.base.base.start_time = sv.clone(); }
                        }
                        _ => {}
                    }
                }
                "BasicIntervalSchedule.value1Unit" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.base.base.base.value1unit = Some(crate::base::UriRef { uri: sv.clone() });
                    }
                }
                "BasicIntervalSchedule.value2Unit" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.base.base.base.value2unit = Some(crate::base::UriRef { uri: sv.clone() });
                    }
                }
                "IdentifiedObject.description" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.base.base.base.base.description = sv.clone(); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.base.base.base.base.description = sv.clone(); }
                        }
                        _ => {}
                    }
                }
                "IdentifiedObject.energyIdentCodeEic" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.base.base.base.base.energy_ident_code_eic = sv.clone(); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.base.base.base.base.energy_ident_code_eic = sv.clone(); }
                        }
                        _ => {}
                    }
                }
                "IdentifiedObject.mRID" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.base.base.base.base.m_rid = sv.clone(); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.base.base.base.base.m_rid = sv.clone(); }
                        }
                        _ => {}
                    }
                }
                "IdentifiedObject.name" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.base.base.base.base.name = sv.clone(); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.base.base.base.base.name = sv.clone(); }
                        }
                        _ => {}
                    }
                }
                "IdentifiedObject.shortName" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.base.base.base.base.short_name = sv.clone(); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.base.base.base.base.short_name = sv.clone(); }
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
