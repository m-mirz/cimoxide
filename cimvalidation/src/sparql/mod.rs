pub mod common;
mod fields;
pub(crate) use fields::Fields;
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
pub mod nc;

use cimmodel::CimDataset;
use crate::{Config, Violation};


/// Per-profile SPARQL checks that only need data from a single profile's file.
pub fn validate_profile_local(dataset: &CimDataset, profile: &str, cfg: &Config) -> Vec<Violation> {
    let mut violations: Vec<Violation> = Vec::new();

    match profile {
        "EQ" => {
            violations.extend(equipment::validate(dataset));
        }
        "SSH" => {
            violations.extend(ssh::validate(dataset));
        }
        "DY" => {
            violations.extend(dynamics::validate(dataset));
        }
        "SC" => {
            violations.extend(shortcircuit::validate(dataset));
        }
        "SV" => {
            violations.extend(state_variables::validate(dataset));
        }
        "DL" => {
            violations.extend(diagram_layout::validate(dataset));
        }
        "EQBD" => {
            if let Some(ref eqbd_bv_ids) = cfg.eqbd_base_voltage_ids {
                violations.extend(quality::check_base_voltage_in_eqbd_impl(dataset, eqbd_bv_ids));
            }
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
    groups.extend(across_groups(cfg));
    groups.extend(mas_groups(cfg));
    crate::par::par_groups(dataset, &groups)
}

/// The rules of a profile's own constraint files that read another profile's
/// data, for each profile in play.
///
/// The APL validates a model as a whole, and these rules assume it: the SSH
/// CsConverter ranges compare SSH's operating mode with EQ's angle limits,
/// the SV ones read SSH's mode, an EQ rule asks for SC's zero-sequence
/// values, a DY rule for the machine's EQ ratings, an OP rule for the EQ
/// terminal's equipment. On their own file they could never fire (or, for
/// the OP rule, fired on every measurement), so they run on the merged
/// dataset, in either solved state.
pub fn across_groups(cfg: &Config) -> Vec<fn(&CimDataset) -> Vec<Violation>> {
    type Group = fn(&CimDataset) -> Vec<Violation>;
    const ACROSS: &[(&str, Group)] = &[
        ("EQ", equipment::validate_across),
        ("SSH", ssh::validate_across),
        ("SV", state_variables::validate_across),
        ("DY", dynamics::validate_across),
        ("OP", operation::validate_across),
    ];
    let has = |p: &str| cfg.profiles.is_empty() || cfg.profiles.iter().any(|x| x == p);
    ACROSS.iter().filter(|(profile, _)| has(profile)).map(|(_, group)| *group).collect()
}

/// The rules of the `*SolvedMAS` and `*NotSolvedMAS` files whose profile is in
/// play, each with the state it applies to.
///
/// Those files are written for a model authority set — EQ, SSH, TP and SV
/// together — and their rules read across profiles: an SSH rule compares a
/// machine's p with its unit's EQ operating limits, an SV rule a flow with
/// them. On one file there is nothing to compare, so they run on the merged
/// dataset, the union relicapgrid validates CGMES's Complex shapes on.
pub fn mas_groups(cfg: &Config) -> Vec<fn(&CimDataset) -> Vec<Violation>> {
    type Group = fn(&CimDataset) -> Vec<Violation>;
    // (profile, runs when solved, group)
    const MAS: &[(&str, bool, Group)] = &[
        ("EQ", false, equipment_not_solved_mas::validate),
        ("EQBD", false, equipment_boundary::validate),
        ("SSH", false, ssh_not_solved_mas::validate),
        ("TP", false, topology_not_solved_mas::validate),
        ("SC", false, shortcircuit_not_solved_mas::validate),
        ("SV", true, state_variables_solved_mas::validate),
    ];
    let has = |p: &str| cfg.profiles.is_empty() || cfg.profiles.iter().any(|x| x == p);
    MAS.iter()
        .filter(|(profile, solved, _)| has(profile) && if *solved { cfg.solved } else { cfg.not_solved })
        .map(|(_, _, group)| *group)
        .collect()
}
