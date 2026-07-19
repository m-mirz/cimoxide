/// IEEE VAR controller type 1 which operates by moving the voltage reference directly. Reference: IEEE 421.5-2005, 11.3.
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct PFVArType1IEEEVArController {
    #[serde(flatten)]
    pub base: super::PFVArControllerType1Dynamics,
    /// Var controller time delay (TVARC) (>= 0). Typical value = 5.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tvarc: Option<f64>,
    /// Synchronous machine power factor (VVAR).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vvar: Option<f64>,
    /// Var controller deadband (VVARC_BW). Typical value = 0,02.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vvarcbw: Option<f64>,
    /// Var controller reference (VVARREF).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vvarref: Option<f64>,
    /// Maximum machine terminal voltage needed for pf/VAr controller to be enabled (VVTMAX) (> PVFArType1IEEEVArController.vvtmin).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vvtmax: Option<f64>,
    /// Minimum machine terminal voltage needed to enable pf/var controller (VVTMIN) (< PVFArType1IEEEVArController.vvtmax).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vvtmin: Option<f64>,
}
impl crate::base::CimElement for PFVArType1IEEEVArController {
    fn mrid(&self) -> &str { &self.base.base.base.id }
    fn type_name(&self) -> &'static str { "PFVArType1IEEEVArController" }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn to_json_value(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or(serde_json::Value::Null)
    }
    fn to_block(&self) -> crate::base::RdfBlock {
        let mut block = self.base.to_block();
        block.type_name = "PFVArType1IEEEVArController".to_string();
        if let Some(v) = self.tvarc {
            block.fields.insert("PFVArType1IEEEVArController.tvarc".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.vvar {
            block.fields.insert("PFVArType1IEEEVArController.vvar".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.vvarcbw {
            block.fields.insert("PFVArType1IEEEVArController.vvarcbw".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.vvarref {
            block.fields.insert("PFVArType1IEEEVArController.vvarref".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.vvtmax {
            block.fields.insert("PFVArType1IEEEVArController.vvtmax".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.vvtmin {
            block.fields.insert("PFVArType1IEEEVArController.vvtmin".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        block
    }
}

impl PFVArType1IEEEVArController {
    pub fn from_block(b: &crate::base::RdfBlock) -> Self {
        let mut obj = Self::default();
        obj.base.base.base.id.clone_from(&b.mrid);
        for (key, val) in &b.fields {
            match key.as_str() {
                "PFVArType1IEEEVArController.tvarc" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.tvarc = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.tvarc = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "PFVArType1IEEEVArController.vvar" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.vvar = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.vvar = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "PFVArType1IEEEVArController.vvarcbw" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.vvarcbw = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.vvarcbw = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "PFVArType1IEEEVArController.vvarref" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.vvarref = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.vvarref = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "PFVArType1IEEEVArController.vvtmax" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.vvtmax = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.vvtmax = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "PFVArType1IEEEVArController.vvtmin" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.vvtmin = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.vvtmin = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "PFVArControllerType1Dynamics.ExcitationSystemDynamics" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.base.excitation_system_dynamics = Some(crate::base::MridRef { mrid: sv.clone() });
                    }
                }
                "DynamicsFunctionBlock.enabled" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.base.base.enabled = Some(sv.trim() == "true"); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.base.base.enabled = Some(sv.trim() == "true"); }
                        }
                        _ => {}
                    }
                }
                "IdentifiedObject.description" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.base.base.base.description = sv.clone(); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.base.base.base.description = sv.clone(); }
                        }
                        _ => {}
                    }
                }
                "IdentifiedObject.energyIdentCodeEic" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.base.base.base.energy_ident_code_eic = sv.clone(); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.base.base.base.energy_ident_code_eic = sv.clone(); }
                        }
                        _ => {}
                    }
                }
                "IdentifiedObject.mRID" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.base.base.base.m_rid = sv.clone(); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.base.base.base.m_rid = sv.clone(); }
                        }
                        _ => {}
                    }
                }
                "IdentifiedObject.name" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.base.base.base.name = sv.clone(); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.base.base.base.name = sv.clone(); }
                        }
                        _ => {}
                    }
                }
                "IdentifiedObject.shortName" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.base.base.base.short_name = sv.clone(); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.base.base.base.short_name = sv.clone(); }
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
