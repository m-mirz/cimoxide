use std::collections::HashMap;
use cimdecoder::CimDataset;
use crate::Violation;

pub fn validate(dataset: &CimDataset) -> Vec<Violation> {
    let mut v = Vec::new();
    v.extend(check_identified_object_string_lengths(dataset));
    v.extend(check_float_special_values(dataset));
    v.extend(check_model_date_time_utc(dataset));
    v.extend(check_mrid_uniqueness(dataset));
    v.extend(check_id_uuid(dataset));
    v.extend(check_id_deprecated(dataset));
    v.extend(check_modeling_authority_set_not_empty(dataset));
    v.extend(check_file_header_exists(dataset));
    v
}

fn check_mrid_uniqueness(dataset: &CimDataset) -> Vec<Violation> {
    let mut seen: HashMap<String, String> = HashMap::new();
    let mut v = Vec::new();
    for (id, entry) in &dataset.entries {
        let block = entry.element.to_block();
        let m_rid = match block.fields.get("IdentifiedObject.mRID") {
            Some(cimstructs::base::FieldValue::Text(s)) if !s.is_empty() => s.clone(),
            _ => continue,
        };
        if let Some(first_id) = seen.get(&m_rid) {
            if first_id != id {
                v.push(Violation {
                    object_id: id.clone(),
                    rule_id:   "all600:All-GENC1".into(),
                    name:      String::new(),
                    class:     block.type_name.clone(),
                    property:  "IdentifiedObject.mRID".into(),
                    message:   "Not a unique identifier.".into(),
                    severity:  "sh:Violation".into(),
                    description: String::new(),
                });
            }
        } else {
            seen.insert(m_rid, id.clone());
        }
    }
    v
}

fn check_id_uuid(dataset: &CimDataset) -> Vec<Violation> {
    use std::sync::OnceLock;
    use regex::Regex;
    static UUID_RE: OnceLock<Regex> = OnceLock::new();
    static URN_UUID_RE: OnceLock<Regex> = OnceLock::new();
    let uuid_re = UUID_RE.get_or_init(|| {
        Regex::new(r"(?i)^[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}$").unwrap()
    });
    let urn_uuid_re = URN_UUID_RE.get_or_init(|| {
        Regex::new(r"(?i)^urn:uuid:[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}$").unwrap()
    });
    let mut v = Vec::new();
    for (id, entry) in &dataset.entries {
        // Extract clean ID
        let clean_id = if id.contains("#_") {
            id.split("#_").nth(1).unwrap_or("").to_string()
        } else if id.starts_with("urn:uuid:") {
            id.clone()
        } else if id.contains('#') {
            let part = id.split('#').nth(1).unwrap_or("");
            if part.starts_with('_') { part[1..].to_string() } else { part.to_string() }
        } else if id.starts_with('_') {
            id[1..].to_string()
        } else {
            id.clone()
        };
        if !uuid_re.is_match(&clean_id) && !urn_uuid_re.is_match(id) {
            v.push(Violation {
                object_id: id.clone(),
                rule_id:   "all600:All-GENC4".into(),
                name:      String::new(),
                class:     entry.element.type_name().to_string(),
                property:  "rdf:ID".into(),
                message:   "Invalid syntax of ID (rdf:ID or rdf:about). UUID expected.".into(),
                severity:  "sh:Info".into(),
                description: String::new(),
            });
        }
    }
    v
}

fn check_id_deprecated(dataset: &CimDataset) -> Vec<Violation> {
    let mut v = Vec::new();
    for (id, entry) in &dataset.entries {
        if id.starts_with("urn:uuid:") { continue; }
        let second_part = if id.contains("#_") {
            id.split("#_").nth(1).unwrap_or("").to_string()
        } else if id.starts_with('_') {
            id[1..].to_string()
        } else {
            String::new()
        };
        if second_part.len() > 59 || second_part.is_empty() {
            v.push(Violation {
                object_id: id.clone(),
                rule_id:   "all600:All-GENC5".into(),
                name:      String::new(),
                class:     entry.element.type_name().to_string(),
                property:  "rdf:ID".into(),
                message:   "The ID string is more than 60 characters or the string does not begin with underscore.".into(),
                severity:  "sh:Violation".into(),
                description: String::new(),
            });
        }
    }
    v
}

fn check_model_date_time_utc(dataset: &CimDataset) -> Vec<Violation> {
    let mut v = Vec::new();
    for type_name in &["FullModel", "DifferenceModel"] {
        for mrid in dataset.by_type.get(*type_name).into_iter().flatten() {
            let entry = &dataset.entries[mrid];
            let model_base = if let Some(fm) = entry.element.as_any().downcast_ref::<cimstructs::FullModel>() {
                Some(&fm.base)
            } else if let Some(dm) = entry.element.as_any().downcast_ref::<cimstructs::DifferenceModel>() {
                Some(&dm.base)
            } else {
                None
            };
            if let Some(m) = model_base {
                if !m.created.is_empty() && !m.created.ends_with('Z') {
                    v.push(Violation {
                        object_id: mrid.clone(),
                        rule_id:   "all600:Model.created-HGEN4".into(),
                        name:      String::new(),
                        class:     type_name.to_string(),
                        property:  "Model.created".into(),
                        message:   "File header Model.created is not a valid UTC date time (missing 'Z').".into(),
                        severity:  "sh:Violation".into(),
                        description: String::new(),
                    });
                }
                if !m.scenario_time.is_empty() && !m.scenario_time.ends_with('Z') {
                    v.push(Violation {
                        object_id: mrid.clone(),
                        rule_id:   "all600:Model.scenarioTime-HGEN4".into(),
                        name:      String::new(),
                        class:     type_name.to_string(),
                        property:  "Model.scenarioTime".into(),
                        message:   "File header Model.scenarioTime is not a valid UTC date time (missing 'Z').".into(),
                        severity:  "sh:Violation".into(),
                        description: String::new(),
                    });
                }
            }
        }
    }
    v
}

fn check_float_special_values(dataset: &CimDataset) -> Vec<Violation> {
    let mut v = Vec::new();
    for (id, entry) in &dataset.entries {
        let block = entry.element.to_block();
        for (key, val) in &block.fields {
            if let cimstructs::base::FieldValue::Text(s) = val {
                if let Ok(f) = s.trim().parse::<f64>() {
                    if f.is_nan() || f.is_infinite() {
                        v.push(Violation {
                            object_id: id.clone(),
                            rule_id:   "all600:Float-specialValues".into(),
                            name:      String::new(),
                            class:     block.type_name.clone(),
                            property:  key.clone(),
                            message:   "INF or NaN used in an attribute defined as float.".into(),
                            severity:  "sh:Violation".into(),
                            description: String::new(),
                        });
                    }
                }
            }
        }
    }
    v
}

fn check_modeling_authority_set_not_empty(dataset: &CimDataset) -> Vec<Violation> {
    let mut v = Vec::new();
    for type_name in &["FullModel", "DifferenceModel"] {
        for mrid in dataset.by_type.get(*type_name).into_iter().flatten() {
            let entry = &dataset.entries[mrid];
            let mas = if let Some(fm) = entry.element.as_any().downcast_ref::<cimstructs::FullModel>() {
                fm.base.modeling_authority_set.trim().to_string()
            } else if let Some(dm) = entry.element.as_any().downcast_ref::<cimstructs::DifferenceModel>() {
                dm.base.modeling_authority_set.trim().to_string()
            } else {
                continue
            };
            if mas.is_empty() {
                v.push(Violation {
                    object_id: mrid.clone(),
                    rule_id:   "all600:Model.modelingAuthoritySet-marp10-12".into(),
                    name:      String::new(),
                    class:     type_name.to_string(),
                    property:  "Model.modelingAuthoritySet".into(),
                    message:   "The modelingAuthoritySet property is defined as empty.".into(),
                    severity:  "sh:Violation".into(),
                    description: String::new(),
                });
            }
        }
    }
    v
}

fn check_identified_object_string_lengths(dataset: &CimDataset) -> Vec<Violation> {
    let mut v = Vec::new();
    for (id, entry) in &dataset.entries {
        let block = entry.element.to_block();
        let class = &block.type_name;
        for (key, val) in &block.fields {
            if let cimstructs::base::FieldValue::Text(s) = val {
                match key.as_str() {
                    "IdentifiedObject.shortName" if s.len() > 12 => {
                        v.push(Violation {
                            object_id: id.clone(),
                            rule_id:   "iosl:IdentifiedObject.shortName-stringLength".into(),
                            name:      String::new(),
                            class:     class.clone(),
                            property:  "IdentifiedObject.shortName".into(),
                            message:   "String length is greater than 12 characters.".into(),
                            severity:  "sh:Violation".into(),
                            description: String::new(),
                        });
                    }
                    "IdentifiedObject.energyIdentCodeEic" if !s.is_empty() && s.len() != 16 => {
                        v.push(Violation {
                            object_id: id.clone(),
                            rule_id:   "iosl:IdentifiedObject.energyIdentCodeEic-stringLength".into(),
                            name:      String::new(),
                            class:     class.clone(),
                            property:  "IdentifiedObject.energyIdentCodeEic".into(),
                            message:   "String length is not 16 characters.".into(),
                            severity:  "sh:Violation".into(),
                            description: String::new(),
                        });
                    }
                    "IdentifiedObject.name" if s.len() > 128 => {
                        v.push(Violation {
                            object_id: id.clone(),
                            rule_id:   "iosl:IdentifiedObject.name-stringLength".into(),
                            name:      String::new(),
                            class:     class.clone(),
                            property:  "IdentifiedObject.name".into(),
                            message:   "String length is greater than 128 characters.".into(),
                            severity:  "sh:Violation".into(),
                            description: String::new(),
                        });
                    }
                    "IdentifiedObject.description" if s.len() > 256 => {
                        v.push(Violation {
                            object_id: id.clone(),
                            rule_id:   "iosl:IdentifiedObject.description-stringLength".into(),
                            name:      String::new(),
                            class:     class.clone(),
                            property:  "IdentifiedObject.description".into(),
                            message:   "String length is greater than 256 characters.".into(),
                            severity:  "sh:Violation".into(),
                            description: String::new(),
                        });
                    }
                    _ => {}
                }
            }
        }
    }
    v
}

fn check_file_header_exists(dataset: &CimDataset) -> Vec<Violation> {
    let has_fm = dataset.by_type.get("FullModel").map_or(0, |v| v.len()) > 0;
    let has_dm = dataset.by_type.get("DifferenceModel").map_or(0, |v| v.len()) > 0;
    if has_fm || has_dm { return Vec::new(); }
    vec![Violation {
        object_id: "global".into(),
        rule_id:   "all600:All-HGEN2".into(),
        name:      String::new(),
        class:     "FullModel".into(),
        property:  "rdf:type".into(),
        message:   "File header is missing.".into(),
        severity:  "sh:Violation".into(),
        description: String::new(),
    }]
}
