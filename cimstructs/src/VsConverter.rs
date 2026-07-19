/// DC side of the voltage source converter (VSC).
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct VsConverter {
    #[serde(flatten)]
    pub base: super::ACDCConverter,
    /// Capability curve of this converter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capability_curve: Option<super::base::MridRef>,
    /// Angle between VsConverter.uv and ACDCConverter.uc. It is converter’s state variable used in power flow. The attribute shall be a positive value or zero.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delta: Option<f64>,
    /// Droop constant. The pu value is obtained as D [kV/MW] x Sb / Ubdc. The attribute shall be a positive value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub droop: Option<f64>,
    /// Compensation constant. Used to compensate for voltage drop when controlling voltage at a distant bus. The attribute shall be a positive value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub droop_compensation: Option<f64>,
    /// The maximum quotient between the AC converter voltage (Uc) and DC voltage (Ud). A factor typically less than 1. It is converter’s configuration data used in power flow.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_modulation_index: Option<f64>,
    /// Kind of control of real power and/or DC voltage.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub p_pcc_control: Option<super::base::UriRef>,
    /// Kind of reactive power control.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub q_pcc_control: Option<super::base::UriRef>,
    /// Reactive power sharing factor among parallel converters on Uac control. The attribute shall be a positive value or zero.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub q_share: Option<f64>,
    /// Magnitude of pulse-modulation factor. The attribute shall be a positive value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_pw_mfactor: Option<f64>,
    /// Phase target at AC side, at point of common coupling. The attribute shall be a positive value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_phase_pcc: Option<f64>,
    /// Power factor target at the AC side, at point of common coupling. The attribute shall be a positive value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_power_factor_pcc: Option<f64>,
    /// Reactive power injection target in AC grid, at point of common coupling. Load sign convention is used, i.e. positive sign means flow out from a node.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_qpcc: Option<f64>,
    /// Voltage target in AC grid, at point of common coupling. The attribute shall be a positive value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_upcc: Option<f64>,
    /// Line-to-line voltage on the valve side of the converter transformer. It is converter’s state variable, result from power flow. The attribute shall be a positive value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uv: Option<f64>,
}
impl crate::base::CimElement for VsConverter {
    fn mrid(&self) -> &str { &self.base.base.base.base.base.id }
    fn type_name(&self) -> &'static str { "VsConverter" }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn to_json_value(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or(serde_json::Value::Null)
    }
    fn to_block(&self) -> crate::base::RdfBlock {
        let mut block = self.base.to_block();
        block.type_name = "VsConverter".to_string();
        if let Some(ref v) = self.capability_curve {
            block.fields.insert("VsConverter.CapabilityCurve".into(), crate::base::FieldValue::Resource(v.mrid.clone()));
        }
        if let Some(v) = self.delta {
            block.fields.insert("VsConverter.delta".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.droop {
            block.fields.insert("VsConverter.droop".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.droop_compensation {
            block.fields.insert("VsConverter.droopCompensation".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.max_modulation_index {
            block.fields.insert("VsConverter.maxModulationIndex".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(ref v) = self.p_pcc_control {
            block.fields.insert("VsConverter.pPccControl".into(), crate::base::FieldValue::Resource(v.uri.clone()));
        }
        if let Some(ref v) = self.q_pcc_control {
            block.fields.insert("VsConverter.qPccControl".into(), crate::base::FieldValue::Resource(v.uri.clone()));
        }
        if let Some(v) = self.q_share {
            block.fields.insert("VsConverter.qShare".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.target_pw_mfactor {
            block.fields.insert("VsConverter.targetPWMfactor".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.target_phase_pcc {
            block.fields.insert("VsConverter.targetPhasePcc".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.target_power_factor_pcc {
            block.fields.insert("VsConverter.targetPowerFactorPcc".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.target_qpcc {
            block.fields.insert("VsConverter.targetQpcc".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.target_upcc {
            block.fields.insert("VsConverter.targetUpcc".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.uv {
            block.fields.insert("VsConverter.uv".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        block
    }
}

impl VsConverter {
    pub fn from_block(b: &crate::base::RdfBlock) -> Self {
        let mut obj = Self::default();
        obj.base.base.base.base.base.id.clone_from(&b.mrid);
        for (key, val) in &b.fields {
            match key.as_str() {
                "VsConverter.CapabilityCurve" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.capability_curve = Some(crate::base::MridRef { mrid: sv.clone() });
                    }
                }
                "VsConverter.delta" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.delta = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.delta = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "VsConverter.droop" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.droop = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.droop = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "VsConverter.droopCompensation" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.droop_compensation = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.droop_compensation = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "VsConverter.maxModulationIndex" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.max_modulation_index = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.max_modulation_index = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "VsConverter.pPccControl" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.p_pcc_control = Some(crate::base::UriRef { uri: sv.clone() });
                    }
                }
                "VsConverter.qPccControl" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.q_pcc_control = Some(crate::base::UriRef { uri: sv.clone() });
                    }
                }
                "VsConverter.qShare" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.q_share = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.q_share = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "VsConverter.targetPWMfactor" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.target_pw_mfactor = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.target_pw_mfactor = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "VsConverter.targetPhasePcc" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.target_phase_pcc = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.target_phase_pcc = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "VsConverter.targetPowerFactorPcc" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.target_power_factor_pcc = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.target_power_factor_pcc = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "VsConverter.targetQpcc" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.target_qpcc = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.target_qpcc = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "VsConverter.targetUpcc" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.target_upcc = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.target_upcc = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "VsConverter.uv" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.uv = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.uv = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ACDCConverter.PccTerminal" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.base.pcc_terminal = Some(crate::base::MridRef { mrid: sv.clone() });
                    }
                }
                "ACDCConverter.baseS" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.base.base_s = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.base.base_s = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ACDCConverter.idc" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.base.idc = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.base.idc = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ACDCConverter.idleLoss" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.base.idle_loss = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.base.idle_loss = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ACDCConverter.maxP" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.base.max_p = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.base.max_p = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ACDCConverter.maxUdc" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.base.max_udc = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.base.max_udc = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ACDCConverter.minP" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.base.min_p = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.base.min_p = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ACDCConverter.minUdc" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.base.min_udc = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.base.min_udc = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ACDCConverter.numberOfValves" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.base.number_of_valves = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.base.number_of_valves = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ACDCConverter.p" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.base.p = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.base.p = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ACDCConverter.poleLossP" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.base.pole_loss_p = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.base.pole_loss_p = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ACDCConverter.q" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.base.q = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.base.q = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ACDCConverter.ratedUdc" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.base.rated_udc = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.base.rated_udc = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ACDCConverter.resistiveLoss" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.base.resistive_loss = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.base.resistive_loss = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ACDCConverter.switchingLoss" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.base.switching_loss = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.base.switching_loss = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ACDCConverter.targetPpcc" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.base.target_ppcc = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.base.target_ppcc = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ACDCConverter.targetUdc" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.base.target_udc = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.base.target_udc = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ACDCConverter.uc" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.base.uc = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.base.uc = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ACDCConverter.udc" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.base.udc = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.base.udc = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ACDCConverter.valveU0" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.base.valve_u0 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.base.valve_u0 = Some(v); } }
                        }
                        _ => {}
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
