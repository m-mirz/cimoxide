/// Cross compound turbine governor. Unlike tandem compound units, cross compound units are not on the same shaft.
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct GovSteamCC {
    #[serde(flatten)]
    pub base: super::CrossCompoundTurbineGovernorDynamics,
    /// HP damping factor (Dhp). Typical value = 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dhp: Option<f64>,
    /// LP damping factor (Dlp). Typical value = 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dlp: Option<f64>,
    /// Fraction of HP power ahead of reheater (Fhp). Typical value = 0,3.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fhp: Option<f64>,
    /// Fraction of LP power ahead of reheater (Flp). Typical value = 0,7.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flp: Option<f64>,
    /// Base for power values (MWbase) (> 0). Unit = MW.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mwbase: Option<f64>,
    /// Maximum HP value position (Pmaxhp). Typical value = 1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pmaxhp: Option<f64>,
    /// Maximum LP value position (Pmaxlp). Typical value = 1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pmaxlp: Option<f64>,
    /// HP governor droop (Rhp) (> 0). Typical value = 0,05.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rhp: Option<f64>,
    /// LP governor droop (Rlp) (> 0). Typical value = 0,05.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rlp: Option<f64>,
    /// HP governor time constant (T1hp) (>= 0). Typical value = 0,1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub t1hp: Option<f64>,
    /// LP governor time constant (T1lp) (>= 0). Typical value = 0,1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub t1lp: Option<f64>,
    /// HP turbine time constant (T3hp) (>= 0). Typical value = 0,1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub t3hp: Option<f64>,
    /// LP turbine time constant (T3lp) (>= 0). Typical value = 0,1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub t3lp: Option<f64>,
    /// HP turbine time constant (T4hp) (>= 0). Typical value = 0,1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub t4hp: Option<f64>,
    /// LP turbine time constant (T4lp) (>= 0). Typical value = 0,1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub t4lp: Option<f64>,
    /// HP reheater time constant (T5hp) (>= 0). Typical value = 10.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub t5hp: Option<f64>,
    /// LP reheater time constant (T5lp) (>= 0). Typical value = 10.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub t5lp: Option<f64>,
}
impl crate::base::CimElement for GovSteamCC {
    fn mrid(&self) -> &str { &self.base.base.base.id }
    fn type_name(&self) -> &'static str { "GovSteamCC" }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn to_json_value(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or(serde_json::Value::Null)
    }
    fn to_block(&self) -> crate::base::RdfBlock {
        let mut block = self.base.to_block();
        block.type_name = "GovSteamCC".to_string();
        if let Some(v) = self.dhp {
            block.fields.insert("GovSteamCC.dhp".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.dlp {
            block.fields.insert("GovSteamCC.dlp".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.fhp {
            block.fields.insert("GovSteamCC.fhp".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.flp {
            block.fields.insert("GovSteamCC.flp".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.mwbase {
            block.fields.insert("GovSteamCC.mwbase".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.pmaxhp {
            block.fields.insert("GovSteamCC.pmaxhp".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.pmaxlp {
            block.fields.insert("GovSteamCC.pmaxlp".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.rhp {
            block.fields.insert("GovSteamCC.rhp".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.rlp {
            block.fields.insert("GovSteamCC.rlp".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.t1hp {
            block.fields.insert("GovSteamCC.t1hp".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.t1lp {
            block.fields.insert("GovSteamCC.t1lp".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.t3hp {
            block.fields.insert("GovSteamCC.t3hp".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.t3lp {
            block.fields.insert("GovSteamCC.t3lp".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.t4hp {
            block.fields.insert("GovSteamCC.t4hp".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.t4lp {
            block.fields.insert("GovSteamCC.t4lp".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.t5hp {
            block.fields.insert("GovSteamCC.t5hp".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.t5lp {
            block.fields.insert("GovSteamCC.t5lp".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        block
    }
}

impl GovSteamCC {
    pub fn from_block(b: &crate::base::RdfBlock) -> Self {
        let mut obj = Self::default();
        obj.base.base.base.id.clone_from(&b.mrid);
        for (key, val) in &b.fields {
            match key.as_str() {
                "GovSteamCC.dhp" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.dhp = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.dhp = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovSteamCC.dlp" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.dlp = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.dlp = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovSteamCC.fhp" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.fhp = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.fhp = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovSteamCC.flp" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.flp = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.flp = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovSteamCC.mwbase" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.mwbase = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.mwbase = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovSteamCC.pmaxhp" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.pmaxhp = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.pmaxhp = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovSteamCC.pmaxlp" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.pmaxlp = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.pmaxlp = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovSteamCC.rhp" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.rhp = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.rhp = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovSteamCC.rlp" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.rlp = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.rlp = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovSteamCC.t1hp" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.t1hp = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.t1hp = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovSteamCC.t1lp" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.t1lp = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.t1lp = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovSteamCC.t3hp" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.t3hp = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.t3hp = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovSteamCC.t3lp" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.t3lp = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.t3lp = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovSteamCC.t4hp" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.t4hp = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.t4hp = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovSteamCC.t4lp" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.t4lp = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.t4lp = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovSteamCC.t5hp" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.t5hp = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.t5hp = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovSteamCC.t5lp" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.t5lp = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.t5lp = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "CrossCompoundTurbineGovernorDynamics.HighPressureSynchronousMachineDynamics" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.base.high_pressure_synchronous_machine_dynamics = Some(crate::base::MridRef { mrid: sv.clone() });
                    }
                }
                "CrossCompoundTurbineGovernorDynamics.LowPressureSynchronousMachineDynamics" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.base.low_pressure_synchronous_machine_dynamics = Some(crate::base::MridRef { mrid: sv.clone() });
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
