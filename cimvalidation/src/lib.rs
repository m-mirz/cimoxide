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

pub fn validate_dl(dataset: &cimdecoder::CimDataset, cfg: &Config) -> Vec<Violation> {
    let mut v = Vec::new();
    if cfg.not_solved {
        v.extend(generated_p61970_301_diagramlayout_ap_con_complex_notsolvedmas_shacl::validate_p61970_301_diagramlayout_ap_con_complex_notsolvedmas_shacl(dataset));
    }
    v.extend(generated_p61970_301_diagramlayout_ap_con_complex_shacl::validate_p61970_301_diagramlayout_ap_con_complex_shacl(dataset));
    v.extend(generated_p61970_453_diagramlayout_ap_con_complex_implicit_crossprofile_shacl::validate_p61970_453_diagramlayout_ap_con_complex_implicit_crossprofile_shacl(dataset));
    v.extend(generated_p61970_453_diagramlayout_ap_con_complex_shacl::validate_p61970_453_diagramlayout_ap_con_complex_shacl(dataset));
    v.extend(generated_p61970_600_2_diagramlayout_ap_con_simple_shacl::validate_p61970_600_2_diagramlayout_ap_con_simple_shacl(dataset));
    v
}

pub fn validate_dy(dataset: &cimdecoder::CimDataset, _cfg: &Config) -> Vec<Violation> {
    let mut v = Vec::new();
    v.extend(generated_p61970_302_dynamics_ap_con_complex_shacl::validate_p61970_302_dynamics_ap_con_complex_shacl(dataset));
    v.extend(generated_p61970_457_dynamics_ap_con_complex_implicit_crossprofile_shacl::validate_p61970_457_dynamics_ap_con_complex_implicit_crossprofile_shacl(dataset));
    v.extend(generated_p61970_600_2_dynamics_ap_con_simple_shacl::validate_p61970_600_2_dynamics_ap_con_simple_shacl(dataset));
    v
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

pub fn validate_gl(dataset: &cimdecoder::CimDataset, _cfg: &Config) -> Vec<Violation> {
    let mut v = Vec::new();
    v.extend(generated_p61968_13_geographicallocation_ap_con_complex_shacl::validate_p61968_13_geographicallocation_ap_con_complex_shacl(dataset));
    v.extend(generated_p61970_600_2_geographicallocation_ap_con_complex_implicit_crossprofile_shacl::validate_p61970_600_2_geographicallocation_ap_con_complex_implicit_crossprofile_shacl(dataset));
    v.extend(generated_p61970_600_2_geographicallocation_ap_con_simple_shacl::validate_p61970_600_2_geographicallocation_ap_con_simple_shacl(dataset));
    v
}

pub fn validate_op(dataset: &cimdecoder::CimDataset, _cfg: &Config) -> Vec<Violation> {
    let mut v = Vec::new();
    v.extend(generated_p61970_301_operation_ap_con_complex_shacl::validate_p61970_301_operation_ap_con_complex_shacl(dataset));
    v.extend(generated_p61970_452_operation_ap_con_complex_shacl::validate_p61970_452_operation_ap_con_complex_shacl(dataset));
    v.extend(generated_p61970_600_2_operation_ap_con_complex_implicit_crossprofile_shacl::validate_p61970_600_2_operation_ap_con_complex_implicit_crossprofile_shacl(dataset));
    v.extend(generated_p61970_600_2_operation_ap_con_simple_shacl::validate_p61970_600_2_operation_ap_con_simple_shacl(dataset));
    v
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

pub fn validate_sv(dataset: &cimdecoder::CimDataset, _cfg: &Config) -> Vec<Violation> {
    let mut v = Vec::new();
    v.extend(generated_p61970_301_statevariables_ap_con_complex_shacl::validate_p61970_301_statevariables_ap_con_complex_shacl(dataset));
    v.extend(generated_p61970_456_statevariables_ap_con_complex_explicit_crossprofile_shacl::validate_p61970_456_statevariables_ap_con_complex_explicit_crossprofile_shacl(dataset));
    v.extend(generated_p61970_456_statevariables_ap_con_complex_implicit_crossprofile_shacl::validate_p61970_456_statevariables_ap_con_complex_implicit_crossprofile_shacl(dataset));
    v.extend(generated_p61970_600_2_statevariables_ap_con_simple_shacl::validate_p61970_600_2_statevariables_ap_con_simple_shacl(dataset));
    v
}

pub fn validate_tp(dataset: &cimdecoder::CimDataset, _cfg: &Config) -> Vec<Violation> {
    let mut v = Vec::new();
    v.extend(generated_p61970_456_topology_ap_con_complex_implicit_crossprofile_shacl::validate_p61970_456_topology_ap_con_complex_implicit_crossprofile_shacl(dataset));
    v.extend(generated_p61970_456_topology_ap_con_complex_shacl::validate_p61970_456_topology_ap_con_complex_shacl(dataset));
    v.extend(generated_p61970_600_2_topology_ap_con_simple_shacl::validate_p61970_600_2_topology_ap_con_simple_shacl(dataset));
    v
}

pub fn validate_generated(dataset: &cimdecoder::CimDataset, cfg: &Config) -> Vec<Violation> {
    let mut v = Vec::new();
    v.extend(generated_p61970_552_header_ap_con_simple_shacl::validate_p61970_552_header_ap_con_simple_shacl(dataset));
    if cfg.profiles.is_empty() || cfg.profiles.iter().any(|p| p == "DL") {
        v.extend(validate_dl(dataset, cfg));
    }
    if cfg.profiles.is_empty() || cfg.profiles.iter().any(|p| p == "DY") {
        v.extend(validate_dy(dataset, cfg));
    }
    if cfg.profiles.is_empty() || cfg.profiles.iter().any(|p| p == "EQ") {
        v.extend(validate_eq(dataset, cfg));
    }
    if cfg.profiles.is_empty() || cfg.profiles.iter().any(|p| p == "EQBD") {
        v.extend(validate_eqbd(dataset, cfg));
    }
    if cfg.profiles.is_empty() || cfg.profiles.iter().any(|p| p == "GL") {
        v.extend(validate_gl(dataset, cfg));
    }
    if cfg.profiles.is_empty() || cfg.profiles.iter().any(|p| p == "OP") {
        v.extend(validate_op(dataset, cfg));
    }
    if cfg.profiles.is_empty() || cfg.profiles.iter().any(|p| p == "SC") {
        v.extend(validate_sc(dataset, cfg));
    }
    if cfg.profiles.is_empty() || cfg.profiles.iter().any(|p| p == "SSH") {
        v.extend(validate_ssh(dataset, cfg));
    }
    if cfg.profiles.is_empty() || cfg.profiles.iter().any(|p| p == "SV") {
        v.extend(validate_sv(dataset, cfg));
    }
    if cfg.profiles.is_empty() || cfg.profiles.iter().any(|p| p == "TP") {
        v.extend(validate_tp(dataset, cfg));
    }
    v
}

pub fn validate(dataset: &cimdecoder::CimDataset, cfg: &Config) -> Vec<Violation> {
    let mut v = validate_generated(dataset, cfg);
    v.extend(sparql::validate(dataset, cfg));
    if !cfg.silenced_rules.is_empty() {
        let silenced: HashSet<&str> = cfg.silenced_rules.iter().map(String::as_str).collect();
        v.retain(|x| !silenced.contains(x.rule_id.as_str()));
    }
    v
}
