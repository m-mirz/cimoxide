/// Modified IEEE DC2A direct current commutator exciter with speed input, one more leg block in feedback loop and without underexcitation limiters (UEL) inputs. DC type 2 excitation system model with added speed multiplier, added lead-lag, and voltage-dependent limits.
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct ExcDC2A {
    #[serde(flatten)]
    pub base: super::ExcitationSystemDynamics,
    /// Exciter voltage at which exciter saturation is defined (Efd1) (> 0). Typical value = 3,05.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub efd1: Option<f64>,
    /// Exciter voltage at which exciter saturation is defined (Efd2) (> 0). Typical value = 2,29.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub efd2: Option<f64>,
    /// (exclim). IEEE standard is ambiguous about lower limit on exciter output. true = a lower limit of zero is applied to integrator output false = a lower limit of zero is not applied to integrator output. Typical value = true.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclim: Option<bool>,
    /// Voltage regulator gain (Ka) (> 0). Typical value = 300.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ka: Option<f64>,
    /// Exciter constant related to self-excited field (Ke). If Ke is entered as zero, the model calculates an effective value of Ke such that the initial condition value of Vr is zero. The zero value of Ke is not changed. If Ke is entered as non-zero, its value is used directly, without change. Typical value = 1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ke: Option<f64>,
    /// Excitation control system stabilizer gain (Kf) (>= 0). Typical value = 0,1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kf: Option<f64>,
    /// Coefficient to allow different usage of the model-speed coefficient (Ks). Typical value = 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ks: Option<f64>,
    /// Exciter saturation function value at the corresponding exciter voltage, Efd1 (Se[Efd1]) (>= 0). Typical value = 0,279.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seefd1: Option<f64>,
    /// Exciter saturation function value at the corresponding exciter voltage, Efd2 (Se[Efd2]) (>= 0). Typical value = 0,117.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seefd2: Option<f64>,
    /// Voltage regulator time constant (Ta) (> 0). Typical value = 0,01.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ta: Option<f64>,
    /// Voltage regulator time constant (Tb) (>= 0). Typical value = 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tb: Option<f64>,
    /// Voltage regulator time constant (Tc) (>= 0). Typical value = 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tc: Option<f64>,
    /// Exciter time constant, integration rate associated with exciter control (Te) (> 0). Typical value = 1,33.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub te: Option<f64>,
    /// Excitation control system stabilizer time constant (Tf) (> 0). Typical value = 0,675.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tf: Option<f64>,
    /// Excitation control system stabilizer time constant (Tf1) (>= 0). Typical value = 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tf1: Option<f64>,
    /// Maximum voltage regulator output (Vrmax) (> ExcDC2A.vrmin). Typical value = 4,95.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vrmax: Option<f64>,
    /// Minimum voltage regulator output (Vrmin) (< 0 and < ExcDC2A.vrmax). Typical value = -4,9.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vrmin: Option<f64>,
    /// (Vtlim). true = limiter at the block (Ka / [1 + sTa]) is dependent on Vt false = limiter at the block is not dependent on Vt. Typical value = true.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vtlim: Option<bool>,
}
impl crate::base::CimElement for ExcDC2A {
    fn mrid(&self) -> &str { &self.base.base.base.id }
    fn type_name(&self) -> &'static str { "ExcDC2A" }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn to_json_value(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or(serde_json::Value::Null)
    }
    fn to_block(&self) -> crate::base::RdfBlock {
        let mut block = self.base.to_block();
        block.type_name = "ExcDC2A".to_string();
        if let Some(v) = self.efd1 {
            block.fields.insert("ExcDC2A.efd1".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.efd2 {
            block.fields.insert("ExcDC2A.efd2".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.exclim {
            block.fields.insert("ExcDC2A.exclim".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.ka {
            block.fields.insert("ExcDC2A.ka".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.ke {
            block.fields.insert("ExcDC2A.ke".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.kf {
            block.fields.insert("ExcDC2A.kf".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.ks {
            block.fields.insert("ExcDC2A.ks".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.seefd1 {
            block.fields.insert("ExcDC2A.seefd1".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.seefd2 {
            block.fields.insert("ExcDC2A.seefd2".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.ta {
            block.fields.insert("ExcDC2A.ta".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.tb {
            block.fields.insert("ExcDC2A.tb".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.tc {
            block.fields.insert("ExcDC2A.tc".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.te {
            block.fields.insert("ExcDC2A.te".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.tf {
            block.fields.insert("ExcDC2A.tf".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.tf1 {
            block.fields.insert("ExcDC2A.tf1".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.vrmax {
            block.fields.insert("ExcDC2A.vrmax".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.vrmin {
            block.fields.insert("ExcDC2A.vrmin".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.vtlim {
            block.fields.insert("ExcDC2A.vtlim".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        block
    }
}

impl ExcDC2A {
    pub fn from_block(b: &crate::base::RdfBlock) -> Self {
        let mut obj = Self::default();
        obj.base.base.base.id.clone_from(&b.mrid);
        for (key, val) in &b.fields {
            match key.as_str() {
                "ExcDC2A.efd1" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.efd1 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.efd1 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcDC2A.efd2" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.efd2 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.efd2 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcDC2A.exclim" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.exclim = Some(sv.trim() == "true"); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.exclim = Some(sv.trim() == "true"); }
                        }
                        _ => {}
                    }
                }
                "ExcDC2A.ka" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.ka = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.ka = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcDC2A.ke" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.ke = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.ke = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcDC2A.kf" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.kf = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.kf = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcDC2A.ks" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.ks = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.ks = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcDC2A.seefd1" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.seefd1 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.seefd1 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcDC2A.seefd2" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.seefd2 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.seefd2 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcDC2A.ta" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.ta = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.ta = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcDC2A.tb" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.tb = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.tb = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcDC2A.tc" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.tc = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.tc = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcDC2A.te" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.te = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.te = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcDC2A.tf" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.tf = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.tf = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcDC2A.tf1" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.tf1 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.tf1 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcDC2A.vrmax" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.vrmax = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.vrmax = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcDC2A.vrmin" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.vrmin = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.vrmin = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcDC2A.vtlim" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.vtlim = Some(sv.trim() == "true"); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.vtlim = Some(sv.trim() == "true"); }
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
