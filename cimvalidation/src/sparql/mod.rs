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

use cimdecoder::CimDataset;
use crate::{Config, Violation};

pub fn validate(dataset: &CimDataset, cfg: &Config) -> Vec<Violation> {
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

    violations
}
