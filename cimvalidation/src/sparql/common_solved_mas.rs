use std::collections::{HashMap, HashSet};
use cimdecoder::CimDataset;
use crate::Violation;

pub fn validate(dataset: &CimDataset) -> Vec<Violation> {
    let mut v = Vec::new();
    v.extend(check_angle_reference(dataset));
    v.extend(check_dangling_references(dataset));
    v.extend(check_sv_tap_step_position_sync(dataset));
    v.extend(check_sv_shunt_compensator_sections_sync(dataset));
    v.extend(check_state_variables_instantiated(dataset));
    v.extend(check_sv_status_instance(dataset));
    v.extend(check_sv_shunt_compensator_sections_instance(dataset));
    v.extend(check_sv_tap_step_instance(dataset));
    v.extend(check_regulating_control_contradictory(dataset));
    v.extend(check_regulating_control_same_island(dataset));
    v
}

fn build_tn_to_island(dataset: &CimDataset) -> HashMap<String, String> {
    let mut map = HashMap::new();
    for mrid in dataset.by_type.get("TopologicalIsland").into_iter().flatten() {
        let entry = &dataset.entries[mrid];
        if let Some(island) = entry.element.as_any().downcast_ref::<cimstructs::TopologicalIsland>() {
            for tn in &island.topological_nodes {
                map.insert(tn.mrid.trim_start_matches('#').to_string(), mrid.clone());
            }
        }
    }
    map
}

fn check_angle_reference(dataset: &CimDataset) -> Vec<Violation> {
    let mut angle_ref_tns: HashSet<String> = HashSet::new();
    for mrid in dataset.by_type.get("TopologicalIsland").into_iter().flatten() {
        let entry = &dataset.entries[mrid];
        if let Some(island) = entry.element.as_any().downcast_ref::<cimstructs::TopologicalIsland>() {
            if let Some(r) = &island.angle_ref_topological_node {
                angle_ref_tns.insert(r.mrid.trim_start_matches('#').to_string());
            }
        }
    }

    // Find SMs with highest referencePriority (> 0)
    let mut min_priority = i64::MAX;
    let mut highest_prio_sms: Vec<String> = Vec::new();
    for mrid in dataset.by_type.get("SynchronousMachine").into_iter().flatten() {
        let entry = &dataset.entries[mrid];
        if let Some(sm) = entry.element.as_any().downcast_ref::<cimstructs::SynchronousMachine>() {
            let prio = sm.reference_priority.unwrap_or(0);
            if prio <= 0 { continue; }
            if prio < min_priority {
                min_priority = prio;
                highest_prio_sms = vec![mrid.clone()];
            } else if prio == min_priority {
                highest_prio_sms.push(mrid.clone());
            }
        }
    }

    if highest_prio_sms.is_empty() { return Vec::new(); }

    let mut v = Vec::new();
    if highest_prio_sms.len() > 1 {
        v.push(Violation {
            object_id:   "global".into(),
            rule_id:     "sm456:Model-angleReference".into(),
            name:        "C:456:SSH:NA:angleReference".into(),
            class:       "SynchronousMachine".into(),
            property:    "referencePriority".into(),
            message:     "Multiple machines with highest SynchronousMachine.referencePriority found.".into(),
            severity:    "sh:Violation".into(),
            description: String::new(),
        });
    }

    // Build terminal → TN map for SM terminals
    let mut sm_term_tns: HashMap<String, Vec<String>> = HashMap::new();
    for mrid in dataset.by_type.get("Terminal").into_iter().flatten() {
        let entry = &dataset.entries[mrid];
        if let Some(term) = entry.element.as_any().downcast_ref::<cimstructs::Terminal>() {
            if let Some(ce) = &term.conducting_equipment {
                let sm_id = ce.mrid.trim_start_matches('#').to_string();
                if let Some(tn) = &term.topological_node {
                    sm_term_tns.entry(sm_id).or_default().push(tn.mrid.trim_start_matches('#').to_string());
                }
            }
        }
    }

    for sm_id in &highest_prio_sms {
        let found = sm_term_tns.get(sm_id).map_or(false, |tns| tns.iter().any(|tn| angle_ref_tns.contains(tn)));
        if !found {
            v.push(Violation {
                object_id:   sm_id.clone(),
                rule_id:     "sm456:Model-angleReference".into(),
                name:        "C:456:SSH:NA:angleReference".into(),
                class:       "SynchronousMachine".into(),
                property:    "referencePriority".into(),
                message:     "The SynchronousMachine with highest priority is not connected to a TopologicalIsland.AngleRefTopologicalNode.".into(),
                severity:    "sh:Violation".into(),
                description: String::new(),
            });
        }
    }
    v
}

fn check_dangling_references(dataset: &CimDataset) -> Vec<Violation> {
    let mut v = Vec::new();
    for (id, entry) in &dataset.entries {
        let block = entry.element.to_block();
        for (field, val) in &block.fields {
            let refs: Vec<&str> = match val {
                cimstructs::base::FieldValue::Resource(r) => vec![r.as_str()],
                cimstructs::base::FieldValue::ResourceList(rs) => rs.iter().map(|s| s.as_str()).collect(),
                _ => continue,
            };
            for target in refs {
                let target_id = target.trim_start_matches('#');
                if target_id.is_empty() { continue; }
                let is_cim_id = target.starts_with("urn:uuid:")
                    || target.contains("#_")
                    || target.ends_with('#');
                if !is_cim_id { continue; }
                if !dataset.entries.contains_key(target_id) {
                    v.push(Violation {
                        object_id:   id.clone(),
                        rule_id:     "sm600:All-DanglingReferences".into(),
                        name:        "C:600:ALL:NA:FBOD4".into(),
                        class:       block.type_name.clone(),
                        property:    field.clone(),
                        message:     format!("Dangling reference to '{}'.", target_id),
                        severity:    "sh:Violation".into(),
                        description: String::new(),
                    });
                }
            }
        }
    }
    v
}

fn check_state_variables_instantiated(dataset: &CimDataset) -> Vec<Violation> {
    let tn_to_island = build_tn_to_island(dataset);
    let mut v = Vec::new();

    // 1. SvVoltage for each TN in island
    let mut tn_has_sv_voltage: HashSet<String> = HashSet::new();
    for mrid in dataset.by_type.get("SvVoltage").into_iter().flatten() {
        let entry = &dataset.entries[mrid];
        if let Some(svv) = entry.element.as_any().downcast_ref::<cimstructs::SvVoltage>() {
            if let Some(r) = &svv.topological_node {
                tn_has_sv_voltage.insert(r.mrid.trim_start_matches('#').to_string());
            }
        }
    }
    for (tn_id, island_id) in &tn_to_island {
        if !tn_has_sv_voltage.contains(tn_id) {
            v.push(Violation {
                object_id:   tn_id.clone(),
                rule_id:     "sm600:SvVoltage-SV__4".into(),
                name:        "C:600:SV:SvVoltage:SV__4".into(),
                class:       "TopologicalNode".into(),
                property:    "rdf:type".into(),
                message:     format!("SvVoltage is not instantiated for energized TopologicalNode part of island {}.", island_id),
                severity:    "sh:Violation".into(),
                description: String::new(),
            });
        }
    }

    // Terminal → TN index
    let mut term_tns: HashMap<String, String> = HashMap::new();
    let mut equip_tns: HashMap<String, Vec<String>> = HashMap::new();
    for mrid in dataset.by_type.get("Terminal").into_iter().flatten() {
        let entry = &dataset.entries[mrid];
        if let Some(term) = entry.element.as_any().downcast_ref::<cimstructs::Terminal>() {
            if let Some(tn) = &term.topological_node {
                let tn_id = tn.mrid.trim_start_matches('#').to_string();
                term_tns.insert(mrid.clone(), tn_id.clone());
                if let Some(ce) = &term.conducting_equipment {
                    let eq_id = ce.mrid.trim_start_matches('#').to_string();
                    equip_tns.entry(eq_id).or_default().push(tn_id);
                }
            }
        }
    }

    // 2. SvSwitch for energized retained switches
    let mut sw_has_sv_switch: HashSet<String> = HashSet::new();
    for mrid in dataset.by_type.get("SvSwitch").into_iter().flatten() {
        let entry = &dataset.entries[mrid];
        if let Some(svsw) = entry.element.as_any().downcast_ref::<cimstructs::SvSwitch>() {
            if let Some(r) = &svsw.switch {
                sw_has_sv_switch.insert(r.mrid.trim_start_matches('#').to_string());
            }
        }
    }
    for mrid in dataset.by_type.get("Switch").into_iter().flatten() {
        let entry = &dataset.entries[mrid];
        if let Some(sw) = entry.element.as_any().downcast_ref::<cimstructs::Switch>() {
            if !sw.retained.unwrap_or(false) { continue; }
            if !sw.base.base.in_service.unwrap_or(false) { continue; }
            let energized = equip_tns.get(mrid).map_or(false, |tns| tns.iter().any(|tn| tn_to_island.contains_key(tn)));
            if !energized { continue; }
            if !sw_has_sv_switch.contains(mrid) {
                v.push(Violation {
                    object_id:   mrid.clone(),
                    rule_id:     "sm600:SvSwitch-SV__4".into(),
                    name:        "C:600:SV:SvSwitch:SV__4".into(),
                    class:       "Switch".into(),
                    property:    "rdf:type".into(),
                    message:     "SvSwitch not instantiated for energized retained Switch.".into(),
                    severity:    "sh:Violation".into(),
                    description: String::new(),
                });
            }
        }
    }

    // 3. SvStatus for all energized ConductingEquipment
    let mut ce_has_sv_status: HashSet<String> = HashSet::new();
    for mrid in dataset.by_type.get("SvStatus").into_iter().flatten() {
        let entry = &dataset.entries[mrid];
        if let Some(svs) = entry.element.as_any().downcast_ref::<cimstructs::SvStatus>() {
            if let Some(r) = &svs.conducting_equipment {
                ce_has_sv_status.insert(r.mrid.trim_start_matches('#').to_string());
            }
        }
    }
    for (eq_id, tns) in &equip_tns {
        let energized = tns.iter().any(|tn| tn_to_island.contains_key(tn));
        if !energized { continue; }
        if !ce_has_sv_status.contains(eq_id) {
            let type_name = dataset.entries.get(eq_id).map_or("ConductingEquipment", |e| e.element.type_name());
            v.push(Violation {
                object_id:   eq_id.clone(),
                rule_id:     "sm600:SvStatus-SV__4".into(),
                name:        "C:600:SV:SvStatus:SV__4".into(),
                class:       type_name.to_string(),
                property:    "rdf:type".into(),
                message:     "SvStatus is not instantiated for energized ConductingEquipment.".into(),
                severity:    "sh:Violation".into(),
                description: String::new(),
            });
        }
    }
    v
}

fn check_regulating_control_contradictory(dataset: &CimDataset) -> Vec<Violation> {
    // group by (termID, modeURI) → Vec<(rc_id, target_value)>
    let mut groups: HashMap<(String, String), Vec<(String, f64)>> = HashMap::new();
    for mrid in dataset.by_type.get("RegulatingControl").into_iter().flatten() {
        let entry = &dataset.entries[mrid];
        let rc = match entry.element.as_any().downcast_ref::<cimstructs::RegulatingControl>() { Some(r) => r, None => continue };
        if !rc.enabled.unwrap_or(false) { continue; }
        let term_id = match &rc.terminal { Some(r) => r.mrid.trim_start_matches('#').to_string(), None => continue };
        let mode_uri = match &rc.mode { Some(r) => r.uri.clone(), None => continue };
        let target = rc.target_value.unwrap_or(0.0);
        groups.entry((term_id, mode_uri)).or_default().push((mrid.clone(), target));
    }
    let mut v = Vec::new();
    for ((_, _), pairs) in &groups {
        if pairs.len() < 2 { continue; }
        let mut sorted = pairs.clone();
        sorted.sort_by(|a, b| a.0.cmp(&b.0));
        let val0 = sorted[0].1;
        for (rc_id, target) in &sorted[1..] {
            if *target != val0 {
                v.push(Violation {
                    object_id:   rc_id.clone(),
                    rule_id:     "sm6002:RegulatingControl-samePoint".into(),
                    name:        "C:452:EQ:RegulatingControl:samePoint".into(),
                    class:       "RegulatingControl".into(),
                    property:    "RegulatingControl.targetValue".into(),
                    message:     format!("Enabled RegulatingControl-s of the same type associated with the same TopologicalNode have different target values. RegulatingControl ID: {}.", rc_id),
                    severity:    "sh:Violation".into(),
                    description: String::new(),
                });
            }
        }
    }
    v
}

fn check_sv_shunt_compensator_sections_sync(dataset: &CimDataset) -> Vec<Violation> {
    // SvStatus lookup: CE id → in_service
    let mut sv_status_in_service: HashMap<String, bool> = HashMap::new();
    for mrid in dataset.by_type.get("SvStatus").into_iter().flatten() {
        let entry = &dataset.entries[mrid];
        if let Some(svs) = entry.element.as_any().downcast_ref::<cimstructs::SvStatus>() {
            if let Some(r) = &svs.conducting_equipment {
                let ce_id = r.mrid.trim_start_matches('#').to_string();
                sv_status_in_service.insert(ce_id, svs.in_service.unwrap_or(false));
            }
        }
    }

    let mut v = Vec::new();
    for mrid in dataset.by_type.get("SvShuntCompensatorSections").into_iter().flatten() {
        let entry = &dataset.entries[mrid];
        let svsc = match entry.element.as_any().downcast_ref::<cimstructs::SvShuntCompensatorSections>() { Some(s) => s, None => continue };
        let sc_id = match &svsc.shunt_compensator { Some(r) => r.mrid.trim_start_matches('#'), None => continue };
        let sv_sections = svsc.sections.unwrap_or(0.0);

        let sc_entry = match dataset.entries.get(sc_id) { Some(e) => e, None => continue };
        let (control_enabled, rc_id, sections, type_name) =
            if let Some(lsc) = sc_entry.element.as_any().downcast_ref::<cimstructs::LinearShuntCompensator>() {
                (lsc.base.base.control_enabled.unwrap_or(false),
                 lsc.base.base.regulating_control.as_ref().map(|r| r.mrid.trim_start_matches('#').to_string()),
                 lsc.base.sections.unwrap_or(0.0),
                 "LinearShuntCompensator")
            } else if let Some(nsc) = sc_entry.element.as_any().downcast_ref::<cimstructs::NonlinearShuntCompensator>() {
                (nsc.base.base.control_enabled.unwrap_or(false),
                 nsc.base.base.regulating_control.as_ref().map(|r| r.mrid.trim_start_matches('#').to_string()),
                 nsc.base.sections.unwrap_or(0.0),
                 "NonlinearShuntCompensator")
            } else {
                continue
            };

        let in_service = sv_status_in_service.get(sc_id).copied().unwrap_or(false);
        if !in_service { continue; }

        let rc_enabled = rc_id.as_deref()
            .and_then(|id| dataset.entries.get(id))
            .and_then(|e| e.element.as_any().downcast_ref::<cimstructs::RegulatingControl>())
            .map_or(true, |rc| rc.enabled.unwrap_or(false));

        if !control_enabled || !rc_enabled {
            if sv_sections != sections {
                v.push(Violation {
                    object_id:   sc_id.to_string(),
                    rule_id:     "sm600:SvShuntCompensatorSections.sections-SV__4".into(),
                    name:        "C:600:SV:SvShuntCompensatorSections.sections:SV__4".into(),
                    class:       type_name.to_string(),
                    property:    "ShuntCompensator.sections".into(),
                    message:     format!("SvShuntCompensatorSections.sections ({}) is not the same as ShuntCompensator.sections ({}) for non-regulating ShuntCompensator.", sv_sections, sections),
                    severity:    "sh:Violation".into(),
                    description: String::new(),
                });
            }
        }
    }
    v
}

fn check_sv_tap_step_position_sync(dataset: &CimDataset) -> Vec<Violation> {
    let mut v = Vec::new();
    for mrid in dataset.by_type.get("SvTapStep").into_iter().flatten() {
        let entry = &dataset.entries[mrid];
        let svts = match entry.element.as_any().downcast_ref::<cimstructs::SvTapStep>() { Some(s) => s, None => continue };
        let tc_id = match &svts.tap_changer { Some(r) => r.mrid.trim_start_matches('#'), None => continue };
        let position = svts.position.unwrap_or(0.0);
        let tc_entry = match dataset.entries.get(tc_id) { Some(e) => e, None => continue };
        let (control_enabled, tcc_id, step, type_name) = match get_tap_changer_info(tc_entry) { Some(i) => i, None => continue };
        let rc_enabled = tcc_id.as_deref()
            .and_then(|id| dataset.entries.get(id))
            .and_then(|e| e.element.as_any().downcast_ref::<cimstructs::TapChangerControl>())
            .map_or(true, |tcc| tcc.base.enabled.unwrap_or(false));
        if !control_enabled || !rc_enabled {
            if position != step {
                v.push(Violation {
                    object_id:   tc_id.to_string(),
                    rule_id:     "sm600:SvTapStep.position-SV__4".into(),
                    name:        "C:600:SV:SvTapStep.position:SV__4".into(),
                    class:       type_name.to_string(),
                    property:    "TapChanger.step".into(),
                    message:     format!("SvTapStep.position ({}) is not the same as TapChanger.step ({}) for non-regulating TapChanger.", position, step),
                    severity:    "sh:Violation".into(),
                    description: String::new(),
                });
            }
        }
    }
    v
}

fn get_tap_changer_info(entry: &cimdecoder::CimEntry) -> Option<(bool, Option<String>, f64, &'static str)> {
    if let Some(tc) = entry.element.as_any().downcast_ref::<cimstructs::RatioTapChanger>() {
        let tcc = tc.base.tap_changer_control.as_ref().map(|r| r.mrid.trim_start_matches('#').to_string());
        return Some((tc.base.control_enabled.unwrap_or(false), tcc, tc.base.step.unwrap_or(0.0), "RatioTapChanger"));
    }
    if let Some(tc) = entry.element.as_any().downcast_ref::<cimstructs::PhaseTapChangerLinear>() {
        let tcc = tc.base.base.tap_changer_control.as_ref().map(|r| r.mrid.trim_start_matches('#').to_string());
        return Some((tc.base.base.control_enabled.unwrap_or(false), tcc, tc.base.base.step.unwrap_or(0.0), "PhaseTapChangerLinear"));
    }
    if let Some(tc) = entry.element.as_any().downcast_ref::<cimstructs::PhaseTapChangerSymmetrical>() {
        let tcc = tc.base.base.base.tap_changer_control.as_ref().map(|r| r.mrid.trim_start_matches('#').to_string());
        return Some((tc.base.base.base.control_enabled.unwrap_or(false), tcc, tc.base.base.base.step.unwrap_or(0.0), "PhaseTapChangerSymmetrical"));
    }
    if let Some(tc) = entry.element.as_any().downcast_ref::<cimstructs::PhaseTapChangerAsymmetrical>() {
        let tcc = tc.base.base.base.tap_changer_control.as_ref().map(|r| r.mrid.trim_start_matches('#').to_string());
        return Some((tc.base.base.base.control_enabled.unwrap_or(false), tcc, tc.base.base.base.step.unwrap_or(0.0), "PhaseTapChangerAsymmetrical"));
    }
    if let Some(tc) = entry.element.as_any().downcast_ref::<cimstructs::PhaseTapChangerTabular>() {
        let tcc = tc.base.base.tap_changer_control.as_ref().map(|r| r.mrid.trim_start_matches('#').to_string());
        return Some((tc.base.base.control_enabled.unwrap_or(false), tcc, tc.base.base.step.unwrap_or(0.0), "PhaseTapChangerTabular"));
    }
    None
}

fn check_sv_status_instance(dataset: &CimDataset) -> Vec<Violation> {
    let tn_to_island = build_tn_to_island(dataset);

    let mut equip_tns: HashMap<String, Vec<String>> = HashMap::new();
    for mrid in dataset.by_type.get("Terminal").into_iter().flatten() {
        let entry = &dataset.entries[mrid];
        if let Some(term) = entry.element.as_any().downcast_ref::<cimstructs::Terminal>() {
            if let Some(tn) = &term.topological_node {
                if let Some(ce) = &term.conducting_equipment {
                    let eq_id = ce.mrid.trim_start_matches('#').to_string();
                    equip_tns.entry(eq_id).or_default().push(tn.mrid.trim_start_matches('#').to_string());
                }
            }
        }
    }
    let mut ce_has_sv_status: HashSet<String> = HashSet::new();
    for mrid in dataset.by_type.get("SvStatus").into_iter().flatten() {
        let entry = &dataset.entries[mrid];
        if let Some(svs) = entry.element.as_any().downcast_ref::<cimstructs::SvStatus>() {
            if let Some(r) = &svs.conducting_equipment {
                ce_has_sv_status.insert(r.mrid.trim_start_matches('#').to_string());
            }
        }
    }

    let ce_type_names = ["SynchronousMachine", "AsynchronousMachine", "EnergyConsumer",
        "ConformLoad", "NonConformLoad", "ACLineSegment", "Breaker", "Disconnector",
        "ExternalNetworkInjection", "EquivalentInjection", "PowerTransformer"];

    let mut v = Vec::new();
    for type_name in &ce_type_names {
        for mrid in dataset.by_type.get(*type_name).into_iter().flatten() {
            let energized = equip_tns.get(mrid).map_or(false, |tns| tns.iter().any(|tn| tn_to_island.contains_key(tn)));
            if !energized { continue; }
            if !ce_has_sv_status.contains(mrid) {
                v.push(Violation {
                    object_id:   mrid.clone(),
                    rule_id:     "sm600:SvStatus-SV__4".into(),
                    name:        "C:600:SV:SvStatus:SV__4".into(),
                    class:       type_name.to_string(),
                    property:    "rdf:type".into(),
                    message:     "SvStatus is not instantiated for a ConductingEquipment connected to a TopologicalNode which is referenced by a TopologicalIsland.".into(),
                    severity:    "sh:Violation".into(),
                    description: String::new(),
                });
            }
        }
    }
    v
}

fn check_sv_shunt_compensator_sections_instance(dataset: &CimDataset) -> Vec<Violation> {
    let tn_to_island = build_tn_to_island(dataset);
    let mut equip_tns: HashMap<String, Vec<String>> = HashMap::new();
    for mrid in dataset.by_type.get("Terminal").into_iter().flatten() {
        let entry = &dataset.entries[mrid];
        if let Some(term) = entry.element.as_any().downcast_ref::<cimstructs::Terminal>() {
            if let Some(tn) = &term.topological_node {
                if let Some(ce) = &term.conducting_equipment {
                    let eq_id = ce.mrid.trim_start_matches('#').to_string();
                    equip_tns.entry(eq_id).or_default().push(tn.mrid.trim_start_matches('#').to_string());
                }
            }
        }
    }
    let mut sc_has_sv: HashSet<String> = HashSet::new();
    for mrid in dataset.by_type.get("SvShuntCompensatorSections").into_iter().flatten() {
        let entry = &dataset.entries[mrid];
        if let Some(svsc) = entry.element.as_any().downcast_ref::<cimstructs::SvShuntCompensatorSections>() {
            if let Some(r) = &svsc.shunt_compensator {
                sc_has_sv.insert(r.mrid.trim_start_matches('#').to_string());
            }
        }
    }
    let mut v = Vec::new();
    for type_name in &["LinearShuntCompensator", "NonlinearShuntCompensator"] {
        for mrid in dataset.by_type.get(*type_name).into_iter().flatten() {
            let energized = equip_tns.get(mrid).map_or(false, |tns| tns.iter().any(|tn| tn_to_island.contains_key(tn)));
            if !energized { continue; }
            if !sc_has_sv.contains(mrid) {
                v.push(Violation {
                    object_id:   mrid.clone(),
                    rule_id:     "sm600:SvShuntCompensatorSections-SV__4".into(),
                    name:        "C:600:SV:SvShuntCompensatorSections:SV__4".into(),
                    class:       type_name.to_string(),
                    property:    "rdf:type".into(),
                    message:     "SvShuntCompensatorSections is not instantiated for an energized ShuntCompensator.".into(),
                    severity:    "sh:Violation".into(),
                    description: String::new(),
                });
            }
        }
    }
    v
}

fn check_sv_tap_step_instance(dataset: &CimDataset) -> Vec<Violation> {
    let tn_to_island = build_tn_to_island(dataset);

    // TapChanger → energized via TransformerEnd → Terminal → TN
    let mut te_terminal: HashMap<String, String> = HashMap::new();
    for type_name in &["PowerTransformerEnd"] {
        for mrid in dataset.by_type.get(*type_name).into_iter().flatten() {
            let entry = &dataset.entries[mrid];
            if let Some(pte) = entry.element.as_any().downcast_ref::<cimstructs::PowerTransformerEnd>() {
                if let Some(r) = &pte.base.terminal {
                    te_terminal.insert(mrid.clone(), r.mrid.trim_start_matches('#').to_string());
                }
            }
        }
    }
    let mut term_tn: HashMap<String, String> = HashMap::new();
    for mrid in dataset.by_type.get("Terminal").into_iter().flatten() {
        let entry = &dataset.entries[mrid];
        if let Some(term) = entry.element.as_any().downcast_ref::<cimstructs::Terminal>() {
            if let Some(tn) = &term.topological_node {
                term_tn.insert(mrid.clone(), tn.mrid.trim_start_matches('#').to_string());
            }
        }
    }

    let is_tc_energized = |te_id: &str| -> bool {
        let term_id = match te_terminal.get(te_id) { Some(t) => t, None => return false };
        let tn_id = match term_tn.get(term_id) { Some(t) => t, None => return false };
        tn_to_island.contains_key(tn_id)
    };

    let mut tc_has_sv: HashSet<String> = HashSet::new();
    for mrid in dataset.by_type.get("SvTapStep").into_iter().flatten() {
        let entry = &dataset.entries[mrid];
        if let Some(svts) = entry.element.as_any().downcast_ref::<cimstructs::SvTapStep>() {
            if let Some(r) = &svts.tap_changer {
                tc_has_sv.insert(r.mrid.trim_start_matches('#').to_string());
            }
        }
    }

    let mut v = Vec::new();
    let tc_types = ["RatioTapChanger", "PhaseTapChangerLinear", "PhaseTapChangerSymmetrical",
                    "PhaseTapChangerAsymmetrical", "PhaseTapChangerTabular"];
    for type_name in &tc_types {
        for mrid in dataset.by_type.get(*type_name).into_iter().flatten() {
            // Get TransformerEnd from this tap changer
            let te_id = dataset.entries.get(mrid).and_then(|e| {
                if let Some(tc) = e.element.as_any().downcast_ref::<cimstructs::RatioTapChanger>() {
                    tc.transformer_end.as_ref().map(|r| r.mrid.trim_start_matches('#').to_string())
                } else if let Some(tc) = e.element.as_any().downcast_ref::<cimstructs::PhaseTapChangerLinear>() {
                    tc.base.transformer_end.as_ref().map(|r| r.mrid.trim_start_matches('#').to_string())
                } else if let Some(tc) = e.element.as_any().downcast_ref::<cimstructs::PhaseTapChangerSymmetrical>() {
                    tc.base.base.transformer_end.as_ref().map(|r| r.mrid.trim_start_matches('#').to_string())
                } else if let Some(tc) = e.element.as_any().downcast_ref::<cimstructs::PhaseTapChangerAsymmetrical>() {
                    tc.base.base.transformer_end.as_ref().map(|r| r.mrid.trim_start_matches('#').to_string())
                } else if let Some(tc) = e.element.as_any().downcast_ref::<cimstructs::PhaseTapChangerTabular>() {
                    tc.base.transformer_end.as_ref().map(|r| r.mrid.trim_start_matches('#').to_string())
                } else {
                    None
                }
            });
            let energized = te_id.as_deref().map_or(false, is_tc_energized);
            if !energized { continue; }
            if !tc_has_sv.contains(mrid) {
                v.push(Violation {
                    object_id:   mrid.clone(),
                    rule_id:     "sm600:SvTapStep-SV__4".into(),
                    name:        "C:600:SV:SvTapStep:SV__4".into(),
                    class:       type_name.to_string(),
                    property:    "rdf:type".into(),
                    message:     "SvTapStep is not instantiated for an energized TapChanger.".into(),
                    severity:    "sh:Violation".into(),
                    description: String::new(),
                });
            }
        }
    }
    v
}

fn check_regulating_control_same_island(dataset: &CimDataset) -> Vec<Violation> {
    let tn_to_island = build_tn_to_island(dataset);
    // Terminal → island
    let mut term_to_island: HashMap<String, String> = HashMap::new();
    for mrid in dataset.by_type.get("Terminal").into_iter().flatten() {
        let entry = &dataset.entries[mrid];
        if let Some(term) = entry.element.as_any().downcast_ref::<cimstructs::Terminal>() {
            if let Some(tn) = &term.topological_node {
                let tn_id = tn.mrid.trim_start_matches('#');
                if let Some(island_id) = tn_to_island.get(tn_id) {
                    term_to_island.insert(mrid.clone(), island_id.clone());
                }
            }
        }
    }
    // Equipment → terminal list
    let mut equip_terms: HashMap<String, Vec<String>> = HashMap::new();
    for mrid in dataset.by_type.get("Terminal").into_iter().flatten() {
        let entry = &dataset.entries[mrid];
        if let Some(term) = entry.element.as_any().downcast_ref::<cimstructs::Terminal>() {
            if let Some(ce) = &term.conducting_equipment {
                let eq_id = ce.mrid.trim_start_matches('#').to_string();
                equip_terms.entry(eq_id).or_default().push(mrid.clone());
            }
        }
    }

    let mut v = Vec::new();
    for mrid in dataset.by_type.get("RegulatingControl").into_iter().flatten() {
        let entry = &dataset.entries[mrid];
        let rc = match entry.element.as_any().downcast_ref::<cimstructs::RegulatingControl>() { Some(r) => r, None => continue };
        if !rc.enabled.unwrap_or(false) { continue; }
        let rc_term_id = match &rc.terminal { Some(r) => r.mrid.trim_start_matches('#'), None => continue };
        let rc_island = match term_to_island.get(rc_term_id) { Some(i) => i.clone(), None => continue };

        // Check SynchronousMachines referencing this RC
        for sm_mrid in dataset.by_type.get("SynchronousMachine").into_iter().flatten() {
            let sm_entry = &dataset.entries[sm_mrid];
            if let Some(sm) = sm_entry.element.as_any().downcast_ref::<cimstructs::SynchronousMachine>() {
                let sm_rc = sm.base.base.regulating_control.as_ref().map(|r| r.mrid.trim_start_matches('#'));
                if sm_rc != Some(mrid.as_str()) { continue; }
                for term_id in equip_terms.get(sm_mrid).into_iter().flatten() {
                    if let Some(sm_island) = term_to_island.get(term_id) {
                        if sm_island != &rc_island {
                            v.push(Violation {
                                object_id:   mrid.clone(),
                                rule_id:     "sm6002:RegulatingControl-point".into(),
                                name:        "C:600:EQ:RegulatingControl:point".into(),
                                class:       "RegulatingControl".into(),
                                property:    "rdf:type".into(),
                                message:     format!("The controlled point and the controlling equipment (SynchronousMachine {}) are not located in the same TopologicalIsland.", sm_mrid),
                                severity:    "sh:Violation".into(),
                                description: String::new(),
                            });
                            break;
                        }
                    }
                }
            }
        }

        // Check TapChangers referencing this RC
        let tc_types = ["RatioTapChanger", "PhaseTapChangerLinear", "PhaseTapChangerSymmetrical",
                        "PhaseTapChangerAsymmetrical", "PhaseTapChangerTabular"];
        for tc_type in &tc_types {
            for tc_mrid in dataset.by_type.get(*tc_type).into_iter().flatten() {
                let tc_entry = &dataset.entries[tc_mrid];
                let (tcc_ref, te_id) = if let Some(tc) = tc_entry.element.as_any().downcast_ref::<cimstructs::RatioTapChanger>() {
                    (tc.base.tap_changer_control.as_ref().map(|r| r.mrid.trim_start_matches('#').to_string()),
                     tc.transformer_end.as_ref().map(|r| r.mrid.trim_start_matches('#').to_string()))
                } else if let Some(tc) = tc_entry.element.as_any().downcast_ref::<cimstructs::PhaseTapChangerLinear>() {
                    (tc.base.base.tap_changer_control.as_ref().map(|r| r.mrid.trim_start_matches('#').to_string()),
                     tc.base.transformer_end.as_ref().map(|r| r.mrid.trim_start_matches('#').to_string()))
                } else if let Some(tc) = tc_entry.element.as_any().downcast_ref::<cimstructs::PhaseTapChangerSymmetrical>() {
                    (tc.base.base.base.tap_changer_control.as_ref().map(|r| r.mrid.trim_start_matches('#').to_string()),
                     tc.base.base.transformer_end.as_ref().map(|r| r.mrid.trim_start_matches('#').to_string()))
                } else if let Some(tc) = tc_entry.element.as_any().downcast_ref::<cimstructs::PhaseTapChangerAsymmetrical>() {
                    (tc.base.base.base.tap_changer_control.as_ref().map(|r| r.mrid.trim_start_matches('#').to_string()),
                     tc.base.base.transformer_end.as_ref().map(|r| r.mrid.trim_start_matches('#').to_string()))
                } else if let Some(tc) = tc_entry.element.as_any().downcast_ref::<cimstructs::PhaseTapChangerTabular>() {
                    (tc.base.base.tap_changer_control.as_ref().map(|r| r.mrid.trim_start_matches('#').to_string()),
                     tc.base.transformer_end.as_ref().map(|r| r.mrid.trim_start_matches('#').to_string()))
                } else {
                    continue
                };
                if tcc_ref.as_deref() != Some(mrid.as_str()) { continue; }
                let te_entry = match te_id.as_deref().and_then(|id| dataset.entries.get(id)) { Some(e) => e, None => continue };
                let term_id = if let Some(pte) = te_entry.element.as_any().downcast_ref::<cimstructs::PowerTransformerEnd>() {
                    pte.base.terminal.as_ref().map(|r| r.mrid.trim_start_matches('#').to_string())
                } else {
                    None
                };
                if let Some(t_id) = term_id {
                    if let Some(tc_island) = term_to_island.get(&t_id) {
                        if tc_island != &rc_island {
                            v.push(Violation {
                                object_id:   mrid.clone(),
                                rule_id:     "sm6002:RegulatingControl-point".into(),
                                name:        "C:600:EQ:RegulatingControl:point".into(),
                                class:       "RegulatingControl".into(),
                                property:    "rdf:type".into(),
                                message:     format!("The controlled point and the controlling equipment ({} {}) are not located in the same TopologicalIsland.", tc_type, tc_mrid),
                                severity:    "sh:Violation".into(),
                                description: String::new(),
                            });
                        }
                    }
                }
            }
        }
    }
    v
}
