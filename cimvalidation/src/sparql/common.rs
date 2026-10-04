use cimstructs::base::FastMap as HashMap;
use cimdecoder::CimDataset;
use crate::Violation;

pub fn validate(dataset: &CimDataset) -> Vec<Violation> {
    let mut v = Vec::new();
    v.extend(check_per_entry_block_checks(dataset));
    v.extend(check_model_date_time_utc(dataset));
    v.extend(check_id_uuid(dataset));
    v.extend(check_id_deprecated(dataset));
    v.extend(check_modeling_authority_set_not_empty(dataset));
    v.extend(check_file_header_exists(dataset));
    v
}

/// Fuses check_mrid_uniqueness and check_float_special_values — these both
/// looped every dataset entry and called `.to_block()` on it (a per-type
/// generated field-map conversion, not free), so the redundant passes over the
/// whole dataset became one shared pass with one `.to_block()` call per entry.
/// (The IdentifiedObject string-length rules that used to share this pass are
/// plain `sh:maxLength`/`sh:length` now and run from the CGMES shape table.)
fn check_per_entry_block_checks(dataset: &CimDataset) -> Vec<Violation> {
    // mRID → every (object, class) carrying it. Collected first and decided
    // after the pass: deciding during it reported whichever duplicate the map
    // iteration reached second, which with a randomly seeded hasher changed
    // from run to run.
    let mut by_mrid: HashMap<String, Vec<(&String, String, &cimdecoder::CimEntry)>> = HashMap::default();
    let mut v = Vec::new();
    for (id, entry) in &dataset.entries {
        // Fast path. These checks read the struct's view of the element
        // (`to_block`), and building it for every element was most of this
        // pass's time. The decoder's block holds every value that view does,
        // as written, so if nothing in it could fail a check, nothing in the
        // struct's view can either. Only elements with a candidate pay for
        // `to_block` and the exact checks below.
        if !entry.block.type_name.is_empty() && !may_fail_entry_checks(&entry.block.fields) {
            if let Some(cimstructs::base::FieldValue::Text(m_rid)) = entry.block.fields.get("IdentifiedObject.mRID") {
                if !m_rid.is_empty() {
                    // Unconfirmed: re-read through `to_block` if it turns out
                    // to be a duplicate, below.
                    by_mrid.entry(m_rid.clone()).or_default().push((id, entry.element.type_name().to_string(), entry));
                }
            }
            continue;
        }

        let block = entry.element.to_block();
        let class = &block.type_name;

        // --- mRID uniqueness (all600:All-GENC1), collected ---
        if let Some(cimstructs::base::FieldValue::Text(m_rid)) = block.fields.get("IdentifiedObject.mRID") {
            if !m_rid.is_empty() {
                by_mrid.entry(m_rid.clone()).or_default().push((id, class.clone(), entry));
            }
        }

        for (key, val) in &block.fields {
            let s = match val {
                cimstructs::base::FieldValue::Text(s) => s,
                _ => continue,
            };

            // --- float special values (all600:Float-specialValues) ---
            if let Ok(f) = s.trim().parse::<f64>() {
                if f.is_nan() || f.is_infinite() {
                    v.push(Violation {
                        object_id: id.clone(),
                        rule_id:   "all600:Float-specialValues".into(),
                        name:      "C:301:ALL:Float:specialValues".into(),
                        class:     class.clone(),
                        property:  key.clone(),
                        message:   "INF or NaN used in an attribute defined as float.".into(),
                        severity:  "sh:Violation".into(),
                        description: String::new(),
                    });
                }
            }
        }
    }

    // --- mRID uniqueness (all600:All-GENC1), decided ---
    // The smallest object id counts as the original and every other one is
    // reported, so the result does not depend on iteration order.
    for (m_rid, owners) in by_mrid.iter_mut().filter(|(_, o)| o.len() > 1) {
        // Duplicates are rare, so confirm each owner against the struct's
        // view, which is what decides here; the fast path read the raw block.
        owners.retain(|(_, _, entry)| {
            matches!(entry.element.to_block().fields.get("IdentifiedObject.mRID"),
                Some(cimstructs::base::FieldValue::Text(m)) if m == m_rid)
        });
        if owners.len() < 2 {
            continue;
        }
        owners.sort_unstable_by(|a, b| a.0.cmp(b.0));
        for (id, class, _) in owners.drain(1..) {
            v.push(Violation {
                object_id: id.clone(),
                rule_id:   "all600:All-GENC1".into(),
                name:      "C:600:ALL:NA:GENC1".into(),
                class,
                property:  "IdentifiedObject.mRID".into(),
                message:   "Not a unique identifier.".into(),
                severity:  "sh:Violation".into(),
                description: String::new(),
            });
        }
    }
    v
}

/// Could any value in this raw field map fail one of the per-entry checks?
///
/// A superset test: every text value the struct's view holds is in the raw map
/// as written (the last of a repeated one included), and a typed float is the
/// raw text parsed the same way. A float is only non-finite for `nan`/`inf`
/// spellings or an overflowing exponent, so the byte scan skips the parse for
/// almost every value.
fn may_fail_entry_checks(fields: &cimstructs::base::FieldMap) -> bool {
    let suspicious = |key: &str, s: &str| {
        let non_finite = s.bytes().any(|b| matches!(b, b'n' | b'N' | b'i' | b'I' | b'e' | b'E'))
            && s.trim().parse::<f64>().is_ok_and(|f| !f.is_finite());
        let _ = key;
        non_finite
    };
    fields.iter().any(|(key, val)| match val {
        cimstructs::base::FieldValue::Text(s) => suspicious(key, s),
        // A repeated value: the struct keeps one of them, so any could matter.
        cimstructs::base::FieldValue::TextList(vs) => vs.iter().any(|s| suspicious(key, s)),
        _ => false,
    })
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
                name:      "C:600:ALL:NA:GENC4".into(),
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
                name:      "C:600:ALL:NA:GENC5".into(),
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
                        name:      "C:600:ALL:Model.created:HGEN4".into(),
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
                        name:      "C:600:ALL:Model.scenarioTime:HGEN4".into(),
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
                    name:      "C:600:ALL:Model.modelingAuthoritySet:marp10-12".into(),
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

fn check_file_header_exists(dataset: &CimDataset) -> Vec<Violation> {
    let has_fm = dataset.by_type.get("FullModel").map_or(0, |v| v.len()) > 0;
    let has_dm = dataset.by_type.get("DifferenceModel").map_or(0, |v| v.len()) > 0;
    if has_fm || has_dm { return Vec::new(); }
    vec![Violation {
        object_id: "global".into(),
        rule_id:   "all600:All-HGEN2".into(),
        name:      "C:600:ALL:NA:HGEN2".into(),
        class:     "FullModel".into(),
        property:  "rdf:type".into(),
        message:   "File header is missing.".into(),
        severity:  "sh:Violation".into(),
        description: String::new(),
    }]
}
