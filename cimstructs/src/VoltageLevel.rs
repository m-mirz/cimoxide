/// A collection of equipment at one common system voltage forming a switchgear. The equipment typically consists of breakers, busbars, instrumentation, control, regulation and protection devices as well as assemblies of all these.
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct VoltageLevel {
    #[serde(flatten)]
    pub base: super::EquipmentContainer,
    /// The base voltage used for all equipment within the voltage level.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_voltage: Option<super::base::MridRef>,
    /// The substation of the voltage level.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub substation: Option<super::base::MridRef>,
    /// The bus bar's high voltage limit. The limit applies to all equipment and nodes contained in a given VoltageLevel. It is not required that it is exchanged in pair with lowVoltageLimit. It is preferable to use operational VoltageLimit, which prevails, if present.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub high_voltage_limit: Option<f64>,
    /// The bus bar's low voltage limit. The limit applies to all equipment and nodes contained in a given VoltageLevel. It is not required that it is exchanged in pair with highVoltageLimit. It is preferable to use operational VoltageLimit, which prevails, if present.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub low_voltage_limit: Option<f64>,
}
impl crate::base::CimElement for VoltageLevel {
    fn mrid(&self) -> &str { &self.base.base.base.base.id }
    fn type_name(&self) -> &'static str { "VoltageLevel" }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn to_json_value(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or(serde_json::Value::Null)
    }
    fn to_block(&self) -> crate::base::RdfBlock {
        let mut block = self.base.to_block();
        block.type_name = "VoltageLevel".to_string();
        if let Some(ref v) = self.base_voltage {
            block.fields.insert("VoltageLevel.BaseVoltage".into(), crate::base::FieldValue::Resource(v.mrid.clone()));
        }
        if let Some(ref v) = self.substation {
            block.fields.insert("VoltageLevel.Substation".into(), crate::base::FieldValue::Resource(v.mrid.clone()));
        }
        if let Some(v) = self.high_voltage_limit {
            block.fields.insert("VoltageLevel.highVoltageLimit".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.low_voltage_limit {
            block.fields.insert("VoltageLevel.lowVoltageLimit".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        block
    }
}

impl VoltageLevel {
    pub fn from_block(b: &crate::base::RdfBlock) -> Self {
        let mut obj = Self::default();
        obj.base.base.base.base.id.clone_from(&b.mrid);
        for (key, val) in &b.fields {
            match key.as_str() {
                "VoltageLevel.BaseVoltage" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.base_voltage = Some(crate::base::MridRef { mrid: sv.clone() });
                    }
                }
                "VoltageLevel.Substation" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.substation = Some(crate::base::MridRef { mrid: sv.clone() });
                    }
                }
                "VoltageLevel.highVoltageLimit" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.high_voltage_limit = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.high_voltage_limit = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "VoltageLevel.lowVoltageLimit" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.low_voltage_limit = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.low_voltage_limit = Some(v); } }
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
