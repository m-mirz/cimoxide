use cimmodel::{CimDataset, CimEntry};
use crate::Violation;
use super::Fields;

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
    v.extend(check_sv_voltage_operational_limits(dataset));
    v
}

fn tc_low_high(entry: &CimEntry) -> Option<(i64, i64)> {
    if let Some(o) = Fields::of_class(entry, "RatioTapChanger") {
        return Some((o.i64("TapChanger.lowStep")?, o.i64("TapChanger.highStep")?));
    }
    if let Some(o) = Fields::of_class(entry, "PhaseTapChangerLinear") {
        return Some((o.i64("TapChanger.lowStep")?, o.i64("TapChanger.highStep")?));
    }
    if let Some(o) = Fields::of_class(entry, "PhaseTapChangerTabular") {
        return Some((o.i64("TapChanger.lowStep")?, o.i64("TapChanger.highStep")?));
    }
    if let Some(o) = Fields::of_class(entry, "PhaseTapChangerNonLinear") {
        return Some((o.i64("TapChanger.lowStep")?, o.i64("TapChanger.highStep")?));
    }
    if let Some(o) = Fields::of_class(entry, "PhaseTapChangerAsymmetrical") {
        return Some((o.i64("TapChanger.lowStep")?, o.i64("TapChanger.highStep")?));
    }
    if let Some(o) = Fields::of_class(entry, "PhaseTapChangerSymmetrical") {
        return Some((o.i64("TapChanger.lowStep")?, o.i64("TapChanger.highStep")?));
    }
    None
}

fn rc_discrete_enabled(entry: &CimEntry) -> (bool, bool) {
    if let Some(o) = Fields::of_class(entry, "RegulatingControl") {
        return (o.bool("RegulatingControl.discrete").unwrap_or(false), o.bool("RegulatingControl.enabled").unwrap_or(false));
    }
    if let Some(o) = Fields::of_class(entry, "TapChangerControl") {
        return (o.bool("RegulatingControl.discrete").unwrap_or(false), o.bool("RegulatingControl.enabled").unwrap_or(false));
    }
    (false, false)
}

fn check_sv_tap_step_position_range(dataset: &CimDataset) -> Vec<Violation> {
    let mut v = Vec::new();
    for mrid in dataset.by_type.get("SvTapStep").into_iter().flatten() {
        let entry = &dataset.entries[mrid];
        let obj = match Fields::of_class(entry, "SvTapStep") {
            Some(o) => o, None => continue,
        };
        let pos = match obj.f64("SvTapStep.position") { Some(p) => p, None => continue };
        let tc_ref = match obj.reference("SvTapStep.TapChanger") { Some(r) => r, None => continue };
        let tc_id = tc_ref.trim_start_matches('#');
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
        let obj = match Fields::of_class(entry, "SvTapStep") {
            Some(o) => o, None => continue,
        };
        let pos = match obj.f64("SvTapStep.position") { Some(p) => p, None => continue };
        let tc_ref = match obj.reference("SvTapStep.TapChanger") { Some(r) => r, None => continue };
        let tc_id = tc_ref.trim_start_matches('#');
        let tc_entry = match dataset.entries.get(tc_id) { Some(e) => e, None => continue };

        let tcc_ref = tap_changer_control_ref(tc_entry);
        let tcc_id = match tcc_ref { Some(r) => r.trim_start_matches('#').to_string(), None => continue };
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

fn tap_changer_control_ref(entry: &CimEntry) -> Option<&str> {
    if let Some(o) = Fields::of_class(entry, "RatioTapChanger") {
        return o.reference("TapChanger.TapChangerControl");
    }
    if let Some(o) = Fields::of_class(entry, "PhaseTapChangerLinear") {
        return o.reference("TapChanger.TapChangerControl");
    }
    if let Some(o) = Fields::of_class(entry, "PhaseTapChangerTabular") {
        return o.reference("TapChanger.TapChangerControl");
    }
    if let Some(o) = Fields::of_class(entry, "PhaseTapChangerNonLinear") {
        return o.reference("TapChanger.TapChangerControl");
    }
    if let Some(o) = Fields::of_class(entry, "PhaseTapChangerAsymmetrical") {
        return o.reference("TapChanger.TapChangerControl");
    }
    if let Some(o) = Fields::of_class(entry, "PhaseTapChangerSymmetrical") {
        return o.reference("TapChanger.TapChangerControl");
    }
    None
}

fn shunt_compensator_regulating_control_ref(entry: &CimEntry) -> Option<&str> {
    if let Some(o) = Fields::of_class(entry, "LinearShuntCompensator") {
        return o.reference("RegulatingCondEq.RegulatingControl");
    }
    if let Some(o) = Fields::of_class(entry, "NonlinearShuntCompensator") {
        return o.reference("RegulatingCondEq.RegulatingControl");
    }
    None
}

fn check_sv_shunt_compensator_sections_integer(dataset: &CimDataset) -> Vec<Violation> {
    let mut v = Vec::new();
    for mrid in dataset.by_type.get("SvShuntCompensatorSections").into_iter().flatten() {
        let entry = &dataset.entries[mrid];
        let obj = match Fields::of_class(entry, "SvShuntCompensatorSections") {
            Some(o) => o, None => continue,
        };
        let sections = match obj.f64("SvShuntCompensatorSections.sections") { Some(s) => s, None => continue };
        let sc_ref = match obj.reference("SvShuntCompensatorSections.ShuntCompensator") { Some(r) => r, None => continue };
        let sc_id = sc_ref.trim_start_matches('#');
        let sc_entry = match dataset.entries.get(sc_id) { Some(e) => e, None => continue };

        let rc_ref = match shunt_compensator_regulating_control_ref(sc_entry) { Some(r) => r, None => continue };
        let rc_id = rc_ref.trim_start_matches('#').to_string();
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
    let mut covered: cimmodel::base::FastSet<String> = cimmodel::base::FastSet::default();
    for mrid in dataset.by_type.get("SvSwitch").into_iter().flatten() {
        let entry = &dataset.entries[mrid];
        if let Some(obj) = Fields::of_class(entry, "SvSwitch")
            && let Some(sw_ref) = obj.reference("SvSwitch.Switch") {
                covered.insert(sw_ref.trim_start_matches('#').to_string());
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
    let mut in_service: cimmodel::base::FastSet<&str> = cimmodel::base::FastSet::default();
    for mrid in dataset.by_type.get("SvStatus").into_iter().flatten() {
        let entry = &dataset.entries[mrid];
        if let Some(obj) = Fields::of_class(entry, "SvStatus")
            && obj.bool("SvStatus.inService").unwrap_or(false)
                && let Some(ce_ref) = obj.reference("SvStatus.ConductingEquipment") {
                    in_service.insert(ce_ref.trim_start_matches('#'));
                }
    }

    // Build set of TN MRIDs that are in a topological island
    let mut tn_in_island: cimmodel::base::FastSet<&str> = cimmodel::base::FastSet::default();
    for mrid in dataset.by_type.get("TopologicalIsland").into_iter().flatten() {
        let entry = &dataset.entries[mrid];
        if let Some(island) = Fields::of_class(entry, "TopologicalIsland") {
            for tn_ref in island.references("TopologicalIsland.TopologicalNodes") {
                tn_in_island.insert(tn_ref.trim_start_matches('#'));
            }
        }
    }

    // Build terminal index: equipment_id → terminal MRIDs
    let mut eq_terminals: cimmodel::base::FastMap<&str, Vec<&str>> = cimmodel::base::FastMap::default();
    for mrid in dataset.by_type.get("Terminal").into_iter().flatten() {
        let entry = &dataset.entries[mrid];
        if let Some(term) = Fields::of_class(entry, "Terminal")
            && let Some(ce_ref) = term.reference("Terminal.ConductingEquipment") {
                eq_terminals.entry(ce_ref.trim_start_matches('#'))
                    .or_default().push(mrid);
            }
    }

    // Build set of terminal MRIDs that have an SvPowerFlow
    let mut terminals_with_svpf: cimmodel::base::FastSet<&str> = cimmodel::base::FastSet::default();
    for mrid in dataset.by_type.get("SvPowerFlow").into_iter().flatten() {
        let entry = &dataset.entries[mrid];
        if let Some(obj) = Fields::of_class(entry, "SvPowerFlow")
            && let Some(t_ref) = obj.reference("SvPowerFlow.Terminal") {
                terminals_with_svpf.insert(t_ref.trim_start_matches('#'));
            }
    }

    let mut v = Vec::new();
    for type_name in INJECTION_TYPES {
        for mrid in dataset.by_type.get(*type_name).into_iter().flatten() {
            if !in_service.contains(mrid.as_str()) { continue; }

            // Check if energized: at least one terminal connected to an island TN
            let energized = eq_terminals.get(mrid.as_str()).is_some_and(|terms| {
                terms.iter().any(|t_mrid| {
                    dataset.entries.get(*t_mrid)
                        .and_then(|e| Fields::of_class(e, "Terminal"))
                        .and_then(|t| t.reference("Terminal.TopologicalNode"))
                        .is_some_and(|tn_ref| tn_in_island.contains(tn_ref.trim_start_matches('#')))
                })
            });
            if !energized { continue; }

            let has_svpf = eq_terminals.get(mrid.as_str()).is_some_and(|terms| {
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
        let svpf = match Fields::of_class(entry, "SvPowerFlow") {
            Some(o) => o, None => continue,
        };
        let p = match svpf.f64("SvPowerFlow.p") { Some(p) => p, None => continue };
        let term_id = match svpf.reference("SvPowerFlow.Terminal") { Some(r) => r.trim_start_matches('#').to_string(), None => continue };
        let term = match dataset.entries.get(&term_id).and_then(|e| Fields::of_class(e, "Terminal")) {
            Some(t) => t, None => continue,
        };
        let eq_id = match term.reference("Terminal.ConductingEquipment") { Some(r) => r.trim_start_matches('#').to_string(), None => continue };
        let sm = match dataset.entries.get(&eq_id).and_then(|e| Fields::of_class(e, "SynchronousMachine")) {
            Some(o) => o, None => continue,
        };
        let gu_id = match sm.reference("RotatingMachine.GeneratingUnit") { Some(r) => r.trim_start_matches('#').to_string(), None => continue };
        let gu = match dataset.entries.get(&gu_id).and_then(|e| Fields::of_class(e, "GeneratingUnit")) {
            Some(o) => o, None => continue,
        };
        let min_p = match gu.f64("GeneratingUnit.minOperatingP") { Some(v) => v, None => continue };
        let max_p = match gu.f64("GeneratingUnit.maxOperatingP") { Some(v) => v, None => continue };
        let sm_id = sm.id();
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
        let svpf = match Fields::of_class(entry, "SvPowerFlow") {
            Some(o) => o, None => continue,
        };
        let q = match svpf.f64("SvPowerFlow.q") { Some(q) => q, None => continue };
        let term_id = match svpf.reference("SvPowerFlow.Terminal") { Some(r) => r.trim_start_matches('#').to_string(), None => continue };
        let term = match dataset.entries.get(&term_id).and_then(|e| Fields::of_class(e, "Terminal")) {
            Some(t) => t, None => continue,
        };
        let eq_id = match term.reference("Terminal.ConductingEquipment") { Some(r) => r.trim_start_matches('#').to_string(), None => continue };
        let sm = match dataset.entries.get(&eq_id).and_then(|e| Fields::of_class(e, "SynchronousMachine")) {
            Some(o) => o, None => continue,
        };

        let mut min_q = sm.f64("SynchronousMachine.minQ").unwrap_or(f64::NEG_INFINITY);
        let mut max_q = sm.f64("SynchronousMachine.maxQ").unwrap_or(f64::INFINITY);

        // Check reactive capability curve if present
        if let Some(rcc_ref) = sm.reference("SynchronousMachine.InitialReactiveCapabilityCurve") {
            let rcc_id = rcc_ref.trim_start_matches('#');
            let mut y1_min = f64::INFINITY;
            let mut y2_max = f64::NEG_INFINITY;
            let mut found = false;
            for cd_mrid in dataset.by_type.get("CurveData").into_iter().flatten() {
                let cd_entry = &dataset.entries[cd_mrid];
                if let Some(cd) = Fields::of_class(cd_entry, "CurveData")
                    && cd.reference("CurveData.Curve").is_some_and(|r| r.trim_start_matches('#') == rcc_id) {
                        if let Some(y1) = cd.f64("CurveData.y1value") { y1_min = y1_min.min(y1); found = true; }
                        if let Some(y2) = cd.f64("CurveData.y2value") { y2_max = y2_max.max(y2); }
                    }
            }
            if found {
                min_q = y1_min;
                max_q = y2_max;
            }
        }

        if q < min_q || q > max_q {
            let sm_id = sm.id();
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
        let svv = match Fields::of_class(entry, "SvVoltage") {
            Some(o) => o, None => continue,
        };
        let volt = match svv.f64("SvVoltage.v") { Some(v) => v, None => continue };
        let tn_id = match svv.reference("SvVoltage.TopologicalNode") { Some(r) => r.trim_start_matches('#').to_string(), None => continue };
        let tn = match dataset.entries.get(&tn_id).and_then(|e| Fields::of_class(e, "TopologicalNode")) {
            Some(o) => o, None => continue,
        };
        let bv_id = match tn.reference("TopologicalNode.BaseVoltage") { Some(r) => r.trim_start_matches('#').to_string(), None => continue };
        let bv = match dataset.entries.get(&bv_id).and_then(|e| Fields::of_class(e, "BaseVoltage")) {
            Some(o) => o, None => continue,
        };
        let nom_v = match bv.f64("BaseVoltage.nominalVoltage") { Some(n) if n != 0.0 => n, _ => continue };

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

fn check_sv_voltage_operational_limits(dataset: &CimDataset) -> Vec<Violation> {
    const HIGH: &str = "OperationalLimitDirectionKind.high";
    const LOW:  &str = "OperationalLimitDirectionKind.low";

    // TopologicalNode -> terminals connected to it (via Terminal.TopologicalNode directly,
    // same convention as check_sv_power_flow_instance above).
    let mut tn_terminals: cimmodel::base::FastMap<String, Vec<String>> = cimmodel::base::FastMap::default();
    for mrid in dataset.by_type.get("Terminal").into_iter().flatten() {
        let entry = &dataset.entries[mrid];
        if let Some(term) = Fields::of_class(entry, "Terminal")
            && let Some(tn_ref) = term.reference("Terminal.TopologicalNode") {
                tn_terminals.entry(tn_ref.trim_start_matches('#').to_string())
                    .or_default().push(mrid.clone());
            }
    }

    // OperationalLimitSet -> terminal.
    let mut ols_terminal: cimmodel::base::FastMap<String, String> = cimmodel::base::FastMap::default();
    for mrid in dataset.by_type.get("OperationalLimitSet").into_iter().flatten() {
        let entry = &dataset.entries[mrid];
        if let Some(ols) = Fields::of_class(entry, "OperationalLimitSet")
            && let Some(term_ref) = ols.reference("OperationalLimitSet.Terminal") {
                ols_terminal.insert(mrid.clone(), term_ref.trim_start_matches('#').to_string());
            }
    }

    // OperationalLimitType -> direction.
    let mut olt_direction: cimmodel::base::FastMap<String, String> = cimmodel::base::FastMap::default();
    for mrid in dataset.by_type.get("OperationalLimitType").into_iter().flatten() {
        let entry = &dataset.entries[mrid];
        if let Some(olt) = Fields::of_class(entry, "OperationalLimitType")
            && let Some(dir) = olt.enumeration("OperationalLimitType.direction") {
                olt_direction.insert(mrid.clone(), dir.to_string());
            }
    }

    // terminal_id -> (max high VoltageLimit.value, min low VoltageLimit.value)
    let mut terminal_vhigh: cimmodel::base::FastMap<String, f64> = cimmodel::base::FastMap::default();
    let mut terminal_vlow: cimmodel::base::FastMap<String, f64> = cimmodel::base::FastMap::default();
    for mrid in dataset.by_type.get("VoltageLimit").into_iter().flatten() {
        let entry = &dataset.entries[mrid];
        let vl = match Fields::of_class(entry, "VoltageLimit") { Some(o) => o, None => continue };
        let value = match vl.f64("VoltageLimit.value") { Some(v) => v, None => continue };
        let ols_id = match vl.reference("OperationalLimit.OperationalLimitSet") { Some(r) => r.trim_start_matches('#'), None => continue };
        let term_id = match ols_terminal.get(ols_id) { Some(t) => t, None => continue };
        let olt_id = match vl.reference("OperationalLimit.OperationalLimitType") { Some(r) => r.trim_start_matches('#'), None => continue };
        let direction = match olt_direction.get(olt_id) { Some(d) => d.as_str(), None => continue };

        if direction == HIGH {
            terminal_vhigh.entry(term_id.clone())
                .and_modify(|v| *v = v.max(value))
                .or_insert(value);
        } else if direction == LOW {
            terminal_vlow.entry(term_id.clone())
                .and_modify(|v| *v = v.min(value))
                .or_insert(value);
        }
    }

    let mut v = Vec::new();
    for mrid in dataset.by_type.get("SvVoltage").into_iter().flatten() {
        let entry = &dataset.entries[mrid];
        let svv = match Fields::of_class(entry, "SvVoltage") { Some(o) => o, None => continue };
        let volt = match svv.f64("SvVoltage.v") { Some(v) => v, None => continue };
        let tn_id = match svv.reference("SvVoltage.TopologicalNode") { Some(r) => r.trim_start_matches('#'), None => continue };
        let terms = match tn_terminals.get(tn_id) { Some(t) => t, None => continue };

        let out_of_range = terms.iter().any(|term_id| {
            match (terminal_vhigh.get(term_id), terminal_vlow.get(term_id)) {
                (Some(&vhigh), Some(&vlow)) => volt > vhigh || volt < vlow,
                _ => false,
            }
        });
        if out_of_range {
            v.push(Violation {
                object_id:   mrid.clone(),
                rule_id:     "svs456:SvVoltage.v-limits".into(),
                name:        "C:456:SV:SvVoltage.v:limits".into(),
                class:       "SvVoltage".into(),
                property:    "SvVoltage.v".into(),
                message:     format!("The value ({volt}) is outside the defined OperationalLimit (VoltageLimit) bounds."),
                severity:    "sh:Violation".into(),
                description: String::new(),
            });
        }
    }
    v
}
