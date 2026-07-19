/// A cut separates a line segment into two parts. The cut appears as a switch inserted between these two parts and connects them together. As the cut is normally open there is no galvanic connection between the two line segment parts. But it is possible to close the cut to get galvanic connection. The cut terminals are oriented towards the line segment terminals with the same sequence number. Hence the cut terminal with sequence number equal to 1 is oriented to the line segment's terminal with sequence number equal to 1. The cut terminals also act as connection points for jumpers and other equipment, e.g. a mobile generator. To enable this, connectivity nodes are placed at the cut terminals. Once the connectivity nodes are in place any conducting equipment can be connected at them.
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct Cut {
    #[serde(flatten)]
    pub base: super::Switch,
    /// The line segment to which the cut is applied.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ac_line_segment: Option<super::base::MridRef>,
    /// The length to the place where the cut is located starting from side one of the cut line segment, i.e. the line segment Terminal with sequenceNumber equal to 1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub length_from_terminal1: Option<f64>,
}
impl crate::base::CimElement for Cut {
    fn mrid(&self) -> &str { &self.base.base.base.base.base.id }
    fn type_name(&self) -> &'static str { "Cut" }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn to_json_value(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or(serde_json::Value::Null)
    }
    fn to_block(&self) -> crate::base::RdfBlock {
        let mut block = self.base.to_block();
        block.type_name = "Cut".to_string();
        if let Some(ref v) = self.ac_line_segment {
            block.fields.insert("Cut.ACLineSegment".into(), crate::base::FieldValue::Resource(v.mrid.clone()));
        }
        if let Some(v) = self.length_from_terminal1 {
            block.fields.insert("Cut.lengthFromTerminal1".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        block
    }
}

impl Cut {
    pub fn from_block(b: &crate::base::RdfBlock) -> Self {
        let mut obj = Self::default();
        obj.base.base.base.base.base.id.clone_from(&b.mrid);
        for (key, val) in &b.fields {
            match key.as_str() {
                "Cut.ACLineSegment" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.ac_line_segment = Some(crate::base::MridRef { mrid: sv.clone() });
                    }
                }
                "Cut.lengthFromTerminal1" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.length_from_terminal1 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.length_from_terminal1 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "Switch.locked" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.base.locked = Some(sv.trim() == "true"); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.base.locked = Some(sv.trim() == "true"); }
                        }
                        _ => {}
                    }
                }
                "Switch.normalOpen" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.base.normal_open = Some(sv.trim() == "true"); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.base.normal_open = Some(sv.trim() == "true"); }
                        }
                        _ => {}
                    }
                }
                "Switch.open" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.base.open = Some(sv.trim() == "true"); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.base.open = Some(sv.trim() == "true"); }
                        }
                        _ => {}
                    }
                }
                "Switch.ratedCurrent" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.base.rated_current = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.base.rated_current = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "Switch.retained" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.base.retained = Some(sv.trim() == "true"); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.base.retained = Some(sv.trim() == "true"); }
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
