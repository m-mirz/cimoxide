/// A PowerTransformerEnd is associated with each Terminal of a PowerTransformer. The impedance values r, r0, x, and x0 of a PowerTransformerEnd represents a star equivalent as follows. 1) for a two Terminal PowerTransformer the high voltage (TransformerEnd.endNumber=1) PowerTransformerEnd has non zero values on r, r0, x, and x0 while the low voltage (TransformerEnd.endNumber=2) PowerTransformerEnd has zero values for r, r0, x, and x0. Parameters are always provided, even if the PowerTransformerEnds have the same rated voltage. In this case, the parameters are provided at the PowerTransformerEnd which has TransformerEnd.endNumber equal to 1. 2) for a three Terminal PowerTransformer the three PowerTransformerEnds represent a star equivalent with each leg in the star represented by r, r0, x, and x0 values. 3) For a three Terminal transformer each PowerTransformerEnd shall have g, g0, b and b0 values corresponding to the no load losses distributed on the three PowerTransformerEnds. The total no load loss shunt impedances may also be placed at one of the PowerTransformerEnds, preferably the end numbered 1, having the shunt values on end 1. This is the preferred way. 4) for a PowerTransformer with more than three Terminals the PowerTransformerEnd impedance values cannot be used. Instead use the TransformerMeshImpedance or split the transformer into multiple PowerTransformers. Each PowerTransformerEnd must be contained by a PowerTransformer. Because a PowerTransformerEnd (or any other object) can not be contained by more than one parent, a PowerTransformerEnd can not have an association to an EquipmentContainer (Substation, VoltageLevel, etc).
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct PowerTransformerEnd {
    #[serde(flatten)]
    pub base: super::TransformerEnd,
    /// The power transformer of this power transformer end.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub power_transformer: Option<super::base::MridRef>,
    /// Magnetizing branch susceptance (B mag). The value can be positive or negative.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub b: Option<f64>,
    /// Zero sequence magnetizing branch susceptance.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub b0: Option<f64>,
    /// Kind of connection.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_kind: Option<super::base::UriRef>,
    /// Magnetizing branch conductance.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub g: Option<f64>,
    /// Zero sequence magnetizing branch conductance (star-model).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub g0: Option<f64>,
    /// Terminal voltage phase angle displacement where 360 degrees are represented with clock hours. The valid values are 0 to 11. For example, for the secondary side end of a transformer with vector group code of 'Dyn11', specify the connection kind as wye with neutral and specify the phase angle of the clock as 11. The clock value of the transformer end number specified as 1, is assumed to be zero. Note the transformer end number is not assumed to be the same as the terminal sequence number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phase_angle_clock: Option<i64>,
    /// Resistance (star-model) of the transformer end. The attribute shall be equal to or greater than zero for non-equivalent transformers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r: Option<f64>,
    /// Zero sequence series resistance (star-model) of the transformer end.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r0: Option<f64>,
    /// Normal apparent power rating. The attribute shall be a positive value. For a two-winding transformer the values for the high and low voltage sides shall be identical.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rated_s: Option<f64>,
    /// Rated voltage: phase-phase for three-phase windings, and either phase-phase or phase-neutral for single-phase windings. A high voltage side, as given by TransformerEnd.endNumber, shall have a ratedU that is greater than or equal to ratedU for the lower voltage sides. The attribute shall be a positive value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rated_u: Option<f64>,
    /// Positive sequence series reactance (star-model) of the transformer end.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x: Option<f64>,
    /// Zero sequence series reactance of the transformer end.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x0: Option<f64>,
}
impl crate::base::CimElement for PowerTransformerEnd {
    fn mrid(&self) -> &str { &self.base.base.id }
    fn type_name(&self) -> &'static str { "PowerTransformerEnd" }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn to_json_value(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or(serde_json::Value::Null)
    }
    fn to_block(&self) -> crate::base::RdfBlock {
        let mut block = self.base.to_block();
        block.type_name = "PowerTransformerEnd".to_string();
        if let Some(ref v) = self.power_transformer {
            block.fields.insert("PowerTransformerEnd.PowerTransformer".into(), crate::base::FieldValue::Resource(v.mrid.clone()));
        }
        if let Some(v) = self.b {
            block.fields.insert("PowerTransformerEnd.b".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.b0 {
            block.fields.insert("PowerTransformerEnd.b0".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(ref v) = self.connection_kind {
            block.fields.insert("PowerTransformerEnd.connectionKind".into(), crate::base::FieldValue::Resource(v.uri.clone()));
        }
        if let Some(v) = self.g {
            block.fields.insert("PowerTransformerEnd.g".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.g0 {
            block.fields.insert("PowerTransformerEnd.g0".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.phase_angle_clock {
            block.fields.insert("PowerTransformerEnd.phaseAngleClock".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.r {
            block.fields.insert("PowerTransformerEnd.r".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.r0 {
            block.fields.insert("PowerTransformerEnd.r0".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.rated_s {
            block.fields.insert("PowerTransformerEnd.ratedS".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.rated_u {
            block.fields.insert("PowerTransformerEnd.ratedU".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.x {
            block.fields.insert("PowerTransformerEnd.x".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.x0 {
            block.fields.insert("PowerTransformerEnd.x0".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        block
    }
}

impl PowerTransformerEnd {
    pub fn from_block(b: &crate::base::RdfBlock) -> Self {
        let mut obj = Self::default();
        obj.base.base.id.clone_from(&b.mrid);
        for (key, val) in &b.fields {
            match key.as_str() {
                "PowerTransformerEnd.PowerTransformer" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.power_transformer = Some(crate::base::MridRef { mrid: sv.clone() });
                    }
                }
                "PowerTransformerEnd.b" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.b = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.b = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "PowerTransformerEnd.b0" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.b0 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.b0 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "PowerTransformerEnd.connectionKind" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.connection_kind = Some(crate::base::UriRef { uri: sv.clone() });
                    }
                }
                "PowerTransformerEnd.g" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.g = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.g = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "PowerTransformerEnd.g0" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.g0 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.g0 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "PowerTransformerEnd.phaseAngleClock" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.phase_angle_clock = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.phase_angle_clock = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "PowerTransformerEnd.r" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.r = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.r = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "PowerTransformerEnd.r0" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.r0 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.r0 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "PowerTransformerEnd.ratedS" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.rated_s = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.rated_s = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "PowerTransformerEnd.ratedU" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.rated_u = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.rated_u = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "PowerTransformerEnd.x" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.x = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.x = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "PowerTransformerEnd.x0" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.x0 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.x0 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "TransformerEnd.BaseVoltage" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.base.base_voltage = Some(crate::base::MridRef { mrid: sv.clone() });
                    }
                }
                "TransformerEnd.Terminal" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.base.terminal = Some(crate::base::MridRef { mrid: sv.clone() });
                    }
                }
                "TransformerEnd.endNumber" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.base.end_number = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.base.end_number = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "TransformerEnd.grounded" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.base.grounded = Some(sv.trim() == "true"); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.base.grounded = Some(sv.trim() == "true"); }
                        }
                        _ => {}
                    }
                }
                "TransformerEnd.rground" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.base.rground = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.base.rground = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "TransformerEnd.xground" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.base.xground = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.base.xground = Some(v); } }
                        }
                        _ => {}
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
