//! Validates a property-bag family against a [`crate::shapes`] table.
//!
//! The counterpart to the generated `generated_*_shacl.rs` validators, which
//! read typed struct fields. Here the element is a
//! [`cimstructs::base::GenericElement`] and every value is a string, so the
//! checks the generated path treats as tautologies — `sh:datatype`,
//! `sh:nodeKind` — are real, and `sh:closed` becomes answerable at all.

use std::collections::HashMap;

use cimdecoder::CimDataset;
use cimstructs::base::{FieldValue, GenericElement};

use crate::helpers;
use crate::shapes::{AltBranch, Check, ClosedShape, Constraint, NodeKind, Path, PropShape, ShapeDef, Target};
use crate::{Config, Violation};

/// Maps a referenced mRID back to the elements pointing at it.
///
/// `sh:inversePath` asks "how many things point at me", which the forward
/// fields cannot answer. Built once per dataset in O(associations) rather than
/// rescanned per shape, and only when some active shape needs it.
#[derive(Default)]
struct ReverseIndex {
    /// `(target mrid, field key)` → how many elements point at it that way.
    counts: HashMap<(String, &'static str), u32>,
}

impl ReverseIndex {
    fn build(ds: &CimDataset, fields: &[&'static str]) -> Self {
        let mut counts: HashMap<(String, &'static str), u32> = HashMap::new();
        for entry in ds.entries.values() {
            let Some(el) = entry.element.as_any().downcast_ref::<GenericElement>() else {
                continue;
            };
            for field in fields {
                for target in el.get_refs(field) {
                    *counts.entry((target.clone(), *field)).or_insert(0) += 1;
                }
            }
        }
        Self { counts }
    }

    fn count(&self, mrid: &str, field: &'static str) -> u32 {
        // The key borrows nothing, so look up by the owned pair. Shapes using
        // inverse paths are a small minority, so this is not a hot path.
        self.counts
            .get(&(mrid.to_string(), field))
            .copied()
            .unwrap_or(0)
    }
}

/// The values a path yields for one element, as far as a constraint needs them.
enum Values<'a> {
    /// Literal text values.
    Text(Vec<&'a str>),
    /// `rdf:resource` references.
    Refs(&'a [String]),
    /// Only a count is available — an inverse or alternative path.
    Count(u32),
    Absent,
}

impl Values<'_> {
    fn count(&self) -> u32 {
        match self {
            Values::Text(v) => v.len() as u32,
            Values::Refs(v) => v.len() as u32,
            Values::Count(n) => *n,
            Values::Absent => 0,
        }
    }
}

fn field_values<'a>(el: &'a GenericElement, field: &str) -> Values<'a> {
    match el.get(field) {
        Some(FieldValue::Text(s)) => Values::Text(vec![s.as_str()]),
        Some(FieldValue::TextList(v)) => Values::Text(v.iter().map(String::as_str).collect()),
        Some(FieldValue::Resource(_)) | Some(FieldValue::ResourceList(_)) => {
            Values::Refs(el.get_refs(field))
        }
        None => Values::Absent,
    }
}

fn resolve<'a>(
    el: &'a GenericElement,
    mrid: &str,
    path: &Path,
    reverse: Option<&ReverseIndex>,
) -> Values<'a> {
    match path {
        Path::Forward(field) | Path::RefType(field) => field_values(el, field),
        Path::Inverse(field) => match reverse {
            Some(r) => Values::Count(r.count(mrid, field)),
            None => Values::Absent,
        },
        Path::Alternative(branches) => {
            // The union across branches. Only the count is well defined: the
            // branches can mix literals, references and inverse hops, and every
            // NCP alternative shape asks a cardinality question.
            let mut total = 0;
            for b in *branches {
                total += match b {
                    AltBranch::Forward(field) => field_values(el, field).count(),
                    AltBranch::Inverse(field) => {
                        reverse.map_or(0, |r| r.count(mrid, field))
                    }
                };
            }
            Values::Count(total)
        }
    }
}

/// Is `s` a valid lexical form of this xsd type?
///
/// The whole reason `sh:datatype` is worth checking here: a bag holds the text
/// as written, so a malformed integer survives decoding.
fn datatype_ok(s: &str, xsd: &str) -> bool {
    let t = s.trim();
    match xsd {
        "string" | "normalizedString" | "token" | "Name" | "NCName" | "language" => true,
        "boolean" => matches!(t, "true" | "false" | "1" | "0"),
        "integer" | "int" | "long" | "short" | "byte" | "nonPositiveInteger"
        | "negativeInteger" => t.parse::<i64>().is_ok(),
        "nonNegativeInteger" | "positiveInteger" | "unsignedInt" | "unsignedLong"
        | "unsignedShort" | "unsignedByte" => t.parse::<u64>().is_ok(),
        "decimal" | "float" | "double" => t.parse::<f64>().is_ok(),
        "dateTime" | "dateTimeStamp" => helpers::is_xsd_datetime(t),
        "date" => helpers::is_xsd_date(t),
        "gMonthDay" => helpers::is_xsd_gmonthday(t),
        "anyURI" => helpers::is_xsd_anyuri(t),
        // An xsd type this does not model is not evidence of a violation.
        _ => true,
    }
}

fn violation(mrid: &str, class: &str, property: &str, c: &Check) -> Violation {
    Violation {
        object_id: mrid.to_string(),
        rule_id: c.rule_id.to_string(),
        class: class.to_string(),
        property: property.to_string(),
        message: c.message.to_string(),
        severity: c.severity.to_string(),
        name: c.name.to_string(),
        description: c.description.to_string(),
    }
}

/// A path's field key, for the `property` column of a violation.
fn path_label(path: &Path) -> &'static str {
    match path {
        Path::Forward(f) | Path::RefType(f) | Path::Inverse(f) => f,
        Path::Alternative(branches) => match branches.first() {
            Some(AltBranch::Forward(f)) | Some(AltBranch::Inverse(f)) => f,
            None => "",
        },
    }
}

fn check_prop(
    ds: &CimDataset,
    el: &GenericElement,
    mrid: &str,
    prop: &PropShape,
    reverse: Option<&ReverseIndex>,
    out: &mut Vec<Violation>,
) {
    let values = resolve(el, mrid, &prop.path, reverse);
    let class = el.class_def().qualified;
    let label = path_label(&prop.path);

    for c in prop.checks {
        let failed = match &c.constraint {
            Constraint::MinCount(n) => values.count() < *n,
            Constraint::MaxCount(n) => values.count() > *n,

            Constraint::Datatype(xsd) => match &values {
                Values::Text(vs) => vs.iter().any(|v| !datatype_ok(v, xsd)),
                // A literal constraint on a reference is a node-kind problem,
                // which the shape's own sh:nodeKind check reports. Flagging it
                // twice would double-count one mistake.
                _ => false,
            },

            Constraint::NodeKind(kind) => match (&values, kind) {
                (Values::Absent, _) => false,
                (Values::Text(_), NodeKind::Literal) => false,
                (Values::Refs(_), NodeKind::Iri) => false,
                // An inverse or alternative path yields only a count, so there
                // is no node to inspect.
                (Values::Count(_), _) => false,
                (Values::Text(_), _) | (Values::Refs(_), _) => true,
            },

            Constraint::In(allowed) => match &values {
                Values::Text(vs) => vs.iter().any(|v| !allowed.contains(&v.trim())),
                Values::Refs(rs) => rs.iter().any(|r| !allowed.contains(&r.as_str())),
                _ => false,
            },

            Constraint::HasValue(want) => match &values {
                Values::Text(vs) => !vs.iter().any(|v| v.trim() == *want),
                Values::Refs(rs) => !rs.iter().any(|r| r == want),
                _ => false,
            },

            Constraint::MaxLength(n) => match &values {
                Values::Text(vs) => vs.iter().any(|v| v.chars().count() as u32 > *n),
                _ => false,
            },
            Constraint::MinLength(n) => match &values {
                Values::Text(vs) => vs.iter().any(|v| (v.chars().count() as u32) < *n),
                _ => false,
            },

            // Both read the class of a referenced element, and both are silent
            // when it is not in this dataset: phase 1 runs per file, so a
            // reference out of the file is normal and belongs to a
            // cross-profile rule, not here. Reporting it would make every
            // cross-file association a value-type violation.
            Constraint::Class(allowed) | Constraint::RefClass(allowed) => match &values {
                Values::Refs(rs) => rs.iter().any(|r| {
                    ds.entries
                        .get(r)
                        .is_some_and(|e| !allowed.contains(&e.element.type_name()))
                }),
                _ => false,
            },
        };

        if failed {
            out.push(violation(mrid, class, label, c));
        }
    }
}

/// `sh:closed` — report every field the profile does not allow on this class.
fn check_closed(el: &GenericElement, mrid: &str, closed: &ClosedShape, out: &mut Vec<Violation>) {
    let class = el.class_def().qualified;
    let mut extra: Vec<&str> = el
        .fields()
        .keys()
        .map(String::as_str)
        .filter(|k| !closed.allowed.contains(k))
        .collect();
    // Deterministic order: HashMap iteration is not, and a violation list that
    // reorders between runs is unusable in a diff.
    extra.sort_unstable();

    for field in extra {
        out.push(Violation {
            object_id: mrid.to_string(),
            rule_id: closed.rule_id.to_string(),
            class: class.to_string(),
            property: field.to_string(),
            message: closed.message.to_string(),
            severity: closed.severity.to_string(),
            name: closed.name.to_string(),
            description: closed.description.to_string(),
        });
    }
}

/// Which elements carry a given field, for `sh:targetSubjectsOf`.
///
/// `by_type` answers class targets directly, but nothing indexes elements by
/// the fields they have. Scanning the dataset per shape is what the obvious
/// implementation does, and it is O(elements x shapes): on a 20k-element
/// dataset that cost 10 ms *for a profile whose shapes matched nothing*, since
/// the scan happens before anything can be ruled out. One pass up front, over
/// only the fields some active shape asks about, removes that.
#[derive(Default)]
struct SubjectIndex {
    by_field: HashMap<&'static str, Vec<String>>,
}

impl SubjectIndex {
    fn build(ds: &CimDataset, fields: &[&'static str]) -> Self {
        let mut by_field: HashMap<&'static str, Vec<String>> =
            fields.iter().map(|f| (*f, Vec::new())).collect();
        for (mrid, entry) in &ds.entries {
            let Some(el) = entry.element.as_any().downcast_ref::<GenericElement>() else {
                continue;
            };
            for field in fields {
                if el.get(field).is_some() {
                    by_field.get_mut(field).expect("seeded above").push(mrid.clone());
                }
            }
        }
        for v in by_field.values_mut() {
            v.sort_unstable();
        }
        Self { by_field }
    }

    fn subjects(&self, field: &'static str) -> &[String] {
        self.by_field.get(field).map_or(&[], Vec::as_slice)
    }
}

/// Every mRID a shape's targets select.
fn targets_of(ds: &CimDataset, shape: &ShapeDef, subjects: &SubjectIndex) -> Vec<String> {
    let mut mrids: Vec<String> = Vec::new();
    for target in shape.targets {
        match target {
            Target::Class(classes) => {
                for class in *classes {
                    if let Some(list) = ds.by_type.get(*class) {
                        mrids.extend(list.iter().cloned());
                    }
                }
            }
            Target::SubjectsOf(field) => {
                mrids.extend(subjects.subjects(field).iter().cloned());
            }
        }
    }
    mrids.sort_unstable();
    mrids.dedup();
    mrids
}

/// Validate one profile's shapes against a dataset.
///
/// `shapes` is the whole table; only shapes belonging to `profile` run, which is
/// what makes a shared constraint file (imported by 17 of 18 manifests) cost
/// nothing on the profiles that do not use it.
pub fn validate_profile(
    ds: &CimDataset,
    profile: &str,
    shapes: &'static [ShapeDef],
    _cfg: &Config,
) -> Vec<Violation> {
    let active: Vec<&ShapeDef> = shapes
        .iter()
        .filter(|s| s.profiles.contains(&profile))
        .collect();
    if active.is_empty() {
        return Vec::new();
    }

    // Build the reverse index only if something asks an inverse question, and
    // only over the fields that are actually asked about.
    let mut inverse_fields: Vec<&'static str> = active
        .iter()
        .flat_map(|s| s.props.iter())
        .flat_map(|p| match p.path {
            Path::Inverse(f) => vec![f],
            Path::Alternative(branches) => branches
                .iter()
                .filter_map(|b| match b {
                    AltBranch::Inverse(f) => Some(*f),
                    AltBranch::Forward(_) => None,
                })
                .collect(),
            _ => Vec::new(),
        })
        .collect();
    inverse_fields.sort_unstable();
    inverse_fields.dedup();
    let reverse = (!inverse_fields.is_empty())
        .then(|| ReverseIndex::build(ds, &inverse_fields));

    // The same treatment for sh:targetSubjectsOf: one pass over the fields any
    // active shape targets, rather than a dataset scan per shape.
    let mut subject_fields: Vec<&'static str> = active
        .iter()
        .flat_map(|s| s.targets.iter())
        .filter_map(|t| match t {
            Target::SubjectsOf(f) => Some(*f),
            Target::Class(_) => None,
        })
        .collect();
    subject_fields.sort_unstable();
    subject_fields.dedup();
    let subjects = if subject_fields.is_empty() {
        SubjectIndex::default()
    } else {
        SubjectIndex::build(ds, &subject_fields)
    };

    let mut out = Vec::new();
    for shape in &active {
        for mrid in targets_of(ds, shape, &subjects) {
            let Some(entry) = ds.entries.get(&mrid) else { continue };
            let Some(el) = entry.element.as_any().downcast_ref::<GenericElement>() else {
                // A target that decoded as a typed CGMES struct. Its NC-only
                // fields were dropped at decode, so anything asked of them
                // would report absent values that the XML did carry.
                continue;
            };
            for prop in shape.props {
                check_prop(ds, el, &mrid, prop, reverse.as_ref(), &mut out);
            }
            if let Some(closed) = shape.closed {
                check_closed(el, &mrid, closed, &mut out);
            }
        }
    }
    out
}
