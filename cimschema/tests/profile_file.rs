//! Every file ENTSO-E ships declares its family and role, so nothing has to go by where it sits.

use std::path::{Path, PathBuf};

use cimschema::profile_file::{Role, identify};

fn root() -> &'static Path {
    Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap()
}

fn files(dir: &str, extension: &str) -> Vec<PathBuf> {
    let mut paths: Vec<PathBuf> = std::fs::read_dir(root().join(dir))
        .unwrap_or_else(|e| panic!("{dir}: {e}"))
        .filter_map(|e| e.ok().map(|e| e.path()))
        .filter(|p| p.extension().is_some_and(|x| x == extension))
        .collect();
    paths.sort();
    assert!(!paths.is_empty(), "no .{extension} in {dir}");
    paths
}

fn assert_all(dir: &str, extension: &str, family: &str, role: Role, except: &[&str]) {
    for path in files(dir, extension) {
        let name = path.file_name().unwrap().to_string_lossy();
        if except.iter().any(|e| name == *e) {
            assert!(identify(&path).is_none(), "{name}");
            continue;
        }
        let (f, r) = identify(&path).unwrap_or_else(|| panic!("{name} declares nothing"));
        assert_eq!((f.id, r), (family, role), "{name}");
    }
}

#[test]
fn every_cgmes_file_declares_itself() {
    assert_all("CGMES-AP/CGMES/SHACL", "ttl", "cgmes", Role::Shapes, &[]);
    // The 2019 header vocabulary has no version IRI; the family loads the 2020 files.
    assert_all(
        "CGMES-AP/CGMES/RDFS",
        "rdf",
        "cgmes",
        Role::Vocabulary,
        &["61970-600-2_Header-AP-Voc-RDFS2019.rdf"],
    );
    assert_all("CGMES-AP/CGMES/PROF", "rdf", "cgmes", Role::Descriptor, &[]);
}

#[test]
fn every_nc_file_declares_itself() {
    assert_all("NC-AP/NCP/SHACL", "ttl", "nc", Role::Shapes, &[]);
    assert_all(
        "NC-AP/NCP/SHACL/Validation",
        "ttl",
        "nc",
        Role::Manifest,
        &[],
    );
    assert_all("NC-AP/NCP/RDFS", "rdf", "nc", Role::Vocabulary, &[]);
    assert_all("NC-AP/NCP/PROF", "rdf", "nc", Role::Descriptor, &[]);
}

/// A file that shares NC's IRIs but binds another CIM is neither family's: ENTSO-E's QoCDC
/// rules for CGMES 2.4 are written that way.
#[test]
fn another_cim_is_no_family() {
    let dir = Path::new(env!("CARGO_TARGET_TMPDIR")).join("profile-file");
    std::fs::create_dir_all(&dir).unwrap();
    let path = dir.join("Level4_IGM.ttl");
    std::fs::write(
        &path,
        "@prefix cim: <http://iec.ch/TC57/2013/CIM-schema-cim16#> .\n\
         <#Ontology> owl:versionIRI <https://ap-con.cim4.eu/QoCDC-Level4-IGM/4.1.4> .\n",
    )
    .unwrap();
    assert!(identify(&path).is_none());
}

/// One search of both repositories finds every file once. CGMES-AP's
/// `RDFS_Beta_501_Ed2_CD` gives the same IRIs again, and is passed over without a
/// conflict; its SHACL, written as RDF/XML, is no vocabulary.
#[test]
fn a_pool_of_both_repositories_finds_each_file_once() {
    use cimschema::family::{CGMES, NC};
    use cimschema::profile_file::Pool;
    let pool = Pool::scan(&[root().join("CGMES-AP"), root().join("NC-AP")]);
    assert!(pool.conflicts.is_empty(), "{:?}", pool.conflicts);
    assert!(pool.empty.is_empty());
    let count = |dir: &str, extension: &str| files(dir, extension).len();
    for (family, role, expected) in [
        (&CGMES, Role::Shapes, count("CGMES-AP/CGMES/SHACL", "ttl")),
        // Less the 2019 header, which has no version IRI.
        (
            &CGMES,
            Role::Vocabulary,
            count("CGMES-AP/CGMES/RDFS", "rdf") - 1,
        ),
        (
            &CGMES,
            Role::Descriptor,
            count("CGMES-AP/CGMES/PROF", "rdf"),
        ),
        (&NC, Role::Shapes, count("NC-AP/NCP/SHACL", "ttl")),
        (
            &NC,
            Role::Manifest,
            count("NC-AP/NCP/SHACL/Validation", "ttl"),
        ),
        (&NC, Role::Vocabulary, count("NC-AP/NCP/RDFS", "rdf")),
        (&NC, Role::Descriptor, count("NC-AP/NCP/PROF", "rdf")),
    ] {
        let found = pool.files(family, role);
        assert_eq!(found.len(), expected, "{} {role:?}", family.id);
        assert!(
            found.iter().all(|p| !p.to_string_lossy().contains("Beta")),
            "{} {role:?} took a beta file",
            family.id
        );
    }
}

/// Another version of a profile already found is reported, and the first stays.
#[test]
fn another_version_of_a_profile_is_a_conflict() {
    use cimschema::family::NC;
    use cimschema::profile_file::Pool;
    let base = Path::new(env!("CARGO_TARGET_TMPDIR")).join("profile-versions");
    let _ = std::fs::remove_dir_all(&base);
    let shapes = |version: &str| {
        format!(
            "@prefix cim: <https://cim.ucaiug.io/ns#> .\n\
             <#Ontology> owl:versionIRI <https://ap-con.cim4.eu/Contingency-Simple/{version}> .\n"
        )
    };
    for (dir, version) in [("a", "2.3"), ("b", "2.3"), ("c", "2.4")] {
        std::fs::create_dir_all(base.join(dir)).unwrap();
        std::fs::write(base.join(dir).join("Contingency.ttl"), shapes(version)).unwrap();
    }
    let pool = Pool::scan(&[base.join("a"), base.join("b"), base.join("c")]);
    assert_eq!(
        pool.files(&NC, Role::Shapes),
        [base.join("a/Contingency.ttl")]
    );
    // b is a copy, c another version.
    assert_eq!(pool.conflicts.len(), 1, "{:?}", pool.conflicts);
    assert!(pool.conflicts[0].contains("Contingency-Simple/2.4"));
    assert_eq!(pool.empty, [base.join("b"), base.join("c")]);
}
