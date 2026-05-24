use cimdecoder::CimDataset;
use crate::Violation;

pub fn validate(dataset: &CimDataset) -> Vec<Violation> {
    check_mutual_coupling_terminals_assignment(dataset)
}

fn check_mutual_coupling_terminals_assignment(dataset: &CimDataset) -> Vec<Violation> {
    let mut v = Vec::new();

    let conducting_equipment_of = |term_ref: &cimstructs::base::MridRef| -> Option<(String, Option<String>)> {
        let term_id = term_ref.mrid.trim_start_matches('#');
        let term_entry = dataset.entries.get(term_id)?;
        let term = term_entry.element.as_any().downcast_ref::<cimstructs::Terminal>()?;
        let ce_ref = term.conducting_equipment.as_ref()?;
        let eq_id = ce_ref.mrid.trim_start_matches('#').to_string();
        let type_name = dataset.entries.get(&eq_id).map(|e| e.element.type_name().to_string());
        Some((eq_id, type_name))
    };

    for mrid in dataset.by_type.get("MutualCoupling").into_iter().flatten() {
        let entry = &dataset.entries[mrid];
        let mc = match entry.element.as_any().downcast_ref::<cimstructs::MutualCoupling>() {
            Some(o) => o, None => continue,
        };
        let (eq1_id, eq1_type) = match mc.first_terminal.as_ref().and_then(|r| conducting_equipment_of(r)) {
            Some(x) => x, None => continue,
        };
        let (eq2_id, eq2_type) = match mc.second_terminal.as_ref().and_then(|r| conducting_equipment_of(r)) {
            Some(x) => x, None => continue,
        };

        let is_line_like = |t: &Option<String>| -> bool {
            matches!(t.as_deref(), Some("ACLineSegment") | Some("Equipment"))
        };

        if !is_line_like(&eq1_type) || !is_line_like(&eq2_type) || eq1_id == eq2_id {
            let t1 = eq1_type.as_deref().unwrap_or("unknown");
            let t2 = eq2_type.as_deref().unwrap_or("unknown");
            v.push(Violation {
                object_id:   mrid.clone(),
                rule_id:     "sccns.MutualCoupling-terminalsAssignment".into(),
                name:        "MutualCoupling-terminalsAssignment".into(),
                class:       "MutualCoupling".into(),
                property:    "MutualCoupling.First_Terminal".into(),
                message:     format!("The terminals are either not related to ACLineSegment or the first and the second terminal associations are not pointing to different ACLineSegments. Type line 1: {t1}. Type line 2: {t2}."),
                severity:    "sh:Violation".into(),
                description: String::new(),
            });
        }
    }
    v
}
