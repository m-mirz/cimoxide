use std::collections::HashMap;
use cimdecoder::CimDataset;
use crate::Violation;

pub fn validate(dataset: &CimDataset) -> Vec<Violation> {
    let mut v = Vec::new();
    v.extend(check_ac_line_segment_base_voltage(dataset));
    v.extend(check_regulating_control_target_value_tap_changer(dataset));
    v.extend(check_ac_line_segment_base_voltage_diff(dataset));
    v.extend(check_boundary_point_bppl(dataset));
    v.extend(check_equivalent_injection_regulation_capability_not_hvdc(dataset));
    v
}

fn terminal_nominal_voltage(dataset: &CimDataset, term: &cimstructs::Terminal) -> Option<f64> {
    let tn_id = term.topological_node.as_ref()?.mrid.trim_start_matches('#').to_string();
    let tn = dataset.entries.get(&tn_id)?.element.as_any().downcast_ref::<cimstructs::TopologicalNode>()?;
    let bv_id = tn.base_voltage.as_ref()?.mrid.trim_start_matches('#').to_string();
    let bv = dataset.entries.get(&bv_id)?.element.as_any().downcast_ref::<cimstructs::BaseVoltage>()?;
    bv.nominal_voltage
}

fn build_terminals_by_equipment_seq(dataset: &CimDataset) -> HashMap<String, HashMap<i64, String>> {
    let mut map: HashMap<String, HashMap<i64, String>> = HashMap::new();
    for mrid in dataset.by_type.get("Terminal").into_iter().flatten() {
        let entry = &dataset.entries[mrid];
        if let Some(term) = entry.element.as_any().downcast_ref::<cimstructs::Terminal>() {
            if let Some(ce) = &term.conducting_equipment {
                let eq_id = ce.mrid.trim_start_matches('#').to_string();
                let seq = term.base.sequence_number.unwrap_or(0);
                map.entry(eq_id).or_default().insert(seq, mrid.clone());
            }
        }
    }
    map
}

fn check_ac_line_segment_base_voltage(dataset: &CimDataset) -> Vec<Violation> {
    let by_eq = build_terminals_by_equipment_seq(dataset);
    let mut v = Vec::new();
    for mrid in dataset.by_type.get("ACLineSegment").into_iter().flatten() {
        let terms = match by_eq.get(mrid) { Some(t) => t, None => continue };
        let t1_id = match terms.get(&1) { Some(id) => id, None => continue };
        let t2_id = match terms.get(&2) { Some(id) => id, None => continue };
        let t1 = match dataset.entries.get(t1_id).and_then(|e| e.element.as_any().downcast_ref::<cimstructs::Terminal>()) { Some(t) => t, None => continue };
        let t2 = match dataset.entries.get(t2_id).and_then(|e| e.element.as_any().downcast_ref::<cimstructs::Terminal>()) { Some(t) => t, None => continue };
        let v1 = match terminal_nominal_voltage(dataset, t1) { Some(v) => v, None => continue };
        let v2 = match terminal_nominal_voltage(dataset, t2) { Some(v) => v, None => continue };
        // Skip if same TN
        if t1.topological_node.as_ref().map(|r| r.mrid.as_str()) == t2.topological_node.as_ref().map(|r| r.mrid.as_str()) { continue; }
        if v1 != v2 {
            v.push(Violation {
                object_id: mrid.clone(), rule_id: "eqn301:ACLineSegment-baseVoltage".into(),
                name: "C:301:EQ:ACLineSegment:baseVoltage".into(), class: "ACLineSegment".into(),
                property: "ACLineSegment.BaseVoltage".into(),
                message: format!("The ACLineSegment has different BaseVoltage.nominalVoltage at the two ends. Voltage at end 1 is: {}. Voltage at end 2 is: {}.", v1, v2),
                severity: "sh:Warning".into(), description: String::new(),
            });
        }
    }
    v
}

fn check_regulating_control_target_value_tap_changer(dataset: &CimDataset) -> Vec<Violation> {
    let mut v = Vec::new();
    let voltage_suffix = "voltage";

    // TapChangerControl MRID → RatioTapChangers referencing it. Built once instead of
    // rescanning all RatioTapChanger per RegulatingControl/TapChangerControl below.
    let mut rtc_by_tcc: HashMap<String, Vec<String>> = HashMap::new();
    for rtc_mrid in dataset.by_type.get("RatioTapChanger").into_iter().flatten() {
        let rtc_entry = &dataset.entries[rtc_mrid];
        if let Some(rtc) = rtc_entry.element.as_any().downcast_ref::<cimstructs::RatioTapChanger>() {
            if let Some(r) = &rtc.base.tap_changer_control {
                let tcc_id = r.mrid.trim_start_matches('#').to_string();
                rtc_by_tcc.entry(tcc_id).or_default().push(rtc_mrid.clone());
            }
        }
    }

    // Collect all RCs with voltage mode enabled
    for rc_mrid in dataset.by_type.get("RegulatingControl").into_iter().chain(dataset.by_type.get("TapChangerControl").into_iter()).flatten() {
        let rc_entry = &dataset.entries[rc_mrid];
        let (mode_ok, enabled, target_value, terminal_ref) =
            if let Some(rc) = rc_entry.element.as_any().downcast_ref::<cimstructs::RegulatingControl>() {
                (rc.mode.as_ref().map_or(false, |r| r.uri.ends_with(voltage_suffix)), rc.enabled.unwrap_or(false), rc.target_value.unwrap_or(0.0), rc.terminal.as_ref())
            } else if let Some(tcc) = rc_entry.element.as_any().downcast_ref::<cimstructs::TapChangerControl>() {
                (tcc.base.mode.as_ref().map_or(false, |r| r.uri.ends_with(voltage_suffix)), tcc.base.enabled.unwrap_or(false), tcc.base.target_value.unwrap_or(0.0), tcc.base.terminal.as_ref())
            } else {
                continue
            };
        if !mode_ok || !enabled { continue; }
        let term_id = match terminal_ref { Some(r) => r.mrid.trim_start_matches('#'), None => continue };

        // Find associated RatioTapChanger referencing this RC
        for rtc_mrid in rtc_by_tcc.get(rc_mrid).into_iter().flatten() {
            let rtc_entry = &dataset.entries[rtc_mrid];
            let rtc = match rtc_entry.element.as_any().downcast_ref::<cimstructs::RatioTapChanger>() { Some(r) => r, None => continue };
            if !rtc.base.control_enabled.unwrap_or(false) { continue; }

            // Get nominal voltage via RC terminal → CN → VoltageLevel → BaseVoltage
            let nominal_u = (|| -> Option<f64> {
                let term = dataset.entries.get(term_id)?.element.as_any().downcast_ref::<cimstructs::Terminal>()?;
                let cn_id = term.connectivity_node.as_ref()?.mrid.trim_start_matches('#');
                let cn = dataset.entries.get(cn_id)?.element.as_any().downcast_ref::<cimstructs::ConnectivityNode>()?;
                let cnc_id = cn.connectivity_node_container.as_ref()?.mrid.trim_start_matches('#');
                let vl = dataset.entries.get(cnc_id)?.element.as_any().downcast_ref::<cimstructs::VoltageLevel>()?;
                let bv_id = vl.base_voltage.as_ref()?.mrid.trim_start_matches('#');
                dataset.entries.get(bv_id)?.element.as_any().downcast_ref::<cimstructs::BaseVoltage>()?.nominal_voltage
            })();
            let nominal_u = match nominal_u { Some(u) if u != 0.0 => u, _ => continue };

            let target_pu = target_value / nominal_u;
            let step_pct = rtc.step_voltage_increment.unwrap_or(0.0) / 100.0;
            let high = rtc.base.high_step.unwrap_or(0);
            let neutral = rtc.base.neutral_step.unwrap_or(0);
            let low = rtc.base.low_step.unwrap_or(0);
            let upper_limit = 1.0 + step_pct * (high - neutral) as f64;
            let lower_limit = 1.0 - step_pct * (neutral - low) as f64;

            if target_pu < lower_limit || target_pu > upper_limit {
                v.push(Violation {
                    object_id: rc_mrid.clone(), rule_id: "eqn452:RegulatingControl.targetValue-tapChanger".into(),
                    name: "C:452:EQ:RegulatingControl.targetValue:tapChanger".into(), class: "RegulatingControl".into(),
                    property: "RegulatingControl.targetValue".into(),
                    message: format!("Target value PU ({}) is outside TapChanger capability limits [{}, {}].", target_pu, lower_limit, upper_limit),
                    severity: "sh:Violation".into(), description: String::new(),
                });
            }
        }
    }
    v
}

fn check_ac_line_segment_base_voltage_diff(dataset: &CimDataset) -> Vec<Violation> {
    let by_eq = build_terminals_by_equipment_seq(dataset);
    let mut v = Vec::new();
    for mrid in dataset.by_type.get("ACLineSegment").into_iter().flatten() {
        let terms = match by_eq.get(mrid) { Some(t) => t, None => continue };
        let t1_id = match terms.get(&1) { Some(id) => id, None => continue };
        let t2_id = match terms.get(&2) { Some(id) => id, None => continue };
        let t1 = match dataset.entries.get(t1_id).and_then(|e| e.element.as_any().downcast_ref::<cimstructs::Terminal>()) { Some(t) => t, None => continue };
        let t2 = match dataset.entries.get(t2_id).and_then(|e| e.element.as_any().downcast_ref::<cimstructs::Terminal>()) { Some(t) => t, None => continue };
        let v1 = match terminal_nominal_voltage(dataset, t1) { Some(v) => v, None => continue };
        let v2 = match terminal_nominal_voltage(dataset, t2) { Some(v) => v, None => continue };
        let diff = if v1 < v2 { (v2 - v1) / v1 } else { (v1 - v2) / v2 };
        if diff > 0.1 {
            v.push(Violation {
                object_id: mrid.clone(), rule_id: "eqn600:ACLineSegment-BaseVoltageDiff".into(),
                name: "C:600:EQ:ACLineSegment:BaseVoltageDiff".into(), class: "ACLineSegment".into(),
                property: "rdf:type".into(),
                message: format!("More than 10% difference of BaseVoltage.nominalVoltage at the two ends (V1: {}, V2: {}).", v1, v2),
                severity: "sh:Violation".into(), description: String::new(),
            });
        }
    }
    v
}

fn check_boundary_point_bppl(dataset: &CimDataset) -> Vec<Violation> {
    // BoundaryPoint → ConnectivityNode
    let mut bp_to_cn: HashMap<String, String> = HashMap::new();
    for mrid in dataset.by_type.get("BoundaryPoint").into_iter().flatten() {
        let entry = &dataset.entries[mrid];
        if let Some(bp) = entry.element.as_any().downcast_ref::<cimstructs::BoundaryPoint>() {
            if let Some(r) = &bp.connectivity_node {
                bp_to_cn.insert(mrid.clone(), r.mrid.trim_start_matches('#').to_string());
            }
        }
    }
    // CN → set of terminal MRIDs
    let mut cn_terminals: HashMap<String, Vec<String>> = HashMap::new();
    for mrid in dataset.by_type.get("Terminal").into_iter().flatten() {
        let entry = &dataset.entries[mrid];
        if let Some(term) = entry.element.as_any().downcast_ref::<cimstructs::Terminal>() {
            if let Some(r) = &term.connectivity_node {
                cn_terminals.entry(r.mrid.trim_start_matches('#').to_string()).or_default().push(mrid.clone());
            }
        }
    }
    let mut v = Vec::new();
    for (_, cn_id) in &bp_to_cn {
        let mut has_eq_injection = false;
        let mut has_two_terminal = false;
        for term_mrid in cn_terminals.get(cn_id).into_iter().flatten() {
            let term = match dataset.entries.get(term_mrid).and_then(|e| e.element.as_any().downcast_ref::<cimstructs::Terminal>()) { Some(t) => t, None => continue };
            let eq_id = match &term.conducting_equipment { Some(r) => r.mrid.trim_start_matches('#').to_string(), None => continue };
            let eq_entry = match dataset.entries.get(&eq_id) { Some(e) => e, None => continue };
            if eq_entry.element.as_any().downcast_ref::<cimstructs::EquivalentInjection>().is_some() { has_eq_injection = true; }
            if eq_entry.element.as_any().downcast_ref::<cimstructs::ACLineSegment>().is_some()
            || eq_entry.element.as_any().downcast_ref::<cimstructs::PowerTransformer>().is_some()
            || eq_entry.element.as_any().downcast_ref::<cimstructs::Breaker>().is_some()
            || eq_entry.element.as_any().downcast_ref::<cimstructs::Disconnector>().is_some() { has_two_terminal = true; }
        }
        if !has_eq_injection {
            v.push(Violation {
                object_id: cn_id.clone(), rule_id: "eqn600:BoundaryPoint-bppl1Bppl2".into(),
                name: "C:600:EQ:BoundaryPoint:bppl1Bppl2".into(), class: "ConnectivityNode".into(),
                property: "rdf:type".into(),
                message: "Boundary Point ConnectivityNode does not have an EquivalentInjection connected.".into(),
                severity: "sh:Violation".into(), description: String::new(),
            });
        }
        if !has_two_terminal {
            v.push(Violation {
                object_id: cn_id.clone(), rule_id: "eqn600:BoundaryPoint-bppl3".into(),
                name: "C:600:EQ:BoundaryPoint:bppl3".into(), class: "ConnectivityNode".into(),
                property: "rdf:type".into(),
                message: "Boundary Point ConnectivityNode does not have a two-terminal ConductingEquipment connected.".into(),
                severity: "sh:Info".into(), description: String::new(),
            });
        }
    }
    v
}

fn check_equivalent_injection_regulation_capability_not_hvdc(dataset: &CimDataset) -> Vec<Violation> {
    // CN → BoundaryPoint DC flag
    let mut cn_is_dc: HashMap<String, bool> = HashMap::new();
    for mrid in dataset.by_type.get("BoundaryPoint").into_iter().flatten() {
        let entry = &dataset.entries[mrid];
        if let Some(bp) = entry.element.as_any().downcast_ref::<cimstructs::BoundaryPoint>() {
            if let Some(r) = &bp.connectivity_node {
                cn_is_dc.insert(r.mrid.trim_start_matches('#').to_string(), bp.is_direct_current.unwrap_or(false));
            }
        }
    }
    // Equipment MRID → true if it has at least one terminal connected to a non-HVDC
    // BoundaryPoint CN. Built once over all Terminals instead of rescanning them per
    // EquivalentInjection below.
    let mut equip_non_hvdc_bp: HashMap<String, bool> = HashMap::new();
    for term_mrid in dataset.by_type.get("Terminal").into_iter().flatten() {
        let term = match dataset.entries.get(term_mrid).and_then(|e| e.element.as_any().downcast_ref::<cimstructs::Terminal>()) { Some(t) => t, None => continue };
        let eq_id = match &term.conducting_equipment { Some(r) => r.mrid.trim_start_matches('#').to_string(), None => continue };
        if let Some(cn) = &term.connectivity_node {
            let cn_id = cn.mrid.trim_start_matches('#');
            if let Some(&is_dc) = cn_is_dc.get(cn_id) {
                if !is_dc {
                    equip_non_hvdc_bp.insert(eq_id, true);
                }
            }
        }
    }

    let mut v = Vec::new();
    for ei_mrid in dataset.by_type.get("EquivalentInjection").into_iter().flatten() {
        let ei_entry = &dataset.entries[ei_mrid];
        let ei = match ei_entry.element.as_any().downcast_ref::<cimstructs::EquivalentInjection>() { Some(e) => e, None => continue };
        let is_non_hvdc_bp = equip_non_hvdc_bp.contains_key(ei_mrid.as_str());
        if is_non_hvdc_bp {
            if ei.regulation_capability.unwrap_or(false) || ei.reactive_capability_curve.is_some() {
                v.push(Violation {
                    object_id: ei_mrid.clone(), rule_id: "eqn600:EquivalentInjection.regulationCapability-notHVDC".into(),
                    name: "C:600:EQ:EquivalentInjection.regulationCapability:notHvdc".into(), class: "EquivalentInjection".into(),
                    property: "EquivalentInjection.regulationCapability".into(),
                    message: "EquivalentInjection at non-HVDC BoundaryPoint has regulationCapability=true or a ReactiveCapabilityCurve.".into(),
                    severity: "sh:Violation".into(), description: String::new(),
                });
            }
        }
    }
    v
}
