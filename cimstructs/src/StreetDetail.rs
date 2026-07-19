/// Street details, in the context of address.
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct StreetDetail {
    pub id: String,
    /// First line of a free form address or some additional address information (for example a mail stop).
    pub address_general: String,
    /// (if applicable) Second line of a free form address.
    pub address_general2: String,
    /// (if applicable) Third line of a free form address.
    pub address_general3: String,
    /// (if applicable) In certain cases the physical location of the place of interest does not have a direct point of entry from the street, but may be located inside a larger structure such as a building, complex, office block, apartment, etc.
    pub building_name: String,
    /// (if applicable) Utilities often make use of external reference systems, such as those of the town-planner's department or surveyor general's mapping system, that allocate global reference codes to streets.
    pub code: String,
    /// The identification by name or number, expressed as text, of the floor in the building as part of this address.
    pub floor_identification: String,
    /// Name of the street.
    pub name: String,
    /// Designator of the specific location on the street.
    pub number: String,
    /// Prefix to the street name. For example: North, South, East, West.
    pub prefix: String,
    /// Suffix to the street name. For example: North, South, East, West.
    pub suffix: String,
    /// Number of the apartment or suite.
    pub suite_number: String,
    /// Type of street. Examples include: street, circle, boulevard, avenue, road, drive, etc.
    pub type_: String,
    /// True if this street is within the legal geographical boundaries of the specified town (default).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub within_town_limits: Option<bool>,
}
impl crate::base::CimElement for StreetDetail {
    fn mrid(&self) -> &str { &self.id }
    fn type_name(&self) -> &'static str { "StreetDetail" }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn to_json_value(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or(serde_json::Value::Null)
    }
    fn to_block(&self) -> crate::base::RdfBlock {
        let mut block = crate::base::RdfBlock {
            type_name: "StreetDetail".to_string(),
            mrid: self.id.clone(),
            fields: std::collections::HashMap::new(),
            duplicate_fields: std::collections::HashSet::new(),
        };
        if !self.address_general.is_empty() {
            block.fields.insert("StreetDetail.addressGeneral".into(), crate::base::FieldValue::Text(self.address_general.clone()));
        }
        if !self.address_general2.is_empty() {
            block.fields.insert("StreetDetail.addressGeneral2".into(), crate::base::FieldValue::Text(self.address_general2.clone()));
        }
        if !self.address_general3.is_empty() {
            block.fields.insert("StreetDetail.addressGeneral3".into(), crate::base::FieldValue::Text(self.address_general3.clone()));
        }
        if !self.building_name.is_empty() {
            block.fields.insert("StreetDetail.buildingName".into(), crate::base::FieldValue::Text(self.building_name.clone()));
        }
        if !self.code.is_empty() {
            block.fields.insert("StreetDetail.code".into(), crate::base::FieldValue::Text(self.code.clone()));
        }
        if !self.floor_identification.is_empty() {
            block.fields.insert("StreetDetail.floorIdentification".into(), crate::base::FieldValue::Text(self.floor_identification.clone()));
        }
        if !self.name.is_empty() {
            block.fields.insert("StreetDetail.name".into(), crate::base::FieldValue::Text(self.name.clone()));
        }
        if !self.number.is_empty() {
            block.fields.insert("StreetDetail.number".into(), crate::base::FieldValue::Text(self.number.clone()));
        }
        if !self.prefix.is_empty() {
            block.fields.insert("StreetDetail.prefix".into(), crate::base::FieldValue::Text(self.prefix.clone()));
        }
        if !self.suffix.is_empty() {
            block.fields.insert("StreetDetail.suffix".into(), crate::base::FieldValue::Text(self.suffix.clone()));
        }
        if !self.suite_number.is_empty() {
            block.fields.insert("StreetDetail.suiteNumber".into(), crate::base::FieldValue::Text(self.suite_number.clone()));
        }
        if !self.type_.is_empty() {
            block.fields.insert("StreetDetail.type".into(), crate::base::FieldValue::Text(self.type_.clone()));
        }
        if let Some(v) = self.within_town_limits {
            block.fields.insert("StreetDetail.withinTownLimits".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        block
    }
}

impl StreetDetail {
    pub fn from_block(b: &crate::base::RdfBlock) -> Self {
        let mut obj = Self::default();
        obj.id.clone_from(&b.mrid);
        for (key, val) in &b.fields {
            match key.as_str() {
                "StreetDetail.addressGeneral" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.address_general = sv.clone(); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.address_general = sv.clone(); }
                        }
                        _ => {}
                    }
                }
                "StreetDetail.addressGeneral2" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.address_general2 = sv.clone(); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.address_general2 = sv.clone(); }
                        }
                        _ => {}
                    }
                }
                "StreetDetail.addressGeneral3" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.address_general3 = sv.clone(); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.address_general3 = sv.clone(); }
                        }
                        _ => {}
                    }
                }
                "StreetDetail.buildingName" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.building_name = sv.clone(); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.building_name = sv.clone(); }
                        }
                        _ => {}
                    }
                }
                "StreetDetail.code" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.code = sv.clone(); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.code = sv.clone(); }
                        }
                        _ => {}
                    }
                }
                "StreetDetail.floorIdentification" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.floor_identification = sv.clone(); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.floor_identification = sv.clone(); }
                        }
                        _ => {}
                    }
                }
                "StreetDetail.name" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.name = sv.clone(); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.name = sv.clone(); }
                        }
                        _ => {}
                    }
                }
                "StreetDetail.number" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.number = sv.clone(); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.number = sv.clone(); }
                        }
                        _ => {}
                    }
                }
                "StreetDetail.prefix" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.prefix = sv.clone(); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.prefix = sv.clone(); }
                        }
                        _ => {}
                    }
                }
                "StreetDetail.suffix" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.suffix = sv.clone(); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.suffix = sv.clone(); }
                        }
                        _ => {}
                    }
                }
                "StreetDetail.suiteNumber" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.suite_number = sv.clone(); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.suite_number = sv.clone(); }
                        }
                        _ => {}
                    }
                }
                "StreetDetail.type" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.type_ = sv.clone(); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.type_ = sv.clone(); }
                        }
                        _ => {}
                    }
                }
                "StreetDetail.withinTownLimits" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.within_town_limits = Some(sv.trim() == "true"); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.within_town_limits = Some(sv.trim() == "true"); }
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
