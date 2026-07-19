/// An electromechanical device that operates with shaft rotating synchronously with the network. It is a single machine operating either as a generator or synchronous condenser or pump.
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct SynchronousMachine {
    #[serde(flatten)]
    pub base: super::RotatingMachine,
    /// The default reactive capability curve for use by a synchronous machine.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initial_reactive_capability_curve: Option<super::base::MridRef>,
    /// Indicates whether or not the generator is earthed. Used for short circuit data exchange according to IEC 60909.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub earthing: Option<bool>,
    /// Generator star point earthing resistance (Re). Used for short circuit data exchange according to IEC 60909.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub earthing_star_point_r: Option<f64>,
    /// Generator star point earthing reactance (Xe). Used for short circuit data exchange according to IEC 60909.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub earthing_star_point_x: Option<f64>,
    /// Steady-state short-circuit current (in A for the profile) of generator with compound excitation during 3-phase short circuit. - Ikk=0: Generator with no compound excitation. - Ikk<>0: Generator with compound excitation. Ikk is used to calculate the minimum steady-state short-circuit current for generators with compound excitation. (4.6.1.2 in IEC 60909-0:2001). Used only for single fed short circuit on a generator. (4.3.4.2. in IEC 60909-0:2001).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ikk: Option<f64>,
    /// Maximum reactive power limit. This is the maximum (nameplate) limit for the unit.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_q: Option<f64>,
    /// Minimum reactive power limit for the unit.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_q: Option<f64>,
    /// Factor to calculate the breaking current (Section 4.5.2.1 in IEC 60909-0). Used only for single fed short circuit on a generator (Section 4.3.4.2. in IEC 60909-0).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mu: Option<f64>,
    /// Current mode of operation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operating_mode: Option<super::base::UriRef>,
    /// Part of the coordinated reactive control that comes from this machine. The attribute is used as a participation factor not necessarily summing up to 100% for the participating devices in the control.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub q_percent: Option<f64>,
    /// Equivalent resistance (RG) of generator. RG is considered for the calculation of all currents, except for the calculation of the peak current ip. Used for short circuit data exchange according to IEC 60909.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r: Option<f64>,
    /// Zero sequence resistance of the synchronous machine.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r0: Option<f64>,
    /// Negative sequence resistance.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r2: Option<f64>,
    /// Priority of unit for use as powerflow voltage phase angle reference bus selection. 0 = don t care (default) 1 = highest priority. 2 is less than 1 and so on.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference_priority: Option<i64>,
    /// Direct-axis subtransient reactance saturated, also known as Xd'sat.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sat_direct_subtrans_x: Option<f64>,
    /// Direct-axes saturated synchronous reactance (xdsat); reciprocal of short-circuit ration. Used for short circuit data exchange, only for single fed short circuit on a generator. (4.3.4.2. in IEC 60909-0:2001).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sat_direct_sync_x: Option<f64>,
    /// Saturated Direct-axis transient reactance. The attribute is primarily used for short circuit calculations according to ANSI.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sat_direct_trans_x: Option<f64>,
    /// Type of rotor, used by short circuit applications, only for single fed short circuit according to IEC 60909.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub short_circuit_rotor_type: Option<super::base::UriRef>,
    /// Modes that this synchronous machine can operate in.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<super::base::UriRef>,
    /// Range of generator voltage regulation (PG in IEC 60909-0) used for calculation of the impedance correction factor KG defined in IEC 60909-0. This attribute is used to describe the operating voltage of the generating unit.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voltage_regulation_range: Option<f64>,
    /// Zero sequence reactance of the synchronous machine.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x0: Option<f64>,
    /// Negative sequence reactance.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x2: Option<f64>,
}
impl crate::base::CimElement for SynchronousMachine {
    fn mrid(&self) -> &str { &self.base.base.base.base.base.base.base.id }
    fn type_name(&self) -> &'static str { "SynchronousMachine" }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn to_json_value(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or(serde_json::Value::Null)
    }
    fn to_block(&self) -> crate::base::RdfBlock {
        let mut block = self.base.to_block();
        block.type_name = "SynchronousMachine".to_string();
        if let Some(ref v) = self.initial_reactive_capability_curve {
            block.fields.insert("SynchronousMachine.InitialReactiveCapabilityCurve".into(), crate::base::FieldValue::Resource(v.mrid.clone()));
        }
        if let Some(v) = self.earthing {
            block.fields.insert("SynchronousMachine.earthing".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.earthing_star_point_r {
            block.fields.insert("SynchronousMachine.earthingStarPointR".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.earthing_star_point_x {
            block.fields.insert("SynchronousMachine.earthingStarPointX".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.ikk {
            block.fields.insert("SynchronousMachine.ikk".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.max_q {
            block.fields.insert("SynchronousMachine.maxQ".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.min_q {
            block.fields.insert("SynchronousMachine.minQ".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.mu {
            block.fields.insert("SynchronousMachine.mu".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(ref v) = self.operating_mode {
            block.fields.insert("SynchronousMachine.operatingMode".into(), crate::base::FieldValue::Resource(v.uri.clone()));
        }
        if let Some(v) = self.q_percent {
            block.fields.insert("SynchronousMachine.qPercent".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.r {
            block.fields.insert("SynchronousMachine.r".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.r0 {
            block.fields.insert("SynchronousMachine.r0".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.r2 {
            block.fields.insert("SynchronousMachine.r2".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.reference_priority {
            block.fields.insert("SynchronousMachine.referencePriority".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.sat_direct_subtrans_x {
            block.fields.insert("SynchronousMachine.satDirectSubtransX".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.sat_direct_sync_x {
            block.fields.insert("SynchronousMachine.satDirectSyncX".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.sat_direct_trans_x {
            block.fields.insert("SynchronousMachine.satDirectTransX".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(ref v) = self.short_circuit_rotor_type {
            block.fields.insert("SynchronousMachine.shortCircuitRotorType".into(), crate::base::FieldValue::Resource(v.uri.clone()));
        }
        if let Some(ref v) = self.type_ {
            block.fields.insert("SynchronousMachine.type".into(), crate::base::FieldValue::Resource(v.uri.clone()));
        }
        if let Some(v) = self.voltage_regulation_range {
            block.fields.insert("SynchronousMachine.voltageRegulationRange".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.x0 {
            block.fields.insert("SynchronousMachine.x0".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.x2 {
            block.fields.insert("SynchronousMachine.x2".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        block
    }
}

impl SynchronousMachine {
    pub fn from_block(b: &crate::base::RdfBlock) -> Self {
        let mut obj = Self::default();
        obj.base.base.base.base.base.base.base.id.clone_from(&b.mrid);
        for (key, val) in &b.fields {
            match key.as_str() {
                "SynchronousMachine.InitialReactiveCapabilityCurve" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.initial_reactive_capability_curve = Some(crate::base::MridRef { mrid: sv.clone() });
                    }
                }
                "SynchronousMachine.earthing" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.earthing = Some(sv.trim() == "true"); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.earthing = Some(sv.trim() == "true"); }
                        }
                        _ => {}
                    }
                }
                "SynchronousMachine.earthingStarPointR" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.earthing_star_point_r = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.earthing_star_point_r = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "SynchronousMachine.earthingStarPointX" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.earthing_star_point_x = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.earthing_star_point_x = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "SynchronousMachine.ikk" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.ikk = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.ikk = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "SynchronousMachine.maxQ" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.max_q = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.max_q = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "SynchronousMachine.minQ" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.min_q = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.min_q = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "SynchronousMachine.mu" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.mu = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.mu = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "SynchronousMachine.operatingMode" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.operating_mode = Some(crate::base::UriRef { uri: sv.clone() });
                    }
                }
                "SynchronousMachine.qPercent" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.q_percent = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.q_percent = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "SynchronousMachine.r" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.r = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.r = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "SynchronousMachine.r0" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.r0 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.r0 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "SynchronousMachine.r2" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.r2 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.r2 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "SynchronousMachine.referencePriority" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.reference_priority = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.reference_priority = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "SynchronousMachine.satDirectSubtransX" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.sat_direct_subtrans_x = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.sat_direct_subtrans_x = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "SynchronousMachine.satDirectSyncX" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.sat_direct_sync_x = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.sat_direct_sync_x = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "SynchronousMachine.satDirectTransX" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.sat_direct_trans_x = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.sat_direct_trans_x = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "SynchronousMachine.shortCircuitRotorType" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.short_circuit_rotor_type = Some(crate::base::UriRef { uri: sv.clone() });
                    }
                }
                "SynchronousMachine.type" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.type_ = Some(crate::base::UriRef { uri: sv.clone() });
                    }
                }
                "SynchronousMachine.voltageRegulationRange" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.voltage_regulation_range = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.voltage_regulation_range = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "SynchronousMachine.x0" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.x0 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.x0 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "SynchronousMachine.x2" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.x2 = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.x2 = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "RotatingMachine.GeneratingUnit" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.base.generating_unit = Some(crate::base::MridRef { mrid: sv.clone() });
                    }
                }
                "RotatingMachine.p" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.base.p = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.base.p = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "RotatingMachine.q" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.base.q = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.base.q = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "RotatingMachine.ratedPowerFactor" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.base.rated_power_factor = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.base.rated_power_factor = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "RotatingMachine.ratedS" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.base.rated_s = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.base.rated_s = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "RotatingMachine.ratedU" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.base.rated_u = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.base.rated_u = Some(v); } }
                        }
                        _ => {}
                    }
                }
                "RegulatingCondEq.RegulatingControl" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.base.base.regulating_control = Some(crate::base::MridRef { mrid: sv.clone() });
                    }
                }
                "RegulatingCondEq.controlEnabled" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.base.base.control_enabled = Some(sv.trim() == "true"); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.base.base.control_enabled = Some(sv.trim() == "true"); }
                        }
                        _ => {}
                    }
                }
                "ConductingEquipment.BaseVoltage" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.base.base.base.base.base_voltage = Some(crate::base::MridRef { mrid: sv.clone() });
                    }
                }
                "Equipment.EquipmentContainer" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.base.base.base.base.base.equipment_container = Some(crate::base::MridRef { mrid: sv.clone() });
                    }
                }
                "Equipment.aggregate" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.base.base.base.base.base.aggregate = Some(sv.trim() == "true"); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.base.base.base.base.base.aggregate = Some(sv.trim() == "true"); }
                        }
                        _ => {}
                    }
                }
                "Equipment.inService" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.base.base.base.base.base.in_service = Some(sv.trim() == "true"); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.base.base.base.base.base.in_service = Some(sv.trim() == "true"); }
                        }
                        _ => {}
                    }
                }
                "Equipment.normallyInService" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.base.base.base.base.base.normally_in_service = Some(sv.trim() == "true"); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.base.base.base.base.base.normally_in_service = Some(sv.trim() == "true"); }
                        }
                        _ => {}
                    }
                }
                "IdentifiedObject.description" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.base.base.base.base.base.base.base.description = sv.clone(); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.base.base.base.base.base.base.base.description = sv.clone(); }
                        }
                        _ => {}
                    }
                }
                "IdentifiedObject.energyIdentCodeEic" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.base.base.base.base.base.base.base.energy_ident_code_eic = sv.clone(); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.base.base.base.base.base.base.base.energy_ident_code_eic = sv.clone(); }
                        }
                        _ => {}
                    }
                }
                "IdentifiedObject.mRID" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.base.base.base.base.base.base.base.m_rid = sv.clone(); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.base.base.base.base.base.base.base.m_rid = sv.clone(); }
                        }
                        _ => {}
                    }
                }
                "IdentifiedObject.name" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.base.base.base.base.base.base.base.name = sv.clone(); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.base.base.base.base.base.base.base.name = sv.clone(); }
                        }
                        _ => {}
                    }
                }
                "IdentifiedObject.shortName" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.base.base.base.base.base.base.base.short_name = sv.clone(); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.base.base.base.base.base.base.base.short_name = sv.clone(); }
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
