use cimmodel::base::FastMap as HashMap;
use cimmodel::CimDataset;
use crate::Violation;
use super::Fields;

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
    let mut node_terminals: HashMap<String, Vec<(String, String)>> = HashMap::default();
    for mrid in dataset.by_type.get("Terminal").into_iter().flatten() {
        let entry = &dataset.entries[mrid];
        let term = match Fields::of_class(entry, "Terminal") {
            Some(t) => t, None => continue,
        };
        let tn_id = match term.reference("Terminal.TopologicalNode") {
            Some(r) => r.trim_start_matches('#').to_string(), None => continue,
        };
        let phase = term.enumeration("Terminal.phases").map_or(String::new(), str::to_string);
        node_terminals.entry(tn_id).or_default().push((mrid.clone(), phase));
    }

    let mut v = Vec::new();
    'outer: for (node_id, terms) in &node_terminals {
        if terms.len() < 2 { continue; }
        // Every ordered pair: ABC beside ABCN fails one way round only. (The
        // SPARQL's `HAVING (?terms>1)` counts the types of one terminal, so it
        // never reports; this is the rule as described.)
        for i in 0..terms.len() {
            for j in (0..terms.len()).filter(|&j| j != i) {
                let val1 = &terms[i].1;
                let val2 = &terms[j].1;

                let failed = if !val1.is_empty() && !val2.is_empty() {
                    if (val1 == ABCN || val1 == N) && val2 != ABCN && val2 != N { true }
                    else { val1 == ABC && val2 != ABC }
                } else if !val1.is_empty() && val2.is_empty() {
                    val1 == ABCN || val1 == N
                } else {
                    false
                };

                if failed {
                    v.push(Violation {
                        object_id:   node_id.clone(),
                        rule_id:     "tpn301:Terminal.phases-consistencyTopologicalNode".into(),
                        name:        "C:301:TP:Terminal.phases:consistencyTopologicalNode".into(),
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

fn get_tn_for_terminal(term: &Fields, dataset: &CimDataset) -> Option<String> {
    if let Some(tn_ref) = term.reference("Terminal.TopologicalNode") {
        return Some(tn_ref.trim_start_matches('#').to_string());
    }
    let cn_id = term.reference("Terminal.ConnectivityNode")?.trim_start_matches('#').to_string();
    let cn = Fields::get(dataset, &cn_id, "ConnectivityNode")?;
    Some(cn.reference("ConnectivityNode.TopologicalNode")?.trim_start_matches('#').to_string())
}

fn check_switch_same_topological_node(dataset: &CimDataset) -> Vec<Violation> {
    // Build index: equipment MRID → [terminal MRIDs]
    let mut eq_terminals: HashMap<String, Vec<String>> = HashMap::default();
    for mrid in dataset.by_type.get("Terminal").into_iter().flatten() {
        let entry = &dataset.entries[mrid];
        if let Some(term) = Fields::of_class(entry, "Terminal")
            && let Some(ce_ref) = term.reference("Terminal.ConductingEquipment") {
                eq_terminals.entry(ce_ref.trim_start_matches('#').to_string())
                    .or_default().push(mrid.clone());
            }
    }

    // Switch and its subclasses, each read through the inherited Switch.retained.
    const SWITCHES: &[&str] = &[
        "Switch", "Disconnector", "Fuse", "Jumper", "Cut", "GroundDisconnector",
        "LoadBreakSwitch", "Breaker", "DisconnectingCircuitBreaker",
    ];
    let mut v = Vec::new();
    for class in SWITCHES {
        for mrid in dataset.by_type.get(*class).into_iter().flatten() {
            let Some(obj) = Fields::of_class(&dataset.entries[mrid], class) else { continue };
            if obj.bool("Switch.retained") != Some(true) { continue; }
            let terms = match eq_terminals.get(mrid) { Some(t) => t, None => continue };
            let mut t1_tn: Option<String> = None;
            let mut t2_tn: Option<String> = None;
            for t_mrid in terms {
                if let Some(term) = Fields::get(dataset, t_mrid, "Terminal") {
                    match term.i64("ACDCTerminal.sequenceNumber") {
                        Some(1) => t1_tn = get_tn_for_terminal(&term, dataset),
                        Some(2) => t2_tn = get_tn_for_terminal(&term, dataset),
                        _ => {}
                    }
                }
            }
            if let (Some(tn1), Some(tn2)) = (t1_tn, t2_tn)
                && !tn1.is_empty() && tn1 == tn2 {
                    v.push(Violation {
                        object_id:   mrid.clone(),
                        rule_id:     "tpn456:Switch-sameTopologicalNode".into(),
                        name:        "C:456:TP:Terminal:switch".into(),
                        class:       class.to_string(),
                        property:    "retained".into(),
                        message:     "Terminals of retained Switch connect to the same TopologicalNode.".into(),
                        severity:    "sh:Violation".into(),
                        description: String::new(),
                    });
                }
        }
    }
    v
}

fn check_terminal_exch8_topological_node(dataset: &CimDataset) -> Vec<Violation> {
    // Collect all terminal MRIDs referenced by any RegulatingControl
    let mut rc_terminals: cimmodel::base::FastSet<String> = cimmodel::base::FastSet::default();
    for type_name in &["RegulatingControl", "TapChangerControl"] {
        for mrid in dataset.by_type.get(*type_name).into_iter().flatten() {
            let entry = &dataset.entries[mrid];
            let rc_term = Fields::of_class(entry, type_name)
                .and_then(|rc| rc.reference("RegulatingControl.Terminal").map(|r| r.trim_start_matches('#').to_string()));
            if let Some(t_id) = rc_term { rc_terminals.insert(t_id); }
        }
    }

    let mut v = Vec::new();
    for mrid in dataset.by_type.get("Terminal").into_iter().flatten() {
        if !rc_terminals.contains(mrid) { continue; }
        let entry = &dataset.entries[mrid];
        let term = match Fields::of_class(entry, "Terminal") {
            Some(t) => t, None => continue,
        };
        if term.reference("Terminal.TopologicalNode").is_some() { continue; }
        // Check if connectivity node has a TN
        let has_tn = term.reference("Terminal.ConnectivityNode").is_some_and(|cn_ref| {
            Fields::get(dataset, cn_ref.trim_start_matches('#'), "ConnectivityNode")
                .is_some_and(|cn| cn.reference("ConnectivityNode.TopologicalNode").is_some())
        });
        if !has_tn {
            v.push(Violation {
                object_id:   mrid.clone(),
                rule_id:     "tpn600:Terminal-EXCH8TopologicalNode".into(),
                name:        "C:600:EQ:Terminal:EXCH8TopologicalNode".into(),
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
