/// A variable impedance device normally used to offset line charging during single line faults in an ungrounded section of network.
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct PetersenCoil {
    #[serde(flatten)]
    pub base: super::EarthFaultCompensator,
    /// The mode of operation of the Petersen coil.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<super::base::UriRef>,
    /// The nominal voltage for which the coil is designed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nominal_u: Option<f64>,
    /// The offset current that the Petersen coil controller is operating from the resonant point. This is normally a fixed amount for which the controller is configured and could be positive or negative. Typically 0 to 60 A depending on voltage and resonance conditions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset_current: Option<f64>,
    /// The control current used to control the Petersen coil also known as the position current. Typically in the range of 20 mA to 200 mA.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position_current: Option<f64>,
    /// The maximum reactance.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x_ground_max: Option<f64>,
    /// The minimum reactance.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x_ground_min: Option<f64>,
    /// The nominal reactance. This is the operating point (normally over compensation) that is defined based on the resonance point in the healthy network condition. The impedance is calculated based on nominal voltage divided by position current.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x_ground_nominal: Option<f64>,
}
impl crate::base::CimElement for PetersenCoil {
    fn mrid(&self) -> &str { &self.base.base.base.base.base.id }
    fn type_name(&self) -> &'static str { "PetersenCoil" }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn to_json_value(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or(serde_json::Value::Null)
    }
    fn to_block(&self) -> crate::base::RdfBlock {
        let mut block = self.base.to_block();
        block.type_name = "PetersenCoil".to_string();
        if let Some(ref v) = self.mode {
            block.fields.insert("PetersenCoil.mode".into(), crate::base::FieldValue::Resource(v.uri.clone()));
        }
        if let Some(v) = self.nominal_u {
            block.fields.insert("PetersenCoil.nominalU".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.offset_current {
            block.fields.insert("PetersenCoil.offsetCurrent".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.position_current {
            block.fields.insert("PetersenCoil.positionCurrent".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.x_ground_max {
            block.fields.insert("PetersenCoil.xGroundMax".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.x_ground_min {
            block.fields.insert("PetersenCoil.xGroundMin".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.x_ground_nominal {
            block.fields.insert("PetersenCoil.xGroundNominal".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        block
    }
}

impl PetersenCoil {
    pub fn from_block(b: &crate::base::RdfBlock) -> Self {
        let mut obj = Self::default();
        obj.base.base.base.base.base.id.clone_from(&b.mrid);
        for (key, val) in &b.fields {
            match key.as_str() {
                "PetersenCoil.mode" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.mode = Some(crate::base::UriRef { uri: sv.clone() });
                    }
                }
                "PetersenCoil.nominalU" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.nominal_u = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.nominal_u = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "PetersenCoil.offsetCurrent" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.offset_current = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.offset_current = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "PetersenCoil.positionCurrent" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.position_current = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.position_current = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "PetersenCoil.xGroundMax" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.x_ground_max = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.x_ground_max = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "PetersenCoil.xGroundMin" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.x_ground_min = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.x_ground_min = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "PetersenCoil.xGroundNominal" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.x_ground_nominal = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.x_ground_nominal = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "EarthFaultCompensator.r" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.base.r = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.base.r = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ConductingEquipment.BaseVoltage" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.base.base.base_voltage = Some(crate::base::MridRef { mrid: sv.clone() });
                    }
                }
                "Equipment.EquipmentContainer" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.base.base.base.equipment_container = Some(crate::base::MridRef { mrid: sv.clone() });
                    }
                }
                "Equipment.aggregate" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.base.base.base.aggregate = Some(sv.trim() == "true"); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.base.base.base.aggregate = Some(sv.trim() == "true"); }
                        }
                        _ => {}
                    }
                }
                "Equipment.inService" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.base.base.base.in_service = Some(sv.trim() == "true"); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.base.base.base.in_service = Some(sv.trim() == "true"); }
                        }
                        _ => {}
                    }
                }
                "Equipment.normallyInService" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.base.base.base.normally_in_service = Some(sv.trim() == "true"); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.base.base.base.normally_in_service = Some(sv.trim() == "true"); }
                        }
                        _ => {}
                    }
                }
                "IdentifiedObject.description" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.base.base.base.base.base.description = sv.clone(); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.base.base.base.base.base.description = sv.clone(); }
                        }
                        _ => {}
                    }
                }
                "IdentifiedObject.energyIdentCodeEic" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.base.base.base.base.base.energy_ident_code_eic = sv.clone(); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.base.base.base.base.base.energy_ident_code_eic = sv.clone(); }
                        }
                        _ => {}
                    }
                }
                "IdentifiedObject.mRID" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.base.base.base.base.base.m_rid = sv.clone(); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.base.base.base.base.base.m_rid = sv.clone(); }
                        }
                        _ => {}
                    }
                }
                "IdentifiedObject.name" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.base.base.base.base.base.name = sv.clone(); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.base.base.base.base.base.name = sv.clone(); }
                        }
                        _ => {}
                    }
                }
                "IdentifiedObject.shortName" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.base.base.base.base.base.short_name = sv.clone(); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.base.base.base.base.base.short_name = sv.clone(); }
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
