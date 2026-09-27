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
fn cimstructs_codegen_stable() {
    let root = workspace_root();
    let out = Path::new(env!("CARGO_TARGET_TMPDIR")).join("cimstructs");
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
    assert_eq!(hash, "f13b6abf847800d21dd182bb4670fd985cbd7ab8e53caa1d72df22fd8eb422d5", "cimstructs output drifted — rerun to update hash");
}

#[test]
fn cimvalidation_codegen_stable() {
    let root = workspace_root();
    let structs_out = Path::new(env!("CARGO_TARGET_TMPDIR")).join("cimstructs-shacl");
    let shacl_out = Path::new(env!("CARGO_TARGET_TMPDIR")).join("cimvalidation");
    let _ = std::fs::remove_dir_all(&structs_out);
    let _ = std::fs::remove_dir_all(&shacl_out);
    std::fs::create_dir_all(&structs_out).unwrap();
    std::fs::create_dir_all(&shacl_out).unwrap();

    let shacl_glob = root.join(
        "application-profiles-library/CGMES/SHACL/*.ttl",
    );

    let status = Command::new(env!("CARGO_BIN_EXE_cimgen"))
        .current_dir(&root)
        .arg("--output")
        .arg(&structs_out)
        .arg("--shacl")
        .arg(&shacl_glob)
        .arg("--shacl-output")
        .arg(&shacl_out)
        .arg("--skip-python-stubs")
        .status()
        .unwrap();
    assert!(status.success(), "cimgen exited with failure");

    let hash = hash_dir_except(&shacl_out, &["nc_shapes.rs", "nc_profiles.rs"]);
    assert_eq!(hash, "5369720ee910af3ab9f30d2e3ed3fd7395df87f6140bc2f2ebb7069d730a4e11", "cimvalidation output drifted — rerun to update hash");
}

/// Hashes the NC shape table on its own, for the same reason
/// `nc_classes_codegen_stable` hashes the class table on its own: the CGMES
/// validators and the NC shapes come out of the same run, and a single hash
/// over both cannot say which family moved.
#[test]
fn nc_shapes_codegen_stable() {
    let root = workspace_root();
    let structs_out = Path::new(env!("CARGO_TARGET_TMPDIR")).join("cimstructs-ncshapes");
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
    assert_eq!(hash, "f9e0e8bb9529f0786cbcd552fbcc30cb032cabadd15066e819f5c92ea297ee97", "NC shape table drifted — rerun to update hash");
}

/// Hashes the NC class table on its own, so a CGMES-only change cannot mask an
/// NC change and vice versa. This is the only mechanical guard that bumping the
/// `application-profiles-library` submodule did not silently alter the NC
/// surface.
#[test]
fn nc_classes_codegen_stable() {
    let root = workspace_root();
    let out = Path::new(env!("CARGO_TARGET_TMPDIR")).join("cimstructs-nc");
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
