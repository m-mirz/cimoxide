/// Describes a tap changer with a linear relation between the tap step and the phase angle difference across the transformer. This is a mathematical model that is an approximation of a real phase tap changer. The phase angle is computed as stepPhaseShiftIncrement times the tap position. The voltage magnitude of both sides is the same.
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct PhaseTapChangerLinear {
    #[serde(flatten)]
    pub base: super::PhaseTapChanger,
    /// Phase shift per step position. A positive value indicates a positive angle variation from the Terminal at the PowerTransformerEnd, where the TapChanger is located, into the transformer. The actual phase shift increment might be more accurately computed from the symmetrical or asymmetrical models or a tap step table lookup if those are available.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub step_phase_shift_increment: Option<f64>,
    /// The reactance depends on the tap position according to a 'u' shaped curve. The maximum reactance (xMax) appears at the low and high tap positions. Depending on the “u” curve the attribute can be either higher or lower than PowerTransformerEnd.x.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x_max: Option<f64>,
    /// The reactance depends on the tap position according to a 'u' shaped curve. The minimum reactance (xMin) appears at the mid tap position. PowerTransformerEnd.x shall be consistent with PhaseTapChangerLinear.xMin and PhaseTapChangerNonLinear.xMin. In case of inconsistency, PowerTransformerEnd.x shall be used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x_min: Option<f64>,
}
impl crate::base::CimElement for PhaseTapChangerLinear {
    fn mrid(&self) -> &str { &self.base.base.base.base.id }
    fn type_name(&self) -> &'static str { "PhaseTapChangerLinear" }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn to_json_value(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or(serde_json::Value::Null)
    }
    fn to_block(&self) -> crate::base::RdfBlock {
        let mut block = self.base.to_block();
        block.type_name = "PhaseTapChangerLinear".to_string();
        if let Some(v) = self.step_phase_shift_increment {
            block.fields.insert("PhaseTapChangerLinear.stepPhaseShiftIncrement".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.x_max {
            block.fields.insert("PhaseTapChangerLinear.xMax".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.x_min {
            block.fields.insert("PhaseTapChangerLinear.xMin".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        block
    }
}

impl PhaseTapChangerLinear {
    pub fn from_block(b: &crate::base::RdfBlock) -> Self {
        let mut obj = Self::default();
        obj.base.base.base.base.id.clone_from(&b.mrid);
        for (key, val) in &b.fields {
            match key.as_str() {
                "PhaseTapChangerLinear.stepPhaseShiftIncrement" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.step_phase_shift_increment = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.step_phase_shift_increment = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "PhaseTapChangerLinear.xMax" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.x_max = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.x_max = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "PhaseTapChangerLinear.xMin" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.x_min = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.x_min = Some(v); } }
                        }
                        _ => {}
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
