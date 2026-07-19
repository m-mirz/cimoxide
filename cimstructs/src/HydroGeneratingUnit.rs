/// A generating unit whose prime mover is a hydraulic turbine (e.g., Francis, Pelton, Kaplan).
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct HydroGeneratingUnit {
    #[serde(flatten)]
    pub base: super::GeneratingUnit,
    /// The hydro generating unit belongs to a hydro power plant.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hydro_power_plant: Option<super::base::MridRef>,
    /// The height water drops from the reservoir mid-point to the turbine.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub drop_height: Option<f64>,
    /// Energy conversion capability for generating.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub energy_conversion_capability: Option<super::base::UriRef>,
    /// Type of turbine.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub turbine_type: Option<super::base::UriRef>,
}
impl crate::base::CimElement for HydroGeneratingUnit {
    fn mrid(&self) -> &str { &self.base.base.base.base.id }
    fn type_name(&self) -> &'static str { "HydroGeneratingUnit" }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn to_json_value(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or(serde_json::Value::Null)
    }
    fn to_block(&self) -> crate::base::RdfBlock {
        let mut block = self.base.to_block();
        block.type_name = "HydroGeneratingUnit".to_string();
        if let Some(ref v) = self.hydro_power_plant {
            block.fields.insert("HydroGeneratingUnit.HydroPowerPlant".into(), crate::base::FieldValue::Resource(v.mrid.clone()));
        }
        if let Some(v) = self.drop_height {
            block.fields.insert("HydroGeneratingUnit.dropHeight".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(ref v) = self.energy_conversion_capability {
            block.fields.insert("HydroGeneratingUnit.energyConversionCapability".into(), crate::base::FieldValue::Resource(v.uri.clone()));
        }
        if let Some(ref v) = self.turbine_type {
            block.fields.insert("HydroGeneratingUnit.turbineType".into(), crate::base::FieldValue::Resource(v.uri.clone()));
        }
        block
    }
}

impl HydroGeneratingUnit {
    pub fn from_block(b: &crate::base::RdfBlock) -> Self {
        let mut obj = Self::default();
        obj.base.base.base.base.id.clone_from(&b.mrid);
        for (key, val) in &b.fields {
            match key.as_str() {
                "HydroGeneratingUnit.HydroPowerPlant" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.hydro_power_plant = Some(crate::base::MridRef { mrid: sv.clone() });
                    }
                }
                "HydroGeneratingUnit.dropHeight" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.drop_height = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.drop_height = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "HydroGeneratingUnit.energyConversionCapability" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.energy_conversion_capability = Some(crate::base::UriRef { uri: sv.clone() });
                    }
                }
                "HydroGeneratingUnit.turbineType" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.turbine_type = Some(crate::base::UriRef { uri: sv.clone() });
                    }
                }
                "GeneratingUnit.genControlSource" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.base.gen_control_source = Some(crate::base::UriRef { uri: sv.clone() });
                    }
                }
                "GeneratingUnit.governorSCD" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.base.governor_scd = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.base.governor_scd = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GeneratingUnit.longPF" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.base.long_pf = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.base.long_pf = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GeneratingUnit.maxOperatingP" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.base.max_operating_p = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.base.max_operating_p = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GeneratingUnit.maximumAllowableSpinningReserve" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.base.maximum_allowable_spinning_reserve = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.base.maximum_allowable_spinning_reserve = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GeneratingUnit.minOperatingP" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.base.min_operating_p = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.base.min_operating_p = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GeneratingUnit.nominalP" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.base.nominal_p = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.base.nominal_p = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GeneratingUnit.normalPF" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.base.normal_pf = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.base.normal_pf = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GeneratingUnit.ratedGrossMaxP" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.base.rated_gross_max_p = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.base.rated_gross_max_p = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GeneratingUnit.ratedGrossMinP" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.base.rated_gross_min_p = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.base.rated_gross_min_p = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GeneratingUnit.ratedNetMaxP" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.base.rated_net_max_p = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.base.rated_net_max_p = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GeneratingUnit.shortPF" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.base.short_pf = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.base.short_pf = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GeneratingUnit.startupCost" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.base.startup_cost = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.base.startup_cost = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GeneratingUnit.startupTime" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.base.startup_time = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.base.startup_time = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GeneratingUnit.totalEfficiency" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.base.total_efficiency = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.base.total_efficiency = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "GeneratingUnit.variableCost" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.base.variable_cost = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.base.variable_cost = Some(v); } }
                        }
                        _ => {}
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
