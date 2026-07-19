/// The place, scene, or point of something where someone or something has been, is, and/or will be at a given moment in time. It can be defined with one or more position points (coordinates) in a given coordinate system.
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct Location {
    #[serde(flatten)]
    pub base: super::IdentifiedObject,
    /// Coordinate system used to describe position points of this location.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coordinate_system: Option<super::base::MridRef>,
    /// All power system resources at this location.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub power_system_resources: Option<super::base::MridRef>,
    /// Main address of the location.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub main_address: Option<super::base::MridRef>,
}
impl crate::base::CimElement for Location {
    fn mrid(&self) -> &str { &self.base.id }
    fn type_name(&self) -> &'static str { "Location" }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn to_json_value(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or(serde_json::Value::Null)
    }
    fn to_block(&self) -> crate::base::RdfBlock {
        let mut block = self.base.to_block();
        block.type_name = "Location".to_string();
        if let Some(ref v) = self.coordinate_system {
            block.fields.insert("Location.CoordinateSystem".into(), crate::base::FieldValue::Resource(v.mrid.clone()));
        }
        if let Some(ref v) = self.power_system_resources {
            block.fields.insert("Location.PowerSystemResources".into(), crate::base::FieldValue::Resource(v.mrid.clone()));
        }
        if let Some(ref v) = self.main_address {
            block.fields.insert("Location.mainAddress".into(), crate::base::FieldValue::Resource(v.mrid.clone()));
        }
        block
    }
}

impl Location {
    pub fn from_block(b: &crate::base::RdfBlock) -> Self {
        let mut obj = Self::default();
        obj.base.id.clone_from(&b.mrid);
        for (key, val) in &b.fields {
            match key.as_str() {
                "Location.CoordinateSystem" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.coordinate_system = Some(crate::base::MridRef { mrid: sv.clone() });
                    }
                }
                "Location.PowerSystemResources" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.power_system_resources = Some(crate::base::MridRef { mrid: sv.clone() });
                    }
                }
                "Location.mainAddress" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.main_address = Some(crate::base::MridRef { mrid: sv.clone() });
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
