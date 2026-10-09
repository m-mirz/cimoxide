use cimmodel::base::FastMap;
use cimmodel::{CimDataset, Element};
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
    v
}

fn tc_low_high(entry: &Element) -> Option<(i64, i64)> {
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

/// Whether the control is discrete and enabled, each written `true`.
fn rc_discrete_enabled(entry: &Element) -> (bool, bool) {
    let o = Fields::of(entry);
    (o.bool("RegulatingControl.discrete") == Some(true), o.bool("RegulatingControl.enabled") == Some(true))
}

/// Topological node → its terminals, reached either way the data says it:
/// `Terminal.TopologicalNode` (bus-branch) or through the terminal's
/// ConnectivityNode (`ConnectivityNode.TopologicalNode`, node-breaker), which
/// is the path the SPARQL follows.
fn node_terminals(dataset: &CimDataset) -> FastMap<&str, Vec<&str>> {
    let mut cn_tn: FastMap<&str, &str> = FastMap::default();
    for mrid in dataset.by_type.get("ConnectivityNode").into_iter().flatten() {
        if let Some(tn) = Fields::of(&dataset.entries[mrid]).reference("ConnectivityNode.TopologicalNode") {
            cn_tn.insert(mrid, tn.trim_start_matches('#'));
        }
    }
    let mut map: FastMap<&str, Vec<&str>> = FastMap::default();
    for mrid in dataset.by_type.get("Terminal").into_iter().flatten() {
        let t = Fields::of(&dataset.entries[mrid]);
        let direct = t.reference("Terminal.TopologicalNode").map(|r| r.trim_start_matches('#'));
        let via_cn = t.reference("Terminal.ConnectivityNode").and_then(|cn| cn_tn.get(cn.trim_start_matches('#')).copied());
        let mut tns = [direct, via_cn];
        if tns[0] == tns[1] { tns[1] = None; }
        for tn in tns.into_iter().flatten() {
            map.entry(tn).or_default().push(mrid);
        }
    }
    map
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

fn tap_changer_control_ref(entry: &Element) -> Option<&str> {
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

fn shunt_compensator_regulating_control_ref(entry: &Element) -> Option<&str> {
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
            && obj.bool("SvStatus.inService") == Some(true)
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

    // Terminals on a node in an island, by either path.
    let energized_terminals: cimmodel::base::FastSet<&str> = node_terminals(dataset).into_iter()
        .filter(|(tn, _)| tn_in_island.contains(tn))
        .flat_map(|(_, ts)| ts)
        .collect();

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
            let energized = eq_terminals.get(mrid.as_str())
                .is_some_and(|terms| terms.iter().any(|t| energized_terminals.contains(t)));
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

/// The machine's curve points, as (x, Some((y1, y2))) — y only where both are
/// given.
type CurvePoint = (Option<f64>, Option<(f64, f64)>);

fn curve_points(dataset: &CimDataset) -> FastMap<&str, Vec<CurvePoint>> {
    let mut map: FastMap<&str, Vec<_>> = FastMap::default();
    for mrid in dataset.by_type.get("CurveData").into_iter().flatten() {
        let cd = Fields::of(&dataset.entries[mrid]);
        if let Some(c) = cd.reference("CurveData.Curve") {
            let y = cd.f64("CurveData.y1value").zip(cd.f64("CurveData.y2value"));
            map.entry(c.trim_start_matches('#')).or_default().push((cd.f64("CurveData.xvalue"), y));
        }
    }
    map
}

/// Whether a flow, in the load sign convention, is outside `[min, max]` once
/// negated — the generator convention the limits are in. A flow of 0 is
/// compared as it is.
fn outside(flow: f64, min: f64, max: f64) -> bool {
    let generated = if flow == 0.0 { 0.0 } else { -flow };
    generated < min || generated > max
}

/// The synchronous machine an SvPowerFlow is on, with its terminal's flow.
fn machine_flows<'a>(dataset: &'a CimDataset, attr: &str) -> Vec<(&'a String, f64, Fields<'a>)> {
    let mut out = Vec::new();
    for mrid in dataset.by_type.get("SvPowerFlow").into_iter().flatten() {
        let svpf = Fields::of(&dataset.entries[mrid]);
        let Some(flow) = svpf.f64(attr) else { continue };
        let Some(sm) = svpf.reference("SvPowerFlow.Terminal")
            .and_then(|t| Fields::get(dataset, t.trim_start_matches('#'), "Terminal"))
            .and_then(|t| t.reference("Terminal.ConductingEquipment"))
            .and_then(|e| Fields::get(dataset, e.trim_start_matches('#'), "SynchronousMachine")) else { continue };
        out.push((mrid, flow, sm));
    }
    out
}

/// With a curve: within its x range (no x values, nothing to compare).
/// Without: within the unit's operating P, both required.
fn check_sv_power_flow_p_limits(dataset: &CimDataset) -> Vec<Violation> {
    let curves = curve_points(dataset);
    let mut v = Vec::new();
    for (mrid, p, sm) in machine_flows(dataset, "SvPowerFlow.p") {
        let (min_p, max_p) = match sm.reference("SynchronousMachine.InitialReactiveCapabilityCurve") {
            Some(c) => {
                let xs: Vec<f64> = curves.get(c.trim_start_matches('#')).into_iter().flatten().filter_map(|(x, _)| *x).collect();
                if xs.is_empty() { continue; }
                (xs.iter().copied().fold(f64::INFINITY, f64::min), xs.iter().copied().fold(f64::NEG_INFINITY, f64::max))
            }
            None => {
                let Some(gu) = sm.reference("RotatingMachine.GeneratingUnit")
                    .and_then(|g| dataset.entries.get(g.trim_start_matches('#'))).map(Fields::of) else { continue };
                let (Some(min_p), Some(max_p)) = (gu.f64("GeneratingUnit.minOperatingP"), gu.f64("GeneratingUnit.maxOperatingP")) else { continue };
                (min_p, max_p)
            }
        };
        if outside(p, min_p, max_p) {
            let sm_id = sm.id();
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

/// With a curve: within the lowest y1 and highest y2 of its points. Without:
/// within minQ and maxQ, both required.
fn check_sv_power_flow_q_limits(dataset: &CimDataset) -> Vec<Violation> {
    let curves = curve_points(dataset);
    let mut v = Vec::new();
    for (mrid, q, sm) in machine_flows(dataset, "SvPowerFlow.q") {
        let (min_q, max_q) = match sm.reference("SynchronousMachine.InitialReactiveCapabilityCurve") {
            Some(c) => {
                let ys: Vec<(f64, f64)> = curves.get(c.trim_start_matches('#')).into_iter().flatten().filter_map(|(_, y)| *y).collect();
                if ys.is_empty() { continue; }
                (ys.iter().map(|y| y.0).fold(f64::INFINITY, f64::min), ys.iter().map(|y| y.1).fold(f64::NEG_INFINITY, f64::max))
            }
            None => {
                let (Some(min_q), Some(max_q)) = (sm.f64("SynchronousMachine.minQ"), sm.f64("SynchronousMachine.maxQ")) else { continue };
                (min_q, max_q)
            }
        };
        if outside(q, min_q, max_q) {
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

/// Terminal → (highest high, lowest low) VoltageLimit value.
fn terminal_voltage_limits(dataset: &CimDataset) -> (FastMap<&str, f64>, FastMap<&str, f64>) {
    const HIGH: &str = "OperationalLimitDirectionKind.high";
    const LOW:  &str = "OperationalLimitDirectionKind.low";
    let mut high: FastMap<&str, f64> = FastMap::default();
    let mut low: FastMap<&str, f64> = FastMap::default();
    for mrid in dataset.by_type.get("VoltageLimit").into_iter().flatten() {
        let vl = Fields::of(&dataset.entries[mrid]);
        let Some(value) = vl.f64("VoltageLimit.value") else { continue };
        let Some(term) = vl.reference("OperationalLimit.OperationalLimitSet")
            .and_then(|s| dataset.entries.get(s.trim_start_matches('#')))
            .and_then(|s| Fields::of(s).reference("OperationalLimitSet.Terminal")) else { continue };
        let Some(direction) = vl.reference("OperationalLimit.OperationalLimitType")
            .and_then(|t| dataset.entries.get(t.trim_start_matches('#')))
            .and_then(|t| Fields::of(t).enumeration("OperationalLimitType.direction")) else { continue };
        let term = term.trim_start_matches('#');
        if direction == HIGH {
            high.entry(term).and_modify(|v| *v = v.max(value)).or_insert(value);
        } else if direction == LOW {
            low.entry(term).and_modify(|v| *v = v.min(value)).or_insert(value);
        }
    }
    (high, low)
}

/// The SvVoltage rules' SPARQL reads `SvVoltage.ToplogicalNode` (sic), which no
/// data carries, so neither can report; both are implemented as described.
/// `v-limits`: v within a terminal's high and low VoltageLimit, where the node
/// has a terminal with both. `v-absoluteLimit`: v above 0.4 pu where none does.
fn check_sv_voltage_limits(dataset: &CimDataset) -> Vec<Violation> {
    let terminals = node_terminals(dataset);
    let (high, low) = terminal_voltage_limits(dataset);
    let mut v = Vec::new();
    for mrid in dataset.by_type.get("SvVoltage").into_iter().flatten() {
        let svv = Fields::of(&dataset.entries[mrid]);
        let Some(volt) = svv.f64("SvVoltage.v") else { continue };
        let Some(tn_id) = svv.reference("SvVoltage.TopologicalNode").map(|r| r.trim_start_matches('#')) else { continue };
        let pairs: Vec<(f64, f64)> = terminals.get(tn_id).into_iter().flatten()
            .filter_map(|t| Some((*high.get(t)?, *low.get(t)?)))
            .collect();
        if !pairs.is_empty() {
            if pairs.iter().any(|&(vhigh, vlow)| volt > vhigh || volt < vlow) {
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
            continue;
        }
        let Some(nom_v) = Fields::get(dataset, tn_id, "TopologicalNode")
            .and_then(|tn| tn.reference("TopologicalNode.BaseVoltage"))
            .and_then(|bv| Fields::get(dataset, bv.trim_start_matches('#'), "BaseVoltage"))
            .and_then(|bv| bv.f64("BaseVoltage.nominalVoltage"))
            .filter(|n| *n != 0.0) else { continue };
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
