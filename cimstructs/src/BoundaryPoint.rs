/// Designates a connection point at which one or more model authority sets shall connect to. The location of the connection point as well as other properties are agreed between organisations responsible for the interconnection, hence all attributes of the class represent this agreement. It is primarily used in a boundary model authority set which can contain one or many BoundaryPoint-s among other Equipment-s and their connections.
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct BoundaryPoint {
    #[serde(flatten)]
    pub base: super::PowerSystemResource,
    /// The connectivity node that is designated as a boundary point.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connectivity_node: Option<super::base::MridRef>,
    /// The ISO code of the region which the 'From' side of the Boundary point belongs to or it is connected to. The ISO code is a two-character country code as defined by ISO 3166 (http://www.iso.org/iso/country_codes). The length of the string is 2 characters maximum.
    pub from_end_iso_code: String,
    /// A human readable name with length of the string 64 characters maximum. It covers the following two cases: -if the Boundary point is placed on a tie-line, it is the name (IdentifiedObject.name) of the substation at which the 'From' side of the tie-line is connected to. -if the Boundary point is placed in a substation, it is the name (IdentifiedObject.name) of the element (e.g. PowerTransformer, ACLineSegment, Switch, etc.) at which the 'From' side of the Boundary point is connected to.
    pub from_end_name: String,
    /// Identifies the name of the transmission system operator, distribution system operator or other entity at which the 'From' side of the interconnection is connected to. The length of the string is 64 characters maximum.
    pub from_end_name_tso: String,
    /// If true, this boundary point is a point of common coupling (PCC) of a direct current (DC) interconnection, otherwise the interconnection is AC (default).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_direct_current: Option<bool>,
    /// If true, this boundary point is on the interconnection that is excluded from control area interchange calculation and consequently has no related tie flows. Otherwise, the interconnection is included in control area interchange and a TieFlow is required at all sides of the boundary point (default).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_excluded_from_area_interchange: Option<bool>,
    /// The ISO code of the region which the 'To' side of the Boundary point belongs to or is connected to. The ISO code is a two-character country code as defined by ISO 3166 (http://www.iso.org/iso/country_codes). The length of the string is 2 characters maximum.
    pub to_end_iso_code: String,
    /// A human readable name with length of the string 64 characters maximum. It covers the following two cases: -if the Boundary point is placed on a tie-line, it is the name (IdentifiedObject.name) of the substation at which the 'To' side of the tie-line is connected to. -if the Boundary point is placed in a substation, it is the name (IdentifiedObject.name) of the element (e.g. PowerTransformer, ACLineSegment, Switch, etc.) at which the 'To' side of the Boundary point is connected to.
    pub to_end_name: String,
    /// Identifies the name of the transmission system operator, distribution system operator or other entity at which the 'To' side of the interconnection is connected to. The length of the string is 64 characters maximum.
    pub to_end_name_tso: String,
}
impl crate::base::CimElement for BoundaryPoint {
    fn mrid(&self) -> &str { &self.base.base.id }
    fn type_name(&self) -> &'static str { "BoundaryPoint" }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn to_json_value(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or(serde_json::Value::Null)
    }
    fn to_block(&self) -> crate::base::RdfBlock {
        let mut block = self.base.to_block();
        block.type_name = "BoundaryPoint".to_string();
        if let Some(ref v) = self.connectivity_node {
            block.fields.insert("BoundaryPoint.ConnectivityNode".into(), crate::base::FieldValue::Resource(v.mrid.clone()));
        }
        if !self.from_end_iso_code.is_empty() {
            block.fields.insert("BoundaryPoint.fromEndIsoCode".into(), crate::base::FieldValue::Text(self.from_end_iso_code.clone()));
        }
        if !self.from_end_name.is_empty() {
            block.fields.insert("BoundaryPoint.fromEndName".into(), crate::base::FieldValue::Text(self.from_end_name.clone()));
        }
        if !self.from_end_name_tso.is_empty() {
            block.fields.insert("BoundaryPoint.fromEndNameTso".into(), crate::base::FieldValue::Text(self.from_end_name_tso.clone()));
        }
        if let Some(v) = self.is_direct_current {
            block.fields.insert("BoundaryPoint.isDirectCurrent".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if let Some(v) = self.is_excluded_from_area_interchange {
            block.fields.insert("BoundaryPoint.isExcludedFromAreaInterchange".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        if !self.to_end_iso_code.is_empty() {
            block.fields.insert("BoundaryPoint.toEndIsoCode".into(), crate::base::FieldValue::Text(self.to_end_iso_code.clone()));
        }
        if !self.to_end_name.is_empty() {
            block.fields.insert("BoundaryPoint.toEndName".into(), crate::base::FieldValue::Text(self.to_end_name.clone()));
        }
        if !self.to_end_name_tso.is_empty() {
            block.fields.insert("BoundaryPoint.toEndNameTso".into(), crate::base::FieldValue::Text(self.to_end_name_tso.clone()));
        }
        block
    }
}

impl BoundaryPoint {
    pub fn from_block(b: &crate::base::RdfBlock) -> Self {
        let mut obj = Self::default();
        obj.base.base.id.clone_from(&b.mrid);
        for (key, val) in &b.fields {
            match key.as_str() {
                "BoundaryPoint.ConnectivityNode" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.connectivity_node = Some(crate::base::MridRef { mrid: sv.clone() });
                    }
                }
                "BoundaryPoint.fromEndIsoCode" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.from_end_iso_code = sv.clone(); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.from_end_iso_code = sv.clone(); }
                        }
                        _ => {}
                    }
                }
                "BoundaryPoint.fromEndName" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.from_end_name = sv.clone(); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.from_end_name = sv.clone(); }
                        }
                        _ => {}
                    }
                }
                "BoundaryPoint.fromEndNameTso" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.from_end_name_tso = sv.clone(); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.from_end_name_tso = sv.clone(); }
                        }
                        _ => {}
                    }
                }
                "BoundaryPoint.isDirectCurrent" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.is_direct_current = Some(sv.trim() == "true"); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.is_direct_current = Some(sv.trim() == "true"); }
                        }
                        _ => {}
                    }
                }
                "BoundaryPoint.isExcludedFromAreaInterchange" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.is_excluded_from_area_interchange = Some(sv.trim() == "true"); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.is_excluded_from_area_interchange = Some(sv.trim() == "true"); }
                        }
                        _ => {}
                    }
                }
                "BoundaryPoint.toEndIsoCode" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.to_end_iso_code = sv.clone(); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.to_end_iso_code = sv.clone(); }
                        }
                        _ => {}
                    }
                }
                "BoundaryPoint.toEndName" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.to_end_name = sv.clone(); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.to_end_name = sv.clone(); }
                        }
                        _ => {}
                    }
                }
                "BoundaryPoint.toEndNameTso" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { obj.to_end_name_tso = sv.clone(); }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { obj.to_end_name_tso = sv.clone(); }
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
