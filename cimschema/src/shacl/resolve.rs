//! Resolves parsed SHACL shapes against a family's schema.
//!
//! The output is what a property-bag validator needs and nothing more: target
//! classes as family-qualified `by_type` keys, paths as the field keys the
//! decoder stores, and class lists already expanded to concrete descendants.
//!
//! This lives here rather than in `cimgen` so the build-time generator and the
//! runtime shape loader in `cimvalidation` resolve *identically* — the same
//! reason the parser moved out of `cimgen`. The alternative, two
//! implementations plus a test comparing them, was what the NC class table
//! ended up doing; sharing the code removes the drift instead of detecting it.
//!
//! The types here are owned. `cimvalidation`'s mirror of them is `&'static`,
//! because the element trait and the violation types demand it, and converting
//! between the two is mechanical and total — an omission is a compile error.

use std::collections::HashMap;

use crate::family::Family;
use crate::model::{CimSpecification, CimType};
use crate::shacl::model::{ConstraintInfo, FileResults, ShapeInfo, LITERALS, NEGATED, UNSUPPORTED};
use crate::shacl::skip::SkipCollector;

// ---------------------------------------------------------------------------
// Resolved model
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AltBranch {
    Forward(String),
    Inverse(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Path {
    Forward(String),
    Inverse(String),
    Alternative(Vec<AltBranch>),
    /// A sequence path, e.g. `( ^cim:Terminal.ConductingEquipment
    /// cim:Terminal.phases )`, or `( nc:X.y rdf:type )` — 170 of NCP's 178
    /// chains are that last form, follow the association and read the class.
    Chain(Vec<Step>),
}

/// One step of a [`Path::Chain`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Step {
    Forward(String),
    Inverse(String),
    /// `rdf:type` — the classes of the elements reached so far. Last only.
    Type,
}

impl Path {
    /// Does the path end in `rdf:type`, so that its values are class names?
    pub fn yields_types(&self) -> bool {
        match self {
            Path::Chain(steps) => matches!(steps.last(), Some(Step::Type)),
            _ => false,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NodeKind {
    Iri,
    Literal,
    BlankNode,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Constraint {
    MinCount(u32),
    MaxCount(u32),
    Datatype(String),
    NodeKind(NodeKind),
    Class(Vec<String>),
    In(Vec<String>),
    HasValue(String),
    MaxLength(u32),
    MinLength(u32),
    /// The referenced element's class, from `sh:in` on a path ending in
    /// `rdf:type`.
    RefClass(Vec<String>),
    MinInclusive(f64),
    MaxInclusive(f64),
    MinExclusive(f64),
    MaxExclusive(f64),
    /// `sh:lessThan` — the field key of the property this one must be below.
    LessThan(String),
    LessThanOrEquals(String),
    /// `sh:not [ sh:class X ]` — the referenced element must *not* be an
    /// instance of these (already expanded) classes.
    NotClass(Vec<String>),
    /// `sh:qualifiedValueShape [ sh:in (..) ]` with `sh:qualifiedMinCount` —
    /// at least `min` values must be among `allowed`.
    QualifiedIn { allowed: Vec<String>, min: u32 },
    /// `sh:length` — exactly this many characters.
    Length(u32),
}

/// One branch of a [`Logic`]: it conforms when none of its checks fail, or —
/// for `[ sh:not X ]` — when at least one does.
#[derive(Debug, Clone)]
pub struct Branch {
    pub props: Vec<PropShape>,
    pub negate: bool,
}

/// How a node-level `sh:and` / `sh:or` / `sh:xone` combines its branches.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogicOp {
    And,
    Or,
    Xone,
}

/// A node-level `sh:and` / `sh:or` / `sh:xone` over anonymous property shapes.
///
/// A branch conforms when none of its checks fail. The checks inside a branch
/// carry no report text of their own: the shape reports once, with the node
/// shape's name and message, when the combination fails.
#[derive(Debug, Clone)]
pub struct Logic {
    pub op: LogicOp,
    pub branches: Vec<Branch>,
    pub rule_id: String,
    pub name: String,
    pub message: String,
    pub description: String,
    pub severity: String,
}

#[derive(Debug, Clone)]
pub struct Check {
    pub constraint: Constraint,
    pub rule_id: String,
    pub name: String,
    pub message: String,
    pub description: String,
    pub severity: String,
}

#[derive(Debug, Clone)]
pub struct PropShape {
    pub path: Path,
    pub checks: Vec<Check>,
}

#[derive(Debug, Clone)]
pub enum Target {
    Class(Vec<String>),
    SubjectsOf(String),
}

#[derive(Debug, Clone)]
pub struct ClosedShape {
    pub allowed: Vec<String>,
    pub rule_id: String,
    pub name: String,
    pub message: String,
    pub description: String,
    pub severity: String,
}

#[derive(Debug, Clone)]
pub struct ShapeDef {
    pub targets: Vec<Target>,
    pub props: Vec<PropShape>,
    pub closed: Option<ClosedShape>,
    pub logic: Vec<Logic>,
    pub profiles: Vec<String>,
    pub file: String,
}

#[derive(Debug, Default)]
pub struct Stats {
    pub shapes: usize,
    pub props: usize,
    pub checks: usize,
    pub closed: usize,
    pub logic: usize,
}

// ---------------------------------------------------------------------------
// Resolver
// ---------------------------------------------------------------------------

/// Resolves simplified IRIs against one family's schema, and against every
/// other family for the cross-family class lists.
pub struct Resolver {
    /// `(namespace, local)` → family-qualified name, this family only.
    ///
    /// Owned keys: lookups come from a TTL file's prefix map, whose lifetime is
    /// unrelated to the schema's. This runs once per shape when the table is
    /// built, never per element.
    by_ns: HashMap<(String, String), String>,
    concrete_of: HashMap<String, Vec<String>>,
    /// The same, across every family.
    ///
    /// NCP's association value-type lists name CGMES classes
    /// (`cim17:ACLineSegment`) beside NC ones, and the referenced element can
    /// be either — `CimElement::type_name` answers for both.
    any_family: HashMap<(String, String), String>,
    any_concrete: HashMap<String, Vec<String>>,
}

fn class_maps(
    spec: &CimSpecification,
) -> (HashMap<(String, String), String>, HashMap<String, Vec<String>>) {
    let prefix = spec.family.type_prefix;
    let qualified = |t: &CimType| format!("{prefix}{}", t.id);
    let by_ns = spec
        .types
        .values()
        .map(|t| ((t.namespace.clone(), t.id.clone()), qualified(t)))
        .collect();

    let mut concrete: HashMap<String, Vec<String>> = HashMap::new();
    for t in spec.types.values() {
        if t.concrete_in.is_empty() {
            concrete.entry(qualified(t)).or_default();
            continue;
        }
        // Register the concrete class against every ancestor, so an abstract
        // target resolves to what can actually appear below it.
        let q = qualified(t);
        let mut cursor = Some(t);
        let mut guard = 0;
        while let Some(c) = cursor {
            concrete.entry(qualified(c)).or_default().push(q.clone());
            // A malformed schema could make this cyclic; the import stage does
            // not guarantee otherwise.
            guard += 1;
            if guard > 64 {
                break;
            }
            cursor = spec.types.get(&c.super_type);
        }
    }
    for v in concrete.values_mut() {
        v.sort();
        v.dedup();
    }
    (by_ns, concrete)
}

impl Resolver {
    pub fn new(spec: &CimSpecification, others: &[&CimSpecification]) -> Self {
        let (by_ns, concrete_of) = class_maps(spec);
        let mut any_family = by_ns.clone();
        let mut any_concrete = concrete_of.clone();
        for other in others {
            let (b, c) = class_maps(other);
            any_family.extend(b);
            for (k, v) in c {
                any_concrete.entry(k).or_default().extend(v);
            }
        }
        for v in any_concrete.values_mut() {
            v.sort();
            v.dedup();
        }
        Self { by_ns, concrete_of, any_family, any_concrete }
    }

    /// Resolve a simplified IRI to this family's qualified class name.
    ///
    /// Needs the file's prefix map: the prefix alone does not identify a class,
    /// because NCP binds `cim` to `https://cim.ucaiug.io/ns#` while CGMES binds
    /// it to `http://iec.ch/TC57/CIM100#` and the families overlap on 164 local
    /// names.
    fn class(&self, iri: &str, prefixes: &HashMap<String, String>) -> Option<&String> {
        let (prefix, local) = iri.split_once(':')?;
        let ns = prefixes.get(prefix)?;
        self.by_ns.get(&(ns.clone(), local.to_string()))
    }

    fn concrete(&self, qualified: &str) -> &[String] {
        self.concrete_of.get(qualified).map_or(&[], Vec::as_slice)
    }

    /// Every concrete class the listed IRIs allow, across all families.
    ///
    /// Members that resolve nowhere are dropped rather than failing the list:
    /// NCP's value-type lists name CIM16 classes, which no family here
    /// declares, so nothing could ever decode as one.
    fn any_classes(&self, iris: &[String], prefixes: &HashMap<String, String>) -> Vec<String> {
        let mut out: Vec<String> = Vec::new();
        for iri in iris {
            let Some((prefix, local)) = iri.split_once(':') else { continue };
            let Some(ns) = prefixes.get(prefix) else { continue };
            if let Some(q) = self.any_family.get(&(ns.clone(), local.to_string())) {
                match self.any_concrete.get(q) {
                    Some(c) if !c.is_empty() => out.extend(c.iter().cloned()),
                    _ => out.push(q.clone()),
                }
            }
        }
        out.sort();
        out.dedup();
        out
    }
}

// ---------------------------------------------------------------------------
// Resolution
// ---------------------------------------------------------------------------

/// The field key the decoder stores for a property: the local XML element
/// name, which is the qname with its prefix dropped.
fn field_key(iri: &str) -> &str {
    iri.rsplit_once(':').map_or(iri, |(_, local)| local)
}

/// An `sh:in` / `sh:hasValue` member as the decoder would store it: a string
/// literal verbatim, an IRI as [`iri_key`] gives it.
fn value_key(c: &ConstraintInfo, v: &str, prefixes: &HashMap<String, String>) -> String {
    let literal = c
        .payload
        .get(LITERALS)
        .and_then(|l| l.as_list())
        .is_some_and(|l| l.iter().any(|x| x == v));
    if literal { v.to_string() } else { iri_key(v, prefixes) }
}

/// An IRI the way the decoder stores an `rdf:resource` (its
/// `strip_fragment`): the part after the last `#`, or the whole IRI when it
/// has none.
///
/// A prefixed name is expanded through the file's prefixes first, so
/// `cim:Kind.value` and `<http://…#Kind.value>` both come out as
/// `Kind.value`, while `<https://ap.cim4.eu/Contingency/2.3>` — and a name in
/// a namespace that ends in `/` — stays whole. An unknown prefix falls back to
/// the local name.
fn iri_key(v: &str, prefixes: &HashMap<String, String>) -> String {
    let full = match v.strip_prefix('<') {
        Some(inner) => inner.trim_end_matches('>').to_string(),
        None => match v.split_once(':') {
            Some((prefix, local)) => match prefixes.get(prefix) {
                Some(ns) => format!("{ns}{local}"),
                None => return local.to_string(),
            },
            None => v.to_string(),
        },
    };
    match full.rfind('#') {
        Some(i) => full[i + 1..].to_string(),
        None => full,
    }
}

/// Resolve every shape in every file.
///
/// `profiles_of` maps a TTL base file name to the profile codes whose manifest
/// imports it; a file no manifest imports is left out, since no profile could
/// ever run it.
pub fn resolve_shapes(
    spec: &CimSpecification,
    others: &[&CimSpecification],
    files: &[FileResults],
    profiles_of: &HashMap<String, Vec<String>>,
    collector: &mut SkipCollector,
) -> (Vec<ShapeDef>, Stats) {
    let r = Resolver::new(spec, others);
    let mut stats = Stats::default();
    let mut out = Vec::new();

    for fr in files {
        let profiles = match profiles_of.get(&fr.file_name) {
            Some(p) if !p.is_empty() => p.clone(),
            _ => {
                collector.push("", &fr.file_name, "file", "",
                    "not imported by any profile manifest");
                continue;
            }
        };
        for shape in &fr.shapes {
            if let Some(def) = resolve_shape(shape, fr, &profiles, &r, collector, &mut stats) {
                stats.shapes += 1;
                out.push(def);
            }
        }
    }
    (out, stats)
}

fn resolve_shape(
    shape: &ShapeInfo,
    fr: &FileResults,
    profiles: &[String],
    r: &Resolver,
    collector: &mut SkipCollector,
    stats: &mut Stats,
) -> Option<ShapeDef> {
    let mut targets: Vec<Target> = Vec::new();
    let mut target_classes: Vec<String> = Vec::new();

    for t in &shape.targets {
        match t.kind.as_str() {
            "targetClass" | "targetNode" => match r.class(&t.value, &fr.prefixes) {
                Some(q) => {
                    let concrete = r.concrete(q);
                    if concrete.is_empty() {
                        collector.push(&t.value, "", "sh:targetClass", &shape.name,
                            "class has no concrete subclass in this family");
                        continue;
                    }
                    target_classes.push(q.clone());
                    targets.push(Target::Class(concrete.to_vec()));
                }
                None => {
                    // The cim16:/cim17: targets land here: NC shapes on CGMES
                    // classes. The decoder builds a typed struct for those and
                    // drops the NC attribute the shape constrains, so checking
                    // them would report absent values that were in the XML.
                    collector.push(&t.value, "", "sh:targetClass", &shape.name,
                        "target class is not in this family's schema (cross-family shape)");
                    continue;
                }
            },
            "targetSubjectsOf" => {
                targets.push(Target::SubjectsOf(field_key(&t.value).to_string()));
            }
            other => {
                collector.push(&t.value, "", other, &shape.name,
                    "unsupported SHACL target mechanism");
            }
        }
    }
    if targets.is_empty() {
        return None;
    }

    let class_label = target_classes.first().cloned().unwrap_or_default();

    let mut props: Vec<PropShape> = Vec::new();
    for prop in &shape.properties {
        if let Some(p) = resolve_prop(prop, fr, &class_label, r, collector, stats) {
            props.push(p);
        }
    }

    let closed = shape.closed.as_ref().map(|allowed| {
        stats.closed += 1;
        ClosedShape {
            allowed: allowed.iter().map(|p| field_key(p).to_string()).collect(),
            rule_id: shape.id.clone(),
            name: shape.name.clone(),
            message: shape.description.clone(),
            description: shape.description.clone(),
            // sh:closed shapes are advisory throughout NCP.
            severity: "sh:Info".to_string(),
        }
    });

    let mut logic: Vec<Logic> = Vec::new();
    for c in &shape.constraints {
        if let Some(l) = resolve_logic(c, shape, fr, &class_label, r, collector) {
            logic.push(l);
        }
    }
    stats.logic += logic.len();

    if props.is_empty() && closed.is_none() && logic.is_empty() {
        return None;
    }
    stats.props += props.len();

    Some(ShapeDef {
        targets,
        props,
        closed,
        logic,
        profiles: profiles.to_vec(),
        file: fr.file_name.clone(),
    })
}

fn resolve_prop(
    prop: &ShapeInfo,
    fr: &FileResults,
    class_label: &str,
    r: &Resolver,
    collector: &mut SkipCollector,
    stats: &mut Stats,
) -> Option<PropShape> {
    let seg = prop.path.first().map_or("", String::as_str);
    let path = match resolve_path(&prop.path) {
        Ok(p) => p,
        Err(reason) => {
            collector.push(class_label, &prop.path.join(" / "), "sh:path", &prop.name, reason);
            return None;
        }
    };

    // No check that the path names a declared attribute. A property bag reads
    // whatever key the XML carried, declared or not, so a shape whose path the
    // class table happens to lack still validates real data —
    // `dcterms:spatial` on `dcat:Dataset` is exactly that, declared in the RDFS
    // and absent from the imported table. Verifying here rejected 43
    // DatasetMetadata shapes whose data is present.
    //
    // The cost is that a misspelled path silently matches nothing, and a
    // cardinality shape on one would over-report. The shapes ship in the same
    // ENTSO-E release as the RDFS, so that would be a schema bug.

    let is_ref_type = path.yields_types();
    let mut checks: Vec<Check> = Vec::new();
    for c in &prop.constraints {
        match resolve_constraint(c, r, &fr.prefixes, is_ref_type) {
            Some(constraint) => checks.push(Check {
                constraint,
                rule_id: if c.rule_id.is_empty() { prop.id.clone() } else { c.rule_id.clone() },
                name: if c.name.is_empty() { prop.name.clone() } else { c.name.clone() },
                message: c.message.clone(),
                description: if c.description.is_empty() {
                    prop.description.clone()
                } else {
                    c.description.clone()
                },
                severity: if c.severity.is_empty() {
                    "sh:Violation".to_string()
                } else {
                    c.severity.clone()
                },
            }),
            None => {
                collector.push(class_label, seg, &c.component, &c.name,
                    "constraint component not supported for bag families");
            }
        }
    }
    if checks.is_empty() {
        return None;
    }
    stats.checks += checks.len();
    Some(PropShape { path, checks })
}

/// A property path as `extract_path` encodes it: one segment per step, each
/// `"iri"`, `"^iri"` (inverse) or `"|a|b"` (alternative).
fn resolve_path(segs: &[String]) -> Result<Path, &'static str> {
    match segs {
        [] => Err("shape has no resolvable path"),
        [one] => decode_path(one).ok_or("unsupported path form"),
        _ => {
            let mut steps = Vec::with_capacity(segs.len());
            for (i, seg) in segs.iter().enumerate() {
                let last = i + 1 == segs.len();
                steps.push(match seg.as_str() {
                    "rdf:type" if last => Step::Type,
                    "rdf:type" => return Err("rdf:type before the end of a sequence path"),
                    s if s.starts_with('|') => {
                        return Err("alternative path inside a sequence path")
                    }
                    s => match s.strip_prefix('^') {
                        Some(inv) => Step::Inverse(field_key(inv).to_string()),
                        None => Step::Forward(field_key(s).to_string()),
                    },
                });
            }
            Ok(Path::Chain(steps))
        }
    }
}

/// Resolve a node-level `sh:and` / `sh:or` / `sh:xone`.
///
/// All or nothing: a branch that drops one of its constraints conforms more
/// often than the schema says, which changes what the combination means rather
/// than just checking less. So any part that does not resolve skips the whole
/// combination, as does anything the importer saw in a branch but could not
/// represent.
fn resolve_logic(
    c: &ConstraintInfo,
    shape: &ShapeInfo,
    fr: &FileResults,
    class_label: &str,
    r: &Resolver,
    collector: &mut SkipCollector,
) -> Option<Logic> {
    let op = match c.component.as_str() {
        "sh:AndConstraintComponent" => LogicOp::And,
        "sh:OrConstraintComponent" => LogicOp::Or,
        "sh:XoneConstraintComponent" => LogicOp::Xone,
        _ => return None,
    };
    let mut skip = |reason: &str| {
        collector.push(class_label, "", &c.component, &shape.name, reason);
        None
    };
    if c.payload.get(UNSUPPORTED).and_then(|v| v.as_list()).is_some_and(|u| !u.is_empty()) {
        return skip("logical branch uses a construct the importer does not represent");
    }
    let Some(raw) = c.payload.get("branches").and_then(|v| v.as_shapes()) else {
        return skip("logical constraint without branches");
    };

    let negated: Vec<usize> = c
        .payload
        .get(NEGATED)
        .and_then(|v| v.as_list())
        .map(|l| l.iter().filter_map(|i| i.parse().ok()).collect())
        .unwrap_or_default();
    let mut branches: Vec<Branch> = Vec::new();
    for (bi, raw_branch) in raw.iter().enumerate() {
        // One branch may constrain several paths; group by path.
        let mut props: Vec<PropShape> = Vec::new();
        for bc in raw_branch {
            let Ok(path) = resolve_path(&bc.path) else {
                return skip("logical branch path does not resolve");
            };
            // minCount 0 is vacuous, and is how EnergySourcePQ spells "absent"
            // next to its maxCount 0.
            if bc.component == "sh:MinCountConstraintComponent"
                && bc.payload.get("minCount").and_then(|v| v.as_int()) == Some(0)
            {
                continue;
            }
            let Some(constraint) = resolve_constraint(bc, r, &fr.prefixes, path.yields_types()) else {
                return skip("logical branch constraint not supported for bag families");
            };
            let check = Check {
                constraint,
                rule_id: String::new(),
                name: String::new(),
                message: String::new(),
                description: String::new(),
                severity: String::new(),
            };
            match props.iter_mut().find(|p| p.path == path) {
                Some(p) => p.checks.push(check),
                None => props.push(PropShape { path, checks: vec![check] }),
            }
        }
        branches.push(Branch { props, negate: negated.contains(&bi) });
    }
    Some(Logic {
        op,
        branches,
        rule_id: c.rule_id.clone(),
        name: c.name.clone(),
        message: c.message.clone(),
        description: c.description.clone(),
        severity: if c.severity.is_empty() { "sh:Violation".to_string() } else { c.severity.clone() },
    })
}

/// Decode the two encodings `extract_path` uses — `"^iri"` for inverse and
/// `"|a|b"` for alternative.
fn decode_path(seg: &str) -> Option<Path> {
    if let Some(alts) = seg.strip_prefix('|') {
        let branches: Vec<AltBranch> = alts
            .split('|')
            .filter(|b| !b.is_empty())
            .map(|b| match b.strip_prefix('^') {
                Some(inv) => AltBranch::Inverse(field_key(inv).to_string()),
                None => AltBranch::Forward(field_key(b).to_string()),
            })
            .collect();
        if branches.is_empty() {
            return None;
        }
        return Some(Path::Alternative(branches));
    }
    if let Some(inv) = seg.strip_prefix('^') {
        return Some(Path::Inverse(field_key(inv).to_string()));
    }
    if seg.is_empty() {
        return None;
    }
    Some(Path::Forward(field_key(seg).to_string()))
}

fn resolve_constraint(
    c: &ConstraintInfo,
    r: &Resolver,
    prefixes: &HashMap<String, String>,
    is_ref_type: bool,
) -> Option<Constraint> {
    let int = |key: &str| c.payload.get(key).and_then(|v| v.as_int());
    let float = |key: &str| c.payload.get(key).and_then(|v| v.as_float());

    // On a path ending in rdf:type the values are class IRIs, so sh:in is a class
    // membership test rather than a literal comparison, and it has to resolve
    // across families.
    if is_ref_type {
        return match c.component.as_str() {
            "sh:InConstraintComponent" => {
                let values = c.payload.get("in")?.as_list()?;
                let classes = r.any_classes(values, prefixes);
                (!classes.is_empty()).then_some(Constraint::RefClass(classes))
            }
            "sh:HasValueConstraintComponent" => {
                let v = c.payload.get("hasValue")?.as_str()?.to_string();
                let classes = r.any_classes(std::slice::from_ref(&v), prefixes);
                (!classes.is_empty()).then_some(Constraint::RefClass(classes))
            }
            // sh:nodeKind sh:IRI on such a shape means "the association value
            // must be an IRI", a statement about the field rather than about
            // the rdf:type values — which is how the shape's own message
            // ("The value is not IRI") reads.
            "sh:NodeKindConstraintComponent" => match c.payload.get("nodeKind")?.as_str()? {
                "sh:IRI" => Some(Constraint::NodeKind(NodeKind::Iri)),
                _ => None,
            },
            "sh:RequiredConstraintComponent" => Some(Constraint::MinCount(1)),
            "sh:MinCountConstraintComponent" => Some(Constraint::MinCount(int("minCount")? as u32)),
            "sh:MaxCountConstraintComponent" => Some(Constraint::MaxCount(int("maxCount")? as u32)),
            _ => None,
        };
    }

    match c.component.as_str() {
        "sh:RequiredConstraintComponent" => Some(Constraint::MinCount(1)),
        "sh:MinCountConstraintComponent" => Some(Constraint::MinCount(int("minCount")? as u32)),
        "sh:MaxCountConstraintComponent" => Some(Constraint::MaxCount(int("maxCount")? as u32)),
        "sh:ExactCountConstraintComponent" => {
            let n = int("minCount").or_else(|| int("maxCount"))?;
            Some(Constraint::MinCount(n as u32))
        }
        "sh:DatatypeConstraintComponent" => {
            let dt = c.payload.get("datatype")?.as_str()?;
            Some(Constraint::Datatype(field_key(dt.trim_end_matches('>')).to_string()))
        }
        "sh:NodeKindConstraintComponent" => {
            let kind = match c.payload.get("nodeKind")?.as_str()? {
                "sh:IRI" => NodeKind::Iri,
                "sh:Literal" => NodeKind::Literal,
                "sh:BlankNode" => NodeKind::BlankNode,
                // sh:BlankNodeOrIRI and friends allow more than one shape of
                // node, so there is nothing to assert.
                _ => return None,
            };
            Some(Constraint::NodeKind(kind))
        }
        "sh:ClassConstraintComponent" => {
            let cls = c.payload.get("class")?.as_str()?.to_string();
            let classes = r.any_classes(std::slice::from_ref(&cls), prefixes);
            (!classes.is_empty()).then_some(Constraint::Class(classes))
        }
        // `sh:or ( [ sh:class A ] [ sh:class B ] )` on one property: each value
        // must be an instance of one of them, which is `sh:class` over the
        // union.
        "sh:OrClassConstraintComponent" => {
            let classes = r.any_classes(c.payload.get("classes")?.as_list()?, prefixes);
            (!classes.is_empty()).then_some(Constraint::Class(classes))
        }
        // Only the one form the profiles use: a qualified shape that is a
        // value list. Anything else in it is left unresolved.
        "sh:QualifiedMinCountConstraintComponent" => {
            let min = int("qualifiedMinCount")? as u32;
            let inner = c.payload.get("shape")?.as_shapes()?.first()?;
            let [ic] = inner.as_slice() else { return None };
            let allowed: Vec<String> = match ic.component.as_str() {
                "sh:InConstraintComponent" => {
                    ic.payload.get("in")?.as_list()?.iter().map(|v| value_key(ic, v, prefixes)).collect()
                }
                "sh:HasValueConstraintComponent" => {
                    vec![value_key(ic, ic.payload.get("hasValue")?.as_str()?, prefixes)]
                }
                _ => return None,
            };
            Some(Constraint::QualifiedIn { allowed, min })
        }
        "sh:NotClassConstraintComponent" => {
            let cls = c.payload.get("class")?.as_str()?.to_string();
            let classes = r.any_classes(std::slice::from_ref(&cls), prefixes);
            (!classes.is_empty()).then_some(Constraint::NotClass(classes))
        }
        "sh:InConstraintComponent" => {
            let values = c.payload.get("in")?.as_list()?;
            Some(Constraint::In(values.iter().map(|v| value_key(c, v, prefixes)).collect()))
        }
        "sh:HasValueConstraintComponent" => {
            let v = c.payload.get("hasValue")?.as_str()?;
            Some(Constraint::HasValue(value_key(c, v, prefixes)))
        }
        "sh:MaxLengthConstraintComponent" => Some(Constraint::MaxLength(int("maxLength")? as u32)),
        "sh:MinLengthConstraintComponent" => Some(Constraint::MinLength(int("minLength")? as u32)),
        "sh:LengthConstraintComponent" => Some(Constraint::Length(int("length")? as u32)),
        "sh:MinInclusiveConstraintComponent" => Some(Constraint::MinInclusive(float("minInclusive")?)),
        "sh:MaxInclusiveConstraintComponent" => Some(Constraint::MaxInclusive(float("maxInclusive")?)),
        "sh:MinExclusiveConstraintComponent" => Some(Constraint::MinExclusive(float("minExclusive")?)),
        "sh:MaxExclusiveConstraintComponent" => Some(Constraint::MaxExclusive(float("maxExclusive")?)),
        // The other property is read from the same element, so it resolves to
        // a field key exactly as the path does.
        "sh:LessThanConstraintComponent" => {
            let other = c.payload.get("lessThan")?.as_str()?;
            Some(Constraint::LessThan(field_key(other).to_string()))
        }
        "sh:LessThanOrEqualsConstraintComponent" => {
            let other = c.payload.get("lessThanOrEquals")?.as_str()?;
            Some(Constraint::LessThanOrEquals(field_key(other).to_string()))
        }
        _ => None,
    }
}

// ---------------------------------------------------------------------------
// End-to-end
// ---------------------------------------------------------------------------

/// Everything a bag family's validator needs, read from a SHACL directory.
#[derive(Debug)]
pub struct ShapeTable {
    pub shapes: Vec<ShapeDef>,
    /// Profile IRI → short code, for `dcterms:conformsTo`.
    pub profile_iris: Vec<(String, String)>,
    /// Every profile code, sorted.
    pub profiles: Vec<String>,
    pub stats: Stats,
}

/// Read a family's SHACL directory into a resolved shape table.
///
/// `dir` is the directory holding the constraint files; the profile manifests
/// are expected in `dir/Validation` and the profile descriptors in
/// `dir/../PROF`, which is how the ENTSO-E library lays them out.
pub fn load_shape_table(
    family: &'static Family,
    spec: &CimSpecification,
    others: &[&CimSpecification],
    dir: &std::path::Path,
    collector: &mut SkipCollector,
) -> Result<ShapeTable, Box<dyn std::error::Error>> {
    let mut ttl_paths: Vec<std::path::PathBuf> = std::fs::read_dir(dir)?
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .filter(|p| p.extension().and_then(|x| x.to_str()) == Some("ttl"))
        .collect();
    ttl_paths.sort();
    if ttl_paths.is_empty() {
        return Err(format!("no SHACL .ttl files in {}", dir.display()).into());
    }

    let profiles_of: HashMap<String, Vec<String>> = match family.shacl_manifest {
        Some(manifest) => {
            let mut m: HashMap<String, Vec<String>> = HashMap::new();
            for (stem, tag) in manifest {
                m.entry((*stem).to_string()).or_default().push((*tag).to_string());
            }
            m
        }
        None => profile_files(&dir.join("Validation"))?,
    };

    let mut files: Vec<FileResults> = Vec::new();
    for path in &ttl_paths {
        match crate::shacl::ttl_import::import_ttl_file(path) {
            Ok(fr) => files.push(fr),
            Err(e) => eprintln!("warning: skipping {}: {e}", path.display()),
        }
    }
    // Simplification drops constraints too (deactivated shapes, vacuous
    // minCount=0), and those belong in the same accounting as the resolution
    // skips — an unreported drop reads like a rule that passed.
    for (_, entries) in crate::shacl::simplify::simplify(&mut files) {
        for e in entries {
            collector.push(
                e.class_names.first().map(String::as_str).unwrap_or(""),
                &e.prop,
                &e.component,
                &e.name,
                &e.reason,
            );
        }
    }

    let (shapes, stats) = resolve_shapes(spec, others, &files, &profiles_of, collector);

    // A family with a written-out manifest detects its profiles another way
    // (CGMES: the `md:Model.profile` header), so it has no index to load.
    let index = if family.shacl_manifest.is_some() {
        Vec::new()
    } else {
        let prof_dir = dir.parent().map_or_else(|| dir.join("PROF"), |p| p.join("PROF"));
        crate::import::import_profile_index(&prof_dir)?
    };
    let mut profile_iris: Vec<(String, String)> = index
        .iter()
        .flat_map(|p| p.iris.iter().map(|i| (i.clone(), p.keyword.clone())))
        .collect();
    profile_iris.sort();
    let mut profiles: Vec<String> = index.iter().map(|p| p.keyword.clone()).collect();
    profiles.sort();

    Ok(ShapeTable { shapes, profile_iris, profiles, stats })
}

/// TTL base file name → the profile codes whose manifest imports it.
pub fn profile_files(
    dir: &std::path::Path,
) -> Result<HashMap<String, Vec<String>>, Box<dyn std::error::Error>> {
    let mut map: HashMap<String, Vec<String>> = HashMap::new();
    let mut paths: Vec<std::path::PathBuf> = std::fs::read_dir(dir)?
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .filter(|p| p.extension().and_then(|x| x.to_str()) == Some("ttl"))
        .collect();
    paths.sort();
    for path in &paths {
        let m = crate::shacl::ttl_import::import_manifest(path)?;
        // The combined "ALL" manifest imports every file; recording it would
        // put a redundant profile code on every shape.
        if m.profile == "ALL" {
            continue;
        }
        for file in m.imports {
            map.entry(file).or_default().push(m.profile.clone());
        }
    }
    for v in map.values_mut() {
        v.sort();
        v.dedup();
    }
    Ok(map)
}
