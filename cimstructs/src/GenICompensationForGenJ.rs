/// Resistive and reactive components of compensation for generator associated with IEEE type 2 voltage compensator for current flow out of another generator in the interconnection.
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct GenICompensationForGenJ {
    #[serde(flatten)]
    pub base: super::IdentifiedObject,
    /// Standard synchronous machine out of which current flow is being compensated for.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub synchronous_machine_dynamics: Option<super::base::MridRef>,
    /// The standard IEEE type 2 voltage compensator of this compensation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vcomp_ieee_type2: Option<super::base::MridRef>,
    /// Resistive component of compensation of generator associated with this IEEE type 2 voltage compensator for current flow out of another generator (Rcij).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rcij: Option<f64>,
    /// Reactive component of compensation of generator associated with this IEEE type 2 voltage compensator for current flow out of another generator (Xcij).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub xcij: Option<f64>,
}
impl crate::base::CimElement for GenICompensationForGenJ {
    fn mrid(&self) -> &str { &self.base.id }
    fn type_name(&self) -> &'static str { "GenICompensationForGenJ" }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn to_json_value(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or(serde_json::Value::Null)
    }
    fn to_block(&self) -> crate::base::RdfBlock {
        let mut block = self.base.to_block();
        block.type_name = "GenICompensationForGenJ".to_string();
        if let Some(ref v) = self.synchronous_machine_dynamics {
            block.fields.insert("GenICompensationForGenJ.SynchronousMachineDynamics".into(), crate::base::FieldValue::Resource(v.mrid.clone()));
        }
        if let Some(ref v) = self.vcomp_ieee_type2 {
            block.fields.insert("GenICompensationForGenJ.VcompIEEEType2".into(), crate::base::FieldValue::Resource(v.mrid.clone()));
        }
        if let Some(v) = self.rcij {
            block.fields.insert("GenICompensationForGenJ.rcij".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.xcij {
            block.fields.insert("GenICompensationForGenJ.xcij".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        block
    }
}

impl GenICompensationForGenJ {
    pub fn from_block(b: &crate::base::RdfBlock) -> Self {
        let mut obj = Self::default();
        obj.base.id.clone_from(&b.mrid);
        for (key, val) in &b.fields {
            match key.as_str() {
                "GenICompensationForGenJ.SynchronousMachineDynamics" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.synchronous_machine_dynamics = Some(crate::base::MridRef { mrid: sv.clone() });
                    }
                }
                "GenICompensationForGenJ.VcompIEEEType2" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.vcomp_ieee_type2 = Some(crate::base::MridRef { mrid: sv.clone() });
                    }
                }
                "GenICompensationForGenJ.rcij" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.rcij = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.rcij = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GenICompensationForGenJ.xcij" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.xcij = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.xcij = Some(v); } }
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
