use std::collections::HashMap;
use cimdecoder::CimDataset;
use crate::Violation;

pub fn validate(dataset: &CimDataset) -> Vec<Violation> {
    let mut v = Vec::new();
    v.extend(check_terminal_phases_consistency_topological_node(dataset));
    v.extend(check_switch_same_topological_node(dataset));
    v.extend(check_terminal_exch8_topological_node(dataset));
    v
}

fn check_terminal_phases_consistency_topological_node(dataset: &CimDataset) -> Vec<Violation> {
    const ABCN: &str = "PhaseCode.ABCN";
    const N:    &str = "PhaseCode.N";
    const ABC:  &str = "PhaseCode.ABC";

    // Group terminals by topological node
    let mut node_terminals: HashMap<String, Vec<(String, String)>> = HashMap::new();
    for mrid in dataset.by_type.get("Terminal").into_iter().flatten() {
        let entry = &dataset.entries[mrid];
        let term = match entry.element.as_any().downcast_ref::<cimstructs::Terminal>() {
            Some(t) => t, None => continue,
        };
        let tn_id = match term.topological_node.as_ref() {
            Some(r) => r.mrid.trim_start_matches('#').to_string(), None => continue,
        };
        let phase = term.phases.as_ref().map_or(String::new(), |p| p.uri.clone());
        node_terminals.entry(tn_id).or_default().push((mrid.clone(), phase));
    }

    let mut v = Vec::new();
    'outer: for (node_id, terms) in &node_terminals {
        if terms.len() < 2 { continue; }
        for i in 0..terms.len() {
            for j in (i+1)..terms.len() {
                let val1 = &terms[i].1;
                let val2 = &terms[j].1;

                let failed = if !val1.is_empty() && !val2.is_empty() {
                    if (val1 == ABCN || val1 == N) && val2 != ABCN && val2 != N { true }
                    else if val1 == ABC && val2 != ABC { true }
                    else { false }
                } else if !val1.is_empty() && val2.is_empty() {
                    val1 == ABCN || val1 == N
                } else {
                    false
                };

                if failed {
                    v.push(Violation {
                        object_id:   node_id.clone(),
                        rule_id:     "topcns.Terminal.phases-consistencyTopologicalNode".into(),
                        name:        "Terminal.phases-consistencyTopologicalNode".into(),
                        class:       "TopologicalNode".into(),
                        property:    "Terminal.phases".into(),
                        message:     format!("The phase codes for the connected terminals are not consistent. Terminal {} code: {}, Terminal {} code: {}.",
                            terms[i].0, val1, terms[j].0, val2),
                        severity:    "sh:Violation".into(),
                        description: String::new(),
                    });
                    continue 'outer;
                }
            }
        }
    }
    v
}

fn get_tn_for_terminal(term: &cimstructs::Terminal, dataset: &CimDataset) -> Option<String> {
    if let Some(tn_ref) = term.topological_node.as_ref() {
        return Some(tn_ref.mrid.trim_start_matches('#').to_string());
    }
    let cn_id = term.connectivity_node.as_ref()?.mrid.trim_start_matches('#').to_string();
    let cn = dataset.entries.get(&cn_id)?.element.as_any().downcast_ref::<cimstructs::ConnectivityNode>()?;
    Some(cn.topological_node.as_ref()?.mrid.trim_start_matches('#').to_string())
}

macro_rules! check_switch_retained {
    ($v:expr, $dataset:expr, $eq_terms:expr, $($T:ident, $ret_path:expr),+) => {$(
        for mrid in $dataset.by_type.get(stringify!($T)).into_iter().flatten() {
            let entry = &$dataset.entries[mrid];
            if let Some(obj) = entry.element.as_any().downcast_ref::<cimstructs::$T>() {
                if !$ret_path(obj).unwrap_or(false) { continue; }
                let terms = match $eq_terms.get(mrid) { Some(t) => t, None => continue };
                let mut t1_tn: Option<String> = None;
                let mut t2_tn: Option<String> = None;
                for t_mrid in terms {
                    if let Some(entry) = $dataset.entries.get(t_mrid) {
                        if let Some(term) = entry.element.as_any().downcast_ref::<cimstructs::Terminal>() {
                            match term.base.sequence_number {
                                Some(1) => t1_tn = get_tn_for_terminal(term, $dataset),
                                Some(2) => t2_tn = get_tn_for_terminal(term, $dataset),
                                _ => {}
                            }
                        }
                    }
                }
                if let (Some(tn1), Some(tn2)) = (t1_tn, t2_tn) {
                    if !tn1.is_empty() && tn1 == tn2 {
                        $v.push(Violation {
                            object_id:   mrid.clone(),
                            rule_id:     "topc456ns:Switch-sameTopologicalNode".into(),
                            name:        "Switch-sameTopologicalNode".into(),
                            class:       stringify!($T).to_string(),
                            property:    "retained".into(),
                            message:     "Terminals of retained Switch connect to the same TopologicalNode.".into(),
                            severity:    "sh:Violation".into(),
                            description: String::new(),
                        });
                    }
                }
            }
        }
    )+};
}

fn check_switch_same_topological_node(dataset: &CimDataset) -> Vec<Violation> {
    // Build index: equipment MRID → [terminal MRIDs]
    let mut eq_terminals: HashMap<String, Vec<String>> = HashMap::new();
    for mrid in dataset.by_type.get("Terminal").into_iter().flatten() {
        let entry = &dataset.entries[mrid];
        if let Some(term) = entry.element.as_any().downcast_ref::<cimstructs::Terminal>() {
            if let Some(ce_ref) = term.conducting_equipment.as_ref() {
                eq_terminals.entry(ce_ref.mrid.trim_start_matches('#').to_string())
                    .or_default().push(mrid.clone());
            }
        }
    }

    let mut v = Vec::new();
    check_switch_retained!(v, dataset, eq_terminals,
        Switch,                   |o: &cimstructs::Switch|                    o.retained,
        Disconnector,             |o: &cimstructs::Disconnector|              o.base.retained,
        Fuse,                     |o: &cimstructs::Fuse|                      o.base.retained,
        Jumper,                   |o: &cimstructs::Jumper|                    o.base.retained,
        Cut,                      |o: &cimstructs::Cut|                       o.base.retained,
        GroundDisconnector,       |o: &cimstructs::GroundDisconnector|        o.base.retained,
        LoadBreakSwitch,          |o: &cimstructs::LoadBreakSwitch|           o.base.base.retained,
        Breaker,                  |o: &cimstructs::Breaker|                   o.base.base.retained,
        DisconnectingCircuitBreaker, |o: &cimstructs::DisconnectingCircuitBreaker| o.base.base.base.retained
    );
    v
}

fn check_terminal_exch8_topological_node(dataset: &CimDataset) -> Vec<Violation> {
    // Collect all terminal MRIDs referenced by any RegulatingControl
    let mut rc_terminals: std::collections::HashSet<String> = std::collections::HashSet::new();
    for type_name in &["RegulatingControl", "TapChangerControl"] {
        for mrid in dataset.by_type.get(*type_name).into_iter().flatten() {
            let entry = &dataset.entries[mrid];
            let rc_term = if let Some(rc) = entry.element.as_any().downcast_ref::<cimstructs::RegulatingControl>() {
                rc.terminal.as_ref().map(|r| r.mrid.trim_start_matches('#').to_string())
            } else if let Some(tcc) = entry.element.as_any().downcast_ref::<cimstructs::TapChangerControl>() {
                tcc.base.terminal.as_ref().map(|r| r.mrid.trim_start_matches('#').to_string())
            } else {
                None
            };
            if let Some(t_id) = rc_term { rc_terminals.insert(t_id); }
        }
    }

    let mut v = Vec::new();
    for mrid in dataset.by_type.get("Terminal").into_iter().flatten() {
        if !rc_terminals.contains(mrid) { continue; }
        let entry = &dataset.entries[mrid];
        let term = match entry.element.as_any().downcast_ref::<cimstructs::Terminal>() {
            Some(t) => t, None => continue,
        };
        if term.topological_node.is_some() { continue; }
        // Check if connectivity node has a TN
        let has_tn = term.connectivity_node.as_ref().and_then(|cn_ref| {
            dataset.entries.get(cn_ref.mrid.trim_start_matches('#'))
                .and_then(|e| e.element.as_any().downcast_ref::<cimstructs::ConnectivityNode>())
                .and_then(|cn| cn.topological_node.as_ref())
        }).is_some();
        if !has_tn {
            v.push(Violation {
                object_id:   mrid.clone(),
                rule_id:     "topc600ns:Terminal-EXCH8TopologicalNode".into(),
                name:        "Terminal-EXCH8TopologicalNode".into(),
                class:       "Terminal".into(),
                property:    "TopologicalNode".into(),
                message:     "The Terminal is referenced by a RegulatingControl but is not associated with a TopologicalNode.".into(),
                severity:    "sh:Violation".into(),
                description: String::new(),
            });
        }
    }
    v
}
