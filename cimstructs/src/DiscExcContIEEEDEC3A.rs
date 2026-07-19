/// IEEE type DEC3A model. In some systems, the stabilizer output is disconnected from the regulator immediately following a severe fault to prevent the stabilizer from competing with action of voltage regulator during the first swing. Reference: IEEE 421.5-2005 12.4.
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct DiscExcContIEEEDEC3A {
    #[serde(flatten)]
    pub base: super::DiscontinuousExcitationControlDynamics,
    /// Reset time delay (TDR) (>= 0).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tdr: Option<f64>,
    /// Terminal undervoltage comparison level (VTMIN).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vtmin: Option<f64>,
}
impl crate::base::CimElement for DiscExcContIEEEDEC3A {
    fn mrid(&self) -> &str { &self.base.base.base.id }
    fn type_name(&self) -> &'static str { "DiscExcContIEEEDEC3A" }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn to_json_value(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or(serde_json::Value::Null)
    }
    fn to_block(&self) -> crate::base::RdfBlock {
        let mut block = self.base.to_block();
        block.type_name = "DiscExcContIEEEDEC3A".to_string();
        if let Some(v) = self.tdr {
            block.fields.insert("DiscExcContIEEEDEC3A.tdr".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.vtmin {
            block.fields.insert("DiscExcContIEEEDEC3A.vtmin".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        block
    }
}

impl DiscExcContIEEEDEC3A {
    pub fn from_block(b: &crate::base::RdfBlock) -> Self {
        let mut obj = Self::default();
        obj.base.base.base.id.clone_from(&b.mrid);
        for (key, val) in &b.fields {
            match key.as_str() {
                "DiscExcContIEEEDEC3A.tdr" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.tdr = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.tdr = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "DiscExcContIEEEDEC3A.vtmin" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.vtmin = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.vtmin = Some(v); } }
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
