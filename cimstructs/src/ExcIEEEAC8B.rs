/// IEEE 421.5-2005 type AC8B model. This model represents a PID voltage regulator with either a brushless exciter or DC exciter. The AVR in this model consists of PID control, with separate constants for the proportional (KPR), integral (KIR), and derivative (KDR) gains. The representation of the brushless exciter (TE, KE, SE, KC, KD) is similar to the model type AC2A. The type AC8B model can be used to represent static voltage regulators applied to brushless excitation systems. Digitally based voltage regulators feeding DC rotating main exciters can be represented with the AC type AC8B model with the parameters KC and KD set to 0. For thyristor power stages fed from the generator terminals, the limits VRMAX and VRMIN should be a function of terminal voltage: VT x VRMAX and VT x VRMIN. Reference: IEEE 421.5-2005, 6.8.
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct ExcIEEEAC8B {
    #[serde(flatten)]
    pub base: super::ExcitationSystemDynamics,
    /// Voltage regulator gain (KA) (> 0). Typical value = 1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ka: Option<f64>,
    /// Rectifier loading factor proportional to commutating reactance (KC) (>= 0). Typical value = 0,55.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kc: Option<f64>,
    /// Demagnetizing factor, a function of exciter alternator reactances (KD) (>= 0). Typical value = 1,1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kd: Option<f64>,
    /// Voltage regulator derivative gain (KDR) (>= 0). Typical value = 10.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kdr: Option<f64>,
    /// Exciter constant related to self-excited field (KE). Typical value = 1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ke: Option<f64>,
    /// Voltage regulator integral gain (KIR) (>= 0). Typical value = 5.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kir: Option<f64>,
    /// Voltage regulator proportional gain (KPR) (> 0 if ExcIEEEAC8B.kir = 0). Typical value = 80.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kpr: Option<f64>,
    /// Exciter saturation function value at the corresponding exciter voltage, VE1, back of commutating reactance (SE[VE1]) (>= 0). Typical value = 0,3.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seve1: Option<f64>,
    /// Exciter saturation function value at the corresponding exciter voltage, VE2, back of commutating reactance (SE[VE2]) (>= 0). Typical value = 3.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seve2: Option<f64>,
    /// Voltage regulator time constant (TA) (>= 0). Typical value = 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ta: Option<f64>,
    /// Lag time constant (TDR) (> 0). Typical value = 0,1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tdr: Option<f64>,
    /// Exciter time constant, integration rate associated with exciter control (TE) (> 0). Typical value = 1,2.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub te: Option<f64>,
    /// Exciter alternator output voltages back of commutating reactance at which saturation is defined (VE1) (> 0). Typical value = 6,5.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ve1: Option<f64>,
    /// Exciter alternator output voltages back of commutating reactance at which saturation is defined (VE2) (> 0). Typical value = 9.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ve2: Option<f64>,
    /// Minimum exciter voltage output (VEMIN) (<= 0). Typical value = 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vemin: Option<f64>,
    /// Exciter field current limit reference (VFEMAX). Typical value = 6.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vfemax: Option<f64>,
    /// Maximum voltage regulator output (VRMAX) (> 0). Typical value = 35.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vrmax: Option<f64>,
    /// Minimum voltage regulator output (VRMIN) (<= 0). Typical value = 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vrmin: Option<f64>,
}
impl crate::base::CimElement for ExcIEEEAC8B {
    fn mrid(&self) -> &str { &self.base.base.base.id }
    fn type_name(&self) -> &'static str { "ExcIEEEAC8B" }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn to_json_value(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or(serde_json::Value::Null)
    }
    fn to_block(&self) -> crate::base::RdfBlock {
        let mut block = self.base.to_block();
        block.type_name = "ExcIEEEAC8B".to_string();
        if let Some(v) = self.ka {
            block.fields.insert("ExcIEEEAC8B.ka".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.kc {
            block.fields.insert("ExcIEEEAC8B.kc".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.kd {
            block.fields.insert("ExcIEEEAC8B.kd".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.kdr {
            block.fields.insert("ExcIEEEAC8B.kdr".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.ke {
            block.fields.insert("ExcIEEEAC8B.ke".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.kir {
            block.fields.insert("ExcIEEEAC8B.kir".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.kpr {
            block.fields.insert("ExcIEEEAC8B.kpr".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.seve1 {
            block.fields.insert("ExcIEEEAC8B.seve1".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.seve2 {
            block.fields.insert("ExcIEEEAC8B.seve2".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.ta {
            block.fields.insert("ExcIEEEAC8B.ta".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.tdr {
            block.fields.insert("ExcIEEEAC8B.tdr".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.te {
            block.fields.insert("ExcIEEEAC8B.te".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.ve1 {
            block.fields.insert("ExcIEEEAC8B.ve1".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.ve2 {
            block.fields.insert("ExcIEEEAC8B.ve2".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.vemin {
            block.fields.insert("ExcIEEEAC8B.vemin".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.vfemax {
            block.fields.insert("ExcIEEEAC8B.vfemax".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.vrmax {
            block.fields.insert("ExcIEEEAC8B.vrmax".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.vrmin {
            block.fields.insert("ExcIEEEAC8B.vrmin".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        block
    }
}

impl ExcIEEEAC8B {
    pub fn from_block(b: &crate::base::RdfBlock) -> Self {
        let mut obj = Self::default();
        obj.base.base.base.id.clone_from(&b.mrid);
        for (key, val) in &b.fields {
            match key.as_str() {
                "ExcIEEEAC8B.ka" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.ka = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.ka = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcIEEEAC8B.kc" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.kc = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.kc = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcIEEEAC8B.kd" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.kd = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.kd = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcIEEEAC8B.kdr" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.kdr = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.kdr = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcIEEEAC8B.ke" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.ke = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.ke = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcIEEEAC8B.kir" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.kir = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.kir = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcIEEEAC8B.kpr" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.kpr = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.kpr = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcIEEEAC8B.seve1" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.seve1 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.seve1 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcIEEEAC8B.seve2" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.seve2 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.seve2 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcIEEEAC8B.ta" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.ta = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.ta = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcIEEEAC8B.tdr" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.tdr = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.tdr = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcIEEEAC8B.te" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.te = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.te = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcIEEEAC8B.ve1" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.ve1 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.ve1 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcIEEEAC8B.ve2" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.ve2 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.ve2 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcIEEEAC8B.vemin" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.vemin = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.vemin = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcIEEEAC8B.vfemax" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.vfemax = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.vfemax = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcIEEEAC8B.vrmax" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.vrmax = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.vrmax = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcIEEEAC8B.vrmin" => {
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
