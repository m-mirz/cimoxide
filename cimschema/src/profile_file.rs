//! What a profile file says it is.
//!
//! ENTSO-E's profile files open with an `owl:Ontology` header whose `owl:versionIRI` names
//! them, and the constraint and vocabulary files bind the `cim:` namespace of the CIM they
//! describe. Those decide a file's family, not the directory it sits in or its name: ENTSO-E
//! moves and renames both (`application-profiles-library` was split into `CGMES-AP` and
//! `NC-AP`), and a copy of the files may be laid out any way at all.
//!
//! | File | `owl:versionIRI` | `cim:` |
//! |---|---|---|
//! | CGMES 3.0 shapes, vocabulary, PROF | `http://iec.ch/TC57/ns/CIM/…`, `http://iec.ch/TC57/61970-552/…` (header) | `http://iec.ch/TC57/CIM100#` |
//! | NC shapes | `https://ap-con.cim4.eu/…` | `https://cim.ucaiug.io/ns#` |
//! | NC manifests | `https://ap-val.cim4.eu/…` | — |
//! | NC vocabulary | `https://ap-voc.cim4.eu/…` | `https://cim.ucaiug.io/ns#` |
//! | NC PROF | `https://ap.cim4.eu/…` | — |
//!
//! The `cim:` binding keeps out files that share a family's IRIs but describe another CIM:
//! ENTSO-E's QoCDC rules for CGMES 2.4 are `https://ap-con.cim4.eu/QoCDC-…` with `cim16`.

use std::path::{Path, PathBuf};

use crate::family::{FAMILIES, Family};

/// What a file holds.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Role {
    /// SHACL constraints (`.ttl`).
    Shapes,
    /// An NC validation manifest: which constraint files apply to a profile (`.ttl`).
    Manifest,
    /// The RDFS vocabulary of a profile (`.rdf`).
    Vocabulary,
    /// A PROF descriptor: a profile's IRIs and code (`.rdf`).
    Descriptor,
}

/// The part of a file its header is read from. Every ENTSO-E file declares itself well
/// inside it.
const HEADER_BYTES: usize = 64 * 1024;

/// The family and role `path` declares, or `None` for a file that declares neither: no
/// version IRI, one no family claims, or a `cim:` namespace other than the family's. An
/// `.rdf` file is a vocabulary only if it declares classes: ENTSO-E also ships SHACL as
/// RDF/XML (CGMES-AP's `RDFS_Beta_501_Ed2_CD`), which cimoxide does not read.
pub fn identify(path: &Path) -> Option<(&'static Family, Role)> {
    declared(path).map(|d| (d.family, d.role))
}

/// What a file declares: its family, its role and its version IRI.
struct Declared {
    family: &'static Family,
    role: Role,
    iri: String,
}

fn declared(path: &Path) -> Option<Declared> {
    let turtle = match path.extension().and_then(|x| x.to_str()) {
        Some("ttl") => true,
        Some("rdf") => false,
        _ => return None,
    };
    let text = head(path)?;
    let iri = version_iri(&text)?;
    let family = FAMILIES.iter().copied().find(|f| {
        f.ontology_prefixes.iter().any(|p| iri.starts_with(p))
            && cim_namespace(&text).is_none_or(|ns| ns == f.cim_namespace)
    })?;
    let role = if turtle {
        if family.manifest_prefix.is_some_and(|p| iri.starts_with(p)) {
            Role::Manifest
        } else {
            Role::Shapes
        }
    } else if text.contains("http://www.w3.org/ns/dx/prof/Profile") {
        Role::Descriptor
    } else if text.contains("rdf-schema#Class") || text.contains("rdfs:Class") {
        Role::Vocabulary
    } else {
        return None;
    };
    Some(Declared {
        family,
        role,
        iri: iri.to_string(),
    })
}

/// Environment variable listing the directories profile files are read from, separated as
/// in `PATH`. See [`Pool`].
pub const PROFILES_ENV: &str = "CIMOXIDE_PROFILES";

/// How deep [`Pool::scan`] looks below each directory: ENTSO-E's former
/// `application-profiles-library` keeps NC's manifests four levels down
/// (`NCP/NCP/SHACL/Validation`).
const SCAN_DEPTH: usize = 5;

/// The profile files found in a list of directories, sorted by what they declare.
///
/// Each directory is searched to [`SCAN_DEPTH`], skipping hidden directories, files before
/// subdirectories, in name order. A profile is a family's role at a version IRI without its
/// version (`…/CoreEquipment-EU/3.0` → `…/CoreEquipment-EU`), and the first file found for
/// it wins:
/// a later file with the same IRI is a copy and is passed over, one with another version
/// is reported.
#[derive(Debug, Default)]
pub struct Pool {
    files: Vec<(&'static Family, Role, PathBuf)>,
    /// What was passed over for declaring a profile already found in another version.
    pub conflicts: Vec<String>,
    /// Directories that held no profile file.
    pub empty: Vec<PathBuf>,
}

impl Pool {
    /// Search `dirs`, in order.
    pub fn scan(dirs: &[PathBuf]) -> Pool {
        let mut pool = Pool::default();
        // By family, role and IRI: CGMES's PROF descriptors carry their
        // vocabulary's IRI.
        let mut seen: std::collections::HashMap<(&str, Role, String), (String, PathBuf)> =
            std::collections::HashMap::new();
        for dir in dirs {
            let before = pool.files.len();
            let mut paths = Vec::new();
            walk(dir, SCAN_DEPTH, &mut paths);
            for path in paths {
                let Some(d) = declared(&path) else { continue };
                let key = (d.family.id, d.role, without_version(&d.iri).to_string());
                match seen.get(&key) {
                    Some((iri, _)) if *iri == d.iri => {}
                    Some((iri, first)) => pool.conflicts.push(format!(
                        "{} declares {}, but {} already gave {iri}; using the first",
                        path.display(),
                        d.iri,
                        first.display()
                    )),
                    None => {
                        seen.insert(key, (d.iri.clone(), path.clone()));
                        pool.files.push((d.family, d.role, path));
                    }
                }
            }
            if pool.files.len() == before {
                pool.empty.push(dir.clone());
            }
        }
        pool
    }

    /// The pool [`PROFILES_ENV`] names, searched once per process and its conflicts and
    /// empty directories reported then; `None` when the variable is unset.
    pub fn from_env() -> Option<&'static Pool> {
        static POOL: std::sync::OnceLock<Option<Pool>> = std::sync::OnceLock::new();
        POOL.get_or_init(|| {
            let value = std::env::var_os(PROFILES_ENV)?;
            let dirs: Vec<PathBuf> = std::env::split_paths(&value).collect();
            let pool = Pool::scan(&dirs);
            for dir in &pool.empty {
                eprintln!(
                    "warning: {PROFILES_ENV} lists {}, which holds no CGMES or NC profile files",
                    dir.display()
                );
            }
            for conflict in &pool.conflicts {
                eprintln!("warning: {PROFILES_ENV}: {conflict}");
            }
            Some(pool)
        })
        .as_ref()
    }

    /// The files declaring `family` and `role`, in the order they were found.
    pub fn files(&self, family: &Family, role: Role) -> Vec<PathBuf> {
        self.files
            .iter()
            .filter(|(f, r, _)| f.id == family.id && *r == role)
            .map(|(_, _, p)| p.clone())
            .collect()
    }
}

/// The `.ttl` and `.rdf` files in `dir` and below it, `depth` levels deep: each directory's
/// files, then its subdirectories, in name order; hidden ones skipped.
fn walk(dir: &Path, depth: usize, out: &mut Vec<PathBuf>) {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return;
    };
    let mut entries: Vec<PathBuf> = entries.filter_map(|e| e.ok().map(|e| e.path())).collect();
    entries.sort();
    let hidden = |p: &Path| {
        p.file_name()
            .is_some_and(|n| n.to_string_lossy().starts_with('.'))
    };
    out.extend(
        entries
            .iter()
            .filter(|p| {
                p.is_file() && !hidden(p) && p.extension().is_some_and(|x| x == "ttl" || x == "rdf")
            })
            .cloned(),
    );
    if depth > 0 {
        for sub in entries.iter().filter(|p| p.is_dir() && !hidden(p)) {
            walk(sub, depth - 1, out);
        }
    }
}

/// `…/CoreEquipment-EU/3.0` → `…/CoreEquipment-EU`: the version IRI without a last segment
/// of digits and dots.
fn without_version(iri: &str) -> &str {
    match iri.rsplit_once('/') {
        Some((base, last))
            if !last.is_empty() && last.chars().all(|c| c.is_ascii_digit() || c == '.') =>
        {
            base
        }
        _ => iri,
    }
}

/// The files directly in `dir` that declare `family` and `role`, sorted.
pub fn in_dir(dir: &Path, family: &Family, role: Role) -> Vec<PathBuf> {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return Vec::new();
    };
    let mut paths: Vec<PathBuf> = entries
        .filter_map(|e| e.ok().map(|e| e.path()))
        .filter(|p| p.is_file())
        .filter(|p| identify(p).is_some_and(|(f, r)| f.id == family.id && r == role))
        .collect();
    paths.sort();
    paths
}

/// The files declaring `family` and `role` in `dir` and in the directories directly in it,
/// sorted: ENTSO-E keeps NC's manifests in `SHACL/Validation/`, and the vocabulary and PROF
/// beside `SHACL/`.
pub fn near(dir: &Path, family: &Family, role: Role) -> Vec<PathBuf> {
    let mut paths = in_dir(dir, family, role);
    if let Ok(entries) = std::fs::read_dir(dir) {
        let mut subdirs: Vec<PathBuf> = entries
            .filter_map(|e| e.ok().map(|e| e.path()))
            .filter(|p| p.is_dir())
            .collect();
        subdirs.sort();
        for sub in subdirs {
            paths.extend(in_dir(&sub, family, role));
        }
    }
    paths
}

/// The families whose `role` files are directly in `dir`.
pub fn families_in(dir: &Path, role: Role) -> Vec<&'static Family> {
    FAMILIES
        .iter()
        .copied()
        .filter(|f| !in_dir(dir, f, role).is_empty())
        .collect()
}

fn head(path: &Path) -> Option<String> {
    use std::io::Read;
    let mut bytes = Vec::with_capacity(HEADER_BYTES);
    std::fs::File::open(path)
        .ok()?
        .take(HEADER_BYTES as u64)
        .read_to_end(&mut bytes)
        .ok()?;
    Some(String::from_utf8_lossy(&bytes).into_owned())
}

/// `owl:versionIRI <…>` in Turtle, `owl:versionIRI rdf:resource="…"` in RDF/XML.
fn version_iri(text: &str) -> Option<&str> {
    let rest = &text[text.find("owl:versionIRI")? + "owl:versionIRI".len()..];
    let rest = rest.trim_start();
    if let Some(rest) = rest.strip_prefix('<') {
        return rest.split_once('>').map(|(iri, _)| iri);
    }
    let rest = rest.strip_prefix("rdf:resource")?.trim_start();
    let rest = rest.strip_prefix('=')?.trim_start();
    let quote = rest.chars().next().filter(|c| *c == '"' || *c == '\'')?;
    rest[1..].split_once(quote).map(|(iri, _)| iri)
}

/// The namespace the file binds to `cim:`, if it binds one.
fn cim_namespace(text: &str) -> Option<&str> {
    if let Some(at) = text.find("xmlns:cim=") {
        let rest = &text[at + "xmlns:cim=".len()..];
        let quote = rest.chars().next().filter(|c| *c == '"' || *c == '\'')?;
        return rest[1..].split_once(quote).map(|(ns, _)| ns);
    }
    let mut from = 0;
    while let Some(at) = text[from..].find("@prefix") {
        let rest = text[from + at + "@prefix".len()..].trim_start();
        if let Some(rest) = rest.strip_prefix("cim:") {
            let rest = rest.trim_start().strip_prefix('<')?;
            return rest.split_once('>').map(|(ns, _)| ns);
        }
        from += at + "@prefix".len();
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_version_is_not_part_of_the_profile() {
        assert_eq!(
            without_version("http://iec.ch/TC57/ns/CIM/CoreEquipment-EU/3.0"),
            "http://iec.ch/TC57/ns/CIM/CoreEquipment-EU"
        );
        assert_eq!(
            without_version("https://ap-con.cim4.eu/Contingency-Simple/2.3"),
            "https://ap-con.cim4.eu/Contingency-Simple"
        );
        assert_eq!(
            without_version("https://ap-con.cim4.eu/x"),
            "https://ap-con.cim4.eu/x"
        );
    }

    #[test]
    fn a_turtle_header_is_read() {
        let text = "@prefix cim: <https://cim.ucaiug.io/ns#> .\n\
                    <#x> owl:versionIRI    <https://ap-con.cim4.eu/Contingency-Simple/2.3>;";
        assert_eq!(
            version_iri(text),
            Some("https://ap-con.cim4.eu/Contingency-Simple/2.3")
        );
        assert_eq!(cim_namespace(text), Some("https://cim.ucaiug.io/ns#"));
    }

    #[test]
    fn an_rdf_xml_header_is_read() {
        let text = r#"<rdf:RDF xmlns:cim="http://iec.ch/TC57/CIM100#">
            <owl:versionIRI rdf:resource="http://iec.ch/TC57/ns/CIM/CoreEquipment-EU/3.0"/>"#;
        assert_eq!(
            version_iri(text),
            Some("http://iec.ch/TC57/ns/CIM/CoreEquipment-EU/3.0")
        );
        assert_eq!(cim_namespace(text), Some("http://iec.ch/TC57/CIM100#"));
    }
}
