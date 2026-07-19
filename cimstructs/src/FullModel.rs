/// It represents the full model header and its contents is described by the Model class.
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct FullModel {
    #[serde(flatten)]
    pub base: super::Model,
}
impl crate::base::CimElement for FullModel {
    fn mrid(&self) -> &str { &self.base.id }
    fn type_name(&self) -> &'static str { "FullModel" }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn to_json_value(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or(serde_json::Value::Null)
    }
    fn to_block(&self) -> crate::base::RdfBlock {
        let mut block = self.base.to_block();
        block.type_name = "FullModel".to_string();
        block
    }
}

impl FullModel {
    pub fn from_block(b: &crate::base::RdfBlock) -> Self {
        let mut obj = Self::default();
        obj.base.id.clone_from(&b.mrid);
        for (key, val) in &b.fields {
            match key.as_str() {
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
