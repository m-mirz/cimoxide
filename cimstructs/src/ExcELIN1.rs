/// Static PI transformer fed excitation system ELIN (VATECH) - simplified model. This model represents an all-static excitation system. A PI voltage controller establishes a desired field current set point for a proportional current controller. The integrator of the PI controller has a follow-up input to match its signal to the present field current. A power system stabilizer with power input is included in the model.
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct ExcELIN1 {
    #[serde(flatten)]
    pub base: super::ExcitationSystemDynamics,
    /// Controller follow up deadband (Dpnf). Typical value = 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dpnf: Option<f64>,
    /// Maximum open circuit excitation voltage (Efmax) (> ExcELIN1.efmin). Typical value = 5.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub efmax: Option<f64>,
    /// Minimum open circuit excitation voltage (Efmin) (< ExcELIN1.efmax). Typical value = -5.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub efmin: Option<f64>,
    /// Stabilizer gain 1 (Ks1). Typical value = 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ks1: Option<f64>,
    /// Stabilizer gain 2 (Ks2). Typical value = 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ks2: Option<f64>,
    /// Stabilizer limit output (smax). Typical value = 0,1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smax: Option<f64>,
    /// Current transducer time constant (Tfi) (>= 0). Typical value = 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tfi: Option<f64>,
    /// Controller reset time constant (Tnu) (>= 0). Typical value = 2.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tnu: Option<f64>,
    /// Stabilizer phase lag time constant (Ts1) (>= 0). Typical value = 1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ts1: Option<f64>,
    /// Stabilizer filter time constant (Ts2) (>= 0). Typical value = 1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ts2: Option<f64>,
    /// Stabilizer parameters (Tsw) (>= 0). Typical value = 3.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tsw: Option<f64>,
    /// Current controller gain (Vpi). Typical value = 12,45.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpi: Option<f64>,
    /// Controller follow up gain (Vpnf). Typical value = 2.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpnf: Option<f64>,
    /// Voltage controller proportional gain (Vpu). Typical value = 34,5.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpu: Option<f64>,
    /// Excitation transformer effective reactance (Xe) (>= 0). Xe represents the regulation of the transformer/rectifier unit. Typical value = 0,06.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub xe: Option<f64>,
}
impl crate::base::CimElement for ExcELIN1 {
    fn mrid(&self) -> &str { &self.base.base.base.id }
    fn type_name(&self) -> &'static str { "ExcELIN1" }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn to_json_value(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or(serde_json::Value::Null)
    }
    fn to_block(&self) -> crate::base::RdfBlock {
        let mut block = self.base.to_block();
        block.type_name = "ExcELIN1".to_string();
        if let Some(v) = self.dpnf {
            block.fields.insert("ExcELIN1.dpnf".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.efmax {
            block.fields.insert("ExcELIN1.efmax".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.efmin {
            block.fields.insert("ExcELIN1.efmin".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.ks1 {
            block.fields.insert("ExcELIN1.ks1".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.ks2 {
            block.fields.insert("ExcELIN1.ks2".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.smax {
            block.fields.insert("ExcELIN1.smax".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.tfi {
            block.fields.insert("ExcELIN1.tfi".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.tnu {
            block.fields.insert("ExcELIN1.tnu".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.ts1 {
            block.fields.insert("ExcELIN1.ts1".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.ts2 {
            block.fields.insert("ExcELIN1.ts2".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.tsw {
            block.fields.insert("ExcELIN1.tsw".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.vpi {
            block.fields.insert("ExcELIN1.vpi".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.vpnf {
            block.fields.insert("ExcELIN1.vpnf".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.vpu {
            block.fields.insert("ExcELIN1.vpu".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.xe {
            block.fields.insert("ExcELIN1.xe".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        block
    }
}

impl ExcELIN1 {
    pub fn from_block(b: &crate::base::RdfBlock) -> Self {
        let mut obj = Self::default();
        obj.base.base.base.id.clone_from(&b.mrid);
        for (key, val) in &b.fields {
            match key.as_str() {
                "ExcELIN1.dpnf" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.dpnf = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.dpnf = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcELIN1.efmax" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.efmax = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.efmax = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcELIN1.efmin" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.efmin = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.efmin = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcELIN1.ks1" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.ks1 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.ks1 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcELIN1.ks2" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.ks2 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.ks2 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcELIN1.smax" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.smax = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.smax = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcELIN1.tfi" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.tfi = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.tfi = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcELIN1.tnu" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.tnu = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.tnu = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcELIN1.ts1" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.ts1 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.ts1 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcELIN1.ts2" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.ts2 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.ts2 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcELIN1.tsw" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.tsw = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.tsw = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcELIN1.vpi" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.vpi = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.vpi = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcELIN1.vpnf" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.vpnf = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.vpnf = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcELIN1.vpu" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.vpu = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.vpu = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcELIN1.xe" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.xe = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.xe = Some(v); } }
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
