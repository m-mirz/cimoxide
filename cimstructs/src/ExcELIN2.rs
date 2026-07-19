/// Detailed excitation system ELIN (VATECH). This model represents an all-static excitation system. A PI voltage controller establishes a desired field current set point for a proportional current controller. The integrator of the PI controller has a follow-up input to match its signal to the present field current. Power system stabilizer models used in conjunction with this excitation system model: PssELIN2, PssIEEE2B, Pss2B.
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct ExcELIN2 {
    #[serde(flatten)]
    pub base: super::ExcitationSystemDynamics,
    /// Gain (Efdbas). Typical value = 0,1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub efdbas: Option<f64>,
    /// Limiter (Iefmax) (> ExcELIN2.iefmin). Typical value = 1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iefmax: Option<f64>,
    /// Minimum open circuit excitation voltage (Iefmax2). Typical value = -5.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iefmax2: Option<f64>,
    /// Limiter (Iefmin) (< ExcELIN2.iefmax). Typical value = 1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iefmin: Option<f64>,
    /// Voltage regulator input gain (K1). Typical value = 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub k1: Option<f64>,
    /// Voltage regulator input limit (K1ec). Typical value = 2.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub k1ec: Option<f64>,
    /// Gain (K2). Typical value = 5.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub k2: Option<f64>,
    /// Gain (K3). Typical value = 0,1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub k3: Option<f64>,
    /// Gain (K4). Typical value = 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub k4: Option<f64>,
    /// Voltage controller derivative gain (Kd1). Typical value = 34,5.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kd1: Option<f64>,
    /// Gain (Ke2). Typical value = 0,1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ke2: Option<f64>,
    /// Gain (Ketb). Typical value = 0,06.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ketb: Option<f64>,
    /// Controller follow up gain (PID1max). Typical value = 2.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pid1max: Option<f64>,
    /// Exciter saturation function value at the corresponding exciter voltage, Ve1, back of commutating reactance (Se[Ve1]) (>= 0). Typical value = 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seve1: Option<f64>,
    /// Exciter saturation function value at the corresponding exciter voltage, Ve2, back of commutating reactance (Se[Ve2]) (>= 0). Typical value = 1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seve2: Option<f64>,
    /// Voltage controller derivative washout time constant (Tb1) (>= 0). Typical value = 12,45.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tb1: Option<f64>,
    /// Time constant (Te) (>= 0). Typical value = 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub te: Option<f64>,
    /// Time Constant (Te2) (>= 0). Typical value = 1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub te2: Option<f64>,
    /// Controller follow up deadband (Ti1). Typical value = 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ti1: Option<f64>,
    /// Time constant (Ti3) (>= 0). Typical value = 3.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ti3: Option<f64>,
    /// Time constant (Ti4) (>= 0). Typical value = 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ti4: Option<f64>,
    /// Time constant (Tr4) (>= 0). Typical value = 1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tr4: Option<f64>,
    /// Limiter (Upmax) (> ExcELIN2.upmin). Typical value = 3.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upmax: Option<f64>,
    /// Limiter (Upmin) (< ExcELIN2.upmax). Typical value = 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upmin: Option<f64>,
    /// Exciter alternator output voltages back of commutating reactance at which saturation is defined (Ve1) (> 0). Typical value = 3.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ve1: Option<f64>,
    /// Exciter alternator output voltages back of commutating reactance at which saturation is defined (Ve2) (> 0). Typical value = 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ve2: Option<f64>,
    /// Excitation transformer effective reactance (Xp). Typical value = 1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub xp: Option<f64>,
}
impl crate::base::CimElement for ExcELIN2 {
    fn mrid(&self) -> &str { &self.base.base.base.id }
    fn type_name(&self) -> &'static str { "ExcELIN2" }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn to_json_value(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or(serde_json::Value::Null)
    }
    fn to_block(&self) -> crate::base::RdfBlock {
        let mut block = self.base.to_block();
        block.type_name = "ExcELIN2".to_string();
        if let Some(v) = self.efdbas {
            block.fields.insert("ExcELIN2.efdbas".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.iefmax {
            block.fields.insert("ExcELIN2.iefmax".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.iefmax2 {
            block.fields.insert("ExcELIN2.iefmax2".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.iefmin {
            block.fields.insert("ExcELIN2.iefmin".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.k1 {
            block.fields.insert("ExcELIN2.k1".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.k1ec {
            block.fields.insert("ExcELIN2.k1ec".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.k2 {
            block.fields.insert("ExcELIN2.k2".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.k3 {
            block.fields.insert("ExcELIN2.k3".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.k4 {
            block.fields.insert("ExcELIN2.k4".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.kd1 {
            block.fields.insert("ExcELIN2.kd1".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.ke2 {
            block.fields.insert("ExcELIN2.ke2".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.ketb {
            block.fields.insert("ExcELIN2.ketb".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.pid1max {
            block.fields.insert("ExcELIN2.pid1max".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.seve1 {
            block.fields.insert("ExcELIN2.seve1".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.seve2 {
            block.fields.insert("ExcELIN2.seve2".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.tb1 {
            block.fields.insert("ExcELIN2.tb1".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.te {
            block.fields.insert("ExcELIN2.te".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.te2 {
            block.fields.insert("ExcELIN2.te2".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.ti1 {
            block.fields.insert("ExcELIN2.ti1".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.ti3 {
            block.fields.insert("ExcELIN2.ti3".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.ti4 {
            block.fields.insert("ExcELIN2.ti4".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.tr4 {
            block.fields.insert("ExcELIN2.tr4".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.upmax {
            block.fields.insert("ExcELIN2.upmax".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.upmin {
            block.fields.insert("ExcELIN2.upmin".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.ve1 {
            block.fields.insert("ExcELIN2.ve1".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.ve2 {
            block.fields.insert("ExcELIN2.ve2".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.xp {
            block.fields.insert("ExcELIN2.xp".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        block
    }
}

impl ExcELIN2 {
    pub fn from_block(b: &crate::base::RdfBlock) -> Self {
        let mut obj = Self::default();
        obj.base.base.base.id.clone_from(&b.mrid);
        for (key, val) in &b.fields {
            match key.as_str() {
                "ExcELIN2.efdbas" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.efdbas = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.efdbas = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcELIN2.iefmax" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.iefmax = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.iefmax = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcELIN2.iefmax2" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.iefmax2 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.iefmax2 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcELIN2.iefmin" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.iefmin = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.iefmin = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcELIN2.k1" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.k1 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.k1 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcELIN2.k1ec" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.k1ec = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.k1ec = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcELIN2.k2" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.k2 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.k2 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcELIN2.k3" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.k3 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.k3 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcELIN2.k4" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.k4 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.k4 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcELIN2.kd1" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.kd1 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.kd1 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcELIN2.ke2" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.ke2 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.ke2 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcELIN2.ketb" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.ketb = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.ketb = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcELIN2.pid1max" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.pid1max = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.pid1max = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcELIN2.seve1" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.seve1 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.seve1 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcELIN2.seve2" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.seve2 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.seve2 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcELIN2.tb1" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.tb1 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.tb1 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcELIN2.te" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.te = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.te = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcELIN2.te2" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.te2 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.te2 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcELIN2.ti1" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.ti1 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.ti1 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcELIN2.ti3" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.ti3 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.ti3 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcELIN2.ti4" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.ti4 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.ti4 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcELIN2.tr4" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.tr4 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.tr4 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcELIN2.upmax" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.upmax = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.upmax = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcELIN2.upmin" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.upmin = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.upmin = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcELIN2.ve1" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.ve1 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.ve1 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcELIN2.ve2" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.ve2 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.ve2 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcELIN2.xp" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.xp = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.xp = Some(v); } }
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
