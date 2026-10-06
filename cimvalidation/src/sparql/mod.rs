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

use cimdecoder::{CimDataset, CimEntry};
use cimstructs::base::RdfBlock;
use crate::{Config, Violation};

/// An element's fields, for reading one named attribute.
///
/// The decoder's block while it is still held, and only otherwise rebuilt from
/// the struct: `to_block` allocates a fresh map per element, and calling it on
/// every element was half of all SPARQL-rule time on RealGrid. For an
/// attribute the struct declares the two hold the same reference.
///
/// The block also holds attributes the struct does not declare, so this is for
/// reading named attributes, not for iterating every field — a rule that does
/// the latter keeps `to_block` and the struct's view of the element.
pub(crate) fn block_of(entry: &CimEntry) -> std::borrow::Cow<'_, RdfBlock> {
    if entry.block.type_name.is_empty() {
        std::borrow::Cow::Owned(entry.element.to_block())
    } else {
        std::borrow::Cow::Borrowed(&entry.block)
    }
}

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
        "TP"
            if cfg.not_solved => {
                violations.extend(topology_not_solved_mas::validate(dataset));
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
///
/// The rule groups are independent reads of the same dataset, each a walk
/// over much of it, so they run on their own threads. Results are joined in a
/// fixed order, so the output does not depend on scheduling.
pub fn validate_crossprofile(dataset: &CimDataset, cfg: &Config) -> Vec<Violation> {
    type Group = fn(&CimDataset) -> Vec<Violation>;
    let mut groups: Vec<Group> = Vec::new();
    if cfg.common {
        groups.push(common::validate);
        if cfg.solved {
            groups.push(common_solved_mas::validate);
        }
    }
    if cfg.quality {
        groups.push(quality::validate);
    }
    groups.push(prof10::validate);
    crate::par::par_groups(dataset, &groups)
}
