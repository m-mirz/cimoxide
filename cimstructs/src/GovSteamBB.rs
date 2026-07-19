/// European governor model.
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct GovSteamBB {
    #[serde(flatten)]
    pub base: super::TurbineGovernorDynamics,
    /// Frequency deadband (fcut) (>= 0). Typical value = 0,002.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fcut: Option<f64>,
    /// Gain (K2). Typical value = 0,75.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub k2: Option<f64>,
    /// Gain (K3). Typical value = 0,5.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub k3: Option<f64>,
    /// Gain (Kd). Typical value = 1,0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kd: Option<f64>,
    /// Gain (Kg). Typical value = 1,0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kg: Option<f64>,
    /// Gain (Kls) (> 0). Typical value = 0,1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kls: Option<f64>,
    /// Gain (Kp). Typical value = 1,0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kp: Option<f64>,
    /// Gain (Ks). Typical value = 21,0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ks: Option<f64>,
    /// Electric power input selection (Peflag). true = electric power input false = feedback signal. Typical value = false.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub peflag: Option<bool>,
    /// High power limit (Pmax) (> GovSteamBB.pmin). Typical value = 1,0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pmax: Option<f64>,
    /// Low power limit (Pmin) (< GovSteamBB.pmax). Typical value = 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pmin: Option<f64>,
    /// Time constant (T1). Typical value = 0,05.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub t1: Option<f64>,
    /// Time constant (T4). Typical value = 0,15.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub t4: Option<f64>,
    /// Time constant (T5). Typical value = 12,0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub t5: Option<f64>,
    /// Time constant (T6). Typical value = 0,75.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub t6: Option<f64>,
    /// Time constant (Td) (> 0). Typical value = 1,0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub td: Option<f64>,
    /// Time constant (Tn) (> 0). Typical value = 1,0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tn: Option<f64>,
}
impl crate::base::CimElement for GovSteamBB {
    fn mrid(&self) -> &str { &self.base.base.base.id }
    fn type_name(&self) -> &'static str { "GovSteamBB" }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn to_json_value(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or(serde_json::Value::Null)
    }
    fn to_block(&self) -> crate::base::RdfBlock {
        let mut block = self.base.to_block();
        block.type_name = "GovSteamBB".to_string();
        if let Some(v) = self.fcut {
            block.fields.insert("GovSteamBB.fcut".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.k2 {
            block.fields.insert("GovSteamBB.k2".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.k3 {
            block.fields.insert("GovSteamBB.k3".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.kd {
            block.fields.insert("GovSteamBB.kd".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.kg {
            block.fields.insert("GovSteamBB.kg".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.kls {
            block.fields.insert("GovSteamBB.kls".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.kp {
            block.fields.insert("GovSteamBB.kp".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.ks {
            block.fields.insert("GovSteamBB.ks".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.peflag {
            block.fields.insert("GovSteamBB.peflag".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.pmax {
            block.fields.insert("GovSteamBB.pmax".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.pmin {
            block.fields.insert("GovSteamBB.pmin".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.t1 {
            block.fields.insert("GovSteamBB.t1".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.t4 {
            block.fields.insert("GovSteamBB.t4".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.t5 {
            block.fields.insert("GovSteamBB.t5".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.t6 {
            block.fields.insert("GovSteamBB.t6".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.td {
            block.fields.insert("GovSteamBB.td".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.tn {
            block.fields.insert("GovSteamBB.tn".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        block
    }
}

impl GovSteamBB {
    pub fn from_block(b: &crate::base::RdfBlock) -> Self {
        let mut obj = Self::default();
        obj.base.base.base.id.clone_from(&b.mrid);
        for (key, val) in &b.fields {
            match key.as_str() {
                "GovSteamBB.fcut" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.fcut = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.fcut = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovSteamBB.k2" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.k2 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.k2 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovSteamBB.k3" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.k3 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.k3 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovSteamBB.kd" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.kd = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.kd = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovSteamBB.kg" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.kg = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.kg = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovSteamBB.kls" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.kls = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.kls = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovSteamBB.kp" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.kp = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.kp = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovSteamBB.ks" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.ks = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.ks = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovSteamBB.peflag" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.peflag = Some(sv.trim() == "true"); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.peflag = Some(sv.trim() == "true"); }
                        }
                        _ => {}
                    }
                }
                "GovSteamBB.pmax" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.pmax = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.pmax = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovSteamBB.pmin" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.pmin = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.pmin = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovSteamBB.t1" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.t1 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.t1 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovSteamBB.t4" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.t4 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.t4 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovSteamBB.t5" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.t5 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.t5 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovSteamBB.t6" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.t6 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.t6 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovSteamBB.td" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.td = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.td = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovSteamBB.tn" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.tn = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.tn = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "TurbineGovernorDynamics.AsynchronousMachineDynamics" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.base.asynchronous_machine_dynamics = Some(crate::base::MridRef { mrid: sv.clone() });
                    }
                }
                "TurbineGovernorDynamics.SynchronousMachineDynamics" => {
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
