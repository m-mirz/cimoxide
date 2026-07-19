/// Current limitation model. The current limitation model combines the physical limits and the control limits. Reference: IEC 61400-27-1:2015, 5.6.5.8.
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct WindContCurrLimIEC {
    #[serde(flatten)]
    pub base: super::IdentifiedObject,
    /// Maximum continuous current at the wind turbine terminals (imax). It is a type-dependent parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub imax: Option<f64>,
    /// Maximum current during voltage dip at the wind turbine terminals (imaxdip). It is a project-dependent parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub imaxdip: Option<f64>,
    /// Partial derivative of reactive current limit (Kpqu) versus voltage. It is a type-dependent parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kpqu: Option<f64>,
    /// Limitation of type 3 stator current (MDFSLim). MDFSLim = 1 for wind turbines type 4. It is a type-dependent parameter. false= total current limitation (0 in the IEC model) true=stator current limitation (1 in the IEC model).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mdfslim: Option<bool>,
    /// Prioritisation of Q control during UVRT (Mqpri). It is a project-dependent parameter. true = reactive power priority (1 in the IEC model) false = active power priority (0 in the IEC model).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mqpri: Option<bool>,
    /// Voltage measurement filter time constant (Tufiltcl) (>= 0). It is a type-dependent parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tufiltcl: Option<f64>,
    /// Wind turbine voltage in the operation point where zero reactive current can be delivered (upqumax). It is a type-dependent parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upqumax: Option<f64>,
}
impl crate::base::CimElement for WindContCurrLimIEC {
    fn mrid(&self) -> &str { &self.base.id }
    fn type_name(&self) -> &'static str { "WindContCurrLimIEC" }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn to_json_value(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or(serde_json::Value::Null)
    }
    fn to_block(&self) -> crate::base::RdfBlock {
        let mut block = self.base.to_block();
        block.type_name = "WindContCurrLimIEC".to_string();
        if let Some(v) = self.imax {
            block.fields.insert("WindContCurrLimIEC.imax".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.imaxdip {
            block.fields.insert("WindContCurrLimIEC.imaxdip".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.kpqu {
            block.fields.insert("WindContCurrLimIEC.kpqu".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.mdfslim {
            block.fields.insert("WindContCurrLimIEC.mdfslim".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.mqpri {
            block.fields.insert("WindContCurrLimIEC.mqpri".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.tufiltcl {
            block.fields.insert("WindContCurrLimIEC.tufiltcl".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.upqumax {
            block.fields.insert("WindContCurrLimIEC.upqumax".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        block
    }
}

impl WindContCurrLimIEC {
    pub fn from_block(b: &crate::base::RdfBlock) -> Self {
        let mut obj = Self::default();
        obj.base.id.clone_from(&b.mrid);
        for (key, val) in &b.fields {
            match key.as_str() {
                "WindContCurrLimIEC.imax" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.imax = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.imax = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "WindContCurrLimIEC.imaxdip" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.imaxdip = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.imaxdip = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "WindContCurrLimIEC.kpqu" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.kpqu = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.kpqu = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "WindContCurrLimIEC.mdfslim" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.mdfslim = Some(sv.trim() == "true"); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.mdfslim = Some(sv.trim() == "true"); }
                        }
                        _ => {}
                    }
                }
                "WindContCurrLimIEC.mqpri" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.mqpri = Some(sv.trim() == "true"); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.mqpri = Some(sv.trim() == "true"); }
                        }
                        _ => {}
                    }
                }
                "WindContCurrLimIEC.tufiltcl" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.tufiltcl = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.tufiltcl = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "WindContCurrLimIEC.upqumax" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.upqumax = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.upqumax = Some(v); } }
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
