/// An electrical connection point (AC or DC) to a piece of conducting equipment. Terminals are connected at physical connection points called connectivity nodes.
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct ACDCTerminal {
    #[serde(flatten)]
    pub base: super::IdentifiedObject,
    /// The bus name marker used to name the bus (topological node).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bus_name_marker: Option<super::base::MridRef>,
    /// The connected status is related to a bus-branch model and the topological node to terminal relation. True implies the terminal is connected to the related topological node and false implies it is not. In a bus-branch model, the connected status is used to tell if equipment is disconnected without having to change the connectivity described by the topological node to terminal relation. A valid case is that conducting equipment can be connected in one end and open in the other. In particular for an AC line segment, where the reactive line charging can be significant, this is a relevant case.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connected: Option<bool>,
    /// The orientation of the terminal connections for a multiple terminal conducting equipment. The sequence numbering starts with 1 and additional terminals should follow in increasing order. The first terminal is the 'starting point' for a two terminal branch.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sequence_number: Option<i64>,
}
impl crate::base::CimElement for ACDCTerminal {
    fn mrid(&self) -> &str { &self.base.id }
    fn type_name(&self) -> &'static str { "ACDCTerminal" }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn to_json_value(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or(serde_json::Value::Null)
    }
    fn to_block(&self) -> crate::base::RdfBlock {
        let mut block = self.base.to_block();
        block.type_name = "ACDCTerminal".to_string();
        if let Some(ref v) = self.bus_name_marker {
            block.fields.insert("ACDCTerminal.BusNameMarker".into(), crate::base::FieldValue::Resource(v.mrid.clone()));
        }
        if let Some(v) = self.connected {
            block.fields.insert("ACDCTerminal.connected".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.sequence_number {
            block.fields.insert("ACDCTerminal.sequenceNumber".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        block
    }
}

impl ACDCTerminal {
    pub fn from_block(b: &crate::base::RdfBlock) -> Self {
        let mut obj = Self::default();
        obj.base.id.clone_from(&b.mrid);
        for (key, val) in &b.fields {
            match key.as_str() {
                "ACDCTerminal.BusNameMarker" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.bus_name_marker = Some(crate::base::MridRef { mrid: sv.clone() });
                    }
                }
                "ACDCTerminal.connected" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.connected = Some(sv.trim() == "true"); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.connected = Some(sv.trim() == "true"); }
                        }
                        _ => {}
                    }
                }
                "ACDCTerminal.sequenceNumber" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.sequence_number = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.sequence_number = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "IdentifiedObject.description" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.base.description = sv.clone(); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.base.description = sv.clone(); }
                        }
                        _ => {}
                    }
                }
                "IdentifiedObject.energyIdentCodeEic" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.base.energy_ident_code_eic = sv.clone(); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.base.energy_ident_code_eic = sv.clone(); }
                        }
                        _ => {}
                    }
                }
                "IdentifiedObject.mRID" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.base.m_rid = sv.clone(); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.base.m_rid = sv.clone(); }
                        }
                        _ => {}
                    }
                }
                "IdentifiedObject.name" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.base.name = sv.clone(); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.base.name = sv.clone(); }
                        }
                        _ => {}
                    }
                }
                "IdentifiedObject.shortName" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.base.short_name = sv.clone(); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.base.short_name = sv.clone(); }
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
