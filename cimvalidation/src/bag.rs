//! Validates a dataset against a [`crate::shapes`] table.
//!
//! The counterpart to the generated `generated_*_shacl.rs` validators, which
//! read typed struct fields. Here every value is the string the XML carried,
//! so the checks the generated path treats as tautologies — `sh:datatype`,
//! `sh:nodeKind` — are real, and `sh:closed` becomes answerable at all.
//!
//! Which elements a run reads is a [`Source`]: NC shapes read property bags,
//! CGMES shapes read typed elements through the [`cimdecoder::CimEntry::block`]
//! the decoder keeps beside every struct. The block holds every field the XML
//! carried, which is what lets one interpreter serve both. It is gone after
//! `CimDataset::drop_blocks()`, so a typed element must be validated before
//! that.


use cimdecoder::{CimDataset, CimEntry};
use cimstructs::base::{FastMap, FastSet, FieldMap, FieldValue, GenericElement};

use crate::helpers;
use crate::shapes::{
    AltBranch, Check, ClosedShape, Constraint, Logic, LogicOp, NodeKind, Path, PropShape, ShapeDef, Step,
    Target,
};
use crate::{Config, Violation};

/// Maps a referenced mRID back to the elements pointing at it.
///
/// `sh:inversePath` asks "how many things point at me", which the forward
/// fields cannot answer. Built once per call in O(associations) rather than
/// rescanned per shape, and only over the fields some active shape asks about.
///
/// A plain inverse path needs only the count; an inverse step inside a
/// [`Path::Chain`] has to continue from the elements themselves, so those
/// fields keep the list. Keys borrow from the dataset: an owned key cost an
/// allocation per reference at build time and another per lookup.
#[derive(Default)]
struct ReverseIndex<'a> {
    /// `(target mrid, field key)` → how many elements point at it that way.
    counts: FastMap<(&'a str, &'static str), u32>,
    /// `(target mrid, field key)` → the elements pointing at it that way.
    sources: FastMap<(&'a str, &'static str), Vec<&'a str>>,
}

impl<'a> ReverseIndex<'a> {
    fn build(
        ds: &'a CimDataset,
        source: Source,
        count_fields: &[&'static str],
        list_fields: &[&'static str],
    ) -> Self {
        let mut idx = Self::default();
        if count_fields.is_empty() && list_fields.is_empty() {
            return idx;
        }
        for (mrid, entry) in &ds.entries {
            let Some(f) = source.fields(entry) else { continue };
            for field in count_fields {
                for target in refs_of(f, field) {
                    *idx.counts.entry((target.as_str(), *field)).or_insert(0) += 1;
                }
            }
            for field in list_fields {
                for target in refs_of(f, field) {
                    idx.sources.entry((target.as_str(), *field)).or_default().push(mrid);
                }
            }
        }
        idx
    }

    fn count(&self, mrid: &str, field: &'static str) -> u32 {
        self.counts.get(&(mrid, field)).copied().unwrap_or(0)
    }

    fn sources<'s>(&'s self, mrid: &'a str, field: &'static str) -> &'s [&'a str] {
        self.sources.get(&(mrid, field)).map_or(&[], Vec::as_slice)
    }
}

/// Everything a check needs besides the element itself.
struct Ctx<'a> {
    ds: &'a CimDataset,
    source: Source,
    reverse: ReverseIndex<'a>,
}

/// What a [`Path::Chain`] collected.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Kind {
    Text,
    Refs,
    /// Class names, from a final `rdf:type` step.
    Types,
}

/// The values a path yields for one element, as far as a constraint needs them.
enum Values<'a> {
    /// Literal text values. A slice of the stored field, so a single value
    /// costs no allocation — this runs once per property per target.
    Text(&'a [String]),
    /// `rdf:resource` references.
    Refs(&'a [String]),
    /// Only a count is available — an inverse or alternative path.
    Count(u32),
    Absent,
    /// The end of a [`Path::Chain`]. Owned, because a chain gathers values
    /// from several elements.
    Chain(Kind, Vec<&'a str>),
    /// A chain stepped onto an element this dataset does not hold. What lies
    /// beyond is not known, so nothing can be said about it either way.
    Unknown,
}

impl<'a> Values<'a> {
    /// `None` when unknown: no cardinality can be asserted.
    fn count(&self) -> Option<u32> {
        Some(match self {
            Values::Text(v) | Values::Refs(v) => v.len() as u32,
            Values::Count(n) => *n,
            Values::Absent => 0,
            Values::Chain(_, v) => v.len() as u32,
            Values::Unknown => return None,
        })
    }

    /// Does `f` hold for any literal value?
    fn any_text(&self, mut f: impl FnMut(&str) -> bool) -> bool {
        match self {
            Values::Text(vs) => vs.iter().any(|v| f(v)),
            Values::Chain(Kind::Text, vs) => vs.iter().any(|v| f(v)),
            _ => false,
        }
    }

    /// Does `f` hold for any referenced mRID?
    fn any_ref(&self, mut f: impl FnMut(&str) -> bool) -> bool {
        match self {
            Values::Refs(vs) => vs.iter().any(|v| f(v)),
            Values::Chain(Kind::Refs, vs) => vs.iter().any(|v| f(v)),
            _ => false,
        }
    }

    fn is_text(&self) -> bool {
        matches!(self, Values::Text(_) | Values::Chain(Kind::Text, _))
    }

    fn is_refs(&self) -> bool {
        matches!(self, Values::Refs(_) | Values::Chain(Kind::Refs, _))
    }
}

type Fields = FieldMap;

/// Which elements a shape table reads: its own family's.
///
/// Not a convenience — the families' shapes would otherwise reach each other's
/// elements. NC has `sh:targetSubjectsOf` shapes on `IdentifiedObject.name`,
/// so reading typed elements would make every named CGMES element in a mixed
/// dataset an NC target; and an inverse-path count would include references
/// from the other family.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Source {
    /// [`GenericElement`] property bags, read directly.
    Bags,
    /// Generated structs, read through the decoder's block.
    Typed,
}

impl Source {
    /// The element's fields, or `None` if it is not this source's. A typed
    /// element whose block was dropped also has none — see [`validate_shapes`]
    /// for why a target in that state is an error rather than a skip.
    fn fields(self, entry: &CimEntry) -> Option<&Fields> {
        let bag = entry.element.as_any().downcast_ref::<GenericElement>();
        match (self, bag) {
            (Source::Bags, Some(el)) => Some(el.fields()),
            (Source::Typed, None) if !entry.block.type_name.is_empty() => Some(&entry.block.fields),
            _ => None,
        }
    }

    fn owns(self, entry: &CimEntry) -> bool {
        let bag = entry.element.as_any().is::<GenericElement>();
        bag == (self == Source::Bags)
    }
}

fn refs_of<'a>(f: &'a Fields, field: &str) -> &'a [String] {
    match f.get(field) {
        Some(FieldValue::Resource(s)) => std::slice::from_ref(s),
        Some(FieldValue::ResourceList(v)) => v.as_slice(),
        _ => &[],
    }
}

fn field_values<'a>(f: &'a Fields, field: &str) -> Values<'a> {
    match f.get(field) {
        Some(FieldValue::Text(s)) => Values::Text(std::slice::from_ref(s)),
        Some(FieldValue::TextList(v)) => Values::Text(v.as_slice()),
        Some(FieldValue::Resource(_)) | Some(FieldValue::ResourceList(_)) => {
            Values::Refs(refs_of(f, field))
        }
        None => Values::Absent,
    }
}

/// A numeric value, or `None` if it does not parse — a typed numeric field
/// that failed to parse is `None` and skipped the same way.
fn number(v: &str) -> Option<f64> {
    v.trim().parse::<f64>().ok()
}

fn resolve<'a>(ctx: &Ctx<'a>, el: &'a Fields, mrid: &'a str, path: &Path) -> Values<'a> {
    match path {
        Path::Forward(field) | Path::RefType(field) => field_values(el, field),
        Path::Inverse(field) => Values::Count(ctx.reverse.count(mrid, field)),
        Path::Alternative(branches) => {
            // The union across branches. Only the count is well defined: the
            // branches can mix literals, references and inverse hops, and every
            // NCP alternative shape asks a cardinality question.
            let mut total = 0;
            for b in *branches {
                total += match b {
                    AltBranch::Forward(field) => field_values(el, field).count().unwrap_or(0),
                    AltBranch::Inverse(field) => ctx.reverse.count(mrid, field),
                };
            }
            Values::Count(total)
        }
        Path::Chain(steps) => chain(ctx, el, mrid, steps),
    }
}

/// Walk a sequence path from `mrid`.
///
/// Every element stepped *from* must be in the dataset and of this source,
/// since its fields are read; otherwise the result is [`Values::Unknown`]. An
/// inverse step never needs that — the reverse index holds what points here.
/// A literal reached before the last step ends that branch of the walk.
fn chain<'a>(ctx: &Ctx<'a>, focus: &'a Fields, mrid: &'a str, steps: &[Step]) -> Values<'a> {
    let mut nodes: Vec<&'a str> = vec![mrid];
    for (i, step) in steps.iter().enumerate() {
        let last = i + 1 == steps.len();
        match step {
            Step::Forward(field) => {
                let mut next: Vec<&'a str> = Vec::new();
                let mut texts: Vec<&'a str> = Vec::new();
                for &node in &nodes {
                    let fields = if node == mrid {
                        focus
                    } else {
                        match ctx.ds.entries.get(node).and_then(|e| ctx.source.fields(e)) {
                            Some(f) => f,
                            None => return Values::Unknown,
                        }
                    };
                    match fields.get(*field) {
                        Some(FieldValue::Text(s)) => texts.push(s),
                        Some(FieldValue::TextList(v)) => texts.extend(v.iter().map(String::as_str)),
                        Some(FieldValue::Resource(r)) => next.push(r),
                        Some(FieldValue::ResourceList(v)) => next.extend(v.iter().map(String::as_str)),
                        None => {}
                    }
                }
                if last {
                    return if texts.is_empty() {
                        Values::Chain(Kind::Refs, next)
                    } else {
                        Values::Chain(Kind::Text, texts)
                    };
                }
                nodes = next;
            }
            Step::Inverse(field) => {
                let next: Vec<&'a str> =
                    nodes.iter().flat_map(|n| ctx.reverse.sources(n, field).iter().copied()).collect();
                if last {
                    return Values::Chain(Kind::Refs, next);
                }
                nodes = next;
            }
            Step::Type => {
                let mut types: Vec<&'a str> = Vec::with_capacity(nodes.len());
                for &node in &nodes {
                    match ctx.ds.entries.get(node) {
                        Some(e) => types.push(e.element.type_name()),
                        None => return Values::Unknown,
                    }
                }
                return Values::Chain(Kind::Types, types);
            }
        }
    }
    Values::Chain(Kind::Refs, nodes)
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

/// A path's field key, for the `property` column of a violation: the property
/// of the focus element the path starts from.
fn path_label(path: &Path) -> &'static str {
    match path {
        Path::Forward(f) | Path::RefType(f) | Path::Inverse(f) => f,
        Path::Alternative(branches) => match branches.first() {
            Some(AltBranch::Forward(f)) | Some(AltBranch::Inverse(f)) => f,
            None => "",
        },
        Path::Chain(steps) => match steps.first() {
            Some(Step::Forward(f)) | Some(Step::Inverse(f)) => f,
            _ => "",
        },
    }
}

/// The class of a referenced element, or `None` when it is not in this
/// dataset. Class checks are silent then: phase 1 runs per file, so a
/// reference out of the file is normal and belongs to a cross-profile rule.
/// Reporting it would make every cross-file association a value-type
/// violation.
fn class_of<'a>(ctx: &Ctx<'a>, mrid: &str) -> Option<&'static str> {
    ctx.ds.entries.get(mrid).map(|e| e.element.type_name())
}

/// Does `values` violate `constraint`? Unknown values violate nothing.
fn fails(ctx: &Ctx<'_>, el: &Fields, values: &Values<'_>, constraint: &Constraint) -> bool {
    if matches!(values, Values::Unknown) {
        return false;
    }
    match constraint {
        Constraint::MinCount(n) => values.count().is_some_and(|c| c < *n),
        Constraint::MaxCount(n) => values.count().is_some_and(|c| c > *n),

        // A literal constraint on a reference is a node-kind problem, which
        // the shape's own sh:nodeKind check reports. Flagging it twice would
        // double-count one mistake.
        Constraint::Datatype(xsd) => values.any_text(|v| !datatype_ok(v, xsd)),

        // An inverse or alternative path yields only a count and an rdf:type
        // step only class names, so there is no node to inspect.
        Constraint::NodeKind(kind) => match kind {
            NodeKind::Literal => values.is_refs() && values.count() != Some(0),
            NodeKind::Iri => values.is_text() && values.count() != Some(0),
            NodeKind::BlankNode => (values.is_text() || values.is_refs()) && values.count() != Some(0),
        },

        Constraint::In(allowed) => {
            values.any_text(|v| !allowed.contains(&v.trim())) || values.any_ref(|r| !allowed.contains(&r))
        }
        Constraint::HasValue(want) => match values {
            v if v.is_text() => !v.any_text(|t| t.trim() == *want),
            v if v.is_refs() => !v.any_ref(|r| r == *want),
            _ => false,
        },

        Constraint::MaxLength(n) => values.any_text(|v| v.chars().count() as u32 > *n),
        Constraint::MinLength(n) => values.any_text(|v| (v.chars().count() as u32) < *n),

        Constraint::Class(allowed) | Constraint::RefClass(allowed) => match values {
            // A chain that ends in rdf:type has the classes already.
            Values::Chain(Kind::Types, types) => types.iter().any(|t| !allowed.contains(t)),
            v => v.any_ref(|r| class_of(ctx, r).is_some_and(|t| !allowed.contains(&t))),
        },
        Constraint::NotClass(forbidden) => {
            values.any_ref(|r| class_of(ctx, r).is_some_and(|t| forbidden.contains(&t)))
        }

        Constraint::MinInclusive(b) => values.any_text(|v| number(v).is_some_and(|x| x < *b)),
        Constraint::MaxInclusive(b) => values.any_text(|v| number(v).is_some_and(|x| x > *b)),
        Constraint::MinExclusive(b) => values.any_text(|v| number(v).is_some_and(|x| x <= *b)),
        Constraint::MaxExclusive(b) => values.any_text(|v| number(v).is_some_and(|x| x >= *b)),

        // Every pairing of this property's values with the other's, as SHACL
        // defines it. Silent unless both sides are present.
        Constraint::LessThan(other) => pair(values, el, other, |a, b| a < b),
        Constraint::LessThanOrEquals(other) => pair(values, el, other, |a, b| a <= b),
    }
}

fn check_prop<'a>(
    ctx: &Ctx<'a>,
    el: &'a Fields,
    class: &str,
    mrid: &'a str,
    prop: &PropShape,
    out: &mut Vec<Violation>,
) {
    let values = resolve(ctx, el, mrid, &prop.path);
    for c in prop.checks {
        if fails(ctx, el, &values, &c.constraint) {
            out.push(violation(mrid, class, path_label(&prop.path), c));
        }
    }
}

/// Does a [`Logic`] branch conform? `None` when a path in it left the
/// dataset, so its conformance is not known.
fn branch_conforms<'a>(ctx: &Ctx<'a>, el: &'a Fields, mrid: &'a str, branch: &[PropShape]) -> Option<bool> {
    for prop in branch {
        let values = resolve(ctx, el, mrid, &prop.path);
        if matches!(values, Values::Unknown) {
            return None;
        }
        if prop.checks.iter().any(|c| fails(ctx, el, &values, &c.constraint)) {
            return Some(false);
        }
    }
    Some(true)
}

/// Node-level `sh:and` / `sh:or` / `sh:xone`. Reported with the node shape's
/// name as the property, which is what the generated validators report.
fn check_logic<'a>(
    ctx: &Ctx<'a>,
    el: &'a Fields,
    class: &str,
    mrid: &'a str,
    logic: &Logic,
    out: &mut Vec<Violation>,
) {
    let mut conforming = 0usize;
    for branch in logic.branches {
        match branch_conforms(ctx, el, mrid, branch) {
            Some(true) => conforming += 1,
            Some(false) => {}
            // An unknown branch could tip any of the three either way.
            None => return,
        }
    }
    let holds = match logic.op {
        LogicOp::And => conforming == logic.branches.len(),
        LogicOp::Or => conforming >= 1,
        LogicOp::Xone => conforming == 1,
    };
    if !holds {
        out.push(Violation {
            object_id: mrid.to_string(),
            rule_id: logic.rule_id.to_string(),
            class: class.to_string(),
            property: logic.name.to_string(),
            message: logic.message.to_string(),
            severity: logic.severity.to_string(),
            name: logic.name.to_string(),
            description: logic.description.to_string(),
        });
    }
}

/// Does any pairing of this property's numbers with `other`'s fail `holds`?
fn pair(values: &Values, el: &Fields, other: &str, holds: impl Fn(f64, f64) -> bool) -> bool {
    let others = field_values(el, other);
    values.any_text(|a| {
        number(a).is_some_and(|a| others.any_text(|b| number(b).is_some_and(|b| !holds(a, b))))
    })
}

/// `sh:closed` — report every field the profile does not allow on this class.
fn check_closed(el: &Fields, class: &str, mrid: &str, closed: &ClosedShape, out: &mut Vec<Violation>) {
    let mut extra: Vec<&str> = el
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
struct SubjectIndex<'a> {
    by_field: FastMap<&'static str, Vec<&'a String>>,
}

impl<'a> SubjectIndex<'a> {
    fn build(ds: &'a CimDataset, source: Source, fields: &[&'static str]) -> Self {
        let mut by_field: FastMap<&'static str, Vec<&'a String>> =
            fields.iter().map(|f| (*f, Vec::new())).collect();
        for (mrid, entry) in &ds.entries {
            let Some(el) = source.fields(entry) else { continue };
            for field in fields {
                if el.contains_key(*field) {
                    by_field.get_mut(field).expect("seeded above").push(mrid);
                }
            }
        }
        Self { by_field }
    }

    fn subjects(&self, field: &'static str) -> &[&'a String] {
        self.by_field.get(field).map_or(&[], Vec::as_slice)
    }
}

/// Every mRID a shape's targets select, in `by_type` order.
///
/// That order is reproducible from run to run: decoding appends in document
/// order, and `merge` walks a map whose hasher has a fixed seed. Sorting per
/// shape for determinism cost up to 30% on profiles with few shapes per class.
///
/// Borrowed rather than cloned: cloning every mRID for every shape was the
/// interpreter's largest avoidable cost on datasets with many small elements.
fn targets_of<'a>(ds: &'a CimDataset, shape: &ShapeDef, subjects: &SubjectIndex<'a>) -> Vec<&'a String> {
    let mut mrids: Vec<&String> = Vec::new();
    let mut subjects_of = false;
    for target in shape.targets {
        match target {
            Target::Class(classes) => {
                for class in *classes {
                    if let Some(list) = ds.by_type.get(*class) {
                        mrids.extend(list.iter());
                    }
                }
            }
            Target::SubjectsOf(field) => {
                subjects_of = true;
                mrids.extend_from_slice(subjects.subjects(field));
            }
        }
    }
    // An element has one class, so class lists never overlap; only a
    // subjectsOf target can repeat an element another target already chose.
    if subjects_of {
        let mut seen = FastSet::default();
        mrids.retain(|m| seen.insert(*m));
    }
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
    validate_shapes(ds, Source::Bags, &active)
}

/// Validate a dataset against shapes the caller has already selected.
///
/// The selection is the caller's because the families select differently: NC
/// by the profile codes its manifests assign, CGMES by profile plus the
/// solved/not-solved and local/cross-profile split its file names encode.
///
/// Targets outside `source` are skipped: a family's shapes describe its own
/// elements.
///
/// # Panics
///
/// If a target is a typed element whose block was dropped: there is nothing
/// left to read, and skipping it would report the element as valid.
pub fn validate_shapes(ds: &CimDataset, source: Source, active: &[&ShapeDef]) -> Vec<Violation> {
    if active.is_empty() {
        return Vec::new();
    }

    // Build the reverse index only if something asks an inverse question, and
    // only over the fields that are actually asked about.
    let mut count_fields: Vec<&'static str> = Vec::new();
    let mut list_fields: Vec<&'static str> = Vec::new();
    let paths = active.iter().flat_map(|s| {
        s.props
            .iter()
            .map(|p| &p.path)
            .chain(s.logic.iter().flat_map(|l| l.branches.iter().flat_map(|b| b.iter().map(|p| &p.path))))
    });
    for path in paths {
        match path {
            Path::Inverse(f) => count_fields.push(f),
            Path::Alternative(branches) => count_fields.extend(branches.iter().filter_map(|b| match b {
                AltBranch::Inverse(f) => Some(*f),
                AltBranch::Forward(_) => None,
            })),
            Path::Chain(steps) => list_fields.extend(steps.iter().filter_map(|s| match s {
                Step::Inverse(f) => Some(*f),
                _ => None,
            })),
            Path::Forward(_) | Path::RefType(_) => {}
        }
    }
    count_fields.sort_unstable();
    count_fields.dedup();
    list_fields.sort_unstable();
    list_fields.dedup();
    let ctx = Ctx {
        ds,
        source,
        reverse: ReverseIndex::build(ds, source, &count_fields, &list_fields),
    };

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
        SubjectIndex::build(ds, source, &subject_fields)
    };

    let mut out = Vec::new();
    for shape in active {
        for mrid in targets_of(ds, shape, &subjects) {
            let Some(entry) = ds.entries.get(mrid) else { continue };
            if !source.owns(entry) {
                // Another family's element, e.g. a CGMES struct under an NC
                // shape: the shape asks about attributes that element's family
                // does not define, and would report them absent.
                continue;
            }
            let el = source.fields(entry).unwrap_or_else(|| {
                panic!(
                    "{mrid} ({}) has no fields to validate: its block was dropped \
                     (CimDataset::drop_blocks) before validation",
                    entry.element.type_name()
                )
            });
            let class = entry.element.type_name();
            for prop in shape.props {
                check_prop(&ctx, el, class, mrid, prop, &mut out);
            }
            for logic in shape.logic {
                check_logic(&ctx, el, class, mrid, logic, &mut out);
            }
            if let Some(closed) = shape.closed {
                check_closed(el, class, mrid, closed, &mut out);
            }
        }
    }
    out
}
