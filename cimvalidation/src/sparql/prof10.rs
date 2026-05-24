use cimdecoder::CimDataset;
use crate::Violation;

const PROF_BASE: &str = "http://iec.ch/TC57/ns/CIM/";
const PROF_EQ:   &str = "http://iec.ch/TC57/ns/CIM/CoreEquipment-EU/3.0";
const PROF_EQBD: &str = "http://iec.ch/TC57/ns/CIM/EquipmentBoundary-EU/3.0";
const PROF_DY:   &str = "http://iec.ch/TC57/ns/CIM/Dynamics-EU/1.0";
const PROF_DL:   &str = "http://iec.ch/TC57/ns/CIM/DiagramLayout-EU/3.0";
const PROF_SC:   &str = "http://iec.ch/TC57/ns/CIM/ShortCircuit-EU/3.0";
const PROF_OP:   &str = "http://iec.ch/TC57/ns/CIM/Operation-EU/3.0";
const PROF_GL:   &str = "http://iec.ch/TC57/ns/CIM/GeographicalLocation-EU/3.0";
const PROF_SV:   &str = "http://iec.ch/TC57/ns/CIM/StateVariables-EU/3.0";
const PROF_TP:   &str = "http://iec.ch/TC57/ns/CIM/Topology-EU/3.0";
const PROF_SSH:  &str = "http://iec.ch/TC57/ns/CIM/SteadyStateHypothesis-EU/3.0";

pub fn validate(dataset: &CimDataset) -> Vec<Violation> {
    let mut v = Vec::new();
    for mrid in dataset.by_type.get("FullModel").into_iter().flatten() {
        let entry = &dataset.entries[mrid];
        if let Some(fm) = entry.element.as_any().downcast_ref::<cimstructs::FullModel>() {
            v.extend(check_prof10_model(mrid, &fm.base, dataset));
        }
    }
    v
}

fn profile_uri(profiles: &[String]) -> &str {
    for p in profiles {
        let p = p.trim();
        if !p.is_empty() { return p; }
    }
    ""
}

fn dependent_on_profiles(dependent_on: &[cimstructs::base::MridRef], dataset: &CimDataset) -> Vec<String> {
    if dependent_on.is_empty() { return Vec::new(); }
    let mut out = Vec::new();
    for dep_ref in dependent_on {
        let dep_id = dep_ref.mrid.trim_start_matches('#');
        if let Some(entry) = dataset.entries.get(dep_id) {
            let prof = if let Some(fm) = entry.element.as_any().downcast_ref::<cimstructs::FullModel>() {
                profile_uri(&fm.base.profile).to_string()
            } else if let Some(dm) = entry.element.as_any().downcast_ref::<cimstructs::DifferenceModel>() {
                profile_uri(&dm.base.profile).to_string()
            } else {
                "external".to_string()
            };
            out.push(prof);
        } else {
            out.push("external".to_string());
        }
    }
    out
}

fn has_value(deps: &[String], target: &str) -> bool {
    deps.iter().any(|d| d == target)
}

fn all_in_set(deps: &[String], allowed: &[&str]) -> bool {
    deps.iter().all(|d| d == "external" || allowed.contains(&d.as_str()))
}

fn dataset_has_profile(dataset: &CimDataset, prof: &str) -> bool {
    for type_name in &["FullModel", "DifferenceModel"] {
        for mrid in dataset.by_type.get(*type_name).into_iter().flatten() {
            let entry = &dataset.entries[mrid];
            let p = if let Some(fm) = entry.element.as_any().downcast_ref::<cimstructs::FullModel>() {
                profile_uri(&fm.base.profile).to_string()
            } else if let Some(dm) = entry.element.as_any().downcast_ref::<cimstructs::DifferenceModel>() {
                profile_uri(&dm.base.profile).to_string()
            } else {
                continue;
            };
            if p == prof { return true; }
        }
    }
    false
}

fn prof10_violation(id: &str, msg: &str, severity: &str) -> Violation {
    Violation {
        object_id:   id.to_string(),
        rule_id:     "prof10:PROF10".into(),
        name:        "PROF10".into(),
        class:       "FullModel".into(),
        property:    "Model.DependentOn".into(),
        message:     msg.to_string(),
        severity:    severity.to_string(),
        description: "CGMES instance file (distribution) dependency shall be declared by md:Model.DependentOn in the header according to Figure 1 and the associated rules.".into(),
    }
}

fn check_prof10_model(id: &str, m: &cimstructs::Model, dataset: &CimDataset) -> Vec<Violation> {
    match profile_uri(&m.profile) {
        PROF_EQ  => check_prof10_eq(id, m, dataset),
        PROF_DY  => check_prof10_dy(id, m, dataset),
        PROF_DL  => check_prof10_dl(id, m, dataset),
        PROF_SC  => check_prof10_sc(id, m, dataset),
        PROF_OP  => check_prof10_op(id, m, dataset),
        PROF_GL  => check_prof10_gl(id, m, dataset),
        PROF_SV  => check_prof10_sv(id, m, dataset),
        PROF_TP  => check_prof10_tp(id, m, dataset),
        PROF_SSH => check_prof10_ssh(id, m, dataset),
        _        => Vec::new(),
    }
}

const MSG_EQ: &str = "The EQ does not have reference to EQBD. The file header dependencies cardinalities and types for EQ profile are not according to PROF10.";
fn check_prof10_eq(id: &str, m: &cimstructs::Model, dataset: &CimDataset) -> Vec<Violation> {
    let deps = dependent_on_profiles(&m.dependent_on, dataset);
    if has_value(&deps, PROF_EQBD) || has_value(&deps, "external") { return Vec::new(); }
    vec![prof10_violation(id, MSG_EQ, "sh:Info")]
}

const MSG_DY: &str = "The file header dependencies cardinalities and types for DY profile are not according to PROF10.";
fn check_prof10_dy(id: &str, m: &cimstructs::Model, dataset: &CimDataset) -> Vec<Violation> {
    let deps = dependent_on_profiles(&m.dependent_on, dataset);
    if deps.is_empty() { return vec![prof10_violation(id, MSG_DY, "sh:Violation")]; }
    if has_value(&deps, PROF_EQ) { return Vec::new(); }
    if has_value(&deps, "external") && !dataset_has_profile(dataset, PROF_EQ) { return Vec::new(); }
    vec![prof10_violation(id, MSG_DY, "sh:Violation")]
}

const MSG_DL: &str = "The file header dependencies cardinalities and types for DL profile are not according to PROF10.";
fn check_prof10_dl(id: &str, m: &cimstructs::Model, dataset: &CimDataset) -> Vec<Violation> {
    let deps = dependent_on_profiles(&m.dependent_on, dataset);
    if !all_in_set(&deps, &[PROF_DY, PROF_TP, PROF_EQ, PROF_SC, PROF_OP]) {
        return vec![prof10_violation(id, MSG_DL, "sh:Violation")];
    }
    Vec::new()
}

const MSG_SC: &str = "The file header dependencies cardinalities and types for SC profile are not according to PROF10.";
fn check_prof10_sc(id: &str, m: &cimstructs::Model, dataset: &CimDataset) -> Vec<Violation> {
    let deps = dependent_on_profiles(&m.dependent_on, dataset);
    if deps.is_empty() { return vec![prof10_violation(id, MSG_SC, "sh:Violation")]; }
    if !all_in_set(&deps, &[PROF_EQ, PROF_EQBD, PROF_OP]) {
        return vec![prof10_violation(id, MSG_SC, "sh:Violation")];
    }
    Vec::new()
}

const MSG_OP: &str = "The file header dependencies cardinalities and types for OP profile are not according to PROF10.";
fn check_prof10_op(id: &str, m: &cimstructs::Model, dataset: &CimDataset) -> Vec<Violation> {
    let deps = dependent_on_profiles(&m.dependent_on, dataset);
    if deps.is_empty() { return vec![prof10_violation(id, MSG_OP, "sh:Violation")]; }
    if !all_in_set(&deps, &[PROF_EQ, PROF_EQBD, PROF_SC]) {
        return vec![prof10_violation(id, MSG_OP, "sh:Violation")];
    }
    Vec::new()
}

const MSG_GL: &str = "The file header dependencies cardinalities and types for GL profile are not according to PROF10.";
fn check_prof10_gl(id: &str, m: &cimstructs::Model, dataset: &CimDataset) -> Vec<Violation> {
    let deps = dependent_on_profiles(&m.dependent_on, dataset);
    if !all_in_set(&deps, &[PROF_EQBD, PROF_EQ, PROF_SC, PROF_OP]) {
        return vec![prof10_violation(id, MSG_GL, "sh:Violation")];
    }
    Vec::new()
}

const MSG_SV: &str = "The file header dependencies cardinalities and types for SV profile are not according to PROF10.";
fn check_prof10_sv(id: &str, m: &cimstructs::Model, dataset: &CimDataset) -> Vec<Violation> {
    let deps = dependent_on_profiles(&m.dependent_on, dataset);
    if deps.is_empty() { return vec![prof10_violation(id, MSG_SV, "sh:Violation")]; }
    if has_value(&deps, PROF_TP) { return Vec::new(); }
    if has_value(&deps, "external") && !dataset_has_profile(dataset, PROF_TP) { return Vec::new(); }
    vec![prof10_violation(id, MSG_SV, "sh:Violation")]
}

const MSG_TP: &str = "The file header dependencies cardinalities and types for TP profile are not according to PROF10.";
fn check_prof10_tp(id: &str, m: &cimstructs::Model, dataset: &CimDataset) -> Vec<Violation> {
    let deps = dependent_on_profiles(&m.dependent_on, dataset);
    if deps.is_empty() { return vec![prof10_violation(id, MSG_TP, "sh:Violation")]; }
    if has_value(&deps, PROF_SSH) { return Vec::new(); }
    if has_value(&deps, "external") && !dataset_has_profile(dataset, PROF_SSH) { return Vec::new(); }
    vec![prof10_violation(id, MSG_TP, "sh:Violation")]
}

const MSG_SSH: &str = "The file header dependencies cardinalities and types for SSH profile are not according to PROF10.";
fn check_prof10_ssh(id: &str, m: &cimstructs::Model, dataset: &CimDataset) -> Vec<Violation> {
    let deps = dependent_on_profiles(&m.dependent_on, dataset);
    if deps.is_empty() { return vec![prof10_violation(id, MSG_SSH, "sh:Violation")]; }
    if has_value(&deps, PROF_EQ) { return Vec::new(); }
    if has_value(&deps, "external") && !dataset_has_profile(dataset, PROF_EQ) { return Vec::new(); }
    vec![prof10_violation(id, MSG_SSH, "sh:Violation")]
}
