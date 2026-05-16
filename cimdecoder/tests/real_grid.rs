use std::path::Path;
use cimdecoder::CimDataset;

const BASE: &str = "../cimgo/CGMES-Test-Configurations/v3.0/RealGrid/RealGrid-Merged";

#[test]
fn decode_eq_counts() {
    let ds = CimDataset::decode_file(Path::new(&format!("{BASE}/RealGrid_EQ.xml")))
        .expect("decode EQ failed");
    eprintln!("EQ: {} objects", ds.entries.len());
    assert!(ds.entries.len() > 10_000, "expected many EQ objects");
}

#[test]
fn decode_and_merge_all() {
    let files: Vec<std::path::PathBuf> = ["RealGrid_EQ.xml", "RealGrid_SSH.xml", "RealGrid_TP.xml", "RealGrid_SV.xml"]
        .iter()
        .map(|f| Path::new(BASE).join(f))
        .collect();
    let paths: Vec<&Path> = files.iter().map(|p| p.as_path()).collect();
    let ds = CimDataset::decode_files(&paths).expect("decode_files failed");
    eprintln!("Merged: {} objects", ds.entries.len());
    assert!(ds.entries.len() > 10_000);
}
