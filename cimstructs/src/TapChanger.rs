/// Mechanism for changing transformer winding tap positions.
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct TapChanger {
    #[serde(flatten)]
    pub base: super::PowerSystemResource,
    /// The regulating control scheme in which this tap changer participates.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tap_changer_control: Option<super::base::MridRef>,
    /// Specifies the regulation status of the equipment. True is regulating, false is not regulating.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub control_enabled: Option<bool>,
    /// Highest possible tap step position, advance from neutral. The attribute shall be greater than lowStep.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub high_step: Option<i64>,
    /// Lowest possible tap step position, retard from neutral.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub low_step: Option<i64>,
    /// Specifies whether or not a TapChanger has load tap changing capabilities.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ltc_flag: Option<bool>,
    /// The neutral tap step position for this winding. The attribute shall be equal to or greater than lowStep and equal or less than highStep. It is the step position where the voltage is neutralU when the other terminals of the transformer are at the ratedU. If there are other tap changers on the transformer those taps are kept constant at their neutralStep.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub neutral_step: Option<i64>,
    /// Voltage at which the winding operates at the neutral tap setting. It is the voltage at the terminal of the PowerTransformerEnd associated with the tap changer when all tap changers on the transformer are at their neutralStep position. Normally neutralU of the tap changer is the same as ratedU of the PowerTransformerEnd, but it can differ in special cases such as when the tapping mechanism is separate from the winding more common on lower voltage transformers. This attribute is not relevant for PhaseTapChangerAsymmetrical, PhaseTapChangerSymmetrical and PhaseTapChangerLinear.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub neutral_u: Option<f64>,
    /// The tap step position used in 'normal' network operation for this winding. For a 'Fixed' tap changer indicates the current physical tap setting. The attribute shall be equal to or greater than lowStep and equal to or less than highStep.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub normal_step: Option<i64>,
    /// Tap changer position. Starting step for a steady state solution. Non integer values are allowed to support continuous tap variables. The reasons for continuous value are to support study cases where no discrete tap changer has yet been designed, a solution where a narrow voltage band forces the tap step to oscillate or to accommodate for a continuous solution as input. The attribute shall be equal to or greater than lowStep and equal to or less than highStep.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub step: Option<f64>,
}
impl crate::base::CimElement for TapChanger {
    fn mrid(&self) -> &str { &self.base.base.id }
    fn type_name(&self) -> &'static str { "TapChanger" }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn to_json_value(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or(serde_json::Value::Null)
    }
    fn to_block(&self) -> crate::base::RdfBlock {
        let mut block = self.base.to_block();
        block.type_name = "TapChanger".to_string();
        if let Some(ref v) = self.tap_changer_control {
            block.fields.insert("TapChanger.TapChangerControl".into(), crate::base::FieldValue::Resource(v.mrid.clone()));
        }
        if let Some(v) = self.control_enabled {
            block.fields.insert("TapChanger.controlEnabled".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.high_step {
            block.fields.insert("TapChanger.highStep".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.low_step {
            block.fields.insert("TapChanger.lowStep".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.ltc_flag {
            block.fields.insert("TapChanger.ltcFlag".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.neutral_step {
            block.fields.insert("TapChanger.neutralStep".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.neutral_u {
            block.fields.insert("TapChanger.neutralU".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.normal_step {
            block.fields.insert("TapChanger.normalStep".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.step {
            block.fields.insert("TapChanger.step".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        block
    }
}

impl TapChanger {
    pub fn from_block(b: &crate::base::RdfBlock) -> Self {
        let mut obj = Self::default();
        obj.base.base.id.clone_from(&b.mrid);
        for (key, val) in &b.fields {
            match key.as_str() {
                "TapChanger.TapChangerControl" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.tap_changer_control = Some(crate::base::MridRef { mrid: sv.clone() });
                    }
                }
                "TapChanger.controlEnabled" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.control_enabled = Some(sv.trim() == "true"); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.control_enabled = Some(sv.trim() == "true"); }
                        }
                        _ => {}
                    }
                }
                "TapChanger.highStep" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.high_step = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.high_step = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "TapChanger.lowStep" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.low_step = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.low_step = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "TapChanger.ltcFlag" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.ltc_flag = Some(sv.trim() == "true"); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.ltc_flag = Some(sv.trim() == "true"); }
                        }
                        _ => {}
                    }
                }
                "TapChanger.neutralStep" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.neutral_step = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.neutral_step = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "TapChanger.neutralU" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.neutral_u = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.neutral_u = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "TapChanger.normalStep" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.normal_step = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.normal_step = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "TapChanger.step" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.step = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.step = Some(v); } }
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
