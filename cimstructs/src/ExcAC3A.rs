/// Modified IEEE AC3A alternator-supplied rectifier excitation system with different field current limit.
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct ExcAC3A {
    #[serde(flatten)]
    pub base: super::ExcitationSystemDynamics,
    /// Value of Efd at which feedback gain changes (Efdn) (> 0). Typical value = 2,36.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub efdn: Option<f64>,
    /// Voltage regulator gain (Ka) (> 0). Typical value = 45,62.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ka: Option<f64>,
    /// Rectifier loading factor proportional to commutating reactance (Kc) (>= 0). Typical value = 0,104.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kc: Option<f64>,
    /// Demagnetizing factor, a function of exciter alternator reactances (Kd) (>= 0). Typical value = 0,499.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kd: Option<f64>,
    /// Exciter constant related to self-excited field (Ke). Typical value = 1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ke: Option<f64>,
    /// Excitation control system stabilizer gains (Kf) (>= 0). Typical value = 0,143.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kf: Option<f64>,
    /// Coefficient to allow different usage of the model (Kf1). Typical value = 1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kf1: Option<f64>,
    /// Coefficient to allow different usage of the model (Kf2). Typical value = 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kf2: Option<f64>,
    /// Gain used in the minimum field voltage limiter loop (Klv). Typical value = 0,194.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub klv: Option<f64>,
    /// Excitation control system stabilizer gain (Kn) (>= 0). Typical value =0,05.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kn: Option<f64>,
    /// Constant associated with regulator and alternator field power supply (Kr) (> 0). Typical value =3,77.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kr: Option<f64>,
    /// Coefficient to allow different usage of the model-speed coefficient (Ks). Typical value = 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ks: Option<f64>,
    /// Exciter saturation function value at the corresponding exciter voltage, Ve1, back of commutating reactance (Se[Ve1]) (>= 0). Typical value = 1,143.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seve1: Option<f64>,
    /// Exciter saturation function value at the corresponding exciter voltage, Ve2, back of commutating reactance (Se[Ve2]) (>= 0). Typical value = 0,1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seve2: Option<f64>,
    /// Voltage regulator time constant (Ta) (> 0). Typical value = 0,013.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ta: Option<f64>,
    /// Voltage regulator time constant (Tb) (>= 0). Typical value = 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tb: Option<f64>,
    /// Voltage regulator time constant (Tc) (>= 0). Typical value = 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tc: Option<f64>,
    /// Exciter time constant, integration rate associated with exciter control (Te) (> 0). Typical value = 1,17.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub te: Option<f64>,
    /// Excitation control system stabilizer time constant (Tf) (> 0). Typical value = 1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tf: Option<f64>,
    /// Maximum voltage regulator output (Vamax) (> 0). Typical value = 1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vamax: Option<f64>,
    /// Minimum voltage regulator output (Vamin) (< 0). Typical value = -0,95.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vamin: Option<f64>,
    /// Exciter alternator output voltages back of commutating reactance at which saturation is defined (Ve1) (> 0). Typical value = 6.24.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ve1: Option<f64>,
    /// Exciter alternator output voltages back of commutating reactance at which saturation is defined (Ve2) (> 0). Typical value = 4,68.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ve2: Option<f64>,
    /// Minimum exciter voltage output (Vemin) (<= 0). Typical value = 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vemin: Option<f64>,
    /// Exciter field current limit reference (Vfemax) (>= 0). Typical value = 16.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vfemax: Option<f64>,
    /// Field voltage used in the minimum field voltage limiter loop (Vlv). Typical value = 0,79.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vlv: Option<f64>,
}
impl crate::base::CimElement for ExcAC3A {
    fn mrid(&self) -> &str { &self.base.base.base.id }
    fn type_name(&self) -> &'static str { "ExcAC3A" }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn to_json_value(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or(serde_json::Value::Null)
    }
    fn to_block(&self) -> crate::base::RdfBlock {
        let mut block = self.base.to_block();
        block.type_name = "ExcAC3A".to_string();
        if let Some(v) = self.efdn {
            block.fields.insert("ExcAC3A.efdn".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.ka {
            block.fields.insert("ExcAC3A.ka".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.kc {
            block.fields.insert("ExcAC3A.kc".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.kd {
            block.fields.insert("ExcAC3A.kd".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.ke {
            block.fields.insert("ExcAC3A.ke".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.kf {
            block.fields.insert("ExcAC3A.kf".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.kf1 {
            block.fields.insert("ExcAC3A.kf1".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.kf2 {
            block.fields.insert("ExcAC3A.kf2".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.klv {
            block.fields.insert("ExcAC3A.klv".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.kn {
            block.fields.insert("ExcAC3A.kn".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.kr {
            block.fields.insert("ExcAC3A.kr".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.ks {
            block.fields.insert("ExcAC3A.ks".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.seve1 {
            block.fields.insert("ExcAC3A.seve1".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.seve2 {
            block.fields.insert("ExcAC3A.seve2".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.ta {
            block.fields.insert("ExcAC3A.ta".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.tb {
            block.fields.insert("ExcAC3A.tb".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.tc {
            block.fields.insert("ExcAC3A.tc".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.te {
            block.fields.insert("ExcAC3A.te".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.tf {
            block.fields.insert("ExcAC3A.tf".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.vamax {
            block.fields.insert("ExcAC3A.vamax".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.vamin {
            block.fields.insert("ExcAC3A.vamin".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.ve1 {
            block.fields.insert("ExcAC3A.ve1".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.ve2 {
            block.fields.insert("ExcAC3A.ve2".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.vemin {
            block.fields.insert("ExcAC3A.vemin".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.vfemax {
            block.fields.insert("ExcAC3A.vfemax".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.vlv {
            block.fields.insert("ExcAC3A.vlv".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        block
    }
}

impl ExcAC3A {
    pub fn from_block(b: &crate::base::RdfBlock) -> Self {
        let mut obj = Self::default();
        obj.base.base.base.id.clone_from(&b.mrid);
        for (key, val) in &b.fields {
            match key.as_str() {
                "ExcAC3A.efdn" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.efdn = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.efdn = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcAC3A.ka" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.ka = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.ka = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcAC3A.kc" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.kc = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.kc = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcAC3A.kd" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.kd = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.kd = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcAC3A.ke" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.ke = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.ke = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcAC3A.kf" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.kf = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.kf = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcAC3A.kf1" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.kf1 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.kf1 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcAC3A.kf2" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.kf2 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.kf2 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcAC3A.klv" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.klv = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.klv = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcAC3A.kn" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.kn = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.kn = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcAC3A.kr" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.kr = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.kr = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcAC3A.ks" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.ks = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.ks = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcAC3A.seve1" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.seve1 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.seve1 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcAC3A.seve2" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.seve2 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.seve2 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcAC3A.ta" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.ta = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.ta = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcAC3A.tb" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.tb = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.tb = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcAC3A.tc" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.tc = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.tc = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcAC3A.te" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.te = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.te = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcAC3A.tf" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.tf = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.tf = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcAC3A.vamax" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.vamax = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.vamax = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcAC3A.vamin" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.vamin = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.vamin = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcAC3A.ve1" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.ve1 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.ve1 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcAC3A.ve2" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.ve2 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.ve2 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcAC3A.vemin" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.vemin = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.vemin = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcAC3A.vfemax" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.vfemax = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.vfemax = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcAC3A.vlv" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.vlv = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.vlv = Some(v); } }
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
