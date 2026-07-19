/// The class represents equivalent branches. In cases where a transformer phase shift is modelled and the EquivalentBranch is spanning the same nodes, the impedance quantities for the EquivalentBranch shall consider the needed phase shift.
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct EquivalentBranch {
    #[serde(flatten)]
    pub base: super::EquivalentEquipment,
    /// Negative sequence series resistance from terminal sequence 1 to terminal sequence 2. Used for short circuit data exchange according to IEC 60909. EquivalentBranch is a result of network reduction prior to the data exchange.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub negative_r12: Option<f64>,
    /// Negative sequence series resistance from terminal sequence 2 to terminal sequence 1. Used for short circuit data exchange according to IEC 60909. EquivalentBranch is a result of network reduction prior to the data exchange.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub negative_r21: Option<f64>,
    /// Negative sequence series reactance from terminal sequence 1 to terminal sequence 2. Used for short circuit data exchange according to IEC 60909. Usage : EquivalentBranch is a result of network reduction prior to the data exchange.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub negative_x12: Option<f64>,
    /// Negative sequence series reactance from terminal sequence 2 to terminal sequence 1. Used for short circuit data exchange according to IEC 60909. Usage: EquivalentBranch is a result of network reduction prior to the data exchange.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub negative_x21: Option<f64>,
    /// Positive sequence series resistance from terminal sequence 1 to terminal sequence 2 . Used for short circuit data exchange according to IEC 60909. EquivalentBranch is a result of network reduction prior to the data exchange.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub positive_r12: Option<f64>,
    /// Positive sequence series resistance from terminal sequence 2 to terminal sequence 1. Used for short circuit data exchange according to IEC 60909. EquivalentBranch is a result of network reduction prior to the data exchange.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub positive_r21: Option<f64>,
    /// Positive sequence series reactance from terminal sequence 1 to terminal sequence 2. Used for short circuit data exchange according to IEC 60909. Usage : EquivalentBranch is a result of network reduction prior to the data exchange.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub positive_x12: Option<f64>,
    /// Positive sequence series reactance from terminal sequence 2 to terminal sequence 1. Used for short circuit data exchange according to IEC 60909. Usage : EquivalentBranch is a result of network reduction prior to the data exchange.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub positive_x21: Option<f64>,
    /// Positive sequence series resistance of the reduced branch.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r: Option<f64>,
    /// Resistance from terminal sequence 2 to terminal sequence 1 .Used for steady state power flow. This attribute is optional and represent unbalanced network such as off-nominal phase shifter. If only EquivalentBranch.r is given, then EquivalentBranch.r21 is assumed equal to EquivalentBranch.r. Usage rule : EquivalentBranch is a result of network reduction prior to the data exchange.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r21: Option<f64>,
    /// Positive sequence series reactance of the reduced branch.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x: Option<f64>,
    /// Reactance from terminal sequence 2 to terminal sequence 1. Used for steady state power flow. This attribute is optional and represents an unbalanced network such as off-nominal phase shifter. If only EquivalentBranch.x is given, then EquivalentBranch.x21 is assumed equal to EquivalentBranch.x. Usage rule: EquivalentBranch is a result of network reduction prior to the data exchange.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x21: Option<f64>,
    /// Zero sequence series resistance from terminal sequence 1 to terminal sequence 2. Used for short circuit data exchange according to IEC 60909. EquivalentBranch is a result of network reduction prior to the data exchange.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zero_r12: Option<f64>,
    /// Zero sequence series resistance from terminal sequence 2 to terminal sequence 1. Used for short circuit data exchange according to IEC 60909. Usage : EquivalentBranch is a result of network reduction prior to the data exchange.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zero_r21: Option<f64>,
    /// Zero sequence series reactance from terminal sequence 1 to terminal sequence 2. Used for short circuit data exchange according to IEC 60909. Usage : EquivalentBranch is a result of network reduction prior to the data exchange.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zero_x12: Option<f64>,
    /// Zero sequence series reactance from terminal sequence 2 to terminal sequence 1. Used for short circuit data exchange according to IEC 60909. Usage : EquivalentBranch is a result of network reduction prior to the data exchange.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zero_x21: Option<f64>,
}
impl crate::base::CimElement for EquivalentBranch {
    fn mrid(&self) -> &str { &self.base.base.base.base.base.id }
    fn type_name(&self) -> &'static str { "EquivalentBranch" }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn to_json_value(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or(serde_json::Value::Null)
    }
    fn to_block(&self) -> crate::base::RdfBlock {
        let mut block = self.base.to_block();
        block.type_name = "EquivalentBranch".to_string();
        if let Some(v) = self.negative_r12 {
            block.fields.insert("EquivalentBranch.negativeR12".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.negative_r21 {
            block.fields.insert("EquivalentBranch.negativeR21".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.negative_x12 {
            block.fields.insert("EquivalentBranch.negativeX12".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.negative_x21 {
            block.fields.insert("EquivalentBranch.negativeX21".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.positive_r12 {
            block.fields.insert("EquivalentBranch.positiveR12".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.positive_r21 {
            block.fields.insert("EquivalentBranch.positiveR21".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.positive_x12 {
            block.fields.insert("EquivalentBranch.positiveX12".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.positive_x21 {
            block.fields.insert("EquivalentBranch.positiveX21".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.r {
            block.fields.insert("EquivalentBranch.r".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.r21 {
            block.fields.insert("EquivalentBranch.r21".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.x {
            block.fields.insert("EquivalentBranch.x".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.x21 {
            block.fields.insert("EquivalentBranch.x21".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.zero_r12 {
            block.fields.insert("EquivalentBranch.zeroR12".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.zero_r21 {
            block.fields.insert("EquivalentBranch.zeroR21".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.zero_x12 {
            block.fields.insert("EquivalentBranch.zeroX12".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.zero_x21 {
            block.fields.insert("EquivalentBranch.zeroX21".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        block
    }
}

impl EquivalentBranch {
    pub fn from_block(b: &crate::base::RdfBlock) -> Self {
        let mut obj = Self::default();
        obj.base.base.base.base.base.id.clone_from(&b.mrid);
        for (key, val) in &b.fields {
            match key.as_str() {
                "EquivalentBranch.negativeR12" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.negative_r12 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.negative_r12 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "EquivalentBranch.negativeR21" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.negative_r21 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.negative_r21 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "EquivalentBranch.negativeX12" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.negative_x12 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.negative_x12 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "EquivalentBranch.negativeX21" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.negative_x21 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.negative_x21 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "EquivalentBranch.positiveR12" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.positive_r12 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.positive_r12 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "EquivalentBranch.positiveR21" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.positive_r21 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.positive_r21 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "EquivalentBranch.positiveX12" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.positive_x12 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.positive_x12 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "EquivalentBranch.positiveX21" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.positive_x21 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.positive_x21 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "EquivalentBranch.r" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.r = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.r = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "EquivalentBranch.r21" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.r21 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.r21 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "EquivalentBranch.x" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.x = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.x = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "EquivalentBranch.x21" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.x21 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.x21 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "EquivalentBranch.zeroR12" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.zero_r12 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.zero_r12 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "EquivalentBranch.zeroR21" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.zero_r21 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.zero_r21 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "EquivalentBranch.zeroX12" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.zero_x12 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.zero_x12 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "EquivalentBranch.zeroX21" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.zero_x21 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.zero_x21 = Some(v); } }
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
