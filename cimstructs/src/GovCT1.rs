/// General model for any prime mover with a PID governor, used primarily for combustion turbine and combined cycle units. This model can be used to represent a variety of prime movers controlled by PID governors. It is suitable, for example, for the representation of: gas turbine and single shaft combined cycle turbines diesel engines with modern electronic or digital governors steam turbines where steam is supplied from a large boiler drum or a large header whose pressure is substantially constant over the period under study simple hydro turbines in dam configurations where the water column length is short and water inertia effects are minimal. Additional information on this model is available in the 2012 IEEE report, Dynamic Models for Turbine-Governors in Power System Studies, 3.1.2.3 pages 3-4 (GGOV1).
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct GovCT1 {
    #[serde(flatten)]
    pub base: super::TurbineGovernorDynamics,
    /// Acceleration limiter setpoint (Aset). Unit = PU / s. Typical value = 0,01.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aset: Option<f64>,
    /// Speed governor deadband in PU speed (db). In the majority of applications, it is recommended that this value be set to zero. Typical value = 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db: Option<f64>,
    /// Speed sensitivity coefficient (Dm). Dm can represent either the variation of the engine power with the shaft speed or the variation of maximum power capability with shaft speed. If it is positive it describes the falling slope of the engine speed verses power characteristic as speed increases. A slightly falling characteristic is typical for reciprocating engines and some aero-derivative turbines. If it is negative the engine power is assumed to be unaffected by the shaft speed, but the maximum permissible fuel flow is taken to fall with falling shaft speed. This is characteristic of single-shaft industrial turbines due to exhaust temperature limits. Typical value = 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dm: Option<f64>,
    /// Acceleration limiter gain (Ka). Typical value = 10.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ka: Option<f64>,
    /// Governor derivative gain (Kdgov). Typical value = 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kdgov: Option<f64>,
    /// Governor integral gain (Kigov). Typical value = 2.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kigov: Option<f64>,
    /// Load limiter integral gain for PI controller (Kiload). Typical value = 0,67.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kiload: Option<f64>,
    /// Power controller (reset) gain (Kimw). The default value of 0,01 corresponds to a reset time of 100 s. A value of 0,001 corresponds to a relatively slow-acting load controller. Typical value = 0,01.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kimw: Option<f64>,
    /// Governor proportional gain (Kpgov). Typical value = 10.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kpgov: Option<f64>,
    /// Load limiter proportional gain for PI controller (Kpload). Typical value = 2.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kpload: Option<f64>,
    /// Turbine gain (Kturb) (> 0). Typical value = 1,5.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kturb: Option<f64>,
    /// Load limiter reference value (Ldref). Typical value = 1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ldref: Option<f64>,
    /// Maximum value for speed error signal (maxerr) (> GovCT1.minerr). Typical value = 0,05.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maxerr: Option<f64>,
    /// Minimum value for speed error signal (minerr) (< GovCT1.maxerr). Typical value = -0,05.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minerr: Option<f64>,
    /// Base for power values (MWbase) (> 0). Unit = MW.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mwbase: Option<f64>,
    /// Permanent droop (R). Typical value = 0,04.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r: Option<f64>,
    /// Minimum valve closing rate (Rclose). Unit = PU / s. Typical value = -0,1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rclose: Option<f64>,
    /// Maximum rate of load limit decrease (Rdown). Typical value = -99.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rdown: Option<f64>,
    /// Maximum valve opening rate (Ropen). Unit = PU / s. Typical value = 0.10.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ropen: Option<f64>,
    /// Feedback signal for droop (Rselect). Typical value = electricalPower.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rselect: Option<super::base::UriRef>,
    /// Maximum rate of load limit increase (Rup). Typical value = 99.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rup: Option<f64>,
    /// Acceleration limiter time constant (Ta) (> 0). Typical value = 0,1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ta: Option<f64>,
    /// Actuator time constant (Tact) (>= 0). Typical value = 0,5.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tact: Option<f64>,
    /// Turbine lag time constant (Tb) (> 0). Typical value = 0,5.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tb: Option<f64>,
    /// Turbine lead time constant (Tc) (>= 0). Typical value = 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tc: Option<f64>,
    /// Governor derivative controller time constant (Tdgov) (>= 0). Typical value = 1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tdgov: Option<f64>,
    /// Transport time delay for diesel engine used in representing diesel engines where there is a small but measurable transport delay between a change in fuel flow setting and the development of torque (Teng) (>= 0). Teng should be zero in all but special cases where this transport delay is of particular concern. Typical value = 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub teng: Option<f64>,
    /// Load-limiter time constant (Tfload) (> 0). Typical value = 3.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tfload: Option<f64>,
    /// Electrical power transducer time constant (Tpelec) (> 0). Typical value = 1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tpelec: Option<f64>,
    /// Temperature detection lead time constant (Tsa) (>= 0). Typical value = 4.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tsa: Option<f64>,
    /// Temperature detection lag time constant (Tsb) (>= 0). Typical value = 5.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tsb: Option<f64>,
    /// Maximum valve position limit (Vmax) (> GovCT1.vmin). Typical value = 1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vmax: Option<f64>,
    /// Minimum valve position limit (Vmin) (< GovCT1.vmax). Typical value = 0,15.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vmin: Option<f64>,
    /// No load fuel flow (Wfnl). Typical value = 0,2.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wfnl: Option<f64>,
    /// Switch for fuel source characteristic to recognize that fuel flow, for a given fuel valve stroke, can be proportional to engine speed (Wfspd). true = fuel flow proportional to speed (for some gas turbines and diesel engines with positive displacement fuel injectors) false = fuel control system keeps fuel flow independent of engine speed. Typical value = true.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wfspd: Option<bool>,
}
impl crate::base::CimElement for GovCT1 {
    fn mrid(&self) -> &str { &self.base.base.base.id }
    fn type_name(&self) -> &'static str { "GovCT1" }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn to_json_value(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or(serde_json::Value::Null)
    }
    fn to_block(&self) -> crate::base::RdfBlock {
        let mut block = self.base.to_block();
        block.type_name = "GovCT1".to_string();
        if let Some(v) = self.aset {
            block.fields.insert("GovCT1.aset".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.db {
            block.fields.insert("GovCT1.db".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.dm {
            block.fields.insert("GovCT1.dm".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.ka {
            block.fields.insert("GovCT1.ka".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.kdgov {
            block.fields.insert("GovCT1.kdgov".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.kigov {
            block.fields.insert("GovCT1.kigov".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.kiload {
            block.fields.insert("GovCT1.kiload".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.kimw {
            block.fields.insert("GovCT1.kimw".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.kpgov {
            block.fields.insert("GovCT1.kpgov".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.kpload {
            block.fields.insert("GovCT1.kpload".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.kturb {
            block.fields.insert("GovCT1.kturb".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.ldref {
            block.fields.insert("GovCT1.ldref".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.maxerr {
            block.fields.insert("GovCT1.maxerr".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.minerr {
            block.fields.insert("GovCT1.minerr".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.mwbase {
            block.fields.insert("GovCT1.mwbase".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.r {
            block.fields.insert("GovCT1.r".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.rclose {
            block.fields.insert("GovCT1.rclose".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.rdown {
            block.fields.insert("GovCT1.rdown".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.ropen {
            block.fields.insert("GovCT1.ropen".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(ref v) = self.rselect {
            block.fields.insert("GovCT1.rselect".into(), crate::base::FieldValue::Resource(v.uri.clone()));
        }
        if let Some(v) = self.rup {
            block.fields.insert("GovCT1.rup".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.ta {
            block.fields.insert("GovCT1.ta".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.tact {
            block.fields.insert("GovCT1.tact".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.tb {
            block.fields.insert("GovCT1.tb".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.tc {
            block.fields.insert("GovCT1.tc".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.tdgov {
            block.fields.insert("GovCT1.tdgov".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.teng {
            block.fields.insert("GovCT1.teng".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.tfload {
            block.fields.insert("GovCT1.tfload".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.tpelec {
            block.fields.insert("GovCT1.tpelec".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.tsa {
            block.fields.insert("GovCT1.tsa".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.tsb {
            block.fields.insert("GovCT1.tsb".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.vmax {
            block.fields.insert("GovCT1.vmax".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.vmin {
            block.fields.insert("GovCT1.vmin".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.wfnl {
            block.fields.insert("GovCT1.wfnl".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.wfspd {
            block.fields.insert("GovCT1.wfspd".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        block
    }
}

impl GovCT1 {
    pub fn from_block(b: &crate::base::RdfBlock) -> Self {
        let mut obj = Self::default();
        obj.base.base.base.id.clone_from(&b.mrid);
        for (key, val) in &b.fields {
            match key.as_str() {
                "GovCT1.aset" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.aset = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.aset = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovCT1.db" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.db = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.db = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovCT1.dm" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.dm = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.dm = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovCT1.ka" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.ka = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.ka = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovCT1.kdgov" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.kdgov = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.kdgov = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovCT1.kigov" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.kigov = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.kigov = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovCT1.kiload" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.kiload = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.kiload = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovCT1.kimw" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.kimw = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.kimw = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovCT1.kpgov" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.kpgov = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.kpgov = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovCT1.kpload" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.kpload = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.kpload = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovCT1.kturb" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.kturb = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.kturb = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovCT1.ldref" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.ldref = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.ldref = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovCT1.maxerr" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.maxerr = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.maxerr = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovCT1.minerr" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.minerr = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.minerr = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovCT1.mwbase" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.mwbase = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.mwbase = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovCT1.r" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.r = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.r = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovCT1.rclose" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.rclose = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.rclose = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovCT1.rdown" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.rdown = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.rdown = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovCT1.ropen" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.ropen = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.ropen = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovCT1.rselect" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.rselect = Some(crate::base::UriRef { uri: sv.clone() });
                    }
                }
                "GovCT1.rup" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.rup = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.rup = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovCT1.ta" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.ta = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.ta = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovCT1.tact" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.tact = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.tact = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovCT1.tb" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.tb = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.tb = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovCT1.tc" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.tc = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.tc = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovCT1.tdgov" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.tdgov = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.tdgov = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovCT1.teng" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.teng = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.teng = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovCT1.tfload" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.tfload = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.tfload = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovCT1.tpelec" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.tpelec = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.tpelec = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovCT1.tsa" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.tsa = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.tsa = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovCT1.tsb" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.tsb = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.tsb = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovCT1.vmax" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.vmax = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.vmax = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovCT1.vmin" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.vmin = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.vmin = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovCT1.wfnl" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.wfnl = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.wfnl = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GovCT1.wfspd" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.wfspd = Some(sv.trim() == "true"); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.wfspd = Some(sv.trim() == "true"); }
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
