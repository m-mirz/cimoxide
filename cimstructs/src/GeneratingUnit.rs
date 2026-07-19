/// A single or set of synchronous machines for converting mechanical power into alternating-current power. For example, individual machines within a set may be defined for scheduling purposes while a single control signal is derived for the set. In this case there would be a GeneratingUnit for each member of the set and an additional GeneratingUnit corresponding to the set.
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct GeneratingUnit {
    #[serde(flatten)]
    pub base: super::Equipment,
    /// The source of controls for a generating unit. Defines the control status of the generating unit.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gen_control_source: Option<super::base::UriRef>,
    /// Governor Speed Changer Droop. This is the change in generator power output divided by the change in frequency normalized by the nominal power of the generator and the nominal frequency and expressed in percent and negated. A positive value of speed change droop provides additional generator output upon a drop in frequency.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub governor_scd: Option<f64>,
    /// Generating unit long term economic participation factor.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub long_pf: Option<f64>,
    /// This is the maximum operating active power limit the dispatcher can enter for this unit.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_operating_p: Option<f64>,
    /// Maximum allowable spinning reserve. Spinning reserve will never be considered greater than this value regardless of the current operating point.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_allowable_spinning_reserve: Option<f64>,
    /// This is the minimum operating active power limit the dispatcher can enter for this unit.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_operating_p: Option<f64>,
    /// The nominal power of the generating unit. Used to give precise meaning to percentage based attributes such as the governor speed change droop (governorSCD attribute). The attribute shall be a positive value equal to or less than RotatingMachine.ratedS.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nominal_p: Option<f64>,
    /// Generating unit economic participation factor. The sum of the participation factors across generating units does not have to sum to one. It is used for representing distributed slack participation factor. The attribute shall be a positive value or zero.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub normal_pf: Option<f64>,
    /// The unit's gross rated maximum capacity (book value). The attribute shall be a positive value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rated_gross_max_p: Option<f64>,
    /// The gross rated minimum generation level which the unit can safely operate at while delivering power to the transmission grid. The attribute shall be a positive value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rated_gross_min_p: Option<f64>,
    /// The net rated maximum capacity determined by subtracting the auxiliary power used to operate the internal plant machinery from the rated gross maximum capacity. The attribute shall be a positive value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rated_net_max_p: Option<f64>,
    /// Generating unit short term economic participation factor.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub short_pf: Option<f64>,
    /// The initial startup cost incurred for each start of the GeneratingUnit.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub startup_cost: Option<f64>,
    /// Time it takes to get the unit on-line, from the time that the prime mover mechanical power is applied.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub startup_time: Option<f64>,
    /// The efficiency of the unit in converting the fuel into electrical energy.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_efficiency: Option<f64>,
    /// The variable cost component of production per unit of ActivePower.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variable_cost: Option<f64>,
}
impl crate::base::CimElement for GeneratingUnit {
    fn mrid(&self) -> &str { &self.base.base.base.id }
    fn type_name(&self) -> &'static str { "GeneratingUnit" }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn to_json_value(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or(serde_json::Value::Null)
    }
    fn to_block(&self) -> crate::base::RdfBlock {
        let mut block = self.base.to_block();
        block.type_name = "GeneratingUnit".to_string();
        if let Some(ref v) = self.gen_control_source {
            block.fields.insert("GeneratingUnit.genControlSource".into(), crate::base::FieldValue::Resource(v.uri.clone()));
        }
        if let Some(v) = self.governor_scd {
            block.fields.insert("GeneratingUnit.governorSCD".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.long_pf {
            block.fields.insert("GeneratingUnit.longPF".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.max_operating_p {
            block.fields.insert("GeneratingUnit.maxOperatingP".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.maximum_allowable_spinning_reserve {
            block.fields.insert("GeneratingUnit.maximumAllowableSpinningReserve".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.min_operating_p {
            block.fields.insert("GeneratingUnit.minOperatingP".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.nominal_p {
            block.fields.insert("GeneratingUnit.nominalP".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.normal_pf {
            block.fields.insert("GeneratingUnit.normalPF".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.rated_gross_max_p {
            block.fields.insert("GeneratingUnit.ratedGrossMaxP".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.rated_gross_min_p {
            block.fields.insert("GeneratingUnit.ratedGrossMinP".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.rated_net_max_p {
            block.fields.insert("GeneratingUnit.ratedNetMaxP".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.short_pf {
            block.fields.insert("GeneratingUnit.shortPF".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.startup_cost {
            block.fields.insert("GeneratingUnit.startupCost".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.startup_time {
            block.fields.insert("GeneratingUnit.startupTime".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.total_efficiency {
            block.fields.insert("GeneratingUnit.totalEfficiency".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.variable_cost {
            block.fields.insert("GeneratingUnit.variableCost".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        block
    }
}

impl GeneratingUnit {
    pub fn from_block(b: &crate::base::RdfBlock) -> Self {
        let mut obj = Self::default();
        obj.base.base.base.id.clone_from(&b.mrid);
        for (key, val) in &b.fields {
            match key.as_str() {
                "GeneratingUnit.genControlSource" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.gen_control_source = Some(crate::base::UriRef { uri: sv.clone() });
                    }
                }
                "GeneratingUnit.governorSCD" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.governor_scd = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.governor_scd = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GeneratingUnit.longPF" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.long_pf = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.long_pf = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GeneratingUnit.maxOperatingP" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.max_operating_p = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.max_operating_p = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GeneratingUnit.maximumAllowableSpinningReserve" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.maximum_allowable_spinning_reserve = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.maximum_allowable_spinning_reserve = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GeneratingUnit.minOperatingP" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.min_operating_p = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.min_operating_p = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GeneratingUnit.nominalP" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.nominal_p = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.nominal_p = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GeneratingUnit.normalPF" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.normal_pf = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.normal_pf = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GeneratingUnit.ratedGrossMaxP" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.rated_gross_max_p = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.rated_gross_max_p = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GeneratingUnit.ratedGrossMinP" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.rated_gross_min_p = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.rated_gross_min_p = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GeneratingUnit.ratedNetMaxP" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.rated_net_max_p = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.rated_net_max_p = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GeneratingUnit.shortPF" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.short_pf = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.short_pf = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GeneratingUnit.startupCost" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.startup_cost = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.startup_cost = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GeneratingUnit.startupTime" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.startup_time = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.startup_time = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GeneratingUnit.totalEfficiency" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.total_efficiency = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.total_efficiency = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GeneratingUnit.variableCost" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.variable_cost = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.variable_cost = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "Equipment.EquipmentContainer" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.base.equipment_container = Some(crate::base::MridRef { mrid: sv.clone() });
                    }
                }
                "Equipment.aggregate" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.base.aggregate = Some(sv.trim() == "true"); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.base.aggregate = Some(sv.trim() == "true"); }
                        }
                        _ => {}
                    }
                }
                "Equipment.inService" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.base.in_service = Some(sv.trim() == "true"); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.base.in_service = Some(sv.trim() == "true"); }
                        }
                        _ => {}
                    }
                }
                "Equipment.normallyInService" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.base.normally_in_service = Some(sv.trim() == "true"); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.base.normally_in_service = Some(sv.trim() == "true"); }
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
