use std::collections::HashMap;
use std::path::Path;
use cimdecoder::CimDataset;
use cimvalidation::Violation;

#[allow(dead_code)]
pub fn load_dataset(path: &str) -> CimDataset {
    CimDataset::decode_file(Path::new(path)).expect("failed to load testdata")
}

#[allow(dead_code)]
pub fn violations_by_id<'a>(vs: &'a [Violation]) -> HashMap<String, Vec<&'a Violation>> {
    let mut m: HashMap<String, Vec<&'a Violation>> = HashMap::new();
    for v in vs {
        m.entry(v.object_id.clone()).or_default().push(v);
    }
    m
}
