//! Model sets, decoded once and kept until a file of theirs changes.
//!
//! A model set is every CIM RDF/XML file in one directory — the unit
//! `cimlsp` validates — or a single file, or files and directories listed
//! together (an IGM with its boundary and common data, which live apart).
//! The merged dataset, the SPARQL store and the findings of each validation
//! configuration are built on first use.

use std::collections::HashMap;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

use cimmodel::CimDataset;
use cimsparql::CimStore;
use cimvalidation::Violation;

/// What `cimcli validate` takes as flags.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Options {
    pub common: bool,
    pub quality: bool,
    pub silence: Vec<String>,
}

pub struct ModelSet {
    /// The set's directory, its one file, or the directory its files share.
    pub root: PathBuf,
    /// The set as named: its path, or its listed paths joined by ` + `.
    pub name: String,
    pub files: Vec<PathBuf>,
    stamps: Vec<Option<SystemTime>>,
    pub dataset: CimDataset,
    store: Option<CimStore>,
    findings: Vec<(Options, Vec<Violation>)>,
}

impl ModelSet {
    /// The SPARQL store over the merged dataset, built on first use.
    pub fn store(&mut self) -> Result<&CimStore, String> {
        if self.store.is_none() {
            self.store = Some(CimStore::from_dataset(&self.dataset).map_err(|e| format!("building the RDF graph: {e}"))?);
        }
        Ok(self.store.as_ref().expect("built above"))
    }

    /// The findings of `cimcli validate` with `opts`, computed on first use,
    /// and the dataset they are about. Validation runs on each file decoded
    /// on its own, so the files are decoded again rather than every set
    /// holding them twice.
    pub fn findings(&mut self, opts: &Options) -> Result<(&CimDataset, &[Violation]), String> {
        let at = match self.findings.iter().position(|(o, _)| o == opts) {
            Some(i) => i,
            None => {
                let paths: Vec<&Path> = self.files.iter().map(PathBuf::as_path).collect();
                let per_file = CimDataset::decode_files_parallel_separate(&paths).map_err(|e| e.to_string())?;
                let cfg = cimvalidation::combined_config(&per_file, None, None, opts.common, opts.quality, opts.silence.clone());
                self.findings.push((opts.clone(), cimvalidation::validate_files(per_file, &cfg)));
                self.findings.len() - 1
            }
        };
        Ok((&self.dataset, &self.findings[at].1))
    }

    /// A file's path relative to the set's root, for display.
    pub fn display(&self, file: &Path) -> String {
        file.strip_prefix(&self.root)
            .ok()
            .filter(|p| !p.as_os_str().is_empty())
            .unwrap_or_else(|| file.file_name().map_or(file, Path::new))
            .display()
            .to_string()
    }
}

pub struct Session {
    /// Where relative paths resolve; the set a tool reads when given none.
    pub base: PathBuf,
    /// Keyed by the paths named, resolved.
    sets: HashMap<Vec<PathBuf>, ModelSet>,
}

impl Session {
    pub fn new(base: PathBuf) -> Self {
        let base = base.canonicalize().unwrap_or(base);
        Self { base, sets: HashMap::new() }
    }

    /// `path` against the base directory.
    pub fn resolve(&self, path: Option<&str>) -> PathBuf {
        match path.filter(|p| !p.trim().is_empty()) {
            None => self.base.clone(),
            Some(p) => {
                let p = Path::new(p.trim());
                let full = if p.is_absolute() { p.to_path_buf() } else { self.base.join(p) };
                full.canonicalize().unwrap_or(full)
            }
        }
    }

    /// `path` relative to the base directory, for display.
    pub fn relative(&self, path: &Path) -> String {
        match path.strip_prefix(&self.base) {
            Ok(p) if p.as_os_str().is_empty() => ".".into(),
            Ok(p) => p.display().to_string(),
            Err(_) => path.display().to_string(),
        }
    }

    /// The model set `path` names, or the one `files` lists (each a file, or a
    /// directory standing for its CIM files, merged in the order given),
    /// decoded if new or changed since.
    pub fn model_set(&mut self, path: Option<&str>, files: &[String]) -> Result<&mut ModelSet, String> {
        let listed: Vec<&str> = files.iter().map(|f| f.trim()).filter(|f| !f.is_empty()).collect();
        let (key, root, name, files) = if listed.is_empty() {
            let root = self.resolve(path);
            let files = if root.is_file() {
                vec![root.clone()]
            } else if root.is_dir() {
                cim_files(&root)
            } else {
                return Err(format!("{} does not exist (paths are relative to {})", root.display(), self.base.display()));
            };
            if files.is_empty() {
                return Err(self.no_cim_files(&root));
            }
            (vec![root.clone()], root.clone(), self.relative(&root), files)
        } else {
            if path.is_some() {
                return Err("give `path` or `files`, not both".into());
            }
            let sources: Vec<PathBuf> = listed.iter().map(|f| self.resolve(Some(f))).collect();
            let mut files: Vec<PathBuf> = Vec::new();
            for source in &sources {
                let found = if source.is_file() {
                    if !head(source, 8192).is_some_and(|h| cimmodel::decode::is_cim(&h)) {
                        return Err(format!("{} is not a CGMES or NC RDF/XML file", self.relative(source)));
                    }
                    vec![source.clone()]
                } else if source.is_dir() {
                    cim_files(source)
                } else {
                    return Err(format!("{} does not exist (paths are relative to {})", source.display(), self.base.display()));
                };
                if found.is_empty() {
                    return Err(self.no_cim_files(source));
                }
                for file in found {
                    if !files.contains(&file) {
                        files.push(file);
                    }
                }
            }
            let name = sources.iter().map(|s| self.relative(s)).collect::<Vec<_>>().join(" + ");
            (sources, common_dir(&files), name, files)
        };
        let stamps: Vec<Option<SystemTime>> = files.iter().map(|f| f.metadata().and_then(|m| m.modified()).ok()).collect();
        let fresh = self.sets.get(&key).is_some_and(|s| s.files == files && s.stamps == stamps);
        if !fresh {
            let paths: Vec<&Path> = files.iter().map(PathBuf::as_path).collect();
            let dataset = CimDataset::decode_files_parallel(&paths).map_err(|e| format!("decoding {name}: {e}"))?;
            let set = ModelSet { root, name, files, stamps, dataset, store: None, findings: Vec::new() };
            self.sets.insert(key.clone(), set);
        }
        Ok(self.sets.get_mut(&key).expect("inserted above"))
    }

    /// That `dir` holds no CIM file, and the model sets below it.
    fn no_cim_files(&self, dir: &Path) -> String {
        let mut msg = format!("{} holds no CGMES or NC RDF/XML file.", self.relative(dir));
        let below = find_sets(dir, 4);
        if !below.is_empty() {
            msg.push_str(" Model sets below it:");
            for d in below.iter().take(20) {
                msg.push_str(&format!("\n  {}", self.relative(d)));
            }
            if below.len() > 20 {
                msg.push_str(&format!("\n  … and {} more (list_model_sets)", below.len() - 20));
            }
        }
        msg
    }
}

/// The deepest directory holding every one of `files`.
fn common_dir(files: &[PathBuf]) -> PathBuf {
    let mut dir = files[0].parent().map_or_else(PathBuf::new, Path::to_path_buf);
    for file in &files[1..] {
        while !file.starts_with(&dir) && dir.pop() {}
    }
    dir
}

/// The first bytes of `path`, enough to tell a CIM file and read its header.
pub fn head(path: &Path, len: usize) -> Option<String> {
    let mut buf = Vec::with_capacity(len);
    std::fs::File::open(path).ok()?.take(len as u64).read_to_end(&mut buf).ok()?;
    // A cut through a multi-byte character costs that character only.
    Some(String::from_utf8_lossy(&buf).into_owned())
}

/// The CIM RDF/XML files of `dir`, in name order.
pub fn cim_files(dir: &Path) -> Vec<PathBuf> {
    let mut files: Vec<PathBuf> = std::fs::read_dir(dir)
        .into_iter()
        .flatten()
        .flatten()
        .map(|e| e.path())
        .filter(|p| p.extension().is_some_and(|x| x.eq_ignore_ascii_case("xml")) && p.is_file())
        .filter(|p| head(p, 8192).is_some_and(|h| cimmodel::decode::is_cim(&h)))
        .collect();
    files.sort();
    files
}

/// Directories at or below `root`, `depth` levels deep at most, that hold
/// CIM files; in path order. Hidden directories, `node_modules` and
/// `target` are skipped.
pub fn find_sets(root: &Path, depth: usize) -> Vec<PathBuf> {
    let mut out = Vec::new();
    walk(root, depth, &mut out);
    out.sort();
    out
}

fn walk(dir: &Path, depth: usize, out: &mut Vec<PathBuf>) {
    if !cim_files(dir).is_empty() {
        out.push(dir.to_path_buf());
    }
    if depth == 0 {
        return;
    }
    let Ok(entries) = std::fs::read_dir(dir) else { return };
    for entry in entries.flatten() {
        let path = entry.path();
        let name = entry.file_name();
        let name = name.to_string_lossy();
        if name.starts_with('.') || name == "node_modules" || name == "target" {
            continue;
        }
        if entry.file_type().is_ok_and(|t| t.is_dir()) {
            walk(&path, depth - 1, out);
        }
    }
}
