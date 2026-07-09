// Static call-graph analysis over `cimvalidation/src/sparql/*.rs`, counting distinct
// `rule_id`s reachable from each profile group's `validate()` entry point(s). Used by
// `cimgen --rule-report` to regenerate README.md's "SPARQL Check Coverage" table numbers
// instead of hand-maintaining them.
//
// A plain "which functions does validate() call" count is not enough: some functions emit
// different rule_ids per branch, some emit the same rule_id from multiple branches (one
// check), some checks are reused across files (e.g. ssh_not_solved_mas.rs calling
// ssh::check_*), and prof10.rs dispatches through a shared `prof10_violation(rule_id, ...)`
// constructor rather than constructing `Violation` directly. This module resolves the call
// graph (including qualified `module::fn` calls) and extracts rule_id string literals from
// both direct `Violation { rule_id: "...", .. }` literals and calls to such constructors.

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
    ("Common / AllProfiles", &[
        ("common", &["validate"]),
        ("common_solved_mas", &["validate"]),
    ]),
    ("Others (TP, DL, OP)", &[
        ("topology_not_solved_mas", &["validate"]),
        ("diagram_layout", &["validate"]),
        ("operation", &["validate"]),
    ]),
    ("C:600 conformance", &[("prof10", &["validate"])]),
    // Not part of the SPARQL Check Coverage table, but reported the same way since these
    // checks now carry real rule_ids too. check_base_voltage_in_eqbd_impl is invoked
    // directly from cimvalidation::sparql::validate_profile_local ("EQBD" arm), not through
    // quality::validate(), so it needs its own entry point.
    ("CIMdesk quality", &[("quality", &["validate", "check_base_voltage_in_eqbd_impl"])]),
];

struct Ctx {
    known_files: HashSet<String>,
    fns: HashMap<(String, String), ItemFn>,
    /// (file, fn) -> index of the parameter whose literal argument becomes `rule_id` when
    /// this function is used as a shared Violation constructor (e.g. prof10.rs's
    /// `prof10_violation`).
    constructors: HashMap<(String, String), usize>,
    /// file -> rule_id literals found inside that file's `macro_rules!` definitions.
    macro_rule_ids: HashMap<String, Vec<String>>,
}

pub struct GroupReport {
    pub label: &'static str,
    /// Number of distinct "leaf" check functions reachable from the group's entry points —
    /// this is the metric README's table historically counted (one row per check function,
    /// not per rule_id: a function that emits several rule_ids from different branches is
    /// still one check).
    pub check_count: usize,
    /// Every distinct rule_id string reachable from the group's entry points — informational,
    /// for `--verbose` output and debugging, not the headline number.
    pub rule_ids: Vec<String>,
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
            };
            let mut leaf_visited: HashSet<(String, String)> = HashSet::new();
            let mut leaves: HashSet<(String, String)> = HashSet::new();
            for (file_stem, entries) in *files {
                for entry in *entries {
                    analyzer.visit_entry(file_stem, entry);
                    collect_from_entry(&ctx, file_stem, entry, &mut leaf_visited, &mut leaves);
                }
                if let Some(ids) = ctx.macro_rule_ids.get(*file_stem) {
                    analyzer.out.extend(ids.iter().cloned());
                }
            }
            let mut rule_ids: Vec<String> = analyzer.out.into_iter().collect();
            rule_ids.sort();
            GroupReport { label, check_count: leaves.len(), rule_ids }
        })
        .collect()
}

/// Distinct, resolvable call targets found anywhere in `f`'s body (one hop; does not recurse
/// into the callees' own bodies). Constructor-helper calls are excluded — they contribute a
/// rule_id, not a separate check. Used only for entry points (`validate`, and overrides like
/// `check_base_voltage_in_eqbd_impl`), which can have an arbitrary shape (a sequence of
/// `v.extend(check_x(dataset))` calls, a bare tail call, or — for a leaf entry point — no
/// further sparql-module calls at all).
fn direct_targets(ctx: &Ctx, file: &str, f: &ItemFn) -> Vec<(String, String)> {
    let mut seen = HashSet::new();
    let mut out = Vec::new();
    scan_shallow(
        &f.block,
        &mut |_s| {},
        &mut |func_expr| {
            let Some(key) = resolve_callee(func_expr, &ctx.known_files, file) else { return };
            if ctx.constructors.contains_key(&key) { return; }
            if ctx.fns.contains_key(&key) && seen.insert(key.clone()) { out.push(key); }
        },
        &mut |_mac| {},
    );
    out
}

/// Entry-point variant of `collect_leaf_checks`: an entry point (`validate`, or an override
/// like `check_base_voltage_in_eqbd_impl`) is itself the check if it has no further
/// sparql-module call targets, otherwise it's unwound into its direct targets like any other
/// dispatcher.
fn collect_from_entry(
    ctx: &Ctx,
    file: &str,
    fn_name: &str,
    visited: &mut HashSet<(String, String)>,
    leaves: &mut HashSet<(String, String)>,
) {
    let key = (file.to_string(), fn_name.to_string());
    if visited.contains(&key) { return; }
    visited.insert(key.clone());
    let Some(f) = ctx.fns.get(&key) else { return };
    let targets = direct_targets(ctx, file, f);
    if targets.is_empty() {
        leaves.insert(key);
    } else {
        for (tf, tn) in targets { collect_leaf_checks(ctx, &tf, &tn, visited, leaves); }
    }
}

/// Recognize the one dispatcher shape actually used in this codebase (prof10.rs's
/// `check_prof10_model`): a function whose *entire* body is nothing but a single match
/// forwarding each arm to another function, with no violation-construction of its own. This is
/// deliberately narrow — unlike an ordinary check function that happens to call a shared local
/// helper as part of its logic (several SSH checks do this), a pure dispatcher has no other
/// statements and no direct `Violation` evidence, so misclassifying an ordinary check as a
/// dispatcher would require it to *coincidentally* have this exact single-statement shape too.
/// Arms that don't resolve to a known sparql-module function (e.g. a `_ => Vec::new()`
/// fallback) are ignored rather than disqualifying the whole function.
fn pure_dispatcher_targets(ctx: &Ctx, file: &str, f: &ItemFn) -> Option<Vec<(String, String)>> {
    let [Stmt::Expr(tail, None)] = f.block.stmts.as_slice() else { return None };
    let Expr::Match(m) = unwrap_paren(tail) else { return None };

    let has_struct = std::cell::Cell::new(false);
    scan_shallow(&f.block, &mut |s| { if is_violation_struct(s) { has_struct.set(true); } }, &mut |_| {}, &mut |_| {});
    if has_struct.get() { return None; }

    let mut seen = HashSet::new();
    let mut targets = Vec::new();
    for arm in &m.arms {
        if let Expr::Call(c) = unwrap_paren(&arm.body) {
            if let Some(key) = resolve_callee(&c.func, &ctx.known_files, file) {
                if ctx.fns.contains_key(&key) && seen.insert(key.clone()) { targets.push(key); }
            }
        }
    }
    if targets.is_empty() { None } else { Some(targets) }
}

fn unwrap_paren(e: &Expr) -> &Expr {
    match e {
        Expr::Paren(p) => unwrap_paren(&p.expr),
        _ => e,
    }
}

/// Recursively unwind pure dispatchers into the set of distinct leaf check functions they
/// forward to, across the whole call graph reachable from `(file, fn_name)`. Every other
/// function directly reached from a group's entry point(s) counts as one check, matching how
/// README's table has historically been maintained (one row per check function, not per
/// rule_id — a function emitting several rule_ids from different branches is still one check).
fn collect_leaf_checks(
    ctx: &Ctx,
    file: &str,
    fn_name: &str,
    visited: &mut HashSet<(String, String)>,
    leaves: &mut HashSet<(String, String)>,
) {
    let key = (file.to_string(), fn_name.to_string());
    if visited.contains(&key) { return; }
    visited.insert(key.clone());
    let Some(f) = ctx.fns.get(&key) else { return };
    match pure_dispatcher_targets(ctx, file, f) {
        Some(targets) => {
            for (tf, tn) in targets { collect_leaf_checks(ctx, &tf, &tn, visited, leaves); }
        }
        None => { leaves.insert(key); }
    }
}

fn build_ctx(dir: &Path) -> Ctx {
    let mut known_files = HashSet::new();
    let mut fns: HashMap<(String, String), ItemFn> = HashMap::new();
    let mut macro_rule_ids: HashMap<String, Vec<String>> = HashMap::new();

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
                // `rule_id: "literal"` pattern anywhere in the macro body (see e.g.
                // dynamics.rs's `check_exc_smd_type!` and topology_not_solved_mas.rs's
                // `check_switch_retained!`, which construct Violations this way).
                Item::Macro(m) => {
                    let ids = extract_rule_ids_from_macro(&m.mac);
                    if !ids.is_empty() {
                        macro_rule_ids.entry(stem.clone()).or_default().extend(ids);
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

    Ctx { known_files, fns, constructors, macro_rule_ids }
}

/// Flatten a token stream (recursing into every delimited group) into a linear sequence, then
/// scan for the token pattern `rule_id : "literal"`. Not a macro expander — just enough to find
/// rule_id literals hiding inside `macro_rules!` template bodies that syn can't parse as exprs.
fn extract_rule_ids_from_macro(mac: &syn::Macro) -> Vec<String> {
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
        let is_rule_id = matches!(&flat[i], proc_macro2::TokenTree::Ident(id) if id == "rule_id");
        let is_colon = matches!(&flat[i + 1], proc_macro2::TokenTree::Punct(p) if p.as_char() == ':');
        if !is_rule_id || !is_colon { continue; }
        if let proc_macro2::TokenTree::Literal(lit) = &flat[i + 2] {
            if let Ok(Lit::Str(s)) = syn::parse_str::<Lit>(&lit.to_string()) {
                let v = s.value();
                if !v.is_empty() { out.push(v); }
            }
        }
    }
    out
}

/// Detect functions like prof10.rs's `prof10_violation(id, rule_id, msg, severity)` whose
/// body directly returns `Violation { rule_id: rule_id, .. }` — i.e. the field is populated
/// straight from a parameter, so the literal has to be read from each call site instead.
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
            if ident == "rule_id" {
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
/// what the full `Analyzer` walk below is for). Shared by the constructor-detection pre-pass,
/// `shallow_produces`, and `direct_targets`.
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
}

impl<'a> Analyzer<'a> {
    fn visit_entry(&mut self, file: &str, fn_name: &str) {
        let key = (file.to_string(), fn_name.to_string());
        if self.visited.contains(&key) { return; }
        self.visited.insert(key.clone());
        if let Some(f) = self.ctx.fns.get(&key) {
            let prev = std::mem::replace(&mut self.current_file, file.to_string());
            let block = f.block.clone();
            self.walk_block(&block);
            self.current_file = prev;
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
            _ => {}
        }
    }

    fn handle_struct(&mut self, s: &ExprStruct) {
        if !is_violation_struct(s) { return; }
        for fv in &s.fields {
            if let Member::Named(ident) = &fv.member {
                if ident == "rule_id" {
                    if let Some(lit) = as_str_literal(&fv.expr) {
                        if !lit.is_empty() { self.out.insert(lit); }
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
                    if !lit.is_empty() { self.out.insert(lit); }
                }
            }
            return;
        }
        self.visit_entry(&target_file, &fn_name);
    }
}
