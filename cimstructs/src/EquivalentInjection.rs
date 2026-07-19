/// This class represents equivalent injections (generation or load). Voltage regulation is allowed only at the point of connection.
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct EquivalentInjection {
    #[serde(flatten)]
    pub base: super::EquivalentEquipment,
    /// The reactive capability curve used by this equivalent injection.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reactive_capability_curve: Option<super::base::MridRef>,
    /// Maximum active power of the injection.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_p: Option<f64>,
    /// Maximum reactive power of the injection. Used for modelling of infeed for load flow exchange. Not used for short circuit modelling. If maxQ and minQ are not used ReactiveCapabilityCurve can be used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_q: Option<f64>,
    /// Minimum active power of the injection.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_p: Option<f64>,
    /// Minimum reactive power of the injection. Used for modelling of infeed for load flow exchange. Not used for short circuit modelling. If maxQ and minQ are not used ReactiveCapabilityCurve can be used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_q: Option<f64>,
    /// Equivalent active power injection. Load sign convention is used, i.e. positive sign means flow out from a node. Starting value for steady state solutions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub p: Option<f64>,
    /// Equivalent reactive power injection. Load sign convention is used, i.e. positive sign means flow out from a node. Starting value for steady state solutions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub q: Option<f64>,
    /// Positive sequence resistance. Used to represent Extended-Ward (IEC 60909). Usage : Extended-Ward is a result of network reduction prior to the data exchange.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r: Option<f64>,
    /// Zero sequence resistance. Used to represent Extended-Ward (IEC 60909). Usage : Extended-Ward is a result of network reduction prior to the data exchange.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r0: Option<f64>,
    /// Negative sequence resistance. Used to represent Extended-Ward (IEC 60909). Usage : Extended-Ward is a result of network reduction prior to the data exchange.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r2: Option<f64>,
    /// Specifies whether or not the EquivalentInjection has the capability to regulate the local voltage. If true the EquivalentInjection can regulate. If false the EquivalentInjection cannot regulate. ReactiveCapabilityCurve can only be associated with EquivalentInjection if the flag is true.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub regulation_capability: Option<bool>,
    /// Specifies the regulation status of the EquivalentInjection. True is regulating. False is not regulating.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub regulation_status: Option<bool>,
    /// The target voltage for voltage regulation. The attribute shall be a positive value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub regulation_target: Option<f64>,
    /// Positive sequence reactance. Used to represent Extended-Ward (IEC 60909). Usage : Extended-Ward is a result of network reduction prior to the data exchange.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x: Option<f64>,
    /// Zero sequence reactance. Used to represent Extended-Ward (IEC 60909). Usage : Extended-Ward is a result of network reduction prior to the data exchange.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x0: Option<f64>,
    /// Negative sequence reactance. Used to represent Extended-Ward (IEC 60909). Usage : Extended-Ward is a result of network reduction prior to the data exchange.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x2: Option<f64>,
}
impl crate::base::CimElement for EquivalentInjection {
    fn mrid(&self) -> &str { &self.base.base.base.base.base.id }
    fn type_name(&self) -> &'static str { "EquivalentInjection" }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn to_json_value(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or(serde_json::Value::Null)
    }
    fn to_block(&self) -> crate::base::RdfBlock {
        let mut block = self.base.to_block();
        block.type_name = "EquivalentInjection".to_string();
        if let Some(ref v) = self.reactive_capability_curve {
            block.fields.insert("EquivalentInjection.ReactiveCapabilityCurve".into(), crate::base::FieldValue::Resource(v.mrid.clone()));
        }
        if let Some(v) = self.max_p {
            block.fields.insert("EquivalentInjection.maxP".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.max_q {
            block.fields.insert("EquivalentInjection.maxQ".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.min_p {
            block.fields.insert("EquivalentInjection.minP".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.min_q {
            block.fields.insert("EquivalentInjection.minQ".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.p {
            block.fields.insert("EquivalentInjection.p".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.q {
            block.fields.insert("EquivalentInjection.q".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.r {
            block.fields.insert("EquivalentInjection.r".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.r0 {
            block.fields.insert("EquivalentInjection.r0".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.r2 {
            block.fields.insert("EquivalentInjection.r2".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.regulation_capability {
            block.fields.insert("EquivalentInjection.regulationCapability".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.regulation_status {
            block.fields.insert("EquivalentInjection.regulationStatus".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.regulation_target {
            block.fields.insert("EquivalentInjection.regulationTarget".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.x {
            block.fields.insert("EquivalentInjection.x".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.x0 {
            block.fields.insert("EquivalentInjection.x0".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.x2 {
            block.fields.insert("EquivalentInjection.x2".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        block
    }
}

impl EquivalentInjection {
    pub fn from_block(b: &crate::base::RdfBlock) -> Self {
        let mut obj = Self::default();
        obj.base.base.base.base.base.id.clone_from(&b.mrid);
        for (key, val) in &b.fields {
            match key.as_str() {
                "EquivalentInjection.ReactiveCapabilityCurve" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.reactive_capability_curve = Some(crate::base::MridRef { mrid: sv.clone() });
                    }
                }
                "EquivalentInjection.maxP" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.max_p = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.max_p = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "EquivalentInjection.maxQ" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.max_q = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.max_q = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "EquivalentInjection.minP" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.min_p = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.min_p = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "EquivalentInjection.minQ" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.min_q = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.min_q = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "EquivalentInjection.p" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.p = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.p = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "EquivalentInjection.q" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.q = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.q = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "EquivalentInjection.r" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.r = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.r = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "EquivalentInjection.r0" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.r0 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.r0 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "EquivalentInjection.r2" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.r2 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.r2 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "EquivalentInjection.regulationCapability" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.regulation_capability = Some(sv.trim() == "true"); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.regulation_capability = Some(sv.trim() == "true"); }
                        }
                        _ => {}
                    }
                }
                "EquivalentInjection.regulationStatus" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.regulation_status = Some(sv.trim() == "true"); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.regulation_status = Some(sv.trim() == "true"); }
                        }
                        _ => {}
                    }
                }
                "EquivalentInjection.regulationTarget" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.regulation_target = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.regulation_target = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "EquivalentInjection.x" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.x = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.x = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "EquivalentInjection.x0" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.x0 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.x0 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "EquivalentInjection.x2" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.x2 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.x2 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "EquivalentEquipment.EquivalentNetwork" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.base.equivalent_network = Some(crate::base::MridRef { mrid: sv.clone() });
                    }
                }
                "ConductingEquipment.BaseVoltage" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.base.base.base_voltage = Some(crate::base::MridRef { mrid: sv.clone() });
                    }
                }
                "Equipment.EquipmentContainer" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.base.base.base.equipment_container = Some(crate::base::MridRef { mrid: sv.clone() });
                    }
                }
                "Equipment.aggregate" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.base.base.base.aggregate = Some(sv.trim() == "true"); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.base.base.base.aggregate = Some(sv.trim() == "true"); }
                        }
                        _ => {}
                    }
                }
                "Equipment.inService" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.base.base.base.in_service = Some(sv.trim() == "true"); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.base.base.base.in_service = Some(sv.trim() == "true"); }
                        }
                        _ => {}
                    }
                }
                "Equipment.normallyInService" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.base.base.base.normally_in_service = Some(sv.trim() == "true"); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.base.base.base.normally_in_service = Some(sv.trim() == "true"); }
                        }
                        _ => {}
                    }
                }
                "IdentifiedObject.description" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.base.base.base.base.base.description = sv.clone(); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.base.base.base.base.base.description = sv.clone(); }
                        }
                        _ => {}
                    }
                }
                "IdentifiedObject.energyIdentCodeEic" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.base.base.base.base.base.energy_ident_code_eic = sv.clone(); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.base.base.base.base.base.energy_ident_code_eic = sv.clone(); }
                        }
                        _ => {}
                    }
                }
                "IdentifiedObject.mRID" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.base.base.base.base.base.m_rid = sv.clone(); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.base.base.base.base.base.m_rid = sv.clone(); }
                        }
                        _ => {}
                    }
                }
                "IdentifiedObject.name" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.base.base.base.base.base.name = sv.clone(); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.base.base.base.base.base.name = sv.clone(); }
                        }
                        _ => {}
                    }
                }
                "IdentifiedObject.shortName" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.base.base.base.base.base.short_name = sv.clone(); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.base.base.base.base.base.short_name = sv.clone(); }
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
