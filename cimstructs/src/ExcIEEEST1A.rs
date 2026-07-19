/// IEEE 421.5-2005 type ST1A model. This model represents systems in which excitation power is supplied through a transformer from the generator terminals (or the unit’s auxiliary bus) and is regulated by a controlled rectifier. The maximum exciter voltage available from such systems is directly related to the generator terminal voltage. Reference: IEEE 421.5-2005, 7.1.
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct ExcIEEEST1A {
    #[serde(flatten)]
    pub base: super::ExcitationSystemDynamics,
    /// Exciter output current limit reference (ILR). Typical value = 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ilr: Option<f64>,
    /// Voltage regulator gain (KA) (> 0). Typical value = 190.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ka: Option<f64>,
    /// Rectifier loading factor proportional to commutating reactance (KC) (>= 0). Typical value = 0,08.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kc: Option<f64>,
    /// Excitation control system stabilizer gains (KF) (>= 0). Typical value = 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kf: Option<f64>,
    /// Exciter output current limiter gain (KLR). Typical value = 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub klr: Option<f64>,
    /// Selector of the Power System Stabilizer (PSS) input (PSSin). true = PSS input (Vs) added to error signal false = PSS input (Vs) added to voltage regulator output. Typical value = true.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pssin: Option<bool>,
    /// Voltage regulator time constant (TA) (>= 0). Typical value = 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ta: Option<f64>,
    /// Voltage regulator time constant (TB) (>= 0). Typical value = 10.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tb: Option<f64>,
    /// Voltage regulator time constant (TB1) (>= 0). Typical value = 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tb1: Option<f64>,
    /// Voltage regulator time constant (TC) (>= 0). Typical value = 1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tc: Option<f64>,
    /// Voltage regulator time constant (TC1) (>= 0). Typical value = 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tc1: Option<f64>,
    /// Excitation control system stabilizer time constant (TF) (>= 0). Typical value = 1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tf: Option<f64>,
    /// Selector of the connection of the UEL input (UELin). Typical value = ignoreUELsignal.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uelin: Option<super::base::UriRef>,
    /// Maximum voltage regulator output (VAMAX) (> 0). Typical value = 14,5.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vamax: Option<f64>,
    /// Minimum voltage regulator output (VAMIN) (< 0). Typical value = -14,5.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vamin: Option<f64>,
    /// Maximum voltage regulator input limit (VIMAX) (> 0). Typical value = 999.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vimax: Option<f64>,
    /// Minimum voltage regulator input limit (VIMIN) (< 0). Typical value = -999.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vimin: Option<f64>,
    /// Maximum voltage regulator outputs (VRMAX) (> 0). Typical value = 7,8.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vrmax: Option<f64>,
    /// Minimum voltage regulator outputs (VRMIN) (< 0). Typical value = -6,7.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vrmin: Option<f64>,
}
impl crate::base::CimElement for ExcIEEEST1A {
    fn mrid(&self) -> &str { &self.base.base.base.id }
    fn type_name(&self) -> &'static str { "ExcIEEEST1A" }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn to_json_value(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or(serde_json::Value::Null)
    }
    fn to_block(&self) -> crate::base::RdfBlock {
        let mut block = self.base.to_block();
        block.type_name = "ExcIEEEST1A".to_string();
        if let Some(v) = self.ilr {
            block.fields.insert("ExcIEEEST1A.ilr".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.ka {
            block.fields.insert("ExcIEEEST1A.ka".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.kc {
            block.fields.insert("ExcIEEEST1A.kc".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.kf {
            block.fields.insert("ExcIEEEST1A.kf".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.klr {
            block.fields.insert("ExcIEEEST1A.klr".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.pssin {
            block.fields.insert("ExcIEEEST1A.pssin".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.ta {
            block.fields.insert("ExcIEEEST1A.ta".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.tb {
            block.fields.insert("ExcIEEEST1A.tb".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.tb1 {
            block.fields.insert("ExcIEEEST1A.tb1".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.tc {
            block.fields.insert("ExcIEEEST1A.tc".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.tc1 {
            block.fields.insert("ExcIEEEST1A.tc1".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.tf {
            block.fields.insert("ExcIEEEST1A.tf".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(ref v) = self.uelin {
            block.fields.insert("ExcIEEEST1A.uelin".into(), crate::base::FieldValue::Resource(v.uri.clone()));
        }
        if let Some(v) = self.vamax {
            block.fields.insert("ExcIEEEST1A.vamax".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.vamin {
            block.fields.insert("ExcIEEEST1A.vamin".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.vimax {
            block.fields.insert("ExcIEEEST1A.vimax".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.vimin {
            block.fields.insert("ExcIEEEST1A.vimin".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.vrmax {
            block.fields.insert("ExcIEEEST1A.vrmax".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.vrmin {
            block.fields.insert("ExcIEEEST1A.vrmin".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        block
    }
}

impl ExcIEEEST1A {
    pub fn from_block(b: &crate::base::RdfBlock) -> Self {
        let mut obj = Self::default();
        obj.base.base.base.id.clone_from(&b.mrid);
        for (key, val) in &b.fields {
            match key.as_str() {
                "ExcIEEEST1A.ilr" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.ilr = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.ilr = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcIEEEST1A.ka" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.ka = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.ka = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcIEEEST1A.kc" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.kc = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.kc = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcIEEEST1A.kf" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.kf = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.kf = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcIEEEST1A.klr" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.klr = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.klr = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcIEEEST1A.pssin" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.pssin = Some(sv.trim() == "true"); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.pssin = Some(sv.trim() == "true"); }
                        }
                        _ => {}
                    }
                }
                "ExcIEEEST1A.ta" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.ta = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.ta = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcIEEEST1A.tb" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.tb = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.tb = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcIEEEST1A.tb1" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.tb1 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.tb1 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcIEEEST1A.tc" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.tc = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.tc = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcIEEEST1A.tc1" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.tc1 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.tc1 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcIEEEST1A.tf" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.tf = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.tf = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcIEEEST1A.uelin" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.uelin = Some(crate::base::UriRef { uri: sv.clone() });
                    }
                }
                "ExcIEEEST1A.vamax" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.vamax = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.vamax = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcIEEEST1A.vamin" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.vamin = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.vamin = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcIEEEST1A.vimax" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.vimax = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.vimax = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcIEEEST1A.vimin" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.vimin = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.vimin = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcIEEEST1A.vrmax" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.vrmax = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.vrmax = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcIEEEST1A.vrmin" => {
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
