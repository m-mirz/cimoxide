//! Emits the shape table a property-bag family is validated against.
//!
//! The counterpart to `classes_gen`: where that turns classes into data rows,
//! this turns SHACL shapes into them. Both exist for the same reason — a bag
//! family has no generated structs, so there is nothing for code generation to
//! reference.
//!
//! Resolution is **not** here. It lives in `cimschema::shacl::resolve`, so the
//! runtime loader in `cimvalidation` resolves identically rather than
//! reimplementing it. This module only renders the resolved model as Rust.

use std::collections::HashMap;
use std::fmt::Write;

use cimschema::shacl::resolve::{
    AltBranch, Check, ClosedShape, Constraint, Logic, LogicOp, NodeKind, Path, PropShape,
    ShapeDef, Step, Target,
};

/// Escape for a Rust string literal. Line breaks are kept as `\n` rather than
/// flattened: the runtime loader interns the text as written, and the two
/// tables must agree (`tests/dynamic_shapes.rs`).
fn esc(s: &str) -> String {
    s.replace('\\', "\\\\").replace('"', "\\\"").replace('\n', "\\n").replace('\r', "\\r")
}

/// Interns the strings that repeat across shapes, and whole property shapes.
///
/// SHACL attaches a message, description and severity to every constraint, and
/// the profiles reuse a handful of sentences across thousands of them. NCP also
/// defines ~4,000 distinct property shapes and references them ~10,000 times,
/// because a shape like `IdentifiedObject.mRID-cardinality` applies to hundreds
/// of classes. Emitting each once takes the generated source from 7.3 MB to
/// 2.6 MB — compile time rather than binary size, since rustc already merges
/// identical string literals in rodata.
#[derive(Default)]
struct Pool {
    index: HashMap<String, usize>,
    order: Vec<String>,
    props: HashMap<String, usize>,
    prop_order: Vec<String>,
}

impl Pool {
    fn intern(&mut self, s: &str) -> String {
        let escaped = esc(s);
        let next = self.index.len();
        let idx = *self.index.entry(escaped.clone()).or_insert(next);
        if idx == self.order.len() {
            self.order.push(escaped);
        }
        format!("S{idx}")
    }

    fn intern_list<S: AsRef<str>>(&mut self, items: &[S]) -> String {
        let refs: Vec<String> = items.iter().map(|i| self.intern(i.as_ref())).collect();
        format!("&[{}]", refs.join(", "))
    }

    fn intern_prop(&mut self, body: &str) -> String {
        let next = self.props.len();
        let idx = *self.props.entry(body.to_string()).or_insert(next);
        if idx == self.prop_order.len() {
            self.prop_order.push(body.to_string());
        }
        format!("P{idx}")
    }

    fn render_strings(&self) -> String {
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
}

fn render_path(path: &Path, pool: &mut Pool) -> String {
    match path {
        Path::Forward(f) => format!("Path::Forward({})", pool.intern(f)),
        Path::Inverse(f) => format!("Path::Inverse({})", pool.intern(f)),
        Path::Chain(steps) => {
            let rendered: Vec<String> = steps
                .iter()
                .map(|s| match s {
                    Step::Forward(f) => format!("Step::Forward({})", pool.intern(f)),
                    Step::Inverse(f) => format!("Step::Inverse({})", pool.intern(f)),
                    Step::Type => "Step::Type".to_string(),
                })
                .collect();
            format!("Path::Chain(&[{}])", rendered.join(", "))
        }
        Path::Alternative(branches) => {
            let rendered: Vec<String> = branches
                .iter()
                .map(|b| match b {
                    AltBranch::Forward(f) => format!("AltBranch::Forward({})", pool.intern(f)),
                    AltBranch::Inverse(f) => format!("AltBranch::Inverse({})", pool.intern(f)),
                })
                .collect();
            format!("Path::Alternative(&[{}])", rendered.join(", "))
        }
    }
}

fn render_constraint(c: &Constraint, pool: &mut Pool) -> String {
    match c {
        Constraint::MinCount(n) => format!("Constraint::MinCount({n})"),
        Constraint::MaxCount(n) => format!("Constraint::MaxCount({n})"),
        Constraint::MaxLength(n) => format!("Constraint::MaxLength({n})"),
        Constraint::MinLength(n) => format!("Constraint::MinLength({n})"),
        Constraint::Length(n) => format!("Constraint::Length({n})"),
        Constraint::Datatype(d) => format!("Constraint::Datatype({})", pool.intern(d)),
        Constraint::HasValue(v) => format!("Constraint::HasValue({})", pool.intern(v)),
        Constraint::NodeKind(k) => {
            let k = match k {
                NodeKind::Iri => "NodeKind::Iri",
                NodeKind::Literal => "NodeKind::Literal",
                NodeKind::BlankNode => "NodeKind::BlankNode",
            };
            format!("Constraint::NodeKind({k})")
        }
        Constraint::Class(v) => format!("Constraint::Class({})", pool.intern_list(v)),
        Constraint::RefClass(v) => format!("Constraint::RefClass({})", pool.intern_list(v)),
        Constraint::In(v) => format!("Constraint::In({})", pool.intern_list(v)),
        Constraint::MinInclusive(v) => format!("Constraint::MinInclusive({v:?})"),
        Constraint::MaxInclusive(v) => format!("Constraint::MaxInclusive({v:?})"),
        Constraint::MinExclusive(v) => format!("Constraint::MinExclusive({v:?})"),
        Constraint::MaxExclusive(v) => format!("Constraint::MaxExclusive({v:?})"),
        Constraint::LessThan(f) => format!("Constraint::LessThan({})", pool.intern(f)),
        Constraint::LessThanOrEquals(f) => {
            format!("Constraint::LessThanOrEquals({})", pool.intern(f))
        }
        Constraint::NotClass(v) => format!("Constraint::NotClass({})", pool.intern_list(v)),
        Constraint::QualifiedIn { allowed, min } => {
            format!("Constraint::QualifiedIn {{ allowed: {}, min: {min} }}", pool.intern_list(allowed))
        }
    }
}

fn render_check(c: &Check, pool: &mut Pool) -> String {
    format!(
        "Check {{ constraint: {}, rule_id: {}, name: {}, message: {}, description: {}, severity: {} }}",
        render_constraint(&c.constraint, pool),
        pool.intern(&c.rule_id),
        pool.intern(&c.name),
        pool.intern(&c.message),
        pool.intern(&c.description),
        pool.intern(&c.severity),
    )
}

fn render_prop(p: &PropShape, pool: &mut Pool) -> String {
    let path = render_path(&p.path, pool);
    let checks: Vec<String> = p.checks.iter().map(|c| render_check(c, pool)).collect();
    format!("PropShape {{ path: {path}, checks: &[{}] }}", checks.join(", "))
}

fn render_logic(l: &Logic, pool: &mut Pool) -> String {
    let op = match l.op {
        LogicOp::And => "LogicOp::And",
        LogicOp::Or => "LogicOp::Or",
        LogicOp::Xone => "LogicOp::Xone",
    };
    let branches: Vec<String> = l
        .branches
        .iter()
        .map(|b| {
            let props: Vec<String> = b.props.iter().map(|p| render_prop(p, pool)).collect();
            format!("Branch {{ props: &[{}], negate: {} }}", props.join(", "), b.negate)
        })
        .collect();
    format!(
        "Logic {{ op: {op}, branches: &[{}], rule_id: {}, name: {}, message: {}, description: {}, severity: {} }}",
        branches.join(", "),
        pool.intern(&l.rule_id),
        pool.intern(&l.name),
        pool.intern(&l.message),
        pool.intern(&l.description),
        pool.intern(&l.severity),
    )
}

fn render_closed(c: &ClosedShape, pool: &mut Pool) -> String {
    format!(
        "&ClosedShape {{ allowed: {}, rule_id: {}, name: {}, message: {}, description: {}, severity: {} }}",
        pool.intern_list(&c.allowed),
        pool.intern(&c.rule_id),
        pool.intern(&c.name),
        pool.intern(&c.message),
        pool.intern(&c.description),
        pool.intern(&c.severity),
    )
}

/// Render a resolved shape table as Rust source.
pub fn render_shapes(family_id: &str, shapes: &[ShapeDef]) -> String {
    let mut pool = Pool::default();
    let mut body = String::new();

    for shape in shapes {
        let targets: Vec<String> = shape
            .targets
            .iter()
            .map(|t| match t {
                Target::Class(classes) => format!("Target::Class({})", pool.intern_list(classes)),
                Target::SubjectsOf(field) => format!("Target::SubjectsOf({})", pool.intern(field)),
            })
            .collect();

        let props: Vec<String> = shape
            .props
            .iter()
            .map(|p| {
                let rendered = render_prop(p, &mut pool);
                format!("&{}", pool.intern_prop(&rendered))
            })
            .collect();

        let closed = shape
            .closed
            .as_ref()
            .map(|c| format!("Some({})", render_closed(c, &mut pool)))
            .unwrap_or_else(|| "None".to_string());

        let logic: Vec<String> = shape.logic.iter().map(|l| render_logic(l, &mut pool)).collect();
        let profiles = pool.intern_list(&shape.profiles);
        let file = pool.intern(&shape.file);

        writeln!(body, "    ShapeDef {{").unwrap();
        writeln!(body, "        targets: &[{}],", targets.join(", ")).unwrap();
        writeln!(body, "        props: &[{}],", props.join(", ")).unwrap();
        writeln!(body, "        closed: {closed},").unwrap();
        writeln!(body, "        logic: &[{}],", logic.join(", ")).unwrap();
        writeln!(body, "        profiles: {profiles},").unwrap();
        writeln!(body, "        file: {file},").unwrap();
        writeln!(body, "    }},").unwrap();
    }

    let mut s = String::new();
    writeln!(s, "// Generated by cimgen — do not edit by hand.").unwrap();
    writeln!(s, "#![allow(clippy::all, dead_code, unused)]").unwrap();
    writeln!(s).unwrap();
    writeln!(
        s,
        "use crate::shapes::{{AltBranch, Branch, Check, ClosedShape, Constraint, Logic, LogicOp, NodeKind, Path, PropShape, ShapeDef, Step, Target}};"
    )
    .unwrap();
    writeln!(s).unwrap();
    writeln!(s, "// Repeated messages, descriptions and severities, interned.").unwrap();
    s.push_str(&pool.render_strings());
    writeln!(s).unwrap();
    writeln!(
        s,
        "// {} distinct property shapes, referenced {} times.",
        pool.prop_order.len(),
        shapes.iter().map(|s| s.props.len()).sum::<usize>()
    )
    .unwrap();
    s.push_str(&pool.render_props());
    writeln!(s).unwrap();
    writeln!(
        s,
        "/// Every reachable shape of the `{family_id}` family, resolved against its class table."
    )
    .unwrap();
    writeln!(s, "pub static SHAPES: &[ShapeDef] = &[").unwrap();
    s.push_str(&body);
    writeln!(s, "];").unwrap();
    s
}

/// Render the profile index: which IRI a dataset declares conformance to, and
/// what short code that means.
///
/// The NC counterpart of `cimvalidation::detect`'s CGMES profile URI table —
/// except read from `NCP/PROF` rather than written out by hand.
pub fn render_profiles_from(rows: &[(String, String)], codes: &[String]) -> String {
    let mut s = String::new();
    writeln!(s, "// Generated by cimgen — do not edit by hand.").unwrap();
    writeln!(s, "#![allow(clippy::all, dead_code)]").unwrap();
    writeln!(s).unwrap();
    writeln!(s, "/// Profile IRI → short code, for the profile a dataset declares (NC: `dcterms:conformsTo`; CGMES: `md:Model.profile`).").unwrap();
    writeln!(s, "pub static PROFILE_IRIS: &[(&str, &str)] = &[").unwrap();
    for (iri, code) in rows {
        writeln!(s, "    (\"{}\", \"{}\"),", esc(iri), esc(code)).unwrap();
    }
    writeln!(s, "];").unwrap();
    writeln!(s).unwrap();
    writeln!(s, "/// Every profile code of this family, sorted.").unwrap();
    writeln!(
        s,
        "pub static PROFILES: &[&str] = &[{}];",
        codes.iter().map(|c| format!("\"{}\"", esc(c))).collect::<Vec<_>>().join(", ")
    )
    .unwrap();
    s
}
