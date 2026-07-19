/// A control area is a grouping of generating units and/or loads and a cutset of tie lines (as terminals) which may be used for a variety of purposes including automatic generation control, power flow solution area interchange control specification, and input to load forecasting. All generation and load within the area defined by the terminals on the border are considered in the area interchange control. Note that any number of overlapping control area specifications can be superimposed on the physical model. The following general principles apply to ControlArea: 1. The control area orientation for net interchange is positive for an import, negative for an export. 2. The control area net interchange is determined by summing flows in Terminals. The Terminals are identified by creating a set of TieFlow objects associated with a ControlArea object. Each TieFlow object identifies one Terminal. 3. In a single network model, a tie between two control areas must be modelled in both control area specifications, such that the two representations of the tie flow sum to zero. 4. The normal orientation of Terminal flow is positive for flow into the conducting equipment that owns the Terminal. (i.e. flow from a bus into a device is positive.) However, the orientation of each flow in the control area specification must align with the control area convention, i.e. import is positive. If the orientation of the Terminal flow referenced by a TieFlow is positive into the control area, then this is confirmed by setting TieFlow.positiveFlowIn flag TRUE. If not, the orientation must be reversed by setting the TieFlow.positiveFlowIn flag FALSE.
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct ControlArea {
    #[serde(flatten)]
    pub base: super::PowerSystemResource,
    /// The energy area that is forecast from this control area specification.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub energy_area: Option<super::base::MridRef>,
    /// The specified positive net interchange into the control area, i.e. positive sign means flow into the area.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub net_interchange: Option<f64>,
    /// Active power net interchange tolerance. The attribute shall be a positive value or zero.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub p_tolerance: Option<f64>,
    /// The primary type of control area definition used to determine if this is used for automatic generation control, for planning interchange control, or other purposes. A control area specified with primary type of automatic generation control could still be forecast and used as an interchange area in power flow analysis.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<super::base::UriRef>,
}
impl crate::base::CimElement for ControlArea {
    fn mrid(&self) -> &str { &self.base.base.id }
    fn type_name(&self) -> &'static str { "ControlArea" }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn to_json_value(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or(serde_json::Value::Null)
    }
    fn to_block(&self) -> crate::base::RdfBlock {
        let mut block = self.base.to_block();
        block.type_name = "ControlArea".to_string();
        if let Some(ref v) = self.energy_area {
            block.fields.insert("ControlArea.EnergyArea".into(), crate::base::FieldValue::Resource(v.mrid.clone()));
        }
        if let Some(v) = self.net_interchange {
            block.fields.insert("ControlArea.netInterchange".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.p_tolerance {
            block.fields.insert("ControlArea.pTolerance".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(ref v) = self.type_ {
            block.fields.insert("ControlArea.type".into(), crate::base::FieldValue::Resource(v.uri.clone()));
        }
        block
    }
}

impl ControlArea {
    pub fn from_block(b: &crate::base::RdfBlock) -> Self {
        let mut obj = Self::default();
        obj.base.base.id.clone_from(&b.mrid);
        for (key, val) in &b.fields {
            match key.as_str() {
                "ControlArea.EnergyArea" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.energy_area = Some(crate::base::MridRef { mrid: sv.clone() });
                    }
                }
                "ControlArea.netInterchange" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.net_interchange = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.net_interchange = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ControlArea.pTolerance" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.p_tolerance = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.p_tolerance = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "ControlArea.type" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.type_ = Some(crate::base::UriRef { uri: sv.clone() });
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
