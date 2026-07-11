use std::collections::{HashMap, HashSet};
use cimdecoder::CimDataset;
use crate::Violation;

pub fn check_base_voltage_in_eqbd_impl(dataset: &CimDataset, eqbd_bv_ids: &HashSet<String>) -> Vec<Violation> {
    let mut v = Vec::new();
    for mrid in dataset.by_type.get("BaseVoltage").into_iter().flatten() {
        if eqbd_bv_ids.contains(mrid.as_str()) { continue; }
        let entry = &dataset.entries[mrid];
        let nominal_voltage = entry.element.as_any().downcast_ref::<cimstructs::BaseVoltage>()
            .and_then(|bv| bv.nominal_voltage).unwrap_or(0.0);
        v.push(Violation {
            object_id:   mrid.clone(),
            rule_id:     "eqbd2:EQBD2".into(),
            name:        "EQBD2".into(),
            class:       "BaseVoltage".into(),
            property:    "rdf:type".into(),
            message:     format!("BaseVoltage ({:.4} kV) is not defined in Boundary EQ.", nominal_voltage),
            severity:    "sh:Warning".into(),
            description: "The BaseVoltage is not defined in Boundary EQ.".into(),
        });
    }
    v
}

pub fn validate(dataset: &CimDataset) -> Vec<Violation> {
    let mut v = Vec::new();
    v.extend(check_no_tap_changer_controls(dataset));
    v.extend(check_no_regulating_controls(dataset));
    v.extend(check_no_shunt_compensators(dataset));
    v.extend(check_substation_has_no_voltage_levels(dataset));
    v.extend(check_control_area_has_no_children(dataset));
    v.extend(check_no_locations_for_conductors(dataset));
    v.extend(check_ac_line_segment_xr_ratio(dataset));
    v.extend(check_base_voltage_duplicate_nominal_voltage(dataset));
    v.extend(check_power_transformer_ends_same_nominal_voltage(dataset));
    v.extend(check_connectivity_node_open_ended(dataset));
    v.extend(check_disconnector_cross_voltage_level(dataset));
    v.extend(check_conform_load_cross_container(dataset));
    v.extend(check_regulating_control_target_voltage_mismatch(dataset));
    v
}

fn count_by_type(dataset: &CimDataset, type_name: &str) -> usize {
    dataset.by_type.get(type_name).map_or(0, |v| v.len())
}

fn check_no_tap_changer_controls(dataset: &CimDataset) -> Vec<Violation> {
    if count_by_type(dataset, "TapChangerControl") > 0 || count_by_type(dataset, "PowerTransformer") == 0 {
        return Vec::new();
    }
    vec![Violation {
        object_id: "global".into(),
        rule_id:   "quality:PowerTransformer.noTapChangerControl".into(),
        name:      "No TapChangerControls found".into(),
        class:     "PowerTransformer".into(),
        property:  "RegulatingControl".into(),
        message:   "No TapChangerControls are found. None of the PowerTransformers are used for voltage regulation.".into(),
        severity:  "sh:Warning".into(),
        description: String::new(),
    }]
}

fn check_no_regulating_controls(dataset: &CimDataset) -> Vec<Violation> {
    let has_rc = count_by_type(dataset, "RegulatingControl") + count_by_type(dataset, "TapChangerControl") > 0;
    let has_equip = count_by_type(dataset, "SynchronousMachine") + count_by_type(dataset, "LinearShuntCompensator")
        + count_by_type(dataset, "NonlinearShuntCompensator") + count_by_type(dataset, "StaticVarCompensator") > 0;
    if has_rc || !has_equip { return Vec::new(); }
    vec![Violation {
        object_id: "global".into(),
        rule_id:   "quality:RegulatingControl.noRegulatingControl".into(),
        name:      "No RegulatingControls found".into(),
        class:     "RegulatingControl".into(),
        property:  "rdf:type".into(),
        message:   "No RegulatingControls are found. None of the RegulatingCondEqs (SynchronousMachine, ShuntCompensator, StaticVarCompensator) are used for voltage regulation.".into(),
        severity:  "sh:Warning".into(),
        description: String::new(),
    }]
}

fn check_no_shunt_compensators(dataset: &CimDataset) -> Vec<Violation> {
    if count_by_type(dataset, "LinearShuntCompensator") + count_by_type(dataset, "NonlinearShuntCompensator") > 0 {
        return Vec::new();
    }
    if count_by_type(dataset, "PowerTransformer") == 0 && count_by_type(dataset, "ACLineSegment") == 0 {
        return Vec::new();
    }
    vec![Violation {
        object_id: "global".into(),
        rule_id:   "quality:ShuntCompensator.notFound".into(),
        name:      "No ShuntCompensator objects found".into(),
        class:     "ShuntCompensator".into(),
        property:  "rdf:type".into(),
        message:   "No ShuntCompensator objects (LinearShuntCompensator, NonlinearShuntCompensator) are found; at least one is expected.".into(),
        severity:  "sh:Warning".into(),
        description: String::new(),
    }]
}

fn check_substation_has_no_voltage_levels(dataset: &CimDataset) -> Vec<Violation> {
    let mut has_vl: HashSet<String> = HashSet::new();
    for mrid in dataset.by_type.get("VoltageLevel").into_iter().flatten() {
        let entry = &dataset.entries[mrid];
        if let Some(vl) = entry.element.as_any().downcast_ref::<cimstructs::VoltageLevel>() {
            if let Some(r) = &vl.substation {
                has_vl.insert(r.mrid.trim_start_matches('#').to_string());
            }
        }
    }
    let mut has_cn: HashSet<String> = HashSet::new();
    for mrid in dataset.by_type.get("ConnectivityNode").into_iter().flatten() {
        let entry = &dataset.entries[mrid];
        if let Some(cn) = entry.element.as_any().downcast_ref::<cimstructs::ConnectivityNode>() {
            if let Some(r) = &cn.connectivity_node_container {
                has_cn.insert(r.mrid.trim_start_matches('#').to_string());
            }
        }
    }
    let mut v = Vec::new();
    for mrid in dataset.by_type.get("Substation").into_iter().flatten() {
        if has_vl.contains(mrid) || has_cn.contains(mrid) { continue; }
        v.push(Violation {
            object_id: mrid.clone(),
            rule_id:   "quality:Substation.noVoltageLevel".into(),
            name:      "Substation has no VoltageLevels".into(),
            class:     "Substation".into(),
            property:  "VoltageLevel".into(),
            message:   "The Substation has no child VoltageLevels and is not referenced by any instance.".into(),
            severity:  "sh:Warning".into(),
            description: String::new(),
        });
    }
    v
}

fn check_control_area_has_no_children(dataset: &CimDataset) -> Vec<Violation> {
    let mut has_cagu: HashSet<String> = HashSet::new();
    for mrid in dataset.by_type.get("ControlAreaGeneratingUnit").into_iter().flatten() {
        let entry = &dataset.entries[mrid];
        if let Some(cagu) = entry.element.as_any().downcast_ref::<cimstructs::ControlAreaGeneratingUnit>() {
            if let Some(r) = &cagu.control_area {
                has_cagu.insert(r.mrid.trim_start_matches('#').to_string());
            }
        }
    }
    let mut has_tf: HashSet<String> = HashSet::new();
    for mrid in dataset.by_type.get("TieFlow").into_iter().flatten() {
        let entry = &dataset.entries[mrid];
        if let Some(tf) = entry.element.as_any().downcast_ref::<cimstructs::TieFlow>() {
            if let Some(r) = &tf.control_area {
                has_tf.insert(r.mrid.trim_start_matches('#').to_string());
            }
        }
    }
    let mut v = Vec::new();
    for mrid in dataset.by_type.get("ControlArea").into_iter().flatten() {
        if has_cagu.contains(mrid) || has_tf.contains(mrid) { continue; }
        v.push(Violation {
            object_id: mrid.clone(),
            rule_id:   "quality:ControlArea.noChildren".into(),
            name:      "ControlArea has no children".into(),
            class:     "ControlArea".into(),
            property:  "ControlAreaGeneratingUnit".into(),
            message:   "The ControlArea has no child instances (no ControlAreaGeneratingUnits and no TieFlows reference it).".into(),
            severity:  "sh:Warning".into(),
            description: String::new(),
        });
    }
    v
}

fn check_no_locations_for_conductors(dataset: &CimDataset) -> Vec<Violation> {
    if count_by_type(dataset, "Location") == 0 { return Vec::new(); }
    let mut covered: HashSet<String> = HashSet::new();
    for mrid in dataset.by_type.get("Location").into_iter().flatten() {
        let entry = &dataset.entries[mrid];
        if let Some(loc) = entry.element.as_any().downcast_ref::<cimstructs::Location>() {
            if let Some(r) = &loc.power_system_resources {
                covered.insert(r.mrid.trim_start_matches('#').to_string());
            }
        }
    }
    let mut v = Vec::new();
    for mrid in dataset.by_type.get("ACLineSegment").into_iter().flatten() {
        if !covered.contains(mrid) {
            v.push(Violation {
                object_id: mrid.clone(),
                rule_id:   "quality:Conductor.noLocation".into(),
                name:      "No Location for ACLineSegment".into(),
                class:     "ACLineSegment".into(),
                property:  "Location".into(),
                message:   "No Location is associated with this ACLineSegment.".into(),
                severity:  "sh:Warning".into(),
                description: String::new(),
            });
        }
    }
    for mrid in dataset.by_type.get("DCLineSegment").into_iter().flatten() {
        if !covered.contains(mrid) {
            v.push(Violation {
                object_id: mrid.clone(),
                rule_id:   "quality:Conductor.noLocation".into(),
                name:      "No Location for DCLineSegment".into(),
                class:     "DCLineSegment".into(),
                property:  "Location".into(),
                message:   "No Location is associated with this DCLineSegment.".into(),
                severity:  "sh:Warning".into(),
                description: String::new(),
            });
        }
    }
    v
}

fn check_ac_line_segment_xr_ratio(dataset: &CimDataset) -> Vec<Violation> {
    const THRESHOLD: f64 = 50.0;
    let mut v = Vec::new();
    for mrid in dataset.by_type.get("ACLineSegment").into_iter().flatten() {
        let entry = &dataset.entries[mrid];
        if let Some(seg) = entry.element.as_any().downcast_ref::<cimstructs::ACLineSegment>() {
            let r = seg.r.unwrap_or(0.0);
            let x = seg.x.unwrap_or(0.0);
            if r == 0.0 || x == 0.0 { continue; }
            let ratio = x / r;
            if ratio > THRESHOLD {
                v.push(Violation {
                    object_id: mrid.clone(),
                    rule_id:   "quality:ACLineSegment.xrRatioTooLarge".into(),
                    name:      "ACLineSegment x/r ratio too large".into(),
                    class:     "ACLineSegment".into(),
                    property:  "ACLineSegment.x".into(),
                    message:   format!("ACLineSegment.x/ACLineSegment.r ratio ({:.4}) exceeds the threshold of {}.", ratio, THRESHOLD),
                    severity:  "sh:Warning".into(),
                    description: String::new(),
                });
            }
        }
    }
    v
}

fn check_base_voltage_duplicate_nominal_voltage(dataset: &CimDataset) -> Vec<Violation> {
    let mut by_voltage: HashMap<u64, Vec<String>> = HashMap::new();
    for mrid in dataset.by_type.get("BaseVoltage").into_iter().flatten() {
        let entry = &dataset.entries[mrid];
        if let Some(bv) = entry.element.as_any().downcast_ref::<cimstructs::BaseVoltage>() {
            let v = bv.nominal_voltage.unwrap_or(0.0);
            let key = v.to_bits();
            by_voltage.entry(key).or_default().push(mrid.clone());
        }
    }
    let mut v = Vec::new();
    for (key, ids) in &by_voltage {
        if ids.len() < 2 { continue; }
        let voltage = f64::from_bits(*key);
        for id in ids {
            v.push(Violation {
                object_id: id.clone(),
                rule_id:   "quality:BaseVoltage.duplicateNominalVoltage".into(),
                name:      "Duplicate BaseVoltage nominalVoltage".into(),
                class:     "BaseVoltage".into(),
                property:  "BaseVoltage.nominalVoltage".into(),
                message:   format!("BaseVoltage.nominalVoltage ({:.4} kV) is shared by {} BaseVoltage instances.", voltage, ids.len()),
                severity:  "sh:Warning".into(),
                description: String::new(),
            });
        }
    }
    v
}

fn check_power_transformer_ends_same_nominal_voltage(dataset: &CimDataset) -> Vec<Violation> {
    let mut ends_by_pt: HashMap<String, Vec<f64>> = HashMap::new();
    for mrid in dataset.by_type.get("PowerTransformerEnd").into_iter().flatten() {
        let entry = &dataset.entries[mrid];
        if let Some(end) = entry.element.as_any().downcast_ref::<cimstructs::PowerTransformerEnd>() {
            if let Some(r) = &end.power_transformer {
                let pt_id = r.mrid.trim_start_matches('#').to_string();
                ends_by_pt.entry(pt_id).or_default().push(end.rated_u.unwrap_or(0.0));
            }
        }
    }
    let mut v = Vec::new();
    for (pt_id, rated_us) in &ends_by_pt {
        if rated_us.len() < 2 { continue; }
        let ref0 = rated_us[0];
        if ref0 == 0.0 { continue; }
        if rated_us.iter().all(|&u| u == ref0) {
            v.push(Violation {
                object_id: pt_id.clone(),
                rule_id:   "quality:PowerTransformer.endsSameNominalVoltage".into(),
                name:      "PowerTransformer ends share nominalVoltage".into(),
                class:     "PowerTransformer".into(),
                property:  "PowerTransformerEnd.ratedU".into(),
                message:   format!("All PowerTransformerEnds have the same ratedU ({:.4} kV); no voltage transformation occurs.", ref0),
                severity:  "sh:Warning".into(),
                description: String::new(),
            });
        }
    }
    v
}

fn check_connectivity_node_open_ended(dataset: &CimDataset) -> Vec<Violation> {
    let mut count: HashMap<String, usize> = HashMap::new();
    for mrid in dataset.by_type.get("Terminal").into_iter().flatten() {
        let entry = &dataset.entries[mrid];
        if let Some(term) = entry.element.as_any().downcast_ref::<cimstructs::Terminal>() {
            if let Some(r) = &term.connectivity_node {
                let cn_id = r.mrid.trim_start_matches('#').to_string();
                *count.entry(cn_id).or_default() += 1;
            }
        }
    }
    let mut v = Vec::new();
    for mrid in dataset.by_type.get("ConnectivityNode").into_iter().flatten() {
        if count.get(mrid).copied().unwrap_or(0) == 1 {
            v.push(Violation {
                object_id: mrid.clone(),
                rule_id:   "quality:ConnectivityNode.openEnded".into(),
                name:      "ConnectivityNode is open-ended".into(),
                class:     "ConnectivityNode".into(),
                property:  "Terminal".into(),
                message:   "The ConnectivityNode is open-ended: only one Terminal is connected to it.".into(),
                severity:  "sh:Warning".into(),
                description: String::new(),
            });
        }
    }
    v
}

fn check_disconnector_cross_voltage_level(dataset: &CimDataset) -> Vec<Violation> {
    // CN → VoltageLevel MRID (only CNs whose container is a VoltageLevel)
    let mut cn_vl: HashMap<String, String> = HashMap::new();
    for mrid in dataset.by_type.get("ConnectivityNode").into_iter().flatten() {
        let entry = &dataset.entries[mrid];
        if let Some(cn) = entry.element.as_any().downcast_ref::<cimstructs::ConnectivityNode>() {
            if let Some(r) = &cn.connectivity_node_container {
                let cont_id = r.mrid.trim_start_matches('#');
                if dataset.by_type.get("VoltageLevel").into_iter().flatten().any(|m| m == cont_id) {
                    cn_vl.insert(mrid.clone(), cont_id.to_string());
                }
            }
        }
    }
    // Equipment → terminal's CN VoltageLevel IDs
    let mut terms_by_equip: HashMap<String, Vec<String>> = HashMap::new();
    for mrid in dataset.by_type.get("Terminal").into_iter().flatten() {
        let entry = &dataset.entries[mrid];
        if let Some(term) = entry.element.as_any().downcast_ref::<cimstructs::Terminal>() {
            if let Some(ce) = &term.conducting_equipment {
                let eq_id = ce.mrid.trim_start_matches('#').to_string();
                if let Some(cn_ref) = &term.connectivity_node {
                    let cn_id = cn_ref.mrid.trim_start_matches('#').to_string();
                    terms_by_equip.entry(eq_id).or_default().push(cn_id);
                }
            }
        }
    }
    let mut v = Vec::new();
    for mrid in dataset.by_type.get("Disconnector").into_iter().flatten() {
        let cns = match terms_by_equip.get(mrid) {
            Some(c) if c.len() >= 2 => c,
            _ => continue,
        };
        let mut vl_ids: HashSet<String> = HashSet::new();
        for cn_id in cns {
            if let Some(vl) = cn_vl.get(cn_id) {
                vl_ids.insert(vl.clone());
            }
        }
        if vl_ids.len() > 1 {
            v.push(Violation {
                object_id: mrid.clone(),
                rule_id:   "quality:Disconnector.crossVoltageLevel".into(),
                name:      "Disconnector spans VoltageLevels".into(),
                class:     "Disconnector".into(),
                property:  "Terminal.ConnectivityNode".into(),
                message:   "The two ConnectivityNodes the Disconnector connects are in different VoltageLevels.".into(),
                severity:  "sh:Warning".into(),
                description: String::new(),
            });
        }
    }
    v
}

fn check_conform_load_cross_container(dataset: &CimDataset) -> Vec<Violation> {
    // CN → container ID
    let mut cn_container: HashMap<String, String> = HashMap::new();
    for mrid in dataset.by_type.get("ConnectivityNode").into_iter().flatten() {
        let entry = &dataset.entries[mrid];
        if let Some(cn) = entry.element.as_any().downcast_ref::<cimstructs::ConnectivityNode>() {
            if let Some(r) = &cn.connectivity_node_container {
                cn_container.insert(mrid.clone(), r.mrid.trim_start_matches('#').to_string());
            }
        }
    }
    // Equipment → terminal list
    let mut terms_by_equip: HashMap<String, Vec<String>> = HashMap::new();
    for mrid in dataset.by_type.get("Terminal").into_iter().flatten() {
        let entry = &dataset.entries[mrid];
        if let Some(term) = entry.element.as_any().downcast_ref::<cimstructs::Terminal>() {
            if let Some(ce) = &term.conducting_equipment {
                let eq_id = ce.mrid.trim_start_matches('#').to_string();
                terms_by_equip.entry(eq_id).or_default().push(mrid.clone());
            }
        }
    }
    let mut v = Vec::new();
    for mrid in dataset.by_type.get("ConformLoad").into_iter().flatten() {
        let entry = &dataset.entries[mrid];
        let load = match entry.element.as_any().downcast_ref::<cimstructs::ConformLoad>() {
            Some(l) => l,
            None => continue,
        };
        let equip_container = match load.base.base.base.base.equipment_container.as_ref() {
            Some(r) => r.mrid.trim_start_matches('#').to_string(),
            None => continue,
        };
        let terms = match terms_by_equip.get(mrid) {
            Some(t) => t,
            None => continue,
        };
        let mut flagged = false;
        for term_id in terms {
            let term_entry = match dataset.entries.get(term_id) { Some(e) => e, None => continue };
            let term = match term_entry.element.as_any().downcast_ref::<cimstructs::Terminal>() { Some(t) => t, None => continue };
            let cn_ref = match &term.connectivity_node { Some(r) => r, None => continue };
            let cn_id = cn_ref.mrid.trim_start_matches('#');
            let cn_cont = match cn_container.get(cn_id) { Some(c) => c, None => continue };
            if cn_cont != &equip_container {
                flagged = true;
                break;
            }
        }
        if flagged {
            v.push(Violation {
                object_id: mrid.clone(),
                rule_id:   "quality:ConformLoad.crossContainer".into(),
                name:      "ConformLoad crosses EquipmentContainer".into(),
                class:     "ConformLoad".into(),
                property:  "EquipmentContainer".into(),
                message:   "The ConformLoad and its connected TopologicalNodes are not contained by the same EquipmentContainer.".into(),
                severity:  "sh:Warning".into(),
                description: String::new(),
            });
        }
    }
    v
}

fn apply_unit_multiplier(value: f64, mult: Option<&cimstructs::base::UriRef>) -> f64 {
    let uri = match mult { Some(r) => &r.uri, None => return value };
    let suffix = if let Some(idx) = uri.rfind(|c| c == '#' || c == '.') {
        &uri[idx+1..]
    } else {
        uri.as_str()
    };
    match suffix {
        "M" => value * 1000.0,
        "G" => value * 1_000_000.0,
        "m" => value / 1000.0,
        _ => value,
    }
}

fn check_regulating_control_target_voltage_mismatch(dataset: &CimDataset) -> Vec<Violation> {
    // cimdecoder's strip_fragment strips every rdf:resource down to its local name (the part
    // after the last '#'), so enum refs are compared against the bare local name, not the
    // full CIM100 namespace URI.
    const VOLTAGE_URI: &str = "RegulatingControlModeKind.voltage";
    const DEV_WARN: f64 = 0.10;

    // CN → nominal voltage: CN → VoltageLevel → BaseVoltage
    let mut cn_nominal_kv: HashMap<String, f64> = HashMap::new();
    for cn_mrid in dataset.by_type.get("ConnectivityNode").into_iter().flatten() {
        let entry = &dataset.entries[cn_mrid];
        let cn = match entry.element.as_any().downcast_ref::<cimstructs::ConnectivityNode>() { Some(c) => c, None => continue };
        let cont_id = match &cn.connectivity_node_container { Some(r) => r.mrid.trim_start_matches('#'), None => continue };
        let vl = match dataset.entries.get(cont_id).and_then(|e| e.element.as_any().downcast_ref::<cimstructs::VoltageLevel>()) { Some(v) => v, None => continue };
        let bv_id = match &vl.base_voltage { Some(r) => r.mrid.trim_start_matches('#'), None => continue };
        let bv = match dataset.entries.get(bv_id).and_then(|e| e.element.as_any().downcast_ref::<cimstructs::BaseVoltage>()) { Some(b) => b, None => continue };
        let nominal = bv.nominal_voltage.unwrap_or(0.0);
        cn_nominal_kv.insert(cn_mrid.clone(), nominal);
    }
    // Terminal → CN
    let mut term_cn: HashMap<String, String> = HashMap::new();
    for mrid in dataset.by_type.get("Terminal").into_iter().flatten() {
        let entry = &dataset.entries[mrid];
        if let Some(term) = entry.element.as_any().downcast_ref::<cimstructs::Terminal>() {
            if let Some(r) = &term.connectivity_node {
                term_cn.insert(mrid.clone(), r.mrid.trim_start_matches('#').to_string());
            }
        }
    }

    let mut v = Vec::new();
    for mrid in dataset.by_type.get("RegulatingControl").into_iter().flatten() {
        let entry = &dataset.entries[mrid];
        let rc = match entry.element.as_any().downcast_ref::<cimstructs::RegulatingControl>() { Some(r) => r, None => continue };
        if rc.mode.as_ref().map_or(true, |m| m.uri != VOLTAGE_URI) { continue; }
        if !rc.enabled.unwrap_or(false) { continue; }
        let term_id = match rc.terminal.as_ref() { Some(r) => r.mrid.trim_start_matches('#'), None => continue };
        let cn_id = match term_cn.get(term_id) { Some(c) => c, None => continue };
        let nominal_kv = match cn_nominal_kv.get(cn_id) { Some(&n) if n != 0.0 => n, _ => continue };
        let target_kv = apply_unit_multiplier(rc.target_value.unwrap_or(0.0), rc.target_value_unit_multiplier.as_ref());
        if target_kv == 0.0 { continue; }
        let deviation = (target_kv - nominal_kv).abs() / nominal_kv;
        if deviation < DEV_WARN { continue; }
        v.push(Violation {
            object_id: mrid.clone(),
            rule_id:   "quality:RegulatingControl.targetVoltageMismatch".into(),
            name:      "RegulatingControl target voltage mismatch".into(),
            class:     "RegulatingControl".into(),
            property:  "RegulatingControl.targetValue".into(),
            message:   format!("RegulatingControl target voltage ({:.4} kV) deviates {:.1}% from the nominal voltage ({:.4} kV) of the regulated node.",
                target_kv, deviation * 100.0, nominal_kv),
            severity:  "sh:Warning".into(),
            description: String::new(),
        });
    }
    v
}
