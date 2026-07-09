mod generator;
mod schema;
mod shacl;

use std::path::Path;

const DEFAULT_SCHEMA: &str =
    "application-profiles-library/CGMES/CurrentRelease/RDFS/61970-600-2_*-AP-Voc-RDFS2020.rdf";
const DEFAULT_OUTPUT: &str = "cimstructs/src";
const DEFAULT_SHACL: &str =
    "application-profiles-library/CGMES/CurrentRelease/SHACL/*.ttl";
const DEFAULT_SHACL_OUTPUT: &str = "cimvalidation/src";
const DEFAULT_SPARQL_DIR: &str = "cimvalidation/src/sparql";
const DEFAULT_PYTHON_STUBS_OUTPUT: &str = "cimoxide-py/python/cimoxide";

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let mut schema = DEFAULT_SCHEMA.to_string();
    let mut output = DEFAULT_OUTPUT.to_string();
    let mut shacl_glob: Option<String> = Some(DEFAULT_SHACL.to_string());
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
            "--output" => {
                i += 1;
                output = args.get(i).cloned().unwrap_or_default();
            }
            "--shacl" => {
                i += 1;
                shacl_glob = args.get(i).cloned();
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

    let mut spec = match schema::import::import_schema_files(&schema, verbose) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("error importing schema: {e}");
            std::process::exit(1);
        }
    };

    if verbose {
        eprintln!(
            "parsed {} types, {} enums, {} datatypes",
            spec.types.len(),
            spec.enums.len(),
            spec.cim_datatypes.len()
        );
    }

    if let Err(e) = generator::rust_gen::generate_rust(&mut spec, Path::new(&output)) {
        eprintln!("error generating code: {e}");
        std::process::exit(1);
    }

    eprintln!(
        "generated {} structs, {} enums into {output}",
        spec.types.len(),
        spec.enums.len()
    );

    if let (Some(glob), Some(out_dir)) = (shacl_glob, shacl_output) {
        run_shacl(&spec, &glob, &out_dir, verbose, skip_report, rule_report);
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

fn run_shacl(
    spec: &schema::model::CimSpecification,
    glob: &str,
    out_dir: &str,
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

    let simplify_skips = shacl::simplify::simplify(&mut results);

    let (total_checks, mut file_skips) = match shacl::codegen::generate_validation(&results, spec, Path::new(out_dir)) {
        Ok(r) => r,
        Err(e) => {
            eprintln!("error generating SHACL validation code: {e}");
            std::process::exit(1);
        }
    };

    // Merge simplify-stage skips into the per-file skip info.
    for (file_name, s_skips) in simplify_skips {
        if s_skips.is_empty() { continue; }
        if let Some(fi) = file_skips.iter_mut().find(|f| f.file_name == file_name) {
            fi.skips.extend(s_skips);
        } else {
            file_skips.push(shacl::skip::FileSkipInfo { file_name, check_count: 0, skips: s_skips });
        }
    }

    let total_skipped: usize = file_skips.iter().map(|f| f.skips.len()).sum();
    if !rule_report {
        // Under --rule-report, this exact total (and more, broken down by profile) is
        // already in the "Generated SHACL Rules by Profile" table below.
        eprintln!(
            "shacl codegen: {} files, {} checks, {} skipped → {out_dir}",
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
        // B1 — Skipped-constraints counts (generated SHACL side): same category totals as
        // --skip-report, exposed without needing the verbose per-entry dump too.
        let mut global_counts: std::collections::HashMap<&'static str, usize> =
            std::collections::HashMap::new();
        for fi in &file_skips {
            shacl::skip::accumulate_counts(&mut global_counts, &fi.skips);
        }
        eprintln!("\n########## README rule-count report ##########");
        shacl::skip::print_global_summary(&global_counts);

        // B1.5 — Generated SHACL rule counts (checks + skips), grouped by profile the same
        // way as the SPARQL Check Coverage table below, using the already-computed
        // per-file check_count/skips from file_skips.
        let mut group_checks: std::collections::HashMap<&'static str, usize> = std::collections::HashMap::new();
        let mut group_skipped: std::collections::HashMap<&'static str, usize> = std::collections::HashMap::new();
        for fi in &file_skips {
            let group = shacl::ttl_report::ttl_group_label(&fi.file_name);
            *group_checks.entry(group).or_insert(0) += fi.check_count;
            *group_skipped.entry(group).or_insert(0) += fi.skips.len();
        }
        eprintln!("\n=== Generated SHACL Rules by Profile ===");
        eprintln!("  {:32}  {:>9}  {:>7}  {:>6}", "Profile Group", "Generated", "Skipped", "Total");
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

