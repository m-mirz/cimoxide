/// DC side of the current source converter (CSC). The firing angle controls the dc voltage at the converter, both for rectifier and inverter. The difference between the dc voltages of the rectifier and inverter determines the dc current. The extinction angle is used to limit the dc voltage at the inverter, if needed, and is not used in active power control. The firing angle, transformer tap position and number of connected filters are the primary means to control a current source dc line. Higher level controls are built on top, e.g. dc voltage, dc current and active power. From a steady state perspective it is sufficient to specify the wanted active power transfer (ACDCConverter.targetPpcc) and the control functions will set the dc voltage, dc current, firing angle, transformer tap position and number of connected filters to meet this. Therefore attributes targetAlpha and targetGamma are not applicable in this case. The reactive power consumed by the converter is a function of the firing angle, transformer tap position and number of connected filter, which can be approximated with half of the active power. The losses is a function of the dc voltage and dc current. The attributes minAlpha and maxAlpha define the range of firing angles for rectifier operation between which no discrete tap changer action takes place. The range is typically 10-18 degrees. The attributes minGamma and maxGamma define the range of extinction angles for inverter operation between which no discrete tap changer action takes place. The range is typically 17-20 degrees.
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct CsConverter {
    #[serde(flatten)]
    pub base: super::ACDCConverter,
    /// Firing angle that determines the dc voltage at the converter dc terminal. Typical value between 10 degrees and 18 degrees for a rectifier. It is converter’s state variable, result from power flow. The attribute shall be a positive value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alpha: Option<f64>,
    /// Extinction angle. It is used to limit the dc voltage at the inverter if needed. Typical value between 17 degrees and 20 degrees for an inverter. It is converter’s state variable, result from power flow. The attribute shall be a positive value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gamma: Option<f64>,
    /// Maximum firing angle. It is converter’s configuration data used in power flow. The attribute shall be a positive value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_alpha: Option<f64>,
    /// Maximum extinction angle. It is converter’s configuration data used in power flow. The attribute shall be a positive value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_gamma: Option<f64>,
    /// The maximum direct current (Id) on the DC side at which the converter should operate. It is converter’s configuration data use in power flow. The attribute shall be a positive value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_idc: Option<f64>,
    /// Minimum firing angle. It is converter’s configuration data used in power flow. The attribute shall be a positive value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_alpha: Option<f64>,
    /// Minimum extinction angle. It is converter’s configuration data used in power flow. The attribute shall be a positive value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_gamma: Option<f64>,
    /// The minimum direct current (Id) on the DC side at which the converter should operate. It is converter’s configuration data used in power flow. The attribute shall be a positive value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_idc: Option<f64>,
    /// Indicates whether the DC pole is operating as an inverter or as a rectifier. It is converter’s control variable used in power flow.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operating_mode: Option<super::base::UriRef>,
    /// Kind of active power control.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub p_pcc_control: Option<super::base::UriRef>,
    /// Rated converter DC current, also called IdN. The attribute shall be a positive value. It is converter’s configuration data used in power flow.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rated_idc: Option<f64>,
    /// Target firing angle. It is converter’s control variable used in power flow. It is only applicable for rectifier if continuous tap changer control is used. Allowed values are within the range minAlpha<=targetAlpha<=maxAlpha. The attribute shall be a positive value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_alpha: Option<f64>,
    /// Target extinction angle. It is converter’s control variable used in power flow. It is only applicable for inverter if continuous tap changer control is used. Allowed values are within the range minGamma<=targetGamma<=maxGamma. The attribute shall be a positive value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_gamma: Option<f64>,
    /// DC current target value. It is converter’s control variable used in power flow. The attribute shall be a positive value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_idc: Option<f64>,
}
impl crate::base::CimElement for CsConverter {
    fn mrid(&self) -> &str { &self.base.base.base.base.base.id }
    fn type_name(&self) -> &'static str { "CsConverter" }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn to_json_value(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or(serde_json::Value::Null)
    }
    fn to_block(&self) -> crate::base::RdfBlock {
        let mut block = self.base.to_block();
        block.type_name = "CsConverter".to_string();
        if let Some(v) = self.alpha {
            block.fields.insert("CsConverter.alpha".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.gamma {
            block.fields.insert("CsConverter.gamma".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.max_alpha {
            block.fields.insert("CsConverter.maxAlpha".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.max_gamma {
            block.fields.insert("CsConverter.maxGamma".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.max_idc {
            block.fields.insert("CsConverter.maxIdc".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.min_alpha {
            block.fields.insert("CsConverter.minAlpha".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.min_gamma {
            block.fields.insert("CsConverter.minGamma".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.min_idc {
            block.fields.insert("CsConverter.minIdc".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(ref v) = self.operating_mode {
            block.fields.insert("CsConverter.operatingMode".into(), crate::base::FieldValue::Resource(v.uri.clone()));
        }
        if let Some(ref v) = self.p_pcc_control {
            block.fields.insert("CsConverter.pPccControl".into(), crate::base::FieldValue::Resource(v.uri.clone()));
        }
        if let Some(v) = self.rated_idc {
            block.fields.insert("CsConverter.ratedIdc".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.target_alpha {
            block.fields.insert("CsConverter.targetAlpha".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.target_gamma {
            block.fields.insert("CsConverter.targetGamma".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.target_idc {
            block.fields.insert("CsConverter.targetIdc".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        block
    }
}

impl CsConverter {
    pub fn from_block(b: &crate::base::RdfBlock) -> Self {
        let mut obj = Self::default();
        obj.base.base.base.base.base.id.clone_from(&b.mrid);
        for (key, val) in &b.fields {
            match key.as_str() {
                "CsConverter.alpha" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.alpha = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.alpha = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "CsConverter.gamma" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.gamma = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.gamma = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "CsConverter.maxAlpha" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.max_alpha = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.max_alpha = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "CsConverter.maxGamma" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.max_gamma = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.max_gamma = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "CsConverter.maxIdc" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.max_idc = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.max_idc = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "CsConverter.minAlpha" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.min_alpha = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.min_alpha = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "CsConverter.minGamma" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.min_gamma = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.min_gamma = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "CsConverter.minIdc" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.min_idc = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.min_idc = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "CsConverter.operatingMode" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.operating_mode = Some(crate::base::UriRef { uri: sv.clone() });
                    }
                }
                "CsConverter.pPccControl" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.p_pcc_control = Some(crate::base::UriRef { uri: sv.clone() });
                    }
                }
                "CsConverter.ratedIdc" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.rated_idc = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.rated_idc = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "CsConverter.targetAlpha" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.target_alpha = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.target_alpha = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "CsConverter.targetGamma" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.target_gamma = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.target_gamma = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "CsConverter.targetIdc" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.target_idc = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.target_idc = Some(v); } }
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
