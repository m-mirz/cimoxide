use cimstructs::base::FastMap as HashMap;
use cimdecoder::CimDataset;
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
    // mRID → every (object, class) carrying it. Collected first and decided
    // after the pass: deciding during it reported whichever duplicate the map
    // iteration reached second, which with a randomly seeded hasher changed
    // from run to run.
    //
    // Keyed on the mRID text as stored (borrowed; owned only for the rare
    // element that takes the `to_block` path), with the class as the static
    // name: cloning both for every element was most of this pass's time.
    //
    // The first owner of each mRID is kept inline and a list allocated only
    // when a second turns up: duplicates are rare, and a list per mRID was an
    // allocation per element.
    type Owner<'a> = (&'a String, &'static str, &'a cimdecoder::CimEntry);
    let mut first: HashMap<std::borrow::Cow<'_, str>, Owner<'_>> = HashMap::default();
    let mut by_mrid: HashMap<std::borrow::Cow<'_, str>, Vec<Owner<'_>>> = HashMap::default();
    fn collect<'a>(
        first: &mut HashMap<std::borrow::Cow<'a, str>, Owner<'a>>,
        by_mrid: &mut HashMap<std::borrow::Cow<'a, str>, Vec<Owner<'a>>>,
        m_rid: std::borrow::Cow<'a, str>,
        owner: Owner<'a>,
    ) {
        match first.entry(m_rid) {
            std::collections::hash_map::Entry::Occupied(o) => {
                by_mrid.entry(o.key().clone()).or_insert_with(|| vec![*o.get()]).push(owner);
            }
            std::collections::hash_map::Entry::Vacant(slot) => {
                slot.insert(owner);
            }
        }
    }
    let mut v = Vec::new();
    for (id, entry) in &dataset.entries {
        v.extend(id_uuid_violation(id, entry));
        v.extend(id_deprecated_violation(id, entry));

        // Fast path. These checks read the struct's view of the element
        // (`to_block`), and building it for every element was most of this
        // pass's time. The decoder's block holds every value that view does,
        // as written, so if nothing in it could fail a check, nothing in the
        // struct's view can either. Only elements with a candidate pay for
        // `to_block` and the exact checks below.
        if !entry.block.type_name.is_empty() && !may_fail_entry_checks(&entry.block.fields) {
            if let Some(cimstructs::base::FieldValue::Text(m_rid)) = entry.block.fields.get("IdentifiedObject.mRID")
                && !m_rid.is_empty() {
                    // Unconfirmed: re-read through `to_block` if it turns out
                    // to be a duplicate, below.
                    collect(&mut first, &mut by_mrid, std::borrow::Cow::Borrowed(m_rid.as_str()), (id, entry.element.type_name(), entry));
                }
            continue;
        }

        let block = entry.element.to_block();
        let class = &block.type_name;

        // --- mRID uniqueness (all600:All-GENC1), collected ---
        if let Some(cimstructs::base::FieldValue::Text(m_rid)) = block.fields.get("IdentifiedObject.mRID")
            && !m_rid.is_empty() {
                collect(&mut first, &mut by_mrid, std::borrow::Cow::Owned(m_rid.clone()), (id, entry.element.type_name(), entry));
            }

        for (key, val) in &block.fields {
            let s = match val {
                cimstructs::base::FieldValue::Text(s) => s,
                _ => continue,
            };

            // --- float special values (all600:Float-specialValues) ---
            if let Ok(f) = s.trim().parse::<f64>()
                && (f.is_nan() || f.is_infinite()) {
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

    drop(first);
    // --- mRID uniqueness (all600:All-GENC1), decided ---
    // The smallest object id counts as the original and every other one is
    // reported, so the result does not depend on iteration order.
    for (m_rid, owners) in by_mrid.iter_mut().filter(|(_, o)| o.len() > 1) {
        // Duplicates are rare, so confirm each owner against the struct's
        // view, which is what decides here; the fast path read the raw block.
        owners.retain(|(_, _, entry)| {
            matches!(entry.element.to_block().fields.get("IdentifiedObject.mRID"),
                Some(cimstructs::base::FieldValue::Text(m)) if m.as_str() == m_rid.as_ref())
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
                class:     class.to_string(),
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
        cimstructs::base::FieldValue::Text(s) => non_finite(s),
        // A repeated value: the struct keeps one of them, so any could matter.
        cimstructs::base::FieldValue::TextList(vs) => vs.iter().any(|s| non_finite(s)),
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
fn id_uuid_violation(id: &str, entry: &cimdecoder::CimEntry) -> Option<Violation> {
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
            class:     entry.element.type_name().to_string(),
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
fn id_deprecated_violation(id: &str, entry: &cimdecoder::CimEntry) -> Option<Violation> {
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
            class:     entry.element.type_name().to_string(),
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
            let model_base = if let Some(fm) = entry.element.as_any().downcast_ref::<cimstructs::FullModel>() {
                Some(&fm.base)
            } else { entry.element.as_any().downcast_ref::<cimstructs::DifferenceModel>().map(|dm| &dm.base) };
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
