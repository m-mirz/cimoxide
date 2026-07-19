/// Aggregate induction motor load. This model is used to represent a fraction of an ordinary load as 'induction motor load'. It allows a load that is treated as an ordinary constant power in power flow analysis to be represented by an induction motor in dynamic simulation. This model is intended for representation of aggregations of many motors dispersed through a load represented at a high voltage bus but where there is no information on the characteristics of individual motors. Either a 'one-cage' or 'two-cage' model of the induction machine can be modelled. Magnetic saturation is not modelled. This model treats a fraction of the constant power part of a load as a motor. During initialisation, the initial power drawn by the motor is set equal to Pfrac times the constant P part of the static load. The remainder of the load is left as a static load. The reactive power demand of the motor is calculated during initialisation as a function of voltage at the load bus. This reactive power demand can be less than or greater than the constant Q component of the load. If the motor's reactive demand is greater than the constant Q component of the load, the model inserts a shunt capacitor at the terminal of the motor to bring its reactive demand down to equal the constant Q reactive load. If an induction motor load model and a static load model are both present for a load, the motor Pfrac is assumed to be subtracted from the power flow constant P load before the static load model is applied. The remainder of the load, if any, is then represented by the static load model.
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct LoadMotor {
    #[serde(flatten)]
    pub base: super::IdentifiedObject,
    /// Aggregate load to which this aggregate motor (dynamic) load belongs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub load_aggregate: Option<super::base::MridRef>,
    /// Damping factor (D). Unit = delta P/delta speed. Typical value = 2.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d: Option<f64>,
    /// Inertia constant (H) (>= 0). Typical value = 0,4.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub h: Option<f64>,
    /// Loading factor (Lfac). The ratio of initial P to motor MVA base. Typical value = 0,8.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lfac: Option<f64>,
    /// Transient reactance (Lp). Typical value = 0,15.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lp: Option<f64>,
    /// Subtransient reactance (Lpp). Typical value = 0,15.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lpp: Option<f64>,
    /// Synchronous reactance (Ls). Typical value = 3,2.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ls: Option<f64>,
    /// Fraction of constant-power load to be represented by this motor model (Pfrac) (>= 0,0 and <= 1,0). Typical value = 0,3.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pfrac: Option<f64>,
    /// Stator resistance (Ra). Typical value = 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ra: Option<f64>,
    /// Circuit breaker operating time (Tbkr) (>= 0). Typical value = 0,08.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tbkr: Option<f64>,
    /// Transient rotor time constant (Tpo) (>= 0). Typical value = 1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tpo: Option<f64>,
    /// Subtransient rotor time constant (Tppo) (>= 0). Typical value = 0,02.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tppo: Option<f64>,
    /// Voltage trip pickup time (Tv) (>= 0). Typical value = 0,1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tv: Option<f64>,
    /// Voltage threshold for tripping (Vt). Typical value = 0,7.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vt: Option<f64>,
}
impl crate::base::CimElement for LoadMotor {
    fn mrid(&self) -> &str { &self.base.id }
    fn type_name(&self) -> &'static str { "LoadMotor" }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn to_json_value(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or(serde_json::Value::Null)
    }
    fn to_block(&self) -> crate::base::RdfBlock {
        let mut block = self.base.to_block();
        block.type_name = "LoadMotor".to_string();
        if let Some(ref v) = self.load_aggregate {
            block.fields.insert("LoadMotor.LoadAggregate".into(), crate::base::FieldValue::Resource(v.mrid.clone()));
        }
        if let Some(v) = self.d {
            block.fields.insert("LoadMotor.d".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.h {
            block.fields.insert("LoadMotor.h".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.lfac {
            block.fields.insert("LoadMotor.lfac".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.lp {
            block.fields.insert("LoadMotor.lp".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.lpp {
            block.fields.insert("LoadMotor.lpp".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.ls {
            block.fields.insert("LoadMotor.ls".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.pfrac {
            block.fields.insert("LoadMotor.pfrac".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.ra {
            block.fields.insert("LoadMotor.ra".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.tbkr {
            block.fields.insert("LoadMotor.tbkr".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.tpo {
            block.fields.insert("LoadMotor.tpo".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.tppo {
            block.fields.insert("LoadMotor.tppo".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.tv {
            block.fields.insert("LoadMotor.tv".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.vt {
            block.fields.insert("LoadMotor.vt".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        block
    }
}

impl LoadMotor {
    pub fn from_block(b: &crate::base::RdfBlock) -> Self {
        let mut obj = Self::default();
        obj.base.id.clone_from(&b.mrid);
        for (key, val) in &b.fields {
            match key.as_str() {
                "LoadMotor.LoadAggregate" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.load_aggregate = Some(crate::base::MridRef { mrid: sv.clone() });
                    }
                }
                "LoadMotor.d" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.d = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.d = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "LoadMotor.h" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.h = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.h = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "LoadMotor.lfac" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.lfac = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.lfac = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "LoadMotor.lp" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.lp = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.lp = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "LoadMotor.lpp" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.lpp = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.lpp = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "LoadMotor.ls" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.ls = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.ls = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "LoadMotor.pfrac" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.pfrac = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.pfrac = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "LoadMotor.ra" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.ra = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.ra = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "LoadMotor.tbkr" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.tbkr = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.tbkr = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "LoadMotor.tpo" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.tpo = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.tpo = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "LoadMotor.tppo" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.tppo = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.tppo = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "LoadMotor.tv" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.tv = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.tv = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "LoadMotor.vt" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.vt = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.vt = Some(v); } }
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
