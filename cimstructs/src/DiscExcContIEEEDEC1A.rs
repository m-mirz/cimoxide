/// IEEE type DEC1A discontinuous excitation control model that boosts generator excitation to a level higher than that demanded by the voltage regulator and stabilizer immediately following a system fault. Reference: IEEE 421.5-2005, 12.2.
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct DiscExcContIEEEDEC1A {
    #[serde(flatten)]
    pub base: super::DiscontinuousExcitationControlDynamics,
    /// Speed change reference (ESC). Typical value = 0,0015.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub esc: Option<f64>,
    /// Discontinuous controller gain (KAN). Typical value = 400.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kan: Option<f64>,
    /// Terminal voltage limiter gain (KETL). Typical value = 47.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ketl: Option<f64>,
    /// Discontinuous controller time constant (TAN) (>= 0). Typical value = 0,08.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tan: Option<f64>,
    /// Time constant (TD) (>= 0). Typical value = 0,03.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub td: Option<f64>,
    /// Time constant (TL1) (>= 0). Typical value = 0,025.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tl1: Option<f64>,
    /// Time constant (TL2) (>= 0). Typical value = 1,25.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tl2: Option<f64>,
    /// DEC washout time constant (TW5) (>= 0). Typical value = 5.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tw5: Option<f64>,
    /// Regulator voltage reference (VAL). Typical value = 5,5.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub val: Option<f64>,
    /// Limiter for Van (VANMAX).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vanmax: Option<f64>,
    /// Limiter (VOMAX) (> DiscExcContIEEEDEC1A.vomin). Typical value = 0,3.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vomax: Option<f64>,
    /// Limiter (VOMIN) (< DiscExcContIEEEDEC1A.vomax). Typical value = 0,1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vomin: Option<f64>,
    /// Limiter (VSMAX)(> DiscExcContIEEEDEC1A.vsmin). Typical value = 0,2.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vsmax: Option<f64>,
    /// Limiter (VSMIN) (< DiscExcContIEEEDEC1A.vsmax). Typical value = -0,066.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vsmin: Option<f64>,
    /// Terminal voltage level reference (VTC). Typical value = 0,95.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vtc: Option<f64>,
    /// Voltage reference (VTLMT). Typical value = 1,1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vtlmt: Option<f64>,
    /// Voltage limits (VTM). Typical value = 1,13.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vtm: Option<f64>,
    /// Voltage limits (VTN). Typical value = 1,12.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vtn: Option<f64>,
}
impl crate::base::CimElement for DiscExcContIEEEDEC1A {
    fn mrid(&self) -> &str { &self.base.base.base.id }
    fn type_name(&self) -> &'static str { "DiscExcContIEEEDEC1A" }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn to_json_value(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or(serde_json::Value::Null)
    }
    fn to_block(&self) -> crate::base::RdfBlock {
        let mut block = self.base.to_block();
        block.type_name = "DiscExcContIEEEDEC1A".to_string();
        if let Some(v) = self.esc {
            block.fields.insert("DiscExcContIEEEDEC1A.esc".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.kan {
            block.fields.insert("DiscExcContIEEEDEC1A.kan".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.ketl {
            block.fields.insert("DiscExcContIEEEDEC1A.ketl".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.tan {
            block.fields.insert("DiscExcContIEEEDEC1A.tan".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.td {
            block.fields.insert("DiscExcContIEEEDEC1A.td".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.tl1 {
            block.fields.insert("DiscExcContIEEEDEC1A.tl1".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.tl2 {
            block.fields.insert("DiscExcContIEEEDEC1A.tl2".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.tw5 {
            block.fields.insert("DiscExcContIEEEDEC1A.tw5".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.val {
            block.fields.insert("DiscExcContIEEEDEC1A.val".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.vanmax {
            block.fields.insert("DiscExcContIEEEDEC1A.vanmax".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.vomax {
            block.fields.insert("DiscExcContIEEEDEC1A.vomax".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.vomin {
            block.fields.insert("DiscExcContIEEEDEC1A.vomin".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.vsmax {
            block.fields.insert("DiscExcContIEEEDEC1A.vsmax".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.vsmin {
            block.fields.insert("DiscExcContIEEEDEC1A.vsmin".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.vtc {
            block.fields.insert("DiscExcContIEEEDEC1A.vtc".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.vtlmt {
            block.fields.insert("DiscExcContIEEEDEC1A.vtlmt".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.vtm {
            block.fields.insert("DiscExcContIEEEDEC1A.vtm".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.vtn {
            block.fields.insert("DiscExcContIEEEDEC1A.vtn".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        block
    }
}

impl DiscExcContIEEEDEC1A {
    pub fn from_block(b: &crate::base::RdfBlock) -> Self {
        let mut obj = Self::default();
        obj.base.base.base.id.clone_from(&b.mrid);
        for (key, val) in &b.fields {
            match key.as_str() {
                "DiscExcContIEEEDEC1A.esc" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.esc = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.esc = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "DiscExcContIEEEDEC1A.kan" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.kan = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.kan = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "DiscExcContIEEEDEC1A.ketl" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.ketl = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.ketl = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "DiscExcContIEEEDEC1A.tan" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.tan = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.tan = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "DiscExcContIEEEDEC1A.td" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.td = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.td = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "DiscExcContIEEEDEC1A.tl1" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.tl1 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.tl1 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "DiscExcContIEEEDEC1A.tl2" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.tl2 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.tl2 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "DiscExcContIEEEDEC1A.tw5" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.tw5 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.tw5 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "DiscExcContIEEEDEC1A.val" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.val = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.val = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "DiscExcContIEEEDEC1A.vanmax" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.vanmax = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.vanmax = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "DiscExcContIEEEDEC1A.vomax" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.vomax = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.vomax = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "DiscExcContIEEEDEC1A.vomin" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.vomin = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.vomin = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "DiscExcContIEEEDEC1A.vsmax" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.vsmax = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.vsmax = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "DiscExcContIEEEDEC1A.vsmin" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.vsmin = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.vsmin = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "DiscExcContIEEEDEC1A.vtc" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.vtc = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.vtc = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "DiscExcContIEEEDEC1A.vtlmt" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.vtlmt = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.vtlmt = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "DiscExcContIEEEDEC1A.vtm" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.vtm = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.vtm = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "DiscExcContIEEEDEC1A.vtn" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.vtn = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.vtn = Some(v); } }
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
