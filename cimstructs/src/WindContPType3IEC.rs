/// P control model type 3. Reference: IEC 61400-27-1:2015, 5.6.5.4.
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct WindContPType3IEC {
    #[serde(flatten)]
    pub base: super::IdentifiedObject,
    /// Maximum wind turbine power ramp rate (dpmax). It is a type-dependent parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dpmax: Option<f64>,
    /// Maximum ramp rate of wind turbine reference power (dprefmax). It is a project-dependent parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dprefmax: Option<f64>,
    /// Minimum ramp rate of wind turbine reference power (dprefmin). It is a project-dependent parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dprefmin: Option<f64>,
    /// Ramp limitation of torque, required in some grid codes (dtmax). It is a project-dependent parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dthetamax: Option<f64>,
    /// Limitation of torque rise rate during UVRT (dthetamaxUVRT). It is a project-dependent parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dthetamaxuvrt: Option<f64>,
    /// Gain for active drive train damping (KDTD). It is a type-dependent parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kdtd: Option<f64>,
    /// PI controller integration parameter (KIp). It is a type-dependent parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kip: Option<f64>,
    /// PI controller proportional gain (KPp). It is a type-dependent parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kpp: Option<f64>,
    /// Enable UVRT power control mode (MpUVRT). It is a project-dependent parameter. true = voltage control (1 in the IEC model) false = reactive power control (0 in the IEC model).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mpuvrt: Option<bool>,
    /// Active drive train damping frequency (omegaDTD). It can be calculated from two mass model parameters. It is a type-dependent parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub omegadtd: Option<f64>,
    /// Offset to reference value that limits controller action during rotor speed changes (omegaoffset). It is a case-dependent parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub omegaoffset: Option<f64>,
    /// Maximum active drive train damping power (pDTDmax). It is a type-dependent parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pdtdmax: Option<f64>,
    /// Time delay after deep voltage sags (TDVS) (>= 0). It is a project-dependent parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tdvs: Option<f64>,
    /// Minimum electrical generator torque (temin). It is a type-dependent parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thetaemin: Option<f64>,
    /// Voltage scaling factor of reset-torque (tuscale). It is a project-dependent parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thetauscale: Option<f64>,
    /// Filter time constant for generator speed measurement (Tomegafiltp3) (>= 0). It is a type-dependent parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tomegafiltp3: Option<f64>,
    /// Time constant in speed reference filter (Tomega,ref) (>= 0). It is a type-dependent parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tomegaref: Option<f64>,
    /// Filter time constant for power measurement (Tpfiltp3) (>= 0). It is a type-dependent parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tpfiltp3: Option<f64>,
    /// Time constant in power order lag (Tpord). It is a type-dependent parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tpord: Option<f64>,
    /// Filter time constant for voltage measurement (Tufiltp3) (>= 0). It is a type-dependent parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tufiltp3: Option<f64>,
    /// Voltage limit for hold UVRT status after deep voltage sags (uDVS). It is a project-dependent parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub udvs: Option<f64>,
    /// Voltage dip threshold for P-control (uPdip). Part of turbine control, often different (e.g 0.8) from converter thresholds. It is a project-dependent parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updip: Option<f64>,
    /// Coefficient for active drive train damping (zeta). It is a type-dependent parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zeta: Option<f64>,
}
impl crate::base::CimElement for WindContPType3IEC {
    fn mrid(&self) -> &str { &self.base.id }
    fn type_name(&self) -> &'static str { "WindContPType3IEC" }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn to_json_value(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or(serde_json::Value::Null)
    }
    fn to_block(&self) -> crate::base::RdfBlock {
        let mut block = self.base.to_block();
        block.type_name = "WindContPType3IEC".to_string();
        if let Some(v) = self.dpmax {
            block.fields.insert("WindContPType3IEC.dpmax".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.dprefmax {
            block.fields.insert("WindContPType3IEC.dprefmax".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.dprefmin {
            block.fields.insert("WindContPType3IEC.dprefmin".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.dthetamax {
            block.fields.insert("WindContPType3IEC.dthetamax".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.dthetamaxuvrt {
            block.fields.insert("WindContPType3IEC.dthetamaxuvrt".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.kdtd {
            block.fields.insert("WindContPType3IEC.kdtd".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.kip {
            block.fields.insert("WindContPType3IEC.kip".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.kpp {
            block.fields.insert("WindContPType3IEC.kpp".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.mpuvrt {
            block.fields.insert("WindContPType3IEC.mpuvrt".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.omegadtd {
            block.fields.insert("WindContPType3IEC.omegadtd".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.omegaoffset {
            block.fields.insert("WindContPType3IEC.omegaoffset".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.pdtdmax {
            block.fields.insert("WindContPType3IEC.pdtdmax".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.tdvs {
            block.fields.insert("WindContPType3IEC.tdvs".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.thetaemin {
            block.fields.insert("WindContPType3IEC.thetaemin".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.thetauscale {
            block.fields.insert("WindContPType3IEC.thetauscale".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.tomegafiltp3 {
            block.fields.insert("WindContPType3IEC.tomegafiltp3".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.tomegaref {
            block.fields.insert("WindContPType3IEC.tomegaref".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.tpfiltp3 {
            block.fields.insert("WindContPType3IEC.tpfiltp3".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.tpord {
            block.fields.insert("WindContPType3IEC.tpord".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.tufiltp3 {
            block.fields.insert("WindContPType3IEC.tufiltp3".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.udvs {
            block.fields.insert("WindContPType3IEC.udvs".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.updip {
            block.fields.insert("WindContPType3IEC.updip".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.zeta {
            block.fields.insert("WindContPType3IEC.zeta".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        block
    }
}

impl WindContPType3IEC {
    pub fn from_block(b: &crate::base::RdfBlock) -> Self {
        let mut obj = Self::default();
        obj.base.id.clone_from(&b.mrid);
        for (key, val) in &b.fields {
            match key.as_str() {
                "WindContPType3IEC.dpmax" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.dpmax = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.dpmax = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "WindContPType3IEC.dprefmax" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.dprefmax = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.dprefmax = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "WindContPType3IEC.dprefmin" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.dprefmin = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.dprefmin = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "WindContPType3IEC.dthetamax" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.dthetamax = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.dthetamax = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "WindContPType3IEC.dthetamaxuvrt" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.dthetamaxuvrt = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.dthetamaxuvrt = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "WindContPType3IEC.kdtd" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.kdtd = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.kdtd = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "WindContPType3IEC.kip" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.kip = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.kip = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "WindContPType3IEC.kpp" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.kpp = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.kpp = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "WindContPType3IEC.mpuvrt" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.mpuvrt = Some(sv.trim() == "true"); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.mpuvrt = Some(sv.trim() == "true"); }
                        }
                        _ => {}
                    }
                }
                "WindContPType3IEC.omegadtd" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.omegadtd = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.omegadtd = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "WindContPType3IEC.omegaoffset" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.omegaoffset = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.omegaoffset = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "WindContPType3IEC.pdtdmax" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.pdtdmax = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.pdtdmax = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "WindContPType3IEC.tdvs" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.tdvs = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.tdvs = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "WindContPType3IEC.thetaemin" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.thetaemin = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.thetaemin = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "WindContPType3IEC.thetauscale" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.thetauscale = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.thetauscale = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "WindContPType3IEC.tomegafiltp3" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.tomegafiltp3 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.tomegafiltp3 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "WindContPType3IEC.tomegaref" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.tomegaref = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.tomegaref = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "WindContPType3IEC.tpfiltp3" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.tpfiltp3 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.tpfiltp3 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "WindContPType3IEC.tpord" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.tpord = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.tpord = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "WindContPType3IEC.tufiltp3" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.tufiltp3 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.tufiltp3 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "WindContPType3IEC.udvs" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.udvs = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.udvs = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "WindContPType3IEC.updip" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.updip = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.updip = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "WindContPType3IEC.zeta" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.zeta = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.zeta = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "IdentifiedObject.description" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.base.description = sv.clone(); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.base.description = sv.clone(); }
                        }
                        _ => {}
                    }
                }
                "IdentifiedObject.energyIdentCodeEic" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.base.energy_ident_code_eic = sv.clone(); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.base.energy_ident_code_eic = sv.clone(); }
                        }
                        _ => {}
                    }
                }
                "IdentifiedObject.mRID" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.base.m_rid = sv.clone(); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.base.m_rid = sv.clone(); }
                        }
                        _ => {}
                    }
                }
                "IdentifiedObject.name" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.base.name = sv.clone(); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.base.name = sv.clone(); }
                        }
                        _ => {}
                    }
                }
                "IdentifiedObject.shortName" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.base.short_name = sv.clone(); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.base.short_name = sv.clone(); }
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
