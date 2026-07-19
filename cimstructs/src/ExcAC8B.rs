/// Modified IEEE AC8B alternator-supplied rectifier excitation system with speed input and input limiter.
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct ExcAC8B {
    #[serde(flatten)]
    pub base: super::ExcitationSystemDynamics,
    /// Input limiter indicator. true = input limiter Vimax and Vimin is considered false = input limiter Vimax and Vimin is not considered. Typical value = true.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inlim: Option<bool>,
    /// Voltage regulator gain (Ka) (> 0). Typical value = 1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ka: Option<f64>,
    /// Rectifier loading factor proportional to commutating reactance (Kc) (>= 0). Typical value = 0,55.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kc: Option<f64>,
    /// Demagnetizing factor, a function of exciter alternator reactances (Kd) (>= 0). Typical value = 1,1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kd: Option<f64>,
    /// Voltage regulator derivative gain (Kdr) (>= 0). Typical value = 10.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kdr: Option<f64>,
    /// Exciter constant related to self-excited field (Ke). Typical value = 1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ke: Option<f64>,
    /// Voltage regulator integral gain (Kir) (>= 0). Typical value = 5.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kir: Option<f64>,
    /// Voltage regulator proportional gain (Kpr) (> 0 if ExcAC8B.kir = 0). Typical value = 80.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kpr: Option<f64>,
    /// Coefficient to allow different usage of the model-speed coefficient (Ks). Typical value = 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ks: Option<f64>,
    /// PID limiter indicator. true = input limiter Vpidmax and Vpidmin is considered false = input limiter Vpidmax and Vpidmin is not considered. Typical value = true.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pidlim: Option<bool>,
    /// Exciter saturation function value at the corresponding exciter voltage, Ve1, back of commutating reactance (Se[Ve1]) (>= 0). Typical value = 0,3.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seve1: Option<f64>,
    /// Exciter saturation function value at the corresponding exciter voltage, Ve2, back of commutating reactance (Se[Ve2]) (>= 0). Typical value = 3.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seve2: Option<f64>,
    /// Voltage regulator time constant (Ta) (>= 0). Typical value = 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ta: Option<f64>,
    /// Lag time constant (Tdr) (> 0 if ExcAC8B.kdr > 0). Typical value = 0,1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tdr: Option<f64>,
    /// Exciter time constant, integration rate associated with exciter control (Te) (> 0). Typical value = 1,2.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub te: Option<f64>,
    /// Selector for the limiter on the block (1/sTe). See diagram for meaning of true and false. Typical value = false.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub telim: Option<bool>,
    /// Exciter alternator output voltages back of commutating reactance at which saturation is defined (Ve1) (> 0). Typical value = 6,5.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ve1: Option<f64>,
    /// Exciter alternator output voltages back of commutating reactance at which saturation is defined (Ve2) (> 0). Typical value = 9.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ve2: Option<f64>,
    /// Minimum exciter voltage output (Vemin) (<= 0). Typical value = 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vemin: Option<f64>,
    /// Exciter field current limit reference (Vfemax). Typical value = 6.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vfemax: Option<f64>,
    /// Input signal maximum (Vimax) (> ExcAC8B.vimin). Typical value = 35.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vimax: Option<f64>,
    /// Input signal minimum (Vimin) (< ExcAC8B.vimax). Typical value = -10.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vimin: Option<f64>,
    /// PID maximum controller output (Vpidmax) (> ExcAC8B.vpidmin). Typical value = 35.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpidmax: Option<f64>,
    /// PID minimum controller output (Vpidmin) (< ExcAC8B.vpidmax). Typical value = -10.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpidmin: Option<f64>,
    /// Maximum voltage regulator output (Vrmax) (> 0). Typical value = 35.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vrmax: Option<f64>,
    /// Minimum voltage regulator output (Vrmin) (< 0). Typical value = 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vrmin: Option<f64>,
    /// Multiply by generator's terminal voltage indicator. true =the limits Vrmax and Vrmin are multiplied by the generator’s terminal voltage to represent a thyristor power stage fed from the generator terminals false = limits are not multiplied by generator's terminal voltage. Typical value = false.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vtmult: Option<bool>,
}
impl crate::base::CimElement for ExcAC8B {
    fn mrid(&self) -> &str { &self.base.base.base.id }
    fn type_name(&self) -> &'static str { "ExcAC8B" }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn to_json_value(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or(serde_json::Value::Null)
    }
    fn to_block(&self) -> crate::base::RdfBlock {
        let mut block = self.base.to_block();
        block.type_name = "ExcAC8B".to_string();
        if let Some(v) = self.inlim {
            block.fields.insert("ExcAC8B.inlim".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.ka {
            block.fields.insert("ExcAC8B.ka".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.kc {
            block.fields.insert("ExcAC8B.kc".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.kd {
            block.fields.insert("ExcAC8B.kd".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.kdr {
            block.fields.insert("ExcAC8B.kdr".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.ke {
            block.fields.insert("ExcAC8B.ke".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.kir {
            block.fields.insert("ExcAC8B.kir".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.kpr {
            block.fields.insert("ExcAC8B.kpr".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.ks {
            block.fields.insert("ExcAC8B.ks".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.pidlim {
            block.fields.insert("ExcAC8B.pidlim".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.seve1 {
            block.fields.insert("ExcAC8B.seve1".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.seve2 {
            block.fields.insert("ExcAC8B.seve2".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.ta {
            block.fields.insert("ExcAC8B.ta".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.tdr {
            block.fields.insert("ExcAC8B.tdr".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.te {
            block.fields.insert("ExcAC8B.te".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.telim {
            block.fields.insert("ExcAC8B.telim".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.ve1 {
            block.fields.insert("ExcAC8B.ve1".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.ve2 {
            block.fields.insert("ExcAC8B.ve2".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.vemin {
            block.fields.insert("ExcAC8B.vemin".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.vfemax {
            block.fields.insert("ExcAC8B.vfemax".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.vimax {
            block.fields.insert("ExcAC8B.vimax".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.vimin {
            block.fields.insert("ExcAC8B.vimin".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.vpidmax {
            block.fields.insert("ExcAC8B.vpidmax".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.vpidmin {
            block.fields.insert("ExcAC8B.vpidmin".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.vrmax {
            block.fields.insert("ExcAC8B.vrmax".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.vrmin {
            block.fields.insert("ExcAC8B.vrmin".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.vtmult {
            block.fields.insert("ExcAC8B.vtmult".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        block
    }
}

impl ExcAC8B {
    pub fn from_block(b: &crate::base::RdfBlock) -> Self {
        let mut obj = Self::default();
        obj.base.base.base.id.clone_from(&b.mrid);
        for (key, val) in &b.fields {
            match key.as_str() {
                "ExcAC8B.inlim" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.inlim = Some(sv.trim() == "true"); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.inlim = Some(sv.trim() == "true"); }
                        }
                        _ => {}
                    }
                }
                "ExcAC8B.ka" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.ka = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.ka = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcAC8B.kc" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.kc = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.kc = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcAC8B.kd" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.kd = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.kd = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcAC8B.kdr" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.kdr = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.kdr = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcAC8B.ke" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.ke = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.ke = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcAC8B.kir" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.kir = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.kir = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcAC8B.kpr" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.kpr = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.kpr = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcAC8B.ks" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.ks = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.ks = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcAC8B.pidlim" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.pidlim = Some(sv.trim() == "true"); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.pidlim = Some(sv.trim() == "true"); }
                        }
                        _ => {}
                    }
                }
                "ExcAC8B.seve1" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.seve1 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.seve1 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcAC8B.seve2" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.seve2 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.seve2 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcAC8B.ta" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.ta = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.ta = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcAC8B.tdr" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.tdr = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.tdr = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcAC8B.te" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.te = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.te = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcAC8B.telim" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.telim = Some(sv.trim() == "true"); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.telim = Some(sv.trim() == "true"); }
                        }
                        _ => {}
                    }
                }
                "ExcAC8B.ve1" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.ve1 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.ve1 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcAC8B.ve2" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.ve2 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.ve2 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcAC8B.vemin" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.vemin = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.vemin = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcAC8B.vfemax" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.vfemax = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.vfemax = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcAC8B.vimax" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.vimax = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.vimax = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcAC8B.vimin" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.vimin = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.vimin = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcAC8B.vpidmax" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.vpidmax = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.vpidmax = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcAC8B.vpidmin" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.vpidmin = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.vpidmin = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcAC8B.vrmax" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.vrmax = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.vrmax = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcAC8B.vrmin" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.vrmin = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.vrmin = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcAC8B.vtmult" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.vtmult = Some(sv.trim() == "true"); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.vtmult = Some(sv.trim() == "true"); }
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
