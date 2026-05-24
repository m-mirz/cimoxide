use cimstructs::base::FieldValue;
use cimdecoder::CimDataset;
use crate::Violation;

pub fn validate(dataset: &CimDataset) -> Vec<Violation> {
    let mut v = Vec::new();
    v.extend(check_excitation_system_smd(dataset));
    v.extend(check_smtcr_model_type(dataset));
    v.extend(check_turbine_governor_mbase(dataset));
    v.extend(check_excitation_system_gains(dataset));
    v.extend(check_pss_input_signals(dataset));
    v.extend(check_gov_hydro4_gain_points(dataset));
    v.extend(check_load_static_model_attributes(dataset));
    v.extend(check_rotating_machine_saturation(dataset));
    v.extend(check_synchronous_machine_simplified_attributes(dataset));
    v.extend(check_dynamics_associations(dataset));
    v
}

// -- ExcitationSystemDynamics.SynchronousMachineDynamics check --

macro_rules! check_exc_smd_type {
    ($v:expr, $dataset:expr, $($T:ident),+) => {$(
        for mrid in $dataset.by_type.get(stringify!($T)).into_iter().flatten() {
            let entry = &$dataset.entries[mrid];
            if let Some(obj) = entry.element.as_any().downcast_ref::<cimstructs::$T>() {
                if let Some(smd_ref) = obj.base.synchronous_machine_dynamics.as_ref() {
                    let target_id = smd_ref.mrid.trim_start_matches('#');
                    let is_simplified = $dataset.entries.get(target_id)
                        .map_or(false, |e| e.element.type_name() == "SynchronousMachineSimplified");
                    if is_simplified {
                        $v.push(Violation {
                            object_id:   mrid.clone(),
                            rule_id:     "dy457:ExcitationSystemDynamics.SynchronousMachineDynamicsSynchronousMachineSimplified-valueType".into(),
                            name:        "ExcitationSystemDynamics.SynchronousMachineDynamicsSynchronousMachineSimplified-valueType".into(),
                            class:       stringify!($T).to_string(),
                            property:    "ExcitationSystemDynamics.SynchronousMachineDynamics".into(),
                            message:     "The association ExcitationSystemDynamics.SynchronousMachineDynamics points to an object of type SynchronousMachineSimplified.".into(),
                            severity:    "sh:Violation".into(),
                            description: String::new(),
                        });
                    }
                }
            }
        }
    )+};
}

fn check_excitation_system_smd(dataset: &CimDataset) -> Vec<Violation> {
    let mut v = Vec::new();
    check_exc_smd_type!(v, dataset,
        ExcAC1A, ExcAC2A, ExcAC3A, ExcAC4A, ExcAC5A, ExcAC6A, ExcAC8B,
        ExcANS, ExcAVR1, ExcAVR2, ExcAVR3, ExcAVR4, ExcAVR5, ExcAVR7,
        ExcBBC, ExcCZ, ExcDC1A, ExcDC2A, ExcDC3A, ExcDC3A1,
        ExcELIN1, ExcELIN2, ExcHU,
        ExcIEEEAC1A, ExcIEEEAC2A, ExcIEEEAC3A, ExcIEEEAC4A, ExcIEEEAC5A, ExcIEEEAC6A,
        ExcIEEEAC7B, ExcIEEEAC8B, ExcIEEEDC1A, ExcIEEEDC2A, ExcIEEEDC3A, ExcIEEEDC4B,
        ExcIEEEST1A, ExcIEEEST2A, ExcIEEEST3A, ExcIEEEST4B, ExcIEEEST5B, ExcIEEEST6B, ExcIEEEST7B,
        ExcNI, ExcOEX3T, ExcPIC, ExcREXS, ExcRQB, ExcSCRX, ExcSEXS, ExcSK,
        ExcST1A, ExcST2A, ExcST3A, ExcST4B, ExcST6B, ExcST7B,
        ExcitationSystemUserDefined
    );
    v
}

// -- SynchronousMachineTimeConstantReactance model type check --

fn check_smtcr_model_type(dataset: &CimDataset) -> Vec<Violation> {
    const SUBTRANS_SIMPLIFIED: &str = "SynchronousMachineModelKind.subtransientSimplified";
    const SUBTRANS:            &str = "SynchronousMachineModelKind.subtransient";
    const ROUND_ROTOR:         &str = "RotorKind.roundRotor";
    const SALIENT_POLE:        &str = "RotorKind.salientPole";

    let mut v = Vec::new();
    for mrid in dataset.by_type.get("SynchronousMachineTimeConstantReactance").into_iter().flatten() {
        let entry = &dataset.entries[mrid];
        let obj = match entry.element.as_any().downcast_ref::<cimstructs::SynchronousMachineTimeConstantReactance>() {
            Some(o) => o, None => continue,
        };
        let mt = match obj.model_type.as_ref() { Some(r) => r.uri.as_str(), None => continue };
        let rt = match obj.rotor_type.as_ref() { Some(r) => r.uri.as_str(), None => continue };

        // stator_resistance, saturation factors in base.base.base (RotatingMachineDynamics)
        let rmd = &obj.base.base.base;
        let det = &obj.base;

        if mt == SUBTRANS_SIMPLIFIED && rt == ROUND_ROTOR {
            if rmd.stator_resistance.unwrap_or(0.0) != 0.0 ||
               det.saturation_factor_q_axis.unwrap_or(0.0) != 0.0 ||
               det.saturation_factor120q_axis.unwrap_or(0.0) != 0.0
            {
                v.push(viol(mrid, "SynchronousMachineTimeConstantReactance", "SynchronousMachineTimeConstantReactance.modelType",
                    "Missing attributes or default values not provided according to 61970-457 Annex A (subtransientSimplified/roundRotor)."));
            }
        } else if mt == SUBTRANS && rt == ROUND_ROTOR {
            if det.saturation_factor_q_axis.unwrap_or(0.0) == 0.0 ||
               det.saturation_factor120q_axis.unwrap_or(0.0) == 0.0 ||
               rmd.saturation_factor.unwrap_or(0.0) == 0.0 ||
               rmd.saturation_factor120.unwrap_or(0.0) == 0.0 ||
               obj.x_quad_trans.unwrap_or(0.0) == 0.0 ||
               obj.tpqo.unwrap_or(0.0) == 0.0
            {
                v.push(viol(mrid, "SynchronousMachineTimeConstantReactance", "SynchronousMachineTimeConstantReactance.modelType",
                    "Missing attributes or default values not provided according to 61970-457 Annex A (subtransient/roundRotor)."));
            }
        } else if mt == SUBTRANS && rt == SALIENT_POLE {
            if det.saturation_factor_q_axis.unwrap_or(0.0) != 0.0 ||
               det.saturation_factor120q_axis.unwrap_or(0.0) != 0.0
            {
                v.push(viol(mrid, "SynchronousMachineTimeConstantReactance", "SynchronousMachineTimeConstantReactance.modelType",
                    "Missing attributes or default values not provided according to 61970-457 Annex A (subtransient/salientPole)."));
            }
        }
    }
    v
}

fn viol(mrid: &str, class: &str, property: &str, message: &str) -> Violation {
    Violation {
        object_id:   mrid.to_string(),
        rule_id:     "dy457:SynchronousMachineTimeConstantReactance-modelType rules".into(),
        name:        "SynchronousMachineTimeConstantReactance-modelType rules".into(),
        class:       class.to_string(),
        property:    property.to_string(),
        message:     message.to_string(),
        severity:    "sh:Violation".into(),
        description: String::new(),
    }
}

// -- TurbineGovernorDynamics mwbase check --

macro_rules! check_gov_mwbase {
    ($v:expr, $dataset:expr, $($T:ident),+) => {$(
        for mrid in $dataset.by_type.get(stringify!($T)).into_iter().flatten() {
            let entry = &$dataset.entries[mrid];
            if let Some(obj) = entry.element.as_any().downcast_ref::<cimstructs::$T>() {
                let mwbase = match obj.mwbase { Some(v) if v != 0.0 => v, _ => continue };
                let smd_id = match obj.base.synchronous_machine_dynamics.as_ref() {
                    Some(r) => r.mrid.trim_start_matches('#').to_string(), None => continue,
                };
                let smd_entry = match $dataset.entries.get(&smd_id) { Some(e) => e, None => continue };
                // Get SynchronousMachineDynamics.SynchronousMachine via to_block
                let smd_block = smd_entry.element.to_block();
                let sm_id = match smd_block.fields.get("SynchronousMachineDynamics.SynchronousMachine") {
                    Some(FieldValue::Resource(s)) => s.trim_start_matches('#').to_string(),
                    _ => continue,
                };
                let sm = match $dataset.entries.get(&sm_id)
                    .and_then(|e| e.element.as_any().downcast_ref::<cimstructs::SynchronousMachine>())
                { Some(o) => o, None => continue };
                let rated_pf = sm.base.rated_power_factor.unwrap_or(0.0);
                let rated_s  = sm.base.rated_s.unwrap_or(0.0);
                let expected = rated_pf * rated_s;
                if (mwbase - expected).abs() > 0.001 {
                    $v.push(Violation {
                        object_id:   mrid.clone(),
                        rule_id:     "dyn457:TurbineGovernorDynamics-mbaseEquation".into(),
                        name:        "TurbineGovernorDynamics-mbaseEquation".into(),
                        class:       stringify!($T).to_string(),
                        property:    "mwbase".into(),
                        message:     format!("The value {mwbase} does not equal RotatingMachine.ratedPowerFactor * RotatingMachine.ratedS ({expected})."),
                        severity:    "sh:Violation".into(),
                        description: String::new(),
                    });
                }
            }
        }
    )+};
}

fn check_turbine_governor_mbase(dataset: &CimDataset) -> Vec<Violation> {
    let mut v = Vec::new();
    check_gov_mwbase!(v, dataset,
        GovCT1, GovCT2, GovGAST, GovGAST1, GovGAST2, GovGASTWD,
        GovHydro1, GovHydro2, GovHydro3, GovHydro4, GovHydroDD,
        GovHydroIEEE0, GovHydroIEEE2, GovHydroPID, GovHydroPID2,
        GovHydroR, GovHydroWEH, GovHydroWPID,
        GovSteam0, GovSteam1, GovSteamEU,
        GovSteamFV2, GovSteamFV3, GovSteamIEEE1, GovSteamSGO
    );
    v
}

// -- ExcitationSystem gain checks --

fn check_excitation_system_gains(dataset: &CimDataset) -> Vec<Violation> {
    let mut v = Vec::new();
    const RULE: &str = "various gain rules for excitation systems";

    for mrid in dataset.by_type.get("ExcAC8B").into_iter().flatten() {
        let entry = &dataset.entries[mrid];
        if let Some(obj) = entry.element.as_any().downcast_ref::<cimstructs::ExcAC8B>() {
            if obj.kir.unwrap_or(0.0) == 0.0 && obj.kpr.unwrap_or(0.0) <= 0.0 {
                v.push(dyn_viol(mrid, RULE, "ExcAC8B", "ExcAC8B.kpr", "The value negative or zero when ExcAC8B.kir = 0."));
            }
        }
    }
    for mrid in dataset.by_type.get("ExcIEEEAC8B").into_iter().flatten() {
        let entry = &dataset.entries[mrid];
        if let Some(obj) = entry.element.as_any().downcast_ref::<cimstructs::ExcIEEEAC8B>() {
            if obj.kir.unwrap_or(0.0) == 0.0 && obj.kpr.unwrap_or(0.0) <= 0.0 {
                v.push(dyn_viol(mrid, RULE, "ExcIEEEAC8B", "ExcIEEEAC8B.kpr", "The value negative or zero when ExcIEEEAC8B.kir = 0."));
            }
        }
    }
    for mrid in dataset.by_type.get("ExcIEEEAC7B").into_iter().flatten() {
        let entry = &dataset.entries[mrid];
        if let Some(obj) = entry.element.as_any().downcast_ref::<cimstructs::ExcIEEEAC7B>() {
            if obj.kia.unwrap_or(0.0) == 0.0 && obj.kpa.unwrap_or(0.0) <= 0.0 {
                v.push(dyn_viol(mrid, RULE, "ExcIEEEAC7B", "ExcIEEEAC7B.kpa", "The value negative or zero when ExcIEEEAC7B.kia = 0."));
            }
            if obj.kir.unwrap_or(0.0) == 0.0 && obj.kpr.unwrap_or(0.0) <= 0.0 {
                v.push(dyn_viol(mrid, RULE, "ExcIEEEAC7B", "ExcIEEEAC7B.kpr", "The value negative or zero when ExcIEEEAC7B.kir = 0."));
            }
        }
    }
    for mrid in dataset.by_type.get("ExcBBC").into_iter().flatten() {
        let entry = &dataset.entries[mrid];
        if let Some(obj) = entry.element.as_any().downcast_ref::<cimstructs::ExcBBC>() {
            if obj.k.unwrap_or(0.0) == 0.0 {
                v.push(dyn_viol(mrid, RULE, "ExcBBC", "ExcBBC.k", "The value is 0."));
            }
        }
    }
    for mrid in dataset.by_type.get("ExcIEEEDC4B").into_iter().flatten() {
        let entry = &dataset.entries[mrid];
        if let Some(obj) = entry.element.as_any().downcast_ref::<cimstructs::ExcIEEEDC4B>() {
            if obj.kd.unwrap_or(0.0) > 0.0 && obj.td.unwrap_or(0.0) <= 0.0 {
                v.push(dyn_viol(mrid, RULE, "ExcIEEEDC4B", "ExcIEEEDC4B.td", "The value negative or zero when ExcIEEEDC4B.kd > 0."));
            }
        }
    }
    v
}

fn dyn_viol(mrid: &str, rule: &str, class: &str, property: &str, message: &str) -> Violation {
    Violation {
        object_id: mrid.to_string(), rule_id: rule.to_string(), name: rule.to_string(),
        class: class.to_string(), property: property.to_string(),
        message: message.to_string(), severity: "sh:Violation".into(), description: String::new(),
    }
}

// -- PSS input signal checks --

fn check_pss_input_signals(dataset: &CimDataset) -> Vec<Violation> {
    const RULE: &str = "signal uniqueness for PSS";
    let mut v = Vec::new();

    for mrid in dataset.by_type.get("Pss2ST").into_iter().flatten() {
        let entry = &dataset.entries[mrid];
        if let Some(obj) = entry.element.as_any().downcast_ref::<cimstructs::Pss2ST>() {
            if let (Some(s1), Some(s2)) = (obj.input_signal1type.as_ref(), obj.input_signal2type.as_ref()) {
                if s1.uri == s2.uri {
                    v.push(dyn_viol(mrid, RULE, "Pss2ST", "Pss2ST.inputSignal1Type", "Input signal #1 and input signal #2 are not different."));
                }
            }
        }
    }
    for mrid in dataset.by_type.get("PssWECC").into_iter().flatten() {
        let entry = &dataset.entries[mrid];
        if let Some(obj) = entry.element.as_any().downcast_ref::<cimstructs::PssWECC>() {
            if let (Some(s1), Some(s2)) = (obj.input_signal1type.as_ref(), obj.input_signal2type.as_ref()) {
                if s1.uri == s2.uri {
                    v.push(dyn_viol(mrid, RULE, "PssWECC", "PssWECC.inputSignal1Type", "Input signal #1 and input signal #2 are not different."));
                }
            }
        }
    }
    v
}

// -- GovHydro4 gain points --

fn check_gov_hydro4_gain_points(dataset: &CimDataset) -> Vec<Violation> {
    const RULE: &str = "various point sequence rules for GovHydro4";
    const SIMPLE:         &str = "GovHydro4ModelKind.simple";
    const FRANCIS_PELTON: &str = "GovHydro4ModelKind.francisPelton";
    const KAPLAN:         &str = "GovHydro4ModelKind.kaplan";

    let mut v = Vec::new();
    for mrid in dataset.by_type.get("GovHydro4").into_iter().flatten() {
        let entry = &dataset.entries[mrid];
        let obj = match entry.element.as_any().downcast_ref::<cimstructs::GovHydro4>() {
            Some(o) => o, None => continue,
        };
        let m = match obj.model.as_ref() { Some(r) => r.uri.as_str(), None => continue };

        let f = |val: Option<f64>| val.unwrap_or(0.0);
        if m == SIMPLE {
            for (val, prop) in [
                (f(obj.bmax), "bmax"), (f(obj.gv0), "gv0"), (f(obj.gv1), "gv1"),
                (f(obj.gv2), "gv2"), (f(obj.gv3), "gv3"), (f(obj.gv4), "gv4"), (f(obj.gv5), "gv5"),
                (f(obj.pgv0), "pgv0"), (f(obj.pgv1), "pgv1"), (f(obj.pgv2), "pgv2"),
                (f(obj.pgv3), "pgv3"), (f(obj.pgv4), "pgv4"), (f(obj.pgv5), "pgv5"),
            ] {
                if val != 0.0 {
                    v.push(dyn_viol(mrid, RULE, "GovHydro4", &format!("GovHydro4.{prop}"),
                        &format!("The value is not 0 when GovHydro4.model is simple.")));
                }
            }
        } else if m == FRANCIS_PELTON || m == KAPLAN {
            if m == FRANCIS_PELTON && f(obj.bmax) != 0.0 {
                v.push(dyn_viol(mrid, RULE, "GovHydro4", "GovHydro4.bmax",
                    "The value is not 0 when GovHydro4.model is francisPelton."));
            }
            for (val, prev, prop) in [
                (f(obj.gv1), f(obj.gv0), "gv1"),
                (f(obj.gv2), f(obj.gv1), "gv2"),
                (f(obj.gv3), f(obj.gv2), "gv3"),
                (f(obj.gv4), f(obj.gv3), "gv4"),
            ] {
                if val <= prev {
                    v.push(dyn_viol(mrid, RULE, "GovHydro4", &format!("GovHydro4.{prop}"),
                        &format!("The value is not greater than GovHydro4.{} when GovHydro4.model is francisPelton or kaplan.", &prop[..prop.len()-1])));
                }
            }
            let gv5 = f(obj.gv5);
            if gv5 <= f(obj.gv4) || gv5 >= 1.0 {
                v.push(dyn_viol(mrid, RULE, "GovHydro4", "GovHydro4.gv5",
                    "The value is either not greater than GovHydro4.gv4 or it is not less than 1 when GovHydro4.model is francisPelton or kaplan."));
            }
        }
    }
    v
}

// -- LoadStatic model attribute checks --

fn check_load_static_model_attributes(dataset: &CimDataset) -> Vec<Violation> {
    const RULE: &str = "required/prohibited rules for LoadStatic models";
    const CONSTANT_Z:  &str = "StaticLoadModelKind.constantZ";
    const EXPONENTIAL: &str = "StaticLoadModelKind.exponential";
    const ZIP1:        &str = "StaticLoadModelKind.zIP1";
    const ZIP2:        &str = "StaticLoadModelKind.zIP2";

    let mut v = Vec::new();
    for mrid in dataset.by_type.get("LoadStatic").into_iter().flatten() {
        let entry = &dataset.entries[mrid];
        let obj = match entry.element.as_any().downcast_ref::<cimstructs::LoadStatic>() {
            Some(o) => o, None => continue,
        };
        let m = match obj.static_load_model_type.as_ref() { Some(r) => r.uri.as_str(), None => continue };
        let f = |v: Option<f64>| v.unwrap_or(0.0);

        if m == CONSTANT_Z {
            if f(obj.kp1)!=0.0 || f(obj.kp2)!=0.0 || f(obj.kp3)!=0.0 || f(obj.kp4)!=0.0 || f(obj.kpf)!=0.0 ||
               f(obj.kq1)!=0.0 || f(obj.kq2)!=0.0 || f(obj.kq3)!=0.0 || f(obj.kq4)!=0.0 || f(obj.kqf)!=0.0 ||
               f(obj.ep1)!=0.0 || f(obj.ep2)!=0.0 || f(obj.ep3)!=0.0 ||
               f(obj.eq1)!=0.0 || f(obj.eq2)!=0.0 || f(obj.eq3)!=0.0
            {
                v.push(dyn_viol(mrid, RULE, "LoadStatic", "LoadStatic.staticLoadModelType",
                    "The load is represented as a constant impedance but other properties (attributes) are defined."));
            }
        } else if m == EXPONENTIAL {
            if f(obj.kp4)!=0.0 || f(obj.kq4)!=0.0 {
                v.push(dyn_viol(mrid, RULE, "LoadStatic", "LoadStatic.staticLoadModelType",
                    "Unnecessary properties defined for exponential model type (kp4/kq4)."));
            }
        } else if m == ZIP1 {
            if f(obj.ep1)!=0.0 || f(obj.ep2)!=0.0 || f(obj.ep3)!=0.0 ||
               f(obj.eq1)!=0.0 || f(obj.eq2)!=0.0 || f(obj.eq3)!=0.0 ||
               f(obj.kp4)!=0.0 || f(obj.kq4)!=0.0
            {
                v.push(dyn_viol(mrid, RULE, "LoadStatic", "LoadStatic.staticLoadModelType",
                    "Unnecessary properties defined for zIP1 model type."));
            }
        } else if m == ZIP2 {
            if f(obj.ep1)!=0.0 || f(obj.ep2)!=0.0 || f(obj.ep3)!=0.0 ||
               f(obj.eq1)!=0.0 || f(obj.eq2)!=0.0 || f(obj.eq3)!=0.0
            {
                v.push(dyn_viol(mrid, RULE, "LoadStatic", "LoadStatic.staticLoadModelType",
                    "Unnecessary properties defined for zIP2 model type."));
            }
        }
    }
    v
}

// -- Rotating machine saturation check --

macro_rules! check_sat {
    ($v:expr, $dataset:expr, $T:ident, $sf_path:expr, $sf120_path:expr) => {
        for mrid in $dataset.by_type.get(stringify!($T)).into_iter().flatten() {
            let entry = &$dataset.entries[mrid];
            if let Some(obj) = entry.element.as_any().downcast_ref::<cimstructs::$T>() {
                let sf   = $sf_path(obj);
                let sf120 = $sf120_path(obj);
                if let (Some(s1), Some(s2)) = (sf, sf120) {
                    if s2 < s1 {
                        $v.push(Violation {
                            object_id: mrid.clone(),
                            rule_id:   "saturation constraints for rotating machines".into(),
                            name:      "saturation constraints for rotating machines".into(),
                            class:     stringify!($T).to_string(),
                            property:  "RotatingMachineDynamics.saturationFactor120".into(),
                            message:   "The value is less than RotatingMachineDynamics.saturationFactor.".into(),
                            severity:  "sh:Violation".into(),
                            description: String::new(),
                        });
                    }
                }
            }
        }
    };
}

fn check_rotating_machine_saturation(dataset: &CimDataset) -> Vec<Violation> {
    let mut v = Vec::new();
    // SM chain: SMDetailed.base.base = RotatingMachineDynamics (after: SMDetailed → SMDynamics → RMD)
    // SMTCR: base=SMDetailed, base.base=SMDynamics, base.base.base=RMD
    check_sat!(v, dataset, SynchronousMachineTimeConstantReactance,
        |o: &cimstructs::SynchronousMachineTimeConstantReactance| o.base.base.base.saturation_factor,
        |o: &cimstructs::SynchronousMachineTimeConstantReactance| o.base.base.base.saturation_factor120);
    // SMEC: base=SMDetailed, same chain
    check_sat!(v, dataset, SynchronousMachineEquivalentCircuit,
        |o: &cimstructs::SynchronousMachineEquivalentCircuit| o.base.base.base.saturation_factor,
        |o: &cimstructs::SynchronousMachineEquivalentCircuit| o.base.base.base.saturation_factor120);
    // SMS: base=SMDynamics, base.base=RMD
    check_sat!(v, dataset, SynchronousMachineSimplified,
        |o: &cimstructs::SynchronousMachineSimplified| o.base.base.saturation_factor,
        |o: &cimstructs::SynchronousMachineSimplified| o.base.base.saturation_factor120);
    // SMUD: base=SMDynamics, base.base=RMD
    check_sat!(v, dataset, SynchronousMachineUserDefined,
        |o: &cimstructs::SynchronousMachineUserDefined| o.base.base.saturation_factor,
        |o: &cimstructs::SynchronousMachineUserDefined| o.base.base.saturation_factor120);
    // AMEC: base=AMDynamics, base.base=RMD
    check_sat!(v, dataset, AsynchronousMachineEquivalentCircuit,
        |o: &cimstructs::AsynchronousMachineEquivalentCircuit| o.base.base.saturation_factor,
        |o: &cimstructs::AsynchronousMachineEquivalentCircuit| o.base.base.saturation_factor120);
    // AMTCR: base=AMDynamics, base.base=RMD
    check_sat!(v, dataset, AsynchronousMachineTimeConstantReactance,
        |o: &cimstructs::AsynchronousMachineTimeConstantReactance| o.base.base.saturation_factor,
        |o: &cimstructs::AsynchronousMachineTimeConstantReactance| o.base.base.saturation_factor120);
    // AMUD: base=AMDynamics, base.base=RMD
    check_sat!(v, dataset, AsynchronousMachineUserDefined,
        |o: &cimstructs::AsynchronousMachineUserDefined| o.base.base.saturation_factor,
        |o: &cimstructs::AsynchronousMachineUserDefined| o.base.base.saturation_factor120);
    v
}

// -- SynchronousMachineSimplified saturation prohibition --

fn check_synchronous_machine_simplified_attributes(dataset: &CimDataset) -> Vec<Violation> {
    let mut v = Vec::new();
    for mrid in dataset.by_type.get("SynchronousMachineSimplified").into_iter().flatten() {
        let entry = &dataset.entries[mrid];
        let obj = match entry.element.as_any().downcast_ref::<cimstructs::SynchronousMachineSimplified>() {
            Some(o) => o, None => continue,
        };
        if obj.base.base.saturation_factor.unwrap_or(0.0) != 0.0 ||
           obj.base.base.saturation_factor120.unwrap_or(0.0) != 0.0
        {
            v.push(Violation {
                object_id:   mrid.clone(),
                rule_id:     "prohibits saturation for simplified machines".into(),
                name:        "prohibits saturation for simplified machines".into(),
                class:       "SynchronousMachineSimplified".into(),
                property:    "rdf:type".into(),
                message:     "Saturation related attributes are not needed for SynchronousMachineSimplified.".into(),
                severity:    "sh:Violation".into(),
                description: String::new(),
            });
        }
    }
    v
}

// -- Dynamics associations check --

fn check_dynamics_associations(dataset: &CimDataset) -> Vec<Violation> {
    let mut v = Vec::new();
    macro_rules! gov_check {
        ($($T:ident),+) => {$(
            for mrid in dataset.by_type.get(stringify!($T)).into_iter().flatten() {
                let entry = &dataset.entries[mrid];
                if let Some(obj) = entry.element.as_any().downcast_ref::<cimstructs::$T>() {
                    if obj.base.synchronous_machine_dynamics.is_none() && obj.base.asynchronous_machine_dynamics.is_none() {
                        v.push(Violation {
                            object_id:   mrid.clone(),
                            rule_id:     "ensures governors and loads point to a machine dynamics model".into(),
                            name:        "ensures governors and loads point to a machine dynamics model".into(),
                            class:       stringify!($T).to_string(),
                            property:    "rdf:type".into(),
                            message:     "Required association to either SynchronousMachineDynamics or to AsynchronousMachineDynamics is missing.".into(),
                            severity:    "sh:Violation".into(),
                            description: String::new(),
                        });
                    }
                }
            }
        )+};
    }
    gov_check!(GovCT1, GovCT2, GovGAST, GovGAST1, GovGAST2, GovGAST3, GovGAST4, GovGASTWD,
               GovHydro1, GovHydro2, GovHydro3, GovHydro4, GovHydroDD, GovHydroFrancis,
               GovHydroIEEE0, GovHydroIEEE2, GovHydroPID, GovHydroPID2, GovHydroPelton,
               GovHydroR, GovHydroWEH, GovHydroWPID,
               GovSteam0, GovSteam1, GovSteam2, GovSteamBB, GovSteamEU,
               GovSteamFV2, GovSteamFV3, GovSteamFV4, GovSteamIEEE1, GovSteamSGO);

    // Mech types: base: MechanicalLoadDynamics
    macro_rules! mech_check {
        ($($T:ident),+) => {$(
            for mrid in dataset.by_type.get(stringify!($T)).into_iter().flatten() {
                let entry = &dataset.entries[mrid];
                if let Some(obj) = entry.element.as_any().downcast_ref::<cimstructs::$T>() {
                    if obj.base.synchronous_machine_dynamics.is_none() && obj.base.asynchronous_machine_dynamics.is_none() {
                        v.push(Violation {
                            object_id:   mrid.clone(),
                            rule_id:     "ensures governors and loads point to a machine dynamics model".into(),
                            name:        "ensures governors and loads point to a machine dynamics model".into(),
                            class:       stringify!($T).to_string(),
                            property:    "rdf:type".into(),
                            message:     "Required association to either SynchronousMachineDynamics or to AsynchronousMachineDynamics is missing.".into(),
                            severity:    "sh:Violation".into(),
                            description: String::new(),
                        });
                    }
                }
            }
        )+};
    }
    mech_check!(MechLoad1, MechanicalLoadUserDefined);
    v
}
