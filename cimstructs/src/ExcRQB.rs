/// Excitation system type RQB (four-loop regulator, r?gulateur quatre boucles, developed in France) primarily used in nuclear or thermal generating units. This excitation system shall be always used together with power system stabilizer type PssRQB.
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct ExcRQB {
    #[serde(flatten)]
    pub base: super::ExcitationSystemDynamics,
    /// Voltage reference input gain (Ki0). Typical value = 12,7.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ki0: Option<f64>,
    /// Voltage input gain (Ki1). Typical value = -16,8.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ki1: Option<f64>,
    /// OEL input gain (KLIR). Typical value = 12,13.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub klir: Option<f64>,
    /// Limiter gain (KLUS). Typical value = 50.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub klus: Option<f64>,
    /// Integrator limiter (LSAT). Typical value = 5,73.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lsat: Option<f64>,
    /// Setpoint (LUS). Typical value = 0,12.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lus: Option<f64>,
    /// Voltage input time constant (MESU) (>= 0). Typical value = 0,02.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mesu: Option<f64>,
    /// Input time constant (T4M) (>= 0). Typical value = 5.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub t4m: Option<f64>,
    /// Lead lag time constant (TC) (>= 0). Typical value = 0,02.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tc: Option<f64>,
    /// Lead lag time constant (TE) (>= 0). Typical value = 0,22.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub te: Option<f64>,
    /// Exciter time constant (TF) (>= 0). Typical value = 0,01.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tf: Option<f64>,
    /// Maximum voltage reference limit (UCMAX) (> ExcRQB.ucmin). Typical value = 1,1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ucmax: Option<f64>,
    /// Minimum voltage reference limit (UCMIN) (< ExcRQB.ucmax). Typical value = 0,9.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ucmin: Option<f64>,
}
impl crate::base::CimElement for ExcRQB {
    fn mrid(&self) -> &str { &self.base.base.base.id }
    fn type_name(&self) -> &'static str { "ExcRQB" }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn to_json_value(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or(serde_json::Value::Null)
    }
    fn to_block(&self) -> crate::base::RdfBlock {
        let mut block = self.base.to_block();
        block.type_name = "ExcRQB".to_string();
        if let Some(v) = self.ki0 {
            block.fields.insert("ExcRQB.ki0".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.ki1 {
            block.fields.insert("ExcRQB.ki1".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.klir {
            block.fields.insert("ExcRQB.klir".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.klus {
            block.fields.insert("ExcRQB.klus".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.lsat {
            block.fields.insert("ExcRQB.lsat".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.lus {
            block.fields.insert("ExcRQB.lus".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.mesu {
            block.fields.insert("ExcRQB.mesu".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.t4m {
            block.fields.insert("ExcRQB.t4m".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.tc {
            block.fields.insert("ExcRQB.tc".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.te {
            block.fields.insert("ExcRQB.te".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.tf {
            block.fields.insert("ExcRQB.tf".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.ucmax {
            block.fields.insert("ExcRQB.ucmax".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.ucmin {
            block.fields.insert("ExcRQB.ucmin".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        block
    }
}

impl ExcRQB {
    pub fn from_block(b: &crate::base::RdfBlock) -> Self {
        let mut obj = Self::default();
        obj.base.base.base.id.clone_from(&b.mrid);
        for (key, val) in &b.fields {
            match key.as_str() {
                "ExcRQB.ki0" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.ki0 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.ki0 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcRQB.ki1" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.ki1 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.ki1 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcRQB.klir" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.klir = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.klir = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcRQB.klus" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.klus = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.klus = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcRQB.lsat" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.lsat = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.lsat = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcRQB.lus" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.lus = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.lus = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcRQB.mesu" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.mesu = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.mesu = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcRQB.t4m" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.t4m = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.t4m = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcRQB.tc" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.tc = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.tc = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcRQB.te" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.te = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.te = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcRQB.tf" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.tf = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.tf = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcRQB.ucmax" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.ucmax = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.ucmax = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcRQB.ucmin" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.ucmin = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.ucmin = Some(v); } }
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
