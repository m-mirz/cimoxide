pub mod violation;
pub use violation::Violation;

pub mod sparql;
pub mod detect;
pub use detect::detect_config;

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
pub mod generated_p61968_13_geographicallocation_ap_con_complex_shacl;
pub mod generated_p61970_301_diagramlayout_ap_con_complex_notsolvedmas_shacl;
pub mod generated_p61970_301_diagramlayout_ap_con_complex_shacl;
pub mod generated_p61970_301_equipment_ap_con_complex_shacl;
pub mod generated_p61970_301_equipmentboundary_ap_con_complex_shacl;
pub mod generated_p61970_301_operation_ap_con_complex_shacl;
pub mod generated_p61970_301_shortcircuit_ap_con_complex_shacl;
pub mod generated_p61970_301_statevariables_ap_con_complex_shacl;
pub mod generated_p61970_301_steadystatehypothesis_ap_con_complex_notsolvedmas_shacl;
pub mod generated_p61970_301_steadystatehypothesis_ap_con_complex_shacl;
pub mod generated_p61970_302_dynamics_ap_con_complex_shacl;
pub mod generated_p61970_452_equipment_ap_con_complex_shacl;
pub mod generated_p61970_452_operation_ap_con_complex_shacl;
pub mod generated_p61970_453_diagramlayout_ap_con_complex_implicit_crossprofile_shacl;
pub mod generated_p61970_453_diagramlayout_ap_con_complex_shacl;
pub mod generated_p61970_456_statevariables_ap_con_complex_explicit_crossprofile_shacl;
pub mod generated_p61970_456_statevariables_ap_con_complex_implicit_crossprofile_shacl;
pub mod generated_p61970_456_steadystatehypothesis_ap_con_complex_notsolvedmas_shacl;
pub mod generated_p61970_456_steadystatehypothesis_ap_con_complex_shacl;
pub mod generated_p61970_456_topology_ap_con_complex_implicit_crossprofile_shacl;
pub mod generated_p61970_456_topology_ap_con_complex_shacl;
pub mod generated_p61970_457_dynamics_ap_con_complex_implicit_crossprofile_shacl;
pub mod generated_p61970_552_header_ap_con_simple_shacl;
pub mod generated_p61970_600_1_equipment_ap_con_complex_shacl;
pub mod generated_p61970_600_2_diagramlayout_ap_con_simple_shacl;
pub mod generated_p61970_600_2_dynamics_ap_con_simple_shacl;
pub mod generated_p61970_600_2_equipment_ap_con_complex_shacl;
pub mod generated_p61970_600_2_equipment_ap_con_simple_shacl;
pub mod generated_p61970_600_2_equipmentboundary_ap_con_simple_shacl;
pub mod generated_p61970_600_2_geographicallocation_ap_con_complex_implicit_crossprofile_shacl;
pub mod generated_p61970_600_2_geographicallocation_ap_con_simple_shacl;
pub mod generated_p61970_600_2_operation_ap_con_complex_implicit_crossprofile_shacl;
pub mod generated_p61970_600_2_operation_ap_con_simple_shacl;
pub mod generated_p61970_600_2_shortcircuit_ap_con_simple_shacl;
pub mod generated_p61970_600_2_statevariables_ap_con_simple_shacl;
pub mod generated_p61970_600_2_steadystatehypothesis_ap_con_simple_shacl;
pub mod generated_p61970_600_2_topology_ap_con_simple_shacl;

// ── per-profile local rules ────────────────────────────────────────────────

fn validate_dl_local(dataset: &cimdecoder::CimDataset, cfg: &Config) -> Vec<Violation> {
    let mut v = Vec::new();
    if cfg.not_solved {
        v.extend(generated_p61970_301_diagramlayout_ap_con_complex_notsolvedmas_shacl::validate_p61970_301_diagramlayout_ap_con_complex_notsolvedmas_shacl(dataset));
    }
    v.extend(generated_p61970_301_diagramlayout_ap_con_complex_shacl::validate_p61970_301_diagramlayout_ap_con_complex_shacl(dataset));
    v.extend(generated_p61970_453_diagramlayout_ap_con_complex_shacl::validate_p61970_453_diagramlayout_ap_con_complex_shacl(dataset));
    v.extend(generated_p61970_600_2_diagramlayout_ap_con_simple_shacl::validate_p61970_600_2_diagramlayout_ap_con_simple_shacl(dataset));
    v
}

fn validate_dl_crossprofile(dataset: &cimdecoder::CimDataset, _cfg: &Config) -> Vec<Violation> {
    generated_p61970_453_diagramlayout_ap_con_complex_implicit_crossprofile_shacl::validate_p61970_453_diagramlayout_ap_con_complex_implicit_crossprofile_shacl(dataset)
}

fn validate_dy_local(dataset: &cimdecoder::CimDataset, _cfg: &Config) -> Vec<Violation> {
    let mut v = Vec::new();
    v.extend(generated_p61970_302_dynamics_ap_con_complex_shacl::validate_p61970_302_dynamics_ap_con_complex_shacl(dataset));
    v.extend(generated_p61970_600_2_dynamics_ap_con_simple_shacl::validate_p61970_600_2_dynamics_ap_con_simple_shacl(dataset));
    v
}

fn validate_dy_crossprofile(dataset: &cimdecoder::CimDataset, _cfg: &Config) -> Vec<Violation> {
    generated_p61970_457_dynamics_ap_con_complex_implicit_crossprofile_shacl::validate_p61970_457_dynamics_ap_con_complex_implicit_crossprofile_shacl(dataset)
}

pub fn validate_eq(dataset: &cimdecoder::CimDataset, _cfg: &Config) -> Vec<Violation> {
    let mut v = Vec::new();
    v.extend(generated_p61970_301_equipment_ap_con_complex_shacl::validate_p61970_301_equipment_ap_con_complex_shacl(dataset));
    v.extend(generated_p61970_452_equipment_ap_con_complex_shacl::validate_p61970_452_equipment_ap_con_complex_shacl(dataset));
    v.extend(generated_p61970_600_1_equipment_ap_con_complex_shacl::validate_p61970_600_1_equipment_ap_con_complex_shacl(dataset));
    v.extend(generated_p61970_600_2_equipment_ap_con_complex_shacl::validate_p61970_600_2_equipment_ap_con_complex_shacl(dataset));
    v.extend(generated_p61970_600_2_equipment_ap_con_simple_shacl::validate_p61970_600_2_equipment_ap_con_simple_shacl(dataset));
    v
}

pub fn validate_eqbd(dataset: &cimdecoder::CimDataset, _cfg: &Config) -> Vec<Violation> {
    let mut v = Vec::new();
    v.extend(generated_p61970_301_equipmentboundary_ap_con_complex_shacl::validate_p61970_301_equipmentboundary_ap_con_complex_shacl(dataset));
    v.extend(generated_p61970_600_2_equipmentboundary_ap_con_simple_shacl::validate_p61970_600_2_equipmentboundary_ap_con_simple_shacl(dataset));
    v
}

fn validate_gl_local(dataset: &cimdecoder::CimDataset, _cfg: &Config) -> Vec<Violation> {
    let mut v = Vec::new();
    v.extend(generated_p61968_13_geographicallocation_ap_con_complex_shacl::validate_p61968_13_geographicallocation_ap_con_complex_shacl(dataset));
    v.extend(generated_p61970_600_2_geographicallocation_ap_con_simple_shacl::validate_p61970_600_2_geographicallocation_ap_con_simple_shacl(dataset));
    v
}

fn validate_gl_crossprofile(dataset: &cimdecoder::CimDataset, _cfg: &Config) -> Vec<Violation> {
    generated_p61970_600_2_geographicallocation_ap_con_complex_implicit_crossprofile_shacl::validate_p61970_600_2_geographicallocation_ap_con_complex_implicit_crossprofile_shacl(dataset)
}

fn validate_op_local(dataset: &cimdecoder::CimDataset, _cfg: &Config) -> Vec<Violation> {
    let mut v = Vec::new();
    v.extend(generated_p61970_301_operation_ap_con_complex_shacl::validate_p61970_301_operation_ap_con_complex_shacl(dataset));
    v.extend(generated_p61970_452_operation_ap_con_complex_shacl::validate_p61970_452_operation_ap_con_complex_shacl(dataset));
    v.extend(generated_p61970_600_2_operation_ap_con_simple_shacl::validate_p61970_600_2_operation_ap_con_simple_shacl(dataset));
    v
}

fn validate_op_crossprofile(dataset: &cimdecoder::CimDataset, _cfg: &Config) -> Vec<Violation> {
    generated_p61970_600_2_operation_ap_con_complex_implicit_crossprofile_shacl::validate_p61970_600_2_operation_ap_con_complex_implicit_crossprofile_shacl(dataset)
}

pub fn validate_sc(dataset: &cimdecoder::CimDataset, _cfg: &Config) -> Vec<Violation> {
    let mut v = Vec::new();
    v.extend(generated_p61970_301_shortcircuit_ap_con_complex_shacl::validate_p61970_301_shortcircuit_ap_con_complex_shacl(dataset));
    v.extend(generated_p61970_600_2_shortcircuit_ap_con_simple_shacl::validate_p61970_600_2_shortcircuit_ap_con_simple_shacl(dataset));
    v
}

pub fn validate_ssh(dataset: &cimdecoder::CimDataset, cfg: &Config) -> Vec<Violation> {
    let mut v = Vec::new();
    if cfg.not_solved {
        v.extend(generated_p61970_301_steadystatehypothesis_ap_con_complex_notsolvedmas_shacl::validate_p61970_301_steadystatehypothesis_ap_con_complex_notsolvedmas_shacl(dataset));
    }
    v.extend(generated_p61970_301_steadystatehypothesis_ap_con_complex_shacl::validate_p61970_301_steadystatehypothesis_ap_con_complex_shacl(dataset));
    if cfg.not_solved {
        v.extend(generated_p61970_456_steadystatehypothesis_ap_con_complex_notsolvedmas_shacl::validate_p61970_456_steadystatehypothesis_ap_con_complex_notsolvedmas_shacl(dataset));
    }
    v.extend(generated_p61970_456_steadystatehypothesis_ap_con_complex_shacl::validate_p61970_456_steadystatehypothesis_ap_con_complex_shacl(dataset));
    v.extend(generated_p61970_600_2_steadystatehypothesis_ap_con_simple_shacl::validate_p61970_600_2_steadystatehypothesis_ap_con_simple_shacl(dataset));
    v
}

fn validate_sv_local(dataset: &cimdecoder::CimDataset, _cfg: &Config) -> Vec<Violation> {
    let mut v = Vec::new();
    v.extend(generated_p61970_301_statevariables_ap_con_complex_shacl::validate_p61970_301_statevariables_ap_con_complex_shacl(dataset));
    v.extend(generated_p61970_600_2_statevariables_ap_con_simple_shacl::validate_p61970_600_2_statevariables_ap_con_simple_shacl(dataset));
    v
}

fn validate_sv_crossprofile(dataset: &cimdecoder::CimDataset, _cfg: &Config) -> Vec<Violation> {
    let mut v = Vec::new();
    v.extend(generated_p61970_456_statevariables_ap_con_complex_explicit_crossprofile_shacl::validate_p61970_456_statevariables_ap_con_complex_explicit_crossprofile_shacl(dataset));
    v.extend(generated_p61970_456_statevariables_ap_con_complex_implicit_crossprofile_shacl::validate_p61970_456_statevariables_ap_con_complex_implicit_crossprofile_shacl(dataset));
    v
}

fn validate_tp_local(dataset: &cimdecoder::CimDataset, _cfg: &Config) -> Vec<Violation> {
    let mut v = Vec::new();
    v.extend(generated_p61970_456_topology_ap_con_complex_shacl::validate_p61970_456_topology_ap_con_complex_shacl(dataset));
    v.extend(generated_p61970_600_2_topology_ap_con_simple_shacl::validate_p61970_600_2_topology_ap_con_simple_shacl(dataset));
    v
}

fn validate_tp_crossprofile(dataset: &cimdecoder::CimDataset, _cfg: &Config) -> Vec<Violation> {
    generated_p61970_456_topology_ap_con_complex_implicit_crossprofile_shacl::validate_p61970_456_topology_ap_con_complex_implicit_crossprofile_shacl(dataset)
}

// ── public two-phase API ───────────────────────────────────────────────────

/// Phase 1 — header: validate FullModel/DifferenceModel rules for a single file's dataset.
pub fn validate_header(dataset: &cimdecoder::CimDataset, _cfg: &Config) -> Vec<Violation> {
    generated_p61970_552_header_ap_con_simple_shacl::validate_p61970_552_header_ap_con_simple_shacl(dataset)
}

/// Phase 1 — per-profile: run local (non-crossprofile) generated SHACL rules and SPARQL
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
    let mut v = match profile {
        "DL"   => validate_dl_local(dataset, cfg),
        "DY"   => validate_dy_local(dataset, cfg),
        "EQ"   => validate_eq(dataset, cfg),
        "EQBD" => validate_eqbd(dataset, cfg),
        "GL"   => validate_gl_local(dataset, cfg),
        "OP"   => validate_op_local(dataset, cfg),
        "SC"   => validate_sc(dataset, cfg),
        "SSH"  => validate_ssh(dataset, cfg),
        "SV"   => validate_sv_local(dataset, cfg),
        "TP"   => validate_tp_local(dataset, cfg),
        _      => Vec::new(),
    };
    v.extend(sparql::validate_profile_local(dataset, profile, cfg));
    v
}

/// Phase 2 — crossprofile: run crossprofile SHACL + cross-profile SPARQL on the merged dataset.
pub fn validate_crossprofile(dataset: &cimdecoder::CimDataset, cfg: &Config) -> Vec<Violation> {
    let has = |p: &str| cfg.profiles.is_empty() || cfg.profiles.iter().any(|x| x == p);

    let mut v = Vec::new();
    if has("DL") { v.extend(validate_dl_crossprofile(dataset, cfg)); }
    if has("DY") { v.extend(validate_dy_crossprofile(dataset, cfg)); }
    if has("GL") { v.extend(validate_gl_crossprofile(dataset, cfg)); }
    if has("OP") { v.extend(validate_op_crossprofile(dataset, cfg)); }
    if has("SV") { v.extend(validate_sv_crossprofile(dataset, cfg)); }
    if has("TP") { v.extend(validate_tp_crossprofile(dataset, cfg)); }

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
        for p in c.profiles {
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
                    let file_cfg = detect_config(ds);
                    for profile in &file_cfg.profiles {
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
