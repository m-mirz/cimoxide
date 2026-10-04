pub mod violation;
pub use violation::Violation;

pub mod sparql;
pub mod detect;
pub use detect::{detect_config, detect_nc_profiles};

use std::collections::HashSet;

#[derive(Debug, Default, Clone)]
pub struct Config {
    /// Profile short names to validate (e.g. "EQ", "SSH"). Empty = all detected.
    pub profiles: Vec<String>,
    /// True if SV profile (power-flow results) is present.
    pub solved: bool,
    /// True if SV profile is absent.
    pub not_solved: bool,
    /// Run common cross-profile checks.
    pub common: bool,
    /// Run CIMdesk modeling quality checks.
    pub quality: bool,
    /// Rule IDs to suppress in the output.
    pub silenced_rules: Vec<String>,
    /// When non-empty, enables the EQBD2 base voltage check.
    pub eqbd_base_voltage_ids: Option<HashSet<String>>,
}

pub mod helpers;

// ── property-bag families ──────────────────────────────────────────────────
//
// NC classes decode into GenericElement bags; their shapes are a data table
// interpreted by `bag`, the same interpreter CGMES uses below.
pub mod bag;
pub mod shapes;
pub mod shape_source;
pub mod nc_shapes;
pub mod nc_profiles;

/// The NC shape table in force: loaded from SHACL if the `dynamic-shapes`
/// feature is on and a directory was supplied, otherwise the generated one.
pub fn nc_shapes() -> &'static [shapes::ShapeDef] {
    static R: std::sync::OnceLock<&'static [shapes::ShapeDef]> = std::sync::OnceLock::new();
    *R.get_or_init(|| shape_source::resolve("nc", nc_shapes::SHAPES))
}

/// The NC profile index in force. Loaded together with the shapes, since an
/// index and a table from different releases would run the wrong rules.
pub fn nc_profile_index() -> (&'static [(&'static str, &'static str)], &'static [&'static str]) {
    static R: std::sync::OnceLock<(
        &'static [(&'static str, &'static str)],
        &'static [&'static str],
    )> = std::sync::OnceLock::new();
    *R.get_or_init(|| {
        shape_source::resolve_profiles("nc", nc_profiles::PROFILE_IRIS, nc_profiles::PROFILES)
    })
}

/// Run one NC profile's shapes against a dataset.
///
/// Separate from `validate_profile_local` so a caller can drive it with an
/// explicit profile code, without depending on header detection.
pub fn validate_nc_profile(
    dataset: &cimdecoder::CimDataset,
    profile: &str,
    cfg: &Config,
) -> Vec<Violation> {
    bag::validate_profile(dataset, profile, nc_shapes(), cfg)
}


// ── CGMES shape table ──────────────────────────────────────────────────────
//
// CGMES runs through the same interpreter as NC, reading the typed elements'
// decoder blocks; it replaced ~250k lines of generated per-check functions.
// Its constraint files carry tags rather than profile codes — see
// `cimschema::shacl::cgmes_manifest` — so one table holds the local,
// not-solved, cross-profile and header rules apart.
pub mod cgmes_shapes;

/// The CGMES shape table in force: loaded from SHACL if the `dynamic-shapes`
/// feature is on and a directory was supplied, otherwise the generated one.
pub fn cgmes_shapes() -> &'static [shapes::ShapeDef] {
    static R: std::sync::OnceLock<&'static [shapes::ShapeDef]> = std::sync::OnceLock::new();
    *R.get_or_init(|| shape_source::resolve("cgmes", cgmes_shapes::SHAPES))
}

/// The CGMES shapes carrying one manifest tag, indexed once per process.
fn cgmes_tagged(tag: &str) -> &'static [&'static shapes::ShapeDef] {
    type Index = std::collections::HashMap<&'static str, Vec<&'static shapes::ShapeDef>>;
    static IDX: std::sync::OnceLock<Index> = std::sync::OnceLock::new();
    let idx = IDX.get_or_init(|| {
        let mut idx = Index::new();
        for shape in cgmes_shapes() {
            for tag in shape.profiles {
                idx.entry(*tag).or_default().push(shape);
            }
        }
        idx
    });
    idx.get(tag).map_or(&[], Vec::as_slice)
}

fn run_cgmes(dataset: &cimdecoder::CimDataset, active: &[&shapes::ShapeDef]) -> Vec<Violation> {
    bag::validate_shapes(dataset, bag::Source::Typed, active)
}

// ── public two-phase API ───────────────────────────────────────────────────

/// Phase 1 — header: validate FullModel/DifferenceModel rules for a single file's dataset.
pub fn validate_header(dataset: &cimdecoder::CimDataset, _cfg: &Config) -> Vec<Violation> {
    run_cgmes(dataset, cgmes_tagged("HDR"))
}

/// The SHACL half of [`validate_profile_local`] for one CGMES profile: its
/// local rules, plus its not-solved rules when `cfg.not_solved`. Without the
/// hand-written `sparql` rules or the `cfg.profiles` filter.
pub fn validate_profile_shacl(dataset: &cimdecoder::CimDataset, profile: &str, cfg: &Config) -> Vec<Violation> {
    let mut active: Vec<&shapes::ShapeDef> = cgmes_tagged(profile).to_vec();
    if cfg.not_solved {
        active.extend_from_slice(cgmes_tagged(&format!("{profile}!NS")));
    }
    // One call, so the indexes are built once for both sets.
    run_cgmes(dataset, &active)
}

/// The SHACL half of [`validate_crossprofile`]: every enabled profile's
/// cross-profile rules on the merged dataset, plus the rules for every profile
/// when `cfg.common`.
pub fn validate_crossprofile_shacl(dataset: &cimdecoder::CimDataset, cfg: &Config) -> Vec<Violation> {
    let has = |p: &str| cfg.profiles.is_empty() || cfg.profiles.iter().any(|x| x == p);
    let mut active: Vec<&shapes::ShapeDef> = cimschema_cross_profiles()
        .iter()
        .filter(|p| has(p))
        .flat_map(|p| cgmes_tagged(&format!("X:{p}")).iter().copied())
        .collect();
    if cfg.common {
        active.extend_from_slice(cgmes_tagged("COMMON"));
    }
    run_cgmes(dataset, &active)
}

/// Every CGMES profile code `detect_config` can report.
const CGMES_PROFILES: &[&str] = &["DL", "DY", "EQ", "EQBD", "GL", "OP", "SC", "SSH", "SV", "TP"];

/// The profiles with cross-profile rules. Mirrors
/// `cimschema::shacl::cgmes_manifest::CROSS_PROFILES`, which this crate does
/// not depend on outside the `dynamic-shapes` feature.
fn cimschema_cross_profiles() -> &'static [&'static str] {
    &["DL", "DY", "GL", "OP", "SV", "TP"]
}


/// Phase 1 — per-profile: run the local (non-crossprofile) SHACL rules and the SPARQL
/// rules for one profile.
///
/// Pass the single-file dataset for the profile and the combined config (solved/not_solved
/// must reflect the full set of files, not just this file). If `cfg.profiles` is non-empty
/// and does not include `profile`, returns an empty vec — this filter applies to both the
/// SHACL and SPARQL checks.
pub fn validate_profile_local(dataset: &cimdecoder::CimDataset, profile: &str, cfg: &Config) -> Vec<Violation> {
    if !cfg.profiles.is_empty() && !cfg.profiles.iter().any(|p| p == profile) {
        return Vec::new();
    }
    // NC profile codes cannot collide with CGMES ones, so one flat
    // `Config::profiles` list carries both families. The CGMES codes are
    // checked first: they are fixed, and asking the NC index loads the NC
    // table — 170 ms when it comes from SHACL — for data that has no NC in it.
    if !CGMES_PROFILES.contains(&profile) && nc_profile_index().1.contains(&profile) {
        return validate_nc_profile(dataset, profile, cfg);
    }

    let mut v = validate_profile_shacl(dataset, profile, cfg);
    v.extend(sparql::validate_profile_local(dataset, profile, cfg));
    v
}


/// Phase 2 — crossprofile: run crossprofile SHACL + cross-profile SPARQL on the merged dataset.
pub fn validate_crossprofile(dataset: &cimdecoder::CimDataset, cfg: &Config) -> Vec<Violation> {
    let mut v = validate_crossprofile_shacl(dataset, cfg);
    v.extend(sparql::validate_crossprofile(dataset, cfg));
    v
}


/// Build a combined `Config` by auto-detecting profiles/solved-state across all files,
/// then applying explicit overrides (each `None`/default leaves the detected value).
pub fn combined_config(
    per_file: &[cimdecoder::CimDataset],
    profiles: Option<Vec<String>>,
    solved: Option<bool>,
    common: bool,
    quality: bool,
    silenced_rules: Vec<String>,
) -> Config {
    let mut cfg = Config::default();
    for ds in per_file {
        let c = detect_config(ds);
        for p in c.profiles.into_iter().chain(detect_nc_profiles(ds)) {
            if !cfg.profiles.contains(&p) {
                cfg.profiles.push(p);
            }
        }
        cfg.solved |= c.solved;
    }
    cfg.not_solved = !cfg.solved;
    if let Some(p) = profiles { cfg.profiles = p; }
    if let Some(s) = solved { cfg.solved = s; cfg.not_solved = !s; }
    cfg.common = common;
    cfg.quality = quality;
    cfg.silenced_rules = silenced_rules;
    cfg
}

/// Run full two-phase validation: per-file local checks (header + per-profile SHACL/SPARQL)
/// in parallel, then crossprofile checks on the merged dataset, then apply rule silencing.
///
/// Consumes `per_file` since it merges them into one dataset for phase 2.
pub fn validate_files(per_file: Vec<cimdecoder::CimDataset>, cfg: &Config) -> Vec<Violation> {
    let mut violations: Vec<Violation> = std::thread::scope(|s| {
        per_file
            .iter()
            .map(|ds| {
                s.spawn(move || {
                    let mut v = validate_header(ds, cfg);
                    let mut profiles = detect_config(ds).profiles;
                    profiles.extend(detect_nc_profiles(ds));
                    for profile in &profiles {
                        v.extend(validate_profile_local(ds, profile, cfg));
                    }
                    v
                })
            })
            .collect::<Vec<_>>()
            .into_iter()
            .flat_map(|h| h.join().expect("validation thread panicked"))
            .collect()
    });

    let mut merged = cimdecoder::CimDataset::new();
    for ds in per_file {
        merged.merge(ds);
    }
    violations.extend(validate_crossprofile(&merged, cfg));

    if !cfg.silenced_rules.is_empty() {
        let silenced: HashSet<&str> = cfg.silenced_rules.iter().map(String::as_str).collect();
        violations.retain(|v| !silenced.contains(v.rule_id.as_str()));
    }
    violations
}
