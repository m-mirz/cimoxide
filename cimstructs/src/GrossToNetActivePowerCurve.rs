/// Relationship between the generating unit's gross active power output on the X-axis (measured at the terminals of the machine(s)) and the generating unit's net active power output on the Y-axis (based on utility-defined measurements at the power station). Station service loads, when modelled, should be treated as non-conforming bus loads. There may be more than one curve, depending on the auxiliary equipment that is in service.
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct GrossToNetActivePowerCurve {
    #[serde(flatten)]
    pub base: super::Curve,
    /// A generating unit may have a gross active power to net active power curve, describing the losses and auxiliary power requirements of the unit.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generating_unit: Option<super::base::MridRef>,
}
impl crate::base::CimElement for GrossToNetActivePowerCurve {
    fn mrid(&self) -> &str { &self.base.base.id }
    fn type_name(&self) -> &'static str { "GrossToNetActivePowerCurve" }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn to_json_value(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or(serde_json::Value::Null)
    }
    fn to_block(&self) -> crate::base::RdfBlock {
        let mut block = self.base.to_block();
        block.type_name = "GrossToNetActivePowerCurve".to_string();
        if let Some(ref v) = self.generating_unit {
            block.fields.insert("GrossToNetActivePowerCurve.GeneratingUnit".into(), crate::base::FieldValue::Resource(v.mrid.clone()));
        }
        block
    }
}

impl GrossToNetActivePowerCurve {
    pub fn from_block(b: &crate::base::RdfBlock) -> Self {
        let mut obj = Self::default();
        obj.base.base.id.clone_from(&b.mrid);
        for (key, val) in &b.fields {
            match key.as_str() {
                "GrossToNetActivePowerCurve.GeneratingUnit" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.generating_unit = Some(crate::base::MridRef { mrid: sv.clone() });
                    }
                }
                "Curve.curveStyle" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.base.curve_style = Some(crate::base::UriRef { uri: sv.clone() });
                    }
                }
                "Curve.xUnit" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.base.x_unit = Some(crate::base::UriRef { uri: sv.clone() });
                    }
                }
                "Curve.y1Unit" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.base.y1unit = Some(crate::base::UriRef { uri: sv.clone() });
                    }
                }
                "Curve.y2Unit" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.base.y2unit = Some(crate::base::UriRef { uri: sv.clone() });
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
