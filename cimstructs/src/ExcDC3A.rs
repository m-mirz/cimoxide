/// Modified IEEE DC3A direct current commutator exciter with speed input, and deadband. DC old type 4.
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct ExcDC3A {
    #[serde(flatten)]
    pub base: super::ExcitationSystemDynamics,
    /// Exciter voltage at which exciter saturation is defined (Efd1) (> 0). Typical value = 2,6.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub efd1: Option<f64>,
    /// Exciter voltage at which exciter saturation is defined (Efd2) (> 0). Typical value = 3,45.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub efd2: Option<f64>,
    /// (Efdlim). true = exciter output limiter is active false = exciter output limiter not active. Typical value = true.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub efdlim: Option<bool>,
    /// Maximum voltage exciter output limiter (Efdmax) (> ExcDC3A.efdmin). Typical value = 99.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub efdmax: Option<f64>,
    /// Minimum voltage exciter output limiter (Efdmin) (< ExcDC3A.efdmax). Typical value = -99.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub efdmin: Option<f64>,
    /// (exclim). IEEE standard is ambiguous about lower limit on exciter output. true = a lower limit of zero is applied to integrator output false = a lower limit of zero not applied to integrator output. Typical value = true.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclim: Option<bool>,
    /// Exciter constant related to self-excited field (Ke). Typical value = 1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ke: Option<f64>,
    /// Deadband (Kr). Typical value = 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kr: Option<f64>,
    /// Coefficient to allow different usage of the model-speed coefficient (Ks). Typical value = 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ks: Option<f64>,
    /// Fast raise/lower contact setting (Kv) (> 0). Typical value = 0,05.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kv: Option<f64>,
    /// Exciter saturation function value at the corresponding exciter voltage, Efd1 (Se[Efd1]) (>= 0). Typical value = 0,1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seefd1: Option<f64>,
    /// Exciter saturation function value at the corresponding exciter voltage, Efd2 (Se[Efd2]) (>= 0). Typical value = 0,35.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seefd2: Option<f64>,
    /// Exciter time constant, integration rate associated with exciter control (Te) (> 0). Typical value = 1,83.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub te: Option<f64>,
    /// Rheostat travel time (Trh) (> 0). Typical value = 20.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trh: Option<f64>,
    /// Maximum voltage regulator output (Vrmax) (> 0). Typical value = 5.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vrmax: Option<f64>,
    /// Minimum voltage regulator output (Vrmin) (<= 0). Typical value = 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vrmin: Option<f64>,
}
impl crate::base::CimElement for ExcDC3A {
    fn mrid(&self) -> &str { &self.base.base.base.id }
    fn type_name(&self) -> &'static str { "ExcDC3A" }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn to_json_value(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or(serde_json::Value::Null)
    }
    fn to_block(&self) -> crate::base::RdfBlock {
        let mut block = self.base.to_block();
        block.type_name = "ExcDC3A".to_string();
        if let Some(v) = self.efd1 {
            block.fields.insert("ExcDC3A.efd1".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.efd2 {
            block.fields.insert("ExcDC3A.efd2".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.efdlim {
            block.fields.insert("ExcDC3A.efdlim".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.efdmax {
            block.fields.insert("ExcDC3A.efdmax".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.efdmin {
            block.fields.insert("ExcDC3A.efdmin".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.exclim {
            block.fields.insert("ExcDC3A.exclim".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.ke {
            block.fields.insert("ExcDC3A.ke".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.kr {
            block.fields.insert("ExcDC3A.kr".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.ks {
            block.fields.insert("ExcDC3A.ks".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.kv {
            block.fields.insert("ExcDC3A.kv".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.seefd1 {
            block.fields.insert("ExcDC3A.seefd1".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.seefd2 {
            block.fields.insert("ExcDC3A.seefd2".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.te {
            block.fields.insert("ExcDC3A.te".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.trh {
            block.fields.insert("ExcDC3A.trh".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.vrmax {
            block.fields.insert("ExcDC3A.vrmax".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.vrmin {
            block.fields.insert("ExcDC3A.vrmin".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        block
    }
}

impl ExcDC3A {
    pub fn from_block(b: &crate::base::RdfBlock) -> Self {
        let mut obj = Self::default();
        obj.base.base.base.id.clone_from(&b.mrid);
        for (key, val) in &b.fields {
            match key.as_str() {
                "ExcDC3A.efd1" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.efd1 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.efd1 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcDC3A.efd2" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.efd2 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.efd2 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcDC3A.efdlim" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.efdlim = Some(sv.trim() == "true"); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.efdlim = Some(sv.trim() == "true"); }
                        }
                        _ => {}
                    }
                }
                "ExcDC3A.efdmax" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.efdmax = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.efdmax = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcDC3A.efdmin" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.efdmin = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.efdmin = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcDC3A.exclim" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.exclim = Some(sv.trim() == "true"); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.exclim = Some(sv.trim() == "true"); }
                        }
                        _ => {}
                    }
                }
                "ExcDC3A.ke" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.ke = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.ke = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcDC3A.kr" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.kr = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.kr = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcDC3A.ks" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.ks = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.ks = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcDC3A.kv" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.kv = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.kv = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcDC3A.seefd1" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.seefd1 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.seefd1 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcDC3A.seefd2" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.seefd2 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.seefd2 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcDC3A.te" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.te = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.te = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcDC3A.trh" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.trh = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.trh = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcDC3A.vrmax" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.vrmax = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.vrmax = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcDC3A.vrmin" => {
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
