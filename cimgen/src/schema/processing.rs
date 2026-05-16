use std::collections::HashMap;

use super::model::*;

pub fn postprocess(spec: &mut CimSpecification) {
    determine_data_types(spec);
    add_origins_of_attributes(spec);
    set_profile_priorities(spec);
    reorder_origins(spec);
    set_main_origin(spec);
    set_has_inverse_role(spec);
    set_has_class_attributes(spec);
    set_is_fixed_attributes(spec);
    set_missing_namespaces(spec);
    sort_attributes(spec);
    set_is_inverse_role_attribute_list(spec);
    rename_conflicting_attributes(spec);
    remove_circular_dependencies(spec);
}

// Pass 1: classify every attribute as primitive / CIMDatatype / enum / class.
fn determine_data_types(spec: &mut CimSpecification) {
    // First: resolve primitive_type for each CIMDatatype via its "value" attribute.
    let dt_ids: Vec<String> = spec.cim_datatypes.keys().cloned().collect();
    for id in &dt_ids {
        let dt = spec.cim_datatypes.get(id).unwrap();
        if dt.cim_stereotype == "Compound" {
            continue;
        }
        let prim = dt
            .attributes
            .iter()
            .find(|a| a.label == "value")
            .map(|a| {
                if a.cim_data_type == DATA_TYPE_DECIMAL {
                    DATA_TYPE_FLOAT.to_string()
                } else {
                    a.cim_data_type.clone()
                }
            })
            .unwrap_or_default();
        spec.cim_datatypes.get_mut(id).unwrap().primitive_type = prim;
    }

    // Collect sets we need for look-up (avoid borrowing spec mutably while reading it)
    let enum_ids: std::collections::HashSet<String> = spec.enums.keys().cloned().collect();
    let dt_ids_set: std::collections::HashSet<String> =
        spec.cim_datatypes.keys().cloned().collect();
    let dt_prim: HashMap<String, String> = spec
        .cim_datatypes
        .iter()
        .map(|(k, v)| (k.clone(), v.primitive_type.clone()))
        .collect();

    // Then: classify each attribute in every CimType.
    for t in spec.types.values_mut() {
        let mut prim_set: std::collections::HashSet<String> = Default::default();
        let mut dt_set: std::collections::HashSet<String> = Default::default();
        let mut enum_set: std::collections::HashSet<String> = Default::default();

        for attr in &mut t.attributes {
            if attr.cim_stereotype == "Primitive" || is_primitive_type(&attr.cim_data_type) {
                attr.is_primitive = true;
                attr.data_type = if attr.cim_data_type == DATA_TYPE_DECIMAL {
                    DATA_TYPE_FLOAT.to_string()
                } else {
                    attr.cim_data_type.clone()
                };
            } else if attr.cim_stereotype == "CIMDatatype"
                || dt_ids_set.contains(&attr.cim_data_type)
            {
                attr.is_cim_datatype = true;
                attr.data_type = dt_prim
                    .get(&attr.cim_data_type)
                    .cloned()
                    .unwrap_or_default();
            } else if enum_ids.contains(&attr.rdf_range) {
                attr.is_enum_value = true;
                attr.data_type = attr.rdf_range.clone();
            } else if !attr.is_list
                && (attr.cim_data_type == "Object" || attr.cim_data_type.is_empty())
            {
                attr.is_class = true;
                attr.data_type = attr.rdf_range.clone();
            } else if !attr.is_list && !attr.cim_data_type.is_empty() {
                attr.is_class = true;
                attr.data_type = attr.cim_data_type.clone();
            } else {
                attr.data_type = attr.rdf_range.clone();
            }

            if attr.is_primitive {
                prim_set.insert(attr.data_type.clone());
            } else if attr.is_cim_datatype {
                dt_set.insert(attr.cim_data_type.clone());
            } else if attr.is_enum_value {
                enum_set.insert(attr.rdf_range.clone());
            }
        }

        let mut prim_vec: Vec<String> = prim_set.into_iter().collect();
        prim_vec.sort();
        t.primitive_types = prim_vec;

        let mut dt_vec: Vec<String> = dt_set.into_iter().collect();
        dt_vec.sort();
        t.cim_datatypes = dt_vec;

        let mut enum_vec: Vec<String> = enum_set.into_iter().collect();
        enum_vec.sort();
        t.enum_types = enum_vec;
    }

    // Primitives: map known ids to data types.
    for p in spec.primitive_types.values_mut() {
        p.data_type = match p.id.as_str() {
            DATA_TYPE_STRING => DATA_TYPE_STRING.to_string(),
            DATA_TYPE_INTEGER => DATA_TYPE_INTEGER.to_string(),
            DATA_TYPE_BOOLEAN => DATA_TYPE_BOOLEAN.to_string(),
            DATA_TYPE_FLOAT => DATA_TYPE_FLOAT.to_string(),
            DATA_TYPE_DECIMAL => DATA_TYPE_FLOAT.to_string(),
            _ => DATA_TYPE_STRING.to_string(),
        };
    }
}

fn is_primitive_type(s: &str) -> bool {
    matches!(
        s,
        DATA_TYPE_STRING
            | DATA_TYPE_INTEGER
            | DATA_TYPE_BOOLEAN
            | DATA_TYPE_FLOAT
            | DATA_TYPE_DATE
            | DATA_TYPE_DATE_TIME
            | DATA_TYPE_MONTH_DAY
            | "URI"
    )
}

// Pass 2: merge attribute origins up into the owning type's origins.
fn add_origins_of_attributes(spec: &mut CimSpecification) {
    for t in spec.types.values_mut() {
        let mut set: std::collections::HashSet<String> =
            t.origins.iter().cloned().collect();
        for attr in &t.attributes {
            for o in &attr.origins {
                set.insert(o.clone());
            }
        }
        let mut v: Vec<String> = set.into_iter().collect();
        v.sort();
        t.origins = v;
    }
}

// Pass 3: assign priorities — EQ=1, rest alphabetical from 2.
fn set_profile_priorities(spec: &mut CimSpecification) {
    if let Some(eq) = spec.ontologies.get_mut("EQ") {
        eq.priority = 1;
    }
    let mut others: Vec<String> = spec
        .ontologies
        .keys()
        .filter(|k| k.as_str() != "EQ")
        .cloned()
        .collect();
    others.sort();
    let mut p = 2u32;
    for k in others {
        if let Some(o) = spec.ontologies.get_mut(&k) {
            o.priority = p;
        }
        p += 1;
    }
    // Build ordered list
    let mut list: Vec<(u32, String)> = spec
        .ontologies
        .iter()
        .map(|(k, v)| (v.priority, k.clone()))
        .collect();
    list.sort_by_key(|(p, _)| *p);
    spec.ontology_list = list.into_iter().map(|(_, k)| k).collect();
}

// Pass 4: sort origins by priority.
fn reorder_origins(spec: &mut CimSpecification) {
    let prio: HashMap<String, u32> = spec
        .ontologies
        .iter()
        .map(|(k, v)| (k.clone(), v.priority))
        .collect();

    for t in spec.types.values_mut() {
        t.origins.sort_by_key(|o| prio.get(o).copied().unwrap_or(u32::MAX));
        for attr in &mut t.attributes {
            attr.origins
                .sort_by_key(|o| prio.get(o).copied().unwrap_or(u32::MAX));
        }
    }
}

// Pass 5: select the dominant origin per type.
fn set_main_origin(spec: &mut CimSpecification) {
    let type_ids: Vec<String> = spec.types.keys().cloned().collect();
    for id in type_ids {
        let origin = compute_main_origin(&id, &spec.types);
        if let Some(t) = spec.types.get_mut(&id) {
            t.origin = origin;
        }
    }
}

fn compute_main_origin(id: &str, types: &HashMap<String, CimType>) -> String {
    let mut counts: HashMap<String, usize> = HashMap::new();
    let mut current_id = id.to_string();

    loop {
        let t = match types.get(&current_id) {
            Some(t) => t,
            None => break,
        };
        for attr in &t.attributes {
            if attr.origins.len() > 1 {
                for o in &attr.origins {
                    if t.origins.contains(o) {
                        *counts.entry(o.clone()).or_insert(0) += 1;
                    }
                }
            }
        }
        if t.super_type.is_empty() || t.super_type == current_id {
            break;
        }
        current_id = t.super_type.clone();
    }

    let t = match types.get(id) {
        Some(t) => t,
        None => return String::new(),
    };

    let candidates: Vec<String> = if counts.is_empty() {
        t.origins.clone()
    } else {
        let max = *counts.values().max().unwrap_or(&0);
        counts
            .into_iter()
            .filter(|(_, c)| *c == max)
            .map(|(o, _)| o)
            .collect()
    };

    if candidates.contains(&"EQ".to_string()) {
        "EQ".to_string()
    } else {
        let mut s = candidates;
        s.sort();
        s.into_iter().next().unwrap_or_default()
    }
}

// Pass 6: populate has_inverse_role and inverse_role_attribute.
fn set_has_inverse_role(spec: &mut CimSpecification) {
    for t in spec.types.values_mut() {
        for attr in &mut t.attributes {
            if !attr.cim_inverse_role.is_empty() {
                attr.has_inverse_role = true;
                if let Some((_, name)) = attr.cim_inverse_role.split_once('.') {
                    attr.inverse_role_attribute = name.to_string();
                }
            }
        }
    }
}

// Pass 7: mark types that have class-valued (non-list) attributes.
fn set_has_class_attributes(spec: &mut CimSpecification) {
    for t in spec.types.values_mut() {
        t.has_class_attributes = t
            .attributes
            .iter()
            .any(|a| a.is_class && !a.is_list);
    }
}

// Pass 8: mark fixed attributes in CIMDatatypes.
fn set_is_fixed_attributes(spec: &mut CimSpecification) {
    for dt in spec.cim_datatypes.values_mut() {
        for attr in &mut dt.attributes {
            attr.is_fixed = !attr.cim_is_fixed.is_empty();
        }
    }
}

// Pass 9: fill missing namespaces; build profile_namespaces.
fn set_missing_namespaces(spec: &mut CimSpecification) {
    let base = spec
        .specification_namespaces
        .get("base")
        .cloned()
        .unwrap_or_default();

    for t in spec.types.values_mut() {
        normalize_ns(&mut t.namespace, &base);
        for attr in &mut t.attributes {
            normalize_ns(&mut attr.namespace, &base);
        }
    }
    for e in spec.enums.values_mut() {
        normalize_ns(&mut e.namespace, &base);
    }

    // Build reverse map ns_url → prefix
    let rev: HashMap<String, String> = spec
        .specification_namespaces
        .iter()
        .filter(|(k, _)| k.as_str() != "base")
        .map(|(k, v)| (v.clone(), k.clone()))
        .collect();

    for t in spec.types.values() {
        if let Some(prefix) = rev.get(&t.namespace) {
            spec.profile_namespaces
                .insert(prefix.clone(), t.namespace.clone());
        }
    }
    for e in spec.enums.values() {
        if let Some(prefix) = rev.get(&e.namespace) {
            spec.profile_namespaces
                .insert(prefix.clone(), e.namespace.clone());
        }
    }

    let md = "http://iec.ch/TC57/61970-552/ModelDescription/1#".to_string();
    let rdf = "http://www.w3.org/1999/02/22-rdf-syntax-ns#".to_string();
    spec.specification_namespaces
        .entry("md".to_string())
        .or_insert(md.clone());
    spec.profile_namespaces.entry("md".to_string()).or_insert(md);
    spec.specification_namespaces
        .entry("rdf".to_string())
        .or_insert(rdf.clone());
    spec.profile_namespaces
        .entry("rdf".to_string())
        .or_insert(rdf);
}

fn normalize_ns(ns: &mut String, base: &str) {
    if !ns.ends_with('#') {
        ns.push('#');
    }
    if ns.is_empty() || ns == "#" {
        *ns = base.to_string();
    }
}

// Pass 10: sort attributes by id within each type.
fn sort_attributes(spec: &mut CimSpecification) {
    for t in spec.types.values_mut() {
        t.attributes.sort_by(|a, b| a.id.cmp(&b.id));
    }
    for dt in spec.cim_datatypes.values_mut() {
        dt.attributes.sort_by(|a, b| a.id.cmp(&b.id));
    }
}

// Pass 11: set is_inverse_role_attribute_list.
fn set_is_inverse_role_attribute_list(spec: &mut CimSpecification) {
    // Collect (type_id, attr_idx) pairs that should be flagged, then apply.
    let mut flags: Vec<(String, usize)> = Vec::new();
    for (tid, t) in &spec.types {
        for (idx, attr) in t.attributes.iter().enumerate() {
            if let Some((inv_type, inv_label)) = attr.cim_inverse_role.split_once('.') {
                if let Some(inv_t) = spec.types.get(inv_type) {
                    if inv_t.attributes.iter().any(|a| a.label == inv_label && a.is_list) {
                        flags.push((tid.clone(), idx));
                    }
                }
            }
        }
    }
    for (tid, idx) in flags {
        if let Some(t) = spec.types.get_mut(&tid) {
            t.attributes[idx].is_inverse_role_attribute_list = true;
        }
    }
}

// Pass 12: rename labels that conflict with language keywords.
fn rename_conflicting_attributes(spec: &mut CimSpecification) {
    for t in spec.types.values_mut() {
        for attr in &mut t.attributes {
            if attr.label == "switch" {
                attr.label = "switch_".to_string();
            } else if attr.label == "IdentifiedObject" {
                attr.label = "IdentifiedObject_".to_string();
            }
        }
    }
}

// Pass 13: mark all non-primitive, non-enum, non-CIMDatatype attributes as ID references.
fn remove_circular_dependencies(spec: &mut CimSpecification) {
    for t in spec.types.values_mut() {
        for attr in &mut t.attributes {
            if !attr.is_primitive && !attr.is_enum_value && !attr.is_cim_datatype {
                attr.use_id_reference = true;
            }
        }
    }
}
