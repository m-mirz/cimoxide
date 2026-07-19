/// This class represents the external network and it is used for IEC 60909 calculations.
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct ExternalNetworkInjection {
    #[serde(flatten)]
    pub base: super::RegulatingCondEq,
    /// Power Frequency Bias. This is the change in power injection divided by the change in frequency and negated. A positive value of the power frequency bias provides additional power injection upon a drop in frequency.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub governor_scd: Option<f64>,
    /// Indicates whether initial symmetrical short-circuit current and power have been calculated according to IEC (Ik'). Used only if short circuit calculations are done according to superposition method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ik_second: Option<bool>,
    /// Maximum initial symmetrical short-circuit currents (Ik' max) in A (Ik' = Sk'/(SQRT(3) Un)). Used for short circuit data exchange according to IEC 60909.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_initial_sym_sh_c_current: Option<f64>,
    /// Maximum active power of the injection.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_p: Option<f64>,
    /// Maximum reactive power limit. It is used for modelling of infeed for load flow exchange and not for short circuit modelling.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_q: Option<f64>,
    /// Maximum ratio of zero sequence resistance of Network Feeder to its zero sequence reactance (R(0)/X(0) max). Used for short circuit data exchange according to IEC 60909.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_r0to_x0ratio: Option<f64>,
    /// Maximum ratio of positive sequence resistance of Network Feeder to its positive sequence reactance (R(1)/X(1) max). Used for short circuit data exchange according to IEC 60909.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_r1to_x1ratio: Option<f64>,
    /// Maximum ratio of zero sequence impedance to its positive sequence impedance (Z(0)/Z(1) max). Used for short circuit data exchange according to IEC 60909.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_z0to_z1ratio: Option<f64>,
    /// Minimum initial symmetrical short-circuit currents (Ik' min) in A (Ik' = Sk'/(SQRT(3) Un)). Used for short circuit data exchange according to IEC 60909.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_initial_sym_sh_c_current: Option<f64>,
    /// Minimum active power of the injection.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_p: Option<f64>,
    /// Minimum reactive power limit. It is used for modelling of infeed for load flow exchange and not for short circuit modelling.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_q: Option<f64>,
    /// Indicates whether initial symmetrical short-circuit current and power have been calculated according to IEC (Ik'). Used for short circuit data exchange according to IEC 6090.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_r0to_x0ratio: Option<f64>,
    /// Minimum ratio of positive sequence resistance of Network Feeder to its positive sequence reactance (R(1)/X(1) min). Used for short circuit data exchange according to IEC 60909.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_r1to_x1ratio: Option<f64>,
    /// Minimum ratio of zero sequence impedance to its positive sequence impedance (Z(0)/Z(1) min). Used for short circuit data exchange according to IEC 60909.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_z0to_z1ratio: Option<f64>,
    /// Active power injection. Load sign convention is used, i.e. positive sign means flow out from a node. Starting value for steady state solutions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub p: Option<f64>,
    /// Reactive power injection. Load sign convention is used, i.e. positive sign means flow out from a node. Starting value for steady state solutions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub q: Option<f64>,
    /// Priority of unit for use as powerflow voltage phase angle reference bus selection. 0 = don t care (default) 1 = highest priority. 2 is less than 1 and so on.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference_priority: Option<i64>,
    /// Voltage factor in pu, which was used to calculate short-circuit current Ik' and power Sk'. Used only if short circuit calculations are done according to superposition method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voltage_factor: Option<f64>,
}
impl crate::base::CimElement for ExternalNetworkInjection {
    fn mrid(&self) -> &str { &self.base.base.base.base.base.base.id }
    fn type_name(&self) -> &'static str { "ExternalNetworkInjection" }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn to_json_value(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or(serde_json::Value::Null)
    }
    fn to_block(&self) -> crate::base::RdfBlock {
        let mut block = self.base.to_block();
        block.type_name = "ExternalNetworkInjection".to_string();
        if let Some(v) = self.governor_scd {
            block.fields.insert("ExternalNetworkInjection.governorSCD".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.ik_second {
            block.fields.insert("ExternalNetworkInjection.ikSecond".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.max_initial_sym_sh_c_current {
            block.fields.insert("ExternalNetworkInjection.maxInitialSymShCCurrent".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.max_p {
            block.fields.insert("ExternalNetworkInjection.maxP".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.max_q {
            block.fields.insert("ExternalNetworkInjection.maxQ".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.max_r0to_x0ratio {
            block.fields.insert("ExternalNetworkInjection.maxR0ToX0Ratio".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.max_r1to_x1ratio {
            block.fields.insert("ExternalNetworkInjection.maxR1ToX1Ratio".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.max_z0to_z1ratio {
            block.fields.insert("ExternalNetworkInjection.maxZ0ToZ1Ratio".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.min_initial_sym_sh_c_current {
            block.fields.insert("ExternalNetworkInjection.minInitialSymShCCurrent".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.min_p {
            block.fields.insert("ExternalNetworkInjection.minP".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.min_q {
            block.fields.insert("ExternalNetworkInjection.minQ".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.min_r0to_x0ratio {
            block.fields.insert("ExternalNetworkInjection.minR0ToX0Ratio".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.min_r1to_x1ratio {
            block.fields.insert("ExternalNetworkInjection.minR1ToX1Ratio".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.min_z0to_z1ratio {
            block.fields.insert("ExternalNetworkInjection.minZ0ToZ1Ratio".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.p {
            block.fields.insert("ExternalNetworkInjection.p".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.q {
            block.fields.insert("ExternalNetworkInjection.q".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.reference_priority {
            block.fields.insert("ExternalNetworkInjection.referencePriority".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.voltage_factor {
            block.fields.insert("ExternalNetworkInjection.voltageFactor".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        block
    }
}

impl ExternalNetworkInjection {
    pub fn from_block(b: &crate::base::RdfBlock) -> Self {
        let mut obj = Self::default();
        obj.base.base.base.base.base.base.id.clone_from(&b.mrid);
        for (key, val) in &b.fields {
            match key.as_str() {
                "ExternalNetworkInjection.governorSCD" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.governor_scd = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.governor_scd = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExternalNetworkInjection.ikSecond" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.ik_second = Some(sv.trim() == "true"); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.ik_second = Some(sv.trim() == "true"); }
                        }
                        _ => {}
                    }
                }
                "ExternalNetworkInjection.maxInitialSymShCCurrent" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.max_initial_sym_sh_c_current = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.max_initial_sym_sh_c_current = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExternalNetworkInjection.maxP" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.max_p = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.max_p = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExternalNetworkInjection.maxQ" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.max_q = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.max_q = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExternalNetworkInjection.maxR0ToX0Ratio" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.max_r0to_x0ratio = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.max_r0to_x0ratio = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExternalNetworkInjection.maxR1ToX1Ratio" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.max_r1to_x1ratio = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.max_r1to_x1ratio = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExternalNetworkInjection.maxZ0ToZ1Ratio" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.max_z0to_z1ratio = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.max_z0to_z1ratio = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExternalNetworkInjection.minInitialSymShCCurrent" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.min_initial_sym_sh_c_current = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.min_initial_sym_sh_c_current = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExternalNetworkInjection.minP" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.min_p = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.min_p = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExternalNetworkInjection.minQ" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.min_q = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.min_q = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExternalNetworkInjection.minR0ToX0Ratio" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.min_r0to_x0ratio = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.min_r0to_x0ratio = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExternalNetworkInjection.minR1ToX1Ratio" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.min_r1to_x1ratio = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.min_r1to_x1ratio = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExternalNetworkInjection.minZ0ToZ1Ratio" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.min_z0to_z1ratio = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.min_z0to_z1ratio = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExternalNetworkInjection.p" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.p = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.p = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExternalNetworkInjection.q" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.q = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.q = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExternalNetworkInjection.referencePriority" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.reference_priority = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.reference_priority = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExternalNetworkInjection.voltageFactor" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.voltage_factor = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.voltage_factor = Some(v); } }
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
