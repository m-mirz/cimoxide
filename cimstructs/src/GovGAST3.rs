/// Generic turbogas with acceleration and temperature controller.
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct GovGAST3 {
    #[serde(flatten)]
    pub base: super::TurbineGovernorDynamics,
    /// Acceleration limit set-point (Bca). Unit = 1/s. Typical value = 0,01.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bca: Option<f64>,
    /// Droop (bp). Typical value = 0,05.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bp: Option<f64>,
    /// Exhaust temperature variation due to fuel flow increasing from 0 to 1 PU (deltaTc). Typical value = 390.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dtc: Option<f64>,
    /// Minimum fuel flow (Ka). Typical value = 0,23.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ka: Option<f64>,
    /// Fuel system feedback (KAC). Typical value = 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kac: Option<f64>,
    /// Acceleration control integral gain (Kca). Unit = 1/s. Typical value = 100.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kca: Option<f64>,
    /// Gain of radiation shield (Ksi). Typical value = 0,8.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ksi: Option<f64>,
    /// Coefficient of transfer function of fuel valve positioner (Ky). Typical value = 1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ky: Option<f64>,
    /// Fuel flow maximum negative error value (MNef). Typical value = -0,05.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mnef: Option<f64>,
    /// Fuel flow maximum positive error value (MXef). Typical value = 0,05.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mxef: Option<f64>,
    /// Minimum fuel flow (RCMN). Typical value = -0,1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rcmn: Option<f64>,
    /// Maximum fuel flow (RCMX). Typical value = 1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rcmx: Option<f64>,
    /// Fuel control time constant (Tac) (>= 0). Typical value = 0,1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tac: Option<f64>,
    /// Compressor discharge volume time constant (Tc) (>= 0). Typical value = 0,2.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tc: Option<f64>,
    /// Temperature controller derivative gain (Td) (>= 0). Typical value = 3,3.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub td: Option<f64>,
    /// Turbine rated exhaust temperature correspondent to Pm=1 PU (Tfen). Typical value = 540.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tfen: Option<f64>,
    /// Time constant of speed governor (Tg) (>= 0). Typical value = 0,05.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tg: Option<f64>,
    /// Time constant of radiation shield (Tsi) (>= 0). Typical value = 15.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tsi: Option<f64>,
    /// Temperature controller integration rate (Tt). Typical value = 250.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tt: Option<f64>,
    /// Time constant of thermocouple (Ttc) (>= 0). Typical value = 2,5.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ttc: Option<f64>,
    /// Time constant of fuel valve positioner (Ty) (>= 0). Typical value = 0,2.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ty: Option<f64>,
}
impl crate::base::CimElement for GovGAST3 {
    fn mrid(&self) -> &str { &self.base.base.base.id }
    fn type_name(&self) -> &'static str { "GovGAST3" }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn to_json_value(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or(serde_json::Value::Null)
    }
    fn to_block(&self) -> crate::base::RdfBlock {
        let mut block = self.base.to_block();
        block.type_name = "GovGAST3".to_string();
        if let Some(v) = self.bca {
            block.fields.insert("GovGAST3.bca".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.bp {
            block.fields.insert("GovGAST3.bp".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.dtc {
            block.fields.insert("GovGAST3.dtc".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.ka {
            block.fields.insert("GovGAST3.ka".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.kac {
            block.fields.insert("GovGAST3.kac".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.kca {
            block.fields.insert("GovGAST3.kca".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.ksi {
            block.fields.insert("GovGAST3.ksi".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.ky {
            block.fields.insert("GovGAST3.ky".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.mnef {
            block.fields.insert("GovGAST3.mnef".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.mxef {
            block.fields.insert("GovGAST3.mxef".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.rcmn {
            block.fields.insert("GovGAST3.rcmn".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.rcmx {
            block.fields.insert("GovGAST3.rcmx".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.tac {
            block.fields.insert("GovGAST3.tac".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.tc {
            block.fields.insert("GovGAST3.tc".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.td {
            block.fields.insert("GovGAST3.td".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.tfen {
            block.fields.insert("GovGAST3.tfen".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.tg {
            block.fields.insert("GovGAST3.tg".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.tsi {
            block.fields.insert("GovGAST3.tsi".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.tt {
            block.fields.insert("GovGAST3.tt".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.ttc {
            block.fields.insert("GovGAST3.ttc".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.ty {
            block.fields.insert("GovGAST3.ty".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        block
    }
}

impl GovGAST3 {
    pub fn from_block(b: &crate::base::RdfBlock) -> Self {
        let mut obj = Self::default();
        obj.base.base.base.id.clone_from(&b.mrid);
        for (key, val) in &b.fields {
            match key.as_str() {
                "GovGAST3.bca" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.bca = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.bca = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovGAST3.bp" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.bp = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.bp = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovGAST3.dtc" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.dtc = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.dtc = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovGAST3.ka" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.ka = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.ka = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovGAST3.kac" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.kac = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.kac = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovGAST3.kca" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.kca = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.kca = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovGAST3.ksi" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.ksi = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.ksi = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovGAST3.ky" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.ky = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.ky = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovGAST3.mnef" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.mnef = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.mnef = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovGAST3.mxef" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.mxef = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.mxef = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovGAST3.rcmn" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.rcmn = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.rcmn = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovGAST3.rcmx" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.rcmx = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.rcmx = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovGAST3.tac" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.tac = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.tac = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovGAST3.tc" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.tc = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.tc = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovGAST3.td" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.td = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.td = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovGAST3.tfen" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.tfen = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.tfen = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovGAST3.tg" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.tg = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.tg = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovGAST3.tsi" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.tsi = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.tsi = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovGAST3.tt" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.tt = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.tt = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovGAST3.ttc" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.ttc = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.ttc = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovGAST3.ty" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.ty = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.ty = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "TurbineGovernorDynamics.AsynchronousMachineDynamics" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.base.asynchronous_machine_dynamics = Some(crate::base::MridRef { mrid: sv.clone() });
                    }
                }
                "TurbineGovernorDynamics.SynchronousMachineDynamics" => {
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
