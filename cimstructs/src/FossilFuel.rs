/// The fossil fuel consumed by the non-nuclear thermal generating unit. For example, coal, oil, gas, etc. These are the specific fuels that the generating unit can consume.
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct FossilFuel {
    #[serde(flatten)]
    pub base: super::IdentifiedObject,
    /// A thermal generating unit may have one or more fossil fuels.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thermal_generating_unit: Option<super::base::MridRef>,
    /// The type of fossil fuel, such as coal, oil, or gas.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fossil_fuel_type: Option<super::base::UriRef>,
}
impl crate::base::CimElement for FossilFuel {
    fn mrid(&self) -> &str { &self.base.id }
    fn type_name(&self) -> &'static str { "FossilFuel" }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn to_json_value(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or(serde_json::Value::Null)
    }
    fn to_block(&self) -> crate::base::RdfBlock {
        let mut block = self.base.to_block();
        block.type_name = "FossilFuel".to_string();
        if let Some(ref v) = self.thermal_generating_unit {
            block.fields.insert("FossilFuel.ThermalGeneratingUnit".into(), crate::base::FieldValue::Resource(v.mrid.clone()));
        }
        if let Some(ref v) = self.fossil_fuel_type {
            block.fields.insert("FossilFuel.fossilFuelType".into(), crate::base::FieldValue::Resource(v.uri.clone()));
        }
        block
    }
}

impl FossilFuel {
    pub fn from_block(b: &crate::base::RdfBlock) -> Self {
        let mut obj = Self::default();
        obj.base.id.clone_from(&b.mrid);
        for (key, val) in &b.fields {
            match key.as_str() {
                "FossilFuel.ThermalGeneratingUnit" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.thermal_generating_unit = Some(crate::base::MridRef { mrid: sv.clone() });
                    }
                }
                "FossilFuel.fossilFuelType" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.fossil_fuel_type = Some(crate::base::UriRef { uri: sv.clone() });
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
