/// Detailed hydro unit - Pelton model. This model can be used to represent the dynamic related to water tunnel and surge chamber. The DetailedHydroModelHydraulicSystem diagram, located under the GovHydroFrancis class, provides a schematic of the hydraulic system of detailed hydro unit models, such as Francis and Pelton.
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct GovHydroPelton {
    #[serde(flatten)]
    pub base: super::TurbineGovernorDynamics,
    /// Area of the surge tank (AV0). Unit = m2. Typical value = 30.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub av0: Option<f64>,
    /// Area of the compensation tank (AV1). Unit = m2. Typical value = 700.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub av1: Option<f64>,
    /// Droop (bp). Typical value = 0,05.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bp: Option<f64>,
    /// Intentional dead-band width (DB1). Unit = Hz. Typical value = 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db1: Option<f64>,
    /// Intentional dead-band width of valve opening error (DB2). Unit = Hz. Typical value = 0,01.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db2: Option<f64>,
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
    /// No-load turbine flow at nominal head (Qc0). Typical value = 0,05.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qc0: Option<f64>,
    /// Rated flow (Qn). Unit = m3/s. Typical value = 250.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qn: Option<f64>,
    /// Simplified Pelton model simulation (Sflag). true = enable of simplified Pelton model simulation false = enable of complete Pelton model simulation (non-linear gain). Typical value = true.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub simplified_pelton: Option<bool>,
    /// Static compensating characteristic (Cflag). It should be true if simplifiedPelton = false. true = enable of static compensating characteristic false = inhibit of static compensating characteristic. Typical value = false.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub static_compensating: Option<bool>,
    /// Derivative gain (accelerometer time constant) (Ta) (>= 0). Typical value = 3.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ta: Option<f64>,
    /// Gate servo time constant (Ts) (>= 0). Typical value = 0,15.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ts: Option<f64>,
    /// Servomotor integrator time constant (Tv) (>= 0). Typical value = 0,3.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tv: Option<f64>,
    /// Water inertia time constant (Twnc) (>= 0). Typical value = 1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub twnc: Option<f64>,
    /// Water tunnel and surge chamber inertia time constant (Twng) (>= 0). Typical value = 3.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub twng: Option<f64>,
    /// Electronic integrator time constant (Tx) (>= 0). Typical value = 0,5.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tx: Option<f64>,
    /// Maximum gate opening velocity (Va). Unit = PU / s. Typical value = 0,06.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub va: Option<f64>,
    /// Maximum gate opening (ValvMax) (> GovHydroPelton.valvmin). Typical value = 1,1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valvmax: Option<f64>,
    /// Minimum gate opening (ValvMin) (< GovHydroPelton.valvmax). Typical value = 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valvmin: Option<f64>,
    /// Maximum servomotor valve opening velocity (Vav). Typical value = 0,1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vav: Option<f64>,
    /// Maximum gate closing velocity (Vc). Unit = PU / s. Typical value = -0,06.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vc: Option<f64>,
    /// Maximum servomotor valve closing velocity (Vcv). Typical value = -0,1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vcv: Option<f64>,
    /// Water tunnel and surge chamber simulation (Tflag). true = enable of water tunnel and surge chamber simulation false = inhibit of water tunnel and surge chamber simulation. Typical value = false.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub water_tunnel_surge_chamber_simulation: Option<bool>,
    /// Head of upper water level with respect to the level of penstock (Zsfc). Unit = km. Typical value = 0,025.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zsfc: Option<f64>,
}
impl crate::base::CimElement for GovHydroPelton {
    fn mrid(&self) -> &str { &self.base.base.base.id }
    fn type_name(&self) -> &'static str { "GovHydroPelton" }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn to_json_value(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or(serde_json::Value::Null)
    }
    fn to_block(&self) -> crate::base::RdfBlock {
        let mut block = self.base.to_block();
        block.type_name = "GovHydroPelton".to_string();
        if let Some(v) = self.av0 {
            block.fields.insert("GovHydroPelton.av0".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.av1 {
            block.fields.insert("GovHydroPelton.av1".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.bp {
            block.fields.insert("GovHydroPelton.bp".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.db1 {
            block.fields.insert("GovHydroPelton.db1".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.db2 {
            block.fields.insert("GovHydroPelton.db2".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.h1 {
            block.fields.insert("GovHydroPelton.h1".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.h2 {
            block.fields.insert("GovHydroPelton.h2".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.hn {
            block.fields.insert("GovHydroPelton.hn".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.kc {
            block.fields.insert("GovHydroPelton.kc".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.kg {
            block.fields.insert("GovHydroPelton.kg".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.qc0 {
            block.fields.insert("GovHydroPelton.qc0".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.qn {
            block.fields.insert("GovHydroPelton.qn".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.simplified_pelton {
            block.fields.insert("GovHydroPelton.simplifiedPelton".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.static_compensating {
            block.fields.insert("GovHydroPelton.staticCompensating".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.ta {
            block.fields.insert("GovHydroPelton.ta".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.ts {
            block.fields.insert("GovHydroPelton.ts".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.tv {
            block.fields.insert("GovHydroPelton.tv".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.twnc {
            block.fields.insert("GovHydroPelton.twnc".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.twng {
            block.fields.insert("GovHydroPelton.twng".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.tx {
            block.fields.insert("GovHydroPelton.tx".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.va {
            block.fields.insert("GovHydroPelton.va".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.valvmax {
            block.fields.insert("GovHydroPelton.valvmax".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.valvmin {
            block.fields.insert("GovHydroPelton.valvmin".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.vav {
            block.fields.insert("GovHydroPelton.vav".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.vc {
            block.fields.insert("GovHydroPelton.vc".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.vcv {
            block.fields.insert("GovHydroPelton.vcv".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.water_tunnel_surge_chamber_simulation {
            block.fields.insert("GovHydroPelton.waterTunnelSurgeChamberSimulation".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.zsfc {
            block.fields.insert("GovHydroPelton.zsfc".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        block
    }
}

impl GovHydroPelton {
    pub fn from_block(b: &crate::base::RdfBlock) -> Self {
        let mut obj = Self::default();
        obj.base.base.base.id.clone_from(&b.mrid);
        for (key, val) in &b.fields {
            match key.as_str() {
                "GovHydroPelton.av0" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.av0 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.av0 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovHydroPelton.av1" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.av1 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.av1 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovHydroPelton.bp" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.bp = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.bp = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovHydroPelton.db1" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.db1 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.db1 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovHydroPelton.db2" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.db2 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.db2 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovHydroPelton.h1" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.h1 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.h1 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovHydroPelton.h2" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.h2 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.h2 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovHydroPelton.hn" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.hn = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.hn = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovHydroPelton.kc" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.kc = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.kc = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovHydroPelton.kg" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.kg = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.kg = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovHydroPelton.qc0" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.qc0 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.qc0 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovHydroPelton.qn" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.qn = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.qn = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovHydroPelton.simplifiedPelton" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.simplified_pelton = Some(sv.trim() == "true"); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.simplified_pelton = Some(sv.trim() == "true"); }
                        }
                        _ => {}
                    }
                }
                "GovHydroPelton.staticCompensating" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.static_compensating = Some(sv.trim() == "true"); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.static_compensating = Some(sv.trim() == "true"); }
                        }
                        _ => {}
                    }
                }
                "GovHydroPelton.ta" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.ta = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.ta = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovHydroPelton.ts" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.ts = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.ts = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovHydroPelton.tv" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.tv = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.tv = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovHydroPelton.twnc" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.twnc = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.twnc = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovHydroPelton.twng" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.twng = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.twng = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovHydroPelton.tx" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.tx = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.tx = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovHydroPelton.va" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.va = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.va = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovHydroPelton.valvmax" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.valvmax = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.valvmax = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovHydroPelton.valvmin" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.valvmin = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.valvmin = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovHydroPelton.vav" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.vav = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.vav = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovHydroPelton.vc" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.vc = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.vc = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovHydroPelton.vcv" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.vcv = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.vcv = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovHydroPelton.waterTunnelSurgeChamberSimulation" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.water_tunnel_surge_chamber_simulation = Some(sv.trim() == "true"); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.water_tunnel_surge_chamber_simulation = Some(sv.trim() == "true"); }
                        }
                        _ => {}
                    }
                }
                "GovHydroPelton.zsfc" => {
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
