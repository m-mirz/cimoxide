/// A Model is a collection of data describing instances, objects or entities, real or computed. In the context of CIM the semantics of the data is defined by profiles. Hence a model can contain equipment data, power flow initial values, power flow results etc. The Model class describes the header content that is the same for the FullModel and the DifferenceModel. A Model is identified by an rdf:about attribute. The rdf:about attribute uniquely describe the model data and not the CIMXML document. A new rdf:about identification is generated for created documents only when the model data has changed. A repeated creation of documents from unchanged model data shall have the same rdf:about identification as previous document generated from the same model data.
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct Model {
    pub id: String,
    /// A reference to the model documents that the model described by this document depends on. In general there can be 0 or many Model.DependentOn depending on the profile and the content of the instance file. For instance: - A load flow solution depends on the topology model it was computed from - A topology model computed by a topology processor depends on the network model it was computed from. The referenced models are identified by the FullModel rdf:about attribute for full model documents and by DifferenceModel rdf:about attribute for difference model documents. The references are maintained by the producer of the CIMXML document and the references are valid for the model with version and identifier for which the document was created.
    pub dependent_on: Vec<super::base::MridRef>,
    /// When a model is updated the resulting model supersedes the models that were used as basis for the update. Hence this is a reference to the CIMXML documents which are superseded by this model. A model (or instance file) can supersede 1 or more models, e.g. a difference model or a full model supersede multiple models (difference or full). In this case more than one Model.Supersedes are included in the header. The referenced document(s) is (are) identified by the URN/MRID/UUID in the FullModel rdf:about attribute when full model(s) is (are) referenced and by the URN/MRID/UUID in the DifferenceModel rdf:about attribute when difference model(s) is (are) referenced.
    pub supersedes: Vec<super::base::MridRef>,
    /// The date and time when the model was created. It is the time of the serialization. The format is an extended format according to the ISO 8601-2005. European exchanges shall refer to UTC, e.g. 2014-05-15T17:48:31.474Z.
    pub created: String,
    /// A description of the model, e.g. the name of person that created the model and for what purpose. The number of UTF-8 characters is limited to 2000.
    pub description: String,
    /// A URN/URI referring to the organisation role / model authority set reference. The organization role is the source of the model. It is the same for all profiles part of a model exchange.
    pub modeling_authority_set: String,
    /// URN/URI describing the profiles that governs this model. It uniquely identifies the profiles and its version, e.g. http://iec.ch/TC57/61970-456/SteadyStateHypothesis/2/0.
    pub profile: Vec<String>,
    /// The date and time that this model represents, i.e. for which the model is valid. The format is an extended format according to the ISO 8601-2005. European exchanges shall refer to UTC, e.g. 2030-01-15T17:00:00.000Z.
    pub scenario_time: String,
    /// The version of the model. If the instance file is imported and exported with no change the version number is the kept same. The version changes only if the content of the file changes. It is the same logic as for the header id. The version is the human readable id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<i64>,
}
impl crate::base::CimElement for Model {
    fn mrid(&self) -> &str { &self.id }
    fn type_name(&self) -> &'static str { "Model" }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn to_json_value(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or(serde_json::Value::Null)
    }
    fn to_block(&self) -> crate::base::RdfBlock {
        let mut block = crate::base::RdfBlock {
            type_name: "Model".to_string(),
            mrid: self.id.clone(),
            fields: std::collections::HashMap::new(),
            duplicate_fields: std::collections::HashSet::new(),
        };
        if !self.dependent_on.is_empty() {
            block.fields.insert("Model.DependentOn".into(), crate::base::FieldValue::ResourceList(self.dependent_on.iter().map(|r| r.mrid.clone()).collect()));
        }
        if !self.supersedes.is_empty() {
            block.fields.insert("Model.Supersedes".into(), crate::base::FieldValue::ResourceList(self.supersedes.iter().map(|r| r.mrid.clone()).collect()));
        }
        if !self.created.is_empty() {
            block.fields.insert("Model.created".into(), crate::base::FieldValue::Text(self.created.clone()));
        }
        if !self.description.is_empty() {
            block.fields.insert("Model.description".into(), crate::base::FieldValue::Text(self.description.clone()));
        }
        if !self.modeling_authority_set.is_empty() {
            block.fields.insert("Model.modelingAuthoritySet".into(), crate::base::FieldValue::Text(self.modeling_authority_set.clone()));
        }
        if !self.profile.is_empty() {
            block.fields.insert("Model.profile".into(), crate::base::FieldValue::TextList(self.profile.iter().map(|v| v.to_string()).collect()));
        }
        if !self.scenario_time.is_empty() {
            block.fields.insert("Model.scenarioTime".into(), crate::base::FieldValue::Text(self.scenario_time.clone()));
        }
        if let Some(v) = self.version {
            block.fields.insert("Model.version".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        block
    }
}

impl Model {
    pub fn from_block(b: &crate::base::RdfBlock) -> Self {
        let mut obj = Self::default();
        obj.id.clone_from(&b.mrid);
        for (key, val) in &b.fields {
            match key.as_str() {
                "Model.DependentOn" => {
                    match val {
                        crate::base::FieldValue::Resource(sv) => obj.dependent_on.push(crate::base::MridRef { mrid: sv.clone() }),
                        crate::base::FieldValue::ResourceList(svs) => {
                            for sv in svs { obj.dependent_on.push(crate::base::MridRef { mrid: sv.clone() }); }
                        }
                        _ => {}
                    }
                }
                "Model.Supersedes" => {
                    match val {
                        crate::base::FieldValue::Resource(sv) => obj.supersedes.push(crate::base::MridRef { mrid: sv.clone() }),
                        crate::base::FieldValue::ResourceList(svs) => {
                            for sv in svs { obj.supersedes.push(crate::base::MridRef { mrid: sv.clone() }); }
                        }
                        _ => {}
                    }
                }
                "Model.created" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.created = sv.clone(); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.created = sv.clone(); }
                        }
                        _ => {}
                    }
                }
                "Model.description" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.description = sv.clone(); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.description = sv.clone(); }
                        }
                        _ => {}
                    }
                }
                "Model.modelingAuthoritySet" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.modeling_authority_set = sv.clone(); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.modeling_authority_set = sv.clone(); }
                        }
                        _ => {}
                    }
                }
                "Model.profile" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.profile.push(sv.trim().to_string()); }
                        crate::base::FieldValue::TextList(svs) => {
                            for sv in svs { obj.profile.push(sv.trim().to_string()); }
                        }
                        _ => {}
                    }
                }
                "Model.scenarioTime" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.scenario_time = sv.clone(); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.scenario_time = sv.clone(); }
                        }
                        _ => {}
                    }
                }
                "Model.version" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.version = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.version = Some(v); } }
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
