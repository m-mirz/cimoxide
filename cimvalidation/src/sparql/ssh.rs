use cimmodel::base::FastMap as HashMap;
use cimmodel::CimDataset;
use crate::Violation;
use super::Fields;

pub fn validate(dataset: &CimDataset) -> Vec<Violation> {
    let mut v = Vec::new();
    v.extend(check_energy_source_active_power_consumer(dataset));
    v.extend(check_regulating_control_target_deadband_applicability(dataset));
    v.extend(check_cs_converter_value_range(dataset));
    v.extend(check_cs_converter_p_pcc_control(dataset));
    v.extend(check_vs_converter_p_pcc_control(dataset));
    v.extend(check_vs_converter_q_pcc_control(dataset));
    // EnergySource-EnergySourcePQ is an `sh:and` node shape, which the CGMES
    // shape table runs.
    // The `sshn456:` rules below are the NotSolvedMAS file's, run by
    // `ssh_not_solved_mas::validate` alone.
    v
}

fn check_energy_source_active_power_consumer(dataset: &CimDataset) -> Vec<Violation> {
    let mut v = Vec::new();
    for mrid in dataset.by_type.get("EnergySource").into_iter().flatten() {
        let entry = &dataset.entries[mrid];
        if let Some(es) = Fields::of_class(entry, "EnergySource")
            && es.f64("EnergySource.activePower").is_some_and(|p| p > 0.0) {
                v.push(Violation {
                    object_id:   mrid.clone(),
                    rule_id:     "sshu:EnergySource.activePower-consumer".into(),
                    name:        "C:301:SSH:EnergySource.activePower:consumer".into(),
                    class:       "EnergySource".into(),
                    property:    "EnergySource.activePower".into(),
                    message:     "EnergySource that is a consumer (activePower > 0).".into(),
                    severity:    "sh:Warning".into(),
                    description: String::new(),
                });
            }
    }
    v
}

fn check_regulating_control_target_deadband_applicability(dataset: &CimDataset) -> Vec<Violation> {
    let mut v = Vec::new();
    // `discrete` is the path, so required; the deadband is `bound` or not —
    // a deadband of 0 is a deadband.
    let check = |mrid: &str, class: &str, has_deadband: bool, discrete: Option<bool>| -> Option<Violation> {
        let discrete = discrete?;
        if (has_deadband && !discrete) || (!has_deadband && discrete) {
            Some(Violation {
                object_id:   mrid.to_string(),
                rule_id:     "sshu:RegulatingControl.targetDeadband-applicability".into(),
                name:        "C:301:SSH:RegulatingControl.targetDeadband:applicability".into(),
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
        if let Some(rc) = Fields::of_class(entry, "RegulatingControl")
            && let Some(viol) = check(mrid, "RegulatingControl", rc.has("RegulatingControl.targetDeadband"), rc.bool("RegulatingControl.discrete")) {
                v.push(viol);
            }
    }
    for mrid in dataset.by_type.get("TapChangerControl").into_iter().flatten() {
        let entry = &dataset.entries[mrid];
        if let Some(tcc) = Fields::of_class(entry, "TapChangerControl")
            && let Some(viol) = check(mrid, "TapChangerControl", tcc.has("RegulatingControl.targetDeadband"), tcc.bool("RegulatingControl.discrete")) {
                v.push(viol);
            }
    }
    v
}

fn check_cs_converter_value_range(dataset: &CimDataset) -> Vec<Violation> {
    let rectifier = "CsOperatingModeKind.rectifier";
    let inverter  = "CsOperatingModeKind.inverter";
    let mut v = Vec::new();
    for mrid in dataset.by_type.get("CsConverter").into_iter().flatten() {
        let entry = &dataset.entries[mrid];
        if let Some(csc) = Fields::of_class(entry, "CsConverter") {
            let mode = match csc.enumeration("CsConverter.operatingMode") { Some(r) => r, None => continue };
            if mode == rectifier {
                // As the SPARQL: a bound value is required, and an absent
                // maximum leaves only the lower bound (`?value > ?max` is an
                // error, so false, when ?max is unbound).
                if csc.f64("CsConverter.maxAlpha").is_some_and(|max| max > 18.0) {
                    v.push(Violation {
                        object_id: mrid.clone(), rule_id: "sshu:CsConverter.maxAlpha-valueRangeTypical".into(),
                        name: "C:301:EQ:CsConverter.maxAlpha:valueRangeTypical".into(), class: "CsConverter".into(),
                        property: "CsConverter.maxAlpha".into(), message: "The maxAlpha value is greater than 18 for a rectifier.".into(),
                        severity: "sh:Warning".into(), description: String::new(),
                    });
                }
                if let Some(min_a) = csc.f64("CsConverter.minAlpha")
                    && (min_a < 10.0 || csc.f64("CsConverter.maxAlpha").is_some_and(|max_a| min_a > max_a))
                {
                    v.push(Violation {
                        object_id: mrid.clone(), rule_id: "sshu:CsConverter.minAlpha-valueRangeTypical".into(),
                        name: "C:301:SV:CsConverter.minAlpha:valueRangeTypical".into(), class: "CsConverter".into(),
                        property: "CsConverter.minAlpha".into(), message: "The minAlpha value is less than 10 or greater than CsConverter.maxAlpha for a rectifier.".into(),
                        severity: "sh:Warning".into(), description: String::new(),
                    });
                }
            } else if mode == inverter {
                if csc.f64("CsConverter.maxGamma").is_some_and(|max| max > 20.0) {
                    v.push(Violation {
                        object_id: mrid.clone(), rule_id: "sshu:CsConverter.maxGamma-valueRangeTypical".into(),
                        name: "C:301:EQ:CsConverter.maxGamma:valueRangeTypical".into(), class: "CsConverter".into(),
                        property: "CsConverter.maxGamma".into(), message: "The maxGamma value is greater than 20 for an inverter.".into(),
                        severity: "sh:Warning".into(), description: String::new(),
                    });
                }
                if let Some(min_g) = csc.f64("CsConverter.minGamma")
                    && (min_g < 17.0 || csc.f64("CsConverter.maxGamma").is_some_and(|max_g| min_g > max_g))
                {
                    v.push(Violation {
                        object_id: mrid.clone(), rule_id: "sshu:CsConverter.minGamma-valueRangeTypical".into(),
                        name: "C:301:SV:CsConverter.minGamma:valueRangeTypical".into(), class: "CsConverter".into(),
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
    let dc_current   = "CsPpccControlKind.dcCurrent";
    let dc_voltage   = "CsPpccControlKind.dcVoltage";
    let active_power = "CsPpccControlKind.activePower";
    let mut v = Vec::new();
    for mrid in dataset.by_type.get("CsConverter").into_iter().flatten() {
        let entry = &dataset.entries[mrid];
        if let Some(csc) = Fields::of_class(entry, "CsConverter") {
            let ctrl = match csc.enumeration("CsConverter.pPccControl") { Some(r) => r, None => continue };
            // The target must be given (`!bound` fails); 0 is a target.
            if ctrl == dc_current && !csc.has("CsConverter.targetIdc") {
                v.push(Violation { object_id: mrid.clone(), rule_id: "sshu:CsConverter.pPccControl-targetValueIdc".into(),
                    name: "C:301:SSH:CsPpccControlKind.dcCurrent:targetValueIdc".into(), class: "CsConverter".into(),
                    property: "CsConverter.pPccControl".into(),
                    message: "CsConverter.targetIdc is not provided for a converter with CsPpccControlKind.dcCurrent.".into(),
                    severity: "sh:Violation".into(), description: String::new() });
            } else if ctrl == dc_voltage && !csc.has("ACDCConverter.targetUdc") {
                v.push(Violation { object_id: mrid.clone(), rule_id: "sshu:CsConverter.pPccControl-targetValueUdc".into(),
                    name: "C:301:SSH:CsPpccControlKind.dcVoltage:targetValueUdc".into(), class: "CsConverter".into(),
                    property: "CsConverter.pPccControl".into(),
                    message: "ACDCConverter.targetUdc is not provided for a converter with CsPpccControlKind.dcVoltage.".into(),
                    severity: "sh:Violation".into(), description: String::new() });
            } else if ctrl == active_power && !csc.has("ACDCConverter.targetPpcc") {
                v.push(Violation { object_id: mrid.clone(), rule_id: "sshu:CsConverter.pPccControl-targetValuePpcc".into(),
                    name: "C:301:SSH:CsPpccControlKind.activePower:targetValuePpcc".into(), class: "CsConverter".into(),
                    property: "CsConverter.pPccControl".into(),
                    message: "ACDCConverter.targetPpcc is not provided for a converter with CsPpccControlKind.activePower.".into(),
                    severity: "sh:Violation".into(), description: String::new() });
            }
        }
    }
    v
}

fn check_vs_converter_p_pcc_control(dataset: &CimDataset) -> Vec<Violation> {
    let prefix = "VsPpccControlKind.";
    let mut v = Vec::new();
    for mrid in dataset.by_type.get("VsConverter").into_iter().flatten() {
        let entry = &dataset.entries[mrid];
        if let Some(vsc) = Fields::of_class(entry, "VsConverter") {
            let ctrl = match vsc.enumeration("VsConverter.pPccControl") { Some(r) => r, None => continue };
            // Each target missing (`!bound`); 0 is a target.
            let ppcc      = !vsc.has("ACDCConverter.targetPpcc");
            let udc       = !vsc.has("ACDCConverter.targetUdc");
            let droop     = !vsc.has("VsConverter.droop");
            let droopcomp = !vsc.has("VsConverter.droopCompensation");
            let phase_pcc = !vsc.has("VsConverter.targetPhasePcc");
            let (rule_id, name, msg): (&str, &str, Option<&str>) =
                if ctrl == format!("{prefix}pPccAndUdcDroop") {
                    ("sshu:VsConverter.pPccControl-targetValuepPccAndUdcDroop",
                     "C:301:SSH:VsPpccControlKind.pPccAndUdcDroop:targetValuepPccAndUdcDroop",
                     if ppcc || udc || droop {
                         Some("One or all among ACDCConverter.targetPpcc, ACDCConverter.targetUdc and VsConverter.droop are not provided for VsPpccControlKind.pPccAndUdcDroop.")
                     } else { None })
                } else if ctrl == format!("{prefix}pPccAndUdcDroopWithCompensation") {
                    ("sshu:VsConverter.pPccControl-targetValuepPccAndUdcDroopWithCompensation",
                     "C:301:SSH:VsPpccControlKind.pPccAndUdcDroopWithCompensation:targetValuepPccAndUdcDroopWithCompensation",
                     if ppcc || udc || droop || droopcomp {
                         Some("One or all among ACDCConverter.targetPpcc, ACDCConverter.targetUdc, VsConverter.droop and VsConverter.droopCompensation are not provided for VsPpccControlKind.pPccAndUdcDroopWithCompensation.")
                     } else { None })
                } else if ctrl == format!("{prefix}pPccAndUdcDroopPilot") {
                    ("sshu:VsConverter.pPccControl-targetValuepPccAndUdcDroopPilot",
                     "C:301:SSH:VsPpccControlKind.pPccAndUdcDroopPilot:targetValuepPccAndUdcDroopPilot",
                     if ppcc || udc || droop {
                         Some("One or all among ACDCConverter.targetPpcc, ACDCConverter.targetUdc and VsConverter.droop are not provided for VsPpccControlKind.pPccAndUdcDroopPilot.")
                     } else { None })
                } else if ctrl == format!("{prefix}udc") {
                    ("sshu:VsConverter.pPccControl-targetValueUdc",
                     "C:301:SSH:VsPpccControlKind.udc:targetValueUdc",
                     if udc { Some("ACDCConverter.targetUdc is not provided for VsPpccControlKind.udc.") } else { None })
                } else if ctrl == format!("{prefix}pPcc") {
                    ("sshu:VsConverter.pPccControl-targetValuePpcc",
                     "C:301:SSH:VsPpccControlKind.pPcc:targetValuePpcc",
                     if ppcc { Some("ACDCConverter.targetPpcc is not provided for VsPpccControlKind.pPcc.") } else { None })
                } else if ctrl == format!("{prefix}phasePcc") {
                    ("sshu:VsConverter.pPccControl-targetValuephasePcc",
                     "C:301:SSH:VsPpccControlKind.phasePcc:targetValuephasePcc",
                     if phase_pcc { Some("VsConverter.targetPhasePcc is not provided for VsPpccControlKind.phasePcc.") } else { None })
                } else {
                    continue;
                };
            if let Some(msg) = msg {
                v.push(Violation { object_id: mrid.clone(), rule_id: rule_id.into(),
                    name: name.into(), class: "VsConverter".into(),
                    property: "VsConverter.pPccControl".into(), message: msg.into(),
                    severity: "sh:Violation".into(), description: String::new() });
            }
        }
    }
    v
}

fn check_vs_converter_q_pcc_control(dataset: &CimDataset) -> Vec<Violation> {
    let prefix = "VsQpccControlKind.";
    let mut v = Vec::new();
    for mrid in dataset.by_type.get("VsConverter").into_iter().flatten() {
        let entry = &dataset.entries[mrid];
        if let Some(vsc) = Fields::of_class(entry, "VsConverter") {
            let ctrl = match vsc.enumeration("VsConverter.qPccControl") { Some(r) => r, None => continue };
            let pf        = !vsc.has("VsConverter.targetPowerFactorPcc");
            let pwm       = !vsc.has("VsConverter.targetPWMfactor");
            let phase_pcc = !vsc.has("VsConverter.targetPhasePcc");
            let qpcc      = !vsc.has("VsConverter.targetQpcc");
            let upcc      = !vsc.has("VsConverter.targetUpcc");
            let (rule_id, name, msg): (&str, &str, Option<&str>) =
                if ctrl == format!("{prefix}powerFactorPcc") {
                    ("sshu:VsConverter.qPccControl-targetValuepowerFactorPcc",
                     "C:301:SSH:VsQpccControlKind.powerFactorPcc:targetValuepowerFactorPcc",
                     if pf { Some("VsConverter.targetPowerFactorPcc is not provided for VsQpccControlKind.powerFactorPcc.") } else { None })
                } else if ctrl == format!("{prefix}pulseWidthModulation") {
                    ("sshu:VsConverter.qPccControl-targetValuepulseWidthModulation",
                     "C:301:SSH:VsQpccControlKind.pulseWidthModulation:targetValuepulseWidthModulation",
                     if pwm || phase_pcc {
                         Some("VsConverter.targetPWMfactor and/or VsConverter.targetPhasePcc are not provided for VsQpccControlKind.pulseWidthModulation.")
                     } else { None })
                } else if ctrl == format!("{prefix}reactivePcc") {
                    ("sshu:VsConverter.qPccControl-targetValuereactivePcc",
                     "C:301:SSH:VsQpccControlKind.reactivePcc:targetValuereactivePcc",
                     if qpcc { Some("VsConverter.targetQpcc is not provided for VsQpccControlKind.reactivePcc.") } else { None })
                } else if ctrl == format!("{prefix}voltagePcc") {
                    ("sshu:VsConverter.qPccControl-targetValuevoltagePcc",
                     "C:301:SSH:VsQpccControlKind.voltagePcc:targetValuevoltagePcc",
                     if upcc { Some("VsConverter.targetUpcc is not provided for VsQpccControlKind.voltagePcc.") } else { None })
                } else {
                    continue;
                };
            if let Some(msg) = msg {
                v.push(Violation { object_id: mrid.clone(), rule_id: rule_id.into(),
                    name: name.into(), class: "VsConverter".into(),
                    property: "VsConverter.qPccControl".into(), message: msg.into(),
                    severity: "sh:Violation".into(), description: String::new() });
            }
        }
    }
    v
}

pub(super) fn check_synchronous_machine_operating_mode_match(dataset: &CimDataset) -> Vec<Violation> {
    let mut v = Vec::new();
    for mrid in dataset.by_type.get("SynchronousMachine").into_iter().flatten() {
        let entry = &dataset.entries[mrid];
        if let Some(sm) = Fields::of_class(entry, "SynchronousMachine") {
            let mode = match sm.enumeration("SynchronousMachine.operatingMode") { Some(r) => r, None => continue };
            let kind = match sm.enumeration("SynchronousMachine.type") { Some(r) => r, None => continue };
            let valid = if mode.ends_with("motor") {
                kind.ends_with("motor") || kind.ends_with("generatorOrMotor") || kind.ends_with("motorOrCondenser") || kind.ends_with("generatorOrCondenserOrMotor")
            } else if mode.ends_with("condenser") {
                kind.ends_with("condenser") || kind.ends_with("generatorOrCondenser") || kind.ends_with("motorOrCondenser") || kind.ends_with("generatorOrCondenserOrMotor")
            } else if mode.ends_with("generator") {
                kind.ends_with("generator") || kind.ends_with("generatorOrMotor") || kind.ends_with("generatorOrCondenser") || kind.ends_with("generatorOrCondenserOrMotor")
            } else {
                // Another mode is no combination the SPARQL tests.
                true
            };
            if !valid {
                v.push(Violation {
                    object_id: mrid.clone(), rule_id: "sshn456:SynchronousMachine.operatingMode-matchType".into(),
                    name: "C:456:SSH:SynchronousMachine.operatingMode:matchType".into(), class: "SynchronousMachine".into(),
                    property: "SynchronousMachine.operatingMode".into(),
                    message: format!("SynchronousMachine.operatingMode ({}) is not consistent with SynchronousMachine.type ({}).", mode, kind),
                    severity: "sh:Violation".into(), description: String::new(),
                });
            }
        }
    }
    v
}

/// `C:456:SSH:NA:singleActivePowerSlack`: exactly one GeneratingUnit holds the
/// highest `normalPF`, and that is not 0 — over the dataset, as the SPARQL
/// counts it (its `MAX` and grouping run over untyped literals; the numbers
/// are compared here). Like the SPARQL's single row, one finding, on the
/// first unit holding the highest value.
pub(super) fn check_generating_unit_single_active_power_slack(dataset: &CimDataset) -> Vec<Violation> {
    let units: Vec<(&String, f64)> = dataset
        .by_type
        .iter()
        .filter(|(class, _)| class.ends_with("GeneratingUnit") && !class.contains(':'))
        .flat_map(|(_, mrids)| mrids)
        .filter_map(|m| Fields::of(&dataset.entries[m]).f64("GeneratingUnit.normalPF").map(|pf| (m, pf)))
        .collect();
    let Some(max) = units.iter().map(|(_, pf)| *pf).reduce(f64::max) else { return Vec::new() };
    let at_max: Vec<&String> = units.iter().filter(|(_, pf)| *pf == max).map(|(m, _)| *m).collect();
    if max != 0.0 && at_max.len() == 1 {
        return Vec::new();
    }
    let first = at_max.iter().min().expect("the maximum is held");
    vec![Violation {
        object_id: (*first).clone(), rule_id: "sshn456:GeneratingUnit-singleActivePowerSlack".into(),
        name: "C:456:SSH:NA:singleActivePowerSlack".into(), class: dataset.entries[*first].type_name().into(),
        property: "GeneratingUnit.normalPF".into(),
        message: if max == 0.0 {
            "The highest GeneratingUnit.normalPF is 0: no unit is the active power slack.".into()
        } else {
            format!("GeneratingUnit.normalPF {max} is the highest, and {} units have it.", at_max.len())
        },
        severity: "sh:Violation".into(), description: String::new(),
    }]
}

pub(super) fn check_external_network_injection_limits(dataset: &CimDataset) -> Vec<Violation> {
    let mut v = Vec::new();
    for mrid in dataset.by_type.get("ExternalNetworkInjection").into_iter().flatten() {
        let entry = &dataset.entries[mrid];
        if let Some(eni) = Fields::of_class(entry, "ExternalNetworkInjection") {
            // The value and both limits are required patterns; the SPARQL has
            // no in-service condition.
            if let (Some(p), Some(min_p), Some(max_p)) =
                (eni.f64("ExternalNetworkInjection.p"), eni.f64("ExternalNetworkInjection.minP"), eni.f64("ExternalNetworkInjection.maxP"))
                && let neg_p = if p == 0.0 { 0.0 } else { -p }
                && (neg_p < min_p || neg_p > max_p) {
                v.push(Violation {
                    object_id: mrid.clone(), rule_id: "sshn456:ExternalNetworkInjection.p-limits".into(),
                    name: "C:456:SSH:ExternalNetworkInjection.p:limits".into(), class: "ExternalNetworkInjection".into(),
                    property: "p".into(),
                    message: format!("Negated active power ({}) is outside of the range [Min:{}, Max:{}].", neg_p, min_p, max_p),
                    severity: "sh:Violation".into(), description: String::new(),
                });
            }
            if let (Some(q), Some(min_q), Some(max_q)) =
                (eni.f64("ExternalNetworkInjection.q"), eni.f64("ExternalNetworkInjection.minQ"), eni.f64("ExternalNetworkInjection.maxQ"))
                && let neg_q = if q == 0.0 { 0.0 } else { -q }
                && (neg_q < min_q || neg_q > max_q) {
                v.push(Violation {
                    object_id: mrid.clone(), rule_id: "sshn456:ExternalNetworkInjection.q-limits".into(),
                    name: "C:456:SSH:ExternalNetworkInjection.q:limits".into(), class: "ExternalNetworkInjection".into(),
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
        if let Some(ei) = Fields::of_class(entry, "EquivalentInjection") {
            // The value and both limits are required patterns; the SPARQL has
            // no in-service condition.
            if let (Some(p), Some(min_p), Some(max_p)) =
                (ei.f64("EquivalentInjection.p"), ei.f64("EquivalentInjection.minP"), ei.f64("EquivalentInjection.maxP"))
                && let neg_p = if p == 0.0 { 0.0 } else { -p }
                && (neg_p < min_p || neg_p > max_p) {
                v.push(Violation {
                    object_id: mrid.clone(), rule_id: "sshn456:EquivalentInjection.p-limits".into(),
                    name: "C:456:SSH:EquivalentInjection.p:limits".into(), class: "EquivalentInjection".into(),
                    property: "p".into(),
                    message: format!("Negated active power ({}) is outside of the range [Min:{}, Max:{}].", neg_p, min_p, max_p),
                    severity: "sh:Violation".into(), description: String::new(),
                });
            }
            if let (Some(q), Some(min_q), Some(max_q)) =
                (ei.f64("EquivalentInjection.q"), ei.f64("EquivalentInjection.minQ"), ei.f64("EquivalentInjection.maxQ"))
                && let neg_q = if q == 0.0 { 0.0 } else { -q }
                && (neg_q < min_q || neg_q > max_q) {
                v.push(Violation {
                    object_id: mrid.clone(), rule_id: "sshn456:EquivalentInjection.q-limits".into(),
                    name: "C:456:SSH:EquivalentInjection.q:limits".into(), class: "EquivalentInjection".into(),
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
    // Curve MRID → (x, y1, y2) points. Built once instead of rescanning all CurveData per
    // SynchronousMachine below.
    // A point counts for P with its x value given, for Q with both y values.
    type Point = (Option<f64>, Option<(f64, f64)>);
    let mut curve_points: HashMap<String, Vec<Point>> = HashMap::default();
    for cd_mrid in dataset.by_type.get("CurveData").into_iter().flatten() {
        let cd_entry = &dataset.entries[cd_mrid];
        if let Some(cd) = Fields::of_class(cd_entry, "CurveData")
            && let Some(r) = &cd.reference("CurveData.Curve") {
                let curve_id = r.trim_start_matches('#').to_string();
                let ys = cd.f64("CurveData.y1value").zip(cd.f64("CurveData.y2value"));
                curve_points.entry(curve_id).or_default().push((cd.f64("CurveData.xvalue"), ys));
            }
    }

    let mut v = Vec::new();
    for mrid in dataset.by_type.get("SynchronousMachine").into_iter().flatten() {
        let entry = &dataset.entries[mrid];
        if let Some(sm) = Fields::of_class(entry, "SynchronousMachine") {
            // No in-service condition, as the SPARQL.
            let rcc_id = match sm.reference("SynchronousMachine.InitialReactiveCapabilityCurve") {
                Some(r) => r.trim_start_matches('#').to_string(),
                None => continue,
            };
            let points = match curve_points.get(&rcc_id) { Some(p) => p, None => continue };
            let xvals: Vec<f64> = points.iter().filter_map(|(x, _)| *x).collect();
            let yvals: Vec<(f64, f64)> = points.iter().filter_map(|(_, y)| *y).collect();
            let neg = |x: f64| if x == 0.0 { 0.0 } else { -x };
            if let Some(p) = sm.f64("RotatingMachine.p")
                && !xvals.is_empty()
                && let min_x = xvals.iter().cloned().fold(f64::INFINITY, f64::min)
                && let max_x = xvals.iter().cloned().fold(f64::NEG_INFINITY, f64::max)
                && let neg_p = neg(p)
                && (neg_p < min_x || neg_p > max_x) {
                v.push(Violation {
                    object_id: mrid.clone(), rule_id: "sshn456:RotatingMachine-pAndQcapabilityCurveP".into(),
                    name: "C:456:SSH:RotatingMachine:pAndQcapabilityCurve".into(), class: "SynchronousMachine".into(),
                    property: "RotatingMachine.p".into(),
                    message: format!("Negated active power ({}) is outside of curve x-range [{}, {}].", neg_p, min_x, max_x),
                    severity: "sh:Violation".into(), description: String::new(),
                });
            }
            if let Some(q) = sm.f64("RotatingMachine.q")
                && !yvals.is_empty()
                && let min_y1 = yvals.iter().map(|(y1, _)| *y1).fold(f64::INFINITY, f64::min)
                && let max_y2 = yvals.iter().map(|(_, y2)| *y2).fold(f64::NEG_INFINITY, f64::max)
                && let neg_q = neg(q)
                && (neg_q < min_y1 || neg_q > max_y2) {
                v.push(Violation {
                    object_id: mrid.clone(), rule_id: "sshn456:RotatingMachine-pAndQcapabilityCurveQ".into(),
                    name: "C:456:SSH:RotatingMachine:pAndQcapabilityCurve".into(), class: "SynchronousMachine".into(),
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
    for class in ["RegulatingControl", "TapChangerControl"] {
    for mrid in dataset.by_type.get(class).into_iter().flatten() {
        let entry = &dataset.entries[mrid];
        if let Some(rc) = Fields::of_class(entry, class)
            && rc.enumeration("RegulatingControl.mode") == Some("RegulatingControlModeKind.voltage")
                && rc.f64("RegulatingControl.targetValue").is_some_and(|t| t <= 0.0) {
                    v.push(Violation {
                        object_id: mrid.clone(), rule_id: "sshn456:RegulatingControl.targetValue-value".into(),
                        name: "C:456:SSH:RegulatingControl.targetValue:value".into(), class: class.into(),
                        property: "targetValue".into(),
                        message: "RegulatingControl.targetValue shall be positive value in cases where the RegulatingControl.mode is set to voltage.".into(),
                        severity: "sh:Violation".into(), description: String::new(),
                    });
                }
    }
    }
    v
}
