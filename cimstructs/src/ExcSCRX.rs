/// Simple excitation system with generic characteristics typical of many excitation systems; intended for use where negative field current could be a problem.
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct ExcSCRX {
    #[serde(flatten)]
    pub base: super::ExcitationSystemDynamics,
    /// Power source switch (Cswitch). true = fixed voltage of 1.0 PU false = generator terminal voltage.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cswitch: Option<bool>,
    /// Maximum field voltage output (Emax) (> ExcSCRX.emin). Typical value = 5.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emax: Option<f64>,
    /// Minimum field voltage output (Emin) (< ExcSCRX.emax). Typical value = 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emin: Option<f64>,
    /// Gain (K) (> 0). Typical value = 200.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub k: Option<f64>,
    /// Ratio of field discharge resistance to field winding resistance ([rc / rfd]). Typical value = 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rcrfd: Option<f64>,
    /// Gain reduction ratio of lag-lead element ([Ta / Tb]). The parameter Ta is not defined explicitly. Typical value = 0.1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tatb: Option<f64>,
    /// Denominator time constant of lag-lead block (Tb) (>= 0). Typical value = 10.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tb: Option<f64>,
    /// Time constant of gain block (Te) (> 0). Typical value = 0,02.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub te: Option<f64>,
}
impl crate::base::CimElement for ExcSCRX {
    fn mrid(&self) -> &str { &self.base.base.base.id }
    fn type_name(&self) -> &'static str { "ExcSCRX" }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn to_json_value(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or(serde_json::Value::Null)
    }
    fn to_block(&self) -> crate::base::RdfBlock {
        let mut block = self.base.to_block();
        block.type_name = "ExcSCRX".to_string();
        if let Some(v) = self.cswitch {
            block.fields.insert("ExcSCRX.cswitch".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.emax {
            block.fields.insert("ExcSCRX.emax".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.emin {
            block.fields.insert("ExcSCRX.emin".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.k {
            block.fields.insert("ExcSCRX.k".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.rcrfd {
            block.fields.insert("ExcSCRX.rcrfd".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.tatb {
            block.fields.insert("ExcSCRX.tatb".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.tb {
            block.fields.insert("ExcSCRX.tb".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.te {
            block.fields.insert("ExcSCRX.te".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        block
    }
}

impl ExcSCRX {
    pub fn from_block(b: &crate::base::RdfBlock) -> Self {
        let mut obj = Self::default();
        obj.base.base.base.id.clone_from(&b.mrid);
        for (key, val) in &b.fields {
            match key.as_str() {
                "ExcSCRX.cswitch" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.cswitch = Some(sv.trim() == "true"); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.cswitch = Some(sv.trim() == "true"); }
                        }
                        _ => {}
                    }
                }
                "ExcSCRX.emax" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.emax = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.emax = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcSCRX.emin" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.emin = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.emin = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcSCRX.k" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.k = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.k = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcSCRX.rcrfd" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.rcrfd = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.rcrfd = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcSCRX.tatb" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.tatb = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.tatb = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcSCRX.tb" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.tb = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.tb = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcSCRX.te" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.te = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.te = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcitationSystemDynamics.SynchronousMachineDynamics" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.base.synchronous_machine_dynamics = Some(crate::base::MridRef { mrid: sv.clone() });
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
