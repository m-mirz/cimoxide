/// Models the characteristic response of the load demand due to changes in system conditions such as voltage and frequency. It is not related to demand response. If LoadResponseCharacteristic.exponentModel is True, the exponential voltage or frequency dependent models are specified and used as to calculate active and reactive power components of the load model. The equations to calculate active and reactive power components of the load model are internal to the power flow calculation, hence they use different quantities depending on the use case of the data exchange. The equations for exponential voltage dependent load model injected power are: pInjection= Pnominal* (Voltage/cim:BaseVoltage.nominalVoltage) ** cim:LoadResponseCharacteristic.pVoltageExponent qInjection= Qnominal* (Voltage/cim:BaseVoltage.nominalVoltage) ** cim:LoadResponseCharacteristic.qVoltageExponent Where: 1) * means 'multiply' and ** is 'raised to power of'; 2) Pnominal and Qnominal represent the active power and reactive power at nominal voltage as any load described by the voltage exponential model shall be given at nominal voltage. This means that EnergyConsumer.p and EnergyConsumer.q are at nominal voltage. 3) After power flow is solved: -pInjection and qInjection correspond to SvPowerflow.p and SvPowerflow.q respectively. - Voltage corresponds to SvVoltage.v at the TopologicalNode where the load is connected.
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct LoadResponseCharacteristic {
    #[serde(flatten)]
    pub base: super::IdentifiedObject,
    /// Indicates the exponential voltage dependency model is to be used. If false, the coefficient model is to be used. The exponential voltage dependency model consist of the attributes: - pVoltageExponent - qVoltageExponent - pFrequencyExponent - qFrequencyExponent. The coefficient model consist of the attributes: - pConstantImpedance - pConstantCurrent - pConstantPower - qConstantImpedance - qConstantCurrent - qConstantPower. The sum of pConstantImpedance, pConstantCurrent and pConstantPower shall equal 1. The sum of qConstantImpedance, qConstantCurrent and qConstantPower shall equal 1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exponent_model: Option<bool>,
    /// Portion of active power load modelled as constant current.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub p_constant_current: Option<f64>,
    /// Portion of active power load modelled as constant impedance.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub p_constant_impedance: Option<f64>,
    /// Portion of active power load modelled as constant power.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub p_constant_power: Option<f64>,
    /// Exponent of per unit frequency effecting active power.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub p_frequency_exponent: Option<f64>,
    /// Exponent of per unit voltage effecting real power.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub p_voltage_exponent: Option<f64>,
    /// Portion of reactive power load modelled as constant current.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub q_constant_current: Option<f64>,
    /// Portion of reactive power load modelled as constant impedance.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub q_constant_impedance: Option<f64>,
    /// Portion of reactive power load modelled as constant power.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub q_constant_power: Option<f64>,
    /// Exponent of per unit frequency effecting reactive power.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub q_frequency_exponent: Option<f64>,
    /// Exponent of per unit voltage effecting reactive power.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub q_voltage_exponent: Option<f64>,
}
impl crate::base::CimElement for LoadResponseCharacteristic {
    fn mrid(&self) -> &str { &self.base.id }
    fn type_name(&self) -> &'static str { "LoadResponseCharacteristic" }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn to_json_value(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or(serde_json::Value::Null)
    }
    fn to_block(&self) -> crate::base::RdfBlock {
        let mut block = self.base.to_block();
        block.type_name = "LoadResponseCharacteristic".to_string();
        if let Some(v) = self.exponent_model {
            block.fields.insert("LoadResponseCharacteristic.exponentModel".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.p_constant_current {
            block.fields.insert("LoadResponseCharacteristic.pConstantCurrent".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.p_constant_impedance {
            block.fields.insert("LoadResponseCharacteristic.pConstantImpedance".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.p_constant_power {
            block.fields.insert("LoadResponseCharacteristic.pConstantPower".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.p_frequency_exponent {
            block.fields.insert("LoadResponseCharacteristic.pFrequencyExponent".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.p_voltage_exponent {
            block.fields.insert("LoadResponseCharacteristic.pVoltageExponent".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.q_constant_current {
            block.fields.insert("LoadResponseCharacteristic.qConstantCurrent".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.q_constant_impedance {
            block.fields.insert("LoadResponseCharacteristic.qConstantImpedance".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.q_constant_power {
            block.fields.insert("LoadResponseCharacteristic.qConstantPower".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.q_frequency_exponent {
            block.fields.insert("LoadResponseCharacteristic.qFrequencyExponent".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.q_voltage_exponent {
            block.fields.insert("LoadResponseCharacteristic.qVoltageExponent".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        block
    }
}

impl LoadResponseCharacteristic {
    pub fn from_block(b: &crate::base::RdfBlock) -> Self {
        let mut obj = Self::default();
        obj.base.id.clone_from(&b.mrid);
        for (key, val) in &b.fields {
            match key.as_str() {
                "LoadResponseCharacteristic.exponentModel" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.exponent_model = Some(sv.trim() == "true"); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.exponent_model = Some(sv.trim() == "true"); }
                        }
                        _ => {}
                    }
                }
                "LoadResponseCharacteristic.pConstantCurrent" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.p_constant_current = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.p_constant_current = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "LoadResponseCharacteristic.pConstantImpedance" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.p_constant_impedance = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.p_constant_impedance = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "LoadResponseCharacteristic.pConstantPower" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.p_constant_power = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.p_constant_power = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "LoadResponseCharacteristic.pFrequencyExponent" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.p_frequency_exponent = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.p_frequency_exponent = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "LoadResponseCharacteristic.pVoltageExponent" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.p_voltage_exponent = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.p_voltage_exponent = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "LoadResponseCharacteristic.qConstantCurrent" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.q_constant_current = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.q_constant_current = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "LoadResponseCharacteristic.qConstantImpedance" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.q_constant_impedance = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.q_constant_impedance = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "LoadResponseCharacteristic.qConstantPower" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.q_constant_power = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.q_constant_power = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "LoadResponseCharacteristic.qFrequencyExponent" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.q_frequency_exponent = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.q_frequency_exponent = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "LoadResponseCharacteristic.qVoltageExponent" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.q_voltage_exponent = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.q_voltage_exponent = Some(v); } }
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
