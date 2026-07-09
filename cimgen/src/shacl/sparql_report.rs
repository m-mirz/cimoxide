// Static call-graph analysis over `cimvalidation/src/sparql/*.rs`, collecting every distinct
// `name` (sh:name, e.g. "C:452:EQ:SynchronousMachine:aggregate") reachable from each profile
// group's `validate()` entry point(s). Used by `cimgen --rule-report`, combined in
// ttl_report.rs with the SPARQL constraint shapes actually defined in the CGMES SHACL TTL
// files, to regenerate README.md's "SPARQL Check Coverage" table instead of hand-maintaining
// it.
//
// Matching is done on `Violation.name` rather than `rule_id`: sh:name is a plain string with
// no namespace to normalize, unlike the SHACL shape IRI backing `rule_id`, whose prefix can
// legitimately differ between the TTL file's own declaration and however the importer
// canonicalizes it.
//
// A plain scan for `Violation { name: "...", .. }` literals is not enough: some functions
// emit different names per branch, some checks are reused across files (e.g.
// ssh_not_solved_mas.rs calling ssh::check_*), and prof10.rs dispatches through a shared
// `prof10_violation(rule_id, name, ...)` constructor rather than constructing `Violation`
// directly. This module resolves the call graph (including qualified `module::fn` calls) and
// extracts name string literals from both direct `Violation { name: "...", .. }` literals and
// calls to such constructors.

use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::Path;

use syn::{Expr, ExprCall, ExprStruct, FnArg, Item, ItemFn, Lit, Member, Pat, Stmt};

type FileEntry = (&'static str, &'static [&'static str]);

const GROUPS: &[(&str, &[FileEntry])] = &[
    ("Equipment (EQ)", &[
        ("equipment", &["validate"]),
        ("equipment_not_solved_mas", &["validate"]),
        ("equipment_boundary", &["validate"]),
    ]),
    ("Steady State Hypothesis (SSH)", &[
        ("ssh", &["validate"]),
        ("ssh_not_solved_mas", &["validate"]),
    ]),
    ("Dynamics (DY)", &[("dynamics", &["validate"])]),
    ("State Variables (SV)", &[
        ("state_variables", &["validate"]),
        ("state_variables_solved_mas", &["validate"]),
    ]),
    ("Short Circuit (SC)", &[
        ("shortcircuit", &["validate"]),
        ("shortcircuit_not_solved_mas", &["validate"]),
    ]),
    // C:600 conformance (prof10) is folded in here rather than given its own row: like
    // Common/AllProfiles, it's a cross-cutting rule that doesn't belong to a single
    // profile, and prof10.rs::validate() is already reached transitively from
    // common.rs's call graph on the Go side, so both tools' README tables can share one
    // "Common / AllProfiles" row without any extra call-graph surgery.
    ("Common / AllProfiles", &[
        ("common", &["validate"]),
        ("common_solved_mas", &["validate"]),
        ("prof10", &["validate"]),
    ]),
    ("Topology (TP)", &[("topology_not_solved_mas", &["validate"])]),
    ("DiagramLayout (DL)", &[("diagram_layout", &["validate"])]),
    ("Operation (OP)", &[("operation", &["validate"])]),
    // Not part of the SPARQL Check Coverage table, but reported the same way since these
    // checks now carry real rule_ids too. check_base_voltage_in_eqbd_impl is invoked
    // directly from cimvalidation::sparql::validate_profile_local ("EQBD" arm), not through
    // quality::validate(), so it needs its own entry point.
    ("CIMdesk quality", &[("quality", &["validate", "check_base_voltage_in_eqbd_impl"])]),
];

struct Ctx {
    known_files: HashSet<String>,
    fns: HashMap<(String, String), ItemFn>,
    /// (file, fn) -> index of the parameter whose literal argument becomes `name` when this
    /// function is used as a shared Violation constructor (e.g. prof10.rs's
    /// `prof10_violation`).
    constructors: HashMap<(String, String), usize>,
    /// file -> name literals found inside that file's `macro_rules!` definitions.
    macro_names: HashMap<String, Vec<String>>,
}

pub struct GroupReport {
    pub label: &'static str,
    /// Every distinct sh:name string reachable from the group's entry points. Combined in
    /// ttl_report.rs with the SPARQL constraint shapes actually defined in the CGMES SHACL TTL
    /// files to produce the Implemented/Total/Coverage figures in README's "SPARQL Check
    /// Coverage" table.
    pub names: Vec<String>,
}

/// Splits `s` on "|" and inserts each non-empty part into `out`. A single shape's sh:name can
/// itself be a "|"-joined compound of several rule names when one SPARQL constraint enforces
/// multiple named conformance rules at once (matching how ttl_report.rs splits the TTL's own
/// compound sh:name); some hand-written checks copy such a compound name verbatim into one
/// `Violation`.
fn add_name(out: &mut HashSet<String>, s: &str) {
    for part in s.split('|') {
        if !part.is_empty() {
            out.insert(part.to_string());
        }
    }
}

pub fn report(sparql_dir: &Path) -> Vec<GroupReport> {
    let ctx = build_ctx(sparql_dir);
    GROUPS
        .iter()
        .map(|(label, files)| {
            let mut analyzer = Analyzer {
                ctx: &ctx,
                visited: HashSet::new(),
                out: HashSet::new(),
                current_file: String::new(),
                current_locals: HashMap::new(),
            };
            for (file_stem, entries) in *files {
                for entry in *entries {
                    analyzer.visit_entry(file_stem, entry);
                }
                if let Some(ns) = ctx.macro_names.get(*file_stem) {
                    for n in ns { add_name(&mut analyzer.out, n); }
                }
            }
            let mut names: Vec<String> = analyzer.out.into_iter().collect();
            names.sort();
            GroupReport { label, names }
        })
        .collect()
}

fn build_ctx(dir: &Path) -> Ctx {
    let mut known_files = HashSet::new();
    let mut fns: HashMap<(String, String), ItemFn> = HashMap::new();
    let mut macro_names: HashMap<String, Vec<String>> = HashMap::new();

    let entries = fs::read_dir(dir).unwrap_or_else(|e| {
        panic!("cannot read sparql directory {}: {e}", dir.display());
    });
    let mut paths: Vec<_> = entries.filter_map(|e| e.ok()).map(|e| e.path()).collect();
    paths.sort();

    for path in paths {
        if path.extension().and_then(|e| e.to_str()) != Some("rs") { continue; }
        let stem = path.file_stem().unwrap().to_string_lossy().to_string();
        if stem == "mod" { continue; }
        let content = match fs::read_to_string(&path) {
            Ok(c) => c,
            Err(e) => { eprintln!("warning: cannot read {}: {e}", path.display()); continue; }
        };
        let file = match syn::parse_file(&content) {
            Ok(f) => f,
            Err(e) => { eprintln!("warning: cannot parse {}: {e}", path.display()); continue; }
        };
        known_files.insert(stem.clone());
        for item in &file.items {
            match item {
                Item::Fn(f) => { fns.insert((stem.clone(), f.sig.ident.to_string()), f.clone()); }
                // `macro_rules!` bodies contain `$`-metavariable/repetition syntax that isn't
                // valid standalone Rust, so syn can't structurally parse (or expand) them the
                // way we walk ordinary function bodies. Fall back to a raw token scan for the
                // `name: "literal"` pattern anywhere in the macro body (see e.g.
                // dynamics.rs's `check_exc_smd_type!` and topology_not_solved_mas.rs's
                // `check_switch_retained!`, which construct Violations this way).
                Item::Macro(m) => {
                    let ns = extract_names_from_macro(&m.mac);
                    if !ns.is_empty() {
                        macro_names.entry(stem.clone()).or_default().extend(ns);
                    }
                }
                _ => {}
            }
        }
    }

    let mut constructors = HashMap::new();
    for ((file, name), f) in &fns {
        if let Some(idx) = detect_constructor(f) {
            constructors.insert((file.clone(), name.clone()), idx);
        }
    }

    Ctx { known_files, fns, constructors, macro_names }
}

/// Flatten a token stream (recursing into every delimited group) into a linear sequence, then
/// scan for the token pattern `name : "literal"`. Not a macro expander — just enough to find
/// name literals hiding inside `macro_rules!` template bodies that syn can't parse as exprs.
fn extract_names_from_macro(mac: &syn::Macro) -> Vec<String> {
    fn flatten(ts: proc_macro2::TokenStream, out: &mut Vec<proc_macro2::TokenTree>) {
        for tt in ts {
            if let proc_macro2::TokenTree::Group(g) = &tt {
                flatten(g.stream(), out);
            } else {
                out.push(tt);
            }
        }
    }
    let mut flat = Vec::new();
    flatten(mac.tokens.clone(), &mut flat);

    let mut out = Vec::new();
    for i in 0..flat.len().saturating_sub(2) {
        let is_name = matches!(&flat[i], proc_macro2::TokenTree::Ident(id) if id == "name");
        let is_colon = matches!(&flat[i + 1], proc_macro2::TokenTree::Punct(p) if p.as_char() == ':');
        if !is_name || !is_colon { continue; }
        if let proc_macro2::TokenTree::Literal(lit) = &flat[i + 2] {
            if let Ok(Lit::Str(s)) = syn::parse_str::<Lit>(&lit.to_string()) {
                let v = s.value();
                if !v.is_empty() { out.push(v); }
            }
        }
    }
    out
}

/// Detect functions like prof10.rs's `prof10_violation(id, rule_id, name, severity)` whose
/// body directly returns `Violation { name: name, .. }` — i.e. the field is populated straight
/// from a parameter, so the literal has to be read from each call site instead.
fn detect_constructor(f: &ItemFn) -> Option<usize> {
    let params: Vec<String> = f.sig.inputs.iter().filter_map(|a| match a {
        FnArg::Typed(pt) => match pt.pat.as_ref() {
            Pat::Ident(pi) => Some(pi.ident.to_string()),
            _ => None,
        },
        FnArg::Receiver(_) => None,
    }).collect();

    let mut found: Option<ExprStruct> = None;
    scan_shallow(
        &f.block,
        &mut |s| { if found.is_none() && is_violation_struct(s) { found = Some(s.clone()); } },
        &mut |_| {},
        &mut |_| {},
    );
    let s = found?;

    for fv in &s.fields {
        if let Member::Named(ident) = &fv.member {
            if ident == "name" {
                if let Some(pname) = as_ident_ref(&fv.expr) {
                    return params.iter().position(|p| *p == pname);
                }
            }
        }
    }
    None
}

fn is_violation_struct(s: &ExprStruct) -> bool {
    s.path.segments.last().map(|seg| seg.ident == "Violation").unwrap_or(false)
}

/// Unwrap simple literal/receiver chains ("...".into(), "...".to_string(), &"...", etc.) down
/// to the underlying string literal, if any.
fn as_str_literal(e: &Expr) -> Option<String> {
    match e {
        Expr::Lit(el) => match &el.lit {
            Lit::Str(s) => Some(s.value()),
            _ => None,
        },
        Expr::MethodCall(mc) => as_str_literal(&mc.receiver),
        Expr::Paren(p) => as_str_literal(&p.expr),
        Expr::Reference(r) => as_str_literal(&r.expr),
        _ => None,
    }
}

/// Unwrap simple identifier/receiver chains (x, x.to_string(), &x, etc.) down to the
/// underlying single-segment path identifier, if any.
fn as_ident_ref(e: &Expr) -> Option<String> {
    match e {
        Expr::Path(p) if p.path.segments.len() == 1 => Some(p.path.segments[0].ident.to_string()),
        Expr::MethodCall(mc) => as_ident_ref(&mc.receiver),
        Expr::Paren(p) => as_ident_ref(&p.expr),
        Expr::Reference(r) => as_ident_ref(&r.expr),
        _ => None,
    }
}

/// Collects every string literal ever bound to a local variable anywhere in `block`
/// (including inside nested closures), whether via simple binding/reassignment (`let x =
/// "lit";` / `x = "lit";`) or tuple-destructuring from an if/else-if chain of literal tuples
/// (`let (rule_id, ..) = if cond { (lit1, ..) } else if .. { (lit2, ..) } else { .. };` — see
/// ssh_not_solved_mas.rs's check_cs_converter_target_angle_applicability and ssh.rs's
/// check_vs_converter_p_pcc_control). Used as a fallback in `Analyzer::handle_struct` when a
/// `Violation { rule_id: x, .. }` field's value is a plain local variable rather than a
/// literal or a known constructor call. Doesn't attempt real control-flow analysis: every
/// literal ever bound to a name, from any branch, is recorded as a candidate.
fn local_var_literals(block: &syn::Block) -> HashMap<String, Vec<String>> {
    let mut out: HashMap<String, Vec<String>> = HashMap::new();
    walk_block_for_locals(block, &mut out);
    out
}

fn walk_block_for_locals(block: &syn::Block, out: &mut HashMap<String, Vec<String>>) {
    for stmt in &block.stmts {
        match stmt {
            Stmt::Local(local) => {
                if let Some(init) = &local.init {
                    // `let x: T = ..;` / `let (a, b): (T1, T2) = ..;` wrap the real pattern in
                    // Pat::Type; unwrap it so Ident/Tuple handling below doesn't need to know
                    // about type annotations.
                    let pat = match &local.pat {
                        Pat::Type(pt) => pt.pat.as_ref(),
                        p => p,
                    };
                    match pat {
                        Pat::Ident(pi) => {
                            if let Some(lit) = as_str_literal(&init.expr) {
                                out.entry(pi.ident.to_string()).or_default().push(lit);
                            }
                        }
                        Pat::Tuple(_) => record_tuple_literals(pat, &init.expr, out),
                        _ => {}
                    }
                    walk_expr_for_locals(&init.expr, out);
                }
            }
            Stmt::Expr(e, _) => walk_expr_for_locals(e, out),
            _ => {}
        }
    }
}

fn record_tuple_literals(pat: &Pat, init: &Expr, out: &mut HashMap<String, Vec<String>>) {
    let Pat::Tuple(pt) = pat else { return };
    let mut tuples: Vec<Vec<Option<String>>> = Vec::new();
    collect_literal_tuples(init, &mut tuples);
    for tuple in &tuples {
        for (p, lit) in pt.elems.iter().zip(tuple.iter()) {
            if let (Pat::Ident(pi), Some(l)) = (p, lit) {
                out.entry(pi.ident.to_string()).or_default().push(l.clone());
            }
        }
    }
}

/// Resolves `init` down to every literal tuple it can evaluate to, recursing through
/// (possibly chained) if/else branches and block tail expressions -- but not through anything
/// that isn't a direct tuple-or-branch shape (e.g. a diverging `else { continue; }` branch
/// contributes nothing, rather than erroring).
fn collect_literal_tuples(e: &Expr, out: &mut Vec<Vec<Option<String>>>) {
    match e {
        Expr::Tuple(t) => out.push(t.elems.iter().map(as_str_literal).collect()),
        Expr::If(i) => {
            if let Some(tail) = block_tail_expr(&i.then_branch) { collect_literal_tuples(tail, out); }
            if let Some((_, else_e)) = &i.else_branch { collect_literal_tuples(else_e, out); }
        }
        Expr::Block(b) => { if let Some(tail) = block_tail_expr(&b.block) { collect_literal_tuples(tail, out); } }
        Expr::Paren(p) => collect_literal_tuples(&p.expr, out),
        _ => {}
    }
}

fn block_tail_expr(b: &syn::Block) -> Option<&Expr> {
    match b.stmts.last()? {
        Stmt::Expr(e, None) => Some(e),
        _ => None,
    }
}

/// Handles `for (val, prop, rule_id, name) in [(..), (..), ..] { .. }` (see dynamics.rs's
/// check_gov_hydro4_gain_points): a tuple pattern destructured from a literal array of
/// tuples. Every element position that lines up with a literal string in every array entry is
/// recorded under that position's identifier -- same "every literal, not just the one taken"
/// approach as `collect_literal_tuples`.
fn record_for_loop_literals(pat: &Pat, iter_expr: &Expr, out: &mut HashMap<String, Vec<String>>) {
    let Pat::Tuple(pt) = pat else { return };
    let arr = match iter_expr {
        Expr::Array(a) => a,
        Expr::Reference(r) => match r.expr.as_ref() { Expr::Array(a) => a, _ => return },
        Expr::Paren(p) => return record_for_loop_literals(pat, &p.expr, out),
        _ => return,
    };
    for elem in &arr.elems {
        let Expr::Tuple(t) = elem else { continue };
        let lits: Vec<Option<String>> = t.elems.iter().map(as_str_literal).collect();
        for (p, lit) in pt.elems.iter().zip(lits.iter()) {
            if let (Pat::Ident(pi), Some(l)) = (p, lit) {
                out.entry(pi.ident.to_string()).or_default().push(l.clone());
            }
        }
    }
}

fn walk_expr_for_locals(e: &Expr, out: &mut HashMap<String, Vec<String>>) {
    match e {
        Expr::Assign(a) => {
            if let Some(name) = as_ident_ref(&a.left) {
                if let Some(lit) = as_str_literal(&a.right) {
                    out.entry(name).or_default().push(lit);
                }
            }
            walk_expr_for_locals(&a.right, out);
        }
        Expr::Closure(c) => walk_expr_for_locals(&c.body, out),
        Expr::Block(b) => walk_block_for_locals(&b.block, out),
        Expr::If(i) => {
            walk_block_for_locals(&i.then_branch, out);
            if let Some((_, e2)) = &i.else_branch { walk_expr_for_locals(e2, out); }
        }
        Expr::Match(m) => { for arm in &m.arms { walk_expr_for_locals(&arm.body, out); } }
        Expr::ForLoop(fl) => {
            record_for_loop_literals(&fl.pat, &fl.expr, out);
            walk_block_for_locals(&fl.body, out);
        }
        Expr::While(w) => walk_block_for_locals(&w.body, out),
        Expr::Loop(lp) => walk_block_for_locals(&lp.body, out),
        Expr::Call(c) => { for a in &c.args { walk_expr_for_locals(a, out); } }
        Expr::MethodCall(mc) => {
            walk_expr_for_locals(&mc.receiver, out);
            for a in &mc.args { walk_expr_for_locals(a, out); }
        }
        Expr::Paren(p) => walk_expr_for_locals(&p.expr, out),
        Expr::Reference(r) => walk_expr_for_locals(&r.expr, out),
        Expr::Unary(u) => walk_expr_for_locals(&u.expr, out),
        Expr::Try(t) => walk_expr_for_locals(&t.expr, out),
        Expr::Cast(c) => walk_expr_for_locals(&c.expr, out),
        Expr::Return(r) => { if let Some(e) = &r.expr { walk_expr_for_locals(e, out); } }
        _ => {}
    }
}

fn resolve_callee(func: &Expr, known_files: &HashSet<String>, current_file: &str) -> Option<(String, String)> {
    let Expr::Path(p) = func else { return None };
    let segs: Vec<String> = p.path.segments.iter().map(|s| s.ident.to_string()).collect();
    match segs.as_slice() {
        [name] => Some((current_file.to_string(), name.clone())),
        [.., module, name] if known_files.contains(module) => Some((module.clone(), name.clone())),
        _ => None,
    }
}

/// Recursively find every `Expr::Struct` literal, call-callee expression, and macro invocation
/// in a function body, without following resolved calls into their callees' own bodies (that's
/// what the full `Analyzer` walk below is for). Used by the constructor-detection pre-pass
/// (`detect_constructor`).
fn scan_shallow(
    block: &syn::Block,
    on_struct: &mut dyn FnMut(&ExprStruct),
    on_call: &mut dyn FnMut(&Expr),
    on_macro: &mut dyn FnMut(&syn::Macro),
) {
    fn walk_expr(e: &Expr, on_struct: &mut dyn FnMut(&ExprStruct), on_call: &mut dyn FnMut(&Expr), on_macro: &mut dyn FnMut(&syn::Macro)) {
        match e {
            Expr::Struct(s) => on_struct(s),
            Expr::Call(c) => {
                on_call(&c.func);
                for a in &c.args { walk_expr(a, on_struct, on_call, on_macro); }
            }
            Expr::MethodCall(mc) => {
                walk_expr(&mc.receiver, on_struct, on_call, on_macro);
                for a in &mc.args { walk_expr(a, on_struct, on_call, on_macro); }
            }
            Expr::If(i) => {
                walk_expr(&i.cond, on_struct, on_call, on_macro);
                walk_block(&i.then_branch, on_struct, on_call, on_macro);
                if let Some((_, e2)) = &i.else_branch { walk_expr(e2, on_struct, on_call, on_macro); }
            }
            Expr::Match(m) => {
                walk_expr(&m.expr, on_struct, on_call, on_macro);
                for arm in &m.arms { walk_expr(&arm.body, on_struct, on_call, on_macro); }
            }
            Expr::Block(b) => walk_block(&b.block, on_struct, on_call, on_macro),
            Expr::Return(r) => { if let Some(e) = &r.expr { walk_expr(e, on_struct, on_call, on_macro); } }
            Expr::Paren(p) => walk_expr(&p.expr, on_struct, on_call, on_macro),
            Expr::Reference(r) => walk_expr(&r.expr, on_struct, on_call, on_macro),
            Expr::Unary(u) => walk_expr(&u.expr, on_struct, on_call, on_macro),
            Expr::Try(t) => walk_expr(&t.expr, on_struct, on_call, on_macro),
            Expr::Cast(c) => walk_expr(&c.expr, on_struct, on_call, on_macro),
            Expr::Binary(b) => { walk_expr(&b.left, on_struct, on_call, on_macro); walk_expr(&b.right, on_struct, on_call, on_macro); }
            Expr::Array(a) => { for e in &a.elems { walk_expr(e, on_struct, on_call, on_macro); } }
            Expr::Tuple(t) => { for e in &t.elems { walk_expr(e, on_struct, on_call, on_macro); } }
            Expr::Let(l) => walk_expr(&l.expr, on_struct, on_call, on_macro),
            Expr::ForLoop(fl) => { walk_expr(&fl.expr, on_struct, on_call, on_macro); walk_block(&fl.body, on_struct, on_call, on_macro); }
            Expr::While(w) => { walk_expr(&w.cond, on_struct, on_call, on_macro); walk_block(&w.body, on_struct, on_call, on_macro); }
            Expr::Loop(lp) => walk_block(&lp.body, on_struct, on_call, on_macro),
            Expr::Macro(m) => {
                on_macro(&m.mac);
                if m.mac.path.is_ident("vec") {
                    if let Ok(exprs) = m.mac.parse_body_with(syn::punctuated::Punctuated::<Expr, syn::Token![,]>::parse_terminated) {
                        for e in exprs.iter() { walk_expr(e, on_struct, on_call, on_macro); }
                    }
                }
            }
            _ => {}
        }
    }
    fn walk_block(b: &syn::Block, on_struct: &mut dyn FnMut(&ExprStruct), on_call: &mut dyn FnMut(&Expr), on_macro: &mut dyn FnMut(&syn::Macro)) {
        for stmt in &b.stmts {
            match stmt {
                Stmt::Expr(e, _) => walk_expr(e, on_struct, on_call, on_macro),
                Stmt::Local(local) => {
                    if let Some(init) = &local.init {
                        walk_expr(&init.expr, on_struct, on_call, on_macro);
                        if let Some((_, d)) = &init.diverge { walk_expr(d, on_struct, on_call, on_macro); }
                    }
                }
                Stmt::Macro(sm) => on_macro(&sm.mac),
                _ => {}
            }
        }
    }
    walk_block(block, on_struct, on_call, on_macro);
}

struct Analyzer<'a> {
    ctx: &'a Ctx,
    visited: HashSet<(String, String)>,
    out: HashSet<String>,
    current_file: String,
    /// Local variable -> every literal ever bound to it, for the top-level function currently
    /// being visited (see `local_var_literals`). Recomputed and saved/restored around each
    /// `visit_entry` call, since the binding is only valid within that one function's body.
    current_locals: HashMap<String, Vec<String>>,
}

impl<'a> Analyzer<'a> {
    fn visit_entry(&mut self, file: &str, fn_name: &str) {
        let key = (file.to_string(), fn_name.to_string());
        if self.visited.contains(&key) { return; }
        self.visited.insert(key.clone());
        if let Some(f) = self.ctx.fns.get(&key) {
            let prev_file = std::mem::replace(&mut self.current_file, file.to_string());
            let prev_locals = std::mem::replace(&mut self.current_locals, local_var_literals(&f.block));
            let block = f.block.clone();
            self.walk_block(&block);
            self.current_file = prev_file;
            self.current_locals = prev_locals;
        }
    }

    fn walk_block(&mut self, b: &syn::Block) {
        for stmt in &b.stmts {
            match stmt {
                Stmt::Expr(e, _) => self.walk_expr(e),
                Stmt::Local(local) => {
                    if let Some(init) = &local.init {
                        self.walk_expr(&init.expr);
                        if let Some((_, d)) = &init.diverge { self.walk_expr(d); }
                    }
                }
                _ => {}
            }
        }
    }

    fn walk_expr(&mut self, e: &Expr) {
        match e {
            Expr::Struct(s) => self.handle_struct(s),
            Expr::Call(c) => {
                self.handle_call(c);
                for a in &c.args { self.walk_expr(a); }
            }
            Expr::MethodCall(mc) => {
                self.walk_expr(&mc.receiver);
                for a in &mc.args { self.walk_expr(a); }
            }
            Expr::If(i) => {
                self.walk_expr(&i.cond);
                self.walk_block(&i.then_branch);
                if let Some((_, e2)) = &i.else_branch { self.walk_expr(e2); }
            }
            Expr::Match(m) => {
                self.walk_expr(&m.expr);
                for arm in &m.arms { self.walk_expr(&arm.body); }
            }
            Expr::Block(b) => self.walk_block(&b.block),
            Expr::Return(r) => { if let Some(e) = &r.expr { self.walk_expr(e); } }
            Expr::Paren(p) => self.walk_expr(&p.expr),
            Expr::Reference(r) => self.walk_expr(&r.expr),
            Expr::Unary(u) => self.walk_expr(&u.expr),
            Expr::Try(t) => self.walk_expr(&t.expr),
            Expr::Cast(c) => self.walk_expr(&c.expr),
            Expr::Binary(b) => { self.walk_expr(&b.left); self.walk_expr(&b.right); }
            Expr::Array(a) => { for e in &a.elems { self.walk_expr(e); } }
            Expr::Tuple(t) => { for e in &t.elems { self.walk_expr(e); } }
            Expr::Let(l) => self.walk_expr(&l.expr),
            Expr::ForLoop(fl) => { self.walk_expr(&fl.expr); self.walk_block(&fl.body); }
            Expr::While(w) => { self.walk_expr(&w.cond); self.walk_block(&w.body); }
            Expr::Loop(lp) => self.walk_block(&lp.body),
            Expr::Macro(m) => {
                if m.mac.path.is_ident("vec") {
                    if let Ok(exprs) = m.mac.parse_body_with(syn::punctuated::Punctuated::<Expr, syn::Token![,]>::parse_terminated) {
                        for e in exprs.iter() { self.walk_expr(e); }
                    }
                }
            }
            // A local closure (e.g. sparql_ssh_notsolvedmas.go's Go analogue,
            // check_tap_changer_step_integer's `report` closure here) is just as reachable as
            // any inline block -- it has its own body but is invoked from within the same
            // function, so its Violation literals belong to this function's rule_ids too.
            Expr::Closure(c) => self.walk_expr(&c.body),
            _ => {}
        }
    }

    fn handle_struct(&mut self, s: &ExprStruct) {
        if !is_violation_struct(s) { return; }
        for fv in &s.fields {
            if let Member::Named(ident) = &fv.member {
                if ident == "name" {
                    if let Some(lit) = as_str_literal(&fv.expr) {
                        if !lit.is_empty() { add_name(&mut self.out, &lit); }
                    } else if let Some(name) = as_ident_ref(&fv.expr) {
                        // Not a literal -- the field's value is a plain local variable (e.g.
                        // check_cs_converter_target_angle_applicability's `rule_name`, bound via
                        // `let (rule_id, rule_name, ..) = if for_alpha { (...) } else { (...) };`).
                        // Every literal ever bound to that name anywhere in the enclosing
                        // top-level function is a candidate, since we don't track which branch
                        // runs.
                        if let Some(lits) = self.current_locals.get(&name) {
                            for l in lits.clone() { add_name(&mut self.out, &l); }
                        }
                    }
                }
            }
        }
    }

    fn handle_call(&mut self, c: &ExprCall) {
        let Some((target_file, fn_name)) = resolve_callee(&c.func, &self.ctx.known_files, &self.current_file) else { return };
        if let Some(&idx) = self.ctx.constructors.get(&(target_file.clone(), fn_name.clone())) {
            if let Some(arg) = c.args.iter().nth(idx) {
                if let Some(lit) = as_str_literal(arg) {
                    if !lit.is_empty() { add_name(&mut self.out, &lit); }
                } else if let Some(name) = as_ident_ref(arg) {
                    // Constructor called with a local variable rather than a literal (e.g.
                    // dynamics.rs's check_gov_hydro4_gain_points calling `dyn_viol(mrid,
                    // rule_id, name, ..)` where `name` comes from destructuring a `for (..,
                    // name) in [(..), ..]` array-of-tuples loop) -- same current_locals
                    // fallback as handle_struct.
                    if let Some(lits) = self.current_locals.get(&name) {
                        for l in lits.clone() { add_name(&mut self.out, &l); }
                    }
                }
            }
            return;
        }
        self.visit_entry(&target_file, &fn_name);
    }
}
