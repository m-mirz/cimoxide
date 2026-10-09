use cimmodel::base::FastMap as HashMap;
use cimmodel::CimDataset;
use crate::Violation;
use super::Fields;

pub fn validate(dataset: &CimDataset) -> Vec<Violation> {
    let mut v = Vec::new();
    v.extend(check_ac_line_segment_base_voltage(dataset));
    v.extend(check_regulating_control_target_value_tap_changer(dataset));
    v.extend(check_ac_line_segment_base_voltage_diff(dataset));
    v.extend(check_boundary_point_bppl(dataset));
    v.extend(check_equivalent_injection_regulation_capability_not_hvdc(dataset));
    v
}

fn terminal_nominal_voltage(dataset: &CimDataset, term: &Fields) -> Option<f64> {
    let tn_id = term.reference("Terminal.TopologicalNode")?.trim_start_matches('#').to_string();
    let tn = Fields::get(dataset, &tn_id, "TopologicalNode")?;
    let bv_id = tn.reference("TopologicalNode.BaseVoltage")?.trim_start_matches('#').to_string();
    let bv = Fields::get(dataset, &bv_id, "BaseVoltage")?;
    bv.f64("BaseVoltage.nominalVoltage")
}

fn build_terminals_by_equipment_seq(dataset: &CimDataset) -> HashMap<String, HashMap<i64, String>> {
    let mut map: HashMap<String, HashMap<i64, String>> = HashMap::default();
    for mrid in dataset.by_type.get("Terminal").into_iter().flatten() {
        let entry = &dataset.entries[mrid];
        if let Some(term) = Fields::of_class(entry, "Terminal")
            && let Some(ce) = &term.reference("Terminal.ConductingEquipment") {
                let eq_id = ce.trim_start_matches('#').to_string();
                if let Some(seq) = term.i64("ACDCTerminal.sequenceNumber") {
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
        let t1 = match dataset.entries.get(t1_id).and_then(|e| Fields::of_class(e, "Terminal")) { Some(t) => t, None => continue };
        let t2 = match dataset.entries.get(t2_id).and_then(|e| Fields::of_class(e, "Terminal")) { Some(t) => t, None => continue };
        let v1 = match terminal_nominal_voltage(dataset, &t1) { Some(v) => v, None => continue };
        let v2 = match terminal_nominal_voltage(dataset, &t2) { Some(v) => v, None => continue };
        // Skip if same TN
        if t1.reference("Terminal.TopologicalNode") == t2.reference("Terminal.TopologicalNode") { continue; }
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
    let mut rtc_by_tcc: HashMap<String, Vec<String>> = HashMap::default();
    for rtc_mrid in dataset.by_type.get("RatioTapChanger").into_iter().flatten() {
        let rtc_entry = &dataset.entries[rtc_mrid];
        if let Some(rtc) = Fields::of_class(rtc_entry, "RatioTapChanger")
            && let Some(r) = &rtc.reference("TapChanger.TapChangerControl") {
                let tcc_id = r.trim_start_matches('#').to_string();
                rtc_by_tcc.entry(tcc_id).or_default().push(rtc_mrid.clone());
            }
    }

    // Collect all RCs with voltage mode enabled
    for rc_mrid in dataset.by_type.get("RegulatingControl").into_iter().chain(dataset.by_type.get("TapChangerControl")).flatten() {
        let rc_entry = &dataset.entries[rc_mrid];
        // Every value the formula reads is a required pattern.
        let rc = Fields::of(rc_entry);
        let mode_ok = rc.enumeration("RegulatingControl.mode").is_some_and(|r| r.ends_with(voltage_suffix));
        let enabled = rc.bool("RegulatingControl.enabled") == Some(true);
        let Some(target_value) = rc.f64("RegulatingControl.targetValue") else { continue };
        let terminal_ref = rc.reference("RegulatingControl.Terminal");
        if !mode_ok || !enabled { continue; }
        let term_id = match terminal_ref { Some(r) => r.trim_start_matches('#'), None => continue };

        // Find associated RatioTapChanger referencing this RC
        for rtc_mrid in rtc_by_tcc.get(rc_mrid).into_iter().flatten() {
            let rtc_entry = &dataset.entries[rtc_mrid];
            let rtc = match Fields::of_class(rtc_entry, "RatioTapChanger") { Some(r) => r, None => continue };
            if rtc.bool("TapChanger.controlEnabled") != Some(true) { continue; }

            // Get nominal voltage via RC terminal → CN → VoltageLevel → BaseVoltage
            let nominal_u = (|| -> Option<f64> {
                let term = Fields::get(dataset, term_id, "Terminal")?;
                let cn_id = term.reference("Terminal.ConnectivityNode")?.trim_start_matches('#');
                let cn = Fields::get(dataset, cn_id, "ConnectivityNode")?;
                let cnc_id = cn.reference("ConnectivityNode.ConnectivityNodeContainer")?.trim_start_matches('#');
                let vl = Fields::get(dataset, cnc_id, "VoltageLevel")?;
                let bv_id = vl.reference("VoltageLevel.BaseVoltage")?.trim_start_matches('#');
                Fields::get(dataset, bv_id, "BaseVoltage")?.f64("BaseVoltage.nominalVoltage")
            })();
            let nominal_u = match nominal_u { Some(u) if u != 0.0 => u, _ => continue };

            let target_pu = target_value / nominal_u;
            let (Some(svi), Some(high), Some(neutral), Some(low)) = (
                rtc.f64("RatioTapChanger.stepVoltageIncrement"), rtc.i64("TapChanger.highStep"),
                rtc.i64("TapChanger.neutralStep"), rtc.i64("TapChanger.lowStep")) else { continue };
            let step_pct = svi / 100.0;
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
        let t1 = match dataset.entries.get(t1_id).and_then(|e| Fields::of_class(e, "Terminal")) { Some(t) => t, None => continue };
        let t2 = match dataset.entries.get(t2_id).and_then(|e| Fields::of_class(e, "Terminal")) { Some(t) => t, None => continue };
        let v1 = match terminal_nominal_voltage(dataset, &t1) { Some(v) => v, None => continue };
        let v2 = match terminal_nominal_voltage(dataset, &t2) { Some(v) => v, None => continue };
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

/// The two-terminal classes a boundary point's node should connect.
const BRANCHES: &[&str] = &[
    "ACLineSegment", "PowerTransformer", "DCLineSegment", "DCSeriesDevice", "DCSwitch", "DCDisconnector",
    "DCBreaker", "DCChopper", "Switch", "Disconnector", "Fuse", "GroundDisconnector", "Jumper", "Breaker",
    "LoadBreakSwitch", "DisconnectingCircuitBreaker", "Cut", "SeriesCompensator", "EquivalentBranch",
];

/// The target is a ConnectivityNode the dataset holds with a BoundaryPoint on
/// it, and both rules need its name. bppl1Bppl2: a branch connects there but
/// no EquivalentInjection does. bppl3: no branch connects there.
fn check_boundary_point_bppl(dataset: &CimDataset) -> Vec<Violation> {
    let mut nodes: Vec<&str> = Vec::new();
    for mrid in dataset.by_type.get("BoundaryPoint").into_iter().flatten() {
        if let Some(r) = Fields::of(&dataset.entries[mrid]).reference("BoundaryPoint.ConnectivityNode") {
            nodes.push(r.trim_start_matches('#'));
        }
    }
    nodes.sort_unstable();
    nodes.dedup();
    // CN → classes of the equipment connected there
    let mut cn_classes: HashMap<&str, Vec<&str>> = HashMap::default();
    for mrid in dataset.by_type.get("Terminal").into_iter().flatten() {
        let term = Fields::of(&dataset.entries[mrid]);
        if let (Some(cn), Some(eq)) = (term.reference("Terminal.ConnectivityNode"), term.reference("Terminal.ConductingEquipment"))
            && let Some(e) = dataset.entries.get(eq.trim_start_matches('#')) {
                cn_classes.entry(cn.trim_start_matches('#')).or_default().push(e.type_name());
            }
    }
    let mut v = Vec::new();
    for cn_id in nodes {
        let Some(cn) = Fields::get(dataset, cn_id, "ConnectivityNode") else { continue };
        if !cn.has("IdentifiedObject.name") { continue; }
        let classes = cn_classes.get(cn_id).map(Vec::as_slice).unwrap_or_default();
        let has_branch = classes.iter().any(|c| BRANCHES.contains(c));
        let has_injection = classes.contains(&"EquivalentInjection");
        if has_branch && !has_injection {
            v.push(Violation {
                object_id: cn_id.to_string(), rule_id: "eqn600:BoundaryPoint-bppl1Bppl2".into(),
                name: "C:600:EQ:BoundaryPoint:bppl1Bppl2".into(), class: "ConnectivityNode".into(),
                property: "rdf:type".into(),
                message: "Boundary Point ConnectivityNode does not have an EquivalentInjection connected.".into(),
                severity: "sh:Violation".into(), description: String::new(),
            });
        }
        if !has_branch {
            v.push(Violation {
                object_id: cn_id.to_string(), rule_id: "eqn600:BoundaryPoint-bppl3".into(),
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
    // Only a BoundaryPoint that says it is not DC counts.
    let mut cn_is_dc: HashMap<String, bool> = HashMap::default();
    for mrid in dataset.by_type.get("BoundaryPoint").into_iter().flatten() {
        let entry = &dataset.entries[mrid];
        if let Some(bp) = Fields::of_class(entry, "BoundaryPoint")
            && let Some(r) = &bp.reference("BoundaryPoint.ConnectivityNode")
            && let Some(is_dc) = bp.bool("BoundaryPoint.isDirectCurrent") {
                cn_is_dc.insert(r.trim_start_matches('#').to_string(), is_dc);
            }
    }
    // Equipment MRID → true if it has at least one terminal connected to a non-HVDC
    // BoundaryPoint CN. Built once over all Terminals instead of rescanning them per
    // EquivalentInjection below.
    let mut equip_non_hvdc_bp: HashMap<String, bool> = HashMap::default();
    for term_mrid in dataset.by_type.get("Terminal").into_iter().flatten() {
        let term = match dataset.entries.get(term_mrid).and_then(|e| Fields::of_class(e, "Terminal")) { Some(t) => t, None => continue };
        let eq_id = match term.reference("Terminal.ConductingEquipment") { Some(r) => r.trim_start_matches('#').to_string(), None => continue };
        if let Some(cn) = &term.reference("Terminal.ConnectivityNode") {
            let cn_id = cn.trim_start_matches('#');
            if let Some(&is_dc) = cn_is_dc.get(cn_id)
                && !is_dc {
                    equip_non_hvdc_bp.insert(eq_id, true);
                }
        }
    }

    let mut v = Vec::new();
    for ei_mrid in dataset.by_type.get("EquivalentInjection").into_iter().flatten() {
        let ei_entry = &dataset.entries[ei_mrid];
        let ei = match Fields::of_class(ei_entry, "EquivalentInjection") { Some(e) => e, None => continue };
        let is_non_hvdc_bp = equip_non_hvdc_bp.contains_key(ei_mrid.as_str());
        if is_non_hvdc_bp
            && let Some(capable) = ei.bool("EquivalentInjection.regulationCapability")
            && (capable || ei.has("EquivalentInjection.ReactiveCapabilityCurve")) {
                v.push(Violation {
                    object_id: ei_mrid.clone(), rule_id: "eqn600:EquivalentInjection.regulationCapability-notHVDC".into(),
                    name: "C:600:EQ:EquivalentInjection.regulationCapability:notHvdc".into(), class: "EquivalentInjection".into(),
                    property: "EquivalentInjection.regulationCapability".into(),
                    message: "EquivalentInjection at non-HVDC BoundaryPoint has regulationCapability=true or a ReactiveCapabilityCurve.".into(),
                    severity: "sh:Violation".into(), description: String::new(),
                });
            }
    }
    v
}
