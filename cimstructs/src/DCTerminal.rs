/// An electrical connection point to generic DC conducting equipment.
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct DCTerminal {
    #[serde(flatten)]
    pub base: super::DCBaseTerminal,
    /// An DC terminal belong to a DC conducting equipment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dc_conducting_equipment: Option<super::base::MridRef>,
}
impl crate::base::CimElement for DCTerminal {
    fn mrid(&self) -> &str { &self.base.base.base.id }
    fn type_name(&self) -> &'static str { "DCTerminal" }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn to_json_value(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or(serde_json::Value::Null)
    }
    fn to_block(&self) -> crate::base::RdfBlock {
        let mut block = self.base.to_block();
        block.type_name = "DCTerminal".to_string();
        if let Some(ref v) = self.dc_conducting_equipment {
            block.fields.insert("DCTerminal.DCConductingEquipment".into(), crate::base::FieldValue::Resource(v.mrid.clone()));
        }
        block
    }
}

impl DCTerminal {
    pub fn from_block(b: &crate::base::RdfBlock) -> Self {
        let mut obj = Self::default();
        obj.base.base.base.id.clone_from(&b.mrid);
        for (key, val) in &b.fields {
            match key.as_str() {
                "DCTerminal.DCConductingEquipment" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.dc_conducting_equipment = Some(crate::base::MridRef { mrid: sv.clone() });
                    }
                }
                "DCBaseTerminal.DCNode" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.base.dc_node = Some(crate::base::MridRef { mrid: sv.clone() });
                    }
                }
                "DCBaseTerminal.DCTopologicalNode" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.base.dc_topological_node = Some(crate::base::MridRef { mrid: sv.clone() });
                    }
                }
                "ACDCTerminal.BusNameMarker" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.base.base.bus_name_marker = Some(crate::base::MridRef { mrid: sv.clone() });
                    }
                }
                "ACDCTerminal.connected" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.base.base.connected = Some(sv.trim() == "true"); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.base.base.connected = Some(sv.trim() == "true"); }
                        }
                        _ => {}
                    }
                }
                "ACDCTerminal.sequenceNumber" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.base.base.sequence_number = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.base.base.sequence_number = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "IdentifiedObject.description" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.base.base.base.description = sv.clone(); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.base.base.base.description = sv.clone(); }
                        }
                        _ => {}
                    }
                }
                "IdentifiedObject.energyIdentCodeEic" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.base.base.base.energy_ident_code_eic = sv.clone(); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.base.base.base.energy_ident_code_eic = sv.clone(); }
                        }
                        _ => {}
                    }
                }
                "IdentifiedObject.mRID" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.base.base.base.m_rid = sv.clone(); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.base.base.base.m_rid = sv.clone(); }
                        }
                        _ => {}
                    }
                }
                "IdentifiedObject.name" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.base.base.base.name = sv.clone(); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.base.base.base.name = sv.clone(); }
                        }
                        _ => {}
                    }
                }
                "IdentifiedObject.shortName" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.base.base.base.short_name = sv.clone(); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.base.base.base.short_name = sv.clone(); }
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
