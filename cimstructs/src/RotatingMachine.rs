/// A rotating machine which may be used as a generator or motor.
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct RotatingMachine {
    #[serde(flatten)]
    pub base: super::RegulatingCondEq,
    /// A synchronous machine may operate as a generator and as such becomes a member of a generating unit.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generating_unit: Option<super::base::MridRef>,
    /// Active power injection. Load sign convention is used, i.e. positive sign means flow out from a node. Starting value for a steady state solution.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub p: Option<f64>,
    /// Reactive power injection. Load sign convention is used, i.e. positive sign means flow out from a node. Starting value for a steady state solution.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub q: Option<f64>,
    /// Power factor (nameplate data). It is primarily used for short circuit data exchange according to IEC 60909. The attribute cannot be a negative value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rated_power_factor: Option<f64>,
    /// Nameplate apparent power rating for the unit. The attribute shall have a positive value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rated_s: Option<f64>,
    /// Rated voltage (nameplate data, Ur in IEC 60909-0). It is primarily used for short circuit data exchange according to IEC 60909. The attribute shall be a positive value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rated_u: Option<f64>,
}
impl crate::base::CimElement for RotatingMachine {
    fn mrid(&self) -> &str { &self.base.base.base.base.base.base.id }
    fn type_name(&self) -> &'static str { "RotatingMachine" }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn to_json_value(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or(serde_json::Value::Null)
    }
    fn to_block(&self) -> crate::base::RdfBlock {
        let mut block = self.base.to_block();
        block.type_name = "RotatingMachine".to_string();
        if let Some(ref v) = self.generating_unit {
            block.fields.insert("RotatingMachine.GeneratingUnit".into(), crate::base::FieldValue::Resource(v.mrid.clone()));
        }
        if let Some(v) = self.p {
            block.fields.insert("RotatingMachine.p".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.q {
            block.fields.insert("RotatingMachine.q".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.rated_power_factor {
            block.fields.insert("RotatingMachine.ratedPowerFactor".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.rated_s {
            block.fields.insert("RotatingMachine.ratedS".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.rated_u {
            block.fields.insert("RotatingMachine.ratedU".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        block
    }
}

impl RotatingMachine {
    pub fn from_block(b: &crate::base::RdfBlock) -> Self {
        let mut obj = Self::default();
        obj.base.base.base.base.base.base.id.clone_from(&b.mrid);
        for (key, val) in &b.fields {
            match key.as_str() {
                "RotatingMachine.GeneratingUnit" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.generating_unit = Some(crate::base::MridRef { mrid: sv.clone() });
                    }
                }
                "RotatingMachine.p" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.p = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.p = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "RotatingMachine.q" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.q = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.q = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "RotatingMachine.ratedPowerFactor" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.rated_power_factor = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.rated_power_factor = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "RotatingMachine.ratedS" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.rated_s = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.rated_s = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "RotatingMachine.ratedU" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.rated_u = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.rated_u = Some(v); } }
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
