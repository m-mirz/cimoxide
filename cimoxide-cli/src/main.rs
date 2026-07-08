mod convert;

use std::path::PathBuf;
use std::process;

/// Unwrap a `Result`, or print `"{context}: {error}"` to stderr and exit(1).
fn or_die<T, E: std::fmt::Display>(result: Result<T, E>, context: &str) -> T {
    result.unwrap_or_else(|e| {
        eprintln!("{context}: {e}");
        process::exit(1);
    })
}

fn usage() -> ! {
    eprintln!("Usage:");
    eprintln!("  cimoxide-cli import [--json] <xml-files...>");
    eprintln!("  cimoxide-cli convert --to json <xml-files...> [--out <output.json>]");
    eprintln!("  cimoxide-cli convert --to xml  <input.json>   [--out <output.xml>]");
    eprintln!("  cimoxide-cli convert --to xml  <input.json>   --profile EQ,SSH [--out <dir/>]");
    eprintln!("  cimoxide-cli validate [--profiles EQ,SSH,...] [--solved] [--not-solved]");
    eprintln!("                        [--common] [--quality] [--silence rule1,rule2]");
    eprintln!("                        [--format json|text] <xml-files...>");
    process::exit(1);
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.is_empty() {
        usage();
    }
    if args[0] == "import" {
        cmd_import(&args[1..]);
        return;
    }
    if args[0] == "validate" {
        cmd_validate(&args[1..]);
        return;
    }
    if args[0] != "convert" {
        usage();
    }

    let mut to_format: Option<String> = None;
    let mut out_path: Option<PathBuf> = None;
    let mut profile_str: Option<String> = None;
    let mut input_files: Vec<PathBuf> = Vec::new();

    let mut i = 1usize;
    while i < args.len() {
        match args[i].as_str() {
            "--to" => {
                i += 1;
                if i >= args.len() {
                    eprintln!("error: --to requires an argument");
                    usage();
                }
                to_format = Some(args[i].clone());
            }
            "--out" => {
                i += 1;
                if i >= args.len() {
                    eprintln!("error: --out requires an argument");
                    usage();
                }
                out_path = Some(PathBuf::from(&args[i]));
            }
            "--profile" => {
                i += 1;
                if i >= args.len() {
                    eprintln!("error: --profile requires an argument");
                    usage();
                }
                profile_str = Some(args[i].clone());
            }
            arg if !arg.starts_with('-') => {
                input_files.push(PathBuf::from(arg));
            }
            unknown => {
                eprintln!("error: unknown flag: {unknown}");
                usage();
            }
        }
        i += 1;
    }

    let format = to_format.as_deref().unwrap_or_else(|| {
        eprintln!("error: --to <json|xml> is required");
        usage();
    });

    if input_files.is_empty() {
        eprintln!("error: at least one input file is required");
        usage();
    }

    match format {
        "json" => cmd_to_json(&input_files, out_path.as_deref()),
        "xml" => {
            let profiles: Vec<&str> = profile_str
                .as_deref()
                .map(|s| s.split(',').collect())
                .unwrap_or_default();
            cmd_to_xml(&input_files[0], out_path.as_deref(), &profiles);
        }
        other => {
            eprintln!("error: unknown format '{other}', expected 'json' or 'xml'");
            usage();
        }
    }
}

fn cmd_to_json(xml_files: &[PathBuf], out: Option<&std::path::Path>) {
    let paths: Vec<&std::path::Path> = xml_files.iter().map(PathBuf::as_path).collect();
    let ds = or_die(cimdecoder::CimDataset::decode_files_parallel(&paths), "error decoding XML");
    let json = convert::dataset_to_json(&ds);
    let text = or_die(serde_json::to_string_pretty(&json), "error serializing JSON");
    write_output(&text, out);
}

fn cmd_to_xml(json_file: &std::path::Path, out: Option<&std::path::Path>, profiles: &[&str]) {
    let src = or_die(
        std::fs::read_to_string(json_file),
        &format!("error reading {}", json_file.display()),
    );
    let ds = or_die(convert::dataset_from_json(&src), "error parsing JSON");

    if profiles.is_empty() {
        let xml = or_die(convert::dataset_to_xml(&ds), "error generating XML");
        write_output(&xml, out);
        return;
    }

    if profiles.len() > 1 {
        let dir = out.unwrap_or_else(|| {
            eprintln!("error: --out <dir/> is required when multiple --profile codes are given");
            usage();
        });
        if !dir.is_dir() {
            or_die(
                std::fs::create_dir_all(dir),
                &format!("error creating directory {}", dir.display()),
            );
        }
        for &code in profiles {
            let xml = or_die(
                convert::dataset_to_xml_for_profile(&ds, code),
                &format!("error generating XML for profile {code}"),
            );
            let path = dir.join(format!("{code}.xml"));
            or_die(std::fs::write(&path, &xml), &format!("error writing {}", path.display()));
        }
    } else {
        let code = profiles[0];
        let xml = or_die(
            convert::dataset_to_xml_for_profile(&ds, code),
            &format!("error generating XML for profile {code}"),
        );
        write_output(&xml, out);
    }
}

fn write_output(text: &str, out: Option<&std::path::Path>) {
    match out {
        Some(path) => or_die(std::fs::write(path, text), &format!("error writing {}", path.display())),
        None => print!("{text}"),
    }
}

fn cmd_import(args: &[String]) {
    let mut input_files: Vec<PathBuf> = Vec::new();
    let mut output_json = false;

    let mut i = 0usize;
    while i < args.len() {
        match args[i].as_str() {
            "--json" => { output_json = true; }
            arg if !arg.starts_with('-') => {
                input_files.push(PathBuf::from(arg));
            }
            unknown => {
                eprintln!("error: unknown flag: {unknown}");
                usage();
            }
        }
        i += 1;
    }

    if input_files.is_empty() {
        eprintln!("error: at least one input file is required");
        usage();
    }

    struct FileResult { name: String, count: usize }

    // Decode all files in parallel, collecting per-file counts before merging.
    let paths: Vec<&std::path::Path> = input_files.iter().map(PathBuf::as_path).collect();
    let datasets = or_die(
        cimdecoder::CimDataset::decode_files_parallel_separate(&paths),
        "error decoding input",
    );

    let mut per_file: Vec<FileResult> = Vec::new();
    let mut combined = cimdecoder::CimDataset::new();
    for (path, ds) in input_files.iter().zip(datasets) {
        let name = path.file_name()
            .map(|n| n.to_string_lossy().into_owned())
            .unwrap_or_else(|| path.display().to_string());
        per_file.push(FileResult { name, count: ds.entries.len() });
        combined.merge(ds);
    }

    let total = combined.entries.len();
    let mut type_counts: Vec<(String, usize)> = combined.by_type
        .iter()
        .map(|(t, v)| (t.clone(), v.len()))
        .collect();
    type_counts.sort_by(|a, b| a.0.cmp(&b.0));

    if output_json {
        let files_json: Vec<serde_json::Value> = per_file.iter()
            .map(|f| serde_json::json!({ "file": f.name, "count": f.count }))
            .collect();
        let type_map: serde_json::Map<String, serde_json::Value> = type_counts.iter()
            .map(|(t, n)| (t.clone(), serde_json::Value::from(*n)))
            .collect();
        println!("{}", serde_json::to_string_pretty(&serde_json::json!({
            "total": total,
            "files": files_json,
            "type_counts": type_map,
        })).unwrap());
    } else {
        println!("Total elements: {} (from {} file(s))\n", total, per_file.len());
        for f in &per_file {
            println!("  {}: {} elements", f.name, f.count);
        }
        if !type_counts.is_empty() {
            println!("\nBy type:");
            for (t, n) in &type_counts {
                println!("  {:<50} {}", t, n);
            }
        }
    }
}

fn cmd_validate(args: &[String]) {
    let mut input_files: Vec<PathBuf> = Vec::new();
    let mut profiles_override: Option<Vec<String>> = None;
    let mut force_solved = false;
    let mut force_not_solved = false;
    let mut enable_common = false;
    let mut enable_quality = false;
    let mut silenced: Vec<String> = Vec::new();
    let mut output_json = false;

    let mut i = 0usize;
    while i < args.len() {
        match args[i].as_str() {
            "--profiles" => {
                i += 1;
                if i >= args.len() { eprintln!("error: --profiles requires an argument"); usage(); }
                profiles_override = Some(args[i].split(',').map(str::to_string).collect());
            }
            "--solved" => { force_solved = true; }
            "--not-solved" => { force_not_solved = true; }
            "--common" => { enable_common = true; }
            "--quality" => { enable_quality = true; }
            "--silence" => {
                i += 1;
                if i >= args.len() { eprintln!("error: --silence requires an argument"); usage(); }
                silenced.extend(args[i].split(',').map(str::to_string));
            }
            "--format" => {
                i += 1;
                if i >= args.len() { eprintln!("error: --format requires an argument"); usage(); }
                match args[i].as_str() {
                    "json" => output_json = true,
                    "text" => output_json = false,
                    other => { eprintln!("error: unknown format '{other}', expected 'json' or 'text'"); usage(); }
                }
            }
            arg if !arg.starts_with('-') => {
                input_files.push(PathBuf::from(arg));
            }
            unknown => {
                eprintln!("error: unknown flag: {unknown}");
                usage();
            }
        }
        i += 1;
    }

    if input_files.is_empty() {
        eprintln!("error: at least one input file is required");
        usage();
    }

    // Decode each file separately (in parallel) to enable per-profile pre-merge validation.
    let paths: Vec<&std::path::Path> = input_files.iter().map(PathBuf::as_path).collect();
    let datasets = or_die(
        cimdecoder::CimDataset::decode_files_parallel_separate(&paths),
        "error decoding input",
    );

    let solved_override = if force_solved {
        Some(true)
    } else if force_not_solved {
        Some(false)
    } else {
        None
    };
    let cfg = cimvalidation::combined_config(
        &datasets,
        profiles_override,
        solved_override,
        enable_common,
        enable_quality,
        silenced,
    );

    // Two-phase validation: per-file local checks in parallel, then crossprofile checks
    // on the merged dataset, with rule silencing applied.
    let violations = cimvalidation::validate_files(datasets, &cfg);

    if output_json {
        let arr: Vec<serde_json::Value> = violations.iter().map(|v| {
            serde_json::json!({
                "object_id": v.object_id,
                "rule_id":   v.rule_id,
                "name":      v.name,
                "class":     v.class,
                "property":  v.property,
                "message":   v.message,
                "severity":  v.severity,
            })
        }).collect();
        println!("{}", serde_json::to_string_pretty(&arr).unwrap());
    } else if violations.is_empty() {
        println!("No violations found.");
    } else {
        for v in &violations {
            println!("[{}] {} — {} ({})", v.severity, v.rule_id, v.message, v.object_id);
        }
        eprintln!("{} violation(s) found.", violations.len());
        process::exit(2);
    }
}
