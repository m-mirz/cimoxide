mod generator;
mod schema;
mod shacl;

use std::path::Path;

const DEFAULT_SCHEMA: &str =
    "application-profiles-library/CGMES/CurrentRelease/RDFS/61970-600-2_*-AP-Voc-RDFS2020.rdf";
const DEFAULT_OUTPUT: &str = "cimstructs/src";

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let mut schema = DEFAULT_SCHEMA.to_string();
    let mut output = DEFAULT_OUTPUT.to_string();
    let mut shacl_glob: Option<String> = None;
    let mut shacl_output: Option<String> = None;
    let mut python_stubs_output: Option<String> = None;
    let mut verbose = false;

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
        run_shacl(&spec, &glob, &out_dir, verbose);
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

    shacl::simplify::simplify(&mut results);

    if let Err(e) = shacl::codegen::generate_validation(&results, spec, Path::new(out_dir)) {
        eprintln!("error generating SHACL validation code: {e}");
        std::process::exit(1);
    }

    eprintln!("shacl codegen: {} files → {out_dir}", results.len());
}

