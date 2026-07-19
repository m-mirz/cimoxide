/// Describes a tap changer with a table defining the relation between the tap step and the phase angle difference across the transformer.
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct PhaseTapChangerTabular {
    #[serde(flatten)]
    pub base: super::PhaseTapChanger,
    /// The phase tap changer table for this phase tap changer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phase_tap_changer_table: Option<super::base::MridRef>,
}
impl crate::base::CimElement for PhaseTapChangerTabular {
    fn mrid(&self) -> &str { &self.base.base.base.base.id }
    fn type_name(&self) -> &'static str { "PhaseTapChangerTabular" }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn to_json_value(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or(serde_json::Value::Null)
    }
    fn to_block(&self) -> crate::base::RdfBlock {
        let mut block = self.base.to_block();
        block.type_name = "PhaseTapChangerTabular".to_string();
        if let Some(ref v) = self.phase_tap_changer_table {
            block.fields.insert("PhaseTapChangerTabular.PhaseTapChangerTable".into(), crate::base::FieldValue::Resource(v.mrid.clone()));
        }
        block
    }
}

impl PhaseTapChangerTabular {
    pub fn from_block(b: &crate::base::RdfBlock) -> Self {
        let mut obj = Self::default();
        obj.base.base.base.base.id.clone_from(&b.mrid);
        for (key, val) in &b.fields {
            match key.as_str() {
                "PhaseTapChangerTabular.PhaseTapChangerTable" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.phase_tap_changer_table = Some(crate::base::MridRef { mrid: sv.clone() });
                    }
                }
                "PhaseTapChanger.TransformerEnd" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.base.transformer_end = Some(crate::base::MridRef { mrid: sv.clone() });
                    }
                }
                "TapChanger.TapChangerControl" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.base.base.tap_changer_control = Some(crate::base::MridRef { mrid: sv.clone() });
                    }
                }
                "TapChanger.controlEnabled" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.base.base.control_enabled = Some(sv.trim() == "true"); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.base.base.control_enabled = Some(sv.trim() == "true"); }
                        }
                        _ => {}
                    }
                }
                "TapChanger.highStep" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.base.base.high_step = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.base.base.high_step = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "TapChanger.lowStep" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.base.base.low_step = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.base.base.low_step = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "TapChanger.ltcFlag" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.base.base.ltc_flag = Some(sv.trim() == "true"); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.base.base.ltc_flag = Some(sv.trim() == "true"); }
                        }
                        _ => {}
                    }
                }
                "TapChanger.neutralStep" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.base.base.neutral_step = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.base.base.neutral_step = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "TapChanger.neutralU" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.base.base.neutral_u = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.base.base.neutral_u = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "TapChanger.normalStep" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.base.base.normal_step = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.base.base.normal_step = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "TapChanger.step" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.base.base.step = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.base.base.step = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "IdentifiedObject.description" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.base.base.base.base.description = sv.clone(); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.base.base.base.base.description = sv.clone(); }
                        }
                        _ => {}
                    }
                }
                "IdentifiedObject.energyIdentCodeEic" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.base.base.base.base.energy_ident_code_eic = sv.clone(); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.base.base.base.base.energy_ident_code_eic = sv.clone(); }
                        }
                        _ => {}
                    }
                }
                "IdentifiedObject.mRID" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.base.base.base.base.m_rid = sv.clone(); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.base.base.base.base.m_rid = sv.clone(); }
                        }
                        _ => {}
                    }
                }
                "IdentifiedObject.name" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.base.base.base.base.name = sv.clone(); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.base.base.base.base.name = sv.clone(); }
                        }
                        _ => {}
                    }
                }
                "IdentifiedObject.shortName" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.base.base.base.base.short_name = sv.clone(); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.base.base.base.base.short_name = sv.clone(); }
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
