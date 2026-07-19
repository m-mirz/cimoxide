/// For a detailed substation model a topological node is a set of connectivity nodes that, in the current network state, are connected together through any type of closed switches, including jumpers. Topological nodes change as the current network state changes (i.e., switches, breakers, etc. change state). For a planning model, switch statuses are not used to form topological nodes. Instead they are manually created or deleted in a model builder tool. Topological nodes maintained this way are also called 'busses'.
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct TopologicalNode {
    #[serde(flatten)]
    pub base: super::IdentifiedObject,
    /// The base voltage of the topological node.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_voltage: Option<super::base::MridRef>,
    /// The connectivity node container to which the topological node belongs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connectivity_node_container: Option<super::base::MridRef>,
    /// The reporting group to which the topological node belongs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reporting_group: Option<super::base::MridRef>,
}
impl crate::base::CimElement for TopologicalNode {
    fn mrid(&self) -> &str { &self.base.id }
    fn type_name(&self) -> &'static str { "TopologicalNode" }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn to_json_value(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or(serde_json::Value::Null)
    }
    fn to_block(&self) -> crate::base::RdfBlock {
        let mut block = self.base.to_block();
        block.type_name = "TopologicalNode".to_string();
        if let Some(ref v) = self.base_voltage {
            block.fields.insert("TopologicalNode.BaseVoltage".into(), crate::base::FieldValue::Resource(v.mrid.clone()));
        }
        if let Some(ref v) = self.connectivity_node_container {
            block.fields.insert("TopologicalNode.ConnectivityNodeContainer".into(), crate::base::FieldValue::Resource(v.mrid.clone()));
        }
        if let Some(ref v) = self.reporting_group {
            block.fields.insert("TopologicalNode.ReportingGroup".into(), crate::base::FieldValue::Resource(v.mrid.clone()));
        }
        block
    }
}

impl TopologicalNode {
    pub fn from_block(b: &crate::base::RdfBlock) -> Self {
        let mut obj = Self::default();
        obj.base.id.clone_from(&b.mrid);
        for (key, val) in &b.fields {
            match key.as_str() {
                "TopologicalNode.BaseVoltage" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.base_voltage = Some(crate::base::MridRef { mrid: sv.clone() });
                    }
                }
                "TopologicalNode.ConnectivityNodeContainer" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.connectivity_node_container = Some(crate::base::MridRef { mrid: sv.clone() });
                    }
                }
                "TopologicalNode.ReportingGroup" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.reporting_group = Some(crate::base::MridRef { mrid: sv.clone() });
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
