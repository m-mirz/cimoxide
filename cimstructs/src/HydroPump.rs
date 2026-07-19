/// A synchronous motor-driven pump, typically associated with a pumped storage plant.
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct HydroPump {
    #[serde(flatten)]
    pub base: super::Equipment,
    /// The hydro pump may be a member of a pumped storage plant or a pump for distributing water.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hydro_power_plant: Option<super::base::MridRef>,
    /// The synchronous machine drives the turbine which moves the water from a low elevation to a higher elevation. The direction of machine rotation for pumping may or may not be the same as for generating.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rotating_machine: Option<super::base::MridRef>,
}
impl crate::base::CimElement for HydroPump {
    fn mrid(&self) -> &str { &self.base.base.base.id }
    fn type_name(&self) -> &'static str { "HydroPump" }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn to_json_value(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or(serde_json::Value::Null)
    }
    fn to_block(&self) -> crate::base::RdfBlock {
        let mut block = self.base.to_block();
        block.type_name = "HydroPump".to_string();
        if let Some(ref v) = self.hydro_power_plant {
            block.fields.insert("HydroPump.HydroPowerPlant".into(), crate::base::FieldValue::Resource(v.mrid.clone()));
        }
        if let Some(ref v) = self.rotating_machine {
            block.fields.insert("HydroPump.RotatingMachine".into(), crate::base::FieldValue::Resource(v.mrid.clone()));
        }
        block
    }
}

impl HydroPump {
    pub fn from_block(b: &crate::base::RdfBlock) -> Self {
        let mut obj = Self::default();
        obj.base.base.base.id.clone_from(&b.mrid);
        for (key, val) in &b.fields {
            match key.as_str() {
                "HydroPump.HydroPowerPlant" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.hydro_power_plant = Some(crate::base::MridRef { mrid: sv.clone() });
                    }
                }
                "HydroPump.RotatingMachine" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.rotating_machine = Some(crate::base::MridRef { mrid: sv.clone() });
                    }
                }
                "Equipment.EquipmentContainer" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.base.equipment_container = Some(crate::base::MridRef { mrid: sv.clone() });
                    }
                }
                "Equipment.aggregate" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.base.aggregate = Some(sv.trim() == "true"); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.base.aggregate = Some(sv.trim() == "true"); }
                        }
                        _ => {}
                    }
                }
                "Equipment.inService" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.base.in_service = Some(sv.trim() == "true"); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.base.in_service = Some(sv.trim() == "true"); }
                        }
                        _ => {}
                    }
                }
                "Equipment.normallyInService" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.base.normally_in_service = Some(sv.trim() == "true"); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.base.normally_in_service = Some(sv.trim() == "true"); }
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
