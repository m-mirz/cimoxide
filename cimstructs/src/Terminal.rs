/// An AC electrical connection point to a piece of conducting equipment. Terminals are connected at physical connection points called connectivity nodes.
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct Terminal {
    #[serde(flatten)]
    pub base: super::ACDCTerminal,
    /// The conducting equipment of the terminal. Conducting equipment have terminals that may be connected to other conducting equipment terminals via connectivity nodes or topological nodes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conducting_equipment: Option<super::base::MridRef>,
    /// The connectivity node to which this terminal connects with zero impedance.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connectivity_node: Option<super::base::MridRef>,
    /// The topological node associated with the terminal. This can be used as an alternative to the connectivity node path to topological node, thus making it unnecessary to model connectivity nodes in some cases. Note that the if connectivity nodes are in the model, this association would probably not be used as an input specification.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topological_node: Option<super::base::MridRef>,
    /// Represents the normal network phasing condition. If the attribute is missing, three phases (ABC) shall be assumed, except for terminals of grounding classes (specializations of EarthFaultCompensator, GroundDisconnector, and Ground) which will be assumed to be N. Therefore, phase code ABCN is explicitly declared when needed, e.g. for star point grounding equipment. The phase code on terminals connecting same ConnectivityNode or same TopologicalNode as well as for equipment between two terminals shall be consistent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phases: Option<super::base::UriRef>,
}
impl crate::base::CimElement for Terminal {
    fn mrid(&self) -> &str { &self.base.base.id }
    fn type_name(&self) -> &'static str { "Terminal" }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn to_json_value(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or(serde_json::Value::Null)
    }
    fn to_block(&self) -> crate::base::RdfBlock {
        let mut block = self.base.to_block();
        block.type_name = "Terminal".to_string();
        if let Some(ref v) = self.conducting_equipment {
            block.fields.insert("Terminal.ConductingEquipment".into(), crate::base::FieldValue::Resource(v.mrid.clone()));
        }
        if let Some(ref v) = self.connectivity_node {
            block.fields.insert("Terminal.ConnectivityNode".into(), crate::base::FieldValue::Resource(v.mrid.clone()));
        }
        if let Some(ref v) = self.topological_node {
            block.fields.insert("Terminal.TopologicalNode".into(), crate::base::FieldValue::Resource(v.mrid.clone()));
        }
        if let Some(ref v) = self.phases {
            block.fields.insert("Terminal.phases".into(), crate::base::FieldValue::Resource(v.uri.clone()));
        }
        block
    }
}

impl Terminal {
    pub fn from_block(b: &crate::base::RdfBlock) -> Self {
        let mut obj = Self::default();
        obj.base.base.id.clone_from(&b.mrid);
        for (key, val) in &b.fields {
            match key.as_str() {
                "Terminal.ConductingEquipment" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.conducting_equipment = Some(crate::base::MridRef { mrid: sv.clone() });
                    }
                }
                "Terminal.ConnectivityNode" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.connectivity_node = Some(crate::base::MridRef { mrid: sv.clone() });
                    }
                }
                "Terminal.TopologicalNode" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.topological_node = Some(crate::base::MridRef { mrid: sv.clone() });
                    }
                }
                "Terminal.phases" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.phases = Some(crate::base::UriRef { uri: sv.clone() });
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
