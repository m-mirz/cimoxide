mod generator;
mod schema;

use std::path::Path;

const DEFAULT_SCHEMA: &str =
    "cimgo/application-profiles-library/CGMES/CurrentRelease/RDFS/61970-600-2_*-AP-Voc-RDFS2020.rdf";
const DEFAULT_OUTPUT: &str = "cimstructs/src";

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let mut schema = DEFAULT_SCHEMA.to_string();
    let mut output = DEFAULT_OUTPUT.to_string();
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
}
