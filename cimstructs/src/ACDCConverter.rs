/// A unit with valves for three phases, together with unit control equipment, essential protective and switching devices, DC storage capacitors, phase reactors and auxiliaries, if any, used for conversion.
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct ACDCConverter {
    #[serde(flatten)]
    pub base: super::ConductingEquipment,
    /// Point of common coupling terminal for this converter DC side. It is typically the terminal on the power transformer (or switch) closest to the AC network.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pcc_terminal: Option<super::base::MridRef>,
    /// Base apparent power of the converter pole. The attribute shall be a positive value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_s: Option<f64>,
    /// Converter DC current, also called Id. It is converter’s state variable, result from power flow.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idc: Option<f64>,
    /// Active power loss in pole at no power transfer. It is converter’s configuration data used in power flow. The attribute shall be a positive value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idle_loss: Option<f64>,
    /// Maximum active power limit. The value is overwritten by values of VsCapabilityCurve, if present.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_p: Option<f64>,
    /// The maximum voltage on the DC side at which the converter should operate. It is converter’s configuration data used in power flow. The attribute shall be a positive value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_udc: Option<f64>,
    /// Minimum active power limit. The value is overwritten by values of VsCapabilityCurve, if present.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_p: Option<f64>,
    /// The minimum voltage on the DC side at which the converter should operate. It is converter’s configuration data used in power flow. The attribute shall be a positive value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_udc: Option<f64>,
    /// Number of valves in the converter. Used in loss calculations.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_valves: Option<i64>,
    /// Active power at the point of common coupling. Load sign convention is used, i.e. positive sign means flow out from a node. Starting value for a steady state solution in the case a simplified power flow model is used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub p: Option<f64>,
    /// The active power loss at a DC Pole = idleLoss + switchingLoss*|Idc| + resitiveLoss*Idc^2. For lossless operation Pdc=Pac. For rectifier operation with losses Pdc=Pac-lossP. For inverter operation with losses Pdc=Pac+lossP. It is converter’s state variable used in power flow. The attribute shall be a positive value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pole_loss_p: Option<f64>,
    /// Reactive power at the point of common coupling. Load sign convention is used, i.e. positive sign means flow out from a node. Starting value for a steady state solution in the case a simplified power flow model is used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub q: Option<f64>,
    /// Rated converter DC voltage, also called UdN. The attribute shall be a positive value. It is converter’s configuration data used in power flow. For instance a bipolar HVDC link with value 200 kV has a 400kV difference between the dc lines.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rated_udc: Option<f64>,
    /// It is converter’s configuration data used in power flow. Refer to poleLossP. The attribute shall be a positive value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resistive_loss: Option<f64>,
    /// Switching losses, relative to the base apparent power 'baseS'. Refer to poleLossP. The attribute shall be a positive value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub switching_loss: Option<f64>,
    /// Real power injection target in AC grid, at point of common coupling. Load sign convention is used, i.e. positive sign means flow out from a node.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_ppcc: Option<f64>,
    /// Target value for DC voltage magnitude. The attribute shall be a positive value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_udc: Option<f64>,
    /// Line-to-line converter voltage, the voltage at the AC side of the valve. It is converter’s state variable, result from power flow. The attribute shall be a positive value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uc: Option<f64>,
    /// Converter voltage at the DC side, also called Ud. It is converter’s state variable, result from power flow. The attribute shall be a positive value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub udc: Option<f64>,
    /// Valve threshold voltage, also called Uvalve. Forward voltage drop when the valve is conducting. Used in loss calculations, i.e. the switchLoss depend on numberOfValves * valveU0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valve_u0: Option<f64>,
}
impl crate::base::CimElement for ACDCConverter {
    fn mrid(&self) -> &str { &self.base.base.base.base.id }
    fn type_name(&self) -> &'static str { "ACDCConverter" }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn to_json_value(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or(serde_json::Value::Null)
    }
    fn to_block(&self) -> crate::base::RdfBlock {
        let mut block = self.base.to_block();
        block.type_name = "ACDCConverter".to_string();
        if let Some(ref v) = self.pcc_terminal {
            block.fields.insert("ACDCConverter.PccTerminal".into(), crate::base::FieldValue::Resource(v.mrid.clone()));
        }
        if let Some(v) = self.base_s {
            block.fields.insert("ACDCConverter.baseS".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.idc {
            block.fields.insert("ACDCConverter.idc".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.idle_loss {
            block.fields.insert("ACDCConverter.idleLoss".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.max_p {
            block.fields.insert("ACDCConverter.maxP".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.max_udc {
            block.fields.insert("ACDCConverter.maxUdc".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.min_p {
            block.fields.insert("ACDCConverter.minP".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.min_udc {
            block.fields.insert("ACDCConverter.minUdc".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.number_of_valves {
            block.fields.insert("ACDCConverter.numberOfValves".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.p {
            block.fields.insert("ACDCConverter.p".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.pole_loss_p {
            block.fields.insert("ACDCConverter.poleLossP".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.q {
            block.fields.insert("ACDCConverter.q".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.rated_udc {
            block.fields.insert("ACDCConverter.ratedUdc".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.resistive_loss {
            block.fields.insert("ACDCConverter.resistiveLoss".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.switching_loss {
            block.fields.insert("ACDCConverter.switchingLoss".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.target_ppcc {
            block.fields.insert("ACDCConverter.targetPpcc".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.target_udc {
            block.fields.insert("ACDCConverter.targetUdc".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.uc {
            block.fields.insert("ACDCConverter.uc".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.udc {
            block.fields.insert("ACDCConverter.udc".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.valve_u0 {
            block.fields.insert("ACDCConverter.valveU0".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        block
    }
}

impl ACDCConverter {
    pub fn from_block(b: &crate::base::RdfBlock) -> Self {
        let mut obj = Self::default();
        obj.base.base.base.base.id.clone_from(&b.mrid);
        for (key, val) in &b.fields {
            match key.as_str() {
                "ACDCConverter.PccTerminal" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.pcc_terminal = Some(crate::base::MridRef { mrid: sv.clone() });
                    }
                }
                "ACDCConverter.baseS" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.base_s = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.base_s = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ACDCConverter.idc" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.idc = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.idc = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ACDCConverter.idleLoss" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.idle_loss = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.idle_loss = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ACDCConverter.maxP" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.max_p = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.max_p = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ACDCConverter.maxUdc" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.max_udc = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.max_udc = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ACDCConverter.minP" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.min_p = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.min_p = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ACDCConverter.minUdc" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.min_udc = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.min_udc = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ACDCConverter.numberOfValves" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.number_of_valves = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.number_of_valves = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ACDCConverter.p" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.p = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.p = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ACDCConverter.poleLossP" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.pole_loss_p = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.pole_loss_p = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ACDCConverter.q" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.q = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.q = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ACDCConverter.ratedUdc" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.rated_udc = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.rated_udc = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ACDCConverter.resistiveLoss" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.resistive_loss = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.resistive_loss = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ACDCConverter.switchingLoss" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.switching_loss = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.switching_loss = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ACDCConverter.targetPpcc" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.target_ppcc = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.target_ppcc = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ACDCConverter.targetUdc" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.target_udc = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.target_udc = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ACDCConverter.uc" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.uc = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.uc = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ACDCConverter.udc" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.udc = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.udc = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ACDCConverter.valveU0" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.valve_u0 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.valve_u0 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ConductingEquipment.BaseVoltage" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.base.base_voltage = Some(crate::base::MridRef { mrid: sv.clone() });
                    }
                }
                "Equipment.EquipmentContainer" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.base.base.equipment_container = Some(crate::base::MridRef { mrid: sv.clone() });
                    }
                }
                "Equipment.aggregate" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.base.base.aggregate = Some(sv.trim() == "true"); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.base.base.aggregate = Some(sv.trim() == "true"); }
                        }
                        _ => {}
                    }
                }
                "Equipment.inService" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.base.base.in_service = Some(sv.trim() == "true"); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.base.base.in_service = Some(sv.trim() == "true"); }
                        }
                        _ => {}
                    }
                }
                "Equipment.normallyInService" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.base.base.normally_in_service = Some(sv.trim() == "true"); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.base.base.normally_in_service = Some(sv.trim() == "true"); }
                        }
                        _ => {}
                    }
                }
                "IdentifiedObject.description" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.base.base.base.base.description = sv.clone(); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.base.base.base.base.description = sv.clone(); }
                        }
                        _ => {}
                    }
                }
                "IdentifiedObject.energyIdentCodeEic" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.base.base.base.base.energy_ident_code_eic = sv.clone(); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.base.base.base.base.energy_ident_code_eic = sv.clone(); }
                        }
                        _ => {}
                    }
                }
                "IdentifiedObject.mRID" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.base.base.base.base.m_rid = sv.clone(); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.base.base.base.base.m_rid = sv.clone(); }
                        }
                        _ => {}
                    }
                }
                "IdentifiedObject.name" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.base.base.base.base.name = sv.clone(); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.base.base.base.base.name = sv.clone(); }
                        }
                        _ => {}
                    }
                }
                "IdentifiedObject.shortName" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.base.base.base.base.short_name = sv.clone(); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.base.base.base.base.short_name = sv.clone(); }
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
