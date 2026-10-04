//! Phase-0 harness: CGMES validation, generated validators vs. an interpreted
//! shape table, on equal terms.
//!
//! Fixes the four ways `cgmes_table.rs` was unfair:
//!
//! 1. The generated side is SHACL only (`generated::validate_profile_shacl` /
//!    `generated::validate_crossprofile_shacl`); the hand-written `sparql` rules are timed
//!    separately, because the table does not replace them.
//! 2. Work is shaped the way `validate_files` shapes it: header + each detected
//!    profile per file, then cross-profile rules once on the merged dataset.
//!    Each table call builds its own indexes, as `bag::validate_profile` does.
//! 3. Results are compared as multisets of `(object_id, rule_id, property)`,
//!    per unit of work, not as totals.
//! 4. The table is the interned `&'static` IR (`shapes::ShapeDef`) that cimgen
//!    or `shape_source` would produce, not the owned resolver output.
//!
//! The table side is the production interpreter, `bag::validate_shapes` with
//! `Source::Typed`, which reads typed elements through `CimEntry::block`.
//!
//! The table is `cimvalidation::cgmes_shapes()`, the one validation uses, unless
//! `EXACT_TARGETS` or `BAG` asks for a variant; variants are resolved here with
//! the same manifest (`cimschema::shacl::cgmes_manifest`).
//!
//! Run from the repository root:
//!   cargo run --release -p cimoxide-validation \
//!       --features dynamic-shapes,generated-validators \
//!       --example cgmes_ab -- [-r rounds] [corpus-dir ...]
//!
//! `BAG=1` keeps `sh:datatype`/`sh:nodeKind` (the untyped `simplify`), which is
//! a behaviour change and expected to break parity. `EXACT_TARGETS=1` looks
//! up only the literal `sh:targetClass`, as the generated code does, so the
//! interpreter can be checked for parity apart from target expansion.
//! `DIFF_OUT=path` writes every unmatched violation as TSV.

use std::collections::{BTreeMap, HashMap};
use std::fmt::Write as _;
use std::path::{Path as FsPath, PathBuf};
use std::time::{Duration, Instant};

use cimdecoder::CimDataset;
use cimschema::family;
use cimschema::shacl::skip::SkipCollector;
use cimvalidation::bag::Source;
use cimvalidation::shapes::{ShapeDef, Target};
use cimvalidation::{Config, Violation};

// ---------------------------------------------------------------------------
// The table: the production one, or a variant built here
// ---------------------------------------------------------------------------

use cimschema::shacl::cgmes_manifest::{CROSS_PROFILES as CROSS, MANIFEST};

fn build_table() -> (&'static [ShapeDef], usize) {
    if std::env::var_os("EXACT_TARGETS").is_none() && std::env::var_os("BAG").is_none() {
        // The table validation uses. Skips are the resolver's, reported by cimgen.
        return (cimvalidation::cgmes_shapes(), 0);
    }
    let spec = cimschema::import::import_schema_files(family::CGMES.default_schema, &family::CGMES, false)
        .expect("could not import the CGMES RDFS (run from the repository root)");
    let nc = cimschema::import::import_schema_files(family::NC.default_schema, &family::NC, false)
        .expect("could not import the NC RDFS");

    let mut files = Vec::new();
    for (stem, _) in MANIFEST {
        let mut path = PathBuf::from("application-profiles-library/CGMES/SHACL");
        path.push(format!("{stem}.ttl"));
        let fr = cimschema::shacl::ttl_import::import_ttl_file(&path)
            .unwrap_or_else(|e| panic!("{}: {e}", path.display()));
        // One file may carry several tags; import it once.
        if !files.iter().any(|f: &cimschema::shacl::model::FileResults| f.file_name == fr.file_name) {
            files.push(fr);
        }
    }

    let fam: &'static family::Family = if std::env::var_os("BAG").is_some() {
        Box::leak(Box::new(family::Family { typed: false, ..family::CGMES }))
    } else {
        &family::CGMES
    };
    let _ = cimschema::shacl::simplify::simplify(&mut files, fam);

    let mut profiles_of: HashMap<String, Vec<String>> = HashMap::new();
    for (stem, tag) in MANIFEST {
        profiles_of.entry((*stem).to_string()).or_default().push((*tag).to_string());
    }

    let mut collector = SkipCollector::new();
    let (shapes, _) =
        cimschema::shacl::resolve::resolve_shapes(&spec, &[&nc], &files, &profiles_of, &mut collector);
    let entries = collector.into_entries();
    let skips = entries.len();
    if std::env::var_os("SKIPS").is_some() {
        let mut by: BTreeMap<(String, String), usize> = BTreeMap::new();
        for e in &entries {
            let reason: String = e.reason.chars().take(70).collect();
            *by.entry((e.component.clone(), reason)).or_default() += 1;
        }
        let mut v: Vec<_> = by.into_iter().collect();
        v.sort_by_key(|x| std::cmp::Reverse(x.1));
        if let Some(pat) = std::env::var_os("SKIP_NAMES") {
            let pat = pat.to_string_lossy().into_owned();
            for e in entries.iter().filter(|e| pat.split(',').any(|p| e.name.contains(p))) {
                println!("skip: {} | {} | {} | {:?} | {}", e.name, e.prop, e.component, e.class_names, e.reason);
            }
        }
        println!("resolver skips by (component, reason):");
        for ((comp, reason), n) in v {
            println!("  {n:>5}  {comp:<40} {reason}");
        }
        println!();
    }
    let table = cimvalidation::shape_source::intern_shapes(&shapes);
    if std::env::var_os("EXACT_TARGETS").is_some() {
        return (exact_targets(table, &spec), skips);
    }
    (table, skips)
}

/// Emulate the generated validators' target lookup: `by_type.get(X)` for the
/// literal `sh:targetClass X`, so an abstract X matches nothing and a concrete
/// X matches none of its subclasses.
///
/// The resolver has already expanded X to its concrete descendants. X is
/// recoverable from that list: if X is concrete it is in the list and is an
/// ancestor of every other member; if no member is, X was abstract.
fn exact_targets(
    table: &'static [ShapeDef],
    spec: &cimschema::model::CimSpecification,
) -> &'static [ShapeDef] {
    fn walk(spec: &cimschema::model::CimSpecification, anc: &str, start: &str) -> bool {
        let mut c = start;
        for _ in 0..64 {
            if c == anc {
                return true;
            }
            match spec.types.get(c) {
                Some(t) if !t.super_type.is_empty() => c = &t.super_type,
                _ => return false,
            }
        }
        false
    }
    let is_ancestor = |anc: &str, c: &str| walk(spec, anc, c);
    let shapes: Vec<ShapeDef> = table
        .iter()
        .map(|s| {
            let targets: Vec<Target> = s
                .targets
                .iter()
                .map(|t| match t {
                    Target::Class(list) => {
                        let root = list.iter().find(|x| list.iter().all(|d| is_ancestor(x, d)));
                        Target::Class(match root {
                            Some(x) => Vec::leak(vec![*x]),
                            None => &[],
                        })
                    }
                    Target::SubjectsOf(f) => Target::SubjectsOf(f),
                })
                .collect();
            ShapeDef {
                targets: Vec::leak(targets),
                props: s.props,
                closed: s.closed,
                logic: s.logic,
                profiles: s.profiles,
                file: s.file,
            }
        })
        .collect();
    Vec::leak(shapes)
}

// ---------------------------------------------------------------------------
// The interpreter
// ---------------------------------------------------------------------------

/// One `bag::validate_profile`-style call over a pre-selected shape set. Each
/// call builds its own reverse and subject indexes, as the real one does.
fn run_table(ds: &CimDataset, active: &[&'static ShapeDef]) -> Vec<Violation> {
    cimvalidation::bag::validate_shapes(ds, Source::Typed, active)
}

// ---------------------------------------------------------------------------
// Units of work, shaped like validate_files
// ---------------------------------------------------------------------------

struct Corpus {
    name: String,
    per_file: Vec<CimDataset>,
    /// Detected profiles per file, as `validate_files` computes them.
    profiles: Vec<Vec<String>>,
    merged: CimDataset,
    cfg: Config,
    elements: usize,
}

fn load_corpus(dir: &FsPath) -> Corpus {
    let mut paths: Vec<PathBuf> = std::fs::read_dir(dir)
        .unwrap_or_else(|e| panic!("cannot read {}: {e}", dir.display()))
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .filter(|p| p.extension().and_then(|x| x.to_str()) == Some("xml"))
        .collect();
    paths.sort();
    let refs: Vec<&FsPath> = paths.iter().map(PathBuf::as_path).collect();
    let per_file = CimDataset::decode_files_parallel_separate(&refs).expect("corpus did not decode");
    let merged = CimDataset::decode_files(&refs).expect("corpus did not decode");
    let cfg = cimvalidation::combined_config(&per_file, None, None, false, false, Vec::new());
    let profiles = per_file.iter().map(|ds| cimvalidation::detect_config(ds).profiles).collect();
    Corpus {
        name: dir.display().to_string(),
        elements: merged.entries.len(),
        per_file,
        profiles,
        merged,
        cfg,
    }
}

/// A slice of the pipeline both paths implement: the header rules, one local
/// profile across every file that declares it, or one profile's cross-profile
/// rules on the merged dataset.
#[derive(Clone)]
enum Unit {
    Header,
    Local(String),
    Cross(String),
}

impl Unit {
    fn label(&self) -> String {
        match self {
            Unit::Header => "HDR".into(),
            Unit::Local(p) => p.clone(),
            Unit::Cross(p) => format!("X:{p}"),
        }
    }
}

fn units(c: &Corpus) -> Vec<Unit> {
    let mut out = vec![Unit::Header];
    let mut local: Vec<&String> = c.profiles.iter().flatten().collect();
    local.sort();
    local.dedup();
    out.extend(local.into_iter().map(|p| Unit::Local(p.clone())));
    let has = |p: &str| c.cfg.profiles.is_empty() || c.cfg.profiles.iter().any(|x| x == p);
    out.extend(CROSS.iter().filter(|p| has(p)).map(|p| Unit::Cross((*p).to_string())));
    out
}

fn active_for(shapes: &'static [ShapeDef], tags: &[String]) -> Vec<&'static ShapeDef> {
    shapes.iter().filter(|s| s.profiles.iter().any(|p| tags.iter().any(|t| t == p))).collect()
}

struct Table {
    shapes: &'static [ShapeDef],
}

impl Table {
    fn local_tags(&self, profile: &str, cfg: &Config) -> Vec<String> {
        let mut tags = vec![profile.to_string()];
        if cfg.not_solved {
            tags.push(format!("{profile}!NS"));
        }
        tags
    }
}

fn run_generated(c: &Corpus, u: &Unit) -> Vec<Violation> {
    match u {
        Unit::Header => c.per_file.iter().flat_map(|ds| cimvalidation::generated::validate_header(ds, &c.cfg)).collect(),
        Unit::Local(p) => c
            .per_file
            .iter()
            .zip(&c.profiles)
            .filter(|(_, ps)| ps.contains(p))
            .flat_map(|(ds, _)| cimvalidation::generated::validate_profile_shacl(ds, p, &c.cfg))
            .collect(),
        Unit::Cross(p) => {
            // validate_crossprofile_shacl runs every enabled profile at once;
            // restrict it to this one.
            let cfg = Config { profiles: vec![p.clone()], ..c.cfg.clone() };
            cimvalidation::generated::validate_crossprofile_shacl(&c.merged, &cfg)
        }
    }
}

fn run_tabled(c: &Corpus, t: &Table, u: &Unit) -> Vec<Violation> {
    match u {
        Unit::Header => {
            let active = active_for(t.shapes, &["HDR".to_string()]);
            c.per_file.iter().flat_map(|ds| run_table(ds, &active)).collect()
        }
        Unit::Local(p) => {
            let active = active_for(t.shapes, &t.local_tags(p, &c.cfg));
            c.per_file
                .iter()
                .zip(&c.profiles)
                .filter(|(_, ps)| ps.contains(p))
                .flat_map(|(ds, _)| run_table(ds, &active))
                .collect()
        }
        Unit::Cross(p) => {
            let active = active_for(t.shapes, &[format!("X:{p}")]);
            run_table(&c.merged, &active)
        }
    }
}

fn run_sparql(c: &Corpus, u: &Unit) -> Vec<Violation> {
    match u {
        Unit::Header => Vec::new(),
        Unit::Local(p) => c
            .per_file
            .iter()
            .zip(&c.profiles)
            .filter(|(_, ps)| ps.contains(p))
            .flat_map(|(ds, _)| cimvalidation::sparql::validate_profile_local(ds, p, &c.cfg))
            .collect(),
        // The cross-profile SPARQL rules are not partitioned by profile; they
        // are timed once as a whole, see `main`.
        Unit::Cross(_) => Vec::new(),
    }
}

// ---------------------------------------------------------------------------
// Parity
// ---------------------------------------------------------------------------

type Key = (String, String, String);

fn key(v: &Violation) -> Key {
    (v.object_id.clone(), v.rule_id.clone(), v.property.clone())
}

fn multiset(vs: &[Violation]) -> BTreeMap<Key, i64> {
    let mut m = BTreeMap::new();
    for v in vs {
        *m.entry(key(v)).or_insert(0) += 1;
    }
    m
}

/// (only in generated, only in table, same violation with a different
/// `property` label), as multisets.
///
/// The label is split out because the two paths disagree on a convention, not
/// on a finding: for an inverse path the generated code reports the shape's
/// `sh:name`, the table the field key.
type Unmatched = Vec<(Key, i64)>;

fn diff(generated: &[Violation], tab: &[Violation]) -> (Unmatched, Unmatched, i64) {
    let mut g = multiset(generated);
    for (k, n) in multiset(tab) {
        *g.entry(k).or_insert(0) -= n;
    }
    // Pair surpluses on (object, rule) regardless of label.
    let mut by_obj_rule: BTreeMap<(String, String), i64> = BTreeMap::new();
    for ((o, r, _), n) in &g {
        *by_obj_rule.entry((o.clone(), r.clone())).or_insert(0) += n;
    }
    let mut relabelled = 0;
    for ((o, r, _), n) in g.iter().filter(|(_, n)| **n > 0) {
        if by_obj_rule[&(o.clone(), r.clone())] == 0 {
            relabelled += n;
        }
    }
    let unpaired = |k: &Key| by_obj_rule[&(k.0.clone(), k.1.clone())] != 0;
    let only_gen = g.iter().filter(|(k, n)| **n > 0 && unpaired(k)).map(|(k, n)| (k.clone(), *n)).collect();
    let only_tab = g.iter().filter(|(k, n)| **n < 0 && unpaired(k)).map(|(k, n)| (k.clone(), -*n)).collect();
    (only_gen, only_tab, relabelled)
}

fn by_rule(xs: &[(Key, i64)]) -> Vec<(String, i64, String)> {
    let mut m: BTreeMap<&str, (i64, &str)> = BTreeMap::new();
    for ((obj, rule, _), n) in xs {
        let e = m.entry(rule.as_str()).or_insert((0, obj.as_str()));
        e.0 += n;
    }
    let mut v: Vec<_> = m.into_iter().map(|(r, (n, s))| (r.to_string(), n, s.to_string())).collect();
    v.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));
    v
}

// ---------------------------------------------------------------------------
// Static coverage: which (tag, rule, class) each path can report at all
// ---------------------------------------------------------------------------

type Cov = std::collections::BTreeSet<(String, String, String)>;

/// Read the generated validator sources the manifest wires in. Every emitted
/// violation is a struct literal with `rule_id` followed by `class`, both
/// string literals, so a line scan recovers the triples exactly.
fn generated_coverage() -> Cov {
    let mut out = Cov::new();
    for (stem, tag) in MANIFEST {
        let module = format!("p{}", stem.to_lowercase().replace('-', "_"));
        let path = format!("cimvalidation/src/generated_{module}.rs");
        let Ok(src) = std::fs::read_to_string(&path) else {
            // cimgen writes no module for a file whose every shape it skipped.
            continue;
        };
        let lit = |l: &str, field: &str| {
            let rest = l.trim().strip_prefix(field)?.trim_start();
            let rest = rest.strip_prefix('"')?;
            Some(rest[..rest.find('"')?].to_string())
        };
        let mut rule: Option<String> = None;
        for line in src.lines() {
            if let Some(r) = lit(line, "rule_id:") {
                rule = Some(r);
            } else if let Some(c) = lit(line, "class:") {
                if let Some(r) = rule.take() {
                    out.insert(((*tag).to_string(), r, c));
                }
            }
        }
    }
    out
}

fn table_coverage(shapes: &[ShapeDef]) -> Cov {
    let mut out = Cov::new();
    for s in shapes {
        let mut classes: Vec<String> = Vec::new();
        for t in s.targets {
            match t {
                Target::Class(list) => classes.extend(list.iter().map(|c| c.to_string())),
                Target::SubjectsOf(f) => classes.push(format!("subjectsOf:{f}")),
            }
        }
        let mut rules: Vec<&str> = s.props.iter().flat_map(|p| p.checks.iter().map(|c| c.rule_id)).collect();
        rules.extend(s.closed.map(|c| c.rule_id));
        rules.extend(s.logic.iter().map(|l| l.rule_id));
        for tag in s.profiles {
            for r in &rules {
                for c in &classes {
                    out.insert((tag.to_string(), r.to_string(), c.clone()));
                }
            }
        }
    }
    out
}

fn report_coverage(shapes: &[ShapeDef]) {
    let g = generated_coverage();
    let t = table_coverage(shapes);
    let only_g: Vec<_> = g.difference(&t).collect();
    let only_t: Vec<_> = t.difference(&g).collect();
    println!(
        "coverage (tag, rule, class): generated {}, table {}, both {}, only generated {}, only table {}",
        g.len(),
        t.len(),
        g.intersection(&t).count(),
        only_g.len(),
        only_t.len()
    );
    for (side, xs) in [("generated", &only_g), ("table", &only_t)] {
        let mut by_rule: BTreeMap<(&str, &str), (usize, &str)> = BTreeMap::new();
        for (tag, rule, class) in xs.iter() {
            let e = by_rule.entry((tag.as_str(), rule.as_str())).or_insert((0, class.as_str()));
            e.0 += 1;
        }
        let mut v: Vec<_> = by_rule.into_iter().collect();
        v.sort_by(|a, b| b.1 .0.cmp(&a.1 .0).then(a.0.cmp(&b.0)));
        println!("  only {side}: {} rules", v.len());
        for ((tag, rule), (n, class)) in v.iter().take(12) {
            println!("    {tag:<6} {rule:<70} {n:>4} classes (e.g. {class})");
        }
    }
    if let Some(path) = std::env::var_os("COVERAGE_OUT") {
        let mut s = String::new();
        for (side, xs) in [("generated", &only_g), ("table", &only_t)] {
            for (tag, rule, class) in xs.iter() {
                let _ = writeln!(s, "{side}\t{tag}\t{rule}\t{class}");
            }
        }
        std::fs::write(&path, s).expect("cannot write COVERAGE_OUT");
    }
    println!();
}

// ---------------------------------------------------------------------------
// Timing
// ---------------------------------------------------------------------------

fn median(mut v: Vec<Duration>) -> Duration {
    v.sort_unstable();
    v[v.len() / 2]
}

fn spread(v: &[Duration]) -> f64 {
    let lo = v.iter().min().unwrap().as_secs_f64();
    let hi = v.iter().max().unwrap().as_secs_f64();
    if lo == 0.0 { 0.0 } else { 100.0 * (hi / lo - 1.0) }
}

fn time<T>(f: impl FnOnce() -> T) -> Duration {
    let start = Instant::now();
    let r = f();
    let e = start.elapsed();
    std::hint::black_box(&r);
    drop(r);
    e
}

/// Generated and table, alternating which goes first each round.
fn ab(rounds: usize, mut generated: impl FnMut(), mut tab: impl FnMut()) -> (Vec<Duration>, Vec<Duration>) {
    generated();
    tab();
    let (mut tg, mut tt) = (Vec::new(), Vec::new());
    for round in 0..rounds {
        if round % 2 == 0 {
            tg.push(time(&mut generated));
            tt.push(time(&mut tab));
        } else {
            tt.push(time(&mut tab));
            tg.push(time(&mut generated));
        }
    }
    (tg, tt)
}

fn ms(d: Duration) -> f64 {
    d.as_secs_f64() * 1e3
}

fn main() {
    let mut rounds = 11usize;
    let mut dirs: Vec<PathBuf> = Vec::new();
    let mut args = std::env::args().skip(1);
    while let Some(a) = args.next() {
        if a == "-r" {
            rounds = args.next().and_then(|s| s.parse().ok()).expect("-r <rounds>");
        } else {
            dirs.push(a.into());
        }
    }
    if dirs.is_empty() {
        dirs.push("CGMES-Test-Configurations/v3.0/RealGrid/RealGrid-Merged".into());
    }

    let t0 = Instant::now();
    let (shapes, skips) = build_table();
    let mut per_tag: BTreeMap<&str, (usize, usize)> = BTreeMap::new();
    for s in shapes {
        for p in s.profiles {
            let e = per_tag.entry(p).or_default();
            e.0 += 1;
            e.1 += s.props.iter().map(|p| p.checks.len()).sum::<usize>();
        }
    }
    let checks: usize = shapes.iter().flat_map(|s| s.props.iter()).map(|p| p.checks.len()).sum();
    println!(
        "table: {} shapes, {checks} checks, {skips} skips, built in {:.0?}{}",
        shapes.len(),
        t0.elapsed(),
        format!(
            "{}{}",
            if std::env::var_os("BAG").is_some() { "  [BAG=1: datatype/nodeKind kept]" } else { "" },
            if std::env::var_os("EXACT_TARGETS").is_some() {
                "  [EXACT_TARGETS=1: literal targetClass only, as generated]"
            } else {
                ""
            },
        )
    );
    let tags: Vec<String> = per_tag.iter().map(|(t, (s, c))| format!("{t}={s}/{c}")).collect();
    println!("  shapes/checks per tag: {}\n", tags.join(" "));
    if let Some(rule) = std::env::var_os("DUMP_RULE") {
        let rule = rule.to_string_lossy().into_owned();
        for s in shapes {
            for p in s.props {
                for c in p.checks.iter().filter(|c| c.rule_id == rule) {
                    println!("{rule}: {:?} on {:?}, targets {:?}, tags {:?}", c.constraint, p.path, s.targets, s.profiles);
                }
            }
        }
    }
    report_coverage(shapes);
    let table = Table { shapes };

    let mut diff_out = String::new();
    let mut parity_ok = true;

    for dir in &dirs {
        let c = load_corpus(dir);
        println!("=== {} ===", c.name);
        println!(
            "{} files, {} elements, profiles {:?}, solved={}",
            c.per_file.len(),
            c.elements,
            c.cfg.profiles,
            c.cfg.solved
        );
        println!(
            "{:<6} {:>9} {:>9} {:>7} {:>10} {:>9} {:>9} {:>8} {:>8} {:>8}",
            "unit", "gen ms", "tab ms", "tab/gen", "sparql ms", "gen viol", "tab viol", "only gen", "only tab", "relabel"
        );

        let (mut sum_g, mut sum_t, mut sum_s) = (0.0, 0.0, 0.0);
        for u in units(&c) {
            let gv = run_generated(&c, &u);
            let tv = run_tabled(&c, &table, &u);
            let (only_gen, only_tab, relabelled) = diff(&gv, &tv);

            let (tg, tt) =
                ab(rounds, || drop(run_generated(&c, &u)), || drop(run_tabled(&c, &table, &u)));
            let ts = median((0..rounds.min(5)).map(|_| time(|| run_sparql(&c, &u))).collect());

            let (mg, mt) = (median(tg.clone()), median(tt.clone()));
            sum_g += ms(mg);
            sum_t += ms(mt);
            sum_s += ms(ts);
            let n_gen: i64 = only_gen.iter().map(|x| x.1).sum();
            let n_tab: i64 = only_tab.iter().map(|x| x.1).sum();
            println!(
                "{:<6} {:>9.2} {:>9.2} {:>7.2} {:>10.2} {:>9} {:>9} {:>8} {:>8} {:>8}   spread g{:.0}% t{:.0}%",
                u.label(),
                ms(mg),
                ms(mt),
                if ms(mg) > 0.0 { ms(mt) / ms(mg) } else { f64::NAN },
                ms(ts),
                gv.len(),
                tv.len(),
                n_gen,
                n_tab,
                relabelled,
                spread(&tg),
                spread(&tt),
            );
            if n_gen + n_tab > 0 {
                parity_ok = false;
                for (side, xs) in [("gen", &only_gen), ("tab", &only_tab)] {
                    for (rule, n, sample) in by_rule(xs).into_iter().take(8) {
                        println!("         only {side}: {n:>6}  {rule}  (e.g. {sample})");
                    }
                    for ((obj, rule, prop), n) in xs.iter() {
                        let _ = writeln!(diff_out, "{}\t{}\t{side}\t{rule}\t{prop}\t{obj}\t{n}", c.name, u.label());
                    }
                }
            }
        }

        let tx = median(
            (0..rounds.min(5)).map(|_| time(|| cimvalidation::sparql::validate_crossprofile(&c.merged, &c.cfg))).collect(),
        );
        sum_s += ms(tx);
        println!(
            "{:<6} {:>9.2} {:>9.2} {:>7.2} {:>10.2}   (sum of unit medians; sparql incl. {:.2} ms cross-profile)",
            "total",
            sum_g,
            sum_t,
            sum_t / sum_g,
            sum_s,
            ms(tx),
        );

        // The whole SHACL pipeline end to end, interleaved, as a cross-check on
        // the sum of unit medians.
        let all = units(&c);
        let (tg, tt) = ab(
            rounds,
            || {
                for u in &all {
                    drop(run_generated(&c, u));
                }
            },
            || {
                for u in &all {
                    drop(run_tabled(&c, &table, u));
                }
            },
        );
        let (mg, mt) = (median(tg.clone()), median(tt.clone()));
        println!(
            "pipeline (interleaved, {rounds} rounds): gen {:.2} ms, table {:.2} ms, table/gen {:.3}  spread g{:.0}% t{:.0}%\n",
            ms(mg),
            ms(mt),
            ms(mt) / ms(mg),
            spread(&tg),
            spread(&tt)
        );
    }

    if let Some(path) = std::env::var_os("DIFF_OUT") {
        std::fs::write(&path, diff_out).expect("cannot write DIFF_OUT");
        println!("unmatched violations written to {}", PathBuf::from(path).display());
    }
    println!("parity: {}", if parity_ok { "IDENTICAL on every unit" } else { "DIFFERENCES (see above)" });
}
