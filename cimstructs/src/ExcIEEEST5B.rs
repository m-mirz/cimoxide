/// IEEE 421.5-2005 type ST5B model. The type ST5B excitation system is a variation of the type ST1A model, with alternative overexcitation and underexcitation inputs and additional limits. The block diagram in the IEEE 421.5 standard has input signal Vc and does not indicate the summation point with Vref. The implementation of the ExcIEEEST5B shall consider summation point with Vref. Reference: IEEE 421.5-2005, 7.5.
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct ExcIEEEST5B {
    #[serde(flatten)]
    pub base: super::ExcitationSystemDynamics,
    /// Rectifier regulation factor (KC) (>= 0). Typical value = 0,004.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kc: Option<f64>,
    /// Regulator gain (KR) (> 0). Typical value = 200.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kr: Option<f64>,
    /// Firing circuit time constant (T1) (>= 0). Typical value = 0,004.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub t1: Option<f64>,
    /// Regulator lag time constant (TB1) (>= 0). Typical value = 6.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tb1: Option<f64>,
    /// Regulator lag time constant (TB2) (>= 0). Typical value = 0,01.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tb2: Option<f64>,
    /// Regulator lead time constant (TC1) (>= 0). Typical value = 0,8.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tc1: Option<f64>,
    /// Regulator lead time constant (TC2) (>= 0). Typical value = 0,08.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tc2: Option<f64>,
    /// OEL lag time constant (TOB1) (>= 0). Typical value = 2.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tob1: Option<f64>,
    /// OEL lag time constant (TOB2) (>= 0). Typical value = 0,08.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tob2: Option<f64>,
    /// OEL lead time constant (TOC1) (>= 0). Typical value = 0,1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub toc1: Option<f64>,
    /// OEL lead time constant (TOC2) (>= 0). Typical value = 0,08.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub toc2: Option<f64>,
    /// UEL lag time constant (TUB1) (>= 0). Typical value = 10.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tub1: Option<f64>,
    /// UEL lag time constant (TUB2) (>= 0). Typical value = 0,05.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tub2: Option<f64>,
    /// UEL lead time constant (TUC1) (>= 0). Typical value = 2.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tuc1: Option<f64>,
    /// UEL lead time constant (TUC2) (>= 0). Typical value = 0,1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tuc2: Option<f64>,
    /// Maximum voltage regulator output (VRMAX) (> 0). Typical value = 5.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vrmax: Option<f64>,
    /// Minimum voltage regulator output (VRMIN) (< 0). Typical value = -4.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vrmin: Option<f64>,
}
impl crate::base::CimElement for ExcIEEEST5B {
    fn mrid(&self) -> &str { &self.base.base.base.id }
    fn type_name(&self) -> &'static str { "ExcIEEEST5B" }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn to_json_value(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or(serde_json::Value::Null)
    }
    fn to_block(&self) -> crate::base::RdfBlock {
        let mut block = self.base.to_block();
        block.type_name = "ExcIEEEST5B".to_string();
        if let Some(v) = self.kc {
            block.fields.insert("ExcIEEEST5B.kc".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.kr {
            block.fields.insert("ExcIEEEST5B.kr".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.t1 {
            block.fields.insert("ExcIEEEST5B.t1".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.tb1 {
            block.fields.insert("ExcIEEEST5B.tb1".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.tb2 {
            block.fields.insert("ExcIEEEST5B.tb2".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.tc1 {
            block.fields.insert("ExcIEEEST5B.tc1".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.tc2 {
            block.fields.insert("ExcIEEEST5B.tc2".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.tob1 {
            block.fields.insert("ExcIEEEST5B.tob1".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.tob2 {
            block.fields.insert("ExcIEEEST5B.tob2".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.toc1 {
            block.fields.insert("ExcIEEEST5B.toc1".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.toc2 {
            block.fields.insert("ExcIEEEST5B.toc2".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.tub1 {
            block.fields.insert("ExcIEEEST5B.tub1".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.tub2 {
            block.fields.insert("ExcIEEEST5B.tub2".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.tuc1 {
            block.fields.insert("ExcIEEEST5B.tuc1".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.tuc2 {
            block.fields.insert("ExcIEEEST5B.tuc2".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.vrmax {
            block.fields.insert("ExcIEEEST5B.vrmax".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.vrmin {
            block.fields.insert("ExcIEEEST5B.vrmin".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        block
    }
}

impl ExcIEEEST5B {
    pub fn from_block(b: &crate::base::RdfBlock) -> Self {
        let mut obj = Self::default();
        obj.base.base.base.id.clone_from(&b.mrid);
        for (key, val) in &b.fields {
            match key.as_str() {
                "ExcIEEEST5B.kc" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.kc = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.kc = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcIEEEST5B.kr" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.kr = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.kr = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcIEEEST5B.t1" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.t1 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.t1 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcIEEEST5B.tb1" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.tb1 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.tb1 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcIEEEST5B.tb2" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.tb2 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.tb2 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcIEEEST5B.tc1" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.tc1 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.tc1 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcIEEEST5B.tc2" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.tc2 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.tc2 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcIEEEST5B.tob1" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.tob1 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.tob1 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcIEEEST5B.tob2" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.tob2 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.tob2 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcIEEEST5B.toc1" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.toc1 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.toc1 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcIEEEST5B.toc2" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.toc2 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.toc2 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcIEEEST5B.tub1" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.tub1 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.tub1 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcIEEEST5B.tub2" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.tub2 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.tub2 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcIEEEST5B.tuc1" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.tuc1 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.tuc1 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcIEEEST5B.tuc2" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.tuc2 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.tuc2 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcIEEEST5B.vrmax" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.vrmax = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.vrmax = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ExcIEEEST5B.vrmin" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.vrmin = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.vrmin = Some(v); } }
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
