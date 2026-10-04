mod generator;
mod shacl;

// Keeps `crate::schema::...` working across the generator and shacl modules.
use cimschema as schema;

use std::path::Path;

const DEFAULT_OUTPUT: &str = "cimstructs/src";
const DEFAULT_SHACL: &str =
    "application-profiles-library/CGMES/SHACL/*.ttl";
const DEFAULT_NC_SHACL: &str = "application-profiles-library/NCP/SHACL/*.ttl";
const DEFAULT_SHACL_OUTPUT: &str = "cimvalidation/src";
const DEFAULT_SPARQL_DIR: &str = "cimvalidation/src/sparql";
const DEFAULT_PYTHON_STUBS_OUTPUT: &str = "cimoxide-py/python/cimoxide";

fn print_usage() {
    eprintln!(
        "cimgen — generate Rust sources from ENTSO-E RDFS and SHACL schemas

Options:
  --schema <glob>               CGMES RDFS glob
  --nc-schema <glob>            NCP RDFS glob
  --families cgmes,nc           families to generate (default: all)
  --output <dir>                where generated structs are written
  --shacl <glob>                SHACL TTL glob
  --nc-shacl <glob>             NCP SHACL TTL glob (bag-family shape table)
  --shacl-output <dir>          where the shape tables are written
  --python-stubs-output <dir>   where types.pyi is written
  --skip-shacl                  do not generate the shape tables
  --skip-python-stubs           do not generate Python stubs
  --skip-report                 suppress the SHACL skip report
  --rule-report                 print the SPARQL rule coverage report
  --verbose, -v                 log each file as it is parsed
  --help, -h                    show this message"
    );
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let mut schema = schema::family::CGMES.default_schema.to_string();
    let mut nc_schema = schema::family::NC.default_schema.to_string();
    let mut families: Vec<&'static schema::family::Family> =
        schema::family::FAMILIES.to_vec();
    let mut output = DEFAULT_OUTPUT.to_string();
    let mut shacl_glob: Option<String> = Some(DEFAULT_SHACL.to_string());
    let mut nc_shacl_glob = DEFAULT_NC_SHACL.to_string();
    let mut shacl_output: Option<String> = Some(DEFAULT_SHACL_OUTPUT.to_string());
    let mut python_stubs_output: Option<String> = Some(DEFAULT_PYTHON_STUBS_OUTPUT.to_string());
    let mut verbose = false;
    let mut skip_report = false;
    let mut rule_report = false;

    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--schema" => {
                i += 1;
                schema = args.get(i).cloned().unwrap_or_default();
            }
            "--nc-schema" => {
                i += 1;
                nc_schema = args.get(i).cloned().unwrap_or_default();
            }
            "--families" => {
                i += 1;
                let list = args.get(i).cloned().unwrap_or_default();
                families = Vec::new();
                for name in list.split(',').filter(|s| !s.is_empty()) {
                    match schema::family::by_id(name) {
                        Some(f) => families.push(f),
                        None => {
                            eprintln!("unknown family: {name}");
                            std::process::exit(1);
                        }
                    }
                }
            }
            "--output" => {
                i += 1;
                output = args.get(i).cloned().unwrap_or_default();
            }
            "--shacl" => {
                i += 1;
                shacl_glob = args.get(i).cloned();
            }
            "--nc-shacl" => {
                i += 1;
                nc_shacl_glob = args.get(i).cloned().unwrap_or_default();
            }
            "--shacl-output" => {
                i += 1;
                shacl_output = args.get(i).cloned();
            }
            "--python-stubs-output" => {
                i += 1;
                python_stubs_output = args.get(i).cloned();
            }
            "--verbose" | "-v" => verbose = true,
            "--skip-report" => skip_report = true,
            "--rule-report" => rule_report = true,
            "--skip-shacl" => shacl_output = None,
            "--skip-python-stubs" => python_stubs_output = None,
            "--help" | "-h" => {
                print_usage();
                return;
            }
            other => {
                eprintln!("unknown argument: {other}");
                std::process::exit(1);
            }
        }
        i += 1;
    }

    if verbose {
        eprintln!("schema pattern : {schema}");
        eprintln!("output dir     : {output}");
    }

    if !families.iter().any(|f| f.typed) {
        eprintln!("--families must include the typed family (cgmes)");
        std::process::exit(1);
    }

    let mut spec = match schema::import::import_schema_files(&schema, &schema::family::CGMES, verbose) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("error importing schema: {e}");
            std::process::exit(1);
        }
    };

    // Property-bag families are imported into their own specification: the
    // prefix-to-namespace maps collide (both bind `cim`, to different IRIs), so
    // a merged import would silently mis-namespace whichever parsed second.
    let mut bag_specs: Vec<schema::model::CimSpecification> = Vec::new();
    for family in families.iter().filter(|f| !f.typed) {
        let pattern = if family.id == "nc" { &nc_schema } else { family.default_schema };
        match schema::import::import_schema_files(pattern, family, verbose) {
            Ok(s) => bag_specs.push(s),
            Err(e) => {
                eprintln!("error importing {} schema: {e}", family.id);
                std::process::exit(1);
            }
        }
    }
    let bags: Vec<&schema::model::CimSpecification> = bag_specs.iter().collect();

    if verbose {
        eprintln!(
            "parsed {} types, {} enums, {} datatypes",
            spec.types.len(),
            spec.enums.len(),
            spec.cim_datatypes.len()
        );
    }

    if let Err(e) = generator::rust_gen::generate_rust(&mut spec, &bags, Path::new(&output)) {
        eprintln!("error generating code: {e}");
        std::process::exit(1);
    }

    eprintln!(
        "generated {} structs, {} enums into {output}",
        spec.types.len(),
        spec.enums.len()
    );
    for bag in &bags {
        eprintln!(
            "generated {} {} classes into {output}/{}_classes.rs",
            bag.types.len(),
            bag.family.id,
            bag.family.id
        );
    }

    if let (Some(glob), Some(out_dir)) = (shacl_glob.clone(), shacl_output.clone()) {
        // The CGMES shape table, resolved with the bag families alongside for
        // the cross-family class lists.
        run_bag_shacl(&spec, &bags, &glob, &out_dir, verbose, skip_report);
        if skip_report || rule_report {
            run_cgmes_reports(&spec, &bags, &glob, verbose, skip_report, rule_report);
        }
    }

    // Bag families get a shape *table* rather than generated check functions:
    // there is no struct to downcast to, so nothing to generate against.
    if let Some(out_dir) = shacl_output {
        // The typed family's spec goes along: NCP's association value-type
        // lists name CGMES classes beside NC ones, and a reference can decode
        // as either.
        for bag in &bags {
            run_bag_shacl(bag, &[&spec], &nc_shacl_glob, &out_dir, verbose, skip_report);
        }
    }

    if let Some(out_dir) = python_stubs_output {
        if let Err(e) =
            generator::python_stubs_gen::generate_python_stubs(&spec, Path::new(&out_dir))
        {
            eprintln!("error generating Python stubs: {e}");
            std::process::exit(1);
        }
        eprintln!("python stubs: types.pyi → {out_dir}");
    }
}

/// The `--skip-report` and `--rule-report` output for the CGMES shape table.
///
/// Counts are per constraint file, so each file is resolved on its own here and
/// its simplification and resolution skips attributed to it; the table itself
/// is resolved in one pass by [`run_bag_shacl`], through the same
/// `resolve_shapes`. Every TTL file is counted, whether or not the manifest
/// runs it, so a file's total does not depend on dispatch.
fn run_cgmes_reports(
    spec: &schema::model::CimSpecification,
    others: &[&schema::model::CimSpecification],
    glob: &str,
    verbose: bool,
    skip_report: bool,
    rule_report: bool,
) {
    let pattern = glob::Pattern::new(glob).unwrap_or_else(|e| {
        eprintln!("invalid shacl glob pattern: {e}");
        std::process::exit(1);
    });

    let ttl_dir = std::path::Path::new(glob)
        .parent()
        .unwrap_or(std::path::Path::new("."));

    let entries = std::fs::read_dir(ttl_dir).unwrap_or_else(|e| {
        eprintln!("cannot read shacl directory {}: {e}", ttl_dir.display());
        std::process::exit(1);
    });

    let mut ttl_paths: Vec<std::path::PathBuf> = entries
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .filter(|p| {
            p.extension().and_then(|x| x.to_str()) == Some("ttl")
                && pattern.matches_path(p)
        })
        .collect();
    ttl_paths.sort();

    if verbose {
        eprintln!("found {} SHACL TTL files", ttl_paths.len());
    }

    let mut results: Vec<shacl::model::FileResults> = Vec::new();
    for path in &ttl_paths {
        match shacl::ttl_import::import_ttl_file(path) {
            Ok(fr) => results.push(fr),
            Err(e) => {
                eprintln!("warning: skipping {}: {e}", path.display());
            }
        }
    }

    let mut simplify_skips: std::collections::HashMap<String, Vec<shacl::skip::SkipEntry>> =
        shacl::simplify::simplify(&mut results, &schema::family::CGMES).into_iter().collect();

    // Resolve every file, under its manifest tags or a placeholder one.
    let mut profiles_of: std::collections::HashMap<String, Vec<String>> = std::collections::HashMap::new();
    for (stem, tag) in schema::family::CGMES.shacl_manifest.unwrap_or(&[]) {
        profiles_of.entry((*stem).to_string()).or_default().push((*tag).to_string());
    }
    for fr in &results {
        profiles_of.entry(fr.file_name.clone()).or_insert_with(|| vec!["-".to_string()]);
    }

    let mut file_skips: Vec<shacl::skip::FileSkipInfo> = Vec::new();
    let mut total_checks = 0;
    for fr in &results {
        let mut collector = shacl::skip::SkipCollector::new();
        let (shapes, _) = schema::shacl::resolve::resolve_shapes(
            spec, others, std::slice::from_ref(fr), &profiles_of, &mut collector,
        );
        // Distinct `(path, component, sh:name)` rule patterns, the unit the
        // skips are counted in too: a property shape the file's node shapes
        // share counts once, not once per node shape. Checked + skipped is
        // then the number of constraints the file defines, independent of
        // what either side can check.
        let mut patterns: std::collections::HashSet<String> = std::collections::HashSet::new();
        for s in &shapes {
            for p in &s.props {
                for c in &p.checks {
                    let kind = format!("{:?}", c.constraint);
                    let kind = kind.split('(').next().unwrap_or("");
                    patterns.insert(format!("{:?}|{kind}|{}", p.path, c.name));
                }
            }
            for l in &s.logic {
                patterns.insert(format!("logic|{:?}|{}", l.op, l.name));
            }
            if let Some(c) = &s.closed {
                patterns.insert(format!("closed|{}", c.rule_id));
            }
        }
        let check_count = patterns.len();
        total_checks += check_count;
        let mut skips = collector.into_entries();
        skips.extend(simplify_skips.remove(&fr.file_name).unwrap_or_default());
        file_skips.push(shacl::skip::FileSkipInfo { file_name: fr.file_name.clone(), check_count, skips });
    }

    let total_skipped: usize = file_skips.iter().map(|f| f.skips.len()).sum();
    if !rule_report {
        // Under --rule-report, this exact total (and more, broken down by profile) is
        // already in the "SHACL Rules by Profile" table below.
        eprintln!(
            "cgmes shape report: {} files, {} checks, {} skipped",
            results.len(), total_checks, total_skipped
        );
    }

    if skip_report {
        // Per-file totals line for every file (checks + skips), parseable for comparison.
        for fi in &file_skips {
            eprintln!("PERFILE\t{}\t{}\t{}", fi.file_name, fi.check_count, fi.skips.len());
        }

        let mut global_counts: std::collections::HashMap<&'static str, usize> =
            std::collections::HashMap::new();
        for fi in &file_skips {
            for e in &fi.skips {
                eprintln!("{}\t{}", fi.file_name, e);
            }
            shacl::skip::print_file_summary(&fi.file_name, fi.check_count, &fi.skips);
            shacl::skip::accumulate_counts(&mut global_counts, &fi.skips);
        }
        shacl::skip::print_global_summary(&global_counts);
    }

    if rule_report {
        // B1 — Skipped-constraints counts (shape table side): same category totals as
        // --skip-report, exposed without needing the verbose per-entry dump too.
        let mut global_counts: std::collections::HashMap<&'static str, usize> =
            std::collections::HashMap::new();
        for fi in &file_skips {
            shacl::skip::accumulate_counts(&mut global_counts, &fi.skips);
        }
        eprintln!("\n########## README rule-count report ##########");
        shacl::skip::print_global_summary(&global_counts);

        // B1.5 — Shape-table rule counts (checks + skips), grouped by profile the same
        // way as the SPARQL Check Coverage table below, using the already-computed
        // per-file check_count/skips from file_skips.
        let mut group_checks: std::collections::HashMap<&'static str, usize> = std::collections::HashMap::new();
        let mut group_skipped: std::collections::HashMap<&'static str, usize> = std::collections::HashMap::new();
        for fi in &file_skips {
            let group = shacl::ttl_report::ttl_group_label(&fi.file_name);
            *group_checks.entry(group).or_insert(0) += fi.check_count;
            *group_skipped.entry(group).or_insert(0) += fi.skips.len();
        }
        eprintln!("\n=== SHACL Rules by Profile (shape table) ===");
        eprintln!("  {:32}  {:>9}  {:>7}  {:>6}", "Profile Group", "Checks", "Skipped", "Total");
        let (mut gen_total, mut skip_total) = (0usize, 0usize);
        for g in shacl::ttl_report::TTL_GROUP_LABEL_ORDER {
            let checks = group_checks.get(g).copied().unwrap_or(0);
            let skipped = group_skipped.get(g).copied().unwrap_or(0);
            gen_total += checks;
            skip_total += skipped;
            eprintln!("  {:32}  {:9}  {:7}  {:6}", g, checks, skipped, checks + skipped);
        }
        eprintln!("  -----");
        eprintln!("  {:32}  {:9}  {:7}  {:6}", "Total", gen_total, skip_total, gen_total + skip_total);

        // Per-file breakdown, for diffing directly against cimgo's -rule-report output (same
        // PERFILE\t<name>\t<checks>\t<skipped>\t<total> line format on both sides): `grep
        // PERFILE cimoxide.log | sort > a; grep PERFILE cimgo.log | sort > b; diff a b` finds
        // every field-level difference, or to compare just the per-file Total (the meaningful
        // cross-tool check -- Generated vs Skipped legitimately differs by codegen capability
        // even when Total agrees): `awk -F'\t' '{print $2, $5}' a | diff - <(awk -F'\t' '{print
        // $2, $5}' b)`. No external script needed either way.
        eprintln!("\n=== Per-File Rule Counts (grep PERFILE to diff against cimgo) ===");
        let mut per_file: Vec<&shacl::skip::FileSkipInfo> = file_skips.iter().collect();
        per_file.sort_by(|a, b| a.file_name.cmp(&b.file_name));
        for fi in &per_file {
            eprintln!(
                "PERFILE\t{}\t{}\t{}\t{}",
                fi.file_name,
                fi.check_count,
                fi.skips.len(),
                fi.check_count + fi.skips.len()
            );
        }

        // B2 — SPARQL Check Coverage (hand-written side): distinct sh:names reachable per
        // profile group, from a call-graph analysis of cimvalidation/src/sparql/*.rs, matched
        // against the SPARQL constraint shapes actually defined in the CGMES SHACL TTL files
        // (already parsed into `results` above) to produce a real Implemented/Total/Coverage
        // figure instead of counting hand-written check functions.
        let groups = shacl::sparql_report::report(std::path::Path::new(DEFAULT_SPARQL_DIR));
        let ttl = shacl::ttl_report::ttl_sparql_names(&results);
        let rows = shacl::ttl_report::combine_coverage(&groups, &ttl);

        eprintln!("\n=== SPARQL Check Coverage (cimvalidation/src/sparql vs {glob}) ===");
        eprintln!("  {:32}  {:>11}  {:>9}  {:>8}", "Profile Group", "Implemented", "TTL Total", "Coverage");
        let (mut total_impl, mut total_ttl) = (0usize, 0usize);
        for r in &rows {
            match r.ttl_total {
                Some(ttl_total) => {
                    total_impl += r.implemented;
                    total_ttl += ttl_total;
                    let coverage = 100.0 * r.implemented as f64 / ttl_total as f64;
                    eprintln!("  {:32}  {:11}  {:9}  {:7.1}%", r.label, r.implemented, ttl_total, coverage);
                }
                None => {
                    eprintln!("  {:32}  {:11}  {:>9}  {:>8}", r.label, r.implemented, "n/a", "n/a");
                }
            }
        }
        eprintln!("  -----");
        if total_ttl > 0 {
            let coverage = 100.0 * total_impl as f64 / total_ttl as f64;
            eprintln!("  {:32}  {:11}  {:9}  {:7.1}%", "Total", total_impl, total_ttl, coverage);
        }

        for r in &rows {
            if r.missing.is_empty() { continue; }
            eprintln!("\n  Not yet implemented in {}:", r.label);
            for m in &r.missing {
                eprintln!("    {m}");
            }
        }

        if verbose {
            eprintln!("\n=== Implemented names (cimvalidation/src/sparql) ===");
            for g in &groups {
                eprintln!("  {} ({} names)", g.label, g.names.len());
                for n in &g.names {
                    eprintln!("    {n}");
                }
            }
        }
    }
}


/// Generate the shape table for one family.
///
/// The profile-to-file mapping comes from `Family::shacl_manifest` where the
/// family has one (CGMES), and otherwise is read: `NCP/SHACL/Validation/`
/// ships one manifest per profile whose `owl:imports` list names exactly the
/// constraint files that apply.
fn run_bag_shacl(
    spec: &schema::model::CimSpecification,
    others: &[&schema::model::CimSpecification],
    glob: &str,
    out_dir: &str,
    verbose: bool,
    skip_report: bool,
) {
    let shacl_dir = std::path::Path::new(glob)
        .parent()
        .unwrap_or(std::path::Path::new("."));

    let mut collector = shacl::skip::SkipCollector::new();
    let table = match schema::shacl::resolve::load_shape_table(
        spec.family,
        spec,
        others,
        shacl_dir,
        &mut collector,
    ) {
        Ok(t) => t,
        Err(e) => {
            eprintln!("warning: skipping {} shapes: {e}", spec.family.id);
            return;
        }
    };

    let write = |name: &str, code: String| {
        let path = std::path::Path::new(out_dir).join(name);
        if let Err(e) = std::fs::write(&path, code) {
            eprintln!("error writing {}: {e}", path.display());
            std::process::exit(1);
        }
        path
    };

    let shapes_path = write(
        &format!("{}_shapes.rs", spec.family.id),
        generator::shapes_gen::render_shapes(spec.family.id, &table.shapes),
    );
    // A family with a written-out manifest detects its profiles from its own
    // headers and has no profile index to emit.
    let profiles_path = spec.family.shacl_manifest.is_none().then(|| {
        write(
            &format!("{}_profiles.rs", spec.family.id),
            generator::shapes_gen::render_profiles_from(&table.profile_iris, &table.profiles),
        )
    });

    let skips = collector.into_entries();
    eprintln!(
        "{} shapes: {} shapes, {} checks, {} closed, {} skipped → {}",
        spec.family.id,
        table.stats.shapes,
        table.stats.checks,
        table.stats.closed,
        skips.len(),
        shapes_path.display(),
    );
    if let Some(profiles_path) = profiles_path {
        eprintln!(
            "{} profiles: {} descriptors → {}",
            spec.family.id,
            table.profiles.len(),
            profiles_path.display()
        );
    }

    if skip_report {
        for e in &skips {
            eprintln!("{}\t{e}", spec.family.id);
        }
    }
    // CGMES gets the per-file breakdown from `run_cgmes_reports` instead.
    if (verbose || skip_report) && spec.family.shacl_manifest.is_none() {
        let mut counts: std::collections::HashMap<&'static str, usize> =
            std::collections::HashMap::new();
        shacl::skip::accumulate_counts(&mut counts, &skips);
        shacl::skip::print_global_summary(&counts);
    }
}

