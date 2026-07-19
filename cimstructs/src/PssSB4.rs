/// Power sensitive stabilizer model.
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct PssSB4 {
    #[serde(flatten)]
    pub base: super::PowerSystemStabilizerDynamics,
    /// Gain (Kx). Typical value = 2,7.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kx: Option<f64>,
    /// Time constant (Ta) (>= 0). Typical value = 0,37.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ta: Option<f64>,
    /// Time constant (Tb) (>= 0). Typical value = 0,37.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tb: Option<f64>,
    /// Time constant (Tc) (>= 0). Typical value = 0,035.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tc: Option<f64>,
    /// Time constant (Td) (>= 0). Typical value = 0,0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub td: Option<f64>,
    /// Time constant (Te) (>= 0). Typical value = 0,0169.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub te: Option<f64>,
    /// Time constant (Tt) (>= 0). Typical value = 0,18.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tt: Option<f64>,
    /// Reset time constant (Tx1) (>= 0). Typical value = 0,035.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tx1: Option<f64>,
    /// Time constant (Tx2) (>= 0). Typical value = 5,0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tx2: Option<f64>,
    /// Limiter (Vsmax) (> PssSB4.vsmin). Typical value = 0,062.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vsmax: Option<f64>,
    /// Limiter (Vsmin) (< PssSB4.vsmax). Typical value = -0,062.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vsmin: Option<f64>,
}
impl crate::base::CimElement for PssSB4 {
    fn mrid(&self) -> &str { &self.base.base.base.id }
    fn type_name(&self) -> &'static str { "PssSB4" }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn to_json_value(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or(serde_json::Value::Null)
    }
    fn to_block(&self) -> crate::base::RdfBlock {
        let mut block = self.base.to_block();
        block.type_name = "PssSB4".to_string();
        if let Some(v) = self.kx {
            block.fields.insert("PssSB4.kx".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.ta {
            block.fields.insert("PssSB4.ta".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.tb {
            block.fields.insert("PssSB4.tb".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.tc {
            block.fields.insert("PssSB4.tc".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.td {
            block.fields.insert("PssSB4.td".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.te {
            block.fields.insert("PssSB4.te".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.tt {
            block.fields.insert("PssSB4.tt".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.tx1 {
            block.fields.insert("PssSB4.tx1".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.tx2 {
            block.fields.insert("PssSB4.tx2".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.vsmax {
            block.fields.insert("PssSB4.vsmax".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.vsmin {
            block.fields.insert("PssSB4.vsmin".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        block
    }
}

impl PssSB4 {
    pub fn from_block(b: &crate::base::RdfBlock) -> Self {
        let mut obj = Self::default();
        obj.base.base.base.id.clone_from(&b.mrid);
        for (key, val) in &b.fields {
            match key.as_str() {
                "PssSB4.kx" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.kx = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.kx = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "PssSB4.ta" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.ta = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.ta = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "PssSB4.tb" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.tb = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.tb = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "PssSB4.tc" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.tc = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.tc = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "PssSB4.td" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.td = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.td = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "PssSB4.te" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.te = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.te = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "PssSB4.tt" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.tt = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.tt = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "PssSB4.tx1" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.tx1 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.tx1 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "PssSB4.tx2" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.tx2 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.tx2 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "PssSB4.vsmax" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.vsmax = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.vsmax = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "PssSB4.vsmin" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.vsmin = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.vsmin = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "PowerSystemStabilizerDynamics.ExcitationSystemDynamics" => {
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
