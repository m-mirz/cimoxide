/// Discrete represents a discrete Measurement, i.e. a Measurement representing discrete values, e.g. a Breaker position.
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct Discrete {
    #[serde(flatten)]
    pub base: super::Measurement,
    /// The ValueAliasSet used for translation of a MeasurementValue.value to a name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value_alias_set: Option<super::base::MridRef>,
}
impl crate::base::CimElement for Discrete {
    fn mrid(&self) -> &str { &self.base.base.id }
    fn type_name(&self) -> &'static str { "Discrete" }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn to_json_value(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or(serde_json::Value::Null)
    }
    fn to_block(&self) -> crate::base::RdfBlock {
        let mut block = self.base.to_block();
        block.type_name = "Discrete".to_string();
        if let Some(ref v) = self.value_alias_set {
            block.fields.insert("Discrete.ValueAliasSet".into(), crate::base::FieldValue::Resource(v.mrid.clone()));
        }
        block
    }
}

impl Discrete {
    pub fn from_block(b: &crate::base::RdfBlock) -> Self {
        let mut obj = Self::default();
        obj.base.base.id.clone_from(&b.mrid);
        for (key, val) in &b.fields {
            match key.as_str() {
                "Discrete.ValueAliasSet" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.value_alias_set = Some(crate::base::MridRef { mrid: sv.clone() });
                    }
                }
                "Measurement.PowerSystemResource" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.base.power_system_resource = Some(crate::base::MridRef { mrid: sv.clone() });
                    }
                }
                "Measurement.Terminal" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.base.terminal = Some(crate::base::MridRef { mrid: sv.clone() });
                    }
                }
                "Measurement.measurementType" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.base.measurement_type = sv.clone(); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.base.measurement_type = sv.clone(); }
                        }
                        _ => {}
                    }
                }
                "Measurement.phases" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.base.phases = Some(crate::base::UriRef { uri: sv.clone() });
                    }
                }
                "Measurement.unitMultiplier" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.base.unit_multiplier = Some(crate::base::UriRef { uri: sv.clone() });
                    }
                }
                "Measurement.unitSymbol" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.base.unit_symbol = Some(crate::base::UriRef { uri: sv.clone() });
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
