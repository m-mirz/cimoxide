/// Modified IEEE DC1A direct current commutator exciter with speed input and without underexcitation limiters (UEL) inputs.
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct ExcDC1A {
    #[serde(flatten)]
    pub base: super::ExcitationSystemDynamics,
    /// Exciter voltage at which exciter saturation is defined (Efd1) (> 0). Typical value = 3,1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub efd1: Option<f64>,
    /// Exciter voltage at which exciter saturation is defined (Efd2) (> 0). Typical value = 2,3.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub efd2: Option<f64>,
    /// Maximum voltage exciter output limiter (Efdmax) (> ExcDC1A.efdmin). Typical value = 99.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub efdmax: Option<f64>,
    /// Minimum voltage exciter output limiter (Efdmin) (< ExcDC1A.edfmax). Typical value = -99.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub efdmin: Option<f64>,
    /// (exclim). IEEE standard is ambiguous about lower limit on exciter output. true = a lower limit of zero is applied to integrator output false = a lower limit of zero is not applied to integrator output. Typical value = true.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclim: Option<bool>,
    /// Voltage regulator gain (Ka) (> 0). Typical value = 46.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ka: Option<f64>,
    /// Exciter constant related to self-excited field (Ke). Typical value = 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ke: Option<f64>,
    /// Excitation control system stabilizer gain (Kf) (>= 0). Typical value = 0,1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kf: Option<f64>,
    /// Coefficient to allow different usage of the model-speed coefficient (Ks). Typical value = 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ks: Option<f64>,
    /// Exciter saturation function value at the corresponding exciter voltage, Efd1 (Se[Eefd1]) (>= 0). Typical value = 0,33.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seefd1: Option<f64>,
    /// Exciter saturation function value at the corresponding exciter voltage, Efd2 (Se[Eefd2]) (>= 0). Typical value = 0,1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seefd2: Option<f64>,
    /// Voltage regulator time constant (Ta) (> 0). Typical value = 0,06.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ta: Option<f64>,
    /// Voltage regulator time constant (Tb) (>= 0). Typical value = 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tb: Option<f64>,
    /// Voltage regulator time constant (Tc) (>= 0). Typical value = 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tc: Option<f64>,
    /// Exciter time constant, integration rate associated with exciter control (Te) (> 0). Typical value = 0,46.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub te: Option<f64>,
    /// Excitation control system stabilizer time constant (Tf) (> 0). Typical value = 1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tf: Option<f64>,
    /// Maximum voltage regulator output (Vrmax) (> ExcDC1A.vrmin). Typical value = 1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vrmax: Option<f64>,
    /// Minimum voltage regulator output (Vrmin) (< 0 and < ExcDC1A.vrmax). Typical value = -0,9.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vrmin: Option<f64>,
}
impl crate::base::CimElement for ExcDC1A {
    fn mrid(&self) -> &str { &self.base.base.base.id }
    fn type_name(&self) -> &'static str { "ExcDC1A" }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn to_json_value(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or(serde_json::Value::Null)
    }
    fn to_block(&self) -> crate::base::RdfBlock {
        let mut block = self.base.to_block();
        block.type_name = "ExcDC1A".to_string();
        if let Some(v) = self.efd1 {
            block.fields.insert("ExcDC1A.efd1".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.efd2 {
            block.fields.insert("ExcDC1A.efd2".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.efdmax {
            block.fields.insert("ExcDC1A.efdmax".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.efdmin {
            block.fields.insert("ExcDC1A.efdmin".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.exclim {
            block.fields.insert("ExcDC1A.exclim".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.ka {
            block.fields.insert("ExcDC1A.ka".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.ke {
            block.fields.insert("ExcDC1A.ke".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.kf {
            block.fields.insert("ExcDC1A.kf".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.ks {
            block.fields.insert("ExcDC1A.ks".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.seefd1 {
            block.fields.insert("ExcDC1A.seefd1".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.seefd2 {
            block.fields.insert("ExcDC1A.seefd2".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.ta {
            block.fields.insert("ExcDC1A.ta".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.tb {
            block.fields.insert("ExcDC1A.tb".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.tc {
            block.fields.insert("ExcDC1A.tc".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.te {
            block.fields.insert("ExcDC1A.te".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.tf {
            block.fields.insert("ExcDC1A.tf".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.vrmax {
            block.fields.insert("ExcDC1A.vrmax".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.vrmin {
            block.fields.insert("ExcDC1A.vrmin".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        block
    }
}

impl ExcDC1A {
    pub fn from_block(b: &crate::base::RdfBlock) -> Self {
        let mut obj = Self::default();
        obj.base.base.base.id.clone_from(&b.mrid);
        for (key, val) in &b.fields {
            match key.as_str() {
                "ExcDC1A.efd1" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.efd1 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.efd1 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcDC1A.efd2" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.efd2 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.efd2 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcDC1A.efdmax" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.efdmax = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.efdmax = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcDC1A.efdmin" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.efdmin = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.efdmin = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcDC1A.exclim" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.exclim = Some(sv.trim() == "true"); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.exclim = Some(sv.trim() == "true"); }
                        }
                        _ => {}
                    }
                }
                "ExcDC1A.ka" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.ka = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.ka = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcDC1A.ke" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.ke = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.ke = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcDC1A.kf" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.kf = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.kf = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcDC1A.ks" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.ks = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.ks = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcDC1A.seefd1" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.seefd1 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.seefd1 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcDC1A.seefd2" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.seefd2 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.seefd2 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcDC1A.ta" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.ta = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.ta = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcDC1A.tb" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.tb = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.tb = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcDC1A.tc" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.tc = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.tc = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcDC1A.te" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.te = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.te = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcDC1A.tf" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.tf = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.tf = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcDC1A.vrmax" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.vrmax = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.vrmax = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcDC1A.vrmin" => {
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
