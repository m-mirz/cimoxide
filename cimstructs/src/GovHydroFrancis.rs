/// Detailed hydro unit - Francis model. This model can be used to represent three types of governors. A schematic of the hydraulic system of detailed hydro unit models, such as Francis and Pelton, is provided in the DetailedHydroModelHydraulicSystem diagram.
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct GovHydroFrancis {
    #[serde(flatten)]
    pub base: super::TurbineGovernorDynamics,
    /// Opening section SEFF at the maximum efficiency (Am). Typical value = 0,7.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub am: Option<f64>,
    /// Area of the surge tank (AV0). Unit = m2. Typical value = 30.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub av0: Option<f64>,
    /// Area of the compensation tank (AV1). Unit = m2. Typical value = 700.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub av1: Option<f64>,
    /// Droop (Bp). Typical value = 0,05.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bp: Option<f64>,
    /// Intentional dead-band width (DB1). Unit = Hz. Typical value = 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db1: Option<f64>,
    /// Maximum efficiency (EtaMax). Typical value = 1,05.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub etamax: Option<f64>,
    /// Governor control flag (Cflag). Typical value = mechanicHydrolicTachoAccelerator.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub governor_control: Option<super::base::UriRef>,
    /// Head of compensation chamber water level with respect to the level of penstock (H1). Unit = km. Typical value = 0,004.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub h1: Option<f64>,
    /// Head of surge tank water level with respect to the level of penstock (H2). Unit = km. Typical value = 0,040.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub h2: Option<f64>,
    /// Rated hydraulic head (Hn). Unit = km. Typical value = 0,250.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hn: Option<f64>,
    /// Penstock loss coefficient (due to friction) (Kc). Typical value = 0,025.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kc: Option<f64>,
    /// Water tunnel and surge chamber loss coefficient (due to friction) (Kg). Typical value = 0,025.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kg: Option<f64>,
    /// Washout gain (Kt). Typical value = 0,25.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kt: Option<f64>,
    /// No-load turbine flow at nominal head (Qc0). Typical value = 0,1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qc0: Option<f64>,
    /// Rated flow (Qn). Unit = m3/s. Typical value = 250.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qn: Option<f64>,
    /// Derivative gain (Ta) (>= 0). Typical value = 3.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ta: Option<f64>,
    /// Washout time constant (Td) (>= 0). Typical value = 6.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub td: Option<f64>,
    /// Gate servo time constant (Ts) (>= 0). Typical value = 0,5.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ts: Option<f64>,
    /// Water inertia time constant (Twnc) (>= 0). Typical value = 1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub twnc: Option<f64>,
    /// Water tunnel and surge chamber inertia time constant (Twng) (>= 0). Typical value = 3.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub twng: Option<f64>,
    /// Derivative feedback gain (Tx) (>= 0). Typical value = 1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tx: Option<f64>,
    /// Maximum gate opening velocity (Va). Unit = PU / s. Typical value = 0,06.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub va: Option<f64>,
    /// Maximum gate opening (ValvMax) (> GovHydroFrancis.valvmin). Typical value = 1,1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valvmax: Option<f64>,
    /// Minimum gate opening (ValvMin) (< GovHydroFrancis.valvmax). Typical value = 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valvmin: Option<f64>,
    /// Maximum gate closing velocity (Vc). Unit = PU / s. Typical value = -0,06.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vc: Option<f64>,
    /// Water tunnel and surge chamber simulation (Tflag). true = enable of water tunnel and surge chamber simulation false = inhibit of water tunnel and surge chamber simulation. Typical value = false.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub water_tunnel_surge_chamber_simulation: Option<bool>,
    /// Head of upper water level with respect to the level of penstock (Zsfc). Unit = km. Typical value = 0,025.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zsfc: Option<f64>,
}
impl crate::base::CimElement for GovHydroFrancis {
    fn mrid(&self) -> &str { &self.base.base.base.id }
    fn type_name(&self) -> &'static str { "GovHydroFrancis" }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn to_json_value(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or(serde_json::Value::Null)
    }
    fn to_block(&self) -> crate::base::RdfBlock {
        let mut block = self.base.to_block();
        block.type_name = "GovHydroFrancis".to_string();
        if let Some(v) = self.am {
            block.fields.insert("GovHydroFrancis.am".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.av0 {
            block.fields.insert("GovHydroFrancis.av0".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.av1 {
            block.fields.insert("GovHydroFrancis.av1".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.bp {
            block.fields.insert("GovHydroFrancis.bp".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.db1 {
            block.fields.insert("GovHydroFrancis.db1".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.etamax {
            block.fields.insert("GovHydroFrancis.etamax".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(ref v) = self.governor_control {
            block.fields.insert("GovHydroFrancis.governorControl".into(), crate::base::FieldValue::Resource(v.uri.clone()));
        }
        if let Some(v) = self.h1 {
            block.fields.insert("GovHydroFrancis.h1".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.h2 {
            block.fields.insert("GovHydroFrancis.h2".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.hn {
            block.fields.insert("GovHydroFrancis.hn".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.kc {
            block.fields.insert("GovHydroFrancis.kc".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.kg {
            block.fields.insert("GovHydroFrancis.kg".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.kt {
            block.fields.insert("GovHydroFrancis.kt".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.qc0 {
            block.fields.insert("GovHydroFrancis.qc0".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.qn {
            block.fields.insert("GovHydroFrancis.qn".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.ta {
            block.fields.insert("GovHydroFrancis.ta".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.td {
            block.fields.insert("GovHydroFrancis.td".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.ts {
            block.fields.insert("GovHydroFrancis.ts".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.twnc {
            block.fields.insert("GovHydroFrancis.twnc".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.twng {
            block.fields.insert("GovHydroFrancis.twng".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.tx {
            block.fields.insert("GovHydroFrancis.tx".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.va {
            block.fields.insert("GovHydroFrancis.va".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.valvmax {
            block.fields.insert("GovHydroFrancis.valvmax".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.valvmin {
            block.fields.insert("GovHydroFrancis.valvmin".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.vc {
            block.fields.insert("GovHydroFrancis.vc".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.water_tunnel_surge_chamber_simulation {
            block.fields.insert("GovHydroFrancis.waterTunnelSurgeChamberSimulation".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.zsfc {
            block.fields.insert("GovHydroFrancis.zsfc".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        block
    }
}

impl GovHydroFrancis {
    pub fn from_block(b: &crate::base::RdfBlock) -> Self {
        let mut obj = Self::default();
        obj.base.base.base.id.clone_from(&b.mrid);
        for (key, val) in &b.fields {
            match key.as_str() {
                "GovHydroFrancis.am" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.am = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.am = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovHydroFrancis.av0" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.av0 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.av0 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovHydroFrancis.av1" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.av1 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.av1 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovHydroFrancis.bp" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.bp = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.bp = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovHydroFrancis.db1" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.db1 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.db1 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovHydroFrancis.etamax" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.etamax = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.etamax = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovHydroFrancis.governorControl" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.governor_control = Some(crate::base::UriRef { uri: sv.clone() });
                    }
                }
                "GovHydroFrancis.h1" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.h1 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.h1 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovHydroFrancis.h2" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.h2 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.h2 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovHydroFrancis.hn" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.hn = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.hn = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovHydroFrancis.kc" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.kc = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.kc = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovHydroFrancis.kg" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.kg = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.kg = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovHydroFrancis.kt" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.kt = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.kt = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovHydroFrancis.qc0" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.qc0 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.qc0 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovHydroFrancis.qn" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.qn = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.qn = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovHydroFrancis.ta" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.ta = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.ta = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovHydroFrancis.td" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.td = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.td = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovHydroFrancis.ts" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.ts = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.ts = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovHydroFrancis.twnc" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.twnc = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.twnc = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovHydroFrancis.twng" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.twng = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.twng = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovHydroFrancis.tx" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.tx = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.tx = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovHydroFrancis.va" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.va = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.va = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovHydroFrancis.valvmax" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.valvmax = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.valvmax = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovHydroFrancis.valvmin" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.valvmin = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.valvmin = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovHydroFrancis.vc" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.vc = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.vc = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovHydroFrancis.waterTunnelSurgeChamberSimulation" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.water_tunnel_surge_chamber_simulation = Some(sv.trim() == "true"); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.water_tunnel_surge_chamber_simulation = Some(sv.trim() == "true"); }
                        }
                        _ => {}
                    }
                }
                "GovHydroFrancis.zsfc" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.zsfc = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.zsfc = Some(v); } }
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
