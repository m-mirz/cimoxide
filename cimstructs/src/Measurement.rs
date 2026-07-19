/// A Measurement represents any measured, calculated or non-measured non-calculated quantity. Any piece of equipment may contain Measurements, e.g. a substation may have temperature measurements and door open indications, a transformer may have oil temperature and tank pressure measurements, a bay may contain a number of power flow measurements and a Breaker may contain a switch status measurement. The PSR - Measurement association is intended to capture this use of Measurement and is included in the naming hierarchy based on EquipmentContainer. The naming hierarchy typically has Measurements as leaves, e.g. Substation-VoltageLevel-Bay-Switch-Measurement. Some Measurements represent quantities related to a particular sensor location in the network, e.g. a voltage transformer (VT) or potential transformer (PT) at a busbar or a current transformer (CT) at the bar between a breaker and an isolator. The sensing position is not captured in the PSR - Measurement association. Instead it is captured by the Measurement - Terminal association that is used to define the sensing location in the network topology. The location is defined by the connection of the Terminal to ConductingEquipment. If both a Terminal and PSR are associated, and the PSR is of type ConductingEquipment, the associated Terminal should belong to that ConductingEquipment instance. When the sensor location is needed both Measurement-PSR and Measurement-Terminal are used. The Measurement-Terminal association is never used alone.
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct Measurement {
    #[serde(flatten)]
    pub base: super::IdentifiedObject,
    /// The power system resource that contains the measurement.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub power_system_resource: Option<super::base::MridRef>,
    /// One or more measurements may be associated with a terminal in the network.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub terminal: Option<super::base::MridRef>,
    /// Specifies the type of measurement. For example, this specifies if the measurement represents an indoor temperature, outdoor temperature, bus voltage, line flow, etc. When the measurementType is set to 'Specialization', the type of Measurement is defined in more detail by the specialized class which inherits from Measurement.
    pub measurement_type: String,
    /// Indicates to which phases the measurement applies and avoids the need to use 'measurementType' to also encode phase information (which would explode the types). The phase information in Measurement, along with 'measurementType' and 'phases' uniquely defines a Measurement for a device, based on normal network phase. Their meaning will not change when the computed energizing phasing is changed due to jumpers or other reasons. If the attribute is missing three phases (ABC) shall be assumed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phases: Option<super::base::UriRef>,
    /// The unit multiplier of the measured quantity.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_multiplier: Option<super::base::UriRef>,
    /// The unit of measure of the measured quantity.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_symbol: Option<super::base::UriRef>,
}
impl crate::base::CimElement for Measurement {
    fn mrid(&self) -> &str { &self.base.id }
    fn type_name(&self) -> &'static str { "Measurement" }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn to_json_value(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or(serde_json::Value::Null)
    }
    fn to_block(&self) -> crate::base::RdfBlock {
        let mut block = self.base.to_block();
        block.type_name = "Measurement".to_string();
        if let Some(ref v) = self.power_system_resource {
            block.fields.insert("Measurement.PowerSystemResource".into(), crate::base::FieldValue::Resource(v.mrid.clone()));
        }
        if let Some(ref v) = self.terminal {
            block.fields.insert("Measurement.Terminal".into(), crate::base::FieldValue::Resource(v.mrid.clone()));
        }
        if !self.measurement_type.is_empty() {
            block.fields.insert("Measurement.measurementType".into(), crate::base::FieldValue::Text(self.measurement_type.clone()));
        }
        if let Some(ref v) = self.phases {
            block.fields.insert("Measurement.phases".into(), crate::base::FieldValue::Resource(v.uri.clone()));
        }
        if let Some(ref v) = self.unit_multiplier {
            block.fields.insert("Measurement.unitMultiplier".into(), crate::base::FieldValue::Resource(v.uri.clone()));
        }
        if let Some(ref v) = self.unit_symbol {
            block.fields.insert("Measurement.unitSymbol".into(), crate::base::FieldValue::Resource(v.uri.clone()));
        }
        block
    }
}

impl Measurement {
    pub fn from_block(b: &crate::base::RdfBlock) -> Self {
        let mut obj = Self::default();
        obj.base.id.clone_from(&b.mrid);
        for (key, val) in &b.fields {
            match key.as_str() {
                "Measurement.PowerSystemResource" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.power_system_resource = Some(crate::base::MridRef { mrid: sv.clone() });
                    }
                }
                "Measurement.Terminal" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.terminal = Some(crate::base::MridRef { mrid: sv.clone() });
                    }
                }
                "Measurement.measurementType" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.measurement_type = sv.clone(); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.measurement_type = sv.clone(); }
                        }
                        _ => {}
                    }
                }
                "Measurement.phases" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.phases = Some(crate::base::UriRef { uri: sv.clone() });
                    }
                }
                "Measurement.unitMultiplier" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.unit_multiplier = Some(crate::base::UriRef { uri: sv.clone() });
                    }
                }
                "Measurement.unitSymbol" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.unit_symbol = Some(crate::base::UriRef { uri: sv.clone() });
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
