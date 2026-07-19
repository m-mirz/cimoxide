/// It represents the difference model header. The content is described by the Model class, the association role forwardDifferences and association role reverseDifferences. Both association roles may have one set of Statements.
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct DifferenceModel {
    #[serde(flatten)]
    pub base: super::Model,
    /// A property of the difference model whose value is a collection of statements (i.e., resources of type rdf:Statement) representing the forward difference statements.
    pub forward_differences: Vec<super::base::MridRef>,
    /// A property of the difference model whose value is the collection of precondition statements.
    pub preconditions: Vec<super::base::MridRef>,
    /// A property of the difference model whose value is the collection of reverse difference statements.
    pub reverse_differences: Vec<super::base::MridRef>,
}
impl crate::base::CimElement for DifferenceModel {
    fn mrid(&self) -> &str { &self.base.id }
    fn type_name(&self) -> &'static str { "DifferenceModel" }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn to_json_value(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or(serde_json::Value::Null)
    }
    fn to_block(&self) -> crate::base::RdfBlock {
        let mut block = self.base.to_block();
        block.type_name = "DifferenceModel".to_string();
        if !self.forward_differences.is_empty() {
            block.fields.insert("DifferenceModel.forwardDifferences".into(), crate::base::FieldValue::ResourceList(self.forward_differences.iter().map(|r| r.mrid.clone()).collect()));
        }
        if !self.preconditions.is_empty() {
            block.fields.insert("DifferenceModel.preconditions".into(), crate::base::FieldValue::ResourceList(self.preconditions.iter().map(|r| r.mrid.clone()).collect()));
        }
        if !self.reverse_differences.is_empty() {
            block.fields.insert("DifferenceModel.reverseDifferences".into(), crate::base::FieldValue::ResourceList(self.reverse_differences.iter().map(|r| r.mrid.clone()).collect()));
        }
        block
    }
}

impl DifferenceModel {
    pub fn from_block(b: &crate::base::RdfBlock) -> Self {
        let mut obj = Self::default();
        obj.base.id.clone_from(&b.mrid);
        for (key, val) in &b.fields {
            match key.as_str() {
                "DifferenceModel.forwardDifferences" => {
                    match val {
                        crate::base::FieldValue::Resource(sv) => obj.forward_differences.push(crate::base::MridRef { mrid: sv.clone() }),
                        crate::base::FieldValue::ResourceList(svs) => {
                            for sv in svs { obj.forward_differences.push(crate::base::MridRef { mrid: sv.clone() }); }
                        }
                        _ => {}
                    }
                }
                "DifferenceModel.preconditions" => {
                    match val {
                        crate::base::FieldValue::Resource(sv) => obj.preconditions.push(crate::base::MridRef { mrid: sv.clone() }),
                        crate::base::FieldValue::ResourceList(svs) => {
                            for sv in svs { obj.preconditions.push(crate::base::MridRef { mrid: sv.clone() }); }
                        }
                        _ => {}
                    }
                }
                "DifferenceModel.reverseDifferences" => {
                    match val {
                        crate::base::FieldValue::Resource(sv) => obj.reverse_differences.push(crate::base::MridRef { mrid: sv.clone() }),
                        crate::base::FieldValue::ResourceList(svs) => {
                            for sv in svs { obj.reverse_differences.push(crate::base::MridRef { mrid: sv.clone() }); }
                        }
                        _ => {}
                    }
                }
                "Model.DependentOn" => {
                    match val {
                        crate::base::FieldValue::Resource(sv) => obj.base.dependent_on.push(crate::base::MridRef { mrid: sv.clone() }),
                        crate::base::FieldValue::ResourceList(svs) => {
                            for sv in svs { obj.base.dependent_on.push(crate::base::MridRef { mrid: sv.clone() }); }
                        }
                        _ => {}
                    }
                }
                "Model.Supersedes" => {
                    match val {
                        crate::base::FieldValue::Resource(sv) => obj.base.supersedes.push(crate::base::MridRef { mrid: sv.clone() }),
                        crate::base::FieldValue::ResourceList(svs) => {
                            for sv in svs { obj.base.supersedes.push(crate::base::MridRef { mrid: sv.clone() }); }
                        }
                        _ => {}
                    }
                }
                "Model.created" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.base.created = sv.clone(); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.base.created = sv.clone(); }
                        }
                        _ => {}
                    }
                }
                "Model.description" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.base.description = sv.clone(); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.base.description = sv.clone(); }
                        }
                        _ => {}
                    }
                }
                "Model.modelingAuthoritySet" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.base.modeling_authority_set = sv.clone(); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.base.modeling_authority_set = sv.clone(); }
                        }
                        _ => {}
                    }
                }
                "Model.profile" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.base.profile.push(sv.trim().to_string()); }
                        crate::base::FieldValue::TextList(svs) => {
                            for sv in svs { obj.base.profile.push(sv.trim().to_string()); }
                        }
                        _ => {}
                    }
                }
                "Model.scenarioTime" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.base.scenario_time = sv.clone(); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.base.scenario_time = sv.clone(); }
                        }
                        _ => {}
                    }
                }
                "Model.version" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.base.version = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.base.version = Some(v); } }
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
