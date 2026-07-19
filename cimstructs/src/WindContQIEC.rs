/// Q control model. Reference: IEC 61400-27-1:2015, 5.6.5.7.
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct WindContQIEC {
    #[serde(flatten)]
    pub base: super::IdentifiedObject,
    /// Maximum reactive current injection during dip (iqh1). It is a type-dependent parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iqh1: Option<f64>,
    /// Maximum reactive current injection (iqmax) (> WindContQIEC.iqmin). It is a type-dependent parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iqmax: Option<f64>,
    /// Minimum reactive current injection (iqmin) (< WindContQIEC.iqmax). It is a type-dependent parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iqmin: Option<f64>,
    /// Post fault reactive current injection (iqpost). It is a project-dependent parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iqpost: Option<f64>,
    /// Reactive power PI controller integration gain (KI,q). It is a type-dependent parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kiq: Option<f64>,
    /// Voltage PI controller integration gain (KI,u). It is a type-dependent parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kiu: Option<f64>,
    /// Reactive power PI controller proportional gain (KP,q). It is a type-dependent parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kpq: Option<f64>,
    /// Voltage PI controller proportional gain (KP,u). It is a type-dependent parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kpu: Option<f64>,
    /// Voltage scaling factor for UVRT current (Kqv). It is a project-dependent parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kqv: Option<f64>,
    /// Resistive component of voltage drop impedance (rdroop) (>= 0). It is a project-dependent parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rdroop: Option<f64>,
    /// Power measurement filter time constant (Tpfiltq) (>= 0). It is a type-dependent parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tpfiltq: Option<f64>,
    /// Length of time period where post fault reactive power is injected (Tpost) (>= 0). It is a project-dependent parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tpost: Option<f64>,
    /// Time constant in reactive power order lag (Tqord) (>= 0). It is a type-dependent parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tqord: Option<f64>,
    /// Voltage measurement filter time constant (Tufiltq) (>= 0). It is a type-dependent parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tufiltq: Option<f64>,
    /// Voltage deadband lower limit (udb1). It is a type-dependent parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub udb1: Option<f64>,
    /// Voltage deadband upper limit (udb2). It is a type-dependent parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub udb2: Option<f64>,
    /// Maximum voltage in voltage PI controller integral term (umax) (> WindContQIEC.umin). It is a type-dependent parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub umax: Option<f64>,
    /// Minimum voltage in voltage PI controller integral term (umin) (< WindContQIEC.umax). It is a type-dependent parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub umin: Option<f64>,
    /// Voltage threshold for UVRT detection in Q control (uqdip). It is a type-dependent parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uqdip: Option<f64>,
    /// User-defined bias in voltage reference (uref0). It is a case-dependent parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uref0: Option<f64>,
    /// Types of general wind turbine Q control modes (MqG). It is a project-dependent parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wind_qcontrol_modes_type: Option<super::base::UriRef>,
    /// Types of UVRT Q control modes (MqUVRT). It is a project-dependent parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wind_uvrt_qcontrol_modes_type: Option<super::base::UriRef>,
    /// Inductive component of voltage drop impedance (xdroop) (>= 0). It is a project-dependent parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub xdroop: Option<f64>,
}
impl crate::base::CimElement for WindContQIEC {
    fn mrid(&self) -> &str { &self.base.id }
    fn type_name(&self) -> &'static str { "WindContQIEC" }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn to_json_value(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or(serde_json::Value::Null)
    }
    fn to_block(&self) -> crate::base::RdfBlock {
        let mut block = self.base.to_block();
        block.type_name = "WindContQIEC".to_string();
        if let Some(v) = self.iqh1 {
            block.fields.insert("WindContQIEC.iqh1".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.iqmax {
            block.fields.insert("WindContQIEC.iqmax".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.iqmin {
            block.fields.insert("WindContQIEC.iqmin".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.iqpost {
            block.fields.insert("WindContQIEC.iqpost".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.kiq {
            block.fields.insert("WindContQIEC.kiq".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.kiu {
            block.fields.insert("WindContQIEC.kiu".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.kpq {
            block.fields.insert("WindContQIEC.kpq".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.kpu {
            block.fields.insert("WindContQIEC.kpu".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.kqv {
            block.fields.insert("WindContQIEC.kqv".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.rdroop {
            block.fields.insert("WindContQIEC.rdroop".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.tpfiltq {
            block.fields.insert("WindContQIEC.tpfiltq".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.tpost {
            block.fields.insert("WindContQIEC.tpost".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.tqord {
            block.fields.insert("WindContQIEC.tqord".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.tufiltq {
            block.fields.insert("WindContQIEC.tufiltq".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.udb1 {
            block.fields.insert("WindContQIEC.udb1".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.udb2 {
            block.fields.insert("WindContQIEC.udb2".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.umax {
            block.fields.insert("WindContQIEC.umax".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.umin {
            block.fields.insert("WindContQIEC.umin".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.uqdip {
            block.fields.insert("WindContQIEC.uqdip".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.uref0 {
            block.fields.insert("WindContQIEC.uref0".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(ref v) = self.wind_qcontrol_modes_type {
            block.fields.insert("WindContQIEC.windQcontrolModesType".into(), crate::base::FieldValue::Resource(v.uri.clone()));
        }
        if let Some(ref v) = self.wind_uvrt_qcontrol_modes_type {
            block.fields.insert("WindContQIEC.windUVRTQcontrolModesType".into(), crate::base::FieldValue::Resource(v.uri.clone()));
        }
        if let Some(v) = self.xdroop {
            block.fields.insert("WindContQIEC.xdroop".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        block
    }
}

impl WindContQIEC {
    pub fn from_block(b: &crate::base::RdfBlock) -> Self {
        let mut obj = Self::default();
        obj.base.id.clone_from(&b.mrid);
        for (key, val) in &b.fields {
            match key.as_str() {
                "WindContQIEC.iqh1" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.iqh1 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.iqh1 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "WindContQIEC.iqmax" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.iqmax = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.iqmax = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "WindContQIEC.iqmin" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.iqmin = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.iqmin = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "WindContQIEC.iqpost" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.iqpost = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.iqpost = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "WindContQIEC.kiq" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.kiq = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.kiq = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "WindContQIEC.kiu" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.kiu = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.kiu = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "WindContQIEC.kpq" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.kpq = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.kpq = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "WindContQIEC.kpu" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.kpu = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.kpu = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "WindContQIEC.kqv" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.kqv = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.kqv = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "WindContQIEC.rdroop" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.rdroop = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.rdroop = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "WindContQIEC.tpfiltq" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.tpfiltq = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.tpfiltq = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "WindContQIEC.tpost" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.tpost = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.tpost = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "WindContQIEC.tqord" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.tqord = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.tqord = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "WindContQIEC.tufiltq" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.tufiltq = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.tufiltq = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "WindContQIEC.udb1" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.udb1 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.udb1 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "WindContQIEC.udb2" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.udb2 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.udb2 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "WindContQIEC.umax" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.umax = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.umax = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "WindContQIEC.umin" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.umin = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.umin = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "WindContQIEC.uqdip" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.uqdip = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.uqdip = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "WindContQIEC.uref0" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.uref0 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.uref0 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "WindContQIEC.windQcontrolModesType" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.wind_qcontrol_modes_type = Some(crate::base::UriRef { uri: sv.clone() });
                    }
                }
                "WindContQIEC.windUVRTQcontrolModesType" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.wind_uvrt_qcontrol_modes_type = Some(crate::base::UriRef { uri: sv.clone() });
                    }
                }
                "WindContQIEC.xdroop" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.xdroop = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.xdroop = Some(v); } }
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
