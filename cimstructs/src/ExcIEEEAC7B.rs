/// IEEE 421.5-2005 type AC7B model. The model represents excitation systems which consist of an AC alternator with either stationary or rotating rectifiers to produce the DC field requirements. It is an upgrade to earlier AC excitation systems, which replace only the controls but retain the AC alternator and diode rectifier bridge. Reference: IEEE 421.5-2005, 6.7. Note, however, that in IEEE 421.5-2005, the [1 / sTE] block is shown as [1 / (1 + sTE)], which is incorrect.
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct ExcIEEEAC7B {
    #[serde(flatten)]
    pub base: super::ExcitationSystemDynamics,
    /// Rectifier loading factor proportional to commutating reactance (KC) (>= 0). Typical value = 0,18.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kc: Option<f64>,
    /// Demagnetizing factor, a function of exciter alternator reactances (KD) (>= 0). Typical value = 0,02.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kd: Option<f64>,
    /// Voltage regulator derivative gain (KDR) (>= 0). Typical value = 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kdr: Option<f64>,
    /// Exciter constant related to self-excited field (KE). Typical value = 1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ke: Option<f64>,
    /// Excitation control system stabilizer gain (KF1) (>= 0). Typical value = 0,212.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kf1: Option<f64>,
    /// Excitation control system stabilizer gain (KF2) (>= 0). Typical value = 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kf2: Option<f64>,
    /// Excitation control system stabilizer gain (KF3) (>= 0). Typical value = 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kf3: Option<f64>,
    /// Voltage regulator integral gain (KIA) (>= 0). Typical value = 59,69.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kia: Option<f64>,
    /// Voltage regulator integral gain (KIR) (>= 0). Typical value = 4,24.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kir: Option<f64>,
    /// Exciter field voltage lower limit parameter (KL). Typical value = 10.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kl: Option<f64>,
    /// Potential circuit gain coefficient (KP) (> 0). Typical value = 4,96.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kp: Option<f64>,
    /// Voltage regulator proportional gain (KPA) (> 0 if ExcIEEEAC7B.kia = 0). Typical value = 65,36.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kpa: Option<f64>,
    /// Voltage regulator proportional gain (KPR) (> 0 if ExcIEEEAC7B.kir = 0). Typical value = 4,24.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kpr: Option<f64>,
    /// Exciter saturation function value at the corresponding exciter voltage, VE1, back of commutating reactance (SE[VE1]) (>= 0). Typical value = 0,44.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seve1: Option<f64>,
    /// Exciter saturation function value at the corresponding exciter voltage, VE2, back of commutating reactance (SE[VE2]) (>= 0). Typical value = 0,075.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seve2: Option<f64>,
    /// Lag time constant (TDR) (>= 0). Typical value = 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tdr: Option<f64>,
    /// Exciter time constant, integration rate associated with exciter control (TE) (> 0). Typical value = 1,1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub te: Option<f64>,
    /// Excitation control system stabilizer time constant (TF) (> 0). Typical value = 1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tf: Option<f64>,
    /// Maximum voltage regulator output (VAMAX) (> 0). Typical value = 1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vamax: Option<f64>,
    /// Minimum voltage regulator output (VAMIN) (< 0). Typical value = -0,95.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vamin: Option<f64>,
    /// Exciter alternator output voltages back of commutating reactance at which saturation is defined (VE1) (> 0). Typical value = 6,3.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ve1: Option<f64>,
    /// Exciter alternator output voltages back of commutating reactance at which saturation is defined (VE2) (> 0). Typical value = 3,02.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ve2: Option<f64>,
    /// Minimum exciter voltage output (VEMIN) (<= 0). Typical value = 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vemin: Option<f64>,
    /// Exciter field current limit reference (VFEMAX). Typical value = 6,9.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vfemax: Option<f64>,
    /// Maximum voltage regulator output (VRMAX) (> 0). Typical value = 5,79.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vrmax: Option<f64>,
    /// Minimum voltage regulator output (VRMIN) (< 0). Typical value = -5,79.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vrmin: Option<f64>,
}
impl crate::base::CimElement for ExcIEEEAC7B {
    fn mrid(&self) -> &str { &self.base.base.base.id }
    fn type_name(&self) -> &'static str { "ExcIEEEAC7B" }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn to_json_value(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or(serde_json::Value::Null)
    }
    fn to_block(&self) -> crate::base::RdfBlock {
        let mut block = self.base.to_block();
        block.type_name = "ExcIEEEAC7B".to_string();
        if let Some(v) = self.kc {
            block.fields.insert("ExcIEEEAC7B.kc".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.kd {
            block.fields.insert("ExcIEEEAC7B.kd".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.kdr {
            block.fields.insert("ExcIEEEAC7B.kdr".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.ke {
            block.fields.insert("ExcIEEEAC7B.ke".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.kf1 {
            block.fields.insert("ExcIEEEAC7B.kf1".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.kf2 {
            block.fields.insert("ExcIEEEAC7B.kf2".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.kf3 {
            block.fields.insert("ExcIEEEAC7B.kf3".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.kia {
            block.fields.insert("ExcIEEEAC7B.kia".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.kir {
            block.fields.insert("ExcIEEEAC7B.kir".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.kl {
            block.fields.insert("ExcIEEEAC7B.kl".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.kp {
            block.fields.insert("ExcIEEEAC7B.kp".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.kpa {
            block.fields.insert("ExcIEEEAC7B.kpa".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.kpr {
            block.fields.insert("ExcIEEEAC7B.kpr".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.seve1 {
            block.fields.insert("ExcIEEEAC7B.seve1".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.seve2 {
            block.fields.insert("ExcIEEEAC7B.seve2".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.tdr {
            block.fields.insert("ExcIEEEAC7B.tdr".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.te {
            block.fields.insert("ExcIEEEAC7B.te".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.tf {
            block.fields.insert("ExcIEEEAC7B.tf".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.vamax {
            block.fields.insert("ExcIEEEAC7B.vamax".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.vamin {
            block.fields.insert("ExcIEEEAC7B.vamin".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.ve1 {
            block.fields.insert("ExcIEEEAC7B.ve1".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.ve2 {
            block.fields.insert("ExcIEEEAC7B.ve2".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.vemin {
            block.fields.insert("ExcIEEEAC7B.vemin".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.vfemax {
            block.fields.insert("ExcIEEEAC7B.vfemax".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.vrmax {
            block.fields.insert("ExcIEEEAC7B.vrmax".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.vrmin {
            block.fields.insert("ExcIEEEAC7B.vrmin".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        block
    }
}

impl ExcIEEEAC7B {
    pub fn from_block(b: &crate::base::RdfBlock) -> Self {
        let mut obj = Self::default();
        obj.base.base.base.id.clone_from(&b.mrid);
        for (key, val) in &b.fields {
            match key.as_str() {
                "ExcIEEEAC7B.kc" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.kc = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.kc = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcIEEEAC7B.kd" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.kd = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.kd = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcIEEEAC7B.kdr" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.kdr = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.kdr = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcIEEEAC7B.ke" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.ke = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.ke = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcIEEEAC7B.kf1" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.kf1 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.kf1 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcIEEEAC7B.kf2" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.kf2 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.kf2 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcIEEEAC7B.kf3" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.kf3 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.kf3 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcIEEEAC7B.kia" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.kia = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.kia = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcIEEEAC7B.kir" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.kir = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.kir = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcIEEEAC7B.kl" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.kl = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.kl = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcIEEEAC7B.kp" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.kp = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.kp = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcIEEEAC7B.kpa" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.kpa = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.kpa = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcIEEEAC7B.kpr" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.kpr = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.kpr = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcIEEEAC7B.seve1" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.seve1 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.seve1 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcIEEEAC7B.seve2" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.seve2 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.seve2 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcIEEEAC7B.tdr" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.tdr = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.tdr = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcIEEEAC7B.te" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.te = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.te = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcIEEEAC7B.tf" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.tf = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.tf = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcIEEEAC7B.vamax" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.vamax = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.vamax = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcIEEEAC7B.vamin" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.vamin = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.vamin = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcIEEEAC7B.ve1" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.ve1 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.ve1 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcIEEEAC7B.ve2" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.ve2 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.ve2 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcIEEEAC7B.vemin" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.vemin = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.vemin = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcIEEEAC7B.vfemax" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.vfemax = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.vfemax = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcIEEEAC7B.vrmax" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.vrmax = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.vrmax = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcIEEEAC7B.vrmin" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.vrmin = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.vrmin = Some(v); } }
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
