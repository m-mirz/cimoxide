/// Proportional/integral regulator excitation system. This model can be used to represent excitation systems with a proportional-integral (PI) voltage regulator controller.
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct ExcPIC {
    #[serde(flatten)]
    pub base: super::ExcitationSystemDynamics,
    /// Field voltage value 1 (E1). Typical value = 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub e1: Option<f64>,
    /// Field voltage value 2 (E2). Typical value = 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub e2: Option<f64>,
    /// Exciter maximum limit (Efdmax) (> ExcPIC.efdmin). Typical value = 8.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub efdmax: Option<f64>,
    /// Exciter minimum limit (Efdmin) (< ExcPIC.efdmax). Typical value = -0,87.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub efdmin: Option<f64>,
    /// PI controller gain (Ka). Typical value = 3,15.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ka: Option<f64>,
    /// Exciter regulation factor (Kc). Typical value = 0,08.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kc: Option<f64>,
    /// Exciter constant (Ke). Typical value = 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ke: Option<f64>,
    /// Rate feedback gain (Kf). Typical value = 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kf: Option<f64>,
    /// Current source gain (Ki). Typical value = 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ki: Option<f64>,
    /// Potential source gain (Kp). Typical value = 6,5.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kp: Option<f64>,
    /// Saturation factor at E1 (Se1). Typical value = 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub se1: Option<f64>,
    /// Saturation factor at E2 (Se2). Typical value = 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub se2: Option<f64>,
    /// PI controller time constant (Ta1) (>= 0). Typical value = 1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ta1: Option<f64>,
    /// Voltage regulator time constant (Ta2) (>= 0). Typical value = 0,01.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ta2: Option<f64>,
    /// Lead time constant (Ta3) (>= 0). Typical value = 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ta3: Option<f64>,
    /// Lag time constant (Ta4) (>= 0). Typical value = 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ta4: Option<f64>,
    /// Exciter time constant (Te) (>= 0). Typical value = 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub te: Option<f64>,
    /// Rate feedback time constant (Tf1) (>= 0). Typical value = 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tf1: Option<f64>,
    /// Rate feedback lag time constant (Tf2) (>= 0). Typical value = 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tf2: Option<f64>,
    /// PI maximum limit (Vr1). Typical value = 1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vr1: Option<f64>,
    /// PI minimum limit (Vr2). Typical value = -0,87.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vr2: Option<f64>,
    /// Voltage regulator maximum limit (Vrmax) (> ExcPIC.vrmin). Typical value = 1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vrmax: Option<f64>,
    /// Voltage regulator minimum limit (Vrmin) (< ExcPIC.vrmax). Typical value = -0,87.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vrmin: Option<f64>,
}
impl crate::base::CimElement for ExcPIC {
    fn mrid(&self) -> &str { &self.base.base.base.id }
    fn type_name(&self) -> &'static str { "ExcPIC" }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn to_json_value(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or(serde_json::Value::Null)
    }
    fn to_block(&self) -> crate::base::RdfBlock {
        let mut block = self.base.to_block();
        block.type_name = "ExcPIC".to_string();
        if let Some(v) = self.e1 {
            block.fields.insert("ExcPIC.e1".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.e2 {
            block.fields.insert("ExcPIC.e2".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.efdmax {
            block.fields.insert("ExcPIC.efdmax".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.efdmin {
            block.fields.insert("ExcPIC.efdmin".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.ka {
            block.fields.insert("ExcPIC.ka".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.kc {
            block.fields.insert("ExcPIC.kc".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.ke {
            block.fields.insert("ExcPIC.ke".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.kf {
            block.fields.insert("ExcPIC.kf".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.ki {
            block.fields.insert("ExcPIC.ki".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.kp {
            block.fields.insert("ExcPIC.kp".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.se1 {
            block.fields.insert("ExcPIC.se1".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.se2 {
            block.fields.insert("ExcPIC.se2".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.ta1 {
            block.fields.insert("ExcPIC.ta1".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.ta2 {
            block.fields.insert("ExcPIC.ta2".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.ta3 {
            block.fields.insert("ExcPIC.ta3".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.ta4 {
            block.fields.insert("ExcPIC.ta4".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.te {
            block.fields.insert("ExcPIC.te".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.tf1 {
            block.fields.insert("ExcPIC.tf1".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.tf2 {
            block.fields.insert("ExcPIC.tf2".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.vr1 {
            block.fields.insert("ExcPIC.vr1".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.vr2 {
            block.fields.insert("ExcPIC.vr2".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.vrmax {
            block.fields.insert("ExcPIC.vrmax".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.vrmin {
            block.fields.insert("ExcPIC.vrmin".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        block
    }
}

impl ExcPIC {
    pub fn from_block(b: &crate::base::RdfBlock) -> Self {
        let mut obj = Self::default();
        obj.base.base.base.id.clone_from(&b.mrid);
        for (key, val) in &b.fields {
            match key.as_str() {
                "ExcPIC.e1" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.e1 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.e1 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcPIC.e2" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.e2 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.e2 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcPIC.efdmax" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.efdmax = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.efdmax = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcPIC.efdmin" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.efdmin = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.efdmin = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcPIC.ka" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.ka = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.ka = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcPIC.kc" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.kc = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.kc = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcPIC.ke" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.ke = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.ke = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcPIC.kf" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.kf = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.kf = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcPIC.ki" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.ki = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.ki = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcPIC.kp" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.kp = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.kp = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcPIC.se1" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.se1 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.se1 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcPIC.se2" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.se2 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.se2 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcPIC.ta1" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.ta1 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.ta1 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcPIC.ta2" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.ta2 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.ta2 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcPIC.ta3" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.ta3 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.ta3 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcPIC.ta4" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.ta4 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.ta4 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcPIC.te" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.te = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.te = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcPIC.tf1" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.tf1 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.tf1 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcPIC.tf2" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.tf2 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.tf2 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcPIC.vr1" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.vr1 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.vr1 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcPIC.vr2" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.vr2 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.vr2 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcPIC.vrmax" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.vrmax = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.vrmax = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcPIC.vrmin" => {
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
