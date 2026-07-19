/// Generic user of energy - a point of consumption on the power system model. EnergyConsumer.pfixed, .qfixed, .pfixedPct and .qfixedPct have meaning only if there is no LoadResponseCharacteristic associated with EnergyConsumer or if LoadResponseCharacteristic.exponentModel is set to False.
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct EnergyConsumer {
    #[serde(flatten)]
    pub base: super::EnergyConnection,
    /// Load dynamics model used to describe dynamic behaviour of this energy consumer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub load_dynamics: Option<super::base::MridRef>,
    /// The load response characteristic of this load. If missing, this load is assumed to be constant power.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub load_response: Option<super::base::MridRef>,
    /// Active power of the load. Load sign convention is used, i.e. positive sign means flow out from a node. For voltage dependent loads the value is at rated voltage. Starting value for a steady state solution.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub p: Option<f64>,
    /// Active power of the load that is a fixed quantity and does not vary as load group value varies. Load sign convention is used, i.e. positive sign means flow out from a node.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pfixed: Option<f64>,
    /// Fixed active power as a percentage of load group fixed active power. Used to represent the time-varying components. Load sign convention is used, i.e. positive sign means flow out from a node.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pfixed_pct: Option<f64>,
    /// Reactive power of the load. Load sign convention is used, i.e. positive sign means flow out from a node. For voltage dependent loads the value is at rated voltage. Starting value for a steady state solution.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub q: Option<f64>,
    /// Reactive power of the load that is a fixed quantity and does not vary as load group value varies. Load sign convention is used, i.e. positive sign means flow out from a node.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qfixed: Option<f64>,
    /// Fixed reactive power as a percentage of load group fixed reactive power. Used to represent the time-varying components. Load sign convention is used, i.e. positive sign means flow out from a node.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qfixed_pct: Option<f64>,
}
impl crate::base::CimElement for EnergyConsumer {
    fn mrid(&self) -> &str { &self.base.base.base.base.base.id }
    fn type_name(&self) -> &'static str { "EnergyConsumer" }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn to_json_value(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or(serde_json::Value::Null)
    }
    fn to_block(&self) -> crate::base::RdfBlock {
        let mut block = self.base.to_block();
        block.type_name = "EnergyConsumer".to_string();
        if let Some(ref v) = self.load_dynamics {
            block.fields.insert("EnergyConsumer.LoadDynamics".into(), crate::base::FieldValue::Resource(v.mrid.clone()));
        }
        if let Some(ref v) = self.load_response {
            block.fields.insert("EnergyConsumer.LoadResponse".into(), crate::base::FieldValue::Resource(v.mrid.clone()));
        }
        if let Some(v) = self.p {
            block.fields.insert("EnergyConsumer.p".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.pfixed {
            block.fields.insert("EnergyConsumer.pfixed".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.pfixed_pct {
            block.fields.insert("EnergyConsumer.pfixedPct".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.q {
            block.fields.insert("EnergyConsumer.q".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.qfixed {
            block.fields.insert("EnergyConsumer.qfixed".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.qfixed_pct {
            block.fields.insert("EnergyConsumer.qfixedPct".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        block
    }
}

impl EnergyConsumer {
    pub fn from_block(b: &crate::base::RdfBlock) -> Self {
        let mut obj = Self::default();
        obj.base.base.base.base.base.id.clone_from(&b.mrid);
        for (key, val) in &b.fields {
            match key.as_str() {
                "EnergyConsumer.LoadDynamics" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.load_dynamics = Some(crate::base::MridRef { mrid: sv.clone() });
                    }
                }
                "EnergyConsumer.LoadResponse" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.load_response = Some(crate::base::MridRef { mrid: sv.clone() });
                    }
                }
                "EnergyConsumer.p" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.p = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.p = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "EnergyConsumer.pfixed" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.pfixed = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.pfixed = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "EnergyConsumer.pfixedPct" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.pfixed_pct = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.pfixed_pct = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "EnergyConsumer.q" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.q = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.q = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "EnergyConsumer.qfixed" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.qfixed = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.qfixed = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "EnergyConsumer.qfixedPct" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.qfixed_pct = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.qfixed_pct = Some(v); } }
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
