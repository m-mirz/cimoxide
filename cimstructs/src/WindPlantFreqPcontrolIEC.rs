/// Frequency and active power controller model. Reference: IEC 61400-27-1:2015, Annex D.
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct WindPlantFreqPcontrolIEC {
    #[serde(flatten)]
    pub base: super::IdentifiedObject,
    /// Maximum ramp rate of pWTref request from the plant controller to the wind turbines (dprefmax) (> WindPlantFreqPcontrolIEC.dprefmin). It is a case-dependent parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dprefmax: Option<f64>,
    /// Minimum (negative) ramp rate of pWTref request from the plant controller to the wind turbines (dprefmin) (< WindPlantFreqPcontrolIEC.dprefmax). It is a project-dependent parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dprefmin: Option<f64>,
    /// Maximum positive ramp rate for wind plant power reference (dpWPrefmax) (> WindPlantFreqPcontrolIEC.dpwprefmin). It is a project-dependent parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dpwprefmax: Option<f64>,
    /// Maximum negative ramp rate for wind plant power reference (dpWPrefmin) (< WindPlantFreqPcontrolIEC.dpwprefmax). It is a project-dependent parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dpwprefmin: Option<f64>,
    /// Plant P controller integral gain (KIWPp). It is a project-dependent parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kiwpp: Option<f64>,
    /// Maximum PI integrator term (KIWPpmax) (> WindPlantFreqPcontrolIEC.kiwppmin). It is a project-dependent parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kiwppmax: Option<f64>,
    /// Minimum PI integrator term (KIWPpmin) (< WindPlantFreqPcontrolIEC.kiwppmax). It is a project-dependent parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kiwppmin: Option<f64>,
    /// Plant P controller proportional gain (KPWPp). It is a project-dependent parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kpwpp: Option<f64>,
    /// Power reference gain (KWPpref). It is a project-dependent parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kwppref: Option<f64>,
    /// Maximum pWTref request from the plant controller to the wind turbines (prefmax) (> WindPlantFreqPcontrolIEC.prefmin). It is a project-dependent parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefmax: Option<f64>,
    /// Minimum pWTref request from the plant controller to the wind turbines (prefmin) (< WindPlantFreqPcontrolIEC.prefmax). It is a project-dependent parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefmin: Option<f64>,
    /// Lead time constant in reference value transfer function (Tpft) (>= 0). It is a project-dependent parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tpft: Option<f64>,
    /// Lag time constant in reference value transfer function (Tpfv) (>= 0). It is a project-dependent parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tpfv: Option<f64>,
    /// Filter time constant for frequency measurement (TWPffiltp) (>= 0). It is a project-dependent parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub twpffiltp: Option<f64>,
    /// Filter time constant for active power measurement (TWPpfiltp) (>= 0). It is a project-dependent parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub twppfiltp: Option<f64>,
}
impl crate::base::CimElement for WindPlantFreqPcontrolIEC {
    fn mrid(&self) -> &str { &self.base.id }
    fn type_name(&self) -> &'static str { "WindPlantFreqPcontrolIEC" }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn to_json_value(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or(serde_json::Value::Null)
    }
    fn to_block(&self) -> crate::base::RdfBlock {
        let mut block = self.base.to_block();
        block.type_name = "WindPlantFreqPcontrolIEC".to_string();
        if let Some(v) = self.dprefmax {
            block.fields.insert("WindPlantFreqPcontrolIEC.dprefmax".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.dprefmin {
            block.fields.insert("WindPlantFreqPcontrolIEC.dprefmin".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.dpwprefmax {
            block.fields.insert("WindPlantFreqPcontrolIEC.dpwprefmax".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.dpwprefmin {
            block.fields.insert("WindPlantFreqPcontrolIEC.dpwprefmin".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.kiwpp {
            block.fields.insert("WindPlantFreqPcontrolIEC.kiwpp".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.kiwppmax {
            block.fields.insert("WindPlantFreqPcontrolIEC.kiwppmax".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.kiwppmin {
            block.fields.insert("WindPlantFreqPcontrolIEC.kiwppmin".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.kpwpp {
            block.fields.insert("WindPlantFreqPcontrolIEC.kpwpp".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.kwppref {
            block.fields.insert("WindPlantFreqPcontrolIEC.kwppref".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.prefmax {
            block.fields.insert("WindPlantFreqPcontrolIEC.prefmax".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.prefmin {
            block.fields.insert("WindPlantFreqPcontrolIEC.prefmin".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.tpft {
            block.fields.insert("WindPlantFreqPcontrolIEC.tpft".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.tpfv {
            block.fields.insert("WindPlantFreqPcontrolIEC.tpfv".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.twpffiltp {
            block.fields.insert("WindPlantFreqPcontrolIEC.twpffiltp".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.twppfiltp {
            block.fields.insert("WindPlantFreqPcontrolIEC.twppfiltp".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        block
    }
}

impl WindPlantFreqPcontrolIEC {
    pub fn from_block(b: &crate::base::RdfBlock) -> Self {
        let mut obj = Self::default();
        obj.base.id.clone_from(&b.mrid);
        for (key, val) in &b.fields {
            match key.as_str() {
                "WindPlantFreqPcontrolIEC.dprefmax" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.dprefmax = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.dprefmax = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "WindPlantFreqPcontrolIEC.dprefmin" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.dprefmin = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.dprefmin = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "WindPlantFreqPcontrolIEC.dpwprefmax" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.dpwprefmax = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.dpwprefmax = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "WindPlantFreqPcontrolIEC.dpwprefmin" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.dpwprefmin = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.dpwprefmin = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "WindPlantFreqPcontrolIEC.kiwpp" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.kiwpp = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.kiwpp = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "WindPlantFreqPcontrolIEC.kiwppmax" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.kiwppmax = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.kiwppmax = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "WindPlantFreqPcontrolIEC.kiwppmin" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.kiwppmin = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.kiwppmin = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "WindPlantFreqPcontrolIEC.kpwpp" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.kpwpp = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.kpwpp = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "WindPlantFreqPcontrolIEC.kwppref" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.kwppref = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.kwppref = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "WindPlantFreqPcontrolIEC.prefmax" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.prefmax = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.prefmax = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "WindPlantFreqPcontrolIEC.prefmin" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.prefmin = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.prefmin = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "WindPlantFreqPcontrolIEC.tpft" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.tpft = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.tpft = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "WindPlantFreqPcontrolIEC.tpfv" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.tpfv = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.tpfv = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "WindPlantFreqPcontrolIEC.twpffiltp" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.twpffiltp = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.twpffiltp = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "WindPlantFreqPcontrolIEC.twppfiltp" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.twppfiltp = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.twppfiltp = Some(v); } }
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
