/// General static load. This model represents the sensitivity of the real and reactive power consumed by the load to the amplitude and frequency of the bus voltage.
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct LoadStatic {
    #[serde(flatten)]
    pub base: super::IdentifiedObject,
    /// Aggregate load to which this aggregate static load belongs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub load_aggregate: Option<super::base::MridRef>,
    /// First term voltage exponent for active power (Ep1). Used only when .staticLoadModelType = exponential.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ep1: Option<f64>,
    /// Second term voltage exponent for active power (Ep2). Used only when .staticLoadModelType = exponential.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ep2: Option<f64>,
    /// Third term voltage exponent for active power (Ep3). Used only when .staticLoadModelType = exponential.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ep3: Option<f64>,
    /// First term voltage exponent for reactive power (Eq1). Used only when .staticLoadModelType = exponential.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eq1: Option<f64>,
    /// Second term voltage exponent for reactive power (Eq2). Used only when .staticLoadModelType = exponential.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eq2: Option<f64>,
    /// Third term voltage exponent for reactive power (Eq3). Used only when .staticLoadModelType = exponential.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eq3: Option<f64>,
    /// First term voltage coefficient for active power (Kp1). Not used when .staticLoadModelType = constantZ.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kp1: Option<f64>,
    /// Second term voltage coefficient for active power (Kp2). Not used when .staticLoadModelType = constantZ.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kp2: Option<f64>,
    /// Third term voltage coefficient for active power (Kp3). Not used when .staticLoadModelType = constantZ.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kp3: Option<f64>,
    /// Frequency coefficient for active power (Kp4) (not = 0 if .staticLoadModelType = zIP2). Used only when .staticLoadModelType = zIP2.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kp4: Option<f64>,
    /// Frequency deviation coefficient for active power (Kpf). Not used when .staticLoadModelType = constantZ.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kpf: Option<f64>,
    /// First term voltage coefficient for reactive power (Kq1). Not used when .staticLoadModelType = constantZ.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kq1: Option<f64>,
    /// Second term voltage coefficient for reactive power (Kq2). Not used when .staticLoadModelType = constantZ.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kq2: Option<f64>,
    /// Third term voltage coefficient for reactive power (Kq3). Not used when .staticLoadModelType = constantZ.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kq3: Option<f64>,
    /// Frequency coefficient for reactive power (Kq4) (not = 0 when .staticLoadModelType = zIP2). Used only when .staticLoadModelType - zIP2.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kq4: Option<f64>,
    /// Frequency deviation coefficient for reactive power (Kqf). Not used when .staticLoadModelType = constantZ.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kqf: Option<f64>,
    /// Type of static load model. Typical value = constantZ.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub static_load_model_type: Option<super::base::UriRef>,
}
impl crate::base::CimElement for LoadStatic {
    fn mrid(&self) -> &str { &self.base.id }
    fn type_name(&self) -> &'static str { "LoadStatic" }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn to_json_value(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or(serde_json::Value::Null)
    }
    fn to_block(&self) -> crate::base::RdfBlock {
        let mut block = self.base.to_block();
        block.type_name = "LoadStatic".to_string();
        if let Some(ref v) = self.load_aggregate {
            block.fields.insert("LoadStatic.LoadAggregate".into(), crate::base::FieldValue::Resource(v.mrid.clone()));
        }
        if let Some(v) = self.ep1 {
            block.fields.insert("LoadStatic.ep1".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.ep2 {
            block.fields.insert("LoadStatic.ep2".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.ep3 {
            block.fields.insert("LoadStatic.ep3".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.eq1 {
            block.fields.insert("LoadStatic.eq1".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.eq2 {
            block.fields.insert("LoadStatic.eq2".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.eq3 {
            block.fields.insert("LoadStatic.eq3".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.kp1 {
            block.fields.insert("LoadStatic.kp1".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.kp2 {
            block.fields.insert("LoadStatic.kp2".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.kp3 {
            block.fields.insert("LoadStatic.kp3".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.kp4 {
            block.fields.insert("LoadStatic.kp4".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.kpf {
            block.fields.insert("LoadStatic.kpf".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.kq1 {
            block.fields.insert("LoadStatic.kq1".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.kq2 {
            block.fields.insert("LoadStatic.kq2".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.kq3 {
            block.fields.insert("LoadStatic.kq3".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.kq4 {
            block.fields.insert("LoadStatic.kq4".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.kqf {
            block.fields.insert("LoadStatic.kqf".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(ref v) = self.static_load_model_type {
            block.fields.insert("LoadStatic.staticLoadModelType".into(), crate::base::FieldValue::Resource(v.uri.clone()));
        }
        block
    }
}

impl LoadStatic {
    pub fn from_block(b: &crate::base::RdfBlock) -> Self {
        let mut obj = Self::default();
        obj.base.id.clone_from(&b.mrid);
        for (key, val) in &b.fields {
            match key.as_str() {
                "LoadStatic.LoadAggregate" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.load_aggregate = Some(crate::base::MridRef { mrid: sv.clone() });
                    }
                }
                "LoadStatic.ep1" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.ep1 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.ep1 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "LoadStatic.ep2" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.ep2 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.ep2 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "LoadStatic.ep3" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.ep3 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.ep3 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "LoadStatic.eq1" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.eq1 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.eq1 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "LoadStatic.eq2" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.eq2 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.eq2 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "LoadStatic.eq3" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.eq3 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.eq3 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "LoadStatic.kp1" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.kp1 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.kp1 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "LoadStatic.kp2" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.kp2 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.kp2 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "LoadStatic.kp3" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.kp3 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.kp3 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "LoadStatic.kp4" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.kp4 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.kp4 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "LoadStatic.kpf" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.kpf = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.kpf = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "LoadStatic.kq1" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.kq1 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.kq1 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "LoadStatic.kq2" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.kq2 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.kq2 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "LoadStatic.kq3" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.kq3 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.kq3 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "LoadStatic.kq4" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.kq4 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.kq4 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "LoadStatic.kqf" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.kqf = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.kqf = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "LoadStatic.staticLoadModelType" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.static_load_model_type = Some(crate::base::UriRef { uri: sv.clone() });
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
