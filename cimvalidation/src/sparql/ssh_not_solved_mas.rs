use std::collections::HashMap;
use cimdecoder::CimDataset;
use crate::Violation;
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
        if let Some(lsc) = entry.element.as_any().downcast_ref::<cimstructs::LinearShuntCompensator>() {
            let sections = lsc.base.sections.unwrap_or(0.0);
            let max_sections = lsc.base.maximum_sections.unwrap_or(0) as f64;
            if sections < 0.0 || sections > max_sections {
                v.push(Violation {
                    object_id: mrid.clone(), rule_id: "sshcns.ShuntCompensator.sections-valueLinear".into(),
                    name: "ShuntCompensator.sections-valueLinear".into(), class: "LinearShuntCompensator".into(),
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
    let mut point_sections: HashMap<String, std::collections::HashSet<i64>> = HashMap::new();
    for mrid in dataset.by_type.get("NonlinearShuntCompensatorPoint").into_iter().flatten() {
        let entry = &dataset.entries[mrid];
        if let Some(pt) = entry.element.as_any().downcast_ref::<cimstructs::NonlinearShuntCompensatorPoint>() {
            if let Some(r) = &pt.nonlinear_shunt_compensator {
                let nsc_id = r.mrid.trim_start_matches('#').to_string();
                if let Some(sn) = pt.section_number {
                    point_sections.entry(nsc_id).or_default().insert(sn);
                }
            }
        }
    }
    let mut v = Vec::new();
    for mrid in dataset.by_type.get("NonlinearShuntCompensator").into_iter().flatten() {
        let entry = &dataset.entries[mrid];
        if let Some(nsc) = entry.element.as_any().downcast_ref::<cimstructs::NonlinearShuntCompensator>() {
            let section = nsc.base.sections.unwrap_or(0.0);
            let is_integer = section == section.floor() && !section.is_nan();
            let valid = is_integer && point_sections.get(mrid).map_or(false, |s| s.contains(&(section as i64)));
            if !valid {
                v.push(Violation {
                    object_id: mrid.clone(), rule_id: "sshcns.ShuntCompensator.sections-valueNonLinear".into(),
                    name: "ShuntCompensator.sections-valueNonLinear".into(), class: "NonlinearShuntCompensator".into(),
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
        let (enabled, discrete) = if let Some(rc) = rc_entry.element.as_any().downcast_ref::<cimstructs::RegulatingControl>() {
            (rc.enabled.unwrap_or(false), rc.discrete.unwrap_or(false))
        } else {
            return;
        };
        if enabled && discrete && sections != sections.floor() {
            v.push(Violation {
                object_id: mrid.to_string(), rule_id: "sshc456ns:ShuntCompensator.sections-value".into(),
                name: "ShuntCompensator.sections-value".into(), class: class.to_string(),
                property: "ShuntCompensator.sections".into(),
                message: format!("The value ({}) is not integer for an active discrete regulating control.", sections),
                severity: "sh:Violation".into(), description: String::new(),
            });
        }
    };
    for mrid in dataset.by_type.get("LinearShuntCompensator").into_iter().flatten() {
        let entry = &dataset.entries[mrid];
        if let Some(lsc) = entry.element.as_any().downcast_ref::<cimstructs::LinearShuntCompensator>() {
            let rc_id = lsc.base.base.regulating_control.as_ref().map(|r| r.mrid.trim_start_matches('#'));
            check_sc(mrid, "LinearShuntCompensator", lsc.base.sections.unwrap_or(0.0), rc_id, &mut v);
        }
    }
    for mrid in dataset.by_type.get("NonlinearShuntCompensator").into_iter().flatten() {
        let entry = &dataset.entries[mrid];
        if let Some(nsc) = entry.element.as_any().downcast_ref::<cimstructs::NonlinearShuntCompensator>() {
            let rc_id = nsc.base.base.regulating_control.as_ref().map(|r| r.mrid.trim_start_matches('#'));
            check_sc(mrid, "NonlinearShuntCompensator", nsc.base.sections.unwrap_or(0.0), rc_id, &mut v);
        }
    }
    v
}

fn check_regulating_control_power_factor_required_attrs(dataset: &CimDataset) -> Vec<Violation> {
    let power_factor_uri = "http://iec.ch/TC57/CIM100#RegulatingControlModeKind.powerFactor";
    let mut v = Vec::new();
    let check = |mrid: &str, class: &str, mode_uri: &str, min_val: f64, max_val: f64, v: &mut Vec<Violation>| {
        if mode_uri != power_factor_uri { return; }
        if min_val == 0.0 || max_val == 0.0 {
            v.push(Violation {
                object_id: mrid.to_string(), rule_id: "sshcns.RegulatingControl-requiredAttributes".into(),
                name: "RegulatingControl-requiredAttributes".into(), class: class.to_string(),
                property: "RegulatingControl.mode".into(),
                message: "Both minAllowedTargetValue and maxAllowedTargetValue are not provided for RegulatingControl in mode powerFactor.".into(),
                severity: "sh:Violation".into(), description: String::new(),
            });
        }
    };
    for mrid in dataset.by_type.get("RegulatingControl").into_iter().flatten() {
        let entry = &dataset.entries[mrid];
        if let Some(rc) = entry.element.as_any().downcast_ref::<cimstructs::RegulatingControl>() {
            if let Some(mode) = &rc.mode {
                check(mrid, "RegulatingControl", &mode.uri, rc.min_allowed_target_value.unwrap_or(0.0), rc.max_allowed_target_value.unwrap_or(0.0), &mut v);
            }
        }
    }
    for mrid in dataset.by_type.get("TapChangerControl").into_iter().flatten() {
        let entry = &dataset.entries[mrid];
        if let Some(tcc) = entry.element.as_any().downcast_ref::<cimstructs::TapChangerControl>() {
            if let Some(mode) = &tcc.base.mode {
                check(mrid, "TapChangerControl", &mode.uri, tcc.base.min_allowed_target_value.unwrap_or(0.0), tcc.base.max_allowed_target_value.unwrap_or(0.0), &mut v);
            }
        }
    }
    v
}

fn check_tap_changer_step_integer(dataset: &CimDataset) -> Vec<Violation> {
    let mut tcc_discrete: HashMap<String, bool> = HashMap::new();
    for mrid in dataset.by_type.get("TapChangerControl").into_iter().flatten() {
        let entry = &dataset.entries[mrid];
        if let Some(tcc) = entry.element.as_any().downcast_ref::<cimstructs::TapChangerControl>() {
            tcc_discrete.insert(mrid.clone(), tcc.base.discrete.unwrap_or(false));
        }
    }
    let mut v = Vec::new();
    let report = |mrid: &str, class: &str, step: f64, tcc_mrid: Option<&str>, v: &mut Vec<Violation>| {
        let tcc_id = match tcc_mrid { Some(id) => id, None => return };
        if !tcc_discrete.get(tcc_id).copied().unwrap_or(false) { return; }
        if step != step.floor() || step.is_nan() {
            v.push(Violation {
                object_id: mrid.to_string(), rule_id: "sshcns.TapChanger.step-valueType".into(),
                name: "TapChanger.step-valueType".into(), class: class.to_string(),
                property: "TapChanger.step".into(),
                message: format!("Non-integer value ({}) for a discrete TapChangerControl.", step),
                severity: "sh:Violation".into(), description: String::new(),
            });
        }
    };
    for mrid in dataset.by_type.get("RatioTapChanger").into_iter().flatten() {
        let entry = &dataset.entries[mrid];
        if let Some(tc) = entry.element.as_any().downcast_ref::<cimstructs::RatioTapChanger>() {
            let tcc_id = tc.base.tap_changer_control.as_ref().map(|r| r.mrid.trim_start_matches('#'));
            report(mrid, "RatioTapChanger", tc.base.step.unwrap_or(0.0), tcc_id, &mut v);
        }
    }
    for mrid in dataset.by_type.get("PhaseTapChangerLinear").into_iter().flatten() {
        let entry = &dataset.entries[mrid];
        if let Some(tc) = entry.element.as_any().downcast_ref::<cimstructs::PhaseTapChangerLinear>() {
            let tcc_id = tc.base.base.tap_changer_control.as_ref().map(|r| r.mrid.trim_start_matches('#'));
            report(mrid, "PhaseTapChangerLinear", tc.base.base.step.unwrap_or(0.0), tcc_id, &mut v);
        }
    }
    for mrid in dataset.by_type.get("PhaseTapChangerSymmetrical").into_iter().flatten() {
        let entry = &dataset.entries[mrid];
        if let Some(tc) = entry.element.as_any().downcast_ref::<cimstructs::PhaseTapChangerSymmetrical>() {
            let tcc_id = tc.base.base.base.tap_changer_control.as_ref().map(|r| r.mrid.trim_start_matches('#'));
            report(mrid, "PhaseTapChangerSymmetrical", tc.base.base.base.step.unwrap_or(0.0), tcc_id, &mut v);
        }
    }
    for mrid in dataset.by_type.get("PhaseTapChangerAsymmetrical").into_iter().flatten() {
        let entry = &dataset.entries[mrid];
        if let Some(tc) = entry.element.as_any().downcast_ref::<cimstructs::PhaseTapChangerAsymmetrical>() {
            let tcc_id = tc.base.base.base.tap_changer_control.as_ref().map(|r| r.mrid.trim_start_matches('#'));
            report(mrid, "PhaseTapChangerAsymmetrical", tc.base.base.base.step.unwrap_or(0.0), tcc_id, &mut v);
        }
    }
    for mrid in dataset.by_type.get("PhaseTapChangerTabular").into_iter().flatten() {
        let entry = &dataset.entries[mrid];
        if let Some(tc) = entry.element.as_any().downcast_ref::<cimstructs::PhaseTapChangerTabular>() {
            let tcc_id = tc.base.base.tap_changer_control.as_ref().map(|r| r.mrid.trim_start_matches('#'));
            report(mrid, "PhaseTapChangerTabular", tc.base.base.step.unwrap_or(0.0), tcc_id, &mut v);
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
    let inverter   = "http://iec.ch/TC57/CIM100#CsOperatingModeKind.inverter";
    let rectifier  = "http://iec.ch/TC57/CIM100#CsOperatingModeKind.rectifier";
    // terminalID → RC.discrete
    let mut rc_discrete: HashMap<String, bool> = HashMap::new();
    for mrid in dataset.by_type.get("RegulatingControl").into_iter().flatten() {
        let entry = &dataset.entries[mrid];
        if let Some(rc) = entry.element.as_any().downcast_ref::<cimstructs::RegulatingControl>() {
            if let Some(r) = &rc.terminal {
                rc_discrete.insert(r.mrid.trim_start_matches('#').to_string(), rc.discrete.unwrap_or(false));
            }
        }
    }
    for mrid in dataset.by_type.get("TapChangerControl").into_iter().flatten() {
        let entry = &dataset.entries[mrid];
        if let Some(tcc) = entry.element.as_any().downcast_ref::<cimstructs::TapChangerControl>() {
            if let Some(r) = &tcc.base.terminal {
                rc_discrete.insert(r.mrid.trim_start_matches('#').to_string(), tcc.base.discrete.unwrap_or(false));
            }
        }
    }
    let (rule_id, rule_name, prop, msg) = if for_alpha {
        ("sshn301.CsConverter.targetAlpha-applicability", "CsConverter.targetAlpha-applicability", "CsConverter.targetAlpha",
         "CsConverter.targetAlpha is provided for an inverter or discrete tap changer control is used or RegulatingControl is not provided.")
    } else {
        ("sshn301.CsConverter.targetGamma-applicability", "CsConverter.targetGamma-applicability", "CsConverter.targetGamma",
         "CsConverter.targetGamma is provided for a rectifier or discrete tap changer control is used or RegulatingControl is not provided.")
    };
    let mut v = Vec::new();
    for mrid in dataset.by_type.get("CsConverter").into_iter().flatten() {
        let entry = &dataset.entries[mrid];
        if let Some(csc) = entry.element.as_any().downcast_ref::<cimstructs::CsConverter>() {
            let value = if for_alpha { csc.target_alpha.unwrap_or(0.0) } else { csc.target_gamma.unwrap_or(0.0) };
            if value == 0.0 { continue; }
            let mode = match &csc.operating_mode { Some(r) => r.uri.as_str(), None => continue };
            let invalid_mode = if for_alpha { inverter } else { rectifier };
            let emit = |v: &mut Vec<Violation>, mrid: &str| {
                v.push(Violation {
                    object_id: mrid.to_string(), rule_id: rule_id.into(), name: rule_name.into(),
                    class: "CsConverter".into(), property: prop.into(), message: msg.into(),
                    severity: "sh:Violation".into(), description: String::new(),
                });
            };
            if mode == invalid_mode { emit(&mut v, mrid); continue; }
            let pcc_term_id = match &csc.base.pcc_terminal { Some(r) => r.mrid.trim_start_matches('#').to_string(), None => { emit(&mut v, mrid); continue; } };
            let pcc_term = match dataset.entries.get(&pcc_term_id).and_then(|e| e.element.as_any().downcast_ref::<cimstructs::Terminal>()) {
                Some(t) => t, None => { emit(&mut v, mrid); continue; }
            };
            let eq_id = match &pcc_term.conducting_equipment { Some(r) => r.mrid.trim_start_matches('#').to_string(), None => { emit(&mut v, mrid); continue; } };
            let is_pt = dataset.entries.get(&eq_id).map_or(false, |e| e.element.as_any().downcast_ref::<cimstructs::PowerTransformer>().is_some());
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
    let interchange_uri = "http://iec.ch/TC57/CIM100#ControlAreaTypeKind.Interchange";
    let mut cn_has_bp: std::collections::HashSet<String> = std::collections::HashSet::new();
    for mrid in dataset.by_type.get("BoundaryPoint").into_iter().flatten() {
        let entry = &dataset.entries[mrid];
        if let Some(bp) = entry.element.as_any().downcast_ref::<cimstructs::BoundaryPoint>() {
            if let Some(r) = &bp.connectivity_node {
                cn_has_bp.insert(r.mrid.trim_start_matches('#').to_string());
            }
        }
    }
    let mut ca_terminals: HashMap<String, Vec<String>> = HashMap::new();
    for mrid in dataset.by_type.get("TieFlow").into_iter().flatten() {
        let entry = &dataset.entries[mrid];
        if let Some(tf) = entry.element.as_any().downcast_ref::<cimstructs::TieFlow>() {
            if let (Some(ca), Some(term)) = (&tf.control_area, &tf.terminal) {
                let ca_id = ca.mrid.trim_start_matches('#').to_string();
                let term_id = term.mrid.trim_start_matches('#').to_string();
                ca_terminals.entry(ca_id).or_default().push(term_id);
            }
        }
    }
    let mut v = Vec::new();
    for mrid in dataset.by_type.get("ControlArea").into_iter().flatten() {
        let entry = &dataset.entries[mrid];
        if let Some(ca) = entry.element.as_any().downcast_ref::<cimstructs::ControlArea>() {
            let is_interchange = ca.type_.as_ref().map_or(false, |r| r.uri == interchange_uri);
            let net_interchange = ca.net_interchange.unwrap_or(0.0);
            if !is_interchange || net_interchange == 0.0 { continue; }
            let mut sum = 0.0;
            for term_id in ca_terminals.get(mrid).into_iter().flatten() {
                let term = match dataset.entries.get(term_id).and_then(|e| e.element.as_any().downcast_ref::<cimstructs::Terminal>()) { Some(t) => t, None => continue };
                let cn_id = match &term.connectivity_node { Some(r) => r.mrid.trim_start_matches('#').to_string(), None => continue };
                if !cn_has_bp.contains(&cn_id) { continue; }
                let eq_id = match &term.conducting_equipment { Some(r) => r.mrid.trim_start_matches('#').to_string(), None => continue };
                if let Some(ei) = dataset.entries.get(&eq_id).and_then(|e| e.element.as_any().downcast_ref::<cimstructs::EquivalentInjection>()) {
                    sum += ei.p.unwrap_or(0.0);
                }
            }
            if net_interchange != sum {
                v.push(Violation {
                    object_id: mrid.clone(), rule_id: "sshn301.ControlArea-netInterchangeCalculation".into(),
                    name: "ControlArea-netInterchangeCalculation".into(), class: "ControlArea".into(),
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
        if let Some(ei) = entry.element.as_any().downcast_ref::<cimstructs::EquivalentInjection>() {
            if ei.regulation_capability.unwrap_or(false) {
                if !ei.regulation_status.unwrap_or(false) || ei.regulation_target.unwrap_or(0.0) == 0.0 {
                    v.push(Violation {
                        object_id: mrid.clone(), rule_id: "sshn456:EquivalentInjection-regulation".into(),
                        name: "EquivalentInjection-regulation".into(), class: "EquivalentInjection".into(),
                        property: "regulationStatus".into(),
                        message: "EquivalentInjection.regulationStatus and regulationTarget are required when regulationCapability is true.".into(),
                        severity: "sh:Violation".into(), description: String::new(),
                    });
                }
            } else if ei.regulation_status.unwrap_or(false) || ei.regulation_target.unwrap_or(0.0) != 0.0 {
                v.push(Violation {
                    object_id: mrid.clone(), rule_id: "sshn456:EquivalentInjection-regulation".into(),
                    name: "EquivalentInjection-regulation".into(), class: "EquivalentInjection".into(),
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
        let gu = match dataset.entries.get(gu_id).and_then(|e| e.element.as_any().downcast_ref::<cimstructs::GeneratingUnit>()) { Some(g) => g, None => return };
        let neg_p = if p == 0.0 { 0.0 } else { -p };
        let min = gu.min_operating_p.unwrap_or(0.0);
        let max = gu.max_operating_p.unwrap_or(0.0);
        if neg_p < min || neg_p > max {
            v.push(Violation {
                object_id: mrid.to_string(), rule_id: "sshn456:RotatingMachine.p-limits".into(),
                name: "RotatingMachine.p-limits".into(), class: class.to_string(),
                property: "RotatingMachine.p".into(),
                message: format!("Negated active power ({}) is outside of the range [Min:{}, Max:{}] of associated GeneratingUnit.", neg_p, min, max),
                severity: "sh:Violation".into(), description: String::new(),
            });
        }
    };
    for mrid in dataset.by_type.get("SynchronousMachine").into_iter().flatten() {
        let entry = &dataset.entries[mrid];
        if let Some(sm) = entry.element.as_any().downcast_ref::<cimstructs::SynchronousMachine>() {
            let gu_id = sm.base.generating_unit.as_ref().map(|r| r.mrid.trim_start_matches('#'));
            check_rm(mrid, "SynchronousMachine", sm.base.p.unwrap_or(0.0), gu_id, &mut v);
        }
    }
    for mrid in dataset.by_type.get("AsynchronousMachine").into_iter().flatten() {
        let entry = &dataset.entries[mrid];
        if let Some(am) = entry.element.as_any().downcast_ref::<cimstructs::AsynchronousMachine>() {
            let gu_id = am.base.generating_unit.as_ref().map(|r| r.mrid.trim_start_matches('#'));
            check_rm(mrid, "AsynchronousMachine", am.base.p.unwrap_or(0.0), gu_id, &mut v);
        }
    }
    v
}

fn check_rotating_machine_q_limits(dataset: &CimDataset) -> Vec<Violation> {
    let mut v = Vec::new();
    for mrid in dataset.by_type.get("SynchronousMachine").into_iter().flatten() {
        let entry = &dataset.entries[mrid];
        if let Some(sm) = entry.element.as_any().downcast_ref::<cimstructs::SynchronousMachine>() {
            if !sm.base.base.base.base.base.in_service.unwrap_or(false) { continue; }
            if sm.initial_reactive_capability_curve.is_some() { continue; }
            let q = sm.base.q.unwrap_or(0.0);
            let neg_q = if q == 0.0 { 0.0 } else { -q };
            let min_q = sm.min_q.unwrap_or(0.0);
            let max_q = sm.max_q.unwrap_or(0.0);
            if neg_q < min_q || neg_q > max_q {
                v.push(Violation {
                    object_id: mrid.clone(), rule_id: "sshn456:RotatingMachine.q-limits".into(),
                    name: "RotatingMachine.q-limits".into(), class: "SynchronousMachine".into(),
                    property: "RotatingMachine.q".into(),
                    message: format!("Negated reactive power ({}) is outside of the range [Min:{}, Max:{}] (no ReactiveCapabilityCurve).", neg_q, min_q, max_q),
                    severity: "sh:Violation".into(), description: String::new(),
                });
            }
        }
    }
    v
}
