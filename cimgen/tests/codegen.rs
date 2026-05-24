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
    let mut files: Vec<(String, Vec<u8>)> = Vec::new();
    collect(dir, dir, &mut files);
    files.sort_by(|a, b| a.0.cmp(&b.0));

    let mut h = Sha256::new();
    for (rel, content) in &files {
        h.update(rel.as_bytes());
        h.update(b"\0");
        h.update(content);
    }
    format!("{:x}", h.finalize())
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
        .status()
        .unwrap();
    assert!(status.success(), "cimgen exited with failure");

    let hash = hash_dir(&out);
    assert_eq!(hash, "4c631428b08bccf07f9b3e7da4404ea07a390eed867670c0d2b4bc828f69f140", "cimstructs output drifted — rerun to update hash");
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
        "application-profiles-library/CGMES/CurrentRelease/SHACL/TTL/*.ttl",
    );

    let status = Command::new(env!("CARGO_BIN_EXE_cimgen"))
        .current_dir(&root)
        .arg("--output")
        .arg(&structs_out)
        .arg("--shacl")
        .arg(&shacl_glob)
        .arg("--shacl-output")
        .arg(&shacl_out)
        .status()
        .unwrap();
    assert!(status.success(), "cimgen exited with failure");

    let hash = hash_dir(&shacl_out);
    assert_eq!(hash, "191e8956acbd20ac969fc32b1b3bd07ab31c390c79d44d081e6bed2adfd4daa0", "cimvalidation output drifted — rerun to update hash");
}
