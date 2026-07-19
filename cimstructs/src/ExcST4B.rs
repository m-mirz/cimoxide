/// Modified IEEE ST4B static excitation system with maximum inner loop feedback gain Vgmax.
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct ExcST4B {
    #[serde(flatten)]
    pub base: super::ExcitationSystemDynamics,
    /// Rectifier loading factor proportional to commutating reactance (Kc) (>= 0). Typical value = 0,113.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kc: Option<f64>,
    /// Feedback gain constant of the inner loop field regulator (Kg) (>= 0). Typical value = 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kg: Option<f64>,
    /// Potential circuit gain coefficient (Ki) (>= 0). Typical value = 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ki: Option<f64>,
    /// Voltage regulator integral gain output (Kim). Typical value = 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kim: Option<f64>,
    /// Voltage regulator integral gain (Kir). Typical value = 10,75.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kir: Option<f64>,
    /// Potential circuit gain coefficient (Kp) (> 0). Typical value = 9,3.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kp: Option<f64>,
    /// Voltage regulator proportional gain output (Kpm). Typical value = 1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kpm: Option<f64>,
    /// Voltage regulator proportional gain (Kpr). Typical value = 10,75.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kpr: Option<f64>,
    /// Selector (LVGate). true = LVGate is part of the block diagram false = LVGate is not part of the block diagram. Typical value = false.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lvgate: Option<bool>,
    /// Voltage regulator time constant (Ta) (>= 0). Typical value = 0,02.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ta: Option<f64>,
    /// Potential circuit phase angle (thetap). Typical value = 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thetap: Option<f64>,
    /// Selector (UEL). true = UEL is part of block diagram false = UEL is not part of block diagram. Typical value = false.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uel: Option<bool>,
    /// Maximum excitation voltage (Vbmax) (> 0). Typical value = 11,63.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vbmax: Option<f64>,
    /// Maximum inner loop feedback voltage (Vgmax) (>= 0). Typical value = 5,8.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vgmax: Option<f64>,
    /// Maximum inner loop output (Vmmax) (> ExcST4B.vmmin). Typical value = 99.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vmmax: Option<f64>,
    /// Minimum inner loop output (Vmmin) (< ExcST4B.vmmax). Typical value = -99.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vmmin: Option<f64>,
    /// Maximum voltage regulator output (Vrmax) (> 0). Typical value = 1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vrmax: Option<f64>,
    /// Minimum voltage regulator output (Vrmin) (< 0). Typical value = -0,87.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vrmin: Option<f64>,
    /// Reactance associated with potential source (Xl) (>= 0). Typical value = 0,124.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub xl: Option<f64>,
}
impl crate::base::CimElement for ExcST4B {
    fn mrid(&self) -> &str { &self.base.base.base.id }
    fn type_name(&self) -> &'static str { "ExcST4B" }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn to_json_value(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or(serde_json::Value::Null)
    }
    fn to_block(&self) -> crate::base::RdfBlock {
        let mut block = self.base.to_block();
        block.type_name = "ExcST4B".to_string();
        if let Some(v) = self.kc {
            block.fields.insert("ExcST4B.kc".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.kg {
            block.fields.insert("ExcST4B.kg".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.ki {
            block.fields.insert("ExcST4B.ki".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.kim {
            block.fields.insert("ExcST4B.kim".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.kir {
            block.fields.insert("ExcST4B.kir".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.kp {
            block.fields.insert("ExcST4B.kp".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.kpm {
            block.fields.insert("ExcST4B.kpm".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.kpr {
            block.fields.insert("ExcST4B.kpr".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.lvgate {
            block.fields.insert("ExcST4B.lvgate".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.ta {
            block.fields.insert("ExcST4B.ta".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.thetap {
            block.fields.insert("ExcST4B.thetap".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.uel {
            block.fields.insert("ExcST4B.uel".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.vbmax {
            block.fields.insert("ExcST4B.vbmax".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.vgmax {
            block.fields.insert("ExcST4B.vgmax".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.vmmax {
            block.fields.insert("ExcST4B.vmmax".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.vmmin {
            block.fields.insert("ExcST4B.vmmin".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.vrmax {
            block.fields.insert("ExcST4B.vrmax".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.vrmin {
            block.fields.insert("ExcST4B.vrmin".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.xl {
            block.fields.insert("ExcST4B.xl".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        block
    }
}

impl ExcST4B {
    pub fn from_block(b: &crate::base::RdfBlock) -> Self {
        let mut obj = Self::default();
        obj.base.base.base.id.clone_from(&b.mrid);
        for (key, val) in &b.fields {
            match key.as_str() {
                "ExcST4B.kc" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.kc = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.kc = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcST4B.kg" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.kg = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.kg = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcST4B.ki" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.ki = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.ki = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcST4B.kim" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.kim = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.kim = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcST4B.kir" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.kir = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.kir = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcST4B.kp" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.kp = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.kp = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcST4B.kpm" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.kpm = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.kpm = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcST4B.kpr" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.kpr = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.kpr = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcST4B.lvgate" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.lvgate = Some(sv.trim() == "true"); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.lvgate = Some(sv.trim() == "true"); }
                        }
                        _ => {}
                    }
                }
                "ExcST4B.ta" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.ta = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.ta = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcST4B.thetap" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.thetap = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.thetap = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcST4B.uel" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.uel = Some(sv.trim() == "true"); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.uel = Some(sv.trim() == "true"); }
                        }
                        _ => {}
                    }
                }
                "ExcST4B.vbmax" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.vbmax = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.vbmax = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcST4B.vgmax" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.vgmax = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.vgmax = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcST4B.vmmax" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.vmmax = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.vmmax = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcST4B.vmmin" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.vmmin = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.vmmin = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcST4B.vrmax" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.vrmax = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.vrmax = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcST4B.vrmin" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.vrmin = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.vrmin = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcST4B.xl" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.xl = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.xl = Some(v); } }
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
