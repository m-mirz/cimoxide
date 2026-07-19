/// A linear shunt compensator has banks or sections with equal admittance values.
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct LinearShuntCompensator {
    #[serde(flatten)]
    pub base: super::ShuntCompensator,
    /// Zero sequence shunt (charging) susceptance per section.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub b0per_section: Option<f64>,
    /// Positive sequence shunt (charging) susceptance per section.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub b_per_section: Option<f64>,
    /// Zero sequence shunt (charging) conductance per section.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub g0per_section: Option<f64>,
    /// Positive sequence shunt (charging) conductance per section.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub g_per_section: Option<f64>,
}
impl crate::base::CimElement for LinearShuntCompensator {
    fn mrid(&self) -> &str { &self.base.base.base.base.base.base.base.id }
    fn type_name(&self) -> &'static str { "LinearShuntCompensator" }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn to_json_value(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or(serde_json::Value::Null)
    }
    fn to_block(&self) -> crate::base::RdfBlock {
        let mut block = self.base.to_block();
        block.type_name = "LinearShuntCompensator".to_string();
        if let Some(v) = self.b0per_section {
            block.fields.insert("LinearShuntCompensator.b0PerSection".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.b_per_section {
            block.fields.insert("LinearShuntCompensator.bPerSection".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.g0per_section {
            block.fields.insert("LinearShuntCompensator.g0PerSection".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.g_per_section {
            block.fields.insert("LinearShuntCompensator.gPerSection".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        block
    }
}

impl LinearShuntCompensator {
    pub fn from_block(b: &crate::base::RdfBlock) -> Self {
        let mut obj = Self::default();
        obj.base.base.base.base.base.base.base.id.clone_from(&b.mrid);
        for (key, val) in &b.fields {
            match key.as_str() {
                "LinearShuntCompensator.b0PerSection" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.b0per_section = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.b0per_section = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "LinearShuntCompensator.bPerSection" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.b_per_section = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.b_per_section = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "LinearShuntCompensator.g0PerSection" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.g0per_section = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.g0per_section = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "LinearShuntCompensator.gPerSection" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.g_per_section = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.g_per_section = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ShuntCompensator.aVRDelay" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.base.a_vr_delay = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.base.a_vr_delay = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ShuntCompensator.grounded" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.base.grounded = Some(sv.trim() == "true"); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.base.grounded = Some(sv.trim() == "true"); }
                        }
                        _ => {}
                    }
                }
                "ShuntCompensator.maximumSections" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.base.maximum_sections = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.base.maximum_sections = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ShuntCompensator.nomU" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.base.nom_u = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.base.nom_u = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ShuntCompensator.normalSections" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.base.normal_sections = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.base.normal_sections = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ShuntCompensator.sections" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.base.sections = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.base.sections = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ShuntCompensator.voltageSensitivity" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.base.voltage_sensitivity = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.base.voltage_sensitivity = Some(v); } }
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
