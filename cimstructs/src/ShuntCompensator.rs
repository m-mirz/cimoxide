/// A shunt capacitor or reactor or switchable bank of shunt capacitors or reactors. A section of a shunt compensator is an individual capacitor or reactor. A negative value for bPerSection indicates that the compensator is a reactor. ShuntCompensator is a single terminal device. Ground is implied.
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct ShuntCompensator {
    #[serde(flatten)]
    pub base: super::RegulatingCondEq,
    /// An automatic voltage regulation delay (AVRDelay) which is the time delay from a change in voltage to when the capacitor is allowed to change state. This filters out temporary changes in voltage.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub a_vr_delay: Option<f64>,
    /// Used for Yn and Zn connections. True if the neutral is solidly grounded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grounded: Option<bool>,
    /// The maximum number of sections that may be switched in.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_sections: Option<i64>,
    /// The voltage at which the nominal reactive power may be calculated. This should normally be within 10% of the voltage at which the capacitor is connected to the network.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nom_u: Option<f64>,
    /// The normal number of sections switched in. The value shall be between zero and ShuntCompensator.maximumSections.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub normal_sections: Option<i64>,
    /// Shunt compensator sections in use. Starting value for steady state solution. The attribute shall be a positive value or zero. Non integer values are allowed to support continuous variables. The reasons for continuous value are to support study cases where no discrete shunt compensators has yet been designed, a solutions where a narrow voltage band force the sections to oscillate or accommodate for a continuous solution as input. For LinearShuntConpensator the value shall be between zero and ShuntCompensator.maximumSections. At value zero the shunt compensator conductance and admittance is zero. Linear interpolation of conductance and admittance between the previous and next integer section is applied in case of non-integer values. For NonlinearShuntCompensator-s shall only be set to one of the NonlinearShuntCompenstorPoint.sectionNumber. There is no interpolation between NonlinearShuntCompenstorPoint-s.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sections: Option<f64>,
    /// Voltage sensitivity required for the device to regulate the bus voltage, in voltage/reactive power.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voltage_sensitivity: Option<f64>,
}
impl crate::base::CimElement for ShuntCompensator {
    fn mrid(&self) -> &str { &self.base.base.base.base.base.base.id }
    fn type_name(&self) -> &'static str { "ShuntCompensator" }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn to_json_value(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or(serde_json::Value::Null)
    }
    fn to_block(&self) -> crate::base::RdfBlock {
        let mut block = self.base.to_block();
        block.type_name = "ShuntCompensator".to_string();
        if let Some(v) = self.a_vr_delay {
            block.fields.insert("ShuntCompensator.aVRDelay".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.grounded {
            block.fields.insert("ShuntCompensator.grounded".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.maximum_sections {
            block.fields.insert("ShuntCompensator.maximumSections".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.nom_u {
            block.fields.insert("ShuntCompensator.nomU".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.normal_sections {
            block.fields.insert("ShuntCompensator.normalSections".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.sections {
            block.fields.insert("ShuntCompensator.sections".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.voltage_sensitivity {
            block.fields.insert("ShuntCompensator.voltageSensitivity".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        block
    }
}

impl ShuntCompensator {
    pub fn from_block(b: &crate::base::RdfBlock) -> Self {
        let mut obj = Self::default();
        obj.base.base.base.base.base.base.id.clone_from(&b.mrid);
        for (key, val) in &b.fields {
            match key.as_str() {
                "ShuntCompensator.aVRDelay" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.a_vr_delay = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.a_vr_delay = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ShuntCompensator.grounded" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.grounded = Some(sv.trim() == "true"); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.grounded = Some(sv.trim() == "true"); }
                        }
                        _ => {}
                    }
                }
                "ShuntCompensator.maximumSections" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.maximum_sections = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.maximum_sections = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ShuntCompensator.nomU" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.nom_u = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.nom_u = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ShuntCompensator.normalSections" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.normal_sections = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.normal_sections = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ShuntCompensator.sections" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.sections = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.sections = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ShuntCompensator.voltageSensitivity" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.voltage_sensitivity = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.voltage_sensitivity = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "RegulatingCondEq.RegulatingControl" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.base.regulating_control = Some(crate::base::MridRef { mrid: sv.clone() });
                    }
                }
                "RegulatingCondEq.controlEnabled" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.base.control_enabled = Some(sv.trim() == "true"); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.base.control_enabled = Some(sv.trim() == "true"); }
                        }
                        _ => {}
                    }
                }
                "ConductingEquipment.BaseVoltage" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.base.base.base.base_voltage = Some(crate::base::MridRef { mrid: sv.clone() });
                    }
                }
                "Equipment.EquipmentContainer" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.base.base.base.base.equipment_container = Some(crate::base::MridRef { mrid: sv.clone() });
                    }
                }
                "Equipment.aggregate" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.base.base.base.base.aggregate = Some(sv.trim() == "true"); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.base.base.base.base.aggregate = Some(sv.trim() == "true"); }
                        }
                        _ => {}
                    }
                }
                "Equipment.inService" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.base.base.base.base.in_service = Some(sv.trim() == "true"); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.base.base.base.base.in_service = Some(sv.trim() == "true"); }
                        }
                        _ => {}
                    }
                }
                "Equipment.normallyInService" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.base.base.base.base.normally_in_service = Some(sv.trim() == "true"); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.base.base.base.base.normally_in_service = Some(sv.trim() == "true"); }
                        }
                        _ => {}
                    }
                }
                "IdentifiedObject.description" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.base.base.base.base.base.base.description = sv.clone(); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.base.base.base.base.base.base.description = sv.clone(); }
                        }
                        _ => {}
                    }
                }
                "IdentifiedObject.energyIdentCodeEic" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.base.base.base.base.base.base.energy_ident_code_eic = sv.clone(); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.base.base.base.base.base.base.energy_ident_code_eic = sv.clone(); }
                        }
                        _ => {}
                    }
                }
                "IdentifiedObject.mRID" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.base.base.base.base.base.base.m_rid = sv.clone(); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.base.base.base.base.base.base.m_rid = sv.clone(); }
                        }
                        _ => {}
                    }
                }
                "IdentifiedObject.name" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.base.base.base.base.base.base.name = sv.clone(); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.base.base.base.base.base.base.name = sv.clone(); }
                        }
                        _ => {}
                    }
                }
                "IdentifiedObject.shortName" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.base.base.base.base.base.base.short_name = sv.clone(); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.base.base.base.base.base.base.short_name = sv.clone(); }
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
