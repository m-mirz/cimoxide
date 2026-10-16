//! Validation of a model set, reported as diagnostics.
//!
//! A CGMES model is several files — EQ, SSH, SV, TP, … — and cross-profile
//! rules need all of them, so the unit of validation is a *model set*: every
//! CIM RDF/XML file in one directory, which is how the CGMES test
//! configurations and most exchanges are laid out. The set runs through
//! [`cimvalidation::validate_files`], exactly as `cimcli validate` does, and
//! each finding is placed on the file it was found in, or for one found on
//! the merged set, on every file that writes its object.

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
    /// The grid at substation level, with each object's findings counted.
    pub grid: crate::grid::Grid,
    /// The set's objects by container, area and class, with their findings.
    pub tree: crate::tree::Tree,
    /// Each object's findings: (property, rule, message), for the tree's fixes.
    pub findings: HashMap<String, Vec<(String, String, String)>>,
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
    // The set file each dataset was decoded from.
    let mut origin = Vec::new();
    let mut out: Vec<(Url, Vec<Diagnostic>)> = Vec::new();
    for ((path, _), (ds, index)) in files.iter().zip(decoded) {
        let Ok(url) = Url::from_file_path(path) else { continue };
        let mut diags = Vec::new();
        match ds {
            Ok(ds) => {
                datasets.push(ds);
                origin.push(set.files.len());
            }
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
        {
            let model = crate::grid::Model::new(&datasets);
            set.grid = crate::grid::build(&model);
            set.tree = crate::tree::build(&model);
        }
        let violations = cimvalidation::validate_files_traced(datasets, &cfg);
        let violations: Vec<_> = violations.into_iter().map(|(d, v)| (d.map(|d| origin[d]), v)).collect();
        set.grid.add_findings(&violations);
        set.tree.add_findings(&violations);
        for (_, v) in &violations {
            if !v.object_id.is_empty() {
                set.findings.entry(v.object_id.clone()).or_default().push((v.property.clone(), v.rule_id.clone(), v.message.clone()));
            }
        }
        place(&set, &violations, &mut out);
    }
    (set, out)
}

/// Put each finding on the files that write its object: where the finding's
/// field is written if any file writes it, else at the object's definition,
/// else on every element describing it. A finding from one file looks only
/// there — an SSH file's `cim:Equipment` lacking the mRID that the EQ file
/// writes belongs on the SSH element, not on the EQ field. A finding about
/// no object in the set (a header check, a dataset-wide count) goes to the
/// top of its file, else of the first.
fn place(set: &ModelSet, violations: &[(Option<usize>, Violation)], out: &mut [(Url, Vec<Diagnostic>)]) {
    let mut by_mrid: HashMap<&str, Vec<(usize, usize)>> = HashMap::new();
    for (fi, f) in set.files.iter().enumerate() {
        for (ei, el) in f.index.elements.iter().enumerate() {
            by_mrid.entry(el.mrid.as_str()).or_default().push((fi, ei));
        }
    }
    for (file, v) in violations {
        let all = by_mrid.get(v.object_id.as_str()).map_or(&[][..], Vec::as_slice);
        let local: Vec<(usize, usize)> = all.iter().filter(|s| Some(s.0) == *file).copied().collect();
        let (sites, def) = if local.is_empty() {
            (all, set.defs.get(v.object_id.as_str()))
        } else {
            (local.as_slice(), None)
        };
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
            placed = match def {
                Some(def) => vec![at(def)],
                None => sites.iter().map(at).collect(),
            };
        }
        if placed.is_empty() && !out.is_empty() {
            placed.push((file.unwrap_or(0), Range::new(Position::new(0, 0), Position::new(0, 0))));
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

#[cfg(test)]
mod tests {
    use super::*;

    fn doc(profile: &str, body: &str) -> Arc<str> {
        Arc::from(format!(
            r##"<rdf:RDF xmlns:cim="http://iec.ch/TC57/CIM100#" xmlns:md="http://iec.ch/TC57/61970-552/ModelDescription/1#" xmlns:rdf="http://www.w3.org/1999/02/22-rdf-syntax-ns#">
  <md:FullModel rdf:about="urn:uuid:{profile}">
    <md:Model.profile>http://iec.ch/TC57/ns/CIM/{profile}/3.0</md:Model.profile>
  </md:FullModel>
{body}
</rdf:RDF>"##
        ))
    }

    #[test]
    fn a_finding_stays_in_the_file_it_was_found_in() {
        // The SSH file lacks the mRID the EQ file writes: the finding is the
        // SSH file's, so it must not land on the EQ file's mRID field.
        let eq = doc("CoreEquipment-EU", r#"  <cim:BusbarSection rdf:ID="_b">
    <cim:IdentifiedObject.mRID>b</cim:IdentifiedObject.mRID>
  </cim:BusbarSection>"#);
        let ssh = doc("SteadyStateHypothesis-EU", r#"  <cim:Equipment rdf:ID="_b">
    <cim:Equipment.inService>true</cim:Equipment.inService>
  </cim:Equipment>"#);
        let dir = std::env::temp_dir();
        let files = vec![(dir.join("x_EQ.xml"), eq), (dir.join("x_SSH.xml"), ssh)];
        let (_, out) = validate(&files, &Options::default());
        let mrid = |i: usize| -> Vec<Range> {
            out[i].1.iter()
                .filter(|d| d.code == Some(NumberOrString::String("ido:IdentifiedObject.mRID-cardinality".into())))
                .map(|d| d.range)
                .collect()
        };
        assert!(mrid(0).is_empty(), "{:?}", out[0].1);
        // On the SSH element's `rdf:ID` value.
        assert_eq!(mrid(1), [Range::new(Position::new(4, 25), Position::new(4, 27))]);
    }
}
