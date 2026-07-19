/// A wire or combination of wires, with consistent electrical characteristics, building a single electrical system, used to carry alternating current between points in the power system. For symmetrical, transposed three phase lines, it is sufficient to use attributes of the line segment, which describe impedances and admittances for the entire length of the segment. Additionally impedances can be computed by using length and associated per length impedances. The BaseVoltage at the two ends of ACLineSegments in a Line shall have the same BaseVoltage.nominalVoltage. However, boundary lines may have slightly different BaseVoltage.nominalVoltages and variation is allowed. Larger voltage difference in general requires use of an equivalent branch.
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct ACLineSegment {
    #[serde(flatten)]
    pub base: super::Conductor,
    /// Zero sequence shunt (charging) susceptance, uniformly distributed, of the entire line section.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub b0ch: Option<f64>,
    /// Positive sequence shunt (charging) susceptance, uniformly distributed, of the entire line section. This value represents the full charging over the full length of the line.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bch: Option<f64>,
    /// Zero sequence shunt (charging) conductance, uniformly distributed, of the entire line section.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub g0ch: Option<f64>,
    /// Positive sequence shunt (charging) conductance, uniformly distributed, of the entire line section.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gch: Option<f64>,
    /// Positive sequence series resistance of the entire line section.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r: Option<f64>,
    /// Zero sequence series resistance of the entire line section.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r0: Option<f64>,
    /// Maximum permitted temperature at the end of SC for the calculation of minimum short-circuit currents. Used for short circuit data exchange according to IEC 60909.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub short_circuit_end_temperature: Option<f64>,
    /// Positive sequence series reactance of the entire line section.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x: Option<f64>,
    /// Zero sequence series reactance of the entire line section.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x0: Option<f64>,
}
impl crate::base::CimElement for ACLineSegment {
    fn mrid(&self) -> &str { &self.base.base.base.base.base.id }
    fn type_name(&self) -> &'static str { "ACLineSegment" }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn to_json_value(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or(serde_json::Value::Null)
    }
    fn to_block(&self) -> crate::base::RdfBlock {
        let mut block = self.base.to_block();
        block.type_name = "ACLineSegment".to_string();
        if let Some(v) = self.b0ch {
            block.fields.insert("ACLineSegment.b0ch".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.bch {
            block.fields.insert("ACLineSegment.bch".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.g0ch {
            block.fields.insert("ACLineSegment.g0ch".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.gch {
            block.fields.insert("ACLineSegment.gch".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.r {
            block.fields.insert("ACLineSegment.r".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.r0 {
            block.fields.insert("ACLineSegment.r0".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.short_circuit_end_temperature {
            block.fields.insert("ACLineSegment.shortCircuitEndTemperature".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.x {
            block.fields.insert("ACLineSegment.x".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.x0 {
            block.fields.insert("ACLineSegment.x0".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        block
    }
}

impl ACLineSegment {
    pub fn from_block(b: &crate::base::RdfBlock) -> Self {
        let mut obj = Self::default();
        obj.base.base.base.base.base.id.clone_from(&b.mrid);
        for (key, val) in &b.fields {
            match key.as_str() {
                "ACLineSegment.b0ch" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.b0ch = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.b0ch = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ACLineSegment.bch" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.bch = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.bch = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ACLineSegment.g0ch" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.g0ch = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.g0ch = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ACLineSegment.gch" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.gch = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.gch = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ACLineSegment.r" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.r = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.r = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ACLineSegment.r0" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.r0 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.r0 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ACLineSegment.shortCircuitEndTemperature" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.short_circuit_end_temperature = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.short_circuit_end_temperature = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ACLineSegment.x" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.x = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.x = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ACLineSegment.x0" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.x0 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.x0 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "Conductor.length" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.base.length = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.base.length = Some(v); } }
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
