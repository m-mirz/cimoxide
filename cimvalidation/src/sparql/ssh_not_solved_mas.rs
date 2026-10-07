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
            let sections = lsc.f64("ShuntCompensator.sections").unwrap_or(0.0);
            let max_sections = lsc.i64("ShuntCompensator.maximumSections").unwrap_or(0) as f64;
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

fn check_nonlinear_shunt_compensator_sections_valid(dataset: &CimDataset) -> Vec<Violation> {
    let mut point_sections: HashMap<String, cimmodel::base::FastSet<i64>> = HashMap::default();
    for mrid in dataset.by_type.get("NonlinearShuntCompensatorPoint").into_iter().flatten() {
        let entry = &dataset.entries[mrid];
        if let Some(pt) = Fields::of_class(entry, "NonlinearShuntCompensatorPoint")
            && let Some(r) = &pt.reference("NonlinearShuntCompensatorPoint.NonlinearShuntCompensator") {
                let nsc_id = r.trim_start_matches('#').to_string();
                if let Some(sn) = pt.i64("NonlinearShuntCompensatorPoint.sectionNumber") {
                    point_sections.entry(nsc_id).or_default().insert(sn);
                }
            }
    }
    let mut v = Vec::new();
    for mrid in dataset.by_type.get("NonlinearShuntCompensator").into_iter().flatten() {
        let entry = &dataset.entries[mrid];
        if let Some(nsc) = Fields::of_class(entry, "NonlinearShuntCompensator") {
            let section = nsc.f64("ShuntCompensator.sections").unwrap_or(0.0);
            let is_integer = section == section.floor() && !section.is_nan();
            let valid = is_integer && point_sections.get(mrid).is_some_and(|s| s.contains(&(section as i64)));
            if !valid {
                v.push(Violation {
                    object_id: mrid.clone(), rule_id: "sshn301:ShuntCompensator.sections-valueNonLinear".into(),
                    name: "C:301:SSH:ShuntCompensator.sections:valueNonLinear".into(), class: "NonlinearShuntCompensator".into(),
                    property: "ShuntCompensator.sections".into(),
                    message: format!("The value ({}) does not equal one of the NonlinearShuntCompenstorPoint.sectionNumber.", section),
                    severity: "sh:Violation".into(), description: String::new(),
                });
            }
        }
    }
    v
}

fn check_shunt_compensator_sections_integer(dataset: &CimDataset) -> Vec<Violation> {
    let mut v = Vec::new();
    let check_sc = |mrid: &str, class: &str, sections: f64, rc_id: Option<&str>, v: &mut Vec<Violation>| {
        let rc_id = match rc_id { Some(id) => id, None => return };
        let rc_entry = match dataset.entries.get(rc_id) { Some(e) => e, None => return };
        let (enabled, discrete) = if let Some(rc) = Fields::of_class(rc_entry, "RegulatingControl") {
            (rc.bool("RegulatingControl.enabled").unwrap_or(false), rc.bool("RegulatingControl.discrete").unwrap_or(false))
        } else {
            return;
        };
        if enabled && discrete && sections != sections.floor() {
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
            check_sc(mrid, "LinearShuntCompensator", lsc.f64("ShuntCompensator.sections").unwrap_or(0.0), rc_id, &mut v);
        }
    }
    for mrid in dataset.by_type.get("NonlinearShuntCompensator").into_iter().flatten() {
        let entry = &dataset.entries[mrid];
        if let Some(nsc) = Fields::of_class(entry, "NonlinearShuntCompensator") {
            let rc_id = nsc.reference("RegulatingCondEq.RegulatingControl").map(|r| r.trim_start_matches('#'));
            check_sc(mrid, "NonlinearShuntCompensator", nsc.f64("ShuntCompensator.sections").unwrap_or(0.0), rc_id, &mut v);
        }
    }
    v
}

fn check_regulating_control_power_factor_required_attrs(dataset: &CimDataset) -> Vec<Violation> {
    let power_factor_uri = "RegulatingControlModeKind.powerFactor";
    let mut v = Vec::new();
    let check = |mrid: &str, class: &str, mode_uri: &str, min_val: f64, max_val: f64, v: &mut Vec<Violation>| {
        if mode_uri != power_factor_uri { return; }
        if min_val == 0.0 || max_val == 0.0 {
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
                check(mrid, "RegulatingControl", mode, rc.f64("RegulatingControl.minAllowedTargetValue").unwrap_or(0.0), rc.f64("RegulatingControl.maxAllowedTargetValue").unwrap_or(0.0), &mut v);
            }
    }
    for mrid in dataset.by_type.get("TapChangerControl").into_iter().flatten() {
        let entry = &dataset.entries[mrid];
        if let Some(tcc) = Fields::of_class(entry, "TapChangerControl")
            && let Some(mode) = &tcc.enumeration("RegulatingControl.mode") {
                check(mrid, "TapChangerControl", mode, tcc.f64("RegulatingControl.minAllowedTargetValue").unwrap_or(0.0), tcc.f64("RegulatingControl.maxAllowedTargetValue").unwrap_or(0.0), &mut v);
            }
    }
    v
}

fn check_tap_changer_step_integer(dataset: &CimDataset) -> Vec<Violation> {
    let mut tcc_discrete_enabled: HashMap<String, (bool, bool)> = HashMap::default();
    for mrid in dataset.by_type.get("TapChangerControl").into_iter().flatten() {
        let entry = &dataset.entries[mrid];
        if let Some(tcc) = Fields::of_class(entry, "TapChangerControl") {
            tcc_discrete_enabled.insert(mrid.clone(), (tcc.bool("RegulatingControl.discrete").unwrap_or(false), tcc.bool("RegulatingControl.enabled").unwrap_or(false)));
        }
    }
    let mut v = Vec::new();
    let report = |mrid: &str, class: &str, step: f64, tcc_mrid: Option<&str>, v: &mut Vec<Violation>| {
        let tcc_id = match tcc_mrid { Some(id) => id, None => return };
        let (discrete, enabled) = tcc_discrete_enabled.get(tcc_id).copied().unwrap_or((false, false));
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
            report(mrid, "RatioTapChanger", tc.f64("TapChanger.step").unwrap_or(0.0), tcc_id, &mut v);
        }
    }
    for mrid in dataset.by_type.get("PhaseTapChangerLinear").into_iter().flatten() {
        let entry = &dataset.entries[mrid];
        if let Some(tc) = Fields::of_class(entry, "PhaseTapChangerLinear") {
            let tcc_id = tc.reference("TapChanger.TapChangerControl").map(|r| r.trim_start_matches('#'));
            report(mrid, "PhaseTapChangerLinear", tc.f64("TapChanger.step").unwrap_or(0.0), tcc_id, &mut v);
        }
    }
    for mrid in dataset.by_type.get("PhaseTapChangerSymmetrical").into_iter().flatten() {
        let entry = &dataset.entries[mrid];
        if let Some(tc) = Fields::of_class(entry, "PhaseTapChangerSymmetrical") {
            let tcc_id = tc.reference("TapChanger.TapChangerControl").map(|r| r.trim_start_matches('#'));
            report(mrid, "PhaseTapChangerSymmetrical", tc.f64("TapChanger.step").unwrap_or(0.0), tcc_id, &mut v);
        }
    }
    for mrid in dataset.by_type.get("PhaseTapChangerAsymmetrical").into_iter().flatten() {
        let entry = &dataset.entries[mrid];
        if let Some(tc) = Fields::of_class(entry, "PhaseTapChangerAsymmetrical") {
            let tcc_id = tc.reference("TapChanger.TapChangerControl").map(|r| r.trim_start_matches('#'));
            report(mrid, "PhaseTapChangerAsymmetrical", tc.f64("TapChanger.step").unwrap_or(0.0), tcc_id, &mut v);
        }
    }
    for mrid in dataset.by_type.get("PhaseTapChangerTabular").into_iter().flatten() {
        let entry = &dataset.entries[mrid];
        if let Some(tc) = Fields::of_class(entry, "PhaseTapChangerTabular") {
            let tcc_id = tc.reference("TapChanger.TapChangerControl").map(|r| r.trim_start_matches('#'));
            report(mrid, "PhaseTapChangerTabular", tc.f64("TapChanger.step").unwrap_or(0.0), tcc_id, &mut v);
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
    let mut rc_discrete: HashMap<String, bool> = HashMap::default();
    for mrid in dataset.by_type.get("RegulatingControl").into_iter().flatten() {
        let entry = &dataset.entries[mrid];
        if let Some(rc) = Fields::of_class(entry, "RegulatingControl")
            && let Some(r) = &rc.reference("RegulatingControl.Terminal") {
                rc_discrete.insert(r.trim_start_matches('#').to_string(), rc.bool("RegulatingControl.discrete").unwrap_or(false));
            }
    }
    for mrid in dataset.by_type.get("TapChangerControl").into_iter().flatten() {
        let entry = &dataset.entries[mrid];
        if let Some(tcc) = Fields::of_class(entry, "TapChangerControl")
            && let Some(r) = &tcc.reference("RegulatingControl.Terminal") {
                rc_discrete.insert(r.trim_start_matches('#').to_string(), tcc.bool("RegulatingControl.discrete").unwrap_or(false));
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
            let value = if for_alpha { csc.f64("CsConverter.targetAlpha").unwrap_or(0.0) } else { csc.f64("CsConverter.targetGamma").unwrap_or(0.0) };
            if value == 0.0 { continue; }
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
            match rc_discrete.get(&pcc_term_id) {
                Some(true) | None => emit(&mut v, mrid),
                Some(false) => {}
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
            let net_interchange = ca.f64("ControlArea.netInterchange").unwrap_or(0.0);
            if !is_interchange || net_interchange == 0.0 { continue; }
            let mut sum = 0.0;
            for term_id in ca_terminals.get(mrid).into_iter().flatten() {
                let term = match dataset.entries.get(term_id).and_then(|e| Fields::of_class(e, "Terminal")) { Some(t) => t, None => continue };
                let cn_id = match term.reference("Terminal.ConnectivityNode") { Some(r) => r.trim_start_matches('#').to_string(), None => continue };
                if !cn_has_bp.contains(&cn_id) { continue; }
                let eq_id = match term.reference("Terminal.ConductingEquipment") { Some(r) => r.trim_start_matches('#').to_string(), None => continue };
                if let Some(ei) = dataset.entries.get(&eq_id).and_then(|e| Fields::of_class(e, "EquivalentInjection")) {
                    sum += ei.f64("EquivalentInjection.p").unwrap_or(0.0);
                }
            }
            if net_interchange != sum {
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
            if ei.bool("EquivalentInjection.regulationCapability").unwrap_or(false) {
                if !ei.bool("EquivalentInjection.regulationStatus").unwrap_or(false) || ei.f64("EquivalentInjection.regulationTarget").unwrap_or(0.0) == 0.0 {
                    v.push(Violation {
                        object_id: mrid.clone(), rule_id: "sshn456:EquivalentInjection-regulation".into(),
                        name: "C:456:SSH:EquivalentInjection:regulation".into(), class: "EquivalentInjection".into(),
                        property: "regulationStatus".into(),
                        message: "EquivalentInjection.regulationStatus and regulationTarget are required when regulationCapability is true.".into(),
                        severity: "sh:Violation".into(), description: String::new(),
                    });
                }
            } else if ei.bool("EquivalentInjection.regulationStatus").unwrap_or(false) || ei.f64("EquivalentInjection.regulationTarget").unwrap_or(0.0) != 0.0 {
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

fn check_rotating_machine_p_limits(dataset: &CimDataset) -> Vec<Violation> {
    let mut v = Vec::new();
    let check_rm = |mrid: &str, class: &str, p: f64, gu_id: Option<&str>, v: &mut Vec<Violation>| {
        let gu_id = match gu_id { Some(id) => id, None => return };
        let gu = match dataset.entries.get(gu_id).and_then(|e| Fields::of_class(e, "GeneratingUnit")) { Some(g) => g, None => return };
        let neg_p = if p == 0.0 { 0.0 } else { -p };
        let min = gu.f64("GeneratingUnit.minOperatingP").unwrap_or(0.0);
        let max = gu.f64("GeneratingUnit.maxOperatingP").unwrap_or(0.0);
        if neg_p < min || neg_p > max {
            v.push(Violation {
                object_id: mrid.to_string(), rule_id: "sshn456:RotatingMachine.p-limits".into(),
                name: "C:456:SSH:RotatingMachine.p:limits".into(), class: class.to_string(),
                property: "RotatingMachine.p".into(),
                message: format!("Negated active power ({}) is outside of the range [Min:{}, Max:{}] of associated GeneratingUnit.", neg_p, min, max),
                severity: "sh:Violation".into(), description: String::new(),
            });
        }
    };
    for mrid in dataset.by_type.get("SynchronousMachine").into_iter().flatten() {
        let entry = &dataset.entries[mrid];
        if let Some(sm) = Fields::of_class(entry, "SynchronousMachine") {
            let gu_id = sm.reference("RotatingMachine.GeneratingUnit").map(|r| r.trim_start_matches('#'));
            check_rm(mrid, "SynchronousMachine", sm.f64("RotatingMachine.p").unwrap_or(0.0), gu_id, &mut v);
        }
    }
    for mrid in dataset.by_type.get("AsynchronousMachine").into_iter().flatten() {
        let entry = &dataset.entries[mrid];
        if let Some(am) = Fields::of_class(entry, "AsynchronousMachine") {
            let gu_id = am.reference("RotatingMachine.GeneratingUnit").map(|r| r.trim_start_matches('#'));
            check_rm(mrid, "AsynchronousMachine", am.f64("RotatingMachine.p").unwrap_or(0.0), gu_id, &mut v);
        }
    }
    v
}

fn check_rotating_machine_q_limits(dataset: &CimDataset) -> Vec<Violation> {
    let mut v = Vec::new();
    for mrid in dataset.by_type.get("SynchronousMachine").into_iter().flatten() {
        let entry = &dataset.entries[mrid];
        if let Some(sm) = Fields::of_class(entry, "SynchronousMachine") {
            if !sm.bool("Equipment.inService").unwrap_or(false) { continue; }
            if sm.reference("SynchronousMachine.InitialReactiveCapabilityCurve").is_some() { continue; }
            let q = sm.f64("RotatingMachine.q").unwrap_or(0.0);
            let neg_q = if q == 0.0 { 0.0 } else { -q };
            let min_q = sm.f64("SynchronousMachine.minQ").unwrap_or(0.0);
            let max_q = sm.f64("SynchronousMachine.maxQ").unwrap_or(0.0);
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
