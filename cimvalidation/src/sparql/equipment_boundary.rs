use cimdecoder::CimDataset;
use crate::Violation;

pub fn validate(dataset: &CimDataset) -> Vec<Violation> {
    check_boundary_point_tie_flow(dataset)
}

fn check_boundary_point_tie_flow(dataset: &CimDataset) -> Vec<Violation> {
    // Build index: terminal MRID → has tie flow
    let mut terminal_has_tf: std::collections::HashSet<String> = std::collections::HashSet::new();
    for mrid in dataset.by_type.get("TieFlow").into_iter().flatten() {
        let entry = &dataset.entries[mrid];
        if let Some(tf) = entry.element.as_any().downcast_ref::<cimstructs::TieFlow>() {
            if let Some(term_ref) = tf.terminal.as_ref() {
                terminal_has_tf.insert(term_ref.mrid.trim_start_matches('#').to_string());
            }
        }
    }

    // Build index: connectivity node MRID → true if any terminal at that CN has a TieFlow.
    // Built once over all Terminals instead of rescanning them per BoundaryPoint below.
    let mut cn_has_tie_flow: std::collections::HashSet<String> = std::collections::HashSet::new();
    for t_mrid in dataset.by_type.get("Terminal").into_iter().flatten() {
        if !terminal_has_tf.contains(t_mrid) {
            continue;
        }
        if let Some(term) = dataset.entries.get(t_mrid).and_then(|e| e.element.as_any().downcast_ref::<cimstructs::Terminal>()) {
            if let Some(cn_ref) = term.connectivity_node.as_ref() {
                cn_has_tie_flow.insert(cn_ref.mrid.trim_start_matches('#').to_string());
            }
        }
    }

    let mut v = Vec::new();
    for mrid in dataset.by_type.get("BoundaryPoint").into_iter().flatten() {
        let entry = &dataset.entries[mrid];
        let bp = match entry.element.as_any().downcast_ref::<cimstructs::BoundaryPoint>() {
            Some(o) => o, None => continue,
        };
        let cn_id = match bp.connectivity_node.as_ref() {
            Some(r) => r.mrid.trim_start_matches('#').to_string(), None => continue,
        };

        let has_tie_flow = cn_has_tie_flow.contains(&cn_id);

        let excluded = bp.is_excluded_from_area_interchange.unwrap_or(false);
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
