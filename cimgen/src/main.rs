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
        run_shacl(&spec, &glob, &out_dir, verbose, skip_report);
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
    eprintln!(
        "shacl codegen: {} files, {} checks, {} skipped → {out_dir}",
        results.len(), total_checks, total_skipped
    );

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
}

