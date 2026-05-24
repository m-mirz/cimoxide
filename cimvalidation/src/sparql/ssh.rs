use cimdecoder::CimDataset;
use crate::Violation;

pub fn validate(dataset: &CimDataset) -> Vec<Violation> {
    let mut v = Vec::new();
    v.extend(check_energy_source_active_power_consumer(dataset));
    v.extend(check_regulating_control_target_deadband_applicability(dataset));
    v.extend(check_cs_converter_value_range(dataset));
    v.extend(check_cs_converter_p_pcc_control(dataset));
    v.extend(check_vs_converter_p_pcc_control(dataset));
    v.extend(check_vs_converter_q_pcc_control(dataset));
    v.extend(check_energy_source_pq(dataset));
    v.extend(check_synchronous_machine_operating_mode_match(dataset));
    v.extend(check_generating_unit_single_active_power_slack(dataset));
    v.extend(check_external_network_injection_limits(dataset));
    v.extend(check_equivalent_injection_limits(dataset));
    v.extend(check_rotating_machine_curve_limits(dataset));
    v.extend(check_regulating_control_target_value_positive(dataset));
    v
}

fn check_energy_source_active_power_consumer(dataset: &CimDataset) -> Vec<Violation> {
    let mut v = Vec::new();
    for mrid in dataset.by_type.get("EnergySource").into_iter().flatten() {
        let entry = &dataset.entries[mrid];
        if let Some(es) = entry.element.as_any().downcast_ref::<cimstructs::EnergySource>() {
            if es.active_power.unwrap_or(0.0) > 0.0 {
                v.push(Violation {
                    object_id:   mrid.clone(),
                    rule_id:     "sshc.EnergySource.activePower-consumer".into(),
                    name:        "EnergySource.activePower-consumer".into(),
                    class:       "EnergySource".into(),
                    property:    "EnergySource.activePower".into(),
                    message:     "EnergySource that is a consumer (activePower > 0).".into(),
                    severity:    "sh:Warning".into(),
                    description: String::new(),
                });
            }
        }
    }
    v
}

fn check_regulating_control_target_deadband_applicability(dataset: &CimDataset) -> Vec<Violation> {
    let mut v = Vec::new();
    let check = |mrid: &str, class: &str, deadband: f64, discrete: bool| -> Option<Violation> {
        if (deadband != 0.0 && !discrete) || (deadband == 0.0 && discrete) {
            Some(Violation {
                object_id:   mrid.to_string(),
                rule_id:     "sshc.RegulatingControl.targetDeadband-applicability".into(),
                name:        "RegulatingControl.targetDeadband-applicability".into(),
                class:       class.to_string(),
                property:    "RegulatingControl.discrete".into(),
                message:     "Either RegulatingControl.targetDeadband is provided for a continuous control or it is not provided for a discrete control.".into(),
                severity:    "sh:Violation".into(),
                description: String::new(),
            })
        } else {
            None
        }
    };
    for mrid in dataset.by_type.get("RegulatingControl").into_iter().flatten() {
        let entry = &dataset.entries[mrid];
        if let Some(rc) = entry.element.as_any().downcast_ref::<cimstructs::RegulatingControl>() {
            if let Some(viol) = check(mrid, "RegulatingControl", rc.target_deadband.unwrap_or(0.0), rc.discrete.unwrap_or(false)) {
                v.push(viol);
            }
        }
    }
    for mrid in dataset.by_type.get("TapChangerControl").into_iter().flatten() {
        let entry = &dataset.entries[mrid];
        if let Some(tcc) = entry.element.as_any().downcast_ref::<cimstructs::TapChangerControl>() {
            if let Some(viol) = check(mrid, "TapChangerControl", tcc.base.target_deadband.unwrap_or(0.0), tcc.base.discrete.unwrap_or(false)) {
                v.push(viol);
            }
        }
    }
    v
}

fn check_cs_converter_value_range(dataset: &CimDataset) -> Vec<Violation> {
    let rectifier = "http://iec.ch/TC57/CIM100#CsOperatingModeKind.rectifier";
    let inverter  = "http://iec.ch/TC57/CIM100#CsOperatingModeKind.inverter";
    let mut v = Vec::new();
    for mrid in dataset.by_type.get("CsConverter").into_iter().flatten() {
        let entry = &dataset.entries[mrid];
        if let Some(csc) = entry.element.as_any().downcast_ref::<cimstructs::CsConverter>() {
            let mode = match &csc.operating_mode { Some(r) => r.uri.as_str(), None => continue };
            if mode == rectifier {
                if csc.max_alpha.unwrap_or(0.0) > 18.0 {
                    v.push(Violation {
                        object_id: mrid.clone(), rule_id: "sshc.CsConverter.maxAlpha-valueRangeTypical".into(),
                        name: "CsConverter.maxAlpha-valueRangeTypical".into(), class: "CsConverter".into(),
                        property: "CsConverter.maxAlpha".into(), message: "The maxAlpha value is greater than 18 for a rectifier.".into(),
                        severity: "sh:Warning".into(), description: String::new(),
                    });
                }
                let min_a = csc.min_alpha.unwrap_or(0.0);
                let max_a = csc.max_alpha.unwrap_or(0.0);
                if min_a < 10.0 || min_a > max_a {
                    v.push(Violation {
                        object_id: mrid.clone(), rule_id: "sshc.CsConverter.minAlpha-valueRangeTypical".into(),
                        name: "CsConverter.minAlpha-valueRangeTypical".into(), class: "CsConverter".into(),
                        property: "CsConverter.minAlpha".into(), message: "The minAlpha value is less than 10 or greater than CsConverter.maxAlpha for a rectifier.".into(),
                        severity: "sh:Warning".into(), description: String::new(),
                    });
                }
            } else if mode == inverter {
                if csc.max_gamma.unwrap_or(0.0) > 20.0 {
                    v.push(Violation {
                        object_id: mrid.clone(), rule_id: "sshc.CsConverter.maxGamma-valueRangeTypical".into(),
                        name: "CsConverter.maxGamma-valueRangeTypical".into(), class: "CsConverter".into(),
                        property: "CsConverter.maxGamma".into(), message: "The maxGamma value is greater than 20 for an inverter.".into(),
                        severity: "sh:Warning".into(), description: String::new(),
                    });
                }
                let min_g = csc.min_gamma.unwrap_or(0.0);
                let max_g = csc.max_gamma.unwrap_or(0.0);
                if min_g < 17.0 || min_g > max_g {
                    v.push(Violation {
                        object_id: mrid.clone(), rule_id: "sshc.CsConverter.minGamma-valueRangeTypical".into(),
                        name: "CsConverter.minGamma-valueRangeTypical".into(), class: "CsConverter".into(),
                        property: "CsConverter.minGamma".into(), message: "The minGamma value is less than 17 or greater than CsConverter.maxGamma for an inverter.".into(),
                        severity: "sh:Warning".into(), description: String::new(),
                    });
                }
            }
        }
    }
    v
}

fn check_cs_converter_p_pcc_control(dataset: &CimDataset) -> Vec<Violation> {
    let dc_current   = "http://iec.ch/TC57/CIM100#CsPpccControlKind.dcCurrent";
    let dc_voltage   = "http://iec.ch/TC57/CIM100#CsPpccControlKind.dcVoltage";
    let active_power = "http://iec.ch/TC57/CIM100#CsPpccControlKind.activePower";
    let mut v = Vec::new();
    for mrid in dataset.by_type.get("CsConverter").into_iter().flatten() {
        let entry = &dataset.entries[mrid];
        if let Some(csc) = entry.element.as_any().downcast_ref::<cimstructs::CsConverter>() {
            let ctrl = match &csc.p_pcc_control { Some(r) => r.uri.as_str(), None => continue };
            if ctrl == dc_current && csc.target_idc.unwrap_or(0.0) == 0.0 {
                v.push(Violation { object_id: mrid.clone(), rule_id: "sshc.CsConverter.pPccControl-targetValueIdc".into(),
                    name: "CsConverter.pPccControl-targetValueIdc".into(), class: "CsConverter".into(),
                    property: "CsConverter.pPccControl".into(),
                    message: "CsConverter.targetIdc is not provided for a converter with CsPpccControlKind.dcCurrent.".into(),
                    severity: "sh:Violation".into(), description: String::new() });
            } else if ctrl == dc_voltage && csc.base.target_udc.unwrap_or(0.0) == 0.0 {
                v.push(Violation { object_id: mrid.clone(), rule_id: "sshc.CsConverter.pPccControl-targetValueUdc".into(),
                    name: "CsConverter.pPccControl-targetValueUdc".into(), class: "CsConverter".into(),
                    property: "CsConverter.pPccControl".into(),
                    message: "ACDCConverter.targetUdc is not provided for a converter with CsPpccControlKind.dcVoltage.".into(),
                    severity: "sh:Violation".into(), description: String::new() });
            } else if ctrl == active_power && csc.base.target_ppcc.unwrap_or(0.0) == 0.0 {
                v.push(Violation { object_id: mrid.clone(), rule_id: "sshc.CsConverter.pPccControl-targetValuePpcc".into(),
                    name: "CsConverter.pPccControl-targetValuePpcc".into(), class: "CsConverter".into(),
                    property: "CsConverter.pPccControl".into(),
                    message: "ACDCConverter.targetPpcc is not provided for a converter with CsPpccControlKind.activePower.".into(),
                    severity: "sh:Violation".into(), description: String::new() });
            }
        }
    }
    v
}

fn check_vs_converter_p_pcc_control(dataset: &CimDataset) -> Vec<Violation> {
    let prefix = "http://iec.ch/TC57/CIM100#VsPpccControlKind.";
    let mut v = Vec::new();
    for mrid in dataset.by_type.get("VsConverter").into_iter().flatten() {
        let entry = &dataset.entries[mrid];
        if let Some(vsc) = entry.element.as_any().downcast_ref::<cimstructs::VsConverter>() {
            let ctrl = match &vsc.p_pcc_control { Some(r) => r.uri.as_str(), None => continue };
            let ppcc  = vsc.base.target_ppcc.unwrap_or(0.0);
            let udc   = vsc.base.target_udc.unwrap_or(0.0);
            let droop = vsc.droop.unwrap_or(0.0);
            let droopcomp = vsc.droop_compensation.unwrap_or(0.0);
            let phase_pcc = vsc.target_phase_pcc.unwrap_or(0.0);
            let msg_opt: Option<&str> = if ctrl == &format!("{prefix}pPccAndUdcDroop") {
                if ppcc == 0.0 || udc == 0.0 || droop == 0.0 { Some("One or all among ACDCConverter.targetPpcc, ACDCConverter.targetUdc and VsConverter.droop are not provided for VsPpccControlKind.pPccAndUdcDroop.") } else { None }
            } else if ctrl == &format!("{prefix}pPccAndUdcDroopWithCompensation") {
                if ppcc == 0.0 || udc == 0.0 || droop == 0.0 || droopcomp == 0.0 { Some("One or all among ACDCConverter.targetPpcc, ACDCConverter.targetUdc, VsConverter.droop and VsConverter.droopCompensation are not provided for VsPpccControlKind.pPccAndUdcDroopWithCompensation.") } else { None }
            } else if ctrl == &format!("{prefix}pPccAndUdcDroopPilot") {
                if ppcc == 0.0 || udc == 0.0 || droop == 0.0 { Some("One or all among ACDCConverter.targetPpcc, ACDCConverter.targetUdc and VsConverter.droop are not provided for VsPpccControlKind.pPccAndUdcDroopPilot.") } else { None }
            } else if ctrl == &format!("{prefix}udc") {
                if udc == 0.0 { Some("ACDCConverter.targetUdc is not provided for VsPpccControlKind.udc.") } else { None }
            } else if ctrl == &format!("{prefix}pPcc") {
                if ppcc == 0.0 { Some("ACDCConverter.targetPpcc is not provided for VsPpccControlKind.pPcc.") } else { None }
            } else if ctrl == &format!("{prefix}phasePcc") {
                if phase_pcc == 0.0 { Some("VsConverter.targetPhasePcc is not provided for VsPpccControlKind.phasePcc.") } else { None }
            } else {
                None
            };
            if let Some(msg) = msg_opt {
                v.push(Violation { object_id: mrid.clone(), rule_id: "sshc.VsConverter.pPccControl rules".into(),
                    name: "VsConverter.pPccControl rules".into(), class: "VsConverter".into(),
                    property: "VsConverter.pPccControl".into(), message: msg.into(),
                    severity: "sh:Violation".into(), description: String::new() });
            }
        }
    }
    v
}

fn check_vs_converter_q_pcc_control(dataset: &CimDataset) -> Vec<Violation> {
    let prefix = "http://iec.ch/TC57/CIM100#VsQpccControlKind.";
    let mut v = Vec::new();
    for mrid in dataset.by_type.get("VsConverter").into_iter().flatten() {
        let entry = &dataset.entries[mrid];
        if let Some(vsc) = entry.element.as_any().downcast_ref::<cimstructs::VsConverter>() {
            let ctrl = match &vsc.q_pcc_control { Some(r) => r.uri.as_str(), None => continue };
            let pf  = vsc.target_power_factor_pcc.unwrap_or(0.0);
            let pwm = vsc.target_pw_mfactor.unwrap_or(0.0);
            let phase_pcc = vsc.target_phase_pcc.unwrap_or(0.0);
            let qpcc = vsc.target_qpcc.unwrap_or(0.0);
            let upcc = vsc.target_upcc.unwrap_or(0.0);
            let msg_opt: Option<&str> = if ctrl == &format!("{prefix}powerFactorPcc") {
                if pf == 0.0 { Some("VsConverter.targetPowerFactorPcc is not provided for VsQpccControlKind.powerFactorPcc.") } else { None }
            } else if ctrl == &format!("{prefix}pulseWidthModulation") {
                if pwm == 0.0 || phase_pcc == 0.0 { Some("VsConverter.targetPWMfactor and/or VsConverter.targetPhasePcc are not provided for VsQpccControlKind.pulseWidthModulation.") } else { None }
            } else if ctrl == &format!("{prefix}reactivePcc") {
                if qpcc == 0.0 { Some("VsConverter.targetQpcc is not provided for VsQpccControlKind.reactivePcc.") } else { None }
            } else if ctrl == &format!("{prefix}voltagePcc") {
                if upcc == 0.0 { Some("VsConverter.targetUpcc is not provided for VsQpccControlKind.voltagePcc.") } else { None }
            } else {
                None
            };
            if let Some(msg) = msg_opt {
                v.push(Violation { object_id: mrid.clone(), rule_id: "sshc.VsConverter.qPccControl rules".into(),
                    name: "VsConverter.qPccControl rules".into(), class: "VsConverter".into(),
                    property: "VsConverter.qPccControl".into(), message: msg.into(),
                    severity: "sh:Violation".into(), description: String::new() });
            }
        }
    }
    v
}

fn check_energy_source_pq(dataset: &CimDataset) -> Vec<Violation> {
    let mut v = Vec::new();
    for mrid in dataset.by_type.get("EnergySource").into_iter().flatten() {
        let entry = &dataset.entries[mrid];
        if let Some(es) = entry.element.as_any().downcast_ref::<cimstructs::EnergySource>() {
            if es.voltage_angle.unwrap_or(0.0) != 0.0 || es.voltage_magnitude.unwrap_or(0.0) != 0.0 {
                v.push(Violation {
                    object_id: mrid.clone(), rule_id: "sshc456:EnergySource-EnergySourcePQ".into(),
                    name: "EnergySource-EnergySourcePQ".into(), class: "EnergySource".into(),
                    property: "EnergySource.voltageAngle".into(),
                    message: "EnergySource modelled as voltage source (attributes voltageAngle and voltageMagnitude are used). Please assess depending on the use case.".into(),
                    severity: "sh:Warning".into(), description: String::new(),
                });
            }
        }
    }
    v
}

pub(super) fn check_synchronous_machine_operating_mode_match(dataset: &CimDataset) -> Vec<Violation> {
    let mut v = Vec::new();
    for mrid in dataset.by_type.get("SynchronousMachine").into_iter().flatten() {
        let entry = &dataset.entries[mrid];
        if let Some(sm) = entry.element.as_any().downcast_ref::<cimstructs::SynchronousMachine>() {
            let mode = match &sm.operating_mode { Some(r) => r.uri.as_str(), None => continue };
            let kind = match &sm.type_ { Some(r) => r.uri.as_str(), None => continue };
            let valid = if mode.ends_with("motor") {
                kind.ends_with("motor") || kind.ends_with("generatorOrMotor") || kind.ends_with("motorOrCondenser") || kind.ends_with("generatorOrCondenserOrMotor")
            } else if mode.ends_with("condenser") {
                kind.ends_with("condenser") || kind.ends_with("generatorOrCondenser") || kind.ends_with("motorOrCondenser") || kind.ends_with("generatorOrCondenserOrMotor")
            } else if mode.ends_with("generator") {
                kind.ends_with("generator") || kind.ends_with("generatorOrMotor") || kind.ends_with("generatorOrCondenser") || kind.ends_with("generatorOrCondenserOrMotor")
            } else {
                false
            };
            if !valid {
                v.push(Violation {
                    object_id: mrid.clone(), rule_id: "sshn456:SynchronousMachine.operatingMode-matchType".into(),
                    name: "SynchronousMachine.operatingMode-matchType".into(), class: "SynchronousMachine".into(),
                    property: "SynchronousMachine.operatingMode".into(),
                    message: format!("SynchronousMachine.operatingMode ({}) is not consistent with SynchronousMachine.type ({}).", mode, kind),
                    severity: "sh:Violation".into(), description: String::new(),
                });
            }
        }
    }
    v
}

pub(super) fn check_generating_unit_single_active_power_slack(dataset: &CimDataset) -> Vec<Violation> {
    use std::collections::HashMap;
    let mut ca_slacks: HashMap<String, Vec<String>> = HashMap::new();
    for mrid in dataset.by_type.get("ControlAreaGeneratingUnit").into_iter().flatten() {
        let entry = &dataset.entries[mrid];
        if let Some(cagu) = entry.element.as_any().downcast_ref::<cimstructs::ControlAreaGeneratingUnit>() {
            let ca_id = match &cagu.control_area { Some(r) => r.mrid.trim_start_matches('#').to_string(), None => continue };
            let gu_id = match &cagu.generating_unit { Some(r) => r.mrid.trim_start_matches('#').to_string(), None => continue };
            let gu_entry = match dataset.entries.get(&gu_id) { Some(e) => e, None => continue };
            if let Some(gu) = gu_entry.element.as_any().downcast_ref::<cimstructs::GeneratingUnit>() {
                if gu.normal_pf.unwrap_or(0.0) > 0.0 {
                    ca_slacks.entry(ca_id).or_default().push(gu_id);
                }
            }
        }
    }
    let mut v = Vec::new();
    for (ca_id, slacks) in &ca_slacks {
        if slacks.len() > 1 {
            v.push(Violation {
                object_id: ca_id.clone(), rule_id: "sshn456:GeneratingUnit-singleActivePowerSlack".into(),
                name: "GeneratingUnit-singleActivePowerSlack".into(), class: "ControlArea".into(),
                property: "rdf:type".into(),
                message: format!("Multiple generating units ({}) in ControlArea {} have non-zero normalPF.", slacks.join(", "), ca_id),
                severity: "sh:Violation".into(), description: String::new(),
            });
        }
    }
    v
}

pub(super) fn check_external_network_injection_limits(dataset: &CimDataset) -> Vec<Violation> {
    let mut v = Vec::new();
    for mrid in dataset.by_type.get("ExternalNetworkInjection").into_iter().flatten() {
        let entry = &dataset.entries[mrid];
        if let Some(eni) = entry.element.as_any().downcast_ref::<cimstructs::ExternalNetworkInjection>() {
            if !eni.base.base.base.base.in_service.unwrap_or(false) { continue; }
            let p = eni.p.unwrap_or(0.0);
            let neg_p = if p == 0.0 { 0.0 } else { -p };
            let min_p = eni.min_p.unwrap_or(0.0);
            let max_p = eni.max_p.unwrap_or(0.0);
            if neg_p < min_p || neg_p > max_p {
                v.push(Violation {
                    object_id: mrid.clone(), rule_id: "sshn456:ExternalNetworkInjection.p-limits".into(),
                    name: "ExternalNetworkInjection.p-limits".into(), class: "ExternalNetworkInjection".into(),
                    property: "p".into(),
                    message: format!("Negated active power ({}) is outside of the range [Min:{}, Max:{}].", neg_p, min_p, max_p),
                    severity: "sh:Violation".into(), description: String::new(),
                });
            }
            let q = eni.q.unwrap_or(0.0);
            let neg_q = if q == 0.0 { 0.0 } else { -q };
            let min_q = eni.min_q.unwrap_or(0.0);
            let max_q = eni.max_q.unwrap_or(0.0);
            if neg_q < min_q || neg_q > max_q {
                v.push(Violation {
                    object_id: mrid.clone(), rule_id: "sshn456:ExternalNetworkInjection.q-limits".into(),
                    name: "ExternalNetworkInjection.q-limits".into(), class: "ExternalNetworkInjection".into(),
                    property: "q".into(),
                    message: format!("Negated reactive power ({}) is outside of the range [Min:{}, Max:{}].", neg_q, min_q, max_q),
                    severity: "sh:Violation".into(), description: String::new(),
                });
            }
        }
    }
    v
}

pub(super) fn check_equivalent_injection_limits(dataset: &CimDataset) -> Vec<Violation> {
    let mut v = Vec::new();
    for mrid in dataset.by_type.get("EquivalentInjection").into_iter().flatten() {
        let entry = &dataset.entries[mrid];
        if let Some(ei) = entry.element.as_any().downcast_ref::<cimstructs::EquivalentInjection>() {
            if !ei.base.base.base.in_service.unwrap_or(false) { continue; }
            let p = ei.p.unwrap_or(0.0);
            let neg_p = if p == 0.0 { 0.0 } else { -p };
            let min_p = ei.min_p.unwrap_or(0.0);
            let max_p = ei.max_p.unwrap_or(0.0);
            if neg_p < min_p || neg_p > max_p {
                v.push(Violation {
                    object_id: mrid.clone(), rule_id: "sshn456:EquivalentInjection.p-limits".into(),
                    name: "EquivalentInjection.p-limits".into(), class: "EquivalentInjection".into(),
                    property: "p".into(),
                    message: format!("Negated active power ({}) is outside of the range [Min:{}, Max:{}].", neg_p, min_p, max_p),
                    severity: "sh:Violation".into(), description: String::new(),
                });
            }
            let q = ei.q.unwrap_or(0.0);
            let neg_q = if q == 0.0 { 0.0 } else { -q };
            let min_q = ei.min_q.unwrap_or(0.0);
            let max_q = ei.max_q.unwrap_or(0.0);
            if neg_q < min_q || neg_q > max_q {
                v.push(Violation {
                    object_id: mrid.clone(), rule_id: "sshn456:EquivalentInjection.q-limits".into(),
                    name: "EquivalentInjection.q-limits".into(), class: "EquivalentInjection".into(),
                    property: "q".into(),
                    message: format!("Negated reactive power ({}) is outside of the range [Min:{}, Max:{}].", neg_q, min_q, max_q),
                    severity: "sh:Violation".into(), description: String::new(),
                });
            }
        }
    }
    v
}

pub(super) fn check_rotating_machine_curve_limits(dataset: &CimDataset) -> Vec<Violation> {
    let mut v = Vec::new();
    for mrid in dataset.by_type.get("SynchronousMachine").into_iter().flatten() {
        let entry = &dataset.entries[mrid];
        if let Some(sm) = entry.element.as_any().downcast_ref::<cimstructs::SynchronousMachine>() {
            if !sm.base.base.base.base.base.in_service.unwrap_or(false) { continue; }
            let rcc_id = match &sm.initial_reactive_capability_curve {
                Some(r) => r.mrid.trim_start_matches('#').to_string(),
                None => continue,
            };
            let mut xvals: Vec<f64> = Vec::new();
            let mut y1vals: Vec<f64> = Vec::new();
            let mut y2vals: Vec<f64> = Vec::new();
            for cd_mrid in dataset.by_type.get("CurveData").into_iter().flatten() {
                let cd_entry = &dataset.entries[cd_mrid];
                if let Some(cd) = cd_entry.element.as_any().downcast_ref::<cimstructs::CurveData>() {
                    if cd.curve.as_ref().map_or(false, |r| r.mrid.trim_start_matches('#') == rcc_id) {
                        xvals.push(cd.xvalue.unwrap_or(0.0));
                        y1vals.push(cd.y1value.unwrap_or(0.0));
                        y2vals.push(cd.y2value.unwrap_or(0.0));
                    }
                }
            }
            if xvals.is_empty() { continue; }
            let min_x = xvals.iter().cloned().fold(f64::INFINITY, f64::min);
            let max_x = xvals.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
            let min_y1 = y1vals.iter().cloned().fold(f64::INFINITY, f64::min);
            let max_y2 = y2vals.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
            let p = sm.base.p.unwrap_or(0.0);
            let neg_p = if p == 0.0 { 0.0 } else { -p };
            let q = sm.base.q.unwrap_or(0.0);
            let neg_q = if q == 0.0 { 0.0 } else { -q };
            if neg_p < min_x || neg_p > max_x {
                v.push(Violation {
                    object_id: mrid.clone(), rule_id: "sshn456:RotatingMachine-pAndQcapabilityCurveP".into(),
                    name: "RotatingMachine-pAndQcapabilityCurveP".into(), class: "SynchronousMachine".into(),
                    property: "RotatingMachine.p".into(),
                    message: format!("Negated active power ({}) is outside of curve x-range [{}, {}].", neg_p, min_x, max_x),
                    severity: "sh:Violation".into(), description: String::new(),
                });
            }
            if neg_q < min_y1 || neg_q > max_y2 {
                v.push(Violation {
                    object_id: mrid.clone(), rule_id: "sshn456:RotatingMachine-pAndQcapabilityCurveQ".into(),
                    name: "RotatingMachine-pAndQcapabilityCurveQ".into(), class: "SynchronousMachine".into(),
                    property: "RotatingMachine.q".into(),
                    message: format!("Negated reactive power ({}) is outside of curve y-range [{}, {}].", neg_q, min_y1, max_y2),
                    severity: "sh:Violation".into(), description: String::new(),
                });
            }
        }
    }
    v
}

pub(super) fn check_regulating_control_target_value_positive(dataset: &CimDataset) -> Vec<Violation> {
    let mut v = Vec::new();
    for mrid in dataset.by_type.get("RegulatingControl").into_iter().flatten() {
        let entry = &dataset.entries[mrid];
        if let Some(rc) = entry.element.as_any().downcast_ref::<cimstructs::RegulatingControl>() {
            if rc.mode.as_ref().map_or(false, |r| r.uri.ends_with("voltage")) {
                if rc.target_value.unwrap_or(0.0) <= 0.0 {
                    v.push(Violation {
                        object_id: mrid.clone(), rule_id: "sshn456:RegulatingControl.targetValue-value".into(),
                        name: "RegulatingControl.targetValue-value".into(), class: "RegulatingControl".into(),
                        property: "targetValue".into(),
                        message: "RegulatingControl.targetValue shall be positive value in cases where the RegulatingControl.mode is set to voltage.".into(),
                        severity: "sh:Violation".into(), description: String::new(),
                    });
                }
            }
        }
    }
    v
}
