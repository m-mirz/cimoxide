/// IEEE type DEC2A model for discontinuous excitation control. This system provides transient excitation boosting via an open-loop control as initiated by a trigger signal generated remotely. Reference: IEEE 421.5-2005 12.3.
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct DiscExcContIEEEDEC2A {
    #[serde(flatten)]
    pub base: super::DiscontinuousExcitationControlDynamics,
    /// Discontinuous controller time constant (TD1) (>= 0).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub td1: Option<f64>,
    /// Discontinuous controller washout time constant (TD2) (>= 0).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub td2: Option<f64>,
    /// Limiter (VDMAX) (> DiscExcContIEEEDEC2A.vdmin).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vdmax: Option<f64>,
    /// Limiter (VDMIN) (< DiscExcContIEEEDEC2A.vdmax).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vdmin: Option<f64>,
    /// Discontinuous controller input reference (VK).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vk: Option<f64>,
}
impl crate::base::CimElement for DiscExcContIEEEDEC2A {
    fn mrid(&self) -> &str { &self.base.base.base.id }
    fn type_name(&self) -> &'static str { "DiscExcContIEEEDEC2A" }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn to_json_value(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or(serde_json::Value::Null)
    }
    fn to_block(&self) -> crate::base::RdfBlock {
        let mut block = self.base.to_block();
        block.type_name = "DiscExcContIEEEDEC2A".to_string();
        if let Some(v) = self.td1 {
            block.fields.insert("DiscExcContIEEEDEC2A.td1".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.td2 {
            block.fields.insert("DiscExcContIEEEDEC2A.td2".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.vdmax {
            block.fields.insert("DiscExcContIEEEDEC2A.vdmax".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.vdmin {
            block.fields.insert("DiscExcContIEEEDEC2A.vdmin".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.vk {
            block.fields.insert("DiscExcContIEEEDEC2A.vk".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        block
    }
}

impl DiscExcContIEEEDEC2A {
    pub fn from_block(b: &crate::base::RdfBlock) -> Self {
        let mut obj = Self::default();
        obj.base.base.base.id.clone_from(&b.mrid);
        for (key, val) in &b.fields {
            match key.as_str() {
                "DiscExcContIEEEDEC2A.td1" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.td1 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.td1 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "DiscExcContIEEEDEC2A.td2" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.td2 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.td2 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "DiscExcContIEEEDEC2A.vdmax" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.vdmax = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.vdmax = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "DiscExcContIEEEDEC2A.vdmin" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.vdmin = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.vdmin = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "DiscExcContIEEEDEC2A.vk" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.vk = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.vk = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "DiscontinuousExcitationControlDynamics.ExcitationSystemDynamics" => {
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
