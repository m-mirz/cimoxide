//! Validation of a model set, reported as diagnostics.
//!
//! A CGMES model is several files — EQ, SSH, SV, TP, … — and cross-profile
//! rules need all of them, so the unit of validation is a *model set*: every
//! CIM RDF/XML file in one directory, which is how the CGMES test
//! configurations and most exchanges are laid out. The set runs through
//! [`cimvalidation::validate_files`], exactly as `cimcli validate` does, and
//! each finding is placed on every file that writes its object.

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use cimvalidation::Violation;
use lsp_types::{Diagnostic, DiagnosticSeverity, NumberOrString, Position, Range, Url};

use crate::index::{IdKind, Index};

/// What `cimcli validate` takes as flags.
#[derive(Debug, Default, Clone, serde::Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct Options {
    pub common: bool,
    pub quality: bool,
    pub silence: Vec<String>,
}

/// One file of a validated set.
pub struct SetFile {
    pub path: PathBuf,
    pub url: Url,
    pub index: Index,
}

/// A validated model set: its files' indexes and where each object is
/// defined, for navigation across files.
#[derive(Default)]
pub struct ModelSet {
    pub files: Vec<SetFile>,
    /// mRID → (file, element) of its `rdf:ID`, else its first `rdf:about`.
    pub defs: HashMap<String, (usize, usize)>,
}

impl ModelSet {
    pub fn definition(&self, mrid: &str) -> Option<(&SetFile, &crate::index::Element)> {
        let &(f, e) = self.defs.get(mrid)?;
        let file = &self.files[f];
        Some((file, &file.index.elements[e]))
    }
}

/// The CIM files of `dir`, in name order, with open buffers in place of
/// what is on disk.
pub fn collect(dir: &Path, open: &HashMap<PathBuf, Arc<str>>) -> Vec<(PathBuf, Arc<str>)> {
    let mut paths: Vec<PathBuf> = std::fs::read_dir(dir)
        .into_iter()
        .flatten()
        .flatten()
        .map(|e| e.path())
        .filter(|p| p.extension().is_some_and(|x| x.eq_ignore_ascii_case("xml")) && p.is_file())
        .collect();
    // An open buffer not yet saved to this directory still belongs to it.
    for p in open.keys() {
        if p.parent() == Some(dir) && !paths.contains(p) {
            paths.push(p.clone());
        }
    }
    paths.sort();
    paths
        .into_iter()
        .filter_map(|p| {
            let text = match open.get(&p) {
                Some(t) => t.clone(),
                None => Arc::from(std::fs::read_to_string(&p).ok()?),
            };
            crate::index::is_cim(&text).then_some((p, text))
        })
        .collect()
}

/// Validate `files` as one model set. Returns the set and the diagnostics
/// of every file in it, an empty list for a clean one.
pub fn validate(files: &[(PathBuf, Arc<str>)], opts: &Options) -> (ModelSet, Vec<(Url, Vec<Diagnostic>)>) {
    // Decode and index each file on its own thread, as `cimcli` decodes.
    let decoded: Vec<(Result<cimmodel::CimDataset, String>, Index)> = std::thread::scope(|s| {
        files
            .iter()
            .map(|(_, text)| {
                s.spawn(move || {
                    let ds = cimmodel::CimDataset::decode_str(text).map_err(|e| e.to_string());
                    (ds, Index::build(text))
                })
            })
            .collect::<Vec<_>>()
            .into_iter()
            .map(|h| h.join().expect("decode thread panicked"))
            .collect()
    });

    let mut set = ModelSet::default();
    let mut datasets = Vec::new();
    let mut out: Vec<(Url, Vec<Diagnostic>)> = Vec::new();
    for ((path, _), (ds, index)) in files.iter().zip(decoded) {
        let Ok(url) = Url::from_file_path(path) else { continue };
        let mut diags = Vec::new();
        match ds {
            Ok(ds) => datasets.push(ds),
            // A file that does not decode drops out of the set; say why.
            Err(e) => diags.push(Diagnostic {
                range: Range::default(),
                severity: Some(DiagnosticSeverity::ERROR),
                source: Some("cimoxide".into()),
                message: format!("not decoded, left out of validation: {e}"),
                ..Default::default()
            }),
        }
        set.files.push(SetFile { path: path.clone(), url: url.clone(), index });
        out.push((url, diags));
    }
    for (fi, f) in set.files.iter().enumerate() {
        for (ei, el) in f.index.elements.iter().enumerate() {
            if el.mrid.is_empty() {
                continue;
            }
            let slot = set.defs.entry(el.mrid.clone()).or_insert((fi, ei));
            let held = &set.files[slot.0].index.elements[slot.1];
            if el.id_kind == Some(IdKind::Id) && held.id_kind != Some(IdKind::Id) {
                *slot = (fi, ei);
            }
        }
    }

    if !datasets.is_empty() {
        let cfg = cimvalidation::combined_config(&datasets, None, None, opts.common, opts.quality, opts.silence.clone());
        let violations = cimvalidation::validate_files(datasets, &cfg);
        place(&set, &violations, &mut out);
    }
    (set, out)
}

/// Put each finding on the files that write its object: where the finding's
/// field is written if any file writes it, else at the object's definition,
/// else on every element describing it. A finding about no object in the
/// set (a dataset-wide count) goes to the top of the first file.
fn place(set: &ModelSet, violations: &[Violation], out: &mut [(Url, Vec<Diagnostic>)]) {
    let mut by_mrid: HashMap<&str, Vec<(usize, usize)>> = HashMap::new();
    for (fi, f) in set.files.iter().enumerate() {
        for (ei, el) in f.index.elements.iter().enumerate() {
            by_mrid.entry(el.mrid.as_str()).or_default().push((fi, ei));
        }
    }
    for v in violations {
        let sites = by_mrid.get(v.object_id.as_str()).map_or(&[][..], Vec::as_slice);
        let key = v.property.trim_start_matches('^');
        let mut placed: Vec<(usize, Range)> = sites
            .iter()
            .flat_map(|&(fi, ei)| {
                let el = &set.files[fi].index.elements[ei];
                el.fields.iter().filter(|f| f.key() == key).map(move |f| (fi, f.full))
            })
            .collect();
        if placed.is_empty() {
            let at = |&(fi, ei): &(usize, usize)| {
                let el = &set.files[fi].index.elements[ei];
                (fi, el.id_range.unwrap_or(el.head))
            };
            placed = match set.defs.get(v.object_id.as_str()) {
                Some(def) => vec![at(def)],
                None => sites.iter().map(at).collect(),
            };
        }
        if placed.is_empty() && !out.is_empty() {
            placed.push((0, Range::new(Position::new(0, 0), Position::new(0, 0))));
        }
        for (fi, range) in placed {
            out[fi].1.push(diagnostic(v, range));
        }
    }
}

fn diagnostic(v: &Violation, range: Range) -> Diagnostic {
    let severity = match v.severity.as_str() {
        "sh:Violation" => DiagnosticSeverity::ERROR,
        "sh:Warning" => DiagnosticSeverity::WARNING,
        _ => DiagnosticSeverity::INFORMATION,
    };
    let mut message = v.message.clone();
    if !v.object_id.is_empty() {
        message.push_str(&format!(" ({} {})", v.class, v.object_id));
    }
    if !v.description.is_empty() && v.description != v.message {
        message.push_str("\n\n");
        message.push_str(&v.description);
    }
    Diagnostic {
        range,
        severity: Some(severity),
        code: Some(NumberOrString::String(v.rule_id.clone())),
        source: Some("cimoxide".into()),
        message,
        // The finding as `cimcli validate --format json` names it.
        data: Some(serde_json::json!({ "object_id": v.object_id, "property": v.property })),
        ..Default::default()
    }
}
