/// Mechanical load function block whose behaviour is described by reference to a standard model or by definition of a user-defined model.
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct MechanicalLoadDynamics {
    #[serde(flatten)]
    pub base: super::DynamicsFunctionBlock,
    /// Asynchronous machine model with which this mechanical load model is associated. MechanicalLoadDynamics shall have either an association to SynchronousMachineDynamics or to AsynchronousMachineDynamics.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asynchronous_machine_dynamics: Option<super::base::MridRef>,
    /// Synchronous machine model with which this mechanical load model is associated. MechanicalLoadDynamics shall have either an association to SynchronousMachineDynamics or AsynchronousMachineDyanmics.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub synchronous_machine_dynamics: Option<super::base::MridRef>,
}
impl crate::base::CimElement for MechanicalLoadDynamics {
    fn mrid(&self) -> &str { &self.base.base.id }
    fn type_name(&self) -> &'static str { "MechanicalLoadDynamics" }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn to_json_value(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or(serde_json::Value::Null)
    }
    fn to_block(&self) -> crate::base::RdfBlock {
        let mut block = self.base.to_block();
        block.type_name = "MechanicalLoadDynamics".to_string();
        if let Some(ref v) = self.asynchronous_machine_dynamics {
            block.fields.insert("MechanicalLoadDynamics.AsynchronousMachineDynamics".into(), crate::base::FieldValue::Resource(v.mrid.clone()));
        }
        if let Some(ref v) = self.synchronous_machine_dynamics {
            block.fields.insert("MechanicalLoadDynamics.SynchronousMachineDynamics".into(), crate::base::FieldValue::Resource(v.mrid.clone()));
        }
        block
    }
}

impl MechanicalLoadDynamics {
    pub fn from_block(b: &crate::base::RdfBlock) -> Self {
        let mut obj = Self::default();
        obj.base.base.id.clone_from(&b.mrid);
        for (key, val) in &b.fields {
            match key.as_str() {
                "MechanicalLoadDynamics.AsynchronousMachineDynamics" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.asynchronous_machine_dynamics = Some(crate::base::MridRef { mrid: sv.clone() });
                    }
                }
                "MechanicalLoadDynamics.SynchronousMachineDynamics" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.synchronous_machine_dynamics = Some(crate::base::MridRef { mrid: sv.clone() });
                    }
                }
                "DynamicsFunctionBlock.enabled" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.base.enabled = Some(sv.trim() == "true"); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.base.enabled = Some(sv.trim() == "true"); }
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
