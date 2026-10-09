use cimmodel::base::FastMap as HashMap;
use cimmodel::CimDataset;
use crate::Violation;
use super::Fields;
use super::ssh;

pub fn validate(dataset: &CimDataset) -> Vec<Violation> {
    let mut v = Vec::new();
    v.extend(check_linear_shunt_compensator_sections_range(dataset));
    v.extend(check_nonlinear_shunt_compensator_sections_valid(dataset));
    v.extend(check_shunt_compensator_sections_integer(dataset));
    v.extend(check_regulating_control_power_factor_required_attrs(dataset));
    v.extend(check_tap_changer_step_integer(dataset));
    v.extend(check_cs_converter_target_alpha_applicability(dataset));
    v.extend(check_cs_converter_target_gamma_applicability(dataset));
    v.extend(check_control_area_net_interchange_calculation(dataset));
    v.extend(check_equivalent_injection_regulation(dataset));
    v.extend(check_rotating_machine_p_limits(dataset));
    v.extend(check_rotating_machine_q_limits(dataset));
    v.extend(ssh::check_synchronous_machine_operating_mode_match(dataset));
    v.extend(ssh::check_generating_unit_single_active_power_slack(dataset));
    v.extend(ssh::check_external_network_injection_limits(dataset));
    v.extend(ssh::check_equivalent_injection_limits(dataset));
    v.extend(ssh::check_rotating_machine_curve_limits(dataset));
    v.extend(ssh::check_regulating_control_target_value_positive(dataset));
    v
}

fn check_linear_shunt_compensator_sections_range(dataset: &CimDataset) -> Vec<Violation> {
    let mut v = Vec::new();
    for mrid in dataset.by_type.get("LinearShuntCompensator").into_iter().flatten() {
        let entry = &dataset.entries[mrid];
        if let Some(lsc) = Fields::of_class(entry, "LinearShuntCompensator") {
            // Both are required patterns.
            let (Some(sections), Some(max_sections)) =
                (lsc.f64("ShuntCompensator.sections"), lsc.f64("ShuntCompensator.maximumSections")) else { continue };
            if sections < 0.0 || sections > max_sections {
                v.push(Violation {
                    object_id: mrid.clone(), rule_id: "sshn301:ShuntCompensator.sections-valueLinear".into(),
                    name: "C:301:SSH:ShuntCompensator.sections:valueLinear".into(), class: "LinearShuntCompensator".into(),
                    property: "ShuntCompensator.sections".into(),
                    message: format!("The value ({}) is not between zero and ShuntCompensator.maximumSections ({}).", sections, max_sections as i64),
                    severity: "sh:Violation".into(), description: String::new(),
                });
            }
        }
    }
    v
}

/// The section count must be one of the compensator's points. The SPARQL counts
/// the points whose section number differs from the count, and reports when
/// that is all of them; a point without a section number, or a compensator
/// without points, leaves nothing to report.
fn check_nonlinear_shunt_compensator_sections_valid(dataset: &CimDataset) -> Vec<Violation> {
    // Compensator → its points' section numbers, `None` for a point without one.
    let mut points: HashMap<&str, Vec<Option<f64>>> = HashMap::default();
    for mrid in dataset.by_type.get("NonlinearShuntCompensatorPoint").into_iter().flatten() {
        let entry = &dataset.entries[mrid];
        if let Some(pt) = Fields::of_class(entry, "NonlinearShuntCompensatorPoint")
            && let Some(r) = pt.reference("NonlinearShuntCompensatorPoint.NonlinearShuntCompensator") {
                points.entry(r.trim_start_matches('#')).or_default().push(pt.f64("NonlinearShuntCompensatorPoint.sectionNumber"));
            }
    }
    let mut v = Vec::new();
    for mrid in dataset.by_type.get("NonlinearShuntCompensator").into_iter().flatten() {
        let entry = &dataset.entries[mrid];
        let Some(nsc) = Fields::of_class(entry, "NonlinearShuntCompensator") else { continue };
        let Some(section) = nsc.f64("ShuntCompensator.sections") else { continue };
        let Some(pts) = points.get(mrid.as_str()) else { continue };
        if pts.iter().all(|n| n.is_some_and(|n| n != section)) {
            v.push(Violation {
                object_id: mrid.clone(), rule_id: "sshn301:ShuntCompensator.sections-valueNonLinear".into(),
                name: "C:301:SSH:ShuntCompensator.sections:valueNonLinear".into(), class: "NonlinearShuntCompensator".into(),
                property: "ShuntCompensator.sections".into(),
                message: format!("The value ({}) does not equal one of the NonlinearShuntCompenstorPoint.sectionNumber.", section),
                severity: "sh:Violation".into(), description: String::new(),
            });
        }
    }
    v
}

fn check_shunt_compensator_sections_integer(dataset: &CimDataset) -> Vec<Violation> {
    let mut v = Vec::new();
    // The SPARQL tests the value's text for a trailing ".", which only "3." has;
    // the rule it describes is a non-integer count under an enabled, discrete
    // control.
    let check_sc = |mrid: &str, class: &str, sections: Option<f64>, rc_id: Option<&str>, v: &mut Vec<Violation>| {
        let Some(sections) = sections else { return };
        let rc_id = match rc_id { Some(id) => id, None => return };
        let rc_entry = match dataset.entries.get(rc_id) { Some(e) => e, None => return };
        let rc = Fields::of(rc_entry);
        let (enabled, discrete) = (rc.bool("RegulatingControl.enabled"), rc.bool("RegulatingControl.discrete"));
        if enabled == Some(true) && discrete == Some(true) && sections != sections.floor() {
            v.push(Violation {
                object_id: mrid.to_string(), rule_id: "sshn456:ShuntCompensator.sections-value".into(),
                name: "C:456:SSH:ShuntCompensator.sections:value".into(), class: class.to_string(),
                property: "ShuntCompensator.sections".into(),
                message: format!("The value ({}) is not integer for an active discrete regulating control.", sections),
                severity: "sh:Violation".into(), description: String::new(),
            });
        }
    };
    for mrid in dataset.by_type.get("LinearShuntCompensator").into_iter().flatten() {
        let entry = &dataset.entries[mrid];
        if let Some(lsc) = Fields::of_class(entry, "LinearShuntCompensator") {
            let rc_id = lsc.reference("RegulatingCondEq.RegulatingControl").map(|r| r.trim_start_matches('#'));
            check_sc(mrid, "LinearShuntCompensator", lsc.f64("ShuntCompensator.sections"), rc_id, &mut v);
        }
    }
    for mrid in dataset.by_type.get("NonlinearShuntCompensator").into_iter().flatten() {
        let entry = &dataset.entries[mrid];
        if let Some(nsc) = Fields::of_class(entry, "NonlinearShuntCompensator") {
            let rc_id = nsc.reference("RegulatingCondEq.RegulatingControl").map(|r| r.trim_start_matches('#'));
            check_sc(mrid, "NonlinearShuntCompensator", nsc.f64("ShuntCompensator.sections"), rc_id, &mut v);
        }
    }
    v
}

fn check_regulating_control_power_factor_required_attrs(dataset: &CimDataset) -> Vec<Violation> {
    let power_factor_uri = "RegulatingControlModeKind.powerFactor";
    let mut v = Vec::new();
    // Either limit absent (`!bound`); 0 is a limit.
    let check = |mrid: &str, class: &str, mode_uri: &str, has_min: bool, has_max: bool, v: &mut Vec<Violation>| {
        if mode_uri != power_factor_uri { return; }
        if !has_min || !has_max {
            v.push(Violation {
                object_id: mrid.to_string(), rule_id: "sshn301:RegulatingControl-requiredAttributes".into(),
                name: "C:301:SSH:RegulatingControl:requiredAttributes".into(), class: class.to_string(),
                property: "RegulatingControl.mode".into(),
                message: "Both minAllowedTargetValue and maxAllowedTargetValue are not provided for RegulatingControl in mode powerFactor.".into(),
                severity: "sh:Violation".into(), description: String::new(),
            });
        }
    };
    for mrid in dataset.by_type.get("RegulatingControl").into_iter().flatten() {
        let entry = &dataset.entries[mrid];
        if let Some(rc) = Fields::of_class(entry, "RegulatingControl")
            && let Some(mode) = &rc.enumeration("RegulatingControl.mode") {
                check(mrid, "RegulatingControl", mode, rc.has("RegulatingControl.minAllowedTargetValue"), rc.has("RegulatingControl.maxAllowedTargetValue"), &mut v);
            }
    }
    for mrid in dataset.by_type.get("TapChangerControl").into_iter().flatten() {
        let entry = &dataset.entries[mrid];
        if let Some(tcc) = Fields::of_class(entry, "TapChangerControl")
            && let Some(mode) = &tcc.enumeration("RegulatingControl.mode") {
                check(mrid, "TapChangerControl", mode, tcc.has("RegulatingControl.minAllowedTargetValue"), tcc.has("RegulatingControl.maxAllowedTargetValue"), &mut v);
            }
    }
    v
}

fn check_tap_changer_step_integer(dataset: &CimDataset) -> Vec<Violation> {
    let mut tcc_discrete_enabled: HashMap<String, (bool, bool)> = HashMap::default();
    for mrid in dataset.by_type.get("TapChangerControl").into_iter().flatten() {
        let entry = &dataset.entries[mrid];
        if let Some(tcc) = Fields::of_class(entry, "TapChangerControl") {
            tcc_discrete_enabled.insert(mrid.clone(), (tcc.bool("RegulatingControl.discrete") == Some(true), tcc.bool("RegulatingControl.enabled") == Some(true)));
        }
    }
    let mut v = Vec::new();
    // As for the shunt sections, the SPARQL's trailing-"." test stands for
    // "not an integer".
    let report = |mrid: &str, class: &str, step: Option<f64>, tcc_mrid: Option<&str>, v: &mut Vec<Violation>| {
        let Some(step) = step else { return };
        let tcc_id = match tcc_mrid { Some(id) => id, None => return };
        let Some(&(discrete, enabled)) = tcc_discrete_enabled.get(tcc_id) else { return };
        if !discrete { return; }
        if step != step.floor() || step.is_nan() {
            v.push(Violation {
                object_id: mrid.to_string(), rule_id: "sshn301:TapChanger.step-valueType".into(),
                name: "C:301:SSH:TapChanger.step:valueType".into(), class: class.to_string(),
                property: "TapChanger.step".into(),
                message: format!("Non-integer value ({}) for a discrete TapChangerControl.", step),
                severity: "sh:Violation".into(), description: String::new(),
            });
            if enabled {
                v.push(Violation {
                    object_id: mrid.to_string(), rule_id: "sshn456:TapChanger.step-value".into(),
                    name: "C:456:SSH:TapChanger.step:value".into(), class: class.to_string(),
                    property: "TapChanger.step".into(),
                    message: format!("Non-integer value ({}) for an active (enabled) discrete TapChangerControl.", step),
                    severity: "sh:Violation".into(), description: String::new(),
                });
            }
        }
    };
    for mrid in dataset.by_type.get("RatioTapChanger").into_iter().flatten() {
        let entry = &dataset.entries[mrid];
        if let Some(tc) = Fields::of_class(entry, "RatioTapChanger") {
            let tcc_id = tc.reference("TapChanger.TapChangerControl").map(|r| r.trim_start_matches('#'));
            report(mrid, "RatioTapChanger", tc.f64("TapChanger.step"), tcc_id, &mut v);
        }
    }
    for mrid in dataset.by_type.get("PhaseTapChangerLinear").into_iter().flatten() {
        let entry = &dataset.entries[mrid];
        if let Some(tc) = Fields::of_class(entry, "PhaseTapChangerLinear") {
            let tcc_id = tc.reference("TapChanger.TapChangerControl").map(|r| r.trim_start_matches('#'));
            report(mrid, "PhaseTapChangerLinear", tc.f64("TapChanger.step"), tcc_id, &mut v);
        }
    }
    for mrid in dataset.by_type.get("PhaseTapChangerSymmetrical").into_iter().flatten() {
        let entry = &dataset.entries[mrid];
        if let Some(tc) = Fields::of_class(entry, "PhaseTapChangerSymmetrical") {
            let tcc_id = tc.reference("TapChanger.TapChangerControl").map(|r| r.trim_start_matches('#'));
            report(mrid, "PhaseTapChangerSymmetrical", tc.f64("TapChanger.step"), tcc_id, &mut v);
        }
    }
    for mrid in dataset.by_type.get("PhaseTapChangerAsymmetrical").into_iter().flatten() {
        let entry = &dataset.entries[mrid];
        if let Some(tc) = Fields::of_class(entry, "PhaseTapChangerAsymmetrical") {
            let tcc_id = tc.reference("TapChanger.TapChangerControl").map(|r| r.trim_start_matches('#'));
            report(mrid, "PhaseTapChangerAsymmetrical", tc.f64("TapChanger.step"), tcc_id, &mut v);
        }
    }
    for mrid in dataset.by_type.get("PhaseTapChangerTabular").into_iter().flatten() {
        let entry = &dataset.entries[mrid];
        if let Some(tc) = Fields::of_class(entry, "PhaseTapChangerTabular") {
            let tcc_id = tc.reference("TapChanger.TapChangerControl").map(|r| r.trim_start_matches('#'));
            report(mrid, "PhaseTapChangerTabular", tc.f64("TapChanger.step"), tcc_id, &mut v);
        }
    }
    v
}

fn check_cs_converter_target_alpha_applicability(dataset: &CimDataset) -> Vec<Violation> {
    check_cs_converter_target_angle_applicability(dataset, true)
}

fn check_cs_converter_target_gamma_applicability(dataset: &CimDataset) -> Vec<Violation> {
    check_cs_converter_target_angle_applicability(dataset, false)
}

fn check_cs_converter_target_angle_applicability(dataset: &CimDataset, for_alpha: bool) -> Vec<Violation> {
    let inverter   = "CsOperatingModeKind.inverter";
    let rectifier  = "CsOperatingModeKind.rectifier";
    // terminalID → RC.discrete
    let mut rc_discrete: HashMap<String, Option<bool>> = HashMap::default();
    for mrid in dataset.by_type.get("RegulatingControl").into_iter().flatten() {
        let entry = &dataset.entries[mrid];
        if let Some(rc) = Fields::of_class(entry, "RegulatingControl")
            && let Some(r) = &rc.reference("RegulatingControl.Terminal") {
                rc_discrete.insert(r.trim_start_matches('#').to_string(), rc.bool("RegulatingControl.discrete"));
            }
    }
    for mrid in dataset.by_type.get("TapChangerControl").into_iter().flatten() {
        let entry = &dataset.entries[mrid];
        if let Some(tcc) = Fields::of_class(entry, "TapChangerControl")
            && let Some(r) = &tcc.reference("RegulatingControl.Terminal") {
                rc_discrete.insert(r.trim_start_matches('#').to_string(), tcc.bool("RegulatingControl.discrete"));
            }
    }
    let (rule_id, rule_name, prop, msg) = if for_alpha {
        ("sshn301:CsConverter.targetAlpha-applicability", "C:301:SSH:CsConverter.targetAlpha:applicability", "CsConverter.targetAlpha",
         "CsConverter.targetAlpha is provided for an inverter or discrete tap changer control is used or RegulatingControl is not provided.")
    } else {
        ("sshn301:CsConverter.targetGamma-applicability", "C:301:SSH:CsConverter.targetGamma:applicability", "CsConverter.targetGamma",
         "CsConverter.targetGamma is provided for a rectifier or discrete tap changer control is used or RegulatingControl is not provided.")
    };
    let mut v = Vec::new();
    for mrid in dataset.by_type.get("CsConverter").into_iter().flatten() {
        let entry = &dataset.entries[mrid];
        if let Some(csc) = Fields::of_class(entry, "CsConverter") {
            // The target given at all — 0 included — is what the rule is about.
            if !csc.has(if for_alpha { "CsConverter.targetAlpha" } else { "CsConverter.targetGamma" }) { continue; }
            let mode = match csc.enumeration("CsConverter.operatingMode") { Some(r) => r, None => continue };
            let invalid_mode = if for_alpha { inverter } else { rectifier };
            let emit = |v: &mut Vec<Violation>, mrid: &str| {
                v.push(Violation {
                    object_id: mrid.to_string(), rule_id: rule_id.into(), name: rule_name.into(),
                    class: "CsConverter".into(), property: prop.into(), message: msg.into(),
                    severity: "sh:Violation".into(), description: String::new(),
                });
            };
            if mode == invalid_mode { emit(&mut v, mrid); continue; }
            let pcc_term_id = match csc.reference("ACDCConverter.PccTerminal") { Some(r) => r.trim_start_matches('#').to_string(), None => { emit(&mut v, mrid); continue; } };
            let pcc_term = match dataset.entries.get(&pcc_term_id).and_then(|e| Fields::of_class(e, "Terminal")) {
                Some(t) => t, None => { emit(&mut v, mrid); continue; }
            };
            let eq_id = match pcc_term.reference("Terminal.ConductingEquipment") { Some(r) => r.trim_start_matches('#').to_string(), None => { emit(&mut v, mrid); continue; } };
            let is_pt = dataset.entries.get(&eq_id).is_some_and(|e| Fields::of_class(e, "PowerTransformer").is_some());
            if !is_pt { emit(&mut v, mrid); continue; }
            // `discrete` true, or not established (`!bound(?discrete)`).
            if rc_discrete.get(&pcc_term_id).copied().flatten() != Some(false) {
                emit(&mut v, mrid);
            }
        }
    }
    v
}

fn check_control_area_net_interchange_calculation(dataset: &CimDataset) -> Vec<Violation> {
    let interchange_uri = "ControlAreaTypeKind.Interchange";
    let mut cn_has_bp: cimmodel::base::FastSet<String> = cimmodel::base::FastSet::default();
    for mrid in dataset.by_type.get("BoundaryPoint").into_iter().flatten() {
        let entry = &dataset.entries[mrid];
        if let Some(bp) = Fields::of_class(entry, "BoundaryPoint")
            && let Some(r) = &bp.reference("BoundaryPoint.ConnectivityNode") {
                cn_has_bp.insert(r.trim_start_matches('#').to_string());
            }
    }
    let mut ca_terminals: HashMap<String, Vec<String>> = HashMap::default();
    for mrid in dataset.by_type.get("TieFlow").into_iter().flatten() {
        let entry = &dataset.entries[mrid];
        if let Some(tf) = Fields::of_class(entry, "TieFlow")
            && let (Some(ca), Some(term)) = (&tf.reference("TieFlow.ControlArea"), &tf.reference("TieFlow.Terminal")) {
                let ca_id = ca.trim_start_matches('#').to_string();
                let term_id = term.trim_start_matches('#').to_string();
                ca_terminals.entry(ca_id).or_default().push(term_id);
            }
    }
    let mut v = Vec::new();
    for mrid in dataset.by_type.get("ControlArea").into_iter().flatten() {
        let entry = &dataset.entries[mrid];
        if let Some(ca) = Fields::of_class(entry, "ControlArea") {
            let is_interchange = ca.enumeration("ControlArea.type").is_some_and(|r| r == interchange_uri);
            // netInterchange given, 0 included; without a single injection to sum
            // the SPARQL's subquery has no row, so nothing is compared.
            let Some(net_interchange) = ca.f64("ControlArea.netInterchange") else { continue };
            if !is_interchange { continue; }
            let mut sum = 0.0;
            let mut summed = 0;
            for term_id in ca_terminals.get(mrid).into_iter().flatten() {
                let term = match dataset.entries.get(term_id).and_then(|e| Fields::of_class(e, "Terminal")) { Some(t) => t, None => continue };
                let cn_id = match term.reference("Terminal.ConnectivityNode") { Some(r) => r.trim_start_matches('#').to_string(), None => continue };
                if !cn_has_bp.contains(&cn_id) { continue; }
                let eq_id = match term.reference("Terminal.ConductingEquipment") { Some(r) => r.trim_start_matches('#').to_string(), None => continue };
                if let Some(p) = dataset.entries.get(&eq_id)
                    .and_then(|e| Fields::of_class(e, "EquivalentInjection"))
                    .and_then(|ei| ei.f64("EquivalentInjection.p")) {
                    sum += p;
                    summed += 1;
                }
            }
            // A sum of decimals is not exact; the SPARQL's `!=` would report
            // rounding.
            if summed > 0 && (net_interchange - sum).abs() > 1e-6 * net_interchange.abs().max(1.0) {
                v.push(Violation {
                    object_id: mrid.clone(), rule_id: "sshn301:ControlArea-netInterchangeCalculation".into(),
                    name: "C:301:SSH:ControlArea:netInterchangeCalculation".into(), class: "ControlArea".into(),
                    property: "ControlArea.netInterchange".into(),
                    message: format!("The sum of the EquivalentInjections which are connected to the BoundaryPoint-s differs from the ControlArea.netInterchange. ControlArea.netInterchange= {}. Sum of the EquivalentInjections= {}.", net_interchange, sum),
                    severity: "sh:Violation".into(), description: String::new(),
                });
            }
        }
    }
    v
}

fn check_equivalent_injection_regulation(dataset: &CimDataset) -> Vec<Violation> {
    let mut v = Vec::new();
    for mrid in dataset.by_type.get("EquivalentInjection").into_iter().flatten() {
        let entry = &dataset.entries[mrid];
        if let Some(ei) = Fields::of_class(entry, "EquivalentInjection") {
            // Status and target by presence; regulationCapability is the path.
            let status = ei.has("EquivalentInjection.regulationStatus");
            let target = ei.has("EquivalentInjection.regulationTarget");
            let Some(capable) = ei.bool("EquivalentInjection.regulationCapability") else { continue };
            if capable {
                if !status || !target {
                    v.push(Violation {
                        object_id: mrid.clone(), rule_id: "sshn456:EquivalentInjection-regulation".into(),
                        name: "C:456:SSH:EquivalentInjection:regulation".into(), class: "EquivalentInjection".into(),
                        property: "regulationStatus".into(),
                        message: "EquivalentInjection.regulationStatus and regulationTarget are required when regulationCapability is true.".into(),
                        severity: "sh:Violation".into(), description: String::new(),
                    });
                }
            } else if status || target {
                v.push(Violation {
                    object_id: mrid.clone(), rule_id: "sshn456:EquivalentInjection-regulation".into(),
                    name: "C:456:SSH:EquivalentInjection:regulation".into(), class: "EquivalentInjection".into(),
                    property: "regulationStatus".into(),
                    message: "EquivalentInjection.regulationStatus and regulationTarget should not be exchanged when regulationCapability is false.".into(),
                    severity: "sh:Violation".into(), description: String::new(),
                });
            }
        }
    }
    v
}

/// The machine in service, its p, and its unit's operating limits, all
/// required patterns.
fn check_rotating_machine_p_limits(dataset: &CimDataset) -> Vec<Violation> {
    let mut v = Vec::new();
    for class in ["SynchronousMachine", "AsynchronousMachine"] {
        for mrid in dataset.by_type.get(class).into_iter().flatten() {
            let Some(rm) = Fields::of_class(&dataset.entries[mrid], class) else { continue };
            if rm.bool("Equipment.inService") != Some(true) { continue; }
            let Some(p) = rm.f64("RotatingMachine.p") else { continue };
            let Some(gu) = rm.reference("RotatingMachine.GeneratingUnit")
                .and_then(|g| dataset.entries.get(g.trim_start_matches('#')))
                .map(Fields::of) else { continue };
            let (Some(min), Some(max)) = (gu.f64("GeneratingUnit.minOperatingP"), gu.f64("GeneratingUnit.maxOperatingP")) else { continue };
            let neg_p = if p == 0.0 { 0.0 } else { -p };
            if neg_p < min || neg_p > max {
                v.push(Violation {
                    object_id: mrid.to_string(), rule_id: "sshn456:RotatingMachine.p-limits".into(),
                    name: "C:456:SSH:RotatingMachine.p:limits".into(), class: class.to_string(),
                    property: "RotatingMachine.p".into(),
                    message: format!("Negated active power ({}) is outside of the range [Min:{}, Max:{}] of associated GeneratingUnit.", neg_p, min, max),
                    severity: "sh:Violation".into(), description: String::new(),
                });
            }
        }
    }
    v
}

fn check_rotating_machine_q_limits(dataset: &CimDataset) -> Vec<Violation> {
    let mut v = Vec::new();
    for mrid in dataset.by_type.get("SynchronousMachine").into_iter().flatten() {
        let entry = &dataset.entries[mrid];
        if let Some(sm) = Fields::of_class(entry, "SynchronousMachine") {
            if sm.bool("Equipment.inService") != Some(true) { continue; }
            if sm.has("SynchronousMachine.InitialReactiveCapabilityCurve") { continue; }
            let (Some(q), Some(min_q), Some(max_q)) =
                (sm.f64("RotatingMachine.q"), sm.f64("SynchronousMachine.minQ"), sm.f64("SynchronousMachine.maxQ")) else { continue };
            let neg_q = if q == 0.0 { 0.0 } else { -q };
            if neg_q < min_q || neg_q > max_q {
                v.push(Violation {
                    object_id: mrid.clone(), rule_id: "sshn456:RotatingMachine.q-limits".into(),
                    name: "C:456:SSH:RotatingMachine.q:limits".into(), class: "SynchronousMachine".into(),
                    property: "RotatingMachine.q".into(),
                    message: format!("Negated reactive power ({}) is outside of the range [Min:{}, Max:{}] (no ReactiveCapabilityCurve).", neg_q, min_q, max_q),
                    severity: "sh:Violation".into(), description: String::new(),
                });
            }
        }
    }
    v
}
