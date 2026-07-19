/// Specifies a set of equipment that works together to control a power system quantity such as voltage or flow. Remote bus voltage control is possible by specifying the controlled terminal located at some place remote from the controlling equipment. The specified terminal shall be associated with the connectivity node of the controlled point. The most specific subtype of RegulatingControl shall be used in case such equipment participate in the control, e.g. TapChangerControl for tap changers. For flow control, load sign convention is used, i.e. positive sign means flow out from a TopologicalNode (bus) into the conducting equipment. The attribute minAllowedTargetValue and maxAllowedTargetValue are required in the following cases: - For a power generating module operated in power factor control mode to specify maximum and minimum power factor values; - Whenever it is necessary to have an off center target voltage for the tap changer regulator. For instance, due to long cables to off shore wind farms and the need to have a simpler setup at the off shore transformer platform, the voltage is controlled from the land at the connection point for the off shore wind farm. Since there usually is a voltage rise along the cable, there is typical and overvoltage of up 3-4 kV compared to the on shore station. Thus in normal operation the tap changer on the on shore station is operated with a target set point, which is in the lower parts of the dead band. The attributes minAllowedTargetValue and maxAllowedTargetValue are not related to the attribute targetDeadband and thus they are not treated as an alternative of the targetDeadband. They are needed due to limitations in the local substation controller. The attribute targetDeadband is used to prevent the power flow from move the tap position in circles (hunting) that is to be used regardless of the attributes minAllowedTargetValue and maxAllowedTargetValue.
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct RegulatingControl {
    #[serde(flatten)]
    pub base: super::PowerSystemResource,
    /// The terminal associated with this regulating control. The terminal is associated instead of a node, since the terminal could connect into either a topological node or a connectivity node. Sometimes it is useful to model regulation at a terminal of a bus bar object.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub terminal: Option<super::base::MridRef>,
    /// The regulation is performed in a discrete mode. This applies to equipment with discrete controls, e.g. tap changers and shunt compensators.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discrete: Option<bool>,
    /// The flag tells if regulation is enabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// Maximum allowed target value (RegulatingControl.targetValue).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_allowed_target_value: Option<f64>,
    /// Minimum allowed target value (RegulatingControl.targetValue).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_allowed_target_value: Option<f64>,
    /// The regulating control mode presently available. This specification allows for determining the kind of regulation without need for obtaining the units from a schedule.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<super::base::UriRef>,
    /// This is a deadband used with discrete control to avoid excessive update of controls like tap changers and shunt compensator banks while regulating. The units of those appropriate for the mode. The attribute shall be a positive value or zero. If RegulatingControl.discrete is set to 'false', the RegulatingControl.targetDeadband is to be ignored. Note that for instance, if the targetValue is 100 kV and the targetDeadband is 2 kV the range is from 99 to 101 kV.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_deadband: Option<f64>,
    /// The target value specified for case input. This value can be used for the target value without the use of schedules. The value has the units appropriate to the mode attribute.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_value: Option<f64>,
    /// Specify the multiplier for used for the targetValue.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_value_unit_multiplier: Option<super::base::UriRef>,
}
impl crate::base::CimElement for RegulatingControl {
    fn mrid(&self) -> &str { &self.base.base.id }
    fn type_name(&self) -> &'static str { "RegulatingControl" }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn to_json_value(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or(serde_json::Value::Null)
    }
    fn to_block(&self) -> crate::base::RdfBlock {
        let mut block = self.base.to_block();
        block.type_name = "RegulatingControl".to_string();
        if let Some(ref v) = self.terminal {
            block.fields.insert("RegulatingControl.Terminal".into(), crate::base::FieldValue::Resource(v.mrid.clone()));
        }
        if let Some(v) = self.discrete {
            block.fields.insert("RegulatingControl.discrete".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.enabled {
            block.fields.insert("RegulatingControl.enabled".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.max_allowed_target_value {
            block.fields.insert("RegulatingControl.maxAllowedTargetValue".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.min_allowed_target_value {
            block.fields.insert("RegulatingControl.minAllowedTargetValue".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(ref v) = self.mode {
            block.fields.insert("RegulatingControl.mode".into(), crate::base::FieldValue::Resource(v.uri.clone()));
        }
        if let Some(v) = self.target_deadband {
            block.fields.insert("RegulatingControl.targetDeadband".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.target_value {
            block.fields.insert("RegulatingControl.targetValue".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(ref v) = self.target_value_unit_multiplier {
            block.fields.insert("RegulatingControl.targetValueUnitMultiplier".into(), crate::base::FieldValue::Resource(v.uri.clone()));
        }
        block
    }
}

impl RegulatingControl {
    pub fn from_block(b: &crate::base::RdfBlock) -> Self {
        let mut obj = Self::default();
        obj.base.base.id.clone_from(&b.mrid);
        for (key, val) in &b.fields {
            match key.as_str() {
                "RegulatingControl.Terminal" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.terminal = Some(crate::base::MridRef { mrid: sv.clone() });
                    }
                }
                "RegulatingControl.discrete" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.discrete = Some(sv.trim() == "true"); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.discrete = Some(sv.trim() == "true"); }
                        }
                        _ => {}
                    }
                }
                "RegulatingControl.enabled" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.enabled = Some(sv.trim() == "true"); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.enabled = Some(sv.trim() == "true"); }
                        }
                        _ => {}
                    }
                }
                "RegulatingControl.maxAllowedTargetValue" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.max_allowed_target_value = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.max_allowed_target_value = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "RegulatingControl.minAllowedTargetValue" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.min_allowed_target_value = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.min_allowed_target_value = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "RegulatingControl.mode" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.mode = Some(crate::base::UriRef { uri: sv.clone() });
                    }
                }
                "RegulatingControl.targetDeadband" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.target_deadband = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.target_deadband = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "RegulatingControl.targetValue" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.target_value = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.target_value = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "RegulatingControl.targetValueUnitMultiplier" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.target_value_unit_multiplier = Some(crate::base::UriRef { uri: sv.clone() });
                    }
                }
                "IdentifiedObject.description" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.base.base.description = sv.clone(); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.base.base.description = sv.clone(); }
                        }
                        _ => {}
                    }
                }
                "IdentifiedObject.energyIdentCodeEic" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.base.base.energy_ident_code_eic = sv.clone(); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.base.base.energy_ident_code_eic = sv.clone(); }
                        }
                        _ => {}
                    }
                }
                "IdentifiedObject.mRID" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.base.base.m_rid = sv.clone(); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.base.base.m_rid = sv.clone(); }
                        }
                        _ => {}
                    }
                }
                "IdentifiedObject.name" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.base.base.name = sv.clone(); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.base.base.name = sv.clone(); }
                        }
                        _ => {}
                    }
                }
                "IdentifiedObject.shortName" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.base.base.short_name = sv.clone(); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.base.base.short_name = sv.clone(); }
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
