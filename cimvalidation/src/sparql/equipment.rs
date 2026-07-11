use std::collections::HashMap;
use cimdecoder::CimDataset;
use crate::Violation;

pub fn validate(dataset: &CimDataset) -> Vec<Violation> {
    let mut v = Vec::new();
    v.extend(check_acdcterminal_sequence_numbering(dataset));
    v.extend(check_terminal_phases_consistency_equipment(dataset));
    v.extend(check_conducting_equipment_base_voltage_usage(dataset));
    v.extend(check_power_transformer_end_number_unique(dataset));
    v.extend(check_power_transformer_end_terminal_consistency(dataset));
    v.extend(check_operational_limit_type_duration(dataset));
    v.extend(check_power_transformer_two_winding_end_values(dataset));
    v.extend(check_phase_tap_changer_linear_x_min_consistency(dataset));
    v.extend(check_phase_tap_changer_non_linear_x_min_consistency(dataset));
    v.extend(check_power_transformer_end_rated_s_2winding(dataset));
    v.extend(check_power_transformer_base_voltage_association(dataset));
    v.extend(check_power_transformer_end_r_value_range(dataset));
    v.extend(check_regulating_control_terminal_connectivity_node(dataset));
    v.extend(check_tap_changer_ltc_flag_control(dataset));
    v.extend(check_load_response_characteristic_exponent_model(dataset));
    v.extend(check_nonlinear_shunt_compensator_point_count(dataset));
    v.extend(check_shunt_compensator_nom_u(dataset));
    v.extend(check_phase_tap_changer_asymmetrical_winding_connection_angle(dataset));
    v.extend(check_power_transformer_end_rated_u_value_range(dataset));
    v.extend(check_voltage_limit_patl(dataset));
    v.extend(check_dc_converter_unit_tap_changer_control(dataset));
    v.extend(check_connectivity_node_terminal_phases_consistency(dataset));
    v.extend(check_equipment_aggregate_not_used(dataset));
    v.extend(check_equivalent_branch_r21_usage(dataset));
    v.extend(check_equivalent_branch_x21_usage(dataset));
    v.extend(check_equivalent_injection_regulation_capability(dataset));
    v.extend(check_generating_unit_nominal_p(dataset));
    v.extend(check_control_area_generating_unit_instance(dataset));
    v.extend(check_dc_converter_unit_cs_converter_power_transformer(dataset));
    v.extend(check_limit_kind_patl_number_of_limit_type(dataset));
    v.extend(check_limit_kind_tc_duration(dataset));
    v.extend(check_synchronous_machine_aggregate(dataset));
    v.extend(check_asynchronous_machine_aggregate(dataset));
    v.extend(check_synchronous_machine_control_mode(dataset));
    v.extend(check_static_var_compensator_control_mode(dataset));
    v.extend(check_phase_tap_changer_control_mode(dataset));
    v.extend(check_ratio_tap_changer_control_mode(dataset));
    v.extend(check_shunt_compensator_control_mode(dataset));
    v.extend(check_synchronous_machine_reactive_limits(dataset));
    v.extend(check_synchronous_machine_type_condenser(dataset));
    v.extend(check_vs_capability_curve_count(dataset));
    v.extend(check_vs_capability_curve_y_values(dataset));
    v.extend(check_generating_unit_type_dependency(dataset));
    v.extend(check_curve_data_reactive_capability_limits(dataset));
    v.extend(check_curve_data_reactive_consistency(dataset));
    v.extend(check_synchronous_machine_curve_x_value_consistency(dataset));
    v.extend(check_switch_connection(dataset));
    v.extend(check_operational_limit_set_terminal(dataset));
    v.extend(check_tap_changer_control_remote_q_control(dataset));
    v.extend(check_reactive_capability_curve_x_value_unique(dataset));
    v.extend(check_power_transformer_end_resistance_x_value(dataset));
    v.extend(check_generating_unit_max_operating_p_rated_s(dataset));
    v.extend(check_hydro_generating_unit_energy_conversion_capability(dataset));
    v.extend(check_terminal_connection_same_node(dataset));
    v.extend(check_reactive_capability_curve_reactive_count_p(dataset));
    v.extend(check_reactive_capability_curve_units(dataset));
    v.extend(check_substation_count(dataset));
    v.extend(check_tap_changer_neutral_u_value_range(dataset));
    v
}

fn build_pt_ends(dataset: &CimDataset) -> HashMap<String, Vec<String>> {
    let mut map: HashMap<String, Vec<String>> = HashMap::new();
    for mrid in dataset.by_type.get("PowerTransformerEnd").into_iter().flatten() {
        let entry = &dataset.entries[mrid];
        if let Some(pte) = entry.element.as_any().downcast_ref::<cimstructs::PowerTransformerEnd>() {
            if let Some(pt_ref) = &pte.power_transformer {
                let pt_id = pt_ref.mrid.trim_start_matches('#').to_string();
                map.entry(pt_id).or_default().push(mrid.clone());
            }
        }
    }
    map
}

fn check_acdcterminal_sequence_numbering(dataset: &CimDataset) -> Vec<Violation> {
    let mut equipment_sns: HashMap<String, Vec<i64>> = HashMap::new();
    for mrid in dataset.by_type.get("Terminal").into_iter().flatten() {
        let entry = &dataset.entries[mrid];
        if let Some(term) = entry.element.as_any().downcast_ref::<cimstructs::Terminal>() {
            if let Some(ce) = &term.conducting_equipment {
                let eq_id = ce.mrid.trim_start_matches('#').to_string();
                let sn = term.base.sequence_number.unwrap_or(0);
                equipment_sns.entry(eq_id).or_default().push(sn);
            }
        }
    }
    for mrid in dataset.by_type.get("DCTerminal").into_iter().flatten() {
        let entry = &dataset.entries[mrid];
        if let Some(dct) = entry.element.as_any().downcast_ref::<cimstructs::DCTerminal>() {
            if let Some(ce) = &dct.dc_conducting_equipment {
                let eq_id = ce.mrid.trim_start_matches('#').to_string();
                let sn = dct.base.base.sequence_number.unwrap_or(0);
                equipment_sns.entry(eq_id).or_default().push(sn);
            }
        }
    }
    let mut v = Vec::new();
    for (eq_id, sns) in &equipment_sns {
        let n = sns.len();
        let min_sn = sns.iter().copied().min().unwrap_or(0);
        let sum_sn: i64 = sns.iter().sum();
        let unique: std::collections::HashSet<i64> = sns.iter().copied().collect();
        let failed = unique.len() != n
            || min_sn != 1
            || (n == 1 && sum_sn != 1)
            || (n == 2 && sum_sn != 3)
            || (n == 3 && sum_sn != 6);
        if failed {
            v.push(Violation {
                object_id: eq_id.clone(), rule_id: "equ:ACDCTerminal.sequenceNumber-numbering".into(),
                name: "C:301:EQ:ACDCTerminal.sequenceNumber:numbering".into(), class: "ConductingEquipment".into(),
                property: "ACDCTerminal.sequenceNumber".into(),
                message: "There is no terminal with sequenceNumber=1 or the numbering is not unique.".into(),
                severity: "sh:Violation".into(), description: String::new(),
            });
        }
    }
    v
}

fn check_terminal_phases_consistency_equipment(dataset: &CimDataset) -> Vec<Violation> {
    let mut eq_terms: HashMap<String, HashMap<i64, String>> = HashMap::new();
    for mrid in dataset.by_type.get("Terminal").into_iter().flatten() {
        let entry = &dataset.entries[mrid];
        if let Some(term) = entry.element.as_any().downcast_ref::<cimstructs::Terminal>() {
            if let Some(ce) = &term.conducting_equipment {
                let eq_id = ce.mrid.trim_start_matches('#').to_string();
                let sn = term.base.sequence_number.unwrap_or(0);
                eq_terms.entry(eq_id).or_default().insert(sn, mrid.clone());
            }
        }
    }
    let abcn = "http://iec.ch/TC57/CIM100#PhaseCode.ABCN";
    let n_code = "http://iec.ch/TC57/CIM100#PhaseCode.N";
    let abc = "http://iec.ch/TC57/CIM100#PhaseCode.ABC";
    let mut v = Vec::new();
    for (eq_id, terms) in &eq_terms {
        let t1_id = match terms.get(&1) { Some(id) => id, None => continue };
        let t2_id = match terms.get(&2) { Some(id) => id, None => continue };
        let t1 = match dataset.entries.get(t1_id).and_then(|e| e.element.as_any().downcast_ref::<cimstructs::Terminal>()) { Some(t) => t, None => continue };
        let t2 = match dataset.entries.get(t2_id).and_then(|e| e.element.as_any().downcast_ref::<cimstructs::Terminal>()) { Some(t) => t, None => continue };
        let val1 = t1.phases.as_ref().map(|r| r.uri.as_str()).unwrap_or("");
        let val2 = t2.phases.as_ref().map(|r| r.uri.as_str()).unwrap_or("");
        let failed = if !val1.is_empty() && !val2.is_empty() {
            ((val1 == abcn || val1 == n_code) && val2 != abcn && val2 != n_code)
            || (val1 == abc && val2 != abc)
        } else if !val1.is_empty() && val2.is_empty() {
            val1 == abcn || val1 == n_code
        } else { false };
        if failed {
            v.push(Violation {
                object_id: eq_id.clone(), rule_id: "equ:Terminal.phases-consistencyEquipment".into(),
                name: "C:301:EQ:Terminal.phases:consistencyEquipment".into(), class: "ConductingEquipment".into(),
                property: "Terminal.phases".into(),
                message: format!("The phase codes for terminals of 2-terminal equipment are not consistent. Terminal 1 code:{} Terminal 2 code: {}.", val1, val2),
                severity: "sh:Violation".into(), description: String::new(),
            });
        }
    }
    v
}

fn check_conducting_equipment_base_voltage_usage(dataset: &CimDataset) -> Vec<Violation> {
    let excluded = ["ACLineSegment", "EquivalentBranch", "SeriesCompensator"];
    let mut v = Vec::new();
    for (mrid, entry) in &dataset.entries {
        let type_name = entry.element.type_name();
        if excluded.contains(&type_name) { continue; }
        let block = entry.element.to_block();
        match block.fields.get("ConductingEquipment.BaseVoltage") {
            Some(cimstructs::base::FieldValue::Resource(_)) => {},
            _ => continue,
        }
        let ec_id = match block.fields.get("Equipment.EquipmentContainer") {
            Some(cimstructs::base::FieldValue::Resource(id)) => id.trim_start_matches('#').to_string(),
            _ => continue,
        };
        if let Some(ec_entry) = dataset.entries.get(&ec_id) {
            if ec_entry.element.type_name() == "VoltageLevel" {
                v.push(Violation {
                    object_id: mrid.clone(), rule_id: "equ:ConductingEquipment.BaseVoltage-usage".into(),
                    name: "C:301:EQ:ConductingEquipment.BaseVoltage:usage".into(), class: type_name.to_string(),
                    property: "Equipment.EquipmentContainer".into(),
                    message: "The association ConductingEquipment.BaseVoltage is defined for a ConductingEquipment contained in a VoltageLevel.".into(),
                    severity: "sh:Violation".into(), description: String::new(),
                });
            }
        }
    }
    v
}

fn check_power_transformer_end_number_unique(dataset: &CimDataset) -> Vec<Violation> {
    let pt_ends = build_pt_ends(dataset);
    let mut v = Vec::new();
    for (pt_id, end_ids) in &pt_ends {
        let mut seen: HashMap<i64, bool> = HashMap::new();
        let mut max_rated_u = -1.0f64;
        let mut max_end_num = 0i64;
        let mut duplicate = false;
        for eid in end_ids {
            let entry = match dataset.entries.get(eid) { Some(e) => e, None => continue };
            let pte = match entry.element.as_any().downcast_ref::<cimstructs::PowerTransformerEnd>() { Some(p) => p, None => continue };
            let en = pte.base.end_number.unwrap_or(0);
            if seen.contains_key(&en) { duplicate = true; }
            seen.insert(en, true);
            let ru = pte.rated_u.unwrap_or(0.0);
            if ru > max_rated_u { max_rated_u = ru; max_end_num = en; }
        }
        if duplicate {
            v.push(Violation {
                object_id: pt_id.clone(), rule_id: "equ:TransformerEnd.endNumber-unique".into(),
                name: "C:301:EQ:TransformerEnd.endNumber:unique".into(), class: "PowerTransformer".into(),
                property: "TransformerEnd.endNumber".into(),
                message: "The PowerTransformer has TransformerEnd.endNumber which is not unique.".into(),
                severity: "sh:Violation".into(), description: String::new(),
            });
        } else if max_rated_u > 0.0 && max_end_num != 1 {
            let found_max_at_1 = end_ids.iter().any(|eid| {
                dataset.entries.get(eid).and_then(|e| e.element.as_any().downcast_ref::<cimstructs::PowerTransformerEnd>())
                    .map(|p| p.rated_u.unwrap_or(0.0) == max_rated_u && p.base.end_number.unwrap_or(0) == 1)
                    .unwrap_or(false)
            });
            if !found_max_at_1 {
                v.push(Violation {
                    object_id: pt_id.clone(), rule_id: "equ:TransformerEnd.endNumber-unique".into(),
                    name: "C:301:EQ:TransformerEnd.endNumber:unique".into(), class: "PowerTransformer".into(),
                    property: "TransformerEnd.endNumber".into(),
                    message: "The PowerTransformerEnd with endNumber 1 is not the highest voltage winding.".into(),
                    severity: "sh:Violation".into(), description: String::new(),
                });
            }
        }
    }
    v
}

fn check_power_transformer_end_terminal_consistency(dataset: &CimDataset) -> Vec<Violation> {
    let mut v = Vec::new();
    for mrid in dataset.by_type.get("PowerTransformerEnd").into_iter().flatten() {
        let entry = &dataset.entries[mrid];
        let pte = match entry.element.as_any().downcast_ref::<cimstructs::PowerTransformerEnd>() { Some(p) => p, None => continue };
        let term_id = match &pte.base.terminal { Some(r) => r.mrid.trim_start_matches('#').to_string(), None => continue };
        let pt_id = match &pte.power_transformer { Some(r) => r.mrid.trim_start_matches('#').to_string(), None => continue };
        let term = match dataset.entries.get(&term_id).and_then(|e| e.element.as_any().downcast_ref::<cimstructs::Terminal>()) { Some(t) => t, None => continue };
        let term_pt_id = match &term.conducting_equipment { Some(r) => r.mrid.trim_start_matches('#'), None => continue };
        if term_pt_id != pt_id.as_str() {
            v.push(Violation {
                object_id: mrid.clone(), rule_id: "equ:PowerTransformerEnd-terminalConsistency".into(),
                name: "C:301:EQ:PowerTransformerEnd:terminalConsistency".into(), class: "PowerTransformerEnd".into(),
                property: "TransformerEnd.Terminal".into(),
                message: "The Terminal referenced by TransformerEnd.Terminal points to a PowerTransformer which is different than the referenced element via PowerTransformerEnd.PowerTransformer.".into(),
                severity: "sh:Violation".into(), description: String::new(),
            });
        }
    }
    v
}

fn check_operational_limit_type_duration(dataset: &CimDataset) -> Vec<Violation> {
    let mut v = Vec::new();
    for mrid in dataset.by_type.get("OperationalLimitType").into_iter().flatten() {
        let entry = &dataset.entries[mrid];
        let olt = match entry.element.as_any().downcast_ref::<cimstructs::OperationalLimitType>() { Some(o) => o, None => continue };
        let is_inf = olt.is_infinite_duration.unwrap_or(false);
        let dur = olt.acceptable_duration.unwrap_or(0.0);
        if is_inf && dur != 0.0 {
            v.push(Violation {
                object_id: mrid.clone(), rule_id: "equ:OperationalLimitType.acceptableDuration-usage".into(),
                name: "C:301:EQ:OperationalLimitType.acceptableDuration:usage".into(), class: "OperationalLimitType".into(),
                property: "OperationalLimitType.acceptableDuration".into(),
                message: "The attribute acceptableDuration is present and isInfiniteDuration is set to true.".into(),
                severity: "sh:Violation".into(), description: String::new(),
            });
        }
        if !is_inf && dur == 0.0 {
            v.push(Violation {
                object_id: mrid.clone(), rule_id: "equ:OperationalLimitType.isInfiniteDuration-usage".into(),
                name: "C:301:EQ:OperationalLimitType.isInfiniteDuration:usage".into(), class: "OperationalLimitType".into(),
                property: "OperationalLimitType.acceptableDuration".into(),
                message: "The attribute acceptableDuration is not present when isInfiniteDuration is set to false.".into(),
                severity: "sh:Violation".into(), description: String::new(),
            });
        }
    }
    v
}

fn check_power_transformer_two_winding_end_values(dataset: &CimDataset) -> Vec<Violation> {
    let pt_ends = build_pt_ends(dataset);
    let mut v = Vec::new();
    for (pt_id, end_ids) in &pt_ends {
        if end_ids.len() != 2 { continue; }
        for eid in end_ids {
            let pte = match dataset.entries.get(eid).and_then(|e| e.element.as_any().downcast_ref::<cimstructs::PowerTransformerEnd>()) { Some(p) => p, None => continue };
            if pte.base.end_number.unwrap_or(0) == 2 {
                let r = pte.r.unwrap_or(0.0);
                let r0 = pte.r0.unwrap_or(0.0);
                let x = pte.x.unwrap_or(0.0);
                let x0 = pte.x0.unwrap_or(0.0);
                if r != 0.0 || r0 != 0.0 || x != 0.0 || x0 != 0.0 {
                    v.push(Violation {
                        object_id: pt_id.clone(), rule_id: "equ:PowerTransformerEnd-secondWindingValues".into(),
                        name: "C:301:EQ:PowerTransformerEnd:secondWindingValues".into(), class: "PowerTransformer".into(),
                        property: "PowerTransformerEnd-secondWindingValues".into(),
                        message: format!("Non-zero values for the PowerTransformerEnd with TransformerEnd.endNumber=2 (R={r}, R0={r0}, X={x}, X0={x0}) for a two Terminal PowerTransformer."),
                        severity: "sh:Violation".into(), description: String::new(),
                    });
                }
            }
        }
    }
    v
}

fn check_phase_tap_changer_linear_x_min_consistency(dataset: &CimDataset) -> Vec<Violation> {
    let mut v = Vec::new();
    for mrid in dataset.by_type.get("PhaseTapChangerLinear").into_iter().flatten() {
        let entry = &dataset.entries[mrid];
        let ptcl = match entry.element.as_any().downcast_ref::<cimstructs::PhaseTapChangerLinear>() { Some(p) => p, None => continue };
        let x_min = match ptcl.x_min { Some(x) => x, None => continue };
        let te_id = match &ptcl.base.transformer_end { Some(r) => r.mrid.trim_start_matches('#').to_string(), None => continue };
        let pte = match dataset.entries.get(&te_id).and_then(|e| e.element.as_any().downcast_ref::<cimstructs::PowerTransformerEnd>()) { Some(p) => p, None => continue };
        let end_x = pte.x.unwrap_or(0.0);
        if x_min != end_x {
            v.push(Violation {
                object_id: mrid.clone(), rule_id: "equ:PhaseTapChangerLinear.xMin-valueRangePair".into(),
                name: "C:301:EQ:PhaseTapChangerLinear.xMin:valueRangePair".into(), class: "PhaseTapChangerLinear".into(),
                property: "PhaseTapChangerLinear.xMin".into(),
                message: format!("Inconsistency between PowerTransformerEnd.x ({end_x}) and PhaseTapChangerLinear.xMin ({x_min})."),
                severity: "sh:Violation".into(), description: String::new(),
            });
        }
    }
    v
}

fn check_phase_tap_changer_non_linear_x_min_consistency(dataset: &CimDataset) -> Vec<Violation> {
    let mut v = Vec::new();
    for type_name in &["PhaseTapChangerNonLinear", "PhaseTapChangerAsymmetrical", "PhaseTapChangerSymmetrical"] {
        for mrid in dataset.by_type.get(*type_name).into_iter().flatten() {
            let entry = &dataset.entries[mrid];
            let (x_min, te_id) = if let Some(o) = entry.element.as_any().downcast_ref::<cimstructs::PhaseTapChangerNonLinear>() {
                (o.x_min, o.base.transformer_end.as_ref().map(|r| r.mrid.trim_start_matches('#').to_string()))
            } else if let Some(o) = entry.element.as_any().downcast_ref::<cimstructs::PhaseTapChangerAsymmetrical>() {
                (o.base.x_min, o.base.base.transformer_end.as_ref().map(|r| r.mrid.trim_start_matches('#').to_string()))
            } else if let Some(o) = entry.element.as_any().downcast_ref::<cimstructs::PhaseTapChangerSymmetrical>() {
                (o.base.x_min, o.base.base.transformer_end.as_ref().map(|r| r.mrid.trim_start_matches('#').to_string()))
            } else { continue };
            let x_min = match x_min { Some(x) => x, None => continue };
            let te_id = match te_id { Some(id) => id, None => continue };
            let pte = match dataset.entries.get(&te_id).and_then(|e| e.element.as_any().downcast_ref::<cimstructs::PowerTransformerEnd>()) { Some(p) => p, None => continue };
            let end_x = pte.x.unwrap_or(0.0);
            if x_min != end_x {
                v.push(Violation {
                    object_id: mrid.clone(), rule_id: "equ:PhaseTapChangerNonLinear.xMin-valueRangePair".into(),
                    name: "C:301:EQ:PhaseTapChangerNonLinear.xMin:valueRangePair".into(), class: "PhaseTapChangerNonLinear".into(),
                    property: "PhaseTapChangerNonLinear.xMin".into(),
                    message: format!("Inconsistency between PowerTransformerEnd.x ({end_x}) and PhaseTapChangerNonLinear.xMin ({x_min})."),
                    severity: "sh:Violation".into(), description: String::new(),
                });
            }
        }
    }
    v
}

fn check_power_transformer_end_rated_s_2winding(dataset: &CimDataset) -> Vec<Violation> {
    let pt_ends = build_pt_ends(dataset);
    let mut v = Vec::new();
    for (pt_id, end_ids) in &pt_ends {
        if end_ids.len() != 2 { continue; }
        let s0 = dataset.entries.get(&end_ids[0]).and_then(|e| e.element.as_any().downcast_ref::<cimstructs::PowerTransformerEnd>()).and_then(|p| p.rated_s).unwrap_or(0.0);
        let s1 = dataset.entries.get(&end_ids[1]).and_then(|e| e.element.as_any().downcast_ref::<cimstructs::PowerTransformerEnd>()).and_then(|p| p.rated_s).unwrap_or(0.0);
        if s0 != s1 {
            v.push(Violation {
                object_id: pt_id.clone(), rule_id: "equ:PowerTransformerEnd.ratedS-valueRange2winding".into(),
                name: "C:301:EQ:PowerTransformerEnd.ratedS:valueRange2winding".into(), class: "PowerTransformer".into(),
                property: "PowerTransformerEnd.ratedS".into(),
                message: format!("The RatedS value is different for a two-winding transformer. End 1: {s0}, End 2: {s1}."),
                severity: "sh:Violation".into(), description: String::new(),
            });
        }
    }
    v
}

fn check_power_transformer_base_voltage_association(dataset: &CimDataset) -> Vec<Violation> {
    let mut v = Vec::new();
    for mrid in dataset.by_type.get("PowerTransformer").into_iter().flatten() {
        let entry = &dataset.entries[mrid];
        let pt = match entry.element.as_any().downcast_ref::<cimstructs::PowerTransformer>() { Some(p) => p, None => continue };
        if pt.base.base_voltage.is_some() {
            v.push(Violation {
                object_id: mrid.clone(), rule_id: "equ:PowerTransformer-associationNotUsed".into(),
                name: "C:301:EQ:PowerTransformer:associationNotUsed".into(), class: "PowerTransformer".into(),
                property: "ConductingEquipment.BaseVoltage".into(),
                message: "The inherited association ConductingEquipment.BaseVoltage is used.".into(),
                severity: "sh:Violation".into(), description: String::new(),
            });
        }
    }
    v
}

fn check_power_transformer_end_r_value_range(dataset: &CimDataset) -> Vec<Violation> {
    let mut v = Vec::new();
    for mrid in dataset.by_type.get("PowerTransformerEnd").into_iter().flatten() {
        let entry = &dataset.entries[mrid];
        let pte = match entry.element.as_any().downcast_ref::<cimstructs::PowerTransformerEnd>() { Some(p) => p, None => continue };
        let r = pte.r.unwrap_or(0.0);
        if r >= 0.0 { continue; }
        let pt_id = match &pte.power_transformer { Some(r) => r.mrid.trim_start_matches('#').to_string(), None => continue };
        let pt = match dataset.entries.get(&pt_id).and_then(|e| e.element.as_any().downcast_ref::<cimstructs::PowerTransformer>()) { Some(p) => p, None => continue };
        if !pt.base.base.aggregate.unwrap_or(false) {
            v.push(Violation {
                object_id: mrid.clone(), rule_id: "equ:PowerTransformerEnd.r-valueRange".into(),
                name: "C:301:EQ:PowerTransformerEnd.r:valueRange".into(), class: "PowerTransformerEnd".into(),
                property: "PowerTransformerEnd.r".into(),
                message: "The value is negative for a non-equivalent transformer.".into(),
                severity: "sh:Violation".into(), description: String::new(),
            });
        }
    }
    v
}

fn check_regulating_control_terminal_connectivity_node(dataset: &CimDataset) -> Vec<Violation> {
    let mut v = Vec::new();
    for type_name in &["RegulatingControl", "TapChangerControl"] {
        for mrid in dataset.by_type.get(*type_name).into_iter().flatten() {
            let entry = &dataset.entries[mrid];
            let term_ref = if let Some(rc) = entry.element.as_any().downcast_ref::<cimstructs::RegulatingControl>() {
                rc.terminal.as_ref().map(|r| r.mrid.trim_start_matches('#').to_string())
            } else if let Some(tcc) = entry.element.as_any().downcast_ref::<cimstructs::TapChangerControl>() {
                tcc.base.terminal.as_ref().map(|r| r.mrid.trim_start_matches('#').to_string())
            } else { continue };
            let term_id = match term_ref { Some(id) => id, None => continue };
            let term = match dataset.entries.get(&term_id).and_then(|e| e.element.as_any().downcast_ref::<cimstructs::Terminal>()) { Some(t) => t, None => continue };
            if term.connectivity_node.is_none() {
                v.push(Violation {
                    object_id: mrid.clone(), rule_id: "equ:RegulatingControl-terminalConnectivityNode".into(),
                    name: "C:301:EQ:RegulatingControl:terminalConnectivityNode".into(), class: "RegulatingControl".into(),
                    property: "RegulatingControl.Terminal".into(),
                    message: "The Terminal referenced by the RegulatingControl is not associated with a ConnectivityNode.".into(),
                    severity: "sh:Violation".into(), description: String::new(),
                });
            }
        }
    }
    v
}

fn check_tap_changer_ltc_flag_control(dataset: &CimDataset) -> Vec<Violation> {
    let mut v = Vec::new();
    let tc_types = ["RatioTapChanger", "PhaseTapChangerLinear", "PhaseTapChangerNonLinear",
        "PhaseTapChangerTabular", "PhaseTapChangerAsymmetrical", "PhaseTapChangerSymmetrical"];
    for type_name in &tc_types {
        for mrid in dataset.by_type.get(*type_name).into_iter().flatten() {
            let entry = &dataset.entries[mrid];
            let (ltc_flag, has_tcc) = if let Some(o) = entry.element.as_any().downcast_ref::<cimstructs::RatioTapChanger>() {
                (o.base.ltc_flag.unwrap_or(true), o.base.tap_changer_control.is_some())
            } else if let Some(o) = entry.element.as_any().downcast_ref::<cimstructs::PhaseTapChangerLinear>() {
                (o.base.base.ltc_flag.unwrap_or(true), o.base.base.tap_changer_control.is_some())
            } else if let Some(o) = entry.element.as_any().downcast_ref::<cimstructs::PhaseTapChangerNonLinear>() {
                (o.base.base.ltc_flag.unwrap_or(true), o.base.base.tap_changer_control.is_some())
            } else if let Some(o) = entry.element.as_any().downcast_ref::<cimstructs::PhaseTapChangerTabular>() {
                (o.base.base.ltc_flag.unwrap_or(true), o.base.base.tap_changer_control.is_some())
            } else if let Some(o) = entry.element.as_any().downcast_ref::<cimstructs::PhaseTapChangerAsymmetrical>() {
                (o.base.base.base.ltc_flag.unwrap_or(true), o.base.base.base.tap_changer_control.is_some())
            } else if let Some(o) = entry.element.as_any().downcast_ref::<cimstructs::PhaseTapChangerSymmetrical>() {
                (o.base.base.base.ltc_flag.unwrap_or(true), o.base.base.base.tap_changer_control.is_some())
            } else { continue };
            if !ltc_flag && has_tcc {
                v.push(Violation {
                    object_id: mrid.clone(), rule_id: "equ:TapChanger.ltcFlag-tapChangerControl".into(),
                    name: "C:301:EQ:TapChanger.ltcFlag:tapChangerControl".into(), class: "TapChanger".into(),
                    property: "TapChanger.ltcFlag".into(),
                    message: "An artificial tap changer is used to simulate control behaviour in power flow (ltcFlag is false but TapChangerControl is present).".into(),
                    severity: "sh:Violation".into(), description: String::new(),
                });
            }
        }
    }
    v
}

fn check_load_response_characteristic_exponent_model(dataset: &CimDataset) -> Vec<Violation> {
    const RULE_ID: &str = "equ:LoadResponseCharacteristic.exponentModel-exponentCoefficient";
    let mut v = Vec::new();
    for mrid in dataset.by_type.get("LoadResponseCharacteristic").into_iter().flatten() {
        let entry = &dataset.entries[mrid];
        let lrc = match entry.element.as_any().downcast_ref::<cimstructs::LoadResponseCharacteristic>() { Some(l) => l, None => continue };
        let exp = match lrc.exponent_model { Some(e) => e, None => continue };

        let exponent_bound = [
            lrc.p_frequency_exponent.is_some(), lrc.p_voltage_exponent.is_some(),
            lrc.q_frequency_exponent.is_some(), lrc.q_voltage_exponent.is_some(),
        ];
        let coeff_vals = [
            lrc.p_constant_current, lrc.p_constant_impedance, lrc.p_constant_power,
            lrc.q_constant_current, lrc.q_constant_impedance, lrc.q_constant_power,
        ];
        let any_exponent_bound = exponent_bound.iter().any(|b| *b);
        let all_exponent_bound = exponent_bound.iter().all(|b| *b);
        let any_coeff_bound = coeff_vals.iter().any(|c| c.is_some());
        let all_coeff_bound = coeff_vals.iter().all(|c| c.is_some());

        if exp {
            if !all_exponent_bound || any_coeff_bound {
                v.push(Violation {
                    object_id: mrid.clone(), rule_id: RULE_ID.into(),
                    name: "C:301:EQ:LoadResponseCharacteristic.exponentModel:exponent".into(), class: "LoadResponseCharacteristic".into(),
                    property: "LoadResponseCharacteristic.exponentModel".into(),
                    message: "Missing required properties (attributes) for the exponential voltage dependency model, or there is a mixture with coefficient model attributes.".into(),
                    severity: "sh:Violation".into(), description: String::new(),
                });
            }
        } else if any_exponent_bound || !all_coeff_bound {
            v.push(Violation {
                object_id: mrid.clone(), rule_id: RULE_ID.into(),
                name: "C:301:EQ:LoadResponseCharacteristic.exponentModel:coefficient".into(), class: "LoadResponseCharacteristic".into(),
                property: "LoadResponseCharacteristic.exponentModel".into(),
                message: "Missing required properties (attributes) for the coefficient model, or there is a mixture with exponential model attributes.".into(),
                severity: "sh:Violation".into(), description: String::new(),
            });
        } else {
            let p_sum = coeff_vals[0].unwrap() + coeff_vals[1].unwrap() + coeff_vals[2].unwrap();
            let q_sum = coeff_vals[3].unwrap() + coeff_vals[4].unwrap() + coeff_vals[5].unwrap();
            let eps = 1e-6;
            if (p_sum - 1.0).abs() > eps || (q_sum - 1.0).abs() > eps {
                v.push(Violation {
                    object_id: mrid.clone(), rule_id: RULE_ID.into(),
                    name: "C:301:EQ:LoadResponseCharacteristic.exponentModel:coefficientSum".into(), class: "LoadResponseCharacteristic".into(),
                    property: "LoadResponseCharacteristic.exponentModel".into(),
                    message: format!("The sum of coefficients does not equal 1 (P sum: {p_sum}, Q sum: {q_sum})."),
                    severity: "sh:Violation".into(), description: String::new(),
                });
            }
        }
    }
    v
}

fn check_nonlinear_shunt_compensator_point_count(dataset: &CimDataset) -> Vec<Violation> {
    let mut point_count: HashMap<String, i64> = HashMap::new();
    for mrid in dataset.by_type.get("NonlinearShuntCompensatorPoint").into_iter().flatten() {
        let entry = &dataset.entries[mrid];
        if let Some(pt) = entry.element.as_any().downcast_ref::<cimstructs::NonlinearShuntCompensatorPoint>() {
            if let Some(r) = &pt.nonlinear_shunt_compensator {
                let nsc_id = r.mrid.trim_start_matches('#').to_string();
                *point_count.entry(nsc_id).or_default() += 1;
            }
        }
    }
    let mut v = Vec::new();
    for (nsc_id, count) in &point_count {
        let entry = match dataset.entries.get(nsc_id) { Some(e) => e, None => continue };
        let nsc = match entry.element.as_any().downcast_ref::<cimstructs::NonlinearShuntCompensator>() { Some(n) => n, None => continue };
        let max_sec = nsc.base.maximum_sections.unwrap_or(0);
        if max_sec != *count {
            v.push(Violation {
                object_id: nsc_id.clone(), rule_id: "equ:ShuntCompensator.maximumSections-numberOfInstances".into(),
                name: "C:301:EQ:NonlinearShuntCompensatorPoint:numberOfInstances".into(), class: "NonlinearShuntCompensator".into(),
                property: "ShuntCompensator.maximumSections".into(),
                message: format!("The number of NonlinearShuntCompenstorPoint instances ({count}) does not equal to maximumSections ({max_sec})."),
                severity: "sh:Violation".into(), description: String::new(),
            });
        }
    }
    v
}

fn check_shunt_compensator_nom_u(dataset: &CimDataset) -> Vec<Violation> {
    let mut v = Vec::new();
    for (type_name, get_nom_u, get_ec) in [
        ("LinearShuntCompensator", |e: &cimdecoder::CimEntry| e.element.as_any().downcast_ref::<cimstructs::LinearShuntCompensator>().and_then(|o| o.base.nom_u),
         |e: &cimdecoder::CimEntry| e.element.as_any().downcast_ref::<cimstructs::LinearShuntCompensator>().and_then(|o| o.base.base.base.base.base.equipment_container.as_ref().map(|r| r.mrid.trim_start_matches('#').to_string())) as Option<String>),
        ("NonlinearShuntCompensator", |e: &cimdecoder::CimEntry| e.element.as_any().downcast_ref::<cimstructs::NonlinearShuntCompensator>().and_then(|o| o.base.nom_u),
         |e: &cimdecoder::CimEntry| e.element.as_any().downcast_ref::<cimstructs::NonlinearShuntCompensator>().and_then(|o| o.base.base.base.base.base.equipment_container.as_ref().map(|r| r.mrid.trim_start_matches('#').to_string())) as Option<String>),
    ] as [(&str, fn(&cimdecoder::CimEntry) -> Option<f64>, fn(&cimdecoder::CimEntry) -> Option<String>); 2] {
        for mrid in dataset.by_type.get(type_name).into_iter().flatten() {
            let entry = &dataset.entries[mrid];
            let nom_u = match get_nom_u(entry) { Some(u) => u, None => continue };
            let ec_id = match get_ec(entry) { Some(id) => id, None => continue };
            let ec_entry = match dataset.entries.get(&ec_id) { Some(e) => e, None => continue };
            let vl = match ec_entry.element.as_any().downcast_ref::<cimstructs::VoltageLevel>() { Some(vl) => vl, None => continue };
            let bv_id = match &vl.base_voltage { Some(r) => r.mrid.trim_start_matches('#').to_string(), None => continue };
            let bv = match dataset.entries.get(&bv_id).and_then(|e| e.element.as_any().downcast_ref::<cimstructs::BaseVoltage>()) { Some(b) => b, None => continue };
            let nom_v = match bv.nominal_voltage { Some(v) => v, None => continue };
            if nom_u < 0.9 * nom_v || nom_u > 1.1 * nom_v {
                v.push(Violation {
                    object_id: mrid.clone(), rule_id: "equ:ShuntCompensator.nomU-nominalVoltageDifference".into(),
                    name: "C:301:EQ:ShuntCompensator.nomU:nominalVoltageDifference".into(), class: "ShuntCompensator".into(),
                    property: "ShuntCompensator.nomU".into(),
                    message: format!("The value nomU ({nom_u}) differs with more than 10% of the nominal voltage ({nom_v})."),
                    severity: "sh:Warning".into(), description: String::new(),
                });
            }
        }
    }
    v
}

fn check_phase_tap_changer_asymmetrical_winding_connection_angle(dataset: &CimDataset) -> Vec<Violation> {
    let mut v = Vec::new();
    for mrid in dataset.by_type.get("PhaseTapChangerAsymmetrical").into_iter().flatten() {
        let entry = &dataset.entries[mrid];
        let ptca = match entry.element.as_any().downcast_ref::<cimstructs::PhaseTapChangerAsymmetrical>() { Some(p) => p, None => continue };
        let val = match ptca.winding_connection_angle { Some(w) => w, None => continue };
        let is_multiple_of_30 = (val as i64) % 30 == 0 && val == (val as i64) as f64;
        let in_range = val >= -150.0 && val <= 150.0 && val != 0.0;
        if !is_multiple_of_30 || !in_range {
            v.push(Violation {
                object_id: mrid.clone(), rule_id: "equ:PhaseTapChangerAsymmetrical.windingConnectionAngle-valueRange".into(),
                name: "C:301:EQ:PhaseTapChangerAsymmetrical.windingConnectionAngle:valueRange".into(), class: "PhaseTapChangerAsymmetrical".into(),
                property: "PhaseTapChangerAsymmetrical.windingConnectionAngle".into(),
                message: "The value is not a multiple of 30 degrees in the range of -150 to 150 degrees (excluding 0).".into(),
                severity: "sh:Violation".into(), description: String::new(),
            });
        }
    }
    v
}

fn check_power_transformer_end_rated_u_value_range(dataset: &CimDataset) -> Vec<Violation> {
    let pt_ends = build_pt_ends(dataset);
    let mut v = Vec::new();
    for (pt_id, end_ids) in &pt_ends {
        let mut max_rated_u = -1.0f64;
        let mut end1_rated_u: Option<f64> = None;
        for eid in end_ids {
            let pte = match dataset.entries.get(eid).and_then(|e| e.element.as_any().downcast_ref::<cimstructs::PowerTransformerEnd>()) { Some(p) => p, None => continue };
            let ru = pte.rated_u.unwrap_or(0.0);
            if ru <= 0.0 {
                v.push(Violation {
                    object_id: pt_id.clone(), rule_id: "equ:PowerTransformerEnd.ratedU-valueRange".into(),
                    name: "C:301:EQ:PowerTransformerEnd.ratedU:valueRange".into(), class: "PowerTransformer".into(),
                    property: "PowerTransformerEnd.ratedU".into(),
                    message: format!("The PowerTransformerEnd {} has a non-positive ratedU ({ru}).", pte.base.base.id),
                    severity: "sh:Violation".into(), description: String::new(),
                });
            }
            if pte.base.end_number.unwrap_or(0) == 1 { end1_rated_u = pte.rated_u; }
            if ru > max_rated_u { max_rated_u = ru; }
        }
        if let Some(e1u) = end1_rated_u {
            if e1u < max_rated_u {
                v.push(Violation {
                    object_id: pt_id.clone(), rule_id: "equ:PowerTransformerEnd.ratedU-valueRange".into(),
                    name: "C:301:EQ:PowerTransformerEnd.ratedU:valueRange".into(), class: "PowerTransformer".into(),
                    property: "PowerTransformerEnd.ratedU".into(),
                    message: "The high voltage side (endNumber=1) does not have the highest ratedU.".into(),
                    severity: "sh:Violation".into(), description: String::new(),
                });
            }
        }
    }
    v
}

fn check_voltage_limit_patl(dataset: &CimDataset) -> Vec<Violation> {
    let patl = "http://iec.ch/TC57/CIM100-European#LimitKind.patl";
    let mut v = Vec::new();
    for mrid in dataset.by_type.get("VoltageLimit").into_iter().flatten() {
        let entry = &dataset.entries[mrid];
        let vl = match entry.element.as_any().downcast_ref::<cimstructs::VoltageLimit>() { Some(v) => v, None => continue };
        let olt_id = match &vl.base.operational_limit_type { Some(r) => r.mrid.trim_start_matches('#').to_string(), None => continue };
        let olt = match dataset.entries.get(&olt_id).and_then(|e| e.element.as_any().downcast_ref::<cimstructs::OperationalLimitType>()) { Some(o) => o, None => continue };
        if olt.kind.as_ref().map(|r| r.mrid.as_str()) == Some(patl) {
            v.push(Violation {
                object_id: mrid.clone(), rule_id: "equ:LimitKind.patl-allowedType".into(),
                name: "C:301:EQ:LimitKind.patl:allowedType".into(), class: "VoltageLimit".into(),
                property: "OperationalLimit.OperationalLimitType".into(),
                message: "PATL type is provided for VoltageLimit.".into(),
                severity: "sh:Violation".into(), description: String::new(),
            });
        }
    }
    v
}

fn tc_transformer_end_id(entry: &cimdecoder::CimEntry) -> Option<String> {
    if let Some(o) = entry.element.as_any().downcast_ref::<cimstructs::RatioTapChanger>() {
        return o.transformer_end.as_ref().map(|r| r.mrid.trim_start_matches('#').to_string());
    }
    if let Some(o) = entry.element.as_any().downcast_ref::<cimstructs::PhaseTapChangerLinear>() {
        return o.base.transformer_end.as_ref().map(|r| r.mrid.trim_start_matches('#').to_string());
    }
    if let Some(o) = entry.element.as_any().downcast_ref::<cimstructs::PhaseTapChangerNonLinear>() {
        return o.base.transformer_end.as_ref().map(|r| r.mrid.trim_start_matches('#').to_string());
    }
    if let Some(o) = entry.element.as_any().downcast_ref::<cimstructs::PhaseTapChangerTabular>() {
        return o.base.transformer_end.as_ref().map(|r| r.mrid.trim_start_matches('#').to_string());
    }
    if let Some(o) = entry.element.as_any().downcast_ref::<cimstructs::PhaseTapChangerAsymmetrical>() {
        return o.base.base.transformer_end.as_ref().map(|r| r.mrid.trim_start_matches('#').to_string());
    }
    if let Some(o) = entry.element.as_any().downcast_ref::<cimstructs::PhaseTapChangerSymmetrical>() {
        return o.base.base.transformer_end.as_ref().map(|r| r.mrid.trim_start_matches('#').to_string());
    }
    None
}

fn tc_has_tcc(entry: &cimdecoder::CimEntry) -> bool {
    if let Some(o) = entry.element.as_any().downcast_ref::<cimstructs::RatioTapChanger>() {
        return o.base.tap_changer_control.is_some();
    }
    if let Some(o) = entry.element.as_any().downcast_ref::<cimstructs::PhaseTapChangerLinear>() {
        return o.base.base.tap_changer_control.is_some();
    }
    if let Some(o) = entry.element.as_any().downcast_ref::<cimstructs::PhaseTapChangerNonLinear>() {
        return o.base.base.tap_changer_control.is_some();
    }
    if let Some(o) = entry.element.as_any().downcast_ref::<cimstructs::PhaseTapChangerTabular>() {
        return o.base.base.tap_changer_control.is_some();
    }
    if let Some(o) = entry.element.as_any().downcast_ref::<cimstructs::PhaseTapChangerAsymmetrical>() {
        return o.base.base.base.tap_changer_control.is_some();
    }
    if let Some(o) = entry.element.as_any().downcast_ref::<cimstructs::PhaseTapChangerSymmetrical>() {
        return o.base.base.base.tap_changer_control.is_some();
    }
    false
}

fn check_dc_converter_unit_tap_changer_control(dataset: &CimDataset) -> Vec<Violation> {
    let tc_types = ["RatioTapChanger", "PhaseTapChangerLinear", "PhaseTapChangerNonLinear",
        "PhaseTapChangerTabular", "PhaseTapChangerAsymmetrical", "PhaseTapChangerSymmetrical"];
    let mut v = Vec::new();
    for type_name in &tc_types {
        for mrid in dataset.by_type.get(*type_name).into_iter().flatten() {
            let entry = &dataset.entries[mrid];
            if !tc_has_tcc(entry) { continue; }
            let te_id = match tc_transformer_end_id(entry) { Some(id) => id, None => continue };
            let pte = match dataset.entries.get(&te_id).and_then(|e| e.element.as_any().downcast_ref::<cimstructs::PowerTransformerEnd>()) { Some(p) => p, None => continue };
            let pt_id = match &pte.power_transformer { Some(r) => r.mrid.trim_start_matches('#').to_string(), None => continue };
            let pt = match dataset.entries.get(&pt_id).and_then(|e| e.element.as_any().downcast_ref::<cimstructs::PowerTransformer>()) { Some(p) => p, None => continue };
            let ec_id = match &pt.base.base.equipment_container { Some(r) => r.mrid.trim_start_matches('#').to_string(), None => continue };
            let ec_entry = match dataset.entries.get(&ec_id) { Some(e) => e, None => continue };
            if ec_entry.element.type_name() == "DCConverterUnit" {
                v.push(Violation {
                    object_id: mrid.clone(), rule_id: "equ:DCConverterUnit-tapChangerControl".into(),
                    name: "C:301:EQ:DCConverterUnit:tapChangerControl".into(), class: "TapChanger".into(),
                    property: "TapChanger.TapChangerControl".into(),
                    message: "TapChangerControl is associated to a transformer contained in DCConverterUnit.".into(),
                    severity: "sh:Violation".into(), description: String::new(),
                });
            }
        }
    }
    v
}

fn check_connectivity_node_terminal_phases_consistency(dataset: &CimDataset) -> Vec<Violation> {
    let mut node_terms: HashMap<String, Vec<String>> = HashMap::new();
    for mrid in dataset.by_type.get("Terminal").into_iter().flatten() {
        let entry = &dataset.entries[mrid];
        if let Some(term) = entry.element.as_any().downcast_ref::<cimstructs::Terminal>() {
            if let Some(cn) = &term.connectivity_node {
                let cn_id = cn.mrid.trim_start_matches('#').to_string();
                node_terms.entry(cn_id).or_default().push(mrid.clone());
            }
        }
    }
    let abcn = "http://iec.ch/TC57/CIM100#PhaseCode.ABCN";
    let n_code = "http://iec.ch/TC57/CIM100#PhaseCode.N";
    let abc = "http://iec.ch/TC57/CIM100#PhaseCode.ABC";
    let mut v = Vec::new();
    'outer: for (node_id, term_ids) in &node_terms {
        if term_ids.len() < 2 { continue; }
        for i in 0..term_ids.len() {
            for j in (i + 1)..term_ids.len() {
                let t_i = match dataset.entries.get(&term_ids[i]).and_then(|e| e.element.as_any().downcast_ref::<cimstructs::Terminal>()) { Some(t) => t, None => continue };
                let t_j = match dataset.entries.get(&term_ids[j]).and_then(|e| e.element.as_any().downcast_ref::<cimstructs::Terminal>()) { Some(t) => t, None => continue };
                let val1 = t_i.phases.as_ref().map(|r| r.uri.as_str()).unwrap_or("");
                let val2 = t_j.phases.as_ref().map(|r| r.uri.as_str()).unwrap_or("");
                let failed = if !val1.is_empty() && !val2.is_empty() {
                    ((val1 == abcn || val1 == n_code) && val2 != abcn && val2 != n_code)
                    || (val1 == abc && val2 != abc)
                } else if !val1.is_empty() && val2.is_empty() {
                    val1 == abcn || val1 == n_code
                } else { false };
                if failed {
                    v.push(Violation {
                        object_id: node_id.clone(), rule_id: "equ:Terminal.phases-consistencyConnectivityNode".into(),
                        name: "C:301:EQ:Terminal.phases:consistencyConnectivityNode".into(), class: "ConnectivityNode".into(),
                        property: "Terminal.phases".into(),
                        message: format!("The phase codes for the connected terminals are not consistent. Terminal {} code: {}, Terminal {} code: {}.", term_ids[i], val1, term_ids[j], val2),
                        severity: "sh:Violation".into(), description: String::new(),
                    });
                    continue 'outer;
                }
            }
        }
    }
    v
}

fn check_equipment_aggregate_not_used(dataset: &CimDataset) -> Vec<Violation> {
    let mut v = Vec::new();
    for mrid in dataset.by_type.get("EquivalentBranch").into_iter().flatten() {
        let entry = &dataset.entries[mrid];
        if let Some(eb) = entry.element.as_any().downcast_ref::<cimstructs::EquivalentBranch>() {
            if eb.base.base.base.aggregate.unwrap_or(false) {
                v.push(Violation {
                    object_id: mrid.clone(), rule_id: "equ:Equipment.aggregate-notUsed".into(),
                    name: "C:301:EQ:Equipment.aggregate:notUsed".into(), class: "EquivalentBranch".into(),
                    property: "Equipment.aggregate".into(),
                    message: "Not allowed property (attribute).".into(),
                    severity: "sh:Violation".into(), description: String::new(),
                });
            }
        }
    }
    for mrid in dataset.by_type.get("EquivalentShunt").into_iter().flatten() {
        let entry = &dataset.entries[mrid];
        if let Some(es) = entry.element.as_any().downcast_ref::<cimstructs::EquivalentShunt>() {
            if es.base.base.base.aggregate.unwrap_or(false) {
                v.push(Violation {
                    object_id: mrid.clone(), rule_id: "equ:Equipment.aggregate-notUsed".into(),
                    name: "C:301:EQ:Equipment.aggregate:notUsed".into(), class: "EquivalentShunt".into(),
                    property: "Equipment.aggregate".into(),
                    message: "Not allowed property (attribute).".into(),
                    severity: "sh:Violation".into(), description: String::new(),
                });
            }
        }
    }
    for mrid in dataset.by_type.get("EquivalentInjection").into_iter().flatten() {
        let entry = &dataset.entries[mrid];
        if let Some(ei) = entry.element.as_any().downcast_ref::<cimstructs::EquivalentInjection>() {
            if ei.base.base.base.aggregate.unwrap_or(false) {
                v.push(Violation {
                    object_id: mrid.clone(), rule_id: "equ:Equipment.aggregate-notUsed".into(),
                    name: "C:301:EQ:Equipment.aggregate:notUsed".into(), class: "EquivalentInjection".into(),
                    property: "Equipment.aggregate".into(),
                    message: "Not allowed property (attribute).".into(),
                    severity: "sh:Violation".into(), description: String::new(),
                });
            }
        }
    }
    v
}

fn check_equivalent_branch_r21_usage(dataset: &CimDataset) -> Vec<Violation> {
    let mut v = Vec::new();
    for mrid in dataset.by_type.get("EquivalentBranch").into_iter().flatten() {
        let entry = &dataset.entries[mrid];
        if let Some(eb) = entry.element.as_any().downcast_ref::<cimstructs::EquivalentBranch>() {
            let r21 = eb.r21.unwrap_or(0.0);
            let r = eb.r.unwrap_or(0.0);
            if r21 != 0.0 && r21 != r {
                v.push(Violation {
                    object_id: mrid.clone(), rule_id: "equ:EquivalentBranch.r21-usage".into(),
                    name: "C:301:EQ:EquivalentBranch.r21:usage".into(), class: "EquivalentBranch".into(),
                    property: "EquivalentBranch.r21".into(),
                    message: "Asymmetrical EquivalentBranch is modelled as EquivalentBranch.r is different from EquivalentBranch.r21.".into(),
                    severity: "sh:Info".into(), description: String::new(),
                });
            }
        }
    }
    v
}

fn check_equivalent_branch_x21_usage(dataset: &CimDataset) -> Vec<Violation> {
    let mut v = Vec::new();
    for mrid in dataset.by_type.get("EquivalentBranch").into_iter().flatten() {
        let entry = &dataset.entries[mrid];
        if let Some(eb) = entry.element.as_any().downcast_ref::<cimstructs::EquivalentBranch>() {
            let x21 = eb.x21.unwrap_or(0.0);
            let x = eb.x.unwrap_or(0.0);
            if x21 != 0.0 && x21 != x {
                v.push(Violation {
                    object_id: mrid.clone(), rule_id: "equ:EquivalentBranch.x21-usage".into(),
                    name: "C:301:EQ:EquivalentBranch.x21:usage".into(), class: "EquivalentBranch".into(),
                    property: "EquivalentBranch.x21".into(),
                    message: "Asymmetrical EquivalentBranch is modelled as EquivalentBranch.x is different from EquivalentBranch.x21.".into(),
                    severity: "sh:Info".into(), description: String::new(),
                });
            }
        }
    }
    v
}

fn check_equivalent_injection_regulation_capability(dataset: &CimDataset) -> Vec<Violation> {
    let mut v = Vec::new();
    for mrid in dataset.by_type.get("EquivalentInjection").into_iter().flatten() {
        let entry = &dataset.entries[mrid];
        if let Some(ei) = entry.element.as_any().downcast_ref::<cimstructs::EquivalentInjection>() {
            if ei.reactive_capability_curve.is_some() && !ei.regulation_capability.unwrap_or(false) {
                v.push(Violation {
                    object_id: mrid.clone(), rule_id: "equ:EquivalentInjection.regulationCapability-associatedCurve".into(),
                    name: "C:301:EQ:EquivalentInjection.regulationCapability:associatedCurve".into(), class: "EquivalentInjection".into(),
                    property: "EquivalentInjection.regulationCapability".into(),
                    message: "The value does not allow a ReactiveCapabilityCurve to be associated.".into(),
                    severity: "sh:Violation".into(), description: String::new(),
                });
            }
        }
    }
    v
}

fn check_generating_unit_nominal_p(dataset: &CimDataset) -> Vec<Violation> {
    let mut rated_s_by_gu: HashMap<String, f64> = HashMap::new();
    for type_name in &["SynchronousMachine", "AsynchronousMachine"] {
        for mrid in dataset.by_type.get(*type_name).into_iter().flatten() {
            let entry = &dataset.entries[mrid];
            let (gu_id, rated_s) = if let Some(sm) = entry.element.as_any().downcast_ref::<cimstructs::SynchronousMachine>() {
                (sm.base.generating_unit.as_ref().map(|r| r.mrid.trim_start_matches('#').to_string()), sm.base.rated_s.unwrap_or(0.0))
            } else if let Some(am) = entry.element.as_any().downcast_ref::<cimstructs::AsynchronousMachine>() {
                (am.base.generating_unit.as_ref().map(|r| r.mrid.trim_start_matches('#').to_string()), am.base.rated_s.unwrap_or(0.0))
            } else { continue };
            if let Some(gu_id) = gu_id {
                let e = rated_s_by_gu.entry(gu_id).or_default();
                if rated_s > *e { *e = rated_s; }
            }
        }
    }
    let mut v = Vec::new();
    let gu_types = ["GeneratingUnit", "ThermalGeneratingUnit", "WindGeneratingUnit",
        "HydroGeneratingUnit", "NuclearGeneratingUnit", "SolarGeneratingUnit"];
    for type_name in &gu_types {
        for mrid in dataset.by_type.get(*type_name).into_iter().flatten() {
            let entry = &dataset.entries[mrid];
            let nominal_p = if let Some(gu) = entry.element.as_any().downcast_ref::<cimstructs::GeneratingUnit>() {
                gu.nominal_p
            } else if let Some(o) = entry.element.as_any().downcast_ref::<cimstructs::ThermalGeneratingUnit>() {
                o.base.nominal_p
            } else if let Some(o) = entry.element.as_any().downcast_ref::<cimstructs::HydroGeneratingUnit>() {
                o.base.nominal_p
            } else if let Some(o) = entry.element.as_any().downcast_ref::<cimstructs::WindGeneratingUnit>() {
                o.base.nominal_p
            } else if let Some(o) = entry.element.as_any().downcast_ref::<cimstructs::NuclearGeneratingUnit>() {
                o.base.nominal_p
            } else if let Some(o) = entry.element.as_any().downcast_ref::<cimstructs::SolarGeneratingUnit>() {
                o.base.nominal_p
            } else { continue };
            let np = match nominal_p { Some(n) => n, None => continue };
            let rated_s = match rated_s_by_gu.get(mrid.as_str()) { Some(&r) => r, None => continue };
            if np <= 0.0 || np > rated_s {
                v.push(Violation {
                    object_id: mrid.clone(), rule_id: "equ:GeneratingUnit.nominalP-valueRangePair".into(),
                    name: "C:301:EQ:GeneratingUnit.nominalP:valueRangePair".into(), class: type_name.to_string(),
                    property: "GeneratingUnit.nominalP".into(),
                    message: format!("The value ({np}) is either negative, zero or greater than RotatingMachine.ratedS ({rated_s})."),
                    severity: "sh:Violation".into(), description: String::new(),
                });
            }
        }
    }
    v
}

fn check_control_area_generating_unit_instance(dataset: &CimDataset) -> Vec<Violation> {
    let mut seen: std::collections::HashSet<(String, String)> = std::collections::HashSet::new();
    let mut duplicates: std::collections::HashSet<String> = std::collections::HashSet::new();
    for mrid in dataset.by_type.get("ControlAreaGeneratingUnit").into_iter().flatten() {
        let entry = &dataset.entries[mrid];
        if let Some(cagu) = entry.element.as_any().downcast_ref::<cimstructs::ControlAreaGeneratingUnit>() {
            let ca_id = match &cagu.control_area { Some(r) => r.mrid.trim_start_matches('#').to_string(), None => continue };
            let gu_id = match &cagu.generating_unit { Some(r) => r.mrid.trim_start_matches('#').to_string(), None => continue };
            let key = (ca_id, gu_id.clone());
            if !seen.insert(key) { duplicates.insert(gu_id); }
        }
    }
    duplicates.into_iter().map(|gu_id| Violation {
        object_id: gu_id, rule_id: "equ:ControlAreaGeneratingUnit.GeneratingUnit-instance".into(),
        name: "C:301:EQ:ControlAreaGeneratingUnit.GeneratingUnit:instance".into(), class: "GeneratingUnit".into(),
        property: "ControlAreaGeneratingUnit.GeneratingUnit".into(),
        message: "The GeneratingUnit is assigned to more than once in a ControlArea.".into(),
        severity: "sh:Violation".into(), description: String::new(),
    }).collect()
}

fn check_dc_converter_unit_cs_converter_power_transformer(dataset: &CimDataset) -> Vec<Violation> {
    let mut container_has_pt: std::collections::HashSet<String> = std::collections::HashSet::new();
    for mrid in dataset.by_type.get("PowerTransformer").into_iter().flatten() {
        let entry = &dataset.entries[mrid];
        if let Some(pt) = entry.element.as_any().downcast_ref::<cimstructs::PowerTransformer>() {
            if let Some(r) = &pt.base.base.equipment_container {
                container_has_pt.insert(r.mrid.trim_start_matches('#').to_string());
            }
        }
    }
    let mut reported: std::collections::HashSet<String> = std::collections::HashSet::new();
    let mut v = Vec::new();
    for mrid in dataset.by_type.get("CsConverter").into_iter().flatten() {
        let entry = &dataset.entries[mrid];
        if let Some(csc) = entry.element.as_any().downcast_ref::<cimstructs::CsConverter>() {
            let ec_id = match &csc.base.base.base.equipment_container { Some(r) => r.mrid.trim_start_matches('#').to_string(), None => continue };
            let ec_entry = match dataset.entries.get(&ec_id) { Some(e) => e, None => continue };
            if ec_entry.element.type_name() != "DCConverterUnit" { continue; }
            if container_has_pt.contains(&ec_id) || !reported.insert(ec_id.clone()) { continue; }
            v.push(Violation {
                object_id: ec_id, rule_id: "equ:DCConverterUnit-cscPowerTransformer".into(),
                name: "C:301:EQ:DCConverterUnit:cscPowerTransformer".into(), class: "DCConverterUnit".into(),
                property: "Equipment.EquipmentContainer".into(),
                message: "A DCConverterUnit that contains CsConverter does not contain a PowerTransformer.".into(),
                severity: "sh:Violation".into(), description: String::new(),
            });
        }
    }
    v
}

fn check_limit_kind_patl_number_of_limit_type(dataset: &CimDataset) -> Vec<Violation> {
    let patl_uri = "http://iec.ch/TC57/CIM100-European#LimitKind.patl";
    let mut patl_olts: HashMap<String, bool> = HashMap::new();
    for mrid in dataset.by_type.get("OperationalLimitType").into_iter().flatten() {
        let entry = &dataset.entries[mrid];
        if let Some(olt) = entry.element.as_any().downcast_ref::<cimstructs::OperationalLimitType>() {
            if olt.kind.as_ref().map(|r| r.mrid.as_str()) == Some(patl_uri) {
                patl_olts.insert(mrid.clone(), olt.is_infinite_duration.unwrap_or(false));
            }
        }
    }
    if patl_olts.is_empty() { return Vec::new(); }
    let mut patl_counts: HashMap<String, HashMap<(String, String), i64>> = HashMap::new();
    let get_olt_set = |entry: &cimdecoder::CimEntry| -> Option<(String, String)> {
        let block = entry.element.to_block();
        let olt_id = match block.fields.get("OperationalLimit.OperationalLimitType") {
            Some(cimstructs::base::FieldValue::Resource(id)) => id.trim_start_matches('#').to_string(),
            _ => return None,
        };
        let set_id = match block.fields.get("OperationalLimit.OperationalLimitSet") {
            Some(cimstructs::base::FieldValue::Resource(id)) => id.trim_start_matches('#').to_string(),
            _ => return None,
        };
        Some((olt_id, set_id))
    };
    for limit_type in &["ApparentPowerLimit", "ActivePowerLimit", "CurrentLimit"] {
        for mrid in dataset.by_type.get(*limit_type).into_iter().flatten() {
            let entry = &dataset.entries[mrid];
            if let Some((olt_id, set_id)) = get_olt_set(entry) {
                if patl_olts.contains_key(&olt_id) {
                    *patl_counts.entry(olt_id).or_default().entry((set_id, limit_type.to_string())).or_default() += 1;
                }
            }
        }
    }
    let mut v = Vec::new();
    for (olt_id, inf_dur) in &patl_olts {
        let per_set = patl_counts.get(olt_id);
        let has_entries = per_set.map(|m| !m.is_empty()).unwrap_or(false);
        let duplicate = per_set.map(|m| m.values().any(|&c| c > 1)).unwrap_or(false);
        if duplicate || (!inf_dur && has_entries) {
            v.push(Violation {
                object_id: olt_id.clone(), rule_id: "equ:LimitKind.patl-numberOfLimitType".into(),
                name: "C:301:EQ:LimitKind.patl:numberOfLimitType".into(), class: "OperationalLimitType".into(),
                property: "OperationalLimitType.kind".into(),
                message: format!("Either there is more than one PATL defined for a given OperationalLimitSet or OperationalLimitType.isInfiniteDuration is not set to true for PATL type. The OperationalLimitType.isInfiniteDuration is: {inf_dur}."),
                severity: "sh:Violation".into(), description: String::new(),
            });
        }
    }
    v
}

fn check_limit_kind_tc_duration(dataset: &CimDataset) -> Vec<Violation> {
    let tc_uri = "http://iec.ch/TC57/CIM100-European#LimitKind.tc";
    let mut tc_olts: HashMap<String, f64> = HashMap::new();
    for mrid in dataset.by_type.get("OperationalLimitType").into_iter().flatten() {
        let entry = &dataset.entries[mrid];
        if let Some(olt) = entry.element.as_any().downcast_ref::<cimstructs::OperationalLimitType>() {
            if olt.kind.as_ref().map(|r| r.mrid.as_str()) == Some(tc_uri) {
                tc_olts.insert(mrid.clone(), olt.acceptable_duration.unwrap_or(0.0));
            }
        }
    }
    if tc_olts.is_empty() { return Vec::new(); }
    let mut counts_per_olt_set: HashMap<String, HashMap<String, i64>> = HashMap::new();
    let add_limit = |counts: &mut HashMap<String, HashMap<String, i64>>, entry: &cimdecoder::CimEntry, tc_olts: &HashMap<String, f64>| {
        let block = entry.element.to_block();
        let olt_id = match block.fields.get("OperationalLimit.OperationalLimitType") {
            Some(cimstructs::base::FieldValue::Resource(id)) => id.trim_start_matches('#').to_string(),
            _ => return,
        };
        if !tc_olts.contains_key(&olt_id) { return; }
        let set_id = match block.fields.get("OperationalLimit.OperationalLimitSet") {
            Some(cimstructs::base::FieldValue::Resource(id)) => id.trim_start_matches('#').to_string(),
            _ => return,
        };
        *counts.entry(olt_id).or_default().entry(set_id).or_default() += 1;
    };
    for limit_type in &["ApparentPowerLimit", "ActivePowerLimit", "CurrentLimit", "VoltageLimit"] {
        for mrid in dataset.by_type.get(*limit_type).into_iter().flatten() {
            add_limit(&mut counts_per_olt_set, &dataset.entries[mrid], &tc_olts);
        }
    }
    let mut v = Vec::new();
    for (olt_id, dur) in &tc_olts {
        let duplicate = counts_per_olt_set.get(olt_id).map(|m| m.values().any(|&c| c > 1)).unwrap_or(false);
        if duplicate || *dur != 0.0 {
            v.push(Violation {
                object_id: olt_id.clone(), rule_id: "equ:LimitKind.tc-duration".into(),
                name: "C:301:EQ:LimitKind.tc:duration".into(), class: "OperationalLimitType".into(),
                property: "OperationalLimitType.kind".into(),
                message: format!("Either OperationalLimitType.acceptableDuration is present and different than 0 or there is more than one limit with TC type. The OperationalLimitType.acceptableDuration is: {dur}."),
                severity: "sh:Violation".into(), description: String::new(),
            });
        }
    }
    v
}

fn check_synchronous_machine_aggregate(dataset: &CimDataset) -> Vec<Violation> {
    let mut gu_sms: HashMap<String, Vec<String>> = HashMap::new();
    for mrid in dataset.by_type.get("SynchronousMachine").into_iter().flatten() {
        let entry = &dataset.entries[mrid];
        if let Some(sm) = entry.element.as_any().downcast_ref::<cimstructs::SynchronousMachine>() {
            if let Some(r) = &sm.base.generating_unit {
                gu_sms.entry(r.mrid.trim_start_matches('#').to_string()).or_default().push(mrid.clone());
            }
        }
    }
    let mut v = Vec::new();
    for (gu_id, sm_ids) in &gu_sms {
        if sm_ids.len() != 1 { continue; }
        let sm = match dataset.entries.get(&sm_ids[0]).and_then(|e| e.element.as_any().downcast_ref::<cimstructs::SynchronousMachine>()) { Some(s) => s, None => continue };
        let gu = match dataset.entries.get(gu_id).and_then(|e| e.element.as_any().downcast_ref::<cimstructs::GeneratingUnit>()) { Some(g) => g, None => continue };
        let sm_agg = sm.base.base.base.base.base.aggregate.unwrap_or(false);
        let gu_agg = gu.base.aggregate.unwrap_or(false);
        if sm_agg != gu_agg {
            v.push(Violation {
                object_id: sm_ids[0].clone(), rule_id: "eq452:SynchronousMachine-aggregate".into(),
                name: "C:452:EQ:SynchronousMachine:aggregate".into(), class: "SynchronousMachine".into(),
                property: "Equipment.aggregate".into(),
                message: format!("SynchronousMachine aggregate flag ({sm_agg}) is not consistent with associated GeneratingUnit ({gu_agg})."),
                severity: "sh:Violation".into(), description: String::new(),
            });
        }
    }
    v
}

fn check_asynchronous_machine_aggregate(dataset: &CimDataset) -> Vec<Violation> {
    let mut gu_ams: HashMap<String, Vec<String>> = HashMap::new();
    for mrid in dataset.by_type.get("AsynchronousMachine").into_iter().flatten() {
        let entry = &dataset.entries[mrid];
        if let Some(am) = entry.element.as_any().downcast_ref::<cimstructs::AsynchronousMachine>() {
            if let Some(r) = &am.base.generating_unit {
                gu_ams.entry(r.mrid.trim_start_matches('#').to_string()).or_default().push(mrid.clone());
            }
        }
    }
    let mut v = Vec::new();
    for (gu_id, am_ids) in &gu_ams {
        if am_ids.len() != 1 { continue; }
        let am = match dataset.entries.get(&am_ids[0]).and_then(|e| e.element.as_any().downcast_ref::<cimstructs::AsynchronousMachine>()) { Some(a) => a, None => continue };
        let gu = match dataset.entries.get(gu_id).and_then(|e| e.element.as_any().downcast_ref::<cimstructs::GeneratingUnit>()) { Some(g) => g, None => continue };
        let am_agg = am.base.base.base.base.base.aggregate.unwrap_or(false);
        let gu_agg = gu.base.aggregate.unwrap_or(false);
        if am_agg != gu_agg {
            v.push(Violation {
                object_id: am_ids[0].clone(), rule_id: "eq452:AsynchronousMachine-aggregate".into(),
                name: "C:452:EQ:AsynchronousMachine:aggregate".into(), class: "AsynchronousMachine".into(),
                property: "Equipment.aggregate".into(),
                message: format!("AsynchronousMachine aggregate flag ({am_agg}) is not consistent with associated GeneratingUnit ({gu_agg})."),
                severity: "sh:Violation".into(), description: String::new(),
            });
        }
    }
    v
}

fn check_synchronous_machine_control_mode(dataset: &CimDataset) -> Vec<Violation> {
    let mut v = Vec::new();
    for mrid in dataset.by_type.get("SynchronousMachine").into_iter().flatten() {
        let entry = &dataset.entries[mrid];
        let sm = match entry.element.as_any().downcast_ref::<cimstructs::SynchronousMachine>() { Some(s) => s, None => continue };
        let rc_id = match &sm.base.base.regulating_control { Some(r) => r.mrid.trim_start_matches('#').to_string(), None => continue };
        let rc = match dataset.entries.get(&rc_id).and_then(|e| e.element.as_any().downcast_ref::<cimstructs::RegulatingControl>()) { Some(r) => r, None => continue };
        let uri = match &rc.mode { Some(r) => r.uri.as_str(), None => continue };
        if !uri.ends_with("reactivePower") && !uri.ends_with("voltage") && !uri.ends_with("powerFactor") {
            v.push(Violation {
                object_id: mrid.clone(), rule_id: "eq452:SynchronousMachine-controlMode".into(),
                name: "C:452:EQ:SynchronousMachine:controlMode".into(), class: "SynchronousMachine".into(),
                property: "RegulatingCondEq.RegulatingControl".into(),
                message: format!("Unallowed regulating control mode '{uri}' for a SynchronousMachine."),
                severity: "sh:Violation".into(), description: String::new(),
            });
        }
    }
    v
}

fn check_static_var_compensator_control_mode(dataset: &CimDataset) -> Vec<Violation> {
    let mut v = Vec::new();
    for mrid in dataset.by_type.get("StaticVarCompensator").into_iter().flatten() {
        let entry = &dataset.entries[mrid];
        let svc = match entry.element.as_any().downcast_ref::<cimstructs::StaticVarCompensator>() { Some(s) => s, None => continue };
        if let Some(rc_ref) = &svc.base.regulating_control {
            let rc_id = rc_ref.mrid.trim_start_matches('#');
            if let Some(rc) = dataset.entries.get(rc_id).and_then(|e| e.element.as_any().downcast_ref::<cimstructs::RegulatingControl>()) {
                if let Some(mode) = &rc.mode {
                    let uri = mode.uri.as_str();
                    if !uri.ends_with("voltage") && !uri.ends_with("reactivePower") {
                        v.push(Violation {
                            object_id: mrid.clone(), rule_id: "eq452:StaticVarCompensator-controlMode".into(),
                            name: "C:452:EQ:StaticVarCompensator:controlMode".into(), class: "StaticVarCompensator".into(),
                            property: "RegulatingCondEq.RegulatingControl".into(),
                            message: format!("Unallowed regulating control mode '{uri}' for a StaticVarCompensator."),
                            severity: "sh:Violation".into(), description: String::new(),
                        });
                    }
                }
            }
        }
        if svc.s_vc_control_mode.is_some() {
            v.push(Violation {
                object_id: mrid.clone(), rule_id: "eq452:StaticVarCompensator-controlMode".into(),
                name: "C:452:EQ:StaticVarCompensator:controlMode".into(), class: "StaticVarCompensator".into(),
                property: "StaticVarCompensator.sVCControlMode".into(),
                message: "StaticVarCompensator.sVCControlMode attribute is not allowed.".into(),
                severity: "sh:Violation".into(), description: String::new(),
            });
        }
        if svc.voltage_set_point.unwrap_or(0.0) != 0.0 {
            v.push(Violation {
                object_id: mrid.clone(), rule_id: "eq452:StaticVarCompensator-controlMode".into(),
                name: "C:452:EQ:StaticVarCompensator:controlMode".into(), class: "StaticVarCompensator".into(),
                property: "StaticVarCompensator.voltageSetPoint".into(),
                message: "StaticVarCompensator.voltageSetPoint attribute is not allowed.".into(),
                severity: "sh:Violation".into(), description: String::new(),
            });
        }
    }
    v
}

fn check_phase_tap_changer_control_mode(dataset: &CimDataset) -> Vec<Violation> {
    let ptc_types = [
        ("PhaseTapChangerAsymmetrical", "PhaseTapChangerAsymmetrical"),
        ("PhaseTapChangerLinear", "PhaseTapChangerLinear"),
        ("PhaseTapChangerSymmetrical", "PhaseTapChangerSymmetrical"),
        ("PhaseTapChangerTabular", "PhaseTapChangerTabular"),
    ];
    let mut v = Vec::new();
    for (type_name, class_name) in &ptc_types {
        for mrid in dataset.by_type.get(*type_name).into_iter().flatten() {
            let entry = &dataset.entries[mrid];
            let tcc_id = if let Some(o) = entry.element.as_any().downcast_ref::<cimstructs::PhaseTapChangerAsymmetrical>() {
                o.base.base.base.tap_changer_control.as_ref().map(|r| r.mrid.trim_start_matches('#').to_string())
            } else if let Some(o) = entry.element.as_any().downcast_ref::<cimstructs::PhaseTapChangerLinear>() {
                o.base.base.tap_changer_control.as_ref().map(|r| r.mrid.trim_start_matches('#').to_string())
            } else if let Some(o) = entry.element.as_any().downcast_ref::<cimstructs::PhaseTapChangerSymmetrical>() {
                o.base.base.base.tap_changer_control.as_ref().map(|r| r.mrid.trim_start_matches('#').to_string())
            } else if let Some(o) = entry.element.as_any().downcast_ref::<cimstructs::PhaseTapChangerTabular>() {
                o.base.base.tap_changer_control.as_ref().map(|r| r.mrid.trim_start_matches('#').to_string())
            } else { continue };
            let tcc_id = match tcc_id { Some(id) => id, None => continue };
            let tcc = match dataset.entries.get(&tcc_id).and_then(|e| e.element.as_any().downcast_ref::<cimstructs::TapChangerControl>()) { Some(t) => t, None => continue };
            let uri = match &tcc.base.mode { Some(r) => r.uri.as_str(), None => continue };
            if !uri.ends_with("activePower") && !uri.ends_with("voltage") {
                v.push(Violation {
                    object_id: mrid.clone(), rule_id: "eq452:PhaseTapChanger-controlModeP".into(),
                    name: "C:452:EQ:PhaseTapChanger:controlModeP".into(), class: class_name.to_string(),
                    property: "TapChanger.TapChangerControl".into(),
                    message: format!("Unallowed regulating control mode '{uri}' for a PhaseTapChanger."),
                    severity: "sh:Violation".into(), description: String::new(),
                });
            }
        }
    }
    v
}

fn check_ratio_tap_changer_control_mode(dataset: &CimDataset) -> Vec<Violation> {
    let mut v = Vec::new();
    for mrid in dataset.by_type.get("RatioTapChanger").into_iter().flatten() {
        let entry = &dataset.entries[mrid];
        let rtc = match entry.element.as_any().downcast_ref::<cimstructs::RatioTapChanger>() { Some(r) => r, None => continue };
        let tcc_id = match &rtc.base.tap_changer_control { Some(r) => r.mrid.trim_start_matches('#').to_string(), None => continue };
        let tcc = match dataset.entries.get(&tcc_id).and_then(|e| e.element.as_any().downcast_ref::<cimstructs::TapChangerControl>()) { Some(t) => t, None => continue };
        let uri = match &tcc.base.mode { Some(r) => r.uri.as_str(), None => continue };
        if !uri.ends_with("voltage") && !uri.ends_with("reactivePower") && !uri.ends_with("powerFactor") {
            v.push(Violation {
                object_id: mrid.clone(), rule_id: "eq452:RatioTapChanger-controlMode".into(),
                name: "C:452:EQ:RatioTapChanger:controlMode".into(), class: "RatioTapChanger".into(),
                property: "TapChanger.TapChangerControl".into(),
                message: format!("Unallowed regulating control mode '{uri}' for a RatioTapChanger."),
                severity: "sh:Violation".into(), description: String::new(),
            });
        }
    }
    v
}

fn check_shunt_compensator_control_mode(dataset: &CimDataset) -> Vec<Violation> {
    let sc_types = [
        ("LinearShuntCompensator", "LinearShuntCompensator"),
        ("NonlinearShuntCompensator", "NonlinearShuntCompensator"),
    ];
    let mut v = Vec::new();
    for (type_name, class_name) in &sc_types {
        for mrid in dataset.by_type.get(*type_name).into_iter().flatten() {
            let entry = &dataset.entries[mrid];
            let rc_id = if let Some(o) = entry.element.as_any().downcast_ref::<cimstructs::LinearShuntCompensator>() {
                o.base.base.regulating_control.as_ref().map(|r| r.mrid.trim_start_matches('#').to_string())
            } else if let Some(o) = entry.element.as_any().downcast_ref::<cimstructs::NonlinearShuntCompensator>() {
                o.base.base.regulating_control.as_ref().map(|r| r.mrid.trim_start_matches('#').to_string())
            } else { continue };
            let rc_id = match rc_id { Some(id) => id, None => continue };
            let rc = match dataset.entries.get(&rc_id).and_then(|e| e.element.as_any().downcast_ref::<cimstructs::RegulatingControl>()) { Some(r) => r, None => continue };
            let uri = match &rc.mode { Some(r) => r.uri.as_str(), None => continue };
            if !uri.ends_with("voltage") && !uri.ends_with("reactivePower") && !uri.ends_with("powerFactor") {
                v.push(Violation {
                    object_id: mrid.clone(), rule_id: "eq452:ShuntCompensator-controlMode".into(),
                    name: "C:452:EQ:ShuntCompensator:controlMode".into(), class: class_name.to_string(),
                    property: "RegulatingCondEq.RegulatingControl".into(),
                    message: format!("Unallowed regulating control mode '{uri}' for a ShuntCompensator."),
                    severity: "sh:Violation".into(), description: String::new(),
                });
            }
        }
    }
    v
}

fn check_synchronous_machine_reactive_limits(dataset: &CimDataset) -> Vec<Violation> {
    let mut curve_cd: HashMap<String, Vec<(f64, f64)>> = HashMap::new();
    for mrid in dataset.by_type.get("CurveData").into_iter().flatten() {
        let entry = &dataset.entries[mrid];
        if let Some(cd) = entry.element.as_any().downcast_ref::<cimstructs::CurveData>() {
            if let Some(r) = &cd.curve {
                let c_id = r.mrid.trim_start_matches('#').to_string();
                curve_cd.entry(c_id).or_default().push((cd.y1value.unwrap_or(0.0), cd.y2value.unwrap_or(0.0)));
            }
        }
    }
    let mut v = Vec::new();
    for mrid in dataset.by_type.get("SynchronousMachine").into_iter().flatten() {
        let entry = &dataset.entries[mrid];
        let sm = match entry.element.as_any().downcast_ref::<cimstructs::SynchronousMachine>() { Some(s) => s, None => continue };
        let rcc_id = match &sm.initial_reactive_capability_curve { Some(r) => r.mrid.trim_start_matches('#').to_string(), None => continue };
        let yvals = match curve_cd.get(&rcc_id) { Some(vals) if !vals.is_empty() => vals, _ => continue };
        let min_y1 = yvals.iter().map(|(y1, _)| *y1).fold(f64::INFINITY, f64::min);
        let max_y2 = yvals.iter().map(|(_, y2)| *y2).fold(f64::NEG_INFINITY, f64::max);
        let eps = 1e-6;
        let min_q = sm.min_q.unwrap_or(0.0);
        let max_q = sm.max_q.unwrap_or(0.0);
        if min_q != 0.0 && (min_q < min_y1 - eps || min_q > min_y1 + eps) {
            v.push(Violation {
                object_id: mrid.clone(), rule_id: "eq452:SynchronousMachine-reactiveLimits".into(),
                name: "C:452:EQ:SynchronousMachine:reactiveLimits".into(), class: "SynchronousMachine".into(),
                property: "SynchronousMachine.minQ".into(),
                message: format!("SynchronousMachine.minQ ({min_q}) is not equal to min of CurveData.y1value-s ({min_y1})."),
                severity: "sh:Violation".into(), description: String::new(),
            });
        }
        if max_q != 0.0 && (max_q < max_y2 - eps || max_q > max_y2 + eps) {
            v.push(Violation {
                object_id: mrid.clone(), rule_id: "eq452:SynchronousMachine-reactiveLimits".into(),
                name: "C:452:EQ:SynchronousMachine:reactiveLimits".into(), class: "SynchronousMachine".into(),
                property: "SynchronousMachine.maxQ".into(),
                message: format!("SynchronousMachine.maxQ ({max_q}) is not equal to max of CurveData.y2value-s ({max_y2})."),
                severity: "sh:Violation".into(), description: String::new(),
            });
        }
    }
    v
}

fn check_synchronous_machine_type_condenser(dataset: &CimDataset) -> Vec<Violation> {
    let mut v = Vec::new();
    for mrid in dataset.by_type.get("SynchronousMachine").into_iter().flatten() {
        let entry = &dataset.entries[mrid];
        if let Some(sm) = entry.element.as_any().downcast_ref::<cimstructs::SynchronousMachine>() {
            if sm.type_.as_ref().map(|r| r.uri.ends_with("condenser")).unwrap_or(false) && sm.base.generating_unit.is_some() {
                v.push(Violation {
                    object_id: mrid.clone(), rule_id: "eq452:SynchronousMachine.type-condenser".into(),
                    name: "C:452:EQ:SynchronousMachine.type:condenser".into(), class: "SynchronousMachine".into(),
                    property: "SynchronousMachine.type".into(),
                    message: "SynchronousMachine of type condenser with associated GeneratingUnit.".into(),
                    severity: "sh:Info".into(), description: String::new(),
                });
            }
        }
    }
    v
}

fn check_vs_capability_curve_count(dataset: &CimDataset) -> Vec<Violation> {
    let mut curve_count: HashMap<String, i64> = HashMap::new();
    for mrid in dataset.by_type.get("CurveData").into_iter().flatten() {
        let entry = &dataset.entries[mrid];
        if let Some(cd) = entry.element.as_any().downcast_ref::<cimstructs::CurveData>() {
            if let Some(r) = &cd.curve {
                *curve_count.entry(r.mrid.trim_start_matches('#').to_string()).or_default() += 1;
            }
        }
    }
    let mut v = Vec::new();
    for mrid in dataset.by_type.get("VsCapabilityCurve").into_iter().flatten() {
        let count = curve_count.get(mrid.as_str()).copied().unwrap_or(0);
        if count < 2 {
            v.push(Violation {
                object_id: mrid.clone(), rule_id: "eq452:VsCapabilityCurve-VsCapabilityCurveCount".into(),
                name: "C:452:EQ:CurveData.Curve:VsCapabilityCurveCount".into(), class: "VsCapabilityCurve".into(),
                property: "rdf:type".into(),
                message: format!("Less than two instances of CurveData are associated ({count} found)."),
                severity: "sh:Violation".into(), description: String::new(),
            });
        }
    }
    v
}

fn check_vs_capability_curve_y_values(dataset: &CimDataset) -> Vec<Violation> {
    let vs_curves: std::collections::HashSet<String> = dataset.by_type.get("VsCapabilityCurve").into_iter().flatten().cloned().collect();
    let mut v = Vec::new();
    for mrid in dataset.by_type.get("CurveData").into_iter().flatten() {
        let entry = &dataset.entries[mrid];
        if let Some(cd) = entry.element.as_any().downcast_ref::<cimstructs::CurveData>() {
            if let Some(r) = &cd.curve {
                let c_id = r.mrid.trim_start_matches('#');
                if vs_curves.contains(c_id) {
                    let y1 = cd.y1value.unwrap_or(0.0);
                    let y2 = cd.y2value.unwrap_or(0.0);
                    if y2 <= y1 {
                        v.push(Violation {
                            object_id: mrid.clone(), rule_id: "eq452:VsCapabilityCurve-yvalues".into(),
                            name: "C:452:EQ:CurveData.Curve:VsCapabilityCurve".into(), class: "CurveData".into(),
                            property: "CurveData.y2value".into(),
                            message: format!("CurveData.y2value ({y2}) is not greater than CurveData.y1value ({y1}) for VsCapabilityCurve."),
                            severity: "sh:Violation".into(), description: String::new(),
                        });
                    }
                }
            }
        }
    }
    v
}

fn check_generating_unit_type_dependency(dataset: &CimDataset) -> Vec<Violation> {
    let mut v = Vec::new();
    for mrid in dataset.by_type.get("SynchronousMachine").into_iter().flatten() {
        let entry = &dataset.entries[mrid];
        let sm = match entry.element.as_any().downcast_ref::<cimstructs::SynchronousMachine>() { Some(s) => s, None => continue };
        let gu_id = match &sm.base.generating_unit { Some(r) => r.mrid.trim_start_matches('#').to_string(), None => continue };
        let sm_type = match &sm.type_ { Some(t) => t.uri.as_str(), None => continue };
        let gu = match dataset.entries.get(&gu_id).and_then(|e| e.element.as_any().downcast_ref::<cimstructs::GeneratingUnit>()) { Some(g) => g, None => continue };
        let max_p = gu.max_operating_p.unwrap_or(0.0);
        let min_p = gu.min_operating_p.unwrap_or(0.0);
        let msg = if sm_type.ends_with("condenser") {
            if max_p != 0.0 || min_p != 0.0 {
                Some(format!("For condenser type, min/max operating P must be 0 (found min: {min_p}, max: {max_p})."))
            } else { None }
        } else if sm_type.ends_with("generator") || sm_type.ends_with("generatorOrCondenser") {
            if max_p <= 0.0 || min_p < 0.0 {
                Some(format!("For {sm_type} type, minP >= 0 and maxP > 0 (found min: {min_p}, max: {max_p})."))
            } else { None }
        } else if sm_type.ends_with("motor") || sm_type.ends_with("motorOrCondenser") {
            if max_p > 0.0 || min_p >= 0.0 {
                Some(format!("For {sm_type} type, minP < 0 and maxP <= 0 (found min: {min_p}, max: {max_p})."))
            } else { None }
        } else if sm_type.ends_with("generatorOrMotor") || sm_type.ends_with("generatorOrCondenserOrMotor") {
            if max_p <= 0.0 || min_p >= 0.0 {
                Some(format!("For {sm_type} type, minP < 0 and maxP > 0 (found min: {min_p}, max: {max_p})."))
            } else { None }
        } else { None };
        if let Some(msg) = msg {
            v.push(Violation {
                object_id: mrid.clone(), rule_id: "eq452:GeneratingUnit-typeDependency".into(),
                name: "C:452:EQ:GeneratingUnit:typeDependency".into(), class: "SynchronousMachine".into(),
                property: "SynchronousMachine.type".into(),
                message: msg, severity: "sh:Violation".into(), description: String::new(),
            });
        }
    }
    v
}

fn check_curve_data_reactive_capability_limits(dataset: &CimDataset) -> Vec<Violation> {
    let mut curve_rated_s: HashMap<String, f64> = HashMap::new();
    for mrid in dataset.by_type.get("SynchronousMachine").into_iter().flatten() {
        let entry = &dataset.entries[mrid];
        if let Some(sm) = entry.element.as_any().downcast_ref::<cimstructs::SynchronousMachine>() {
            if let Some(r) = &sm.initial_reactive_capability_curve {
                let c_id = r.mrid.trim_start_matches('#').to_string();
                curve_rated_s.insert(c_id, sm.base.rated_s.unwrap_or(0.0));
            }
        }
    }
    let rcc_set: std::collections::HashSet<String> = dataset.by_type.get("ReactiveCapabilityCurve").into_iter().flatten().cloned().collect();
    let mut v = Vec::new();
    for mrid in dataset.by_type.get("CurveData").into_iter().flatten() {
        let entry = &dataset.entries[mrid];
        let cd = match entry.element.as_any().downcast_ref::<cimstructs::CurveData>() { Some(c) => c, None => continue };
        let c_id = match &cd.curve { Some(r) => r.mrid.trim_start_matches('#').to_string(), None => continue };
        if !rcc_set.contains(&c_id) { continue; }
        let rated_s = match curve_rated_s.get(&c_id) { Some(&s) if s != 0.0 => s, _ => continue };
        let xv = cd.xvalue.unwrap_or(0.0);
        let y1 = cd.y1value.unwrap_or(0.0);
        let y2 = cd.y2value.unwrap_or(0.0);
        let s2 = rated_s * rated_s;
        let eps = 1e-4;
        if xv * xv + y1 * y1 > s2 + eps {
            v.push(Violation {
                object_id: mrid.clone(), rule_id: "eq452:CurveData-equationY1".into(),
                name: "C:452:EQ:CurveData.Curve:equationY1".into(), class: "CurveData".into(),
                property: "CurveData.y1value".into(),
                message: format!("x^2 + y1^2 ({}) > ratedS^2 ({s2}).", xv * xv + y1 * y1),
                severity: "sh:Violation".into(), description: String::new(),
            });
        }
        if xv * xv + y2 * y2 > s2 + eps {
            v.push(Violation {
                object_id: mrid.clone(), rule_id: "eq452:CurveData-equationY2".into(),
                name: "C:452:EQ:CurveData.Curve:equationY2".into(), class: "CurveData".into(),
                property: "CurveData.y2value".into(),
                message: format!("x^2 + y2^2 ({}) > ratedS^2 ({s2}).", xv * xv + y2 * y2),
                severity: "sh:Violation".into(), description: String::new(),
            });
        }
    }
    v
}

fn check_curve_data_reactive_consistency(dataset: &CimDataset) -> Vec<Violation> {
    let rcc_set: std::collections::HashSet<String> = dataset.by_type.get("ReactiveCapabilityCurve").into_iter().flatten().cloned().collect();
    let mut curve_points: HashMap<String, Vec<String>> = HashMap::new();
    for mrid in dataset.by_type.get("CurveData").into_iter().flatten() {
        let entry = &dataset.entries[mrid];
        if let Some(cd) = entry.element.as_any().downcast_ref::<cimstructs::CurveData>() {
            if let Some(r) = &cd.curve {
                let c_id = r.mrid.trim_start_matches('#').to_string();
                if rcc_set.contains(&c_id) {
                    curve_points.entry(c_id).or_default().push(mrid.clone());
                }
            }
        }
    }
    let mut v = Vec::new();
    for (curve_id, point_ids) in &curve_points {
        let mut all_same = true;
        for pid in point_ids {
            let cd = match dataset.entries.get(pid).and_then(|e| e.element.as_any().downcast_ref::<cimstructs::CurveData>()) { Some(c) => c, None => continue };
            let y1 = cd.y1value.unwrap_or(0.0);
            let y2 = cd.y2value.unwrap_or(0.0);
            if y2 < y1 {
                v.push(Violation {
                    object_id: pid.clone(), rule_id: "eq452:CurveData-reactive".into(),
                    name: "C:452:EQ:CurveData.Curve:reactive".into(), class: "CurveData".into(),
                    property: "CurveData.y2value".into(),
                    message: format!("CurveData.y2value ({y2}) is less than y1value ({y1})."),
                    severity: "sh:Violation".into(), description: String::new(),
                });
            }
            if y2 != y1 { all_same = false; }
        }
        if all_same && !point_ids.is_empty() {
            v.push(Violation {
                object_id: curve_id.clone(), rule_id: "eq452:CurveData-reactive".into(),
                name: "C:452:EQ:CurveData.Curve:reactive".into(), class: "ReactiveCapabilityCurve".into(),
                property: "rdf:type".into(),
                message: "All CurveData.y2value values are equal to CurveData.y1value values.".into(),
                severity: "sh:Violation".into(), description: String::new(),
            });
        }
    }
    v
}

fn check_synchronous_machine_curve_x_value_consistency(dataset: &CimDataset) -> Vec<Violation> {
    let mut curve_xvals: HashMap<String, Vec<f64>> = HashMap::new();
    for mrid in dataset.by_type.get("CurveData").into_iter().flatten() {
        let entry = &dataset.entries[mrid];
        if let Some(cd) = entry.element.as_any().downcast_ref::<cimstructs::CurveData>() {
            if let Some(r) = &cd.curve {
                curve_xvals.entry(r.mrid.trim_start_matches('#').to_string()).or_default().push(cd.xvalue.unwrap_or(0.0));
            }
        }
    }
    let mut v = Vec::new();
    for mrid in dataset.by_type.get("SynchronousMachine").into_iter().flatten() {
        let entry = &dataset.entries[mrid];
        let sm = match entry.element.as_any().downcast_ref::<cimstructs::SynchronousMachine>() { Some(s) => s, None => continue };
        let gu_id = match &sm.base.generating_unit { Some(r) => r.mrid.trim_start_matches('#').to_string(), None => continue };
        let rcc_id = match &sm.initial_reactive_capability_curve { Some(r) => r.mrid.trim_start_matches('#').to_string(), None => continue };
        let gu = match dataset.entries.get(&gu_id).and_then(|e| e.element.as_any().downcast_ref::<cimstructs::GeneratingUnit>()) { Some(g) => g, None => continue };
        let xvals = match curve_xvals.get(&rcc_id) { Some(v) if !v.is_empty() => v, _ => continue };
        let min_x = xvals.iter().copied().fold(f64::INFINITY, f64::min);
        let max_x = xvals.iter().copied().fold(f64::NEG_INFINITY, f64::max);
        let eps = 1e-6;
        let min_p = gu.min_operating_p.unwrap_or(0.0);
        let max_p = gu.max_operating_p.unwrap_or(0.0);
        if (min_p - min_x).abs() > eps {
            v.push(Violation {
                object_id: mrid.clone(), rule_id: "eq452:CurveData.xvalue-value".into(),
                name: "C:452:EQ:CurveData.xvalue:value".into(), class: "SynchronousMachine".into(),
                property: "GeneratingUnit.minOperatingP".into(),
                message: format!("GeneratingUnit.minOperatingP ({min_p}) is not consistent with min CurveData.xvalue ({min_x})."),
                severity: "sh:Violation".into(), description: String::new(),
            });
        }
        if (max_p - max_x).abs() > eps {
            v.push(Violation {
                object_id: mrid.clone(), rule_id: "eq452:CurveData.xvalue-value".into(),
                name: "C:452:EQ:CurveData.xvalue:value".into(), class: "SynchronousMachine".into(),
                property: "GeneratingUnit.maxOperatingP".into(),
                message: format!("GeneratingUnit.maxOperatingP ({max_p}) is not consistent with max CurveData.xvalue ({max_x})."),
                severity: "sh:Violation".into(), description: String::new(),
            });
        }
    }
    v
}

fn check_switch_connection(dataset: &CimDataset) -> Vec<Violation> {
    let switch_types = ["Breaker", "Disconnector", "Fuse", "GroundDisconnector", "Jumper",
        "LoadBreakSwitch", "DisconnectingCircuitBreaker", "Cut"];
    let mut switch_ids: std::collections::HashSet<String> = std::collections::HashSet::new();
    for type_name in &switch_types {
        for mrid in dataset.by_type.get(*type_name).into_iter().flatten() {
            switch_ids.insert(mrid.clone());
        }
    }
    let mut switch_terms: HashMap<String, Vec<String>> = HashMap::new();
    for mrid in dataset.by_type.get("Terminal").into_iter().flatten() {
        let entry = &dataset.entries[mrid];
        if let Some(term) = entry.element.as_any().downcast_ref::<cimstructs::Terminal>() {
            if let Some(ce) = &term.conducting_equipment {
                let eq_id = ce.mrid.trim_start_matches('#');
                if switch_ids.contains(eq_id) {
                    switch_terms.entry(eq_id.to_string()).or_default().push(mrid.clone());
                }
            }
        }
    }
    let mut v = Vec::new();
    for (eq_id, term_ids) in &switch_terms {
        if term_ids.len() < 2 { continue; }
        let mut bvs: std::collections::HashSet<i64> = std::collections::HashSet::new();
        let mut cncs: std::collections::HashSet<String> = std::collections::HashSet::new();
        for tid in term_ids {
            let term = match dataset.entries.get(tid).and_then(|e| e.element.as_any().downcast_ref::<cimstructs::Terminal>()) { Some(t) => t, None => continue };
            let cn_id = match &term.connectivity_node { Some(r) => r.mrid.trim_start_matches('#').to_string(), None => continue };
            let cn = match dataset.entries.get(&cn_id).and_then(|e| e.element.as_any().downcast_ref::<cimstructs::ConnectivityNode>()) { Some(c) => c, None => continue };
            let cnc_id = match &cn.connectivity_node_container { Some(r) => r.mrid.trim_start_matches('#').to_string(), None => continue };
            cncs.insert(cnc_id.clone());
            let vl = match dataset.entries.get(&cnc_id).and_then(|e| e.element.as_any().downcast_ref::<cimstructs::VoltageLevel>()) { Some(vl) => vl, None => continue };
            let bv_id = match &vl.base_voltage { Some(r) => r.mrid.trim_start_matches('#').to_string(), None => continue };
            let bv = match dataset.entries.get(&bv_id).and_then(|e| e.element.as_any().downcast_ref::<cimstructs::BaseVoltage>()) { Some(b) => b, None => continue };
            if let Some(nv) = bv.nominal_voltage { bvs.insert((nv * 1000.0) as i64); }
        }
        if cncs.len() > 1 && bvs.len() > 1 {
            v.push(Violation {
                object_id: eq_id.clone(), rule_id: "eq452:Switch-connection".into(),
                name: "C:452:EQ:Switch:connection".into(), class: "Switch".into(),
                property: "rdf:type".into(),
                message: "Switch (or its subclasses) connects ConnectivityNode-s that are not contained in either the same VoltageLevel or in different VoltageLevel-s which have the same BaseVoltage.".into(),
                severity: "sh:Violation".into(), description: String::new(),
            });
        }
    }
    v
}

fn check_operational_limit_set_terminal(dataset: &CimDataset) -> Vec<Violation> {
    let mut aux_term_ids: std::collections::HashSet<String> = std::collections::HashSet::new();
    for mrid in dataset.by_type.get("CurrentTransformer").into_iter().flatten() {
        let entry = &dataset.entries[mrid];
        if let Some(ct) = entry.element.as_any().downcast_ref::<cimstructs::CurrentTransformer>() {
            if let Some(r) = &ct.base.base.terminal {
                aux_term_ids.insert(r.mrid.trim_start_matches('#').to_string());
            }
        }
    }
    let mut term_eq: HashMap<String, String> = HashMap::new();
    for mrid in dataset.by_type.get("Terminal").into_iter().flatten() {
        let entry = &dataset.entries[mrid];
        if let Some(term) = entry.element.as_any().downcast_ref::<cimstructs::Terminal>() {
            if let Some(ce) = &term.conducting_equipment {
                term_eq.insert(mrid.clone(), ce.mrid.trim_start_matches('#').to_string());
            }
        }
    }
    let mut v = Vec::new();
    for mrid in dataset.by_type.get("OperationalLimitSet").into_iter().flatten() {
        let entry = &dataset.entries[mrid];
        let ols = match entry.element.as_any().downcast_ref::<cimstructs::OperationalLimitSet>() { Some(o) => o, None => continue };
        let t_id = match &ols.terminal { Some(r) => r.mrid.trim_start_matches('#').to_string(), None => continue };
        if aux_term_ids.contains(&t_id) && ols.equipment.is_none() {
            v.push(Violation {
                object_id: mrid.clone(), rule_id: "eq452:OperationalLimitSet-limits".into(),
                name: "C:452:EQ:OperationalLimitSet:limits".into(), class: "OperationalLimitSet".into(),
                property: "OperationalLimitSet.Equipment".into(),
                message: "OperationalLimitSet.Equipment is not provided for a Terminal associated with AuxiliaryEquipment.".into(),
                severity: "sh:Violation".into(), description: String::new(),
            });
        }
        if let Some(eq_ref) = &ols.equipment {
            let eq_id = eq_ref.mrid.trim_start_matches('#');
            if term_eq.get(&t_id).map(|s| s.as_str()) != Some(eq_id) {
                v.push(Violation {
                    object_id: mrid.clone(), rule_id: "eq452:OperationalLimitSet-limits".into(),
                    name: "C:452:EQ:OperationalLimitSet:limits".into(), class: "OperationalLimitSet".into(),
                    property: "OperationalLimitSet.Terminal".into(),
                    message: format!("Terminal {t_id} is not a terminal of ConductingEquipment {eq_id}."),
                    severity: "sh:Violation".into(), description: String::new(),
                });
            }
        }
    }
    v
}

fn check_tap_changer_control_remote_q_control(dataset: &CimDataset) -> Vec<Violation> {
    let mut tcc_te: HashMap<String, Vec<String>> = HashMap::new();
    for mrid in dataset.by_type.get("RatioTapChanger").into_iter().flatten() {
        let entry = &dataset.entries[mrid];
        if let Some(rtc) = entry.element.as_any().downcast_ref::<cimstructs::RatioTapChanger>() {
            if let (Some(tcc_r), Some(te_r)) = (&rtc.base.tap_changer_control, &rtc.transformer_end) {
                let tcc_id = tcc_r.mrid.trim_start_matches('#').to_string();
                let te_id = te_r.mrid.trim_start_matches('#').to_string();
                tcc_te.entry(tcc_id).or_default().push(te_id);
            }
        }
    }
    for mrid in dataset.by_type.get("PhaseTapChangerAsymmetrical").into_iter().flatten() {
        let entry = &dataset.entries[mrid];
        if let Some(ptc) = entry.element.as_any().downcast_ref::<cimstructs::PhaseTapChangerAsymmetrical>() {
            if let (Some(tcc_r), Some(te_r)) = (&ptc.base.base.base.tap_changer_control, &ptc.base.base.transformer_end) {
                tcc_te.entry(tcc_r.mrid.trim_start_matches('#').to_string()).or_default().push(te_r.mrid.trim_start_matches('#').to_string());
            }
        }
    }
    let mut v = Vec::new();
    for mrid in dataset.by_type.get("TapChangerControl").into_iter().flatten() {
        let entry = &dataset.entries[mrid];
        let tcc = match entry.element.as_any().downcast_ref::<cimstructs::TapChangerControl>() { Some(t) => t, None => continue };
        let mode_uri = match &tcc.base.mode { Some(r) => r.uri.as_str(), None => continue };
        if !mode_uri.ends_with("reactivePower") { continue; }
        let rc_term_id = match &tcc.base.terminal { Some(r) => r.mrid.trim_start_matches('#').to_string(), None => continue };
        let te_ids = match tcc_te.get(mrid.as_str()) { Some(ids) => ids, None => continue };
        for te_id in te_ids {
            let pte = match dataset.entries.get(te_id).and_then(|e| e.element.as_any().downcast_ref::<cimstructs::PowerTransformerEnd>()) { Some(p) => p, None => continue };
            let pte_term_id = match &pte.base.terminal { Some(r) => r.mrid.trim_start_matches('#'), None => continue };
            if pte_term_id != rc_term_id.as_str() {
                v.push(Violation {
                    object_id: mrid.clone(), rule_id: "eq452:TapChangerControl-remoteQcontrol".into(),
                    name: "C:452:EQ:TapChangerControl:remoteQcontrol".into(), class: "TapChangerControl".into(),
                    property: "RegulatingControl.Terminal".into(),
                    message: "TapChangerControl in reactivePower mode controls a Terminal not associated with its PowerTransformerEnd.".into(),
                    severity: "sh:Violation".into(), description: String::new(),
                });
            }
        }
    }
    v
}

fn check_reactive_capability_curve_x_value_unique(dataset: &CimDataset) -> Vec<Violation> {
    let mut curve_xvals: HashMap<String, Vec<f64>> = HashMap::new();
    for mrid in dataset.by_type.get("CurveData").into_iter().flatten() {
        let entry = &dataset.entries[mrid];
        if let Some(cd) = entry.element.as_any().downcast_ref::<cimstructs::CurveData>() {
            if let Some(r) = &cd.curve {
                curve_xvals.entry(r.mrid.trim_start_matches('#').to_string()).or_default().push(cd.xvalue.unwrap_or(0.0));
            }
        }
    }
    let mut v = Vec::new();
    for mrid in dataset.by_type.get("ReactiveCapabilityCurve").into_iter().flatten() {
        let xvals = match curve_xvals.get(mrid.as_str()) { Some(v) => v, None => continue };
        let mut seen: std::collections::HashSet<i64> = std::collections::HashSet::new();
        for &xv in xvals {
            let key = (xv * 1e9) as i64;
            if !seen.insert(key) {
                v.push(Violation {
                    object_id: mrid.clone(), rule_id: "eq452:ReactiveCapabilityCurve-xvalue".into(),
                    name: "C:452:EQ:ReactiveCapabiltyCurve.CurveData:xvalue".into(), class: "ReactiveCapabilityCurve".into(),
                    property: "rdf:type".into(),
                    message: format!("CurveData.xvalue ({xv}) for ReactiveCapabilityCurve is not unique."),
                    severity: "sh:Violation".into(), description: String::new(),
                });
                break;
            }
        }
    }
    v
}

fn check_power_transformer_end_resistance_x_value(dataset: &CimDataset) -> Vec<Violation> {
    let pt_ends = build_pt_ends(dataset);
    let mut v = Vec::new();
    for (_, end_ids) in &pt_ends {
        let n = end_ids.len();
        if n == 2 {
            for eid in end_ids {
                let pte = match dataset.entries.get(eid).and_then(|e| e.element.as_any().downcast_ref::<cimstructs::PowerTransformerEnd>()) { Some(p) => p, None => continue };
                if pte.base.end_number.unwrap_or(0) == 1 && pte.x.unwrap_or(0.0) <= 0.0 {
                    let xv = pte.x.unwrap_or(0.0);
                    v.push(Violation {
                        object_id: eid.clone(), rule_id: "eq452:PowerTransformerEnd.x-value".into(),
                        name: "C:452:EQ:PowerTransformerEnd.x:value".into(), class: "PowerTransformerEnd".into(),
                        property: "PowerTransformerEnd.x".into(),
                        message: format!("PowerTransformerEnd.x ({xv}) for winding 1 of a two-winding transformer must be positive."),
                        severity: "sh:Violation".into(), description: String::new(),
                    });
                }
            }
        } else if n == 3 {
            for eid in end_ids {
                let pte = match dataset.entries.get(eid).and_then(|e| e.element.as_any().downcast_ref::<cimstructs::PowerTransformerEnd>()) { Some(p) => p, None => continue };
                if pte.x.unwrap_or(0.0) == 0.0 {
                    v.push(Violation {
                        object_id: eid.clone(), rule_id: "eq452:PowerTransformerEnd.x-value".into(),
                        name: "C:452:EQ:PowerTransformerEnd.x:value".into(), class: "PowerTransformerEnd".into(),
                        property: "PowerTransformerEnd.x".into(),
                        message: "PowerTransformerEnd.x cannot be zero for a three-winding transformer.".into(),
                        severity: "sh:Violation".into(), description: String::new(),
                    });
                }
            }
        }
    }
    v
}

fn check_generating_unit_max_operating_p_rated_s(dataset: &CimDataset) -> Vec<Violation> {
    let mut gu_rated_s: HashMap<String, f64> = HashMap::new();
    for type_name in &["SynchronousMachine", "AsynchronousMachine"] {
        for mrid in dataset.by_type.get(*type_name).into_iter().flatten() {
            let entry = &dataset.entries[mrid];
            let (gu_id, rs) = if let Some(sm) = entry.element.as_any().downcast_ref::<cimstructs::SynchronousMachine>() {
                (sm.base.generating_unit.as_ref().map(|r| r.mrid.trim_start_matches('#').to_string()), sm.base.rated_s.unwrap_or(0.0))
            } else if let Some(am) = entry.element.as_any().downcast_ref::<cimstructs::AsynchronousMachine>() {
                (am.base.generating_unit.as_ref().map(|r| r.mrid.trim_start_matches('#').to_string()), am.base.rated_s.unwrap_or(0.0))
            } else { continue };
            if let Some(id) = gu_id { *gu_rated_s.entry(id).or_default() += rs; }
        }
    }
    let mut v = Vec::new();
    for mrid in dataset.by_type.get("GeneratingUnit").into_iter().flatten() {
        let entry = &dataset.entries[mrid];
        if let Some(gu) = entry.element.as_any().downcast_ref::<cimstructs::GeneratingUnit>() {
            let max_p = gu.max_operating_p.unwrap_or(0.0);
            let sum_rs = gu_rated_s.get(mrid.as_str()).copied().unwrap_or(0.0);
            if max_p > sum_rs {
                v.push(Violation {
                    object_id: mrid.clone(), rule_id: "eq452:GeneratingUnit.maxOperatingP-ratedS".into(),
                    name: "C:452:EQ:GeneratingUnit:maxOperatingP:ratedS".into(), class: "GeneratingUnit".into(),
                    property: "GeneratingUnit.maxOperatingP".into(),
                    message: format!("GeneratingUnit.maxOperatingP ({max_p}) is greater than sum of RotatingMachine.ratedS ({sum_rs})."),
                    severity: "sh:Violation".into(), description: String::new(),
                });
            }
        }
    }
    v
}

fn check_hydro_generating_unit_energy_conversion_capability(dataset: &CimDataset) -> Vec<Violation> {
    let mut gu_sm: HashMap<String, String> = HashMap::new();
    for mrid in dataset.by_type.get("SynchronousMachine").into_iter().flatten() {
        let entry = &dataset.entries[mrid];
        if let Some(sm) = entry.element.as_any().downcast_ref::<cimstructs::SynchronousMachine>() {
            if let Some(r) = &sm.base.generating_unit {
                gu_sm.insert(r.mrid.trim_start_matches('#').to_string(), mrid.clone());
            }
        }
    }
    let mut v = Vec::new();
    for mrid in dataset.by_type.get("HydroGeneratingUnit").into_iter().flatten() {
        let entry = &dataset.entries[mrid];
        let hgu = match entry.element.as_any().downcast_ref::<cimstructs::HydroGeneratingUnit>() { Some(h) => h, None => continue };
        let ecc_uri = match &hgu.energy_conversion_capability { Some(r) => r.uri.as_str(), None => continue };
        let sm_id = match gu_sm.get(mrid.as_str()) { Some(id) => id, None => continue };
        let sm = match dataset.entries.get(sm_id).and_then(|e| e.element.as_any().downcast_ref::<cimstructs::SynchronousMachine>()) { Some(s) => s, None => continue };
        let sm_type = match &sm.type_ { Some(t) => t.uri.as_str(), None => continue };
        let msg = if ecc_uri.ends_with("generator") {
            if !sm_type.ends_with("generator") && !sm_type.ends_with("generatorOrCondenser") {
                Some(format!("HydroGeneratingUnit as generator but associated SynchronousMachine type is '{sm_type}'."))
            } else { None }
        } else if ecc_uri.ends_with("pumpAndGenerator") {
            if !sm_type.ends_with("motor") && !sm_type.ends_with("generatorOrMotor") && !sm_type.ends_with("generatorOrCondenserOrMotor") {
                Some(format!("HydroGeneratingUnit as pumpAndGenerator but associated SynchronousMachine type is '{sm_type}'."))
            } else { None }
        } else { None };
        if let Some(msg) = msg {
            v.push(Violation {
                object_id: mrid.clone(), rule_id: "eq452:HydroGeneratingUnit.energyConversionCapability-typeConsistency".into(),
                name: "C:452:EQ:HydroGeneratingUnit.energyConversionCapability:typeConsistency".into(), class: "HydroGeneratingUnit".into(),
                property: "HydroGeneratingUnit.energyConversionCapability".into(),
                message: msg, severity: "sh:Violation".into(), description: String::new(),
            });
        }
    }
    v
}

fn check_terminal_connection_same_node(dataset: &CimDataset) -> Vec<Violation> {
    let mut eq_terms: HashMap<String, Vec<String>> = HashMap::new();
    for mrid in dataset.by_type.get("Terminal").into_iter().flatten() {
        let entry = &dataset.entries[mrid];
        if let Some(term) = entry.element.as_any().downcast_ref::<cimstructs::Terminal>() {
            if let Some(ce) = &term.conducting_equipment {
                eq_terms.entry(ce.mrid.trim_start_matches('#').to_string()).or_default().push(mrid.clone());
            }
        }
    }
    let mut v = Vec::new();
    for (eq_id, term_ids) in &eq_terms {
        if term_ids.len() != 2 { continue; }
        let t1 = match dataset.entries.get(&term_ids[0]).and_then(|e| e.element.as_any().downcast_ref::<cimstructs::Terminal>()) { Some(t) => t, None => continue };
        let t2 = match dataset.entries.get(&term_ids[1]).and_then(|e| e.element.as_any().downcast_ref::<cimstructs::Terminal>()) { Some(t) => t, None => continue };
        if let (Some(cn1), Some(cn2)) = (&t1.connectivity_node, &t2.connectivity_node) {
            if cn1.mrid == cn2.mrid {
                v.push(Violation {
                    object_id: eq_id.clone(), rule_id: "eq452:Terminal-connection".into(),
                    name: "C:452:EQ:Terminal:connection".into(), class: "ConductingEquipment".into(),
                    property: "rdf:type".into(),
                    message: "Terminals of a two-terminal equipment connect to the same ConnectivityNode.".into(),
                    severity: "sh:Violation".into(), description: String::new(),
                });
            }
        }
    }
    v
}

fn check_reactive_capability_curve_reactive_count_p(dataset: &CimDataset) -> Vec<Violation> {
    let mut curve_sm: HashMap<String, String> = HashMap::new();
    for mrid in dataset.by_type.get("SynchronousMachine").into_iter().flatten() {
        let entry = &dataset.entries[mrid];
        if let Some(sm) = entry.element.as_any().downcast_ref::<cimstructs::SynchronousMachine>() {
            if let Some(r) = &sm.initial_reactive_capability_curve {
                curve_sm.insert(r.mrid.trim_start_matches('#').to_string(), mrid.clone());
            }
        }
    }
    let mut curve_xcount: HashMap<String, usize> = HashMap::new();
    for mrid in dataset.by_type.get("CurveData").into_iter().flatten() {
        let entry = &dataset.entries[mrid];
        if let Some(cd) = entry.element.as_any().downcast_ref::<cimstructs::CurveData>() {
            if let Some(r) = &cd.curve {
                *curve_xcount.entry(r.mrid.trim_start_matches('#').to_string()).or_default() += 1;
            }
        }
    }
    let mut v = Vec::new();
    for mrid in dataset.by_type.get("ReactiveCapabilityCurve").into_iter().flatten() {
        let sm_id = match curve_sm.get(mrid.as_str()) { Some(id) => id, None => continue };
        let sm = match dataset.entries.get(sm_id).and_then(|e| e.element.as_any().downcast_ref::<cimstructs::SynchronousMachine>()) { Some(s) => s, None => continue };
        let sm_type = match &sm.type_ { Some(t) => t.uri.as_str(), None => continue };
        let count = curve_xcount.get(mrid.as_str()).copied().unwrap_or(0);
        let msg = if sm_type.ends_with("condenser") {
            if count > 0 { Some("SynchronousMachine of type condenser should not have a ReactiveCapabilityCurve.".to_string()) } else { None }
        } else if sm_type.ends_with("generator") || sm_type.ends_with("generatorOrCondenser") {
            if count < 2 { Some(format!("Generator type ReactiveCapabilityCurve needs at least 2 points (found {count}).")) } else { None }
        } else if sm_type.ends_with("motor") || sm_type.ends_with("motorOrCondenser") {
            if count < 2 { Some(format!("Motor type ReactiveCapabilityCurve needs at least 2 points (found {count}).")) } else { None }
        } else if sm_type.ends_with("generatorOrMotor") || sm_type.ends_with("generatorOrCondenserOrMotor") {
            if count < 3 { Some(format!("Combined type ReactiveCapabilityCurve needs at least 3 points (found {count}).")) } else { None }
        } else { None };
        if let Some(msg) = msg {
            v.push(Violation {
                object_id: mrid.clone(), rule_id: "eq452:ReactiveCapabilityCurve-reactiveCountP".into(),
                name: "C:452:EQ:CurveData.Curve:reactiveCountP".into(), class: "ReactiveCapabilityCurve".into(),
                property: "rdf:type".into(),
                message: msg, severity: "sh:Violation".into(), description: String::new(),
            });
        }
    }
    v
}

fn check_reactive_capability_curve_units(dataset: &CimDataset) -> Vec<Violation> {
    let sm_curves: std::collections::HashSet<String> = dataset.by_type.get("SynchronousMachine").into_iter().flatten()
        .filter_map(|mrid| {
            dataset.entries.get(mrid)?.element.as_any().downcast_ref::<cimstructs::SynchronousMachine>()
                .and_then(|sm| sm.initial_reactive_capability_curve.as_ref())
                .map(|r| r.mrid.trim_start_matches('#').to_string())
        }).collect();
    let mut v = Vec::new();
    for mrid in dataset.by_type.get("ReactiveCapabilityCurve").into_iter().flatten() {
        if !sm_curves.contains(mrid.as_str()) { continue; }
        let entry = &dataset.entries[mrid];
        let rcc = match entry.element.as_any().downcast_ref::<cimstructs::ReactiveCapabilityCurve>() { Some(r) => r, None => continue };
        let xu = match &rcc.base.x_unit { Some(r) => r.uri.as_str(), None => continue };
        let y1u = match &rcc.base.y1unit { Some(r) => r.uri.as_str(), None => continue };
        let y2u = match &rcc.base.y2unit { Some(r) => r.uri.as_str(), None => continue };
        if !xu.ends_with('W') || !y1u.ends_with("VAr") || !y2u.ends_with("VAr") {
            v.push(Violation {
                object_id: mrid.clone(), rule_id: "eq600:ReactiveCapabilityCurve-units".into(),
                name: "C:600:EQ:ReactiveCapabilityCurve:units".into(), class: "ReactiveCapabilityCurve".into(),
                property: "rdf:type".into(),
                message: format!("Incorrect units for ReactiveCapabilityCurve (x: {xu}, y1: {y1u}, y2: {y2u}). Expected x: W, y1: VAr, y2: VAr."),
                severity: "sh:Violation".into(), description: String::new(),
            });
        }
    }
    v
}

fn check_substation_count(dataset: &CimDataset) -> Vec<Violation> {
    let substations = dataset.by_type.get("Substation").map(|v| v.len()).unwrap_or(0);
    let voltage_levels = dataset.by_type.get("VoltageLevel").map(|v| v.len()).unwrap_or(0);
    if substations == 1 || (substations > 0 && substations == voltage_levels) {
        vec![Violation {
            object_id: "global".into(), rule_id: "eq600:Substation-count".into(),
            name: "C:600:EQ:Substation:count".into(), class: "Substation".into(),
            property: "rdf:type".into(),
            message: format!("The model has either one Substation or a Substation per VoltageLevel. Number of Substation-s: {substations}. Number of VoltageLevel-s: {voltage_levels}."),
            severity: "sh:Warning".into(),
            description: "The number of Substation-s shall reflect the design of the power system. Cases of a single Substation in a power system model or having a Substation per VoltageLevel are reported as warnings.".into(),
        }]
    } else {
        Vec::new()
    }
}

fn check_tap_changer_neutral_u_value_range(dataset: &CimDataset) -> Vec<Violation> {
    const EPS: f64 = 1e-6;
    let mut v = Vec::new();
    let check = |v: &mut Vec<Violation>, mrid: &str, neutral_u: f64, te_id: &str, class: &str| {
        let pte = match dataset.entries.get(te_id).and_then(|e| e.element.as_any().downcast_ref::<cimstructs::PowerTransformerEnd>()) { Some(p) => p, None => return };
        let rated_u = pte.rated_u.unwrap_or(0.0);
        if (neutral_u - rated_u).abs() > EPS {
            v.push(Violation {
                object_id: mrid.to_string(), rule_id: "eq600:TapChanger.neutralU-valueRangePair".into(),
                name: "C:600:EQ:TapChanger.neutralU:ValueRangePair".into(), class: class.to_string(),
                property: "TapChanger.neutralU".into(),
                message: format!("TapChanger.neutralU ({neutral_u}) is not equal to PowerTransformerEnd.ratedU ({rated_u})."),
                severity: "sh:Violation".into(), description: String::new(),
            });
        }
    };
    for mrid in dataset.by_type.get("RatioTapChanger").into_iter().flatten() {
        let entry = &dataset.entries[mrid];
        if let Some(rtc) = entry.element.as_any().downcast_ref::<cimstructs::RatioTapChanger>() {
            if let (Some(nu), Some(te_r)) = (rtc.base.neutral_u, &rtc.transformer_end) {
                check(&mut v, mrid, nu, te_r.mrid.trim_start_matches('#'), "RatioTapChanger");
            }
        }
    }
    for mrid in dataset.by_type.get("PhaseTapChangerAsymmetrical").into_iter().flatten() {
        let entry = &dataset.entries[mrid];
        if let Some(ptc) = entry.element.as_any().downcast_ref::<cimstructs::PhaseTapChangerAsymmetrical>() {
            if let (Some(nu), Some(te_r)) = (ptc.base.base.base.neutral_u, &ptc.base.base.transformer_end) {
                check(&mut v, mrid, nu, te_r.mrid.trim_start_matches('#'), "PhaseTapChangerAsymmetrical");
            }
        }
    }
    for mrid in dataset.by_type.get("PhaseTapChangerLinear").into_iter().flatten() {
        let entry = &dataset.entries[mrid];
        if let Some(ptc) = entry.element.as_any().downcast_ref::<cimstructs::PhaseTapChangerLinear>() {
            if let (Some(nu), Some(te_r)) = (ptc.base.base.neutral_u, &ptc.base.transformer_end) {
                check(&mut v, mrid, nu, te_r.mrid.trim_start_matches('#'), "PhaseTapChangerLinear");
            }
        }
    }
    for mrid in dataset.by_type.get("PhaseTapChangerSymmetrical").into_iter().flatten() {
        let entry = &dataset.entries[mrid];
        if let Some(ptc) = entry.element.as_any().downcast_ref::<cimstructs::PhaseTapChangerSymmetrical>() {
            if let (Some(nu), Some(te_r)) = (ptc.base.base.base.neutral_u, &ptc.base.base.transformer_end) {
                check(&mut v, mrid, nu, te_r.mrid.trim_start_matches('#'), "PhaseTapChangerSymmetrical");
            }
        }
    }
    for mrid in dataset.by_type.get("PhaseTapChangerTabular").into_iter().flatten() {
        let entry = &dataset.entries[mrid];
        if let Some(ptc) = entry.element.as_any().downcast_ref::<cimstructs::PhaseTapChangerTabular>() {
            if let (Some(nu), Some(te_r)) = (ptc.base.base.neutral_u, &ptc.base.transformer_end) {
                check(&mut v, mrid, nu, te_r.mrid.trim_start_matches('#'), "PhaseTapChangerTabular");
            }
        }
    }
    v
}
