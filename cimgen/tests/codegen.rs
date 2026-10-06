use sha2::{Digest, Sha256};
use std::path::{Path, PathBuf};
use std::process::Command;

fn workspace_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .to_path_buf()
}

fn hash_dir(dir: &Path) -> String {
    hash_dir_except(dir, &[])
}

/// `skip` names files to leave out, relative to `dir`.
///
/// The bag families' shape tables are written into the same output directory as
/// the CGMES validators, and folding them into this hash would mean an NC-only
/// change trips the CGMES test and vice versa — exactly what the separate
/// `nc_classes_codegen_stable` test exists to prevent.
fn hash_dir_except(dir: &Path, skip: &[&str]) -> String {
    let mut files: Vec<(String, Vec<u8>)> = Vec::new();
    collect(dir, dir, &mut files);
    files.retain(|(rel, _)| !skip.contains(&rel.as_str()));
    files.sort_by(|a, b| a.0.cmp(&b.0));

    let mut h = Sha256::new();
    for (rel, content) in &files {
        h.update(rel.as_bytes());
        h.update(b"\0");
        h.update(content);
    }
    // sha2 0.11's `finalize()` returns an `Array` with no `LowerHex` impl; the byte string
    // is identical to what `format!("{:x}", ..)` produced on 0.10, so stored hashes still match.
    h.finalize().iter().map(|b| format!("{b:02x}")).collect()
}

fn collect(base: &Path, dir: &Path, out: &mut Vec<(String, Vec<u8>)>) {
    for entry in std::fs::read_dir(dir).unwrap().flatten() {
        let path = entry.path();
        if path.is_dir() {
            collect(base, &path, out);
        } else {
            let rel = path.strip_prefix(base).unwrap().to_string_lossy().into_owned();
            out.push((rel, std::fs::read(&path).unwrap()));
        }
    }
}

#[test]
fn cimmodel_codegen_stable() {
    let root = workspace_root();
    let out = Path::new(env!("CARGO_TARGET_TMPDIR")).join("cimmodel");
    let _ = std::fs::remove_dir_all(&out);
    std::fs::create_dir_all(&out).unwrap();

    let status = Command::new(env!("CARGO_BIN_EXE_cimgen"))
        .current_dir(&root)
        .arg("--output")
        .arg(&out)
        .arg("--skip-shacl")
        .arg("--skip-python-stubs")
        .status()
        .unwrap();
    assert!(status.success(), "cimgen exited with failure");

    let hash = hash_dir(&out);
    assert_eq!(hash, "ddecee9a07c717aa1dd4a46921ea2843a850b1601c6e962ec0531062fa4245c0", "cimmodel output drifted — rerun to update hash");
}

/// Hashes the NC shape table on its own, for the same reason
/// `nc_classes_codegen_stable` hashes the class table on its own: the CGMES
/// validators and the NC shapes come out of the same run, and a single hash
/// over both cannot say which family moved.
#[test]
fn nc_shapes_codegen_stable() {
    let root = workspace_root();
    let structs_out = Path::new(env!("CARGO_TARGET_TMPDIR")).join("cimmodel-ncshapes");
    let shacl_out = Path::new(env!("CARGO_TARGET_TMPDIR")).join("cimvalidation-ncshapes");
    let _ = std::fs::remove_dir_all(&structs_out);
    let _ = std::fs::remove_dir_all(&shacl_out);
    std::fs::create_dir_all(&structs_out).unwrap();
    std::fs::create_dir_all(&shacl_out).unwrap();

    let status = Command::new(env!("CARGO_BIN_EXE_cimgen"))
        .current_dir(&root)
        .arg("--output")
        .arg(&structs_out)
        .arg("--shacl-output")
        .arg(&shacl_out)
        .arg("--skip-python-stubs")
        .status()
        .unwrap();
    assert!(status.success(), "cimgen exited with failure");

    let mut h = Sha256::new();
    h.update(std::fs::read(shacl_out.join("nc_shapes.rs")).unwrap());
    let hash: String = h.finalize().iter().map(|b| format!("{b:02x}")).collect();
    assert_eq!(hash, "8a6e12a38432782bb500a036108a7766ca77941f7e0fc9ed07961700066b7a5e", "NC shape table drifted — rerun to update hash");
}

/// Hashes the CGMES shape table on its own: it replaced the generated
/// validators as what validation runs, and comes out of the same cimgen run as
/// they and the NC table do.
#[test]
fn cgmes_shapes_codegen_stable() {
    let root = workspace_root();
    let structs_out = Path::new(env!("CARGO_TARGET_TMPDIR")).join("cimmodel-cgmesshapes");
    let shacl_out = Path::new(env!("CARGO_TARGET_TMPDIR")).join("cimvalidation-cgmesshapes");
    let _ = std::fs::remove_dir_all(&structs_out);
    let _ = std::fs::remove_dir_all(&shacl_out);
    std::fs::create_dir_all(&structs_out).unwrap();
    std::fs::create_dir_all(&shacl_out).unwrap();

    let status = Command::new(env!("CARGO_BIN_EXE_cimgen"))
        .current_dir(&root)
        .arg("--output")
        .arg(&structs_out)
        .arg("--shacl-output")
        .arg(&shacl_out)
        .arg("--skip-python-stubs")
        .status()
        .unwrap();
    assert!(status.success(), "cimgen exited with failure");

    let mut h = Sha256::new();
    h.update(std::fs::read(shacl_out.join("cgmes_shapes.rs")).unwrap());
    let hash: String = h.finalize().iter().map(|b| format!("{b:02x}")).collect();
    assert_eq!(hash, "554dc2b20a97850653cae56a50436f4dd3a1ea04cb49c6489e99b1e979170409", "CGMES shape table drifted — rerun to update hash");
}

/// Hashes the NC class table on its own, so a CGMES-only change cannot mask an
/// NC change and vice versa. This is the only mechanical guard that bumping the
/// `application-profiles-library` submodule did not silently alter the NC
/// surface.
#[test]
fn nc_classes_codegen_stable() {
    let root = workspace_root();
    let out = Path::new(env!("CARGO_TARGET_TMPDIR")).join("cimmodel-nc");
    let _ = std::fs::remove_dir_all(&out);
    std::fs::create_dir_all(&out).unwrap();

    let status = Command::new(env!("CARGO_BIN_EXE_cimgen"))
        .current_dir(&root)
        .arg("--output")
        .arg(&out)
        .arg("--skip-shacl")
        .arg("--skip-python-stubs")
        .status()
        .unwrap();
    assert!(status.success(), "cimgen exited with failure");

    let mut h = Sha256::new();
    h.update(std::fs::read(out.join("nc_classes.rs")).unwrap());
    let hash: String = h.finalize().iter().map(|b| format!("{b:02x}")).collect();
    assert_eq!(hash, "637d0b191948a64e9dec994a7f5d175dd5d93b9b285164555fe67dbc1eb32e8d", "NC class table drifted — rerun to update hash");
}
