/// An electrically connected subset of the network. DC topological islands can change as the current network state changes, e.g. due to: - disconnect switches or breakers changing state in a SCADA/EMS. - manual creation, change or deletion of topological nodes in a planning tool. Only energised TopologicalNode-s shall be part of the topological island.
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct DCTopologicalIsland {
    #[serde(flatten)]
    pub base: super::IdentifiedObject,
    /// The DC topological nodes in a DC topological island.
    pub dc_topological_nodes: Vec<super::base::MridRef>,
}
impl crate::base::CimElement for DCTopologicalIsland {
    fn mrid(&self) -> &str { &self.base.id }
    fn type_name(&self) -> &'static str { "DCTopologicalIsland" }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn to_json_value(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or(serde_json::Value::Null)
    }
    fn to_block(&self) -> crate::base::RdfBlock {
        let mut block = self.base.to_block();
        block.type_name = "DCTopologicalIsland".to_string();
        if !self.dc_topological_nodes.is_empty() {
            block.fields.insert("DCTopologicalIsland.DCTopologicalNodes".into(), crate::base::FieldValue::ResourceList(self.dc_topological_nodes.iter().map(|r| r.mrid.clone()).collect()));
        }
        block
    }
}

impl DCTopologicalIsland {
    pub fn from_block(b: &crate::base::RdfBlock) -> Self {
        let mut obj = Self::default();
        obj.base.id.clone_from(&b.mrid);
        for (key, val) in &b.fields {
            match key.as_str() {
                "DCTopologicalIsland.DCTopologicalNodes" => {
                    match val {
                        crate::base::FieldValue::Resource(sv) => obj.dc_topological_nodes.push(crate::base::MridRef { mrid: sv.clone() }),
                        crate::base::FieldValue::ResourceList(svs) => {
                            for sv in svs { obj.dc_topological_nodes.push(crate::base::MridRef { mrid: sv.clone() }); }
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
