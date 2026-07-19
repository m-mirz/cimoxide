/// Used to apply user standard names to TopologicalNodes. Associated with one or more terminals that are normally connected with the bus name. The associated terminals are normally connected by non-retained switches. For a ring bus station configuration, all BusbarSection terminals in the ring are typically associated. For a breaker and a half scheme, both BusbarSections would normally be associated. For a ring bus, all BusbarSections would normally be associated. For a 'straight' busbar configuration, normally only the main terminal at the BusbarSection would be associated.
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct BusNameMarker {
    #[serde(flatten)]
    pub base: super::IdentifiedObject,
    /// The reporting group to which this bus name marker belongs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reporting_group: Option<super::base::MridRef>,
    /// Priority of bus name marker for use as topology bus name. Use 0 for do not care. Use 1 for highest priority. Use 2 as priority is less than 1 and so on.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<i64>,
}
impl crate::base::CimElement for BusNameMarker {
    fn mrid(&self) -> &str { &self.base.id }
    fn type_name(&self) -> &'static str { "BusNameMarker" }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn to_json_value(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or(serde_json::Value::Null)
    }
    fn to_block(&self) -> crate::base::RdfBlock {
        let mut block = self.base.to_block();
        block.type_name = "BusNameMarker".to_string();
        if let Some(ref v) = self.reporting_group {
            block.fields.insert("BusNameMarker.ReportingGroup".into(), crate::base::FieldValue::Resource(v.mrid.clone()));
        }
        if let Some(v) = self.priority {
            block.fields.insert("BusNameMarker.priority".into(), crate::base::FieldValue::Text(v.to_string()));
        }
        block
    }
}

impl BusNameMarker {
    pub fn from_block(b: &crate::base::RdfBlock) -> Self {
        let mut obj = Self::default();
        obj.base.id.clone_from(&b.mrid);
        for (key, val) in &b.fields {
            match key.as_str() {
                "BusNameMarker.ReportingGroup" => {
                    if let crate::base::FieldValue::Resource(sv) = val {
                        obj.reporting_group = Some(crate::base::MridRef { mrid: sv.clone() });
                    }
                }
                "BusNameMarker.priority" => {
                    match val {
                        crate::base::FieldValue::Text(sv) => { if let Ok(v) = sv.trim().parse() { obj.priority = Some(v); } }
                        crate::base::FieldValue::TextList(svs) => {
                            if let Some(sv) = svs.last() { if let Ok(v) = sv.trim().parse() { obj.priority = Some(v); } }
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
