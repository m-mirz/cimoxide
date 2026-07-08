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

/// Per-profile SPARQL checks that only need data from a single profile's file.
pub fn validate_profile_local(dataset: &CimDataset, profile: &str, cfg: &Config) -> Vec<Violation> {
    let mut violations: Vec<Violation> = Vec::new();

    match profile {
        "EQ" => {
            violations.extend(equipment::validate(dataset));
            if cfg.not_solved {
                violations.extend(equipment_not_solved_mas::validate(dataset));
            }
        }
        "SSH" => {
            violations.extend(ssh::validate(dataset));
            if cfg.not_solved {
                violations.extend(ssh_not_solved_mas::validate(dataset));
            }
        }
        "TP" => {
            if cfg.not_solved {
                violations.extend(topology_not_solved_mas::validate(dataset));
            }
        }
        "DY" => {
            violations.extend(dynamics::validate(dataset));
        }
        "SC" => {
            violations.extend(shortcircuit::validate(dataset));
            if cfg.not_solved {
                violations.extend(shortcircuit_not_solved_mas::validate(dataset));
            }
        }
        "SV" => {
            violations.extend(state_variables::validate(dataset));
            if cfg.solved {
                violations.extend(state_variables_solved_mas::validate(dataset));
            }
        }
        "DL" => {
            violations.extend(diagram_layout::validate(dataset));
        }
        "EQBD" => {
            violations.extend(equipment_boundary::validate(dataset));
            if let Some(ref eqbd_bv_ids) = cfg.eqbd_base_voltage_ids {
                violations.extend(quality::check_base_voltage_in_eqbd_impl(dataset, eqbd_bv_ids));
            }
        }
        "OP" => {
            violations.extend(operation::validate(dataset));
        }
        _ => {}
    }

    violations
}

/// Cross-profile SPARQL checks that require the fully merged dataset.
pub fn validate_crossprofile(dataset: &CimDataset, cfg: &Config) -> Vec<Violation> {
    let mut violations: Vec<Violation> = Vec::new();

    if cfg.common {
        violations.extend(common::validate(dataset));
        if cfg.solved {
            violations.extend(common_solved_mas::validate(dataset));
        }
    }

    if cfg.quality {
        violations.extend(quality::validate(dataset));
    }

    violations.extend(prof10::validate(dataset));

    violations
}
