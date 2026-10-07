use cimmodel::CimDataset;
use super::Fields;
use crate::Violation;

pub fn validate(dataset: &CimDataset) -> Vec<Violation> {
    check_boundary_point_tie_flow(dataset)
}

fn check_boundary_point_tie_flow(dataset: &CimDataset) -> Vec<Violation> {
    // Build index: terminal MRID → has tie flow
    let mut terminal_has_tf: cimmodel::base::FastSet<String> = cimmodel::base::FastSet::default();
    for mrid in dataset.by_type.get("TieFlow").into_iter().flatten() {
        let entry = &dataset.entries[mrid];
        if let Some(tf) = Fields::of_class(entry, "TieFlow")
            && let Some(term_ref) = tf.reference("TieFlow.Terminal") {
                terminal_has_tf.insert(term_ref.trim_start_matches('#').to_string());
            }
    }

    // Build index: connectivity node MRID → true if any terminal at that CN has a TieFlow.
    // Built once over all Terminals instead of rescanning them per BoundaryPoint below.
    let mut cn_has_tie_flow: cimmodel::base::FastSet<String> = cimmodel::base::FastSet::default();
    for t_mrid in dataset.by_type.get("Terminal").into_iter().flatten() {
        if !terminal_has_tf.contains(t_mrid) {
            continue;
        }
        if let Some(term) = Fields::get(dataset, t_mrid, "Terminal")
            && let Some(cn_ref) = term.reference("Terminal.ConnectivityNode") {
                cn_has_tie_flow.insert(cn_ref.trim_start_matches('#').to_string());
            }
    }

    let mut v = Vec::new();
    for mrid in dataset.by_type.get("BoundaryPoint").into_iter().flatten() {
        let entry = &dataset.entries[mrid];
        let bp = match Fields::of_class(entry, "BoundaryPoint") {
            Some(o) => o, None => continue,
        };
        let cn_id = match bp.reference("BoundaryPoint.ConnectivityNode") {
            Some(r) => r.trim_start_matches('#').to_string(), None => continue,
        };

        let has_tie_flow = cn_has_tie_flow.contains(&cn_id);

        let excluded = bp.bool("BoundaryPoint.isExcludedFromAreaInterchange").unwrap_or(false);
        if excluded && has_tie_flow {
            v.push(Violation {
                object_id:   mrid.clone(),
                rule_id:     "eqbdn301:BoundaryPoint.isExcludedFromAreaInterchange-requiredTieFlow".into(),
                name:        "C:301:EQBD:BoundaryPoint.isExcludedFromAreaInterchange:requiredTieFlow".into(),
                class:       "BoundaryPoint".into(),
                property:    "isExcludedFromAreaInterchange".into(),
                message:     "TieFlow is modelled but isExcludedFromAreaInterchange is true.".into(),
                severity:    "sh:Violation".into(),
                description: String::new(),
            });
        } else if !excluded && !has_tie_flow {
            v.push(Violation {
                object_id:   mrid.clone(),
                rule_id:     "eqbdn301:BoundaryPoint.isExcludedFromAreaInterchange-requiredTieFlow".into(),
                name:        "C:301:EQBD:BoundaryPoint.isExcludedFromAreaInterchange:requiredTieFlow".into(),
                class:       "BoundaryPoint".into(),
                property:    "isExcludedFromAreaInterchange".into(),
                message:     "TieFlow is required but not modelled for this BoundaryPoint.".into(),
                severity:    "sh:Violation".into(),
                description: String::new(),
            });
        }
    }
    v
}
