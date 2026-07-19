/// A generic equivalent for an energy supplier on a transmission or distribution voltage level.
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct EnergySource {
    #[serde(flatten)]
    pub base: super::EnergyConnection,
    /// Energy Scheduling Type of an Energy Source.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub energy_scheduling_type: Option<super::base::MridRef>,
    /// High voltage source active injection. Load sign convention is used, i.e. positive sign means flow out from a node. Starting value for steady state solutions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_power: Option<f64>,
    /// Phase-to-phase nominal voltage.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nominal_voltage: Option<f64>,
    /// This is the maximum active power that can be produced by the source. Load sign convention is used, i.e. positive sign means flow out from a TopologicalNode (bus) into the conducting equipment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub p_max: Option<f64>,
    /// This is the minimum active power that can be produced by the source. Load sign convention is used, i.e. positive sign means flow out from a TopologicalNode (bus) into the conducting equipment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub p_min: Option<f64>,
    /// Positive sequence Thevenin resistance.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r: Option<f64>,
    /// Zero sequence Thevenin resistance.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r0: Option<f64>,
    /// High voltage source reactive injection. Load sign convention is used, i.e. positive sign means flow out from a node. Starting value for steady state solutions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reactive_power: Option<f64>,
    /// Negative sequence Thevenin resistance.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rn: Option<f64>,
    /// Phase angle of a-phase open circuit used when voltage characteristics need to be imposed at the node associated with the terminal of the energy source, such as when voltages and angles from the transmission level are used as input to the distribution network. The attribute shall be a positive value or zero.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voltage_angle: Option<f64>,
    /// Phase-to-phase open circuit voltage magnitude used when voltage characteristics need to be imposed at the node associated with the terminal of the energy source, such as when voltages and angles from the transmission level are used as input to the distribution network. The attribute shall be a positive value or zero.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voltage_magnitude: Option<f64>,
    /// Positive sequence Thevenin reactance.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x: Option<f64>,
    /// Zero sequence Thevenin reactance.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x0: Option<f64>,
    /// Negative sequence Thevenin reactance.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub xn: Option<f64>,
}
impl crate::base::CimElement for EnergySource {
    fn mrid(&self) -> &str { &self.base.base.base.base.base.id }
    fn type_name(&self) -> &'static str { "EnergySource" }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn to_json_value(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or(serde_json::Value::Null)
    }
    fn to_block(&self) -> crate::base::RdfBlock {
        let mut block = self.base.to_block();
        block.type_name = "EnergySource".to_string();
        if let Some(ref v) = self.energy_scheduling_type {
            block.fields.insert("EnergySource.EnergySchedulingType".into(), crate::base::FieldValue::Resource(v.mrid.clone()));
        }
        if let Some(v) = self.active_power {
            block.fields.insert("EnergySource.activePower".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.nominal_voltage {
            block.fields.insert("EnergySource.nominalVoltage".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.p_max {
            block.fields.insert("EnergySource.pMax".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.p_min {
            block.fields.insert("EnergySource.pMin".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.r {
            block.fields.insert("EnergySource.r".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.r0 {
            block.fields.insert("EnergySource.r0".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.reactive_power {
            block.fields.insert("EnergySource.reactivePower".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.rn {
            block.fields.insert("EnergySource.rn".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.voltage_angle {
            block.fields.insert("EnergySource.voltageAngle".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.voltage_magnitude {
            block.fields.insert("EnergySource.voltageMagnitude".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.x {
            block.fields.insert("EnergySource.x".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.x0 {
            block.fields.insert("EnergySource.x0".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.xn {
            block.fields.insert("EnergySource.xn".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        block
    }
}

impl EnergySource {
    pub fn from_block(b: &crate::base::RdfBlock) -> Self {
        let mut obj = Self::default();
        obj.base.base.base.base.base.id.clone_from(&b.mrid);
        for (key, val) in &b.fields {
            match key.as_str() {
                "EnergySource.EnergySchedulingType" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.energy_scheduling_type = Some(crate::base::MridRef { mrid: sv.clone() });
                    }
                }
                "EnergySource.activePower" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.active_power = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.active_power = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "EnergySource.nominalVoltage" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.nominal_voltage = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.nominal_voltage = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "EnergySource.pMax" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.p_max = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.p_max = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "EnergySource.pMin" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.p_min = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.p_min = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "EnergySource.r" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.r = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.r = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "EnergySource.r0" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.r0 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.r0 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "EnergySource.reactivePower" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.reactive_power = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.reactive_power = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "EnergySource.rn" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.rn = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.rn = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "EnergySource.voltageAngle" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.voltage_angle = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.voltage_angle = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "EnergySource.voltageMagnitude" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.voltage_magnitude = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.voltage_magnitude = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "EnergySource.x" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.x = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.x = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "EnergySource.x0" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.x0 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.x0 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "EnergySource.xn" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.xn = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.xn = Some(v); } }
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
