/// Look up table for the purpose of wind standard models.
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct WindDynamicsLookupTable {
    #[serde(flatten)]
    pub base: super::IdentifiedObject,
    /// The current control limitation model with which this wind dynamics lookup table is associated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wind_cont_curr_lim_iec: Option<super::base::MridRef>,
    /// The P control type 3 model with which this wind dynamics lookup table is associated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wind_cont_p_type3iec: Option<super::base::MridRef>,
    /// The QP and QU limitation model with which this wind dynamics lookup table is associated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wind_cont_qpqu_lim_iec: Option<super::base::MridRef>,
    /// The rotor resistance control model with which this wind dynamics lookup table is associated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wind_cont_rotor_riec: Option<super::base::MridRef>,
    /// The generator type 3B model with which this wind dynamics lookup table is associated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wind_gen_type3b_iec: Option<super::base::MridRef>,
    /// The pitch control power model with which this wind dynamics lookup table is associated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wind_pitch_cont_power_iec: Option<super::base::MridRef>,
    /// The frequency and active power wind plant control model with which this wind dynamics lookup table is associated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wind_plant_freq_pcontrol_iec: Option<super::base::MridRef>,
    /// The voltage and reactive power wind plant control model with which this wind dynamics lookup table is associated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wind_plant_reactive_control_iec: Option<super::base::MridRef>,
    /// The grid protection model with which this wind dynamics lookup table is associated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wind_protection_iec: Option<super::base::MridRef>,
    /// Input value (x) for the lookup table function.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input: Option<f64>,
    /// Type of the lookup table function.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lookup_table_function_type: Option<super::base::UriRef>,
    /// Output value (y) for the lookup table function.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output: Option<f64>,
    /// Sequence numbers of the pairs of the input (x) and the output (y) of the lookup table function.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sequence: Option<i64>,
}
impl crate::base::CimElement for WindDynamicsLookupTable {
    fn mrid(&self) -> &str { &self.base.id }
    fn type_name(&self) -> &'static str { "WindDynamicsLookupTable" }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn to_json_value(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or(serde_json::Value::Null)
    }
    fn to_block(&self) -> crate::base::RdfBlock {
        let mut block = self.base.to_block();
        block.type_name = "WindDynamicsLookupTable".to_string();
        if let Some(ref v) = self.wind_cont_curr_lim_iec {
            block.fields.insert("WindDynamicsLookupTable.WindContCurrLimIEC".into(), crate::base::FieldValue::Resource(v.mrid.clone()));
        }
        if let Some(ref v) = self.wind_cont_p_type3iec {
            block.fields.insert("WindDynamicsLookupTable.WindContPType3IEC".into(), crate::base::FieldValue::Resource(v.mrid.clone()));
        }
        if let Some(ref v) = self.wind_cont_qpqu_lim_iec {
            block.fields.insert("WindDynamicsLookupTable.WindContQPQULimIEC".into(), crate::base::FieldValue::Resource(v.mrid.clone()));
        }
        if let Some(ref v) = self.wind_cont_rotor_riec {
            block.fields.insert("WindDynamicsLookupTable.WindContRotorRIEC".into(), crate::base::FieldValue::Resource(v.mrid.clone()));
        }
        if let Some(ref v) = self.wind_gen_type3b_iec {
            block.fields.insert("WindDynamicsLookupTable.WindGenType3bIEC".into(), crate::base::FieldValue::Resource(v.mrid.clone()));
        }
        if let Some(ref v) = self.wind_pitch_cont_power_iec {
            block.fields.insert("WindDynamicsLookupTable.WindPitchContPowerIEC".into(), crate::base::FieldValue::Resource(v.mrid.clone()));
        }
        if let Some(ref v) = self.wind_plant_freq_pcontrol_iec {
            block.fields.insert("WindDynamicsLookupTable.WindPlantFreqPcontrolIEC".into(), crate::base::FieldValue::Resource(v.mrid.clone()));
        }
        if let Some(ref v) = self.wind_plant_reactive_control_iec {
            block.fields.insert("WindDynamicsLookupTable.WindPlantReactiveControlIEC".into(), crate::base::FieldValue::Resource(v.mrid.clone()));
        }
        if let Some(ref v) = self.wind_protection_iec {
            block.fields.insert("WindDynamicsLookupTable.WindProtectionIEC".into(), crate::base::FieldValue::Resource(v.mrid.clone()));
        }
        if let Some(v) = self.input {
            block.fields.insert("WindDynamicsLookupTable.input".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(ref v) = self.lookup_table_function_type {
            block.fields.insert("WindDynamicsLookupTable.lookupTableFunctionType".into(), crate::base::FieldValue::Resource(v.uri.clone()));
        }
        if let Some(v) = self.output {
            block.fields.insert("WindDynamicsLookupTable.output".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.sequence {
            block.fields.insert("WindDynamicsLookupTable.sequence".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        block
    }
}

impl WindDynamicsLookupTable {
    pub fn from_block(b: &crate::base::RdfBlock) -> Self {
        let mut obj = Self::default();
        obj.base.id.clone_from(&b.mrid);
        for (key, val) in &b.fields {
            match key.as_str() {
                "WindDynamicsLookupTable.WindContCurrLimIEC" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.wind_cont_curr_lim_iec = Some(crate::base::MridRef { mrid: sv.clone() });
                    }
                }
                "WindDynamicsLookupTable.WindContPType3IEC" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.wind_cont_p_type3iec = Some(crate::base::MridRef { mrid: sv.clone() });
                    }
                }
                "WindDynamicsLookupTable.WindContQPQULimIEC" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.wind_cont_qpqu_lim_iec = Some(crate::base::MridRef { mrid: sv.clone() });
                    }
                }
                "WindDynamicsLookupTable.WindContRotorRIEC" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.wind_cont_rotor_riec = Some(crate::base::MridRef { mrid: sv.clone() });
                    }
                }
                "WindDynamicsLookupTable.WindGenType3bIEC" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.wind_gen_type3b_iec = Some(crate::base::MridRef { mrid: sv.clone() });
                    }
                }
                "WindDynamicsLookupTable.WindPitchContPowerIEC" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.wind_pitch_cont_power_iec = Some(crate::base::MridRef { mrid: sv.clone() });
                    }
                }
                "WindDynamicsLookupTable.WindPlantFreqPcontrolIEC" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.wind_plant_freq_pcontrol_iec = Some(crate::base::MridRef { mrid: sv.clone() });
                    }
                }
                "WindDynamicsLookupTable.WindPlantReactiveControlIEC" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.wind_plant_reactive_control_iec = Some(crate::base::MridRef { mrid: sv.clone() });
                    }
                }
                "WindDynamicsLookupTable.WindProtectionIEC" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.wind_protection_iec = Some(crate::base::MridRef { mrid: sv.clone() });
                    }
                }
                "WindDynamicsLookupTable.input" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.input = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.input = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "WindDynamicsLookupTable.lookupTableFunctionType" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.lookup_table_function_type = Some(crate::base::UriRef { uri: sv.clone() });
                    }
                }
                "WindDynamicsLookupTable.output" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.output = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.output = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "WindDynamicsLookupTable.sequence" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.sequence = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.sequence = Some(v); } }
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
