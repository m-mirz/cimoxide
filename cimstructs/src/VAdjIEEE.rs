/// IEEE voltage adjuster which is used to represent the voltage adjuster in either a power factor or VAr control system. Reference: IEEE 421.5-2005, 11.1.
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct VAdjIEEE {
    #[serde(flatten)]
    pub base: super::VoltageAdjusterDynamics,
    /// Rate at which output of adjuster changes (ADJ_SLEW). Unit = s / PU. Typical value = 300.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub adjslew: Option<f64>,
    /// Time that adjuster pulses are off (TAOFF) (>= 0). Typical value = 0,5.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub taoff: Option<f64>,
    /// Time that adjuster pulses are on (TAON) (>= 0). Typical value = 0,1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub taon: Option<f64>,
    /// Set high to provide a continuous raise or lower (VADJF).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vadjf: Option<f64>,
    /// Maximum output of the adjuster (VADJMAX) (> VAdjIEEE.vadjmin). Typical value = 1,1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vadjmax: Option<f64>,
    /// Minimum output of the adjuster (VADJMIN) (< VAdjIEEE.vadjmax). Typical value = 0,9.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vadjmin: Option<f64>,
}
impl crate::base::CimElement for VAdjIEEE {
    fn mrid(&self) -> &str { &self.base.base.base.id }
    fn type_name(&self) -> &'static str { "VAdjIEEE" }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn to_json_value(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or(serde_json::Value::Null)
    }
    fn to_block(&self) -> crate::base::RdfBlock {
        let mut block = self.base.to_block();
        block.type_name = "VAdjIEEE".to_string();
        if let Some(v) = self.adjslew {
            block.fields.insert("VAdjIEEE.adjslew".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.taoff {
            block.fields.insert("VAdjIEEE.taoff".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.taon {
            block.fields.insert("VAdjIEEE.taon".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.vadjf {
            block.fields.insert("VAdjIEEE.vadjf".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.vadjmax {
            block.fields.insert("VAdjIEEE.vadjmax".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.vadjmin {
            block.fields.insert("VAdjIEEE.vadjmin".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        block
    }
}

impl VAdjIEEE {
    pub fn from_block(b: &crate::base::RdfBlock) -> Self {
        let mut obj = Self::default();
        obj.base.base.base.id.clone_from(&b.mrid);
        for (key, val) in &b.fields {
            match key.as_str() {
                "VAdjIEEE.adjslew" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.adjslew = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.adjslew = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "VAdjIEEE.taoff" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.taoff = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.taoff = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "VAdjIEEE.taon" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.taon = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.taon = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "VAdjIEEE.vadjf" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.vadjf = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.vadjf = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "VAdjIEEE.vadjmax" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.vadjmax = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.vadjmax = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "VAdjIEEE.vadjmin" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.vadjmin = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.vadjmin = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "VoltageAdjusterDynamics.PFVArControllerType1Dynamics" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.base.pfv_ar_controller_type1dynamics = Some(crate::base::MridRef { mrid: sv.clone() });
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
