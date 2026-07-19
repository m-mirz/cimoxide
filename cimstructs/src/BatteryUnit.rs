/// An electrochemical energy storage device.
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct BatteryUnit {
    #[serde(flatten)]
    pub base: super::PowerElectronicsUnit,
    /// The current state of the battery (charging, full, etc.).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub battery_state: Option<super::base::UriRef>,
    /// Full energy storage capacity of the battery. The attribute shall be a positive value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rated_e: Option<f64>,
    /// Amount of energy currently stored. The attribute shall be a positive value or zero and lower than BatteryUnit.ratedE.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stored_e: Option<f64>,
}
impl crate::base::CimElement for BatteryUnit {
    fn mrid(&self) -> &str { &self.base.base.base.base.id }
    fn type_name(&self) -> &'static str { "BatteryUnit" }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn to_json_value(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or(serde_json::Value::Null)
    }
    fn to_block(&self) -> crate::base::RdfBlock {
        let mut block = self.base.to_block();
        block.type_name = "BatteryUnit".to_string();
        if let Some(ref v) = self.battery_state {
            block.fields.insert("BatteryUnit.batteryState".into(), crate::base::FieldValue::Resource(v.uri.clone()));
        }
        if let Some(v) = self.rated_e {
            block.fields.insert("BatteryUnit.ratedE".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.stored_e {
            block.fields.insert("BatteryUnit.storedE".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        block
    }
}

impl BatteryUnit {
    pub fn from_block(b: &crate::base::RdfBlock) -> Self {
        let mut obj = Self::default();
        obj.base.base.base.base.id.clone_from(&b.mrid);
        for (key, val) in &b.fields {
            match key.as_str() {
                "BatteryUnit.batteryState" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.battery_state = Some(crate::base::UriRef { uri: sv.clone() });
                    }
                }
                "BatteryUnit.ratedE" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.rated_e = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.rated_e = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "BatteryUnit.storedE" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.stored_e = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.stored_e = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "PowerElectronicsUnit.maxP" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.base.max_p = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.base.max_p = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "PowerElectronicsUnit.minP" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.base.min_p = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.base.min_p = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "Equipment.EquipmentContainer" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.base.base.equipment_container = Some(crate::base::MridRef { mrid: sv.clone() });
                    }
                }
                "Equipment.aggregate" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.base.base.aggregate = Some(sv.trim() == "true"); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.base.base.aggregate = Some(sv.trim() == "true"); }
                        }
                        _ => {}
                    }
                }
                "Equipment.inService" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.base.base.in_service = Some(sv.trim() == "true"); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.base.base.in_service = Some(sv.trim() == "true"); }
                        }
                        _ => {}
                    }
                }
                "Equipment.normallyInService" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.base.base.normally_in_service = Some(sv.trim() == "true"); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.base.base.normally_in_service = Some(sv.trim() == "true"); }
                        }
                        _ => {}
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
