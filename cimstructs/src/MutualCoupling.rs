/// This class represents the zero sequence line mutual coupling.
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct MutualCoupling {
    #[serde(flatten)]
    pub base: super::IdentifiedObject,
    /// The starting terminal for the calculation of distances along the first branch of the mutual coupling. Normally MutualCoupling would only be used for terminals of AC line segments. The first and second terminals of a mutual coupling should point to different AC line segments.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_terminal: Option<super::base::MridRef>,
    /// The starting terminal for the calculation of distances along the second branch of the mutual coupling.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub second_terminal: Option<super::base::MridRef>,
    /// Zero sequence mutual coupling shunt (charging) susceptance, uniformly distributed, of the entire line section.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub b0ch: Option<f64>,
    /// Distance to the start of the coupled region from the first line's terminal having sequence number equal to 1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub distance11: Option<f64>,
    /// Distance to the end of the coupled region from the first line's terminal with sequence number equal to 1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub distance12: Option<f64>,
    /// Distance to the start of coupled region from the second line's terminal with sequence number equal to 1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub distance21: Option<f64>,
    /// Distance to the end of coupled region from the second line's terminal with sequence number equal to 1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub distance22: Option<f64>,
    /// Zero sequence mutual coupling shunt (charging) conductance, uniformly distributed, of the entire line section.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub g0ch: Option<f64>,
    /// Zero sequence branch-to-branch mutual impedance coupling, resistance.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r0: Option<f64>,
    /// Zero sequence branch-to-branch mutual impedance coupling, reactance.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x0: Option<f64>,
}
impl crate::base::CimElement for MutualCoupling {
    fn mrid(&self) -> &str { &self.base.id }
    fn type_name(&self) -> &'static str { "MutualCoupling" }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn to_json_value(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or(serde_json::Value::Null)
    }
    fn to_block(&self) -> crate::base::RdfBlock {
        let mut block = self.base.to_block();
        block.type_name = "MutualCoupling".to_string();
        if let Some(ref v) = self.first_terminal {
            block.fields.insert("MutualCoupling.First_Terminal".into(), crate::base::FieldValue::Resource(v.mrid.clone()));
        }
        if let Some(ref v) = self.second_terminal {
            block.fields.insert("MutualCoupling.Second_Terminal".into(), crate::base::FieldValue::Resource(v.mrid.clone()));
        }
        if let Some(v) = self.b0ch {
            block.fields.insert("MutualCoupling.b0ch".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.distance11 {
            block.fields.insert("MutualCoupling.distance11".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.distance12 {
            block.fields.insert("MutualCoupling.distance12".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.distance21 {
            block.fields.insert("MutualCoupling.distance21".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.distance22 {
            block.fields.insert("MutualCoupling.distance22".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.g0ch {
            block.fields.insert("MutualCoupling.g0ch".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.r0 {
            block.fields.insert("MutualCoupling.r0".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.x0 {
            block.fields.insert("MutualCoupling.x0".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        block
    }
}

impl MutualCoupling {
    pub fn from_block(b: &crate::base::RdfBlock) -> Self {
        let mut obj = Self::default();
        obj.base.id.clone_from(&b.mrid);
        for (key, val) in &b.fields {
            match key.as_str() {
                "MutualCoupling.First_Terminal" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.first_terminal = Some(crate::base::MridRef { mrid: sv.clone() });
                    }
                }
                "MutualCoupling.Second_Terminal" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.second_terminal = Some(crate::base::MridRef { mrid: sv.clone() });
                    }
                }
                "MutualCoupling.b0ch" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.b0ch = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.b0ch = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "MutualCoupling.distance11" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.distance11 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.distance11 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "MutualCoupling.distance12" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.distance12 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.distance12 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "MutualCoupling.distance21" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.distance21 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.distance21 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "MutualCoupling.distance22" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.distance22 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.distance22 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "MutualCoupling.g0ch" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.g0ch = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.g0ch = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "MutualCoupling.r0" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.r0 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.r0 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "MutualCoupling.x0" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.x0 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.x0 = Some(v); } }
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
