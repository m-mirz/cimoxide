//! Emits the shape table a property-bag family is validated against.
//!
//! The counterpart to `classes_gen`: where that turns classes into data rows,
//! this turns SHACL shapes into them. Both exist for the same reason — a bag
//! family has no generated structs, so there is nothing for code generation to
//! reference.
//!
//! Everything that can be resolved once is resolved here rather than per
//! element: target classes become family-qualified `by_type` keys with abstract
//! classes expanded to concrete descendants, and property paths become the
//! field keys the decoder actually stores. What cannot be resolved is recorded
//! as a skip, because a rule that silently checks nothing is indistinguishable
//! from one that passed.

use std::collections::HashMap;
use std::fmt::Write;

use crate::schema::model::{CimSpecification, CimType};
use crate::schema::shacl::model::{ConstraintInfo, FileResults, ShapeInfo};
use crate::shacl::skip::SkipCollector;

/// Resolves simplified IRIs against one family's schema.
struct Resolver {
    /// `(namespace, local)` → family-qualified name.
    ///
    /// Owned keys: lookups come from a TTL file's prefix map, whose lifetime is
    /// unrelated to the schema's. This runs once per shape at build time, not
    /// per element, so the allocation is not on any hot path.
    by_ns: HashMap<(String, String), String>,
    /// Family-qualified name → the concrete classes an instance of it can be.
    ///
    /// For a concrete class that is itself; for an abstract one it is its
    /// concrete descendants. Three NCP target classes are abstract, and exact
    /// `by_type` lookup on those would check nothing at all.
    concrete_of: HashMap<String, Vec<String>>,
    /// `(namespace, local)` → qualified name across *every* family.
    ///
    /// NCP's association value-type lists name CGMES classes
    /// (`cim17:ACLineSegment`) beside NC ones, and the referenced element can
    /// be either — `CimElement::type_name` answers for both. Resolving those
    /// needs more than one family's schema.
    any_family: HashMap<(String, String), String>,
    /// Concrete descendants, across every family.
    any_concrete: HashMap<String, Vec<String>>,
}

impl Resolver {
    fn new(spec: &CimSpecification, others: &[&CimSpecification]) -> Self {
        let prefix = spec.family.type_prefix;
        let qualified = |t: &CimType| format!("{prefix}{}", t.id);

        let by_ns: HashMap<(String, String), String> = spec
            .types
            .values()
            .map(|t| ((t.namespace.clone(), t.id.clone()), qualified(t)))
            .collect();

        // Ancestor chains, walked once per class.
        let mut concrete_of: HashMap<String, Vec<String>> = HashMap::new();
        for t in spec.types.values() {
            let q = qualified(t);
            if !t.concrete_in.is_empty() {
                // Register against every ancestor, so an abstract target
                // resolves to the concrete classes below it.
                let mut cursor = Some(t);
                let mut guard = 0;
                while let Some(c) = cursor {
                    concrete_of.entry(qualified(c)).or_default().push(q.clone());
                    guard += 1;
                    if guard > 64 {
                        break;
                    }
                    cursor = spec.types.get(&c.super_type);
                }
            } else {
                concrete_of.entry(q).or_default();
            }
        }
        for v in concrete_of.values_mut() {
            v.sort();
            v.dedup();
        }

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

    /// Resolve a simplified IRI to a family-qualified class name.
    ///
    /// Needs the file's prefix map: the prefix alone is not enough, because
    /// NCP binds `cim` to `https://cim.ucaiug.io/ns#` while CGMES binds it to
    /// `http://iec.ch/TC57/CIM100#` and the families overlap on 164 local
    /// names.
    fn class(&self, iri: &str, prefixes: &HashMap<String, String>) -> Option<&String> {
        let (prefix, local) = iri.split_once(':')?;
        let ns = prefixes.get(prefix)?;
        self.by_ns.get(&(ns.clone(), local.to_string()))
    }

    /// The concrete classes an instance of `qualified` can be.
    fn concrete(&self, qualified: &str) -> &[String] {
        self.concrete_of
            .get(qualified)
            .map(Vec::as_slice)
            .unwrap_or(&[])
    }

    /// Every concrete class the listed IRIs allow, across all families.
    ///
    /// Members that resolve nowhere are dropped rather than failing the list:
    /// NCP's value-type lists name CIM16 classes, which no family in this
    /// workspace declares, so nothing could ever decode as one.
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

/// `(namespace, local)` → qualified name, and qualified → concrete descendants,
/// for one family.
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
        let q = qualified(t);
        let mut cursor = Some(t);
        let mut guard = 0;
        while let Some(c) = cursor {
            concrete.entry(qualified(c)).or_default().push(q.clone());
            guard += 1;
            if guard > 64 {
                break;
            }
            cursor = spec.types.get(&c.super_type);
        }
    }
    (by_ns, concrete)
}

/// The field key the decoder stores for a property, which is the local XML
/// element name — the qname with its prefix dropped.
fn field_key(iri: &str) -> &str {
    iri.rsplit_once(':').map_or(iri, |(_, local)| local)
}

fn esc(s: &str) -> String {
    s.replace('\\', "\\\\").replace('"', "\\\"").replace('\n', " ")
}

/// Interns the strings that repeat across shapes.
///
/// SHACL attaches a message, description and severity to every constraint, and
/// the profiles reuse a handful of sentences across thousands of them —
/// "This constraint validates the cardinality of the property (attribute)."
/// alone accounts for a large fraction of the table. Emitting each as a named
/// const takes the generated source from ~7 MB to a fraction of that, which is
/// compile time rather than binary size: rustc already merges identical string
/// literals in rodata.
#[derive(Default)]
struct Pool {
    index: HashMap<String, usize>,
    order: Vec<String>,
    /// Rendered `PropShape` bodies, deduplicated the same way.
    props: HashMap<String, usize>,
    prop_order: Vec<String>,
}

impl Pool {
    /// The const name to reference this string by.
    fn intern(&mut self, s: &str) -> String {
        let escaped = esc(s);
        let next = self.index.len();
        let idx = *self.index.entry(escaped.clone()).or_insert_with(|| {
            next
        });
        if idx == self.order.len() {
            self.order.push(escaped);
        }
        format!("S{idx}")
    }

    /// A pooled list of strings, for the repeated profile and class lists.
    fn intern_list(&mut self, items: &[String]) -> String {
        let refs: Vec<String> = items.iter().map(|i| self.intern(i)).collect();
        format!("&[{}]", refs.join(", "))
    }

    /// Intern a rendered `PropShape` body, returning the const name.
    fn intern_prop(&mut self, body: &str) -> String {
        let next = self.props.len();
        let idx = *self.props.entry(body.to_string()).or_insert(next);
        if idx == self.prop_order.len() {
            self.prop_order.push(body.to_string());
        }
        format!("P{idx}")
    }

    fn render(&self) -> String {
        let mut out = String::new();
        for (i, v) in self.order.iter().enumerate() {
            writeln!(out, "const S{i}: &str = \"{v}\";").unwrap();
        }
        out
    }

    fn render_props(&self) -> String {
        let mut out = String::new();
        for (i, v) in self.prop_order.iter().enumerate() {
            writeln!(out, "const P{i}: PropShape = {v};").unwrap();
        }
        out
    }

    fn distinct_props(&self) -> usize {
        self.prop_order.len()
    }
}

/// A rendered property shape, or the reason it could not be rendered.
enum Rendered {
    Prop(String),
    Skipped,
}

pub struct ShapeStats {
    pub shapes: usize,
    pub props: usize,
    pub constraints: usize,
    pub closed: usize,
}

/// Render the shape table for a bag family.
///
/// `files` are the already-simplified per-file results, `profiles_of` maps a
/// TTL base file name to the profile codes whose manifest imports it.
pub fn render_shapes(
    spec: &CimSpecification,
    others: &[&CimSpecification],
    files: &[FileResults],
    profiles_of: &HashMap<String, Vec<String>>,
    collector: &mut SkipCollector,
) -> (String, ShapeStats) {
    let r = Resolver::new(spec, others);
    let mut stats = ShapeStats { shapes: 0, props: 0, constraints: 0, closed: 0 };
    let mut pool = Pool::default();

    let mut body = String::new();
    for fr in files {
        let profiles = match profiles_of.get(&fr.file_name) {
            Some(p) if !p.is_empty() => p.clone(),
            // Not imported by any manifest, so no profile ever runs it. Better
            // to leave it out than to emit shapes nothing can reach.
            _ => {
                collector.push("", &fr.file_name, "file", "", "not imported by any NC profile manifest");
                continue;
            }
        };
        for shape in &fr.shapes {
            if let Some(rendered) =
                render_shape(shape, fr, &profiles, &r, &mut pool, collector, &mut stats)
            {
                body.push_str(&rendered);
                stats.shapes += 1;
            }
        }
    }

    let mut s = String::new();
    writeln!(s, "// Generated by cimgen — do not edit by hand.").unwrap();
    writeln!(s, "#![allow(clippy::all, dead_code, unused)]").unwrap();
    writeln!(s).unwrap();
    writeln!(
        s,
        "use crate::shapes::{{AltBranch, Check, ClosedShape, Constraint, NodeKind, Path, PropShape, ShapeDef, Target}};"
    )
    .unwrap();
    writeln!(s).unwrap();
    writeln!(s, "// Repeated messages, descriptions and severities, interned.").unwrap();
    s.push_str(&pool.render());
    writeln!(s).unwrap();
    writeln!(
        s,
        "// {} distinct property shapes, referenced {} times.",
        pool.distinct_props(),
        stats.props
    )
    .unwrap();
    s.push_str(&pool.render_props());
    writeln!(s).unwrap();
    writeln!(
        s,
        "/// Every reachable shape of the `{}` family, resolved against its class table.",
        spec.family.id
    )
    .unwrap();
    writeln!(s, "pub static SHAPES: &[ShapeDef] = &[").unwrap();
    s.push_str(&body);
    writeln!(s, "];").unwrap();
    (s, stats)
}

fn render_shape(
    shape: &ShapeInfo,
    fr: &FileResults,
    profiles: &[String],
    r: &Resolver,
    pool: &mut Pool,
    collector: &mut SkipCollector,
    stats: &mut ShapeStats,
) -> Option<String> {
    // Resolve targets first: without one there is nothing to iterate, and the
    // reason a shape is unreachable is worth recording per target.
    let mut targets: Vec<String> = Vec::new();
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
                    targets.push(format!("Target::Class({})", pool.intern_list(concrete)));
                }
                None => {
                    // The cim16:/cim17: targets land here: NCP shapes on CGMES
                    // classes. The decoder builds a typed struct for those and
                    // drops the NC attribute the shape constrains, so checking
                    // them would report absent values that were in the XML.
                    collector.push(&t.value, "", "sh:targetClass", &shape.name,
                        "target class is not in this family's schema (cross-family shape)");
                    continue;
                }
            },
            "targetSubjectsOf" => {
                targets.push(format!(
                    "Target::SubjectsOf({})",
                    pool.intern(field_key(&t.value))
                ));
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

    let mut props: Vec<String> = Vec::new();
    for prop in &shape.properties {
        match render_prop(prop, fr, &class_label, r, pool, collector, stats) {
            Rendered::Prop(p) => props.push(format!("&{}", pool.intern_prop(&p))),
            Rendered::Skipped => {}
        }
    }

    let closed = shape.closed.as_ref().map(|allowed| {
        stats.closed += 1;
        let keys: Vec<String> = allowed.iter().map(|p| field_key(p).to_string()).collect();
        format!(
            "&ClosedShape {{ allowed: {}, rule_id: {}, name: {}, message: {}, description: {}, severity: {} }}",
            pool.intern_list(&keys),
            pool.intern(&shape.id),
            pool.intern(&shape.name),
            pool.intern(&shape.description),
            pool.intern(&shape.description),
            pool.intern("sh:Info"),
        )
    });

    if props.is_empty() && closed.is_none() {
        return None;
    }
    stats.props += props.len();

    let mut s = String::new();
    writeln!(s, "    ShapeDef {{").unwrap();
    writeln!(s, "        targets: &[{}],", targets.join(", ")).unwrap();
    if props.is_empty() {
        writeln!(s, "        props: &[],").unwrap();
    } else {
        writeln!(s, "        props: &[{}],", props.join(", ")).unwrap();
    }
    match closed {
        Some(c) => writeln!(s, "        closed: Some({c}),").unwrap(),
        None => writeln!(s, "        closed: None,").unwrap(),
    }
    writeln!(s, "        profiles: {},", pool.intern_list(profiles)).unwrap();
    writeln!(s, "        file: {},", pool.intern(&fr.file_name)).unwrap();
    writeln!(s, "    }},").unwrap();
    Some(s)
}

fn render_prop(
    prop: &ShapeInfo,
    fr: &FileResults,
    class_label: &str,
    r: &Resolver,
    pool: &mut Pool,
    collector: &mut SkipCollector,
    stats: &mut ShapeStats,
) -> Rendered {
    let (seg, path) = match prop.path.as_slice() {
        [one] => match render_path(one) {
            Some(p) => (one.as_str(), p),
            None => {
                collector.push(class_label, one, "sh:path", &prop.name, "unsupported path form");
                return Rendered::Skipped;
            }
        },
        // `( nc:X.y rdf:type )` — follow the association, then read the
        // referenced element's class. 170 of NCP's 178 chains are this.
        [first, last] if last == "rdf:type" => {
            (first.as_str(), format!("Path::RefType(\"{}\")", esc(field_key(first))))
        }
        // Anything longer, or ending elsewhere, walks through intermediate
        // objects. Guessing at one would be worse than reporting it.
        segs if !segs.is_empty() => {
            collector.push(class_label, &segs.join(" / "), "sh:path", &prop.name,
                "multi-segment property path not supported for bag families");
            return Rendered::Skipped;
        }
        _ => {
            collector.push(class_label, "", "sh:path", &prop.name, "shape has no resolvable path");
            return Rendered::Skipped;
        }
    };
    let is_ref_type = matches!(prop.path.as_slice(), [_, last] if last == "rdf:type");

    // No check that the path names a declared attribute. A property bag reads
    // whatever key the XML carried, declared or not, so a shape whose path the
    // class table happens to be missing still validates real data correctly —
    // `dcterms:spatial` on `dcat:Dataset` is exactly that case, declared in the
    // RDFS and absent from the imported table. Verifying here would reject 43
    // DatasetMetadata shapes whose data is present.
    //
    // The cost is that a genuinely misspelled path silently matches nothing,
    // and a cardinality shape on one would over-report. The shapes ship in the
    // same ENTSO-E release as the RDFS, so that would be a schema bug rather
    // than a resolution bug here.

    let mut checks: Vec<String> = Vec::new();
    for c in &prop.constraints {
        match render_constraint(c, r, pool, &fr.prefixes, is_ref_type) {
            Some(rendered) => checks.push(format!(
                "Check {{ constraint: {}, rule_id: {}, name: {}, message: {}, description: {}, severity: {} }}",
                rendered,
                pool.intern(if c.rule_id.is_empty() { &prop.id } else { &c.rule_id }),
                pool.intern(if c.name.is_empty() { &prop.name } else { &c.name }),
                pool.intern(&c.message),
                pool.intern(if c.description.is_empty() { &prop.description } else { &c.description }),
                pool.intern(if c.severity.is_empty() { "sh:Violation" } else { &c.severity }),
            )),
            None => {
                collector.push(class_label, seg, &c.component, &c.name,
                    "constraint component not supported for bag families");
            }
        }
    }
    if checks.is_empty() {
        return Rendered::Skipped;
    }
    stats.constraints += checks.len();

    Rendered::Prop(format!(
        "PropShape {{ path: {}, checks: &[{}] }}",
        path,
        checks.join(", "),
    ))
}

/// Decode the two encodings `extract_path` uses — `"^iri"` for inverse and
/// `"|a|b"` for alternative — into the IR's [`Path`].
fn render_path(seg: &str) -> Option<String> {
    if let Some(alts) = seg.strip_prefix('|') {
        let branches: Vec<String> = alts
            .split('|')
            .filter(|b| !b.is_empty())
            .map(|b| match b.strip_prefix('^') {
                Some(inv) => format!("AltBranch::Inverse(\"{}\")", esc(field_key(inv))),
                None => format!("AltBranch::Forward(\"{}\")", esc(field_key(b))),
            })
            .collect();
        if branches.is_empty() {
            return None;
        }
        return Some(format!("Path::Alternative(&[{}])", branches.join(", ")));
    }
    if let Some(inv) = seg.strip_prefix('^') {
        return Some(format!("Path::Inverse(\"{}\")", esc(field_key(inv))));
    }
    if seg.is_empty() {
        return None;
    }
    Some(format!("Path::Forward(\"{}\")", esc(field_key(seg))))
}

fn render_constraint(
    c: &ConstraintInfo,
    r: &Resolver,
    pool: &mut Pool,
    prefixes: &HashMap<String, String>,
    is_ref_type: bool,
) -> Option<String> {
    let int = |key: &str| c.payload.get(key).and_then(|v| v.as_int());
    // On a RefType path the values are class IRIs, so sh:in is a class-
    // membership test rather than a literal comparison, and it must resolve
    // across families.
    if is_ref_type {
        return match c.component.as_str() {
            "sh:InConstraintComponent" => {
                let values = c.payload.get("in")?.as_list()?;
                let classes = r.any_classes(values, prefixes);
                if classes.is_empty() {
                    return None;
                }
                Some(format!("Constraint::RefClass({})", pool.intern_list(&classes)))
            }
            "sh:HasValueConstraintComponent" => {
                let v = c.payload.get("hasValue")?.as_str()?;
                let classes = r.any_classes(std::slice::from_ref(&v.to_string()), prefixes);
                if classes.is_empty() {
                    return None;
                }
                Some(format!("Constraint::RefClass({})", pool.intern_list(&classes)))
            }
            // sh:nodeKind sh:IRI on such a shape means "the association value
            // must be an IRI", which is a statement about the field, not about
            // the rdf:type values. Evaluated against the field, as the shape's
            // own message ("The value is not IRI") reads.
            "sh:NodeKindConstraintComponent" => match c.payload.get("nodeKind")?.as_str()? {
                "sh:IRI" => Some("Constraint::NodeKind(NodeKind::Iri)".to_string()),
                _ => None,
            },
            "sh:RequiredConstraintComponent" => Some("Constraint::MinCount(1)".to_string()),
            "sh:MinCountConstraintComponent" => {
                Some(format!("Constraint::MinCount({})", int("minCount")?))
            }
            "sh:MaxCountConstraintComponent" => {
                Some(format!("Constraint::MaxCount({})", int("maxCount")?))
            }
            _ => None,
        };
    }
    match c.component.as_str() {
        "sh:RequiredConstraintComponent" => Some("Constraint::MinCount(1)".to_string()),
        "sh:MinCountConstraintComponent" => Some(format!("Constraint::MinCount({})", int("minCount")?)),
        "sh:MaxCountConstraintComponent" => Some(format!("Constraint::MaxCount({})", int("maxCount")?)),
        "sh:ExactCountConstraintComponent" => {
            let n = int("minCount").or_else(|| int("maxCount"))?;
            Some(format!("Constraint::MinCount({n})"))
        }
        "sh:DatatypeConstraintComponent" => {
            let dt = c.payload.get("datatype")?.as_str()?;
            Some(format!("Constraint::Datatype({})", pool.intern(field_key(dt.trim_end_matches('>')))))
        }
        "sh:NodeKindConstraintComponent" => {
            let kind = match c.payload.get("nodeKind")?.as_str()? {
                "sh:IRI" => "NodeKind::Iri",
                "sh:Literal" => "NodeKind::Literal",
                "sh:BlankNode" => "NodeKind::BlankNode",
                // sh:BlankNodeOrIRI and friends allow more than one shape of
                // node, so there is nothing to assert.
                _ => return None,
            };
            Some(format!("Constraint::NodeKind({kind})"))
        }
        "sh:ClassConstraintComponent" => {
            let cls = c.payload.get("class")?.as_str()?;
            // Resolved through the file's prefix map across every family, for
            // the same reason sh:in is: the referenced element can belong to
            // either, and the prefix alone does not identify a class.
            let classes = r.any_classes(std::slice::from_ref(&cls.to_string()), prefixes);
            if classes.is_empty() {
                return None;
            }
            Some(format!("Constraint::Class({})", pool.intern_list(&classes)))
        }
        "sh:InConstraintComponent" => {
            let values = c.payload.get("in")?.as_list()?;
            let keys: Vec<String> = values.iter().map(|v| enum_fragment(v)).collect();
            Some(format!("Constraint::In({})", pool.intern_list(&keys)))
        }
        "sh:HasValueConstraintComponent" => {
            let v = c.payload.get("hasValue")?.as_str()?;
            Some(format!("Constraint::HasValue({})", pool.intern(&enum_fragment(v))))
        }
        "sh:MaxLengthConstraintComponent" => Some(format!("Constraint::MaxLength({})", int("maxLength")?)),
        "sh:MinLengthConstraintComponent" => Some(format!("Constraint::MinLength({})", int("minLength")?)),
        _ => None,
    }
}

/// An enum or IRI value as the decoder stores it.
///
/// `rdf:resource` values keep only the fragment (see the decoder's
/// `strip_fragment`), so `cim:Kind.value` and `<http://…#Kind.value>` must both
/// compare as `Kind.value`.
fn enum_fragment(v: &str) -> String {
    let v = v.trim_start_matches('<').trim_end_matches('>');
    match v.rsplit_once('#') {
        Some((_, frag)) => frag.to_string(),
        None => field_key(v).to_string(),
    }
}
