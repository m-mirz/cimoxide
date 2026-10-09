use cimmodel::base::{FastMap, FastSet};
use cimmodel::CimDataset;
use super::Fields;
use crate::Violation;

pub fn validate(dataset: &CimDataset) -> Vec<Violation> {
    check_boundary_point_tie_flow(dataset)
}

/// A BoundaryPoint on terminal 1 or 2 of an element whose terminals 1 and 2
/// are on different nodes: unless excluded from area interchange (absent is
/// not excluded), that terminal needs a TieFlow; if excluded, it must have
/// none. Without such an element there is nothing to check, so the boundary
/// file alone, which holds no terminals, reports nothing.
fn check_boundary_point_tie_flow(dataset: &CimDataset) -> Vec<Violation> {
    let tie_flow_terminals: FastSet<&str> = dataset.by_type.get("TieFlow").into_iter().flatten()
        .filter_map(|m| Fields::of(&dataset.entries[m]).reference("TieFlow.Terminal"))
        .map(|r| r.trim_start_matches('#'))
        .collect();
    // Equipment → (sequence number, terminal, node) of its terminals 1 and 2.
    let mut by_eq: FastMap<&str, Vec<(i64, &str, &str)>> = FastMap::default();
    for mrid in dataset.by_type.get("Terminal").into_iter().flatten() {
        let t = Fields::of(&dataset.entries[mrid]);
        if let (Some(eq), Some(n @ 1..=2), Some(cn)) = (
            t.reference("Terminal.ConductingEquipment"), t.i64("ACDCTerminal.sequenceNumber"), t.reference("Terminal.ConnectivityNode")) {
            by_eq.entry(eq.trim_start_matches('#')).or_default().push((n, mrid, cn.trim_start_matches('#')));
        }
    }
    // Node → whether a terminal of a spanning element there has a TieFlow.
    let mut at_node: FastMap<&str, bool> = FastMap::default();
    for ts in by_eq.values() {
        let (Some(t1), Some(t2)) = (ts.iter().find(|t| t.0 == 1), ts.iter().find(|t| t.0 == 2)) else { continue };
        if t1.2 == t2.2 { continue; }
        for (_, term, cn) in [t1, t2] {
            *at_node.entry(cn).or_default() |= tie_flow_terminals.contains(term);
        }
    }

    let mut v = Vec::new();
    for mrid in dataset.by_type.get("BoundaryPoint").into_iter().flatten() {
        let bp = Fields::of(&dataset.entries[mrid]);
        let Some(cn_id) = bp.reference("BoundaryPoint.ConnectivityNode") else { continue };
        let Some(&has_tie_flow) = at_node.get(cn_id.trim_start_matches('#')) else { continue };
        let excluded = bp.bool("BoundaryPoint.isExcludedFromAreaInterchange") == Some(true);
        let message = match (excluded, has_tie_flow) {
            (true, true) => "TieFlow is modelled but isExcludedFromAreaInterchange is true.",
            (false, false) => "TieFlow is required but not modelled for this BoundaryPoint.",
            _ => continue,
        };
        v.push(Violation {
            object_id:   mrid.clone(),
            rule_id:     "eqbdn301:BoundaryPoint.isExcludedFromAreaInterchange-requiredTieFlow".into(),
            name:        "C:301:EQBD:BoundaryPoint.isExcludedFromAreaInterchange:requiredTieFlow".into(),
            class:       "BoundaryPoint".into(),
            property:    "isExcludedFromAreaInterchange".into(),
            message:     message.into(),
            severity:    "sh:Violation".into(),
            description: String::new(),
        });
    }
    v
}
