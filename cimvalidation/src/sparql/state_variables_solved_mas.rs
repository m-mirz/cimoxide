use cimdecoder::{CimDataset, CimEntry};
use crate::Violation;
use cimstructs::base::CimElement;

pub fn validate(dataset: &CimDataset) -> Vec<Violation> {
    let mut v = Vec::new();
    v.extend(check_sv_tap_step_position_range(dataset));
    v.extend(check_sv_tap_step_position_integer(dataset));
    v.extend(check_sv_shunt_compensator_sections_integer(dataset));
    v.extend(check_sv_switch_instance(dataset));
    v.extend(check_sv_power_flow_instance(dataset));
    v.extend(check_sv_power_flow_p_limits(dataset));
    v.extend(check_sv_power_flow_q_limits(dataset));
    v.extend(check_sv_voltage_limits(dataset));
    v
}

fn tc_low_high(entry: &CimEntry) -> Option<(i64, i64)> {
    if let Some(o) = entry.element.as_any().downcast_ref::<cimstructs::RatioTapChanger>() {
        return Some((o.base.low_step?, o.base.high_step?));
    }
    if let Some(o) = entry.element.as_any().downcast_ref::<cimstructs::PhaseTapChangerLinear>() {
        return Some((o.base.base.low_step?, o.base.base.high_step?));
    }
    if let Some(o) = entry.element.as_any().downcast_ref::<cimstructs::PhaseTapChangerTabular>() {
        return Some((o.base.base.low_step?, o.base.base.high_step?));
    }
    if let Some(o) = entry.element.as_any().downcast_ref::<cimstructs::PhaseTapChangerNonLinear>() {
        return Some((o.base.base.low_step?, o.base.base.high_step?));
    }
    if let Some(o) = entry.element.as_any().downcast_ref::<cimstructs::PhaseTapChangerAsymmetrical>() {
        return Some((o.base.base.base.low_step?, o.base.base.base.high_step?));
    }
    if let Some(o) = entry.element.as_any().downcast_ref::<cimstructs::PhaseTapChangerSymmetrical>() {
        return Some((o.base.base.base.low_step?, o.base.base.base.high_step?));
    }
    None
}

fn rc_discrete_enabled(entry: &CimEntry) -> (bool, bool) {
    if let Some(o) = entry.element.as_any().downcast_ref::<cimstructs::RegulatingControl>() {
        return (o.discrete.unwrap_or(false), o.enabled.unwrap_or(false));
    }
    if let Some(o) = entry.element.as_any().downcast_ref::<cimstructs::TapChangerControl>() {
        return (o.base.discrete.unwrap_or(false), o.base.enabled.unwrap_or(false));
    }
    (false, false)
}

fn check_sv_tap_step_position_range(dataset: &CimDataset) -> Vec<Violation> {
    let mut v = Vec::new();
    for mrid in dataset.by_type.get("SvTapStep").into_iter().flatten() {
        let entry = &dataset.entries[mrid];
        let obj = match entry.element.as_any().downcast_ref::<cimstructs::SvTapStep>() {
            Some(o) => o, None => continue,
        };
        let pos = match obj.position { Some(p) => p, None => continue };
        let tc_ref = match obj.tap_changer.as_ref() { Some(r) => r, None => continue };
        let tc_id = tc_ref.mrid.trim_start_matches('#');
        let tc_entry = match dataset.entries.get(tc_id) { Some(e) => e, None => continue };
        let (low, high) = match tc_low_high(tc_entry) { Some(p) => p, None => continue };
        if pos < low as f64 || pos > high as f64 {
            v.push(Violation {
                object_id:   mrid.clone(),
                rule_id:     "svs301:SvTapStep.position-valueRange".into(),
                name:        "C:301:SV:SvTapStep.position:valueRange".into(),
                class:       "SvTapStep".into(),
                property:    "SvTapStep.position".into(),
                message:     format!("The value ({pos}) is out of range [{low},{high}]."),
                severity:    "sh:Violation".into(),
                description: String::new(),
            });
        }
    }
    v
}

fn check_sv_tap_step_position_integer(dataset: &CimDataset) -> Vec<Violation> {
    let mut v = Vec::new();
    for mrid in dataset.by_type.get("SvTapStep").into_iter().flatten() {
        let entry = &dataset.entries[mrid];
        let obj = match entry.element.as_any().downcast_ref::<cimstructs::SvTapStep>() {
            Some(o) => o, None => continue,
        };
        let pos = match obj.position { Some(p) => p, None => continue };
        let tc_ref = match obj.tap_changer.as_ref() { Some(r) => r, None => continue };
        let tc_id = tc_ref.mrid.trim_start_matches('#');
        let tc_entry = match dataset.entries.get(tc_id) { Some(e) => e, None => continue };

        let tcc_ref = tap_changer_control_ref(tc_entry);
        let tcc_id = match tcc_ref { Some(r) => r.mrid.trim_start_matches('#').to_string(), None => continue };
        let tcc_entry = match dataset.entries.get(&tcc_id) { Some(e) => e, None => continue };
        let (discrete, enabled) = rc_discrete_enabled(tcc_entry);
        if discrete && enabled && pos != pos.floor() {
            v.push(Violation {
                object_id:   mrid.clone(),
                rule_id:     "svs456:SvTapStep.position-value".into(),
                name:        "C:456:SV:SvTapStep.position:value".into(),
                class:       "SvTapStep".into(),
                property:    "SvTapStep.position".into(),
                message:     format!("The value ({pos}) is not integer for an active discrete regulating control."),
                severity:    "sh:Violation".into(),
                description: String::new(),
            });
        }
    }
    v
}

fn tap_changer_control_ref(entry: &CimEntry) -> Option<&cimstructs::base::MridRef> {
    if let Some(o) = entry.element.as_any().downcast_ref::<cimstructs::RatioTapChanger>() {
        return o.base.tap_changer_control.as_ref();
    }
    if let Some(o) = entry.element.as_any().downcast_ref::<cimstructs::PhaseTapChangerLinear>() {
        return o.base.base.tap_changer_control.as_ref();
    }
    if let Some(o) = entry.element.as_any().downcast_ref::<cimstructs::PhaseTapChangerTabular>() {
        return o.base.base.tap_changer_control.as_ref();
    }
    if let Some(o) = entry.element.as_any().downcast_ref::<cimstructs::PhaseTapChangerNonLinear>() {
        return o.base.base.tap_changer_control.as_ref();
    }
    if let Some(o) = entry.element.as_any().downcast_ref::<cimstructs::PhaseTapChangerAsymmetrical>() {
        return o.base.base.base.tap_changer_control.as_ref();
    }
    if let Some(o) = entry.element.as_any().downcast_ref::<cimstructs::PhaseTapChangerSymmetrical>() {
        return o.base.base.base.tap_changer_control.as_ref();
    }
    None
}

fn shunt_compensator_regulating_control_ref(entry: &CimEntry) -> Option<&cimstructs::base::MridRef> {
    if let Some(o) = entry.element.as_any().downcast_ref::<cimstructs::LinearShuntCompensator>() {
        return o.base.base.regulating_control.as_ref();
    }
    if let Some(o) = entry.element.as_any().downcast_ref::<cimstructs::NonlinearShuntCompensator>() {
        return o.base.base.regulating_control.as_ref();
    }
    None
}

fn check_sv_shunt_compensator_sections_integer(dataset: &CimDataset) -> Vec<Violation> {
    let mut v = Vec::new();
    for mrid in dataset.by_type.get("SvShuntCompensatorSections").into_iter().flatten() {
        let entry = &dataset.entries[mrid];
        let obj = match entry.element.as_any().downcast_ref::<cimstructs::SvShuntCompensatorSections>() {
            Some(o) => o, None => continue,
        };
        let sections = match obj.sections { Some(s) => s, None => continue };
        let sc_ref = match obj.shunt_compensator.as_ref() { Some(r) => r, None => continue };
        let sc_id = sc_ref.mrid.trim_start_matches('#');
        let sc_entry = match dataset.entries.get(sc_id) { Some(e) => e, None => continue };

        let rc_ref = match shunt_compensator_regulating_control_ref(sc_entry) { Some(r) => r, None => continue };
        let rc_id = rc_ref.mrid.trim_start_matches('#').to_string();
        let rc_entry = match dataset.entries.get(&rc_id) { Some(e) => e, None => continue };
        let (discrete, enabled) = rc_discrete_enabled(rc_entry);
        if discrete && enabled && sections != sections.floor() {
            v.push(Violation {
                object_id:   mrid.clone(),
                rule_id:     "svs456:SvShuntCompensatorSections.sections-value".into(),
                name:        "C:456:SV:SvShuntCompensatorSections.sections:value".into(),
                class:       "SvShuntCompensatorSections".into(),
                property:    "SvShuntCompensatorSections.sections".into(),
                message:     format!("The value ({sections}) is not integer for an active discrete regulating control."),
                severity:    "sh:Violation".into(),
                description: String::new(),
            });
        }
    }
    v
}

const SWITCH_TYPES: &[&str] = &[
    "Switch", "Breaker", "LoadBreakSwitch", "Disconnector", "Fuse", "Jumper",
    "GroundDisconnector", "DisconnectingCircuitBreaker", "Cut",
];

fn check_sv_switch_instance(dataset: &CimDataset) -> Vec<Violation> {
    // Build a set of switch MRIDs that have an SvSwitch
    let mut covered: std::collections::HashSet<String> = std::collections::HashSet::new();
    for mrid in dataset.by_type.get("SvSwitch").into_iter().flatten() {
        let entry = &dataset.entries[mrid];
        if let Some(obj) = entry.element.as_any().downcast_ref::<cimstructs::SvSwitch>() {
            if let Some(sw_ref) = obj.switch.as_ref() {
                covered.insert(sw_ref.mrid.trim_start_matches('#').to_string());
            }
        }
    }

    let mut v = Vec::new();
    for type_name in SWITCH_TYPES {
        for mrid in dataset.by_type.get(*type_name).into_iter().flatten() {
            if !covered.contains(mrid) {
                v.push(Violation {
                    object_id:   mrid.clone(),
                    rule_id:     "svs456:SvSwitch-instance".into(),
                    name:        "C:456:SV:SvSwitch:instance".into(),
                    class:       (*type_name).to_string(),
                    property:    "rdf:type".into(),
                    message:     "SvSwitch not instantiated.".into(),
                    severity:    "sh:Violation".into(),
                    description: String::new(),
                });
            }
        }
    }
    v
}

const INJECTION_TYPES: &[&str] = &[
    "NonConformLoad", "EquivalentInjection", "EnergySource", "ExternalNetworkInjection",
    "PowerElectronicsConnection", "AsynchronousMachine", "EnergyConsumer",
    "LinearShuntCompensator", "NonlinearShuntCompensator", "StaticVarCompensator",
    "SynchronousMachine", "StationSupply", "ConformLoad",
];

fn check_sv_power_flow_instance(dataset: &CimDataset) -> Vec<Violation> {
    // Build in-service equipment set from SvStatus
    let mut in_service: std::collections::HashSet<String> = std::collections::HashSet::new();
    for mrid in dataset.by_type.get("SvStatus").into_iter().flatten() {
        let entry = &dataset.entries[mrid];
        if let Some(obj) = entry.element.as_any().downcast_ref::<cimstructs::SvStatus>() {
            if obj.in_service.unwrap_or(false) {
                if let Some(ce_ref) = obj.conducting_equipment.as_ref() {
                    in_service.insert(ce_ref.mrid.trim_start_matches('#').to_string());
                }
            }
        }
    }

    // Build set of TN MRIDs that are in a topological island
    let mut tn_in_island: std::collections::HashSet<String> = std::collections::HashSet::new();
    for mrid in dataset.by_type.get("TopologicalIsland").into_iter().flatten() {
        let entry = &dataset.entries[mrid];
        if let Some(island) = entry.element.as_any().downcast_ref::<cimstructs::TopologicalIsland>() {
            for tn_ref in &island.topological_nodes {
                tn_in_island.insert(tn_ref.mrid.trim_start_matches('#').to_string());
            }
        }
    }

    // Build terminal index: equipment_id → terminal MRIDs
    let mut eq_terminals: std::collections::HashMap<String, Vec<String>> = std::collections::HashMap::new();
    for mrid in dataset.by_type.get("Terminal").into_iter().flatten() {
        let entry = &dataset.entries[mrid];
        if let Some(term) = entry.element.as_any().downcast_ref::<cimstructs::Terminal>() {
            if let Some(ce_ref) = term.conducting_equipment.as_ref() {
                eq_terminals.entry(ce_ref.mrid.trim_start_matches('#').to_string())
                    .or_default().push(mrid.clone());
            }
        }
    }

    // Build set of terminal MRIDs that have an SvPowerFlow
    let mut terminals_with_svpf: std::collections::HashSet<String> = std::collections::HashSet::new();
    for mrid in dataset.by_type.get("SvPowerFlow").into_iter().flatten() {
        let entry = &dataset.entries[mrid];
        if let Some(obj) = entry.element.as_any().downcast_ref::<cimstructs::SvPowerFlow>() {
            if let Some(t_ref) = obj.terminal.as_ref() {
                terminals_with_svpf.insert(t_ref.mrid.trim_start_matches('#').to_string());
            }
        }
    }

    let mut v = Vec::new();
    for type_name in INJECTION_TYPES {
        for mrid in dataset.by_type.get(*type_name).into_iter().flatten() {
            if !in_service.contains(mrid) { continue; }

            // Check if energized: at least one terminal connected to an island TN
            let energized = eq_terminals.get(mrid).map_or(false, |terms| {
                terms.iter().any(|t_mrid| {
                    dataset.entries.get(t_mrid)
                        .and_then(|e| e.element.as_any().downcast_ref::<cimstructs::Terminal>())
                        .and_then(|t| t.topological_node.as_ref())
                        .map_or(false, |tn_ref| tn_in_island.contains(tn_ref.mrid.trim_start_matches('#')))
                })
            });
            if !energized { continue; }

            let has_svpf = eq_terminals.get(mrid).map_or(false, |terms| {
                terms.iter().any(|t_mrid| terminals_with_svpf.contains(t_mrid))
            });
            if !has_svpf {
                v.push(Violation {
                    object_id:   mrid.clone(),
                    rule_id:     "svs456:SvPowerFlow-instance".into(),
                    name:        "R:456:SV:SvPowerFlow:instance".into(),
                    class:       (*type_name).to_string(),
                    property:    "rdf:type".into(),
                    message:     "SvPowerFlow is not instantiated for energized equipment.".into(),
                    severity:    "sh:Violation".into(),
                    description: String::new(),
                });
            }
        }
    }
    v
}

fn check_sv_power_flow_p_limits(dataset: &CimDataset) -> Vec<Violation> {
    let mut v = Vec::new();
    for mrid in dataset.by_type.get("SvPowerFlow").into_iter().flatten() {
        let entry = &dataset.entries[mrid];
        let svpf = match entry.element.as_any().downcast_ref::<cimstructs::SvPowerFlow>() {
            Some(o) => o, None => continue,
        };
        let p = match svpf.p { Some(p) => p, None => continue };
        let term_id = match svpf.terminal.as_ref() { Some(r) => r.mrid.trim_start_matches('#').to_string(), None => continue };
        let term = match dataset.entries.get(&term_id).and_then(|e| e.element.as_any().downcast_ref::<cimstructs::Terminal>()) {
            Some(t) => t, None => continue,
        };
        let eq_id = match term.conducting_equipment.as_ref() { Some(r) => r.mrid.trim_start_matches('#').to_string(), None => continue };
        let sm = match dataset.entries.get(&eq_id).and_then(|e| e.element.as_any().downcast_ref::<cimstructs::SynchronousMachine>()) {
            Some(o) => o, None => continue,
        };
        let gu_id = match sm.base.generating_unit.as_ref() { Some(r) => r.mrid.trim_start_matches('#').to_string(), None => continue };
        let gu = match dataset.entries.get(&gu_id).and_then(|e| e.element.as_any().downcast_ref::<cimstructs::GeneratingUnit>()) {
            Some(o) => o, None => continue,
        };
        let min_p = match gu.min_operating_p { Some(v) => v, None => continue };
        let max_p = match gu.max_operating_p { Some(v) => v, None => continue };
        let sm_id = sm.mrid();
        if p < min_p || p > max_p {
            v.push(Violation {
                object_id:   mrid.clone(),
                rule_id:     "svs456:SvPowerFlow.p-synchronousMachine".into(),
                name:        "C:456:SV:SvPowerFlow.p:synchronousMachine".into(),
                class:       "SvPowerFlow".into(),
                property:    "SvPowerFlow.p".into(),
                message:     format!("Active power ({p}) is outside of the range [Min:{min_p}, Max:{max_p}] for SynchronousMachine {sm_id}."),
                severity:    "sh:Warning".into(),
                description: String::new(),
            });
        }
    }
    v
}

fn check_sv_power_flow_q_limits(dataset: &CimDataset) -> Vec<Violation> {
    let mut v = Vec::new();
    for mrid in dataset.by_type.get("SvPowerFlow").into_iter().flatten() {
        let entry = &dataset.entries[mrid];
        let svpf = match entry.element.as_any().downcast_ref::<cimstructs::SvPowerFlow>() {
            Some(o) => o, None => continue,
        };
        let q = match svpf.q { Some(q) => q, None => continue };
        let term_id = match svpf.terminal.as_ref() { Some(r) => r.mrid.trim_start_matches('#').to_string(), None => continue };
        let term = match dataset.entries.get(&term_id).and_then(|e| e.element.as_any().downcast_ref::<cimstructs::Terminal>()) {
            Some(t) => t, None => continue,
        };
        let eq_id = match term.conducting_equipment.as_ref() { Some(r) => r.mrid.trim_start_matches('#').to_string(), None => continue };
        let sm = match dataset.entries.get(&eq_id).and_then(|e| e.element.as_any().downcast_ref::<cimstructs::SynchronousMachine>()) {
            Some(o) => o, None => continue,
        };

        let mut min_q = sm.min_q.unwrap_or(f64::NEG_INFINITY);
        let mut max_q = sm.max_q.unwrap_or(f64::INFINITY);

        // Check reactive capability curve if present
        if let Some(rcc_ref) = sm.initial_reactive_capability_curve.as_ref() {
            let rcc_id = rcc_ref.mrid.trim_start_matches('#');
            let mut y1_min = f64::INFINITY;
            let mut y2_max = f64::NEG_INFINITY;
            let mut found = false;
            for cd_mrid in dataset.by_type.get("CurveData").into_iter().flatten() {
                let cd_entry = &dataset.entries[cd_mrid];
                if let Some(cd) = cd_entry.element.as_any().downcast_ref::<cimstructs::CurveData>() {
                    if cd.curve.as_ref().map_or(false, |r| r.mrid.trim_start_matches('#') == rcc_id) {
                        if let Some(y1) = cd.y1value { y1_min = y1_min.min(y1); found = true; }
                        if let Some(y2) = cd.y2value { y2_max = y2_max.max(y2); }
                    }
                }
            }
            if found {
                min_q = y1_min;
                max_q = y2_max;
            }
        }

        if q < min_q || q > max_q {
            let sm_id = sm.mrid();
            v.push(Violation {
                object_id:   mrid.clone(),
                rule_id:     "svs456:SvPowerFlow.q-synchronousMachine".into(),
                name:        "C:456:SV:SvPowerFlow.q:synchronousMachine".into(),
                class:       "SvPowerFlow".into(),
                property:    "SvPowerFlow.q".into(),
                message:     format!("Reactive power ({q}) is outside of the capability range [Min:{min_q}, Max:{max_q}] for SynchronousMachine {sm_id}."),
                severity:    "sh:Warning".into(),
                description: String::new(),
            });
        }
    }
    v
}

fn check_sv_voltage_limits(dataset: &CimDataset) -> Vec<Violation> {
    let mut v = Vec::new();
    for mrid in dataset.by_type.get("SvVoltage").into_iter().flatten() {
        let entry = &dataset.entries[mrid];
        let svv = match entry.element.as_any().downcast_ref::<cimstructs::SvVoltage>() {
            Some(o) => o, None => continue,
        };
        let volt = match svv.v { Some(v) => v, None => continue };
        let tn_id = match svv.topological_node.as_ref() { Some(r) => r.mrid.trim_start_matches('#').to_string(), None => continue };
        let tn = match dataset.entries.get(&tn_id).and_then(|e| e.element.as_any().downcast_ref::<cimstructs::TopologicalNode>()) {
            Some(o) => o, None => continue,
        };
        let bv_id = match tn.base_voltage.as_ref() { Some(r) => r.mrid.trim_start_matches('#').to_string(), None => continue };
        let bv = match dataset.entries.get(&bv_id).and_then(|e| e.element.as_any().downcast_ref::<cimstructs::BaseVoltage>()) {
            Some(o) => o, None => continue,
        };
        let nom_v = match bv.nominal_voltage { Some(n) if n != 0.0 => n, _ => continue };

        if volt / nom_v <= 0.4 {
            v.push(Violation {
                object_id:   mrid.clone(),
                rule_id:     "svs456:SvVoltage.v-absoluteLimit".into(),
                name:        "C:456:SV:SvVoltage.v:absoluteLimit".into(),
                class:       "SvVoltage".into(),
                property:    "SvVoltage.v".into(),
                message:     format!("The value ({volt}) is <=0.4 pu of nominal voltage ({nom_v})."),
                severity:    "sh:Violation".into(),
                description: String::new(),
            });
        }
    }
    v
}
