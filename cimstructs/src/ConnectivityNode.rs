/// Connectivity nodes are points where terminals of AC conducting equipment are connected together with zero impedance.
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct ConnectivityNode {
    #[serde(flatten)]
    pub base: super::IdentifiedObject,
    /// Container of this connectivity node.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connectivity_node_container: Option<super::base::MridRef>,
    /// The topological node to which this connectivity node is assigned. May depend on the current state of switches in the network.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topological_node: Option<super::base::MridRef>,
}
impl crate::base::CimElement for ConnectivityNode {
    fn mrid(&self) -> &str { &self.base.id }
    fn type_name(&self) -> &'static str { "ConnectivityNode" }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn to_json_value(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or(serde_json::Value::Null)
    }
    fn to_block(&self) -> crate::base::RdfBlock {
        let mut block = self.base.to_block();
        block.type_name = "ConnectivityNode".to_string();
        if let Some(ref v) = self.connectivity_node_container {
            block.fields.insert("ConnectivityNode.ConnectivityNodeContainer".into(), crate::base::FieldValue::Resource(v.mrid.clone()));
        }
        if let Some(ref v) = self.topological_node {
            block.fields.insert("ConnectivityNode.TopologicalNode".into(), crate::base::FieldValue::Resource(v.mrid.clone()));
        }
        block
    }
}

impl ConnectivityNode {
    pub fn from_block(b: &crate::base::RdfBlock) -> Self {
        let mut obj = Self::default();
        obj.base.id.clone_from(&b.mrid);
        for (key, val) in &b.fields {
            match key.as_str() {
                "ConnectivityNode.ConnectivityNodeContainer" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.connectivity_node_container = Some(crate::base::MridRef { mrid: sv.clone() });
                    }
                }
                "ConnectivityNode.TopologicalNode" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.topological_node = Some(crate::base::MridRef { mrid: sv.clone() });
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
