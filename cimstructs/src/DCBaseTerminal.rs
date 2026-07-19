/// An electrical connection point at a piece of DC conducting equipment. DC terminals are connected at one physical DC node that may have multiple DC terminals connected. A DC node is similar to an AC connectivity node. The model requires that DC connections are distinct from AC connections.
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct DCBaseTerminal {
    #[serde(flatten)]
    pub base: super::ACDCTerminal,
    /// The DC connectivity node to which this DC base terminal connects with zero impedance.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dc_node: Option<super::base::MridRef>,
    /// See association end Terminal.TopologicalNode.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dc_topological_node: Option<super::base::MridRef>,
}
impl crate::base::CimElement for DCBaseTerminal {
    fn mrid(&self) -> &str { &self.base.base.id }
    fn type_name(&self) -> &'static str { "DCBaseTerminal" }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn to_json_value(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or(serde_json::Value::Null)
    }
    fn to_block(&self) -> crate::base::RdfBlock {
        let mut block = self.base.to_block();
        block.type_name = "DCBaseTerminal".to_string();
        if let Some(ref v) = self.dc_node {
            block.fields.insert("DCBaseTerminal.DCNode".into(), crate::base::FieldValue::Resource(v.mrid.clone()));
        }
        if let Some(ref v) = self.dc_topological_node {
            block.fields.insert("DCBaseTerminal.DCTopologicalNode".into(), crate::base::FieldValue::Resource(v.mrid.clone()));
        }
        block
    }
}

impl DCBaseTerminal {
    pub fn from_block(b: &crate::base::RdfBlock) -> Self {
        let mut obj = Self::default();
        obj.base.base.id.clone_from(&b.mrid);
        for (key, val) in &b.fields {
            match key.as_str() {
                "DCBaseTerminal.DCNode" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.dc_node = Some(crate::base::MridRef { mrid: sv.clone() });
                    }
                }
                "DCBaseTerminal.DCTopologicalNode" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.dc_topological_node = Some(crate::base::MridRef { mrid: sv.clone() });
                    }
                }
                "ACDCTerminal.BusNameMarker" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.base.bus_name_marker = Some(crate::base::MridRef { mrid: sv.clone() });
                    }
                }
                "ACDCTerminal.connected" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.base.connected = Some(sv.trim() == "true"); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.base.connected = Some(sv.trim() == "true"); }
                        }
                        _ => {}
                    }
                }
                "ACDCTerminal.sequenceNumber" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.base.sequence_number = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.base.sequence_number = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "IdentifiedObject.description" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.base.base.description = sv.clone(); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.base.base.description = sv.clone(); }
                        }
                        _ => {}
                    }
                }
                "IdentifiedObject.energyIdentCodeEic" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.base.base.energy_ident_code_eic = sv.clone(); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.base.base.energy_ident_code_eic = sv.clone(); }
                        }
                        _ => {}
                    }
                }
                "IdentifiedObject.mRID" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.base.base.m_rid = sv.clone(); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.base.base.m_rid = sv.clone(); }
                        }
                        _ => {}
                    }
                }
                "IdentifiedObject.name" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.base.base.name = sv.clone(); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.base.base.name = sv.clone(); }
                        }
                        _ => {}
                    }
                }
                "IdentifiedObject.shortName" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.base.base.short_name = sv.clone(); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.base.base.short_name = sv.clone(); }
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
