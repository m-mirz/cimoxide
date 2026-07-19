/// A facility for providing variable and controllable shunt reactive power. The SVC typically consists of a stepdown transformer, filter, thyristor-controlled reactor, and thyristor-switched capacitor arms. The SVC may operate in fixed MVar output mode or in voltage control mode. When in voltage control mode, the output of the SVC will be proportional to the deviation of voltage at the controlled bus from the voltage setpoint. The SVC characteristic slope defines the proportion. If the voltage at the controlled bus is equal to the voltage setpoint, the SVC MVar output is zero.
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct StaticVarCompensator {
    #[serde(flatten)]
    pub base: super::RegulatingCondEq,
    /// Capacitive reactance at maximum capacitive reactive power. Shall always be positive.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capacitive_rating: Option<f64>,
    /// Inductive reactance at maximum inductive reactive power. Shall always be negative.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inductive_rating: Option<f64>,
    /// Reactive power injection. Load sign convention is used, i.e. positive sign means flow out from a node. Starting value for a steady state solution.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub q: Option<f64>,
    /// SVC control mode.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s_vc_control_mode: Option<super::base::MridRef>,
    /// The characteristics slope of an SVC defines how the reactive power output changes in proportion to the difference between the regulated bus voltage and the voltage setpoint. The attribute shall be a positive value or zero.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slope: Option<f64>,
    /// The reactive power output of the SVC is proportional to the difference between the voltage at the regulated bus and the voltage setpoint. When the regulated bus voltage is equal to the voltage setpoint, the reactive power output is zero.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voltage_set_point: Option<f64>,
}
impl crate::base::CimElement for StaticVarCompensator {
    fn mrid(&self) -> &str { &self.base.base.base.base.base.base.id }
    fn type_name(&self) -> &'static str { "StaticVarCompensator" }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn to_json_value(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or(serde_json::Value::Null)
    }
    fn to_block(&self) -> crate::base::RdfBlock {
        let mut block = self.base.to_block();
        block.type_name = "StaticVarCompensator".to_string();
        if let Some(v) = self.capacitive_rating {
            block.fields.insert("StaticVarCompensator.capacitiveRating".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.inductive_rating {
            block.fields.insert("StaticVarCompensator.inductiveRating".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.q {
            block.fields.insert("StaticVarCompensator.q".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(ref v) = self.s_vc_control_mode {
            block.fields.insert("StaticVarCompensator.sVCControlMode".into(), crate::base::FieldValue::Resource(v.mrid.clone()));
        }
        if let Some(v) = self.slope {
            block.fields.insert("StaticVarCompensator.slope".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.voltage_set_point {
            block.fields.insert("StaticVarCompensator.voltageSetPoint".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        block
    }
}

impl StaticVarCompensator {
    pub fn from_block(b: &crate::base::RdfBlock) -> Self {
        let mut obj = Self::default();
        obj.base.base.base.base.base.base.id.clone_from(&b.mrid);
        for (key, val) in &b.fields {
            match key.as_str() {
                "StaticVarCompensator.capacitiveRating" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.capacitive_rating = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.capacitive_rating = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "StaticVarCompensator.inductiveRating" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.inductive_rating = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.inductive_rating = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "StaticVarCompensator.q" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.q = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.q = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "StaticVarCompensator.sVCControlMode" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.s_vc_control_mode = Some(crate::base::MridRef { mrid: sv.clone() });
                    }
                }
                "StaticVarCompensator.slope" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.slope = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.slope = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "StaticVarCompensator.voltageSetPoint" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.voltage_set_point = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.voltage_set_point = Some(v); } }
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
