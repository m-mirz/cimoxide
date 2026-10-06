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
use crate::par::{par_concat, par_map, runs, threads_for};
use crate::shapes::{
    AltBranch, Branch, Check, ClosedShape, Constraint, Logic, LogicOp, NodeKind, Path, PropShape, ShapeDef, Step,
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
        Path::Forward(field) => field_values(el, field),
        // `( X.y rdf:type )`: what the general walk below yields for it — the
        // field's references, or its literal — without the allocations.
        Path::Chain([Step::Forward(field), Step::Type]) => field_values(el, field),
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
/// inverse step never needs that — the reverse index holds what points here,
/// and neither does a final `rdf:type` step. A literal reached before the last
/// step ends that branch of the walk, unless `rdf:type` follows: then it is
/// the value, the way a one-step path would report it.
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
                // A literal where the path continues to rdf:type is reported
                // as the literal it is, so `sh:nodeKind sh:IRI` can flag it.
                let before_type = matches!(steps.get(i + 1), Some(Step::Type));
                if (last || before_type) && !texts.is_empty() {
                    return Values::Chain(Kind::Text, texts);
                }
                if last {
                    return Values::Chain(Kind::Refs, next);
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
            // The values are the elements reached. Nothing is read from them,
            // so they count even when absent; the class constraints a type
            // path carries look up the ones that are present.
            Step::Type => return Values::Chain(Kind::Refs, nodes),
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
fn path_label(path: &Path) -> std::borrow::Cow<'static, str> {
    use std::borrow::Cow;
    // An inverse step reads as SHACL writes it: `^Terminal.ConductingEquipment`
    // on a Switch means the terminals pointing at it, not an attribute of its own.
    let inverse = |f: &str| Cow::Owned(format!("^{f}"));
    match path {
        Path::Forward(f) => Cow::Borrowed(f),
        Path::Inverse(f) => inverse(f),
        Path::Alternative(branches) => match branches.first() {
            Some(AltBranch::Forward(f)) => Cow::Borrowed(f),
            Some(AltBranch::Inverse(f)) => inverse(f),
            None => Cow::Borrowed(""),
        },
        Path::Chain(steps) => match steps.first() {
            Some(Step::Forward(f)) => Cow::Borrowed(f),
            Some(Step::Inverse(f)) => inverse(f),
            _ => Cow::Borrowed(""),
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

        // An inverse or alternative path yields only a count, so there is no
        // node to inspect.
        Constraint::NodeKind(kind) => match kind {
            NodeKind::Literal => values.is_refs() && values.count() != Some(0),
            NodeKind::Iri => values.is_text() && values.count() != Some(0),
            // A reference decodes the same whether the XML wrote an IRI or a
            // blank node, so only a literal is certainly not one.
            NodeKind::BlankNode => values.is_text() && values.count() != Some(0),
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
        Constraint::Length(n) => values.any_text(|v| v.chars().count() as u32 != *n),

        Constraint::Class(allowed) | Constraint::RefClass(allowed) => {
            values.any_ref(|r| class_of(ctx, r).is_some_and(|t| !allowed.contains(&t)))
        }
        Constraint::NotClass(forbidden) => {
            values.any_ref(|r| class_of(ctx, r).is_some_and(|t| forbidden.contains(&t)))
        }
        Constraint::QualifiedIn { allowed, min } => {
            let mut n = 0u32;
            values.any_text(|v| {
                n += u32::from(allowed.contains(&v.trim()));
                false
            });
            values.any_ref(|r| {
                n += u32::from(allowed.contains(&r));
                false
            });
            n < *min
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
            out.push(violation(mrid, class, &path_label(&prop.path), c));
        }
    }
}

/// Does a [`Logic`] branch conform? `None` when a path in it left the
/// dataset, so its conformance is not known.
fn branch_conforms<'a>(ctx: &Ctx<'a>, el: &'a Fields, mrid: &'a str, branch: &Branch) -> Option<bool> {
    let mut holds = true;
    for prop in branch.props {
        let values = resolve(ctx, el, mrid, &prop.path);
        if matches!(values, Values::Unknown) {
            return None;
        }
        if prop.checks.iter().any(|c| fails(ctx, el, &values, &c.constraint)) {
            holds = false;
            break;
        }
    }
    Some(holds != branch.negate)
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

/// A target element, resolved once per call: its mRID, its class, and the
/// fields it is read through.
#[derive(Clone, Copy)]
struct Resolved<'a> {
    mrid: &'a String,
    class: &'static str,
    fields: &'a Fields,
}

/// Resolve `mrid` for `source`: `None` when it is missing or another family's
/// element — e.g. a CGMES struct under an NC shape, whose attributes that
/// family does not define and would report as absent.
///
/// # Panics
///
/// For a typed element of this source whose block was dropped: there is
/// nothing left to read, and skipping it would report the element as valid.
fn resolve_target<'a>(ds: &'a CimDataset, source: Source, mrid: &'a String) -> Option<Resolved<'a>> {
    let entry = ds.entries.get(mrid)?;
    if !source.owns(entry) {
        return None;
    }
    let fields = source.fields(entry).unwrap_or_else(|| {
        panic!(
            "{mrid} ({}) has no fields to validate: its block was dropped \
             (CimDataset::drop_blocks) before validation",
            entry.element.type_name()
        )
    });
    Some(Resolved { mrid, class: entry.element.type_name(), fields })
}

/// The reverse index and the `sh:targetSubjectsOf` index, built in one pass.
///
/// Both need a walk over every element, and that walk — random access into a
/// map of every element, then into each one's fields — was most of their
/// cost; one pass does it once. Only fields some active shape asks about are
/// indexed, and nothing is scanned when no shape asks.
///
/// `sh:targetSubjectsOf` has no `by_type` to answer it, and scanning per shape
/// is O(elements x shapes): on a 20k-element dataset that cost 10 ms for a
/// profile whose shapes matched nothing.
fn build_indexes<'a>(
    ds: &'a CimDataset,
    source: Source,
    count_fields: &[&'static str],
    list_fields: &[&'static str],
    subject_fields: &[&'static str],
) -> (ReverseIndex<'a>, FastMap<&'static str, Vec<Resolved<'a>>>) {
    if count_fields.is_empty() && list_fields.is_empty() && subject_fields.is_empty() {
        return (ReverseIndex::default(), FastMap::default());
    }
    type Partial<'a> = (ReverseIndex<'a>, FastMap<&'static str, Vec<Resolved<'a>>>);
    let index = |entries: &mut dyn Iterator<Item = (&'a String, &'a CimEntry)>| -> Partial<'a> {
        let mut reverse = ReverseIndex::default();
        let mut subjects: FastMap<&'static str, Vec<Resolved<'a>>> =
            subject_fields.iter().map(|f| (*f, Vec::new())).collect();
        for (mrid, entry) in entries {
            let Some(f) = source.fields(entry) else { continue };
            for field in count_fields {
                for target in refs_of(f, field) {
                    *reverse.counts.entry((target.as_str(), *field)).or_insert(0) += 1;
                }
            }
            for field in list_fields {
                for target in refs_of(f, field) {
                    reverse.sources.entry((target.as_str(), *field)).or_default().push(mrid);
                }
            }
            for field in subject_fields {
                if f.contains_key(*field) {
                    let r = Resolved { mrid, class: entry.element.type_name(), fields: f };
                    subjects.get_mut(field).expect("seeded above").push(r);
                }
            }
        }
        (reverse, subjects)
    };

    let threads = threads_for(ds.entries.len());
    if threads == 1 {
        return index(&mut ds.entries.iter());
    }
    // One pass split into contiguous runs of the map's own order, each indexed
    // on its own thread, then merged in run order: the lists come out exactly
    // as the single pass builds them.
    let all: Vec<(&'a String, &'a CimEntry)> = ds.entries.iter().collect();
    let parts: Vec<Partial<'a>> = par_map(&runs(&all, threads, |_| 1), |run| index(&mut run.iter().copied()));
    let mut parts = parts.into_iter();
    let (mut reverse, mut subjects) = parts.next().expect("at least one run");
    for (r, subj) in parts {
        for (k, n) in r.counts {
            *reverse.counts.entry(k).or_insert(0) += n;
        }
        for (k, list) in r.sources {
            reverse.sources.entry(k).or_default().extend(list);
        }
        for (field, list) in subj {
            subjects.get_mut(field).expect("seeded above").extend(list);
        }
    }
    (reverse, subjects)
}

/// Every element a shape's targets select, in `by_type` order — for a shape
/// that mixes target kinds; the others are walked element-major in
/// [`validate_shapes`].
///
/// That order is reproducible from run to run: decoding appends in document
/// order, and `merge` walks a map whose hasher has a fixed seed. A class's
/// elements are resolved once per call and shared with the element-major walk.
fn targets_of<'a>(
    ds: &'a CimDataset,
    source: Source,
    shape: &ShapeDef,
    subjects: &FastMap<&'static str, Vec<Resolved<'a>>>,
    by_class: &mut FastMap<&'static str, Vec<Resolved<'a>>>,
) -> Vec<Resolved<'a>> {
    let mut out: Vec<Resolved<'a>> = Vec::new();
    for target in shape.targets {
        match target {
            Target::Class(classes) => {
                for class in *classes {
                    let list = by_class.entry(class).or_insert_with(|| {
                        ds.by_type
                            .get(*class)
                            .into_iter()
                            .flatten()
                            .filter_map(|mrid| resolve_target(ds, source, mrid))
                            .collect()
                    });
                    out.extend_from_slice(list);
                }
            }
            Target::SubjectsOf(field) => {
                out.extend_from_slice(subjects.get(field).map_or(&[], Vec::as_slice));
            }
        }
    }
    // A shape's targets overlap whenever it lists a class and one of its
    // subclasses (`cim:Switch, cim:Breaker`), and a subjectsOf target can pick
    // any of them again. SHACL validates a focus node once per shape.
    let mut seen = FastSet::default();
    out.retain(|t| seen.insert(t.mrid));
    out
}

/// Every check of one shape against one target.
fn run_shape<'a>(ctx: &Ctx<'a>, shape: &ShapeDef, t: &Resolved<'a>, out: &mut Vec<Violation>) {
    for prop in shape.props {
        check_prop(ctx, t.fields, t.class, t.mrid, prop, out);
    }
    for logic in shape.logic {
        check_logic(ctx, t.fields, t.class, t.mrid, logic, out);
    }
    if let Some(closed) = shape.closed {
        check_closed(t.fields, t.class, t.mrid, closed, out);
    }
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
/// Elements per slice of a class's targets, the unit of work a thread takes.
const SLICE: usize = 2048;

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
            .chain(s.logic.iter().flat_map(|l| l.branches.iter().flat_map(|b| b.props.iter().map(|p| &p.path))))
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
            Path::Forward(_) => {}
        }
    }
    count_fields.sort_unstable();
    count_fields.dedup();
    list_fields.sort_unstable();
    list_fields.dedup();

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

    let (reverse, subjects) = build_indexes(ds, source, &count_fields, &list_fields, &subject_fields);
    let ctx = Ctx { ds, source, reverse };

    // Element-major: visit each target once and apply every shape that
    // targets it, rather than walking a class's elements once per shape. The
    // cost per visit is reaching the element's fields — a cache miss into its
    // own map — so doing it once for all of a class's shapes, not once per
    // shape, is what pays. Shapes are grouped by their one kind of target; a
    // shape mixing kinds can select an element twice and keeps the per-shape
    // walk, which removes the duplicate. Within one kind, the classes are
    // de-duplicated per shape: `sh:targetClass cim:Switch, cim:Breaker`
    // expands `Switch` to its subclasses, Breaker among them, and a breaker
    // must still be checked once.
    let mut by_class_shapes: FastMap<&'static str, Vec<&ShapeDef>> = FastMap::default();
    let mut by_subject_shapes: FastMap<&'static str, Vec<&ShapeDef>> = FastMap::default();
    let mut mixed: Vec<&ShapeDef> = Vec::new();
    for shape in active {
        let all_class = shape.targets.iter().all(|t| matches!(t, Target::Class(_)));
        match shape.targets {
            _ if all_class => {
                let mut classes: Vec<&'static str> = shape
                    .targets
                    .iter()
                    .flat_map(|t| match t {
                        Target::Class(cs) => cs.iter().copied(),
                        Target::SubjectsOf(_) => [].iter().copied(),
                    })
                    .collect();
                classes.sort_unstable();
                classes.dedup();
                for class in classes {
                    by_class_shapes.entry(class).or_default().push(shape);
                }
            }
            [Target::SubjectsOf(field)] => by_subject_shapes.entry(field).or_default().push(shape),
            _ => mixed.push(shape),
        }
    }

    // The class-targeted walk is most of the work, and it is memory-bound:
    // each element costs a cache miss into its own field map, which threads
    // overlap well. Each class's elements are cut into slices, and the slices,
    // in order, are dealt out to threads in contiguous runs of about equal
    // size. Concatenating the runs in order gives exactly the sequential
    // result, order included.
    let work: Vec<(&[&ShapeDef], &[String])> = by_class_shapes
        .iter()
        .flat_map(|(class, shapes)| {
            ds.by_type
                .get(*class)
                .map_or(&[][..], Vec::as_slice)
                .chunks(SLICE)
                .map(move |mrids| (shapes.as_slice(), mrids))
        })
        .collect();
    let total: usize = work.iter().map(|(_, m)| m.len()).sum();
    let mut out = par_concat(&runs(&work, threads_for(total), |(_, m)| m.len()), |items| {
        let mut out = Vec::new();
        for (shapes, mrids) in items {
            for t in mrids.iter().filter_map(|mrid| resolve_target(ds, source, mrid)) {
                for shape in *shapes {
                    run_shape(&ctx, shape, &t, &mut out);
                }
            }
        }
        out
    });

    // The same for `sh:targetSubjectsOf`, whose targets the index pass found.
    let work: Vec<(&[&ShapeDef], &[Resolved])> = by_subject_shapes
        .iter()
        .flat_map(|(field, shapes)| {
            subjects
                .get(field)
                .map_or(&[][..], Vec::as_slice)
                .chunks(SLICE)
                .map(move |targets| (shapes.as_slice(), targets))
        })
        .collect();
    let total: usize = work.iter().map(|(_, t)| t.len()).sum();
    out.extend(par_concat(&runs(&work, threads_for(total), |(_, t)| t.len()), |items| {
        let mut out = Vec::new();
        for (shapes, targets) in items {
            for t in *targets {
                for shape in *shapes {
                    run_shape(&ctx, shape, t, &mut out);
                }
            }
        }
        out
    }));
    let mut by_class: FastMap<&'static str, Vec<Resolved>> = FastMap::default();
    for shape in mixed {
        for t in targets_of(ds, source, shape, &subjects, &mut by_class) {
            run_shape(&ctx, shape, &t, &mut out);
        }
    }

    // One result per element and rule. A property shape that several node
    // shapes share — `eq:Switch.retained-cardinality` under both `eq:Switch`
    // and `eq:Breaker` — reaches a breaker through each once `Switch` is
    // expanded to its subclasses, and would report the same finding twice.
    let mut seen = FastSet::default();
    let keep: Vec<bool> = out
        .iter()
        .map(|v| seen.insert((v.object_id.as_str(), v.rule_id.as_str(), v.property.as_str(), v.message.as_str())))
        .collect();
    drop(seen);
    let mut keep = keep.into_iter();
    out.retain(|_| keep.next().unwrap_or(true));
    out
}
