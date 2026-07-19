/// A type of conducting equipment that can regulate a quantity (i.e. voltage or flow) at a specific point in the network.
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct RegulatingCondEq {
    #[serde(flatten)]
    pub base: super::EnergyConnection,
    /// The regulating control scheme in which this equipment participates.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub regulating_control: Option<super::base::MridRef>,
    /// Specifies the regulation status of the equipment. True is regulating, false is not regulating.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub control_enabled: Option<bool>,
}
impl crate::base::CimElement for RegulatingCondEq {
    fn mrid(&self) -> &str { &self.base.base.base.base.base.id }
    fn type_name(&self) -> &'static str { "RegulatingCondEq" }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn to_json_value(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or(serde_json::Value::Null)
    }
    fn to_block(&self) -> crate::base::RdfBlock {
        let mut block = self.base.to_block();
        block.type_name = "RegulatingCondEq".to_string();
        if let Some(ref v) = self.regulating_control {
            block.fields.insert("RegulatingCondEq.RegulatingControl".into(), crate::base::FieldValue::Resource(v.mrid.clone()));
        }
        if let Some(v) = self.control_enabled {
            block.fields.insert("RegulatingCondEq.controlEnabled".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        block
    }
}

impl RegulatingCondEq {
    pub fn from_block(b: &crate::base::RdfBlock) -> Self {
        let mut obj = Self::default();
        obj.base.base.base.base.base.id.clone_from(&b.mrid);
        for (key, val) in &b.fields {
            match key.as_str() {
                "RegulatingCondEq.RegulatingControl" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.regulating_control = Some(crate::base::MridRef { mrid: sv.clone() });
                    }
                }
                "RegulatingCondEq.controlEnabled" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.control_enabled = Some(sv.trim() == "true"); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.control_enabled = Some(sv.trim() == "true"); }
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
