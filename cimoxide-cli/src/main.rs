mod convert;

use std::path::PathBuf;
use std::process;

fn usage() -> ! {
    eprintln!("Usage:");
    eprintln!("  cimoxide-cli convert --to json <xml-files...> [--out <output.json>]");
    eprintln!("  cimoxide-cli convert --to xml  <input.json>   [--out <output.xml>]");
    eprintln!("  cimoxide-cli convert --to xml  <input.json>   --profile EQ,SSH [--out <dir/>]");
    process::exit(1);
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.is_empty() || args[0] != "convert" {
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
    let ds = cimdecoder::CimDataset::decode_files(&paths).unwrap_or_else(|e| {
        eprintln!("error decoding XML: {e}");
        process::exit(1);
    });
    let json = convert::dataset_to_json(&ds);
    let text = serde_json::to_string_pretty(&json).unwrap_or_else(|e| {
        eprintln!("error serializing JSON: {e}");
        process::exit(1);
    });
    write_output(&text, out);
}

fn cmd_to_xml(json_file: &std::path::Path, out: Option<&std::path::Path>, profiles: &[&str]) {
    let src = std::fs::read_to_string(json_file).unwrap_or_else(|e| {
        eprintln!("error reading {}: {e}", json_file.display());
        process::exit(1);
    });
    let ds = convert::dataset_from_json(&src).unwrap_or_else(|e| {
        eprintln!("error parsing JSON: {e}");
        process::exit(1);
    });

    if profiles.is_empty() {
        let xml = convert::dataset_to_xml(&ds).unwrap_or_else(|e| {
            eprintln!("error generating XML: {e}");
            process::exit(1);
        });
        write_output(&xml, out);
        return;
    }

    if profiles.len() > 1 {
        let dir = out.unwrap_or_else(|| {
            eprintln!("error: --out <dir/> is required when multiple --profile codes are given");
            usage();
        });
        if !dir.is_dir() {
            std::fs::create_dir_all(dir).unwrap_or_else(|e| {
                eprintln!("error creating directory {}: {e}", dir.display());
                process::exit(1);
            });
        }
        for &code in profiles {
            let xml = convert::dataset_to_xml_for_profile(&ds, code).unwrap_or_else(|e| {
                eprintln!("error generating XML for profile {code}: {e}");
                process::exit(1);
            });
            let path = dir.join(format!("{code}.xml"));
            std::fs::write(&path, &xml).unwrap_or_else(|e| {
                eprintln!("error writing {}: {e}", path.display());
                process::exit(1);
            });
        }
    } else {
        let code = profiles[0];
        let xml = convert::dataset_to_xml_for_profile(&ds, code).unwrap_or_else(|e| {
            eprintln!("error generating XML for profile {code}: {e}");
            process::exit(1);
        });
        write_output(&xml, out);
    }
}

fn write_output(text: &str, out: Option<&std::path::Path>) {
    match out {
        Some(path) => std::fs::write(path, text).unwrap_or_else(|e| {
            eprintln!("error writing {}: {e}", path.display());
            process::exit(1);
        }),
        None => print!("{text}"),
    }
}
