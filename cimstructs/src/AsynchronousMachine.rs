/// A rotating machine whose shaft rotates asynchronously with the electrical field. Also known as an induction machine with no external connection to the rotor windings, e.g. squirrel-cage induction machine.
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct AsynchronousMachine {
    #[serde(flatten)]
    pub base: super::RotatingMachine,
    /// Indicates the type of Asynchronous Machine (motor or generator).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asynchronous_machine_type: Option<super::base::UriRef>,
    /// Indicates whether the machine is a converter fed drive. Used for short circuit data exchange according to IEC 60909.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub converter_fed_drive: Option<bool>,
    /// Efficiency of the asynchronous machine at nominal operation as a percentage. Indicator for converter drive motors. Used for short circuit data exchange according to IEC 60909.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub efficiency: Option<f64>,
    /// Ratio of locked-rotor current to the rated current of the motor (Ia/Ir). Used for short circuit data exchange according to IEC 60909.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ia_ir_ratio: Option<f64>,
    /// Nameplate data indicates if the machine is 50 Hz or 60 Hz.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nominal_frequency: Option<f64>,
    /// Nameplate data. Depends on the slip and number of pole pairs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nominal_speed: Option<f64>,
    /// Number of pole pairs of stator. Used for short circuit data exchange according to IEC 60909.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pole_pair_number: Option<i64>,
    /// Rated mechanical power (Pr in IEC 60909-0). Used for short circuit data exchange according to IEC 60909.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rated_mechanical_power: Option<f64>,
    /// Indicates for converter drive motors if the power can be reversible. Used for short circuit data exchange according to IEC 60909.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reversible: Option<bool>,
    /// Locked rotor ratio (R/X). Used for short circuit data exchange according to IEC 60909.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rx_locked_rotor_ratio: Option<f64>,
}
impl crate::base::CimElement for AsynchronousMachine {
    fn mrid(&self) -> &str { &self.base.base.base.base.base.base.base.id }
    fn type_name(&self) -> &'static str { "AsynchronousMachine" }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn to_json_value(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or(serde_json::Value::Null)
    }
    fn to_block(&self) -> crate::base::RdfBlock {
        let mut block = self.base.to_block();
        block.type_name = "AsynchronousMachine".to_string();
        if let Some(ref v) = self.asynchronous_machine_type {
            block.fields.insert("AsynchronousMachine.asynchronousMachineType".into(), crate::base::FieldValue::Resource(v.uri.clone()));
        }
        if let Some(v) = self.converter_fed_drive {
            block.fields.insert("AsynchronousMachine.converterFedDrive".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.efficiency {
            block.fields.insert("AsynchronousMachine.efficiency".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.ia_ir_ratio {
            block.fields.insert("AsynchronousMachine.iaIrRatio".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.nominal_frequency {
            block.fields.insert("AsynchronousMachine.nominalFrequency".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.nominal_speed {
            block.fields.insert("AsynchronousMachine.nominalSpeed".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.pole_pair_number {
            block.fields.insert("AsynchronousMachine.polePairNumber".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.rated_mechanical_power {
            block.fields.insert("AsynchronousMachine.ratedMechanicalPower".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.reversible {
            block.fields.insert("AsynchronousMachine.reversible".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.rx_locked_rotor_ratio {
            block.fields.insert("AsynchronousMachine.rxLockedRotorRatio".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        block
    }
}

impl AsynchronousMachine {
    pub fn from_block(b: &crate::base::RdfBlock) -> Self {
        let mut obj = Self::default();
        obj.base.base.base.base.base.base.base.id.clone_from(&b.mrid);
        for (key, val) in &b.fields {
            match key.as_str() {
                "AsynchronousMachine.asynchronousMachineType" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.asynchronous_machine_type = Some(crate::base::UriRef { uri: sv.clone() });
                    }
                }
                "AsynchronousMachine.converterFedDrive" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.converter_fed_drive = Some(sv.trim() == "true"); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.converter_fed_drive = Some(sv.trim() == "true"); }
                        }
                        _ => {}
                    }
                }
                "AsynchronousMachine.efficiency" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.efficiency = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.efficiency = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "AsynchronousMachine.iaIrRatio" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.ia_ir_ratio = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.ia_ir_ratio = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "AsynchronousMachine.nominalFrequency" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.nominal_frequency = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.nominal_frequency = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "AsynchronousMachine.nominalSpeed" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.nominal_speed = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.nominal_speed = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "AsynchronousMachine.polePairNumber" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.pole_pair_number = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.pole_pair_number = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "AsynchronousMachine.ratedMechanicalPower" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.rated_mechanical_power = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.rated_mechanical_power = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "AsynchronousMachine.reversible" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.reversible = Some(sv.trim() == "true"); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.reversible = Some(sv.trim() == "true"); }
                        }
                        _ => {}
                    }
                }
                "AsynchronousMachine.rxLockedRotorRatio" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.rx_locked_rotor_ratio = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.rx_locked_rotor_ratio = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "RotatingMachine.GeneratingUnit" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.base.generating_unit = Some(crate::base::MridRef { mrid: sv.clone() });
                    }
                }
                "RotatingMachine.p" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.base.p = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.base.p = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "RotatingMachine.q" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.base.q = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.base.q = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "RotatingMachine.ratedPowerFactor" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.base.rated_power_factor = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.base.rated_power_factor = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "RotatingMachine.ratedS" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.base.rated_s = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.base.rated_s = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "RotatingMachine.ratedU" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.base.rated_u = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.base.rated_u = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "RegulatingCondEq.RegulatingControl" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.base.base.regulating_control = Some(crate::base::MridRef { mrid: sv.clone() });
                    }
                }
                "RegulatingCondEq.controlEnabled" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.base.base.control_enabled = Some(sv.trim() == "true"); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.base.base.control_enabled = Some(sv.trim() == "true"); }
                        }
                        _ => {}
                    }
                }
                "ConductingEquipment.BaseVoltage" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.base.base.base.base.base_voltage = Some(crate::base::MridRef { mrid: sv.clone() });
                    }
                }
                "Equipment.EquipmentContainer" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.base.base.base.base.base.equipment_container = Some(crate::base::MridRef { mrid: sv.clone() });
                    }
                }
                "Equipment.aggregate" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.base.base.base.base.base.aggregate = Some(sv.trim() == "true"); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.base.base.base.base.base.aggregate = Some(sv.trim() == "true"); }
                        }
                        _ => {}
                    }
                }
                "Equipment.inService" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.base.base.base.base.base.in_service = Some(sv.trim() == "true"); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.base.base.base.base.base.in_service = Some(sv.trim() == "true"); }
                        }
                        _ => {}
                    }
                }
                "Equipment.normallyInService" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.base.base.base.base.base.normally_in_service = Some(sv.trim() == "true"); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.base.base.base.base.base.normally_in_service = Some(sv.trim() == "true"); }
                        }
                        _ => {}
                    }
                }
                "IdentifiedObject.description" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.base.base.base.base.base.base.base.description = sv.clone(); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.base.base.base.base.base.base.base.description = sv.clone(); }
                        }
                        _ => {}
                    }
                }
                "IdentifiedObject.energyIdentCodeEic" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.base.base.base.base.base.base.base.energy_ident_code_eic = sv.clone(); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.base.base.base.base.base.base.base.energy_ident_code_eic = sv.clone(); }
                        }
                        _ => {}
                    }
                }
                "IdentifiedObject.mRID" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.base.base.base.base.base.base.base.m_rid = sv.clone(); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.base.base.base.base.base.base.base.m_rid = sv.clone(); }
                        }
                        _ => {}
                    }
                }
                "IdentifiedObject.name" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.base.base.base.base.base.base.base.name = sv.clone(); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.base.base.base.base.base.base.base.name = sv.clone(); }
                        }
                        _ => {}
                    }
                }
                "IdentifiedObject.shortName" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.base.base.base.base.base.base.base.short_name = sv.clone(); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.base.base.base.base.base.base.base.short_name = sv.clone(); }
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
