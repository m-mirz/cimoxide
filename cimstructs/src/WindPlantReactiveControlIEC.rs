/// Simplified plant voltage and reactive power control model for use with type 3 and type 4 wind turbine models. Reference: IEC 61400-27-1:2015, Annex D.
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct WindPlantReactiveControlIEC {
    #[serde(flatten)]
    pub base: super::IdentifiedObject,
    /// Maximum positive ramp rate for wind turbine reactive power/voltage reference (dxrefmax) (> WindPlantReactiveControlIEC.dxrefmin). It is a project-dependent parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dxrefmax: Option<f64>,
    /// Maximum negative ramp rate for wind turbine reactive power/voltage reference (dxrefmin) (< WindPlantReactiveControlIEC.dxrefmax). It is a project-dependent parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dxrefmin: Option<f64>,
    /// Plant Q controller integral gain (KIWPx). It is a project-dependent parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kiwpx: Option<f64>,
    /// Maximum reactive power/voltage reference from integration (KIWPxmax) (> WindPlantReactiveControlIEC.kiwpxmin). It is a project-dependent parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kiwpxmax: Option<f64>,
    /// Minimum reactive power/voltage reference from integration (KIWPxmin) (< WindPlantReactiveControlIEC.kiwpxmax). It is a project-dependent parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kiwpxmin: Option<f64>,
    /// Plant Q controller proportional gain (KPWPx). It is a project-dependent parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kpwpx: Option<f64>,
    /// Reactive power reference gain (KWPqref). It is a project-dependent parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kwpqref: Option<f64>,
    /// Plant voltage control droop (KWPqu). It is a project-dependent parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kwpqu: Option<f64>,
    /// Filter time constant for voltage-dependent reactive power (Tuqfilt) (>= 0). It is a project-dependent parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tuqfilt: Option<f64>,
    /// Filter time constant for active power measurement (TWPpfiltq) (>= 0). It is a project-dependent parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub twppfiltq: Option<f64>,
    /// Filter time constant for reactive power measurement (TWPqfiltq) (>= 0). It is a project-dependent parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub twpqfiltq: Option<f64>,
    /// Filter time constant for voltage measurement (TWPufiltq) (>= 0). It is a project-dependent parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub twpufiltq: Option<f64>,
    /// Lead time constant in reference value transfer function (Txft) (>= 0). It is a project-dependent parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub txft: Option<f64>,
    /// Lag time constant in reference value transfer function (Txfv) (>= 0). It is a project-dependent parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub txfv: Option<f64>,
    /// Voltage threshold for UVRT detection in Q control (uWPqdip). It is a project-dependent parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uwpqdip: Option<f64>,
    /// Reactive power/voltage controller mode (MWPqmode). It is a case-dependent parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wind_plant_qcontrol_modes_type: Option<super::base::UriRef>,
    /// Maximum xWTref (qWTref or delta uWTref) request from the plant controller (xrefmax) (> WindPlantReactiveControlIEC.xrefmin). It is a case-dependent parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub xrefmax: Option<f64>,
    /// Minimum xWTref (qWTref or delta uWTref) request from the plant controller (xrefmin) (< WindPlantReactiveControlIEC.xrefmax). It is a project-dependent parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub xrefmin: Option<f64>,
}
impl crate::base::CimElement for WindPlantReactiveControlIEC {
    fn mrid(&self) -> &str { &self.base.id }
    fn type_name(&self) -> &'static str { "WindPlantReactiveControlIEC" }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn to_json_value(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or(serde_json::Value::Null)
    }
    fn to_block(&self) -> crate::base::RdfBlock {
        let mut block = self.base.to_block();
        block.type_name = "WindPlantReactiveControlIEC".to_string();
        if let Some(v) = self.dxrefmax {
            block.fields.insert("WindPlantReactiveControlIEC.dxrefmax".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.dxrefmin {
            block.fields.insert("WindPlantReactiveControlIEC.dxrefmin".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.kiwpx {
            block.fields.insert("WindPlantReactiveControlIEC.kiwpx".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.kiwpxmax {
            block.fields.insert("WindPlantReactiveControlIEC.kiwpxmax".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.kiwpxmin {
            block.fields.insert("WindPlantReactiveControlIEC.kiwpxmin".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.kpwpx {
            block.fields.insert("WindPlantReactiveControlIEC.kpwpx".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.kwpqref {
            block.fields.insert("WindPlantReactiveControlIEC.kwpqref".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.kwpqu {
            block.fields.insert("WindPlantReactiveControlIEC.kwpqu".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.tuqfilt {
            block.fields.insert("WindPlantReactiveControlIEC.tuqfilt".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.twppfiltq {
            block.fields.insert("WindPlantReactiveControlIEC.twppfiltq".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.twpqfiltq {
            block.fields.insert("WindPlantReactiveControlIEC.twpqfiltq".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.twpufiltq {
            block.fields.insert("WindPlantReactiveControlIEC.twpufiltq".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.txft {
            block.fields.insert("WindPlantReactiveControlIEC.txft".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.txfv {
            block.fields.insert("WindPlantReactiveControlIEC.txfv".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.uwpqdip {
            block.fields.insert("WindPlantReactiveControlIEC.uwpqdip".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(ref v) = self.wind_plant_qcontrol_modes_type {
            block.fields.insert("WindPlantReactiveControlIEC.windPlantQcontrolModesType".into(), crate::base::FieldValue::Resource(v.uri.clone()));
        }
        if let Some(v) = self.xrefmax {
            block.fields.insert("WindPlantReactiveControlIEC.xrefmax".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.xrefmin {
            block.fields.insert("WindPlantReactiveControlIEC.xrefmin".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        block
    }
}

impl WindPlantReactiveControlIEC {
    pub fn from_block(b: &crate::base::RdfBlock) -> Self {
        let mut obj = Self::default();
        obj.base.id.clone_from(&b.mrid);
        for (key, val) in &b.fields {
            match key.as_str() {
                "WindPlantReactiveControlIEC.dxrefmax" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.dxrefmax = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.dxrefmax = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "WindPlantReactiveControlIEC.dxrefmin" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.dxrefmin = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.dxrefmin = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "WindPlantReactiveControlIEC.kiwpx" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.kiwpx = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.kiwpx = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "WindPlantReactiveControlIEC.kiwpxmax" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.kiwpxmax = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.kiwpxmax = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "WindPlantReactiveControlIEC.kiwpxmin" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.kiwpxmin = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.kiwpxmin = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "WindPlantReactiveControlIEC.kpwpx" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.kpwpx = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.kpwpx = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "WindPlantReactiveControlIEC.kwpqref" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.kwpqref = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.kwpqref = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "WindPlantReactiveControlIEC.kwpqu" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.kwpqu = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.kwpqu = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "WindPlantReactiveControlIEC.tuqfilt" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.tuqfilt = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.tuqfilt = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "WindPlantReactiveControlIEC.twppfiltq" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.twppfiltq = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.twppfiltq = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "WindPlantReactiveControlIEC.twpqfiltq" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.twpqfiltq = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.twpqfiltq = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "WindPlantReactiveControlIEC.twpufiltq" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.twpufiltq = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.twpufiltq = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "WindPlantReactiveControlIEC.txft" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.txft = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.txft = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "WindPlantReactiveControlIEC.txfv" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.txfv = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.txfv = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "WindPlantReactiveControlIEC.uwpqdip" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.uwpqdip = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.uwpqdip = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "WindPlantReactiveControlIEC.windPlantQcontrolModesType" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.wind_plant_qcontrol_modes_type = Some(crate::base::UriRef { uri: sv.clone() });
                    }
                }
                "WindPlantReactiveControlIEC.xrefmax" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.xrefmax = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.xrefmax = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "WindPlantReactiveControlIEC.xrefmin" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.xrefmin = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.xrefmin = Some(v); } }
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
