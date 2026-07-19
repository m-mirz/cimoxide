/// IEEE PF controller type 1 which operates by moving the voltage reference directly. Reference: IEEE 421.5-2005, 11.2.
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct PFVArType1IEEEPFController {
    #[serde(flatten)]
    pub base: super::PFVArControllerType1Dynamics,
    /// Overexcitation Flag (OVEX) true = overexcited false = underexcited.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ovex: Option<bool>,
    /// PF controller time delay (TPFC) (>= 0). Typical value = 5.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tpfc: Option<f64>,
    /// Minimum machine terminal current needed to enable pf/var controller (VITMIN).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vitmin: Option<f64>,
    /// Synchronous machine power factor (VPF).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpf: Option<f64>,
    /// PF controller deadband (VPFC_BW). Typical value = 0,05.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpfcbw: Option<f64>,
    /// PF controller reference (VPFREF).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpfref: Option<f64>,
    /// Maximum machine terminal voltage needed for pf/var controller to be enabled (VVTMAX) (> PFVArType1IEEEPFController.vvtmin).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vvtmax: Option<f64>,
    /// Minimum machine terminal voltage needed to enable pf/var controller (VVTMIN) (< PFVArType1IEEEPFController.vvtmax).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vvtmin: Option<f64>,
}
impl crate::base::CimElement for PFVArType1IEEEPFController {
    fn mrid(&self) -> &str { &self.base.base.base.id }
    fn type_name(&self) -> &'static str { "PFVArType1IEEEPFController" }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn to_json_value(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or(serde_json::Value::Null)
    }
    fn to_block(&self) -> crate::base::RdfBlock {
        let mut block = self.base.to_block();
        block.type_name = "PFVArType1IEEEPFController".to_string();
        if let Some(v) = self.ovex {
            block.fields.insert("PFVArType1IEEEPFController.ovex".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.tpfc {
            block.fields.insert("PFVArType1IEEEPFController.tpfc".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.vitmin {
            block.fields.insert("PFVArType1IEEEPFController.vitmin".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.vpf {
            block.fields.insert("PFVArType1IEEEPFController.vpf".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.vpfcbw {
            block.fields.insert("PFVArType1IEEEPFController.vpfcbw".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.vpfref {
            block.fields.insert("PFVArType1IEEEPFController.vpfref".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.vvtmax {
            block.fields.insert("PFVArType1IEEEPFController.vvtmax".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.vvtmin {
            block.fields.insert("PFVArType1IEEEPFController.vvtmin".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        block
    }
}

impl PFVArType1IEEEPFController {
    pub fn from_block(b: &crate::base::RdfBlock) -> Self {
        let mut obj = Self::default();
        obj.base.base.base.id.clone_from(&b.mrid);
        for (key, val) in &b.fields {
            match key.as_str() {
                "PFVArType1IEEEPFController.ovex" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.ovex = Some(sv.trim() == "true"); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.ovex = Some(sv.trim() == "true"); }
                        }
                        _ => {}
                    }
                }
                "PFVArType1IEEEPFController.tpfc" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.tpfc = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.tpfc = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "PFVArType1IEEEPFController.vitmin" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.vitmin = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.vitmin = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "PFVArType1IEEEPFController.vpf" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.vpf = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.vpf = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "PFVArType1IEEEPFController.vpfcbw" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.vpfcbw = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.vpfcbw = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "PFVArType1IEEEPFController.vpfref" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.vpfref = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.vpfref = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "PFVArType1IEEEPFController.vvtmax" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.vvtmax = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.vvtmax = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "PFVArType1IEEEPFController.vvtmin" => {
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
