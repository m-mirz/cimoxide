pub mod common;
pub mod common_solved_mas;
pub mod equipment;
pub mod equipment_not_solved_mas;
pub mod ssh;
pub mod ssh_not_solved_mas;
pub mod shortcircuit;
pub mod shortcircuit_not_solved_mas;
pub mod state_variables;
pub mod state_variables_solved_mas;
pub mod topology_not_solved_mas;
pub mod dynamics;
pub mod diagram_layout;
pub mod equipment_boundary;
pub mod operation;
pub mod prof10;
pub mod quality;

use std::collections::HashSet;
use cimdecoder::CimDataset;
use crate::Violation;

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

pub fn run_validation(dataset: &CimDataset, cfg: &Config) -> Vec<Violation> {
    let profile_selected = |p: &str| -> bool {
        cfg.profiles.is_empty() || cfg.profiles.iter().any(|s| s == p)
    };

    let mut violations: Vec<Violation> = Vec::new();

    if cfg.common {
        violations.extend(common::validate(dataset));
        if cfg.solved {
            violations.extend(common_solved_mas::validate(dataset));
        }
    }

    if profile_selected("EQ") {
        violations.extend(equipment::validate(dataset));
        if cfg.not_solved {
            violations.extend(equipment_not_solved_mas::validate(dataset));
        }
    }

    if profile_selected("SSH") {
        violations.extend(ssh::validate(dataset));
        if cfg.not_solved {
            violations.extend(ssh_not_solved_mas::validate(dataset));
        }
    }

    if profile_selected("TP") {
        if cfg.not_solved {
            violations.extend(topology_not_solved_mas::validate(dataset));
        }
    }

    if profile_selected("DY") {
        violations.extend(dynamics::validate(dataset));
    }

    if profile_selected("SC") {
        violations.extend(shortcircuit::validate(dataset));
        if cfg.not_solved {
            violations.extend(shortcircuit_not_solved_mas::validate(dataset));
        }
    }

    if profile_selected("SV") {
        violations.extend(state_variables::validate(dataset));
        if cfg.solved {
            violations.extend(state_variables_solved_mas::validate(dataset));
        }
    }

    if profile_selected("DL") {
        violations.extend(diagram_layout::validate(dataset));
    }

    if profile_selected("EQBD") {
        violations.extend(equipment_boundary::validate(dataset));
        if let Some(ref eqbd_bv_ids) = cfg.eqbd_base_voltage_ids {
            violations.extend(quality::check_base_voltage_in_eqbd_impl(dataset, eqbd_bv_ids));
        }
    }

    if profile_selected("OP") {
        violations.extend(operation::validate(dataset));
    }

    if cfg.quality {
        violations.extend(quality::validate(dataset));
    }

    violations.extend(prof10::validate(dataset));

    if !cfg.silenced_rules.is_empty() {
        let silenced: HashSet<&str> = cfg.silenced_rules.iter().map(String::as_str).collect();
        violations.retain(|v| !silenced.contains(v.rule_id.as_str()));
    }

    violations
}
