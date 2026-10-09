use cimmodel::base::FastMap as HashMap;
use cimmodel::CimDataset;
use crate::Violation;

pub fn validate(dataset: &CimDataset) -> Vec<Violation> {
    let mut v = Vec::new();
    v.extend(check_per_entry_block_checks(dataset));
    v.extend(check_model_date_time_utc(dataset));
    v.extend(check_modeling_authority_set_not_empty(dataset));
    v.extend(check_file_header_exists(dataset));
    v
}

/// Fuses the checks that visit every element — mRID uniqueness, float special
/// values, ID syntax (GENC4) and ID length (GENC5) — into one walk: on a large
/// model the walk itself, touching each element once, is a large share of the
/// cost. (The IdentifiedObject string-length
/// rules that used to share this pass are plain `sh:maxLength`/`sh:length`
/// now and run from the CGMES shape table.)
fn check_per_entry_block_checks(dataset: &CimDataset) -> Vec<Violation> {
    use std::borrow::Cow;
    use std::hash::BuildHasher;

    // Each element is checked on its own except for mRID uniqueness, so the
    // walk is split into runs of elements on their own threads. Each run also
    // sorts its elements' mRIDs into buckets by hash; a duplicate always lands
    // in one bucket, so the buckets are then decided independently, again on
    // threads.
    //
    // mRIDs are keyed on the text as stored, borrowed, with the class as the
    // static name: cloning both for every element was most of this pass's time.
    type Owner<'a> = (&'a String, &'static str, &'a cimmodel::Element);
    type Candidate<'a> = (Cow<'a, str>, Owner<'a>);

    let threads = crate::par::threads_for(dataset.entries.len());
    let bucket_of = |m_rid: &str| {
        (cimmodel::base::FastBuildHasher::default().hash_one(m_rid) % threads as u64) as usize
    };
    let all: Vec<(&String, &cimmodel::Element)> = dataset.entries.iter().collect();
    let parts = crate::par::par_map(&crate::par::runs(&all, threads, |_| 1), |run| {
        let mut v = Vec::new();
        let mut buckets: Vec<Vec<Candidate<'_>>> = (0..threads).map(|_| Vec::new()).collect();
        for &(id, entry) in run {
            v.extend(id_uuid_violation(id, entry));
            v.extend(id_deprecated_violation(id, entry));

            // --- mRID uniqueness (all600:All-GENC1), collected ---
            if let Some(m_rid) = mrid(entry) {
                buckets[bucket_of(m_rid)].push((Cow::Borrowed(m_rid), (id, entry.type_name(), entry)));
            }

            // --- float special values (all600:Float-specialValues) ---
            // The byte scan rules out nearly every element before a field's
            // attribute is looked up.
            if may_be_non_finite(entry.fields()) {
                v.extend(float_special_values(id, entry));
            }
        }
        (v, buckets)
    });

    let mut v = Vec::new();
    let mut by_bucket: Vec<Vec<Vec<Candidate<'_>>>> = (0..threads).map(|_| Vec::new()).collect();
    for (part_v, buckets) in parts {
        v.extend(part_v);
        for (b, list) in buckets.into_iter().enumerate() {
            by_bucket[b].push(list);
        }
    }

    // --- mRID uniqueness (all600:All-GENC1), decided per bucket ---
    // The first owner of each mRID is kept inline and a list allocated only
    // when a second turns up: duplicates are rare, and a list per mRID was an
    // allocation per element.
    let mut genc1 = crate::par::par_concat(&crate::par::runs(&by_bucket, threads, |_| 1), |buckets| {
        let mut out = Vec::new();
        for lists in buckets {
            let mut first: HashMap<&Cow<'_, str>, Owner<'_>> = HashMap::default();
            let mut by_mrid: HashMap<&Cow<'_, str>, Vec<Owner<'_>>> = HashMap::default();
            for (m_rid, owner) in lists.iter().flatten() {
                match first.entry(m_rid) {
                    std::collections::hash_map::Entry::Occupied(o) => {
                        by_mrid.entry(*o.key()).or_insert_with(|| vec![*o.get()]).push(*owner);
                    }
                    std::collections::hash_map::Entry::Vacant(slot) => {
                        slot.insert(*owner);
                    }
                }
            }
            // The smallest object id counts as the original and every other
            // one is reported, so the result does not depend on iteration
            // order.
            for owners in by_mrid.values_mut() {
                owners.sort_unstable_by(|a, b| a.0.cmp(b.0));
                for (id, class, _) in owners.drain(1..) {
                    out.push(Violation {
                        object_id: id.clone(),
                        rule_id:   "all600:All-GENC1".into(),
                        name:      "C:600:ALL:NA:GENC1".into(),
                        class:     class.to_string(),
                        property:  "IdentifiedObject.mRID".into(),
                        message:   "Not a unique identifier.".into(),
                        severity:  "sh:Violation".into(),
                        description: String::new(),
                    });
                }
            }
        }
        out
    });
    // Which bucket an mRID lands in depends on the thread count; sorting keeps
    // the output the same on every machine.
    genc1.sort_unstable_by(|a, b| a.object_id.cmp(&b.object_id));
    v.extend(genc1);
    v
}

/// An element's `IdentifiedObject.mRID`, the last value if it is repeated;
/// `None` when absent or empty.
fn mrid(entry: &cimmodel::Element) -> Option<&str> {
    let m = match entry.fields().get("IdentifiedObject.mRID")? {
        cimmodel::base::FieldValue::Text(s) => s.as_str(),
        cimmodel::base::FieldValue::TextList(v) => v.last()?.as_str(),
        _ => return None,
    };
    (!m.is_empty()).then_some(m)
}

/// `C:301:ALL:Float:specialValues`: an attribute the schema types as Float
/// (`xsd:double`) holding INF or NaN. A field the class does not declare has
/// no type to read, and a text attribute may spell `NaN` freely.
fn float_special_values(id: &str, entry: &cimmodel::Element) -> Vec<Violation> {
    let reg = cimmodel::registry::type_registry();
    let non_finite = |s: &String| s.trim().parse::<f64>().is_ok_and(|f| !f.is_finite());
    let mut keys: Vec<&&'static str> = entry
        .fields()
        .iter()
        .filter(|(_, value)| match value {
            cimmodel::base::FieldValue::Text(s) => non_finite(s),
            cimmodel::base::FieldValue::TextList(vs) => vs.iter().any(non_finite),
            _ => false,
        })
        .filter(|(key, _)| reg.attr(entry.class(), key).is_some_and(|a| a.xsd == "double"))
        .map(|(key, _)| key)
        .collect();
    keys.sort_unstable();
    keys.into_iter()
        .map(|key| Violation {
            object_id: id.to_string(),
            rule_id:   "all600:Float-specialValues".into(),
            name:      "C:301:ALL:Float:specialValues".into(),
            class:     entry.type_name().to_string(),
            property:  key.to_string(),
            message:   "INF or NaN used in an attribute defined as float.".into(),
            severity:  "sh:Violation".into(),
            description: String::new(),
        })
        .collect()
}

/// Could any text value in this field map be a non-finite number?
///
/// A float is only non-finite for `nan`/`inf` spellings or an overflowing
/// exponent, so the byte scan skips the parse for almost every value.
fn may_be_non_finite(fields: &cimmodel::base::FieldMap) -> bool {
    // Only `nan` / `inf` / `infinity` spellings, or a number that overflows
    // — which needs an exponent or an absurd digit count — parse as
    // non-finite. Anything else, a name above all, is ruled out from its first
    // byte without parsing.
    let non_finite = |s: &str| {
        let t = s.trim();
        let body = t.strip_prefix(['+', '-']).unwrap_or(t);
        let candidate = match body.as_bytes().first() {
            Some(b'i' | b'I' | b'n' | b'N') => true,
            Some(b'0'..=b'9' | b'.') => body.len() > 300 || body.bytes().any(|b| b == b'e' || b == b'E'),
            _ => false,
        };
        candidate && t.parse::<f64>().is_ok_and(|f| !f.is_finite())
    };
    fields.values().any(|val| match val {
        cimmodel::base::FieldValue::Text(s) => non_finite(s),
        cimmodel::base::FieldValue::TextList(vs) => vs.iter().any(|s| non_finite(s)),
        _ => false,
    })
}

/// `[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}`, any case —
/// what the GENC4 regex matched, without the regex.
fn is_uuid(s: &str) -> bool {
    s.len() == 36
        && s.bytes().enumerate().all(|(i, b)| match i {
            8 | 13 | 18 | 23 => b == b'-',
            _ => b.is_ascii_hexdigit(),
        })
}

/// GENC4 for one element: its ID must be a UUID.
fn id_uuid_violation(id: &str, entry: &cimmodel::Element) -> Option<Violation> {
    // Extract clean ID
    let clean_id: &str = if id.contains("#_") {
        id.split("#_").nth(1).unwrap_or("")
    } else if id.starts_with("urn:uuid:") {
        id
    } else if id.contains('#') {
        let part = id.split('#').nth(1).unwrap_or("");
        part.strip_prefix('_').unwrap_or(part)
    } else if let Some(rest) = id.strip_prefix('_') {
        rest
    } else {
        id
    };
    // Checked slicing: an ID is input text, and byte 9 need not be a char
    // boundary.
    let urn_uuid = id.get(..9).is_some_and(|p| p.eq_ignore_ascii_case("urn:uuid:"))
        && id.get(9..).is_some_and(is_uuid);
    if !is_uuid(clean_id) && !urn_uuid {
        return Some(Violation {
            object_id: id.to_string(),
            rule_id:   "all600:All-GENC4".into(),
            name:      "C:600:ALL:NA:GENC4".into(),
            class:     entry.type_name().to_string(),
            property:  "rdf:ID".into(),
            message:   "Invalid syntax of ID (rdf:ID or rdf:about). UUID expected.".into(),
            severity:  "sh:Info".into(),
            description: String::new(),
        });
    }
    None
}

/// GENC5 for one element: a non-URN ID starts with `_` and is at most 60
/// characters.
fn id_deprecated_violation(id: &str, entry: &cimmodel::Element) -> Option<Violation> {
    if id.starts_with("urn:uuid:") { return None; }
    let second_part: &str = if id.contains("#_") {
        id.split("#_").nth(1).unwrap_or("")
    } else {
        id.strip_prefix('_').unwrap_or_default()
    };
    if second_part.len() > 59 || second_part.is_empty() {
        return Some(Violation {
            object_id: id.to_string(),
            rule_id:   "all600:All-GENC5".into(),
            name:      "C:600:ALL:NA:GENC5".into(),
            class:     entry.type_name().to_string(),
            property:  "rdf:ID".into(),
            message:   "The ID string is more than 60 characters or the string does not begin with underscore.".into(),
            severity:  "sh:Violation".into(),
            description: String::new(),
        });
    }
    None
}

fn check_model_date_time_utc(dataset: &CimDataset) -> Vec<Violation> {
    let mut v = Vec::new();
    for type_name in &["FullModel", "DifferenceModel"] {
        for mrid in dataset.by_type.get(*type_name).into_iter().flatten() {
            let entry = &dataset.entries[mrid];
            if let Some(m) = super::Fields::of_class(entry, type_name) {
                let created = m.text("Model.created");
                if !created.is_empty() && !created.ends_with('Z') {
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
                let scenario_time = m.text("Model.scenarioTime");
                if !scenario_time.is_empty() && !scenario_time.ends_with('Z') {
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
            let mas = match super::Fields::of_class(entry, type_name) {
                Some(m) => m.text("Model.modelingAuthoritySet").trim().to_string(),
                None => continue,
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
        property:  "^rdf:type".into(),
        message:   "File header is missing.".into(),
        severity:  "sh:Violation".into(),
        description: String::new(),
    }]
}
