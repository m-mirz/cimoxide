/// A generic device designed to close, or open, or both, one or more electric circuits. All switches are two terminal devices including grounding switches. The ACDCTerminal.connected at the two sides of the switch shall not be considered for assessing switch connectivity, i.e. only Switch.open, .normalOpen and .locked are relevant.
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct Switch {
    #[serde(flatten)]
    pub base: super::ConductingEquipment,
    /// If true, the switch is locked. The resulting switch state is a combination of locked and Switch.open attributes as follows: locked=true and Switch.open=true. The resulting state is open and locked; locked=false and Switch.open=true. The resulting state is open; locked=false and Switch.open=false. The resulting state is closed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locked: Option<bool>,
    /// The attribute is used in cases when no Measurement for the status value is present. If the Switch has a status measurement the Discrete.normalValue is expected to match with the Switch.normalOpen.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub normal_open: Option<bool>,
    /// The attribute tells if the switch is considered open when used as input to topology processing.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub open: Option<bool>,
    /// The maximum continuous current carrying capacity in amps governed by the device material and construction. The attribute shall be a positive value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rated_current: Option<f64>,
    /// Branch is retained in the topological solution. The flow through retained switches will normally be calculated in power flow.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retained: Option<bool>,
}
impl crate::base::CimElement for Switch {
    fn mrid(&self) -> &str { &self.base.base.base.base.id }
    fn type_name(&self) -> &'static str { "Switch" }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn to_json_value(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or(serde_json::Value::Null)
    }
    fn to_block(&self) -> crate::base::RdfBlock {
        let mut block = self.base.to_block();
        block.type_name = "Switch".to_string();
        if let Some(v) = self.locked {
            block.fields.insert("Switch.locked".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.normal_open {
            block.fields.insert("Switch.normalOpen".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.open {
            block.fields.insert("Switch.open".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.rated_current {
            block.fields.insert("Switch.ratedCurrent".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.retained {
            block.fields.insert("Switch.retained".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        block
    }
}

impl Switch {
    pub fn from_block(b: &crate::base::RdfBlock) -> Self {
        let mut obj = Self::default();
        obj.base.base.base.base.id.clone_from(&b.mrid);
        for (key, val) in &b.fields {
            match key.as_str() {
                "Switch.locked" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.locked = Some(sv.trim() == "true"); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.locked = Some(sv.trim() == "true"); }
                        }
                        _ => {}
                    }
                }
                "Switch.normalOpen" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.normal_open = Some(sv.trim() == "true"); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.normal_open = Some(sv.trim() == "true"); }
                        }
                        _ => {}
                    }
                }
                "Switch.open" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.open = Some(sv.trim() == "true"); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.open = Some(sv.trim() == "true"); }
                        }
                        _ => {}
                    }
                }
                "Switch.ratedCurrent" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.rated_current = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.rated_current = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "Switch.retained" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.retained = Some(sv.trim() == "true"); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.retained = Some(sv.trim() == "true"); }
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
