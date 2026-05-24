use std::collections::HashMap;
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

    let mut v = Vec::new();
    for mrid in dataset.by_type.get("BoundaryPoint").into_iter().flatten() {
        let entry = &dataset.entries[mrid];
        let bp = match entry.element.as_any().downcast_ref::<cimstructs::BoundaryPoint>() {
            Some(o) => o, None => continue,
        };
        let cn_id = match bp.connectivity_node.as_ref() {
            Some(r) => r.mrid.trim_start_matches('#').to_string(), None => continue,
        };

        // Find if any terminal at this CN has a TieFlow
        let has_tie_flow = dataset.by_type.get("Terminal").into_iter().flatten().any(|t_mrid| {
            dataset.entries.get(t_mrid)
                .and_then(|e| e.element.as_any().downcast_ref::<cimstructs::Terminal>())
                .and_then(|t| t.connectivity_node.as_ref())
                .map_or(false, |cn_ref| cn_ref.mrid.trim_start_matches('#') == cn_id)
                && terminal_has_tf.contains(t_mrid)
        });

        let excluded = bp.is_excluded_from_area_interchange.unwrap_or(false);
        if excluded && has_tie_flow {
            v.push(Violation {
                object_id:   mrid.clone(),
                rule_id:     "eqbdn301:BoundaryPoint.isExcludedFromAreaInterchange-requiredTieFlow".into(),
                name:        "BoundaryPoint.isExcludedFromAreaInterchange-requiredTieFlow".into(),
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
                name:        "BoundaryPoint.isExcludedFromAreaInterchange-requiredTieFlow".into(),
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
