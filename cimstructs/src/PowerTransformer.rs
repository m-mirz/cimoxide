/// An electrical device consisting of two or more coupled windings, with or without a magnetic core, for introducing mutual coupling between electric circuits. Transformers can be used to control voltage and phase shift (active power flow). A power transformer may be composed of separate transformer tanks that need not be identical. A power transformer can be modelled with or without tanks and is intended for use in both balanced and unbalanced representations. A power transformer typically has two terminals, but may have one (grounding), three or more terminals. The inherited association ConductingEquipment.BaseVoltage should not be used. The association from TransformerEnd to BaseVoltage should be used instead.
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct PowerTransformer {
    #[serde(flatten)]
    pub base: super::ConductingEquipment,
    /// The highest operating current (Ib in IEC 60909-0) before short circuit (depends on network configuration and relevant reliability philosophy). It is used for calculation of the impedance correction factor KT defined in IEC 60909-0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before_sh_circuit_highest_operating_current: Option<f64>,
    /// The highest operating voltage (Ub in IEC 60909-0) before short circuit. It is used for calculation of the impedance correction factor KT defined in IEC 60909-0. This is worst case voltage on the low side winding (3.7.1 of IEC 60909:2001). Used to define operating conditions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before_sh_circuit_highest_operating_voltage: Option<f64>,
    /// The angle of power factor before short circuit (phib in IEC 60909-0). It is used for calculation of the impedance correction factor KT defined in IEC 60909-0. This is the worst case power factor. Used to define operating conditions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before_short_circuit_angle_pf: Option<f64>,
    /// The minimum operating voltage (uQmin in IEC 60909-0) at the high voltage side (Q side) of the unit transformer of the power station unit. A value well established from long-term operating experience of the system. It is used for calculation of the impedance correction factor KG defined in IEC 60909-0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub high_side_min_operating_u: Option<f64>,
    /// Indicates whether the machine is part of a power station unit. Used for short circuit data exchange according to IEC 60909. It has an impact on how the correction factors are calculated for transformers, since the transformer is not necessarily part of a synchronous machine and generating unit. It is not always possible to derive this information from the model. This is why the attribute is necessary.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_part_of_generator_unit: Option<bool>,
    /// It is used to define if the data (other attributes related to short circuit data exchange) defines long term operational conditions or not. Used for short circuit data exchange according to IEC 60909.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operational_values_considered: Option<bool>,
}
impl crate::base::CimElement for PowerTransformer {
    fn mrid(&self) -> &str { &self.base.base.base.base.id }
    fn type_name(&self) -> &'static str { "PowerTransformer" }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn to_json_value(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or(serde_json::Value::Null)
    }
    fn to_block(&self) -> crate::base::RdfBlock {
        let mut block = self.base.to_block();
        block.type_name = "PowerTransformer".to_string();
        if let Some(v) = self.before_sh_circuit_highest_operating_current {
            block.fields.insert("PowerTransformer.beforeShCircuitHighestOperatingCurrent".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.before_sh_circuit_highest_operating_voltage {
            block.fields.insert("PowerTransformer.beforeShCircuitHighestOperatingVoltage".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.before_short_circuit_angle_pf {
            block.fields.insert("PowerTransformer.beforeShortCircuitAnglePf".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.high_side_min_operating_u {
            block.fields.insert("PowerTransformer.highSideMinOperatingU".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.is_part_of_generator_unit {
            block.fields.insert("PowerTransformer.isPartOfGeneratorUnit".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.operational_values_considered {
            block.fields.insert("PowerTransformer.operationalValuesConsidered".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        block
    }
}

impl PowerTransformer {
    pub fn from_block(b: &crate::base::RdfBlock) -> Self {
        let mut obj = Self::default();
        obj.base.base.base.base.id.clone_from(&b.mrid);
        for (key, val) in &b.fields {
            match key.as_str() {
                "PowerTransformer.beforeShCircuitHighestOperatingCurrent" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.before_sh_circuit_highest_operating_current = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.before_sh_circuit_highest_operating_current = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "PowerTransformer.beforeShCircuitHighestOperatingVoltage" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.before_sh_circuit_highest_operating_voltage = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.before_sh_circuit_highest_operating_voltage = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "PowerTransformer.beforeShortCircuitAnglePf" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.before_short_circuit_angle_pf = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.before_short_circuit_angle_pf = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "PowerTransformer.highSideMinOperatingU" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.high_side_min_operating_u = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.high_side_min_operating_u = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "PowerTransformer.isPartOfGeneratorUnit" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.is_part_of_generator_unit = Some(sv.trim() == "true"); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.is_part_of_generator_unit = Some(sv.trim() == "true"); }
                        }
                        _ => {}
                    }
                }
                "PowerTransformer.operationalValuesConsidered" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.operational_values_considered = Some(sv.trim() == "true"); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.operational_values_considered = Some(sv.trim() == "true"); }
                        }
                        _ => {}
                    }
                }
                "ConductingEquipment.BaseVoltage" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.base.base_voltage = Some(crate::base::MridRef { mrid: sv.clone() });
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
