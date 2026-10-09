use cimmodel::CimDataset;
use crate::Violation;
use super::Fields;

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
    v.extend(check_gov_steam_fv3_t5(dataset));
    // TurbineGovernorDynamics and MechanicalLoadDynamics "associationsCondition"
    // are `sh:xone` node shapes, which the CGMES shape table runs.
    v
}

// -- ExcitationSystemDynamics.SynchronousMachineDynamics check --

fn check_excitation_system_smd(dataset: &CimDataset) -> Vec<Violation> {
    const EXCITATION_SYSTEMS: &[&str] = &[
        "ExcAC1A", "ExcAC2A", "ExcAC3A", "ExcAC4A", "ExcAC5A", "ExcAC6A", "ExcAC8B",
        "ExcANS", "ExcAVR1", "ExcAVR2", "ExcAVR3", "ExcAVR4", "ExcAVR5", "ExcAVR7",
        "ExcBBC", "ExcCZ", "ExcDC1A", "ExcDC2A", "ExcDC3A", "ExcDC3A1",
        "ExcELIN1", "ExcELIN2", "ExcHU",
        "ExcIEEEAC1A", "ExcIEEEAC2A", "ExcIEEEAC3A", "ExcIEEEAC4A", "ExcIEEEAC5A", "ExcIEEEAC6A",
        "ExcIEEEAC7B", "ExcIEEEAC8B", "ExcIEEEDC1A", "ExcIEEEDC2A", "ExcIEEEDC3A", "ExcIEEEDC4B",
        "ExcIEEEST1A", "ExcIEEEST2A", "ExcIEEEST3A", "ExcIEEEST4B", "ExcIEEEST5B", "ExcIEEEST6B", "ExcIEEEST7B",
        "ExcNI", "ExcOEX3T", "ExcPIC", "ExcREXS", "ExcRQB", "ExcSCRX", "ExcSEXS", "ExcSK",
        "ExcST1A", "ExcST2A", "ExcST3A", "ExcST4B", "ExcST6B", "ExcST7B",
        "ExcitationSystemUserDefined",
    ];
    let mut v = Vec::new();
    for class in EXCITATION_SYSTEMS {
        for mrid in dataset.by_type.get(*class).into_iter().flatten() {
            let Some(obj) = Fields::of_class(&dataset.entries[mrid], class) else { continue };
            if let Some(smd_ref) = obj.reference("ExcitationSystemDynamics.SynchronousMachineDynamics") {
                let target_id = smd_ref.trim_start_matches('#');
                let is_simplified = dataset.entries.get(target_id)
                    .is_some_and(|e| e.type_name() == "SynchronousMachineSimplified");
                if is_simplified {
                    v.push(Violation {
                        object_id:   mrid.clone(),
                        rule_id:     "dy457:ExcitationSystemDynamics.SynchronousMachineDynamicsSynchronousMachineSimplified-valueType".into(),
                        name:        "C:457:DY:ExcitationSystemDynamics.SynchronousMachineDynamics:reference".into(),
                        class:       class.to_string(),
                        property:    "ExcitationSystemDynamics.SynchronousMachineDynamics".into(),
                        message:     "The association ExcitationSystemDynamics.SynchronousMachineDynamics points to an object of type SynchronousMachineSimplified.".into(),
                        severity:    "sh:Violation".into(),
                        description: String::new(),
                    });
                }
            }
        }
    }
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
        let obj = match Fields::of_class(entry, "SynchronousMachineTimeConstantReactance") {
            Some(o) => o, None => continue,
        };
        let mt = match obj.enumeration("SynchronousMachineTimeConstantReactance.modelType") { Some(r) => r, None => continue };
        let rt = match obj.enumeration("SynchronousMachineTimeConstantReactance.rotorType") { Some(r) => r, None => continue };

        // As the SPARQL: an optional factor that is absent compares as an
        // error, so only a given, non-zero one counts.
        let nonzero = |key: &str| obj.f64(key).is_some_and(|x| x != 0.0);
        if mt == SUBTRANS_SIMPLIFIED && rt == ROUND_ROTOR {
            // The stator resistance is a required pattern: without it, nothing.
            if obj.f64("RotatingMachineDynamics.statorResistance").is_some_and(|r| {
                r != 0.0
                    || nonzero("SynchronousMachineDetailed.saturationFactorQAxis")
                    || nonzero("SynchronousMachineDetailed.saturationFactor120QAxis")
            }) {
                v.push(Violation {
                    object_id:   mrid.clone(),
                    rule_id:     "dy457:SynchronousMachineTimeConstantReactance-modelType-SubtransientRoundRotorSimplified".into(),
                    name:        "C:457:DY:RotatingMachineDynamics:modelType-SubtransientRoundRotorSimplified".into(),
                    class:       "SynchronousMachineTimeConstantReactance".into(),
                    property:    "SynchronousMachineTimeConstantReactance.modelType".into(),
                    message:     "Missing attributes or default values not provided according to 61970-457 Annex A (subtransientSimplified/roundRotor).".into(),
                    severity:    "sh:Violation".into(),
                    description: String::new(),
                });
            }
        } else if mt == SUBTRANS && rt == ROUND_ROTOR {
            // Any of them absent (`!bound`); zero is a value.
            if [
                "SynchronousMachineDetailed.saturationFactorQAxis",
                "SynchronousMachineDetailed.saturationFactor120QAxis",
                "RotatingMachineDynamics.saturationFactor",
                "RotatingMachineDynamics.saturationFactor120",
                "SynchronousMachineTimeConstantReactance.xQuadTrans",
                "SynchronousMachineTimeConstantReactance.tpqo",
            ]
            .iter()
            .any(|k| !obj.has(k))
            {
                v.push(Violation {
                    object_id:   mrid.clone(),
                    rule_id:     "dy457:SynchronousMachineTimeConstantReactance-modelType-SubtransientRoundRotor".into(),
                    name:        "C:457:DY:RotatingMachineDynamics:modelType-SubtransientRoundRotor".into(),
                    class:       "SynchronousMachineTimeConstantReactance".into(),
                    property:    "SynchronousMachineTimeConstantReactance.modelType".into(),
                    message:     "Missing attributes or default values not provided according to 61970-457 Annex A (subtransient/roundRotor).".into(),
                    severity:    "sh:Violation".into(),
                    description: String::new(),
                });
            }
        } else if mt == SUBTRANS && rt == SALIENT_POLE
            && (nonzero("SynchronousMachineDetailed.saturationFactorQAxis")
                || nonzero("SynchronousMachineDetailed.saturationFactor120QAxis"))
            {
                v.push(Violation {
                    object_id:   mrid.clone(),
                    rule_id:     "dy457:SynchronousMachineTimeConstantReactance-modelType-SubtransientSalientPole".into(),
                    name:        "C:457:DY:RotatingMachineDynamics:modelType-SubtransientSalientPole".into(),
                    class:       "SynchronousMachineTimeConstantReactance".into(),
                    property:    "SynchronousMachineTimeConstantReactance.modelType".into(),
                    message:     "Missing attributes or default values not provided according to 61970-457 Annex A (subtransient/salientPole).".into(),
                    severity:    "sh:Violation".into(),
                    description: String::new(),
                });
            }
    }
    v
}

// -- TurbineGovernorDynamics mwbase check --

fn check_turbine_governor_mbase(dataset: &CimDataset) -> Vec<Violation> {
    // Each class declares its own mwbase; the shape's targets.
    const GOVERNORS: &[&str] = &[
        "GovCT1", "GovCT2", "GovGAST", "GovGAST1", "GovGAST2", "GovGASTWD",
        "GovHydro1", "GovHydro2", "GovHydro3", "GovHydro4", "GovHydroDD",
        "GovHydroIEEE0", "GovHydroIEEE2", "GovHydroPID", "GovHydroPID2",
        "GovHydroR", "GovHydroWEH", "GovHydroWPID",
        "GovSteam0", "GovSteam1", "GovSteamCC", "GovSteamEU",
        "GovSteamFV2", "GovSteamFV3", "GovSteamIEEE1", "GovSteamSGO",
    ];
    let mut v = Vec::new();
    for class in GOVERNORS {
        let mwbase_key = format!("{class}.mwbase");
        for mrid in dataset.by_type.get(*class).into_iter().flatten() {
            let Some(obj) = Fields::of_class(&dataset.entries[mrid], class) else { continue };
            let smd_id = match obj.reference("TurbineGovernorDynamics.SynchronousMachineDynamics") {
                Some(r) => r.trim_start_matches('#').to_string(), None => continue,
            };
            let smd_entry = match dataset.entries.get(&smd_id) { Some(e) => e, None => continue };
            let sm_id = match Fields::of(smd_entry).reference("SynchronousMachineDynamics.SynchronousMachine") {
                Some(s) => s.trim_start_matches('#').to_string(),
                None => continue,
            };
            // A machine the dataset does not hold is unknown, not missing its
            // ratings — DY is validated without EQ.
            let Some(sm) = dataset.entries.get(&sm_id).map(Fields::of) else { continue };
            let message = match (sm.f64("RotatingMachine.ratedPowerFactor"), sm.f64("RotatingMachine.ratedS")) {
                // The SPARQL reports a missing rating whatever mwbase is.
                (None, _) | (_, None) => "Either both or one of RotatingMachine.ratedPowerFactor and RotatingMachine.ratedS are not defined.".to_string(),
                (Some(pf), Some(s)) => match obj.f64(&mwbase_key) {
                    Some(mwbase) if (mwbase - pf * s).abs() > 0.001 => format!(
                        "The value {mwbase} does not equal RotatingMachine.ratedPowerFactor * RotatingMachine.ratedS ({}).",
                        pf * s
                    ),
                    _ => continue,
                },
            };
            v.push(Violation {
                object_id:   mrid.clone(),
                rule_id:     "dyn457:TurbineGovernorDynamics-mbaseEquation".into(),
                name:        "C:457:DY:mwbase:equation".into(),
                class:       class.to_string(),
                property:    "mwbase".into(),
                message,
                severity:    "sh:Violation".into(),
                description: String::new(),
            });
        }
    }
    v
}

// -- ExcitationSystem gain checks --

/// Each SPARQL binds both values as required patterns: an absent gain or time
/// constant is no violation.
fn check_excitation_system_gains(dataset: &CimDataset) -> Vec<Violation> {
    let mut v = Vec::new();

    for mrid in dataset.by_type.get("ExcAC8B").into_iter().flatten() {
        let entry = &dataset.entries[mrid];
        if let Some(obj) = Fields::of_class(entry, "ExcAC8B")
            && obj.f64("ExcAC8B.kir") == Some(0.0) && obj.f64("ExcAC8B.kpr").is_some_and(|x| x <= 0.0) {
                v.push(dyn_viol(mrid, "dyu:ExcAC8B.kpr-valueRange", "C:302:DY:ExcAC8B.kpr:valueRange",
                    "ExcAC8B", "ExcAC8B.kpr", "The value negative or zero when ExcAC8B.kir = 0."));
            }
    }
    for mrid in dataset.by_type.get("ExcIEEEAC8B").into_iter().flatten() {
        let entry = &dataset.entries[mrid];
        if let Some(obj) = Fields::of_class(entry, "ExcIEEEAC8B")
            && obj.f64("ExcIEEEAC8B.kir") == Some(0.0) && obj.f64("ExcIEEEAC8B.kpr").is_some_and(|x| x <= 0.0) {
                v.push(dyn_viol(mrid, "dyu:ExcIEEEAC8B.kpr-valueRange", "C:302:DY:ExcIEEEAC8B.kpr:valueRange",
                    "ExcIEEEAC8B", "ExcIEEEAC8B.kpr", "The value negative or zero when ExcIEEEAC8B.kir = 0."));
            }
    }
    for mrid in dataset.by_type.get("ExcIEEEAC7B").into_iter().flatten() {
        let entry = &dataset.entries[mrid];
        if let Some(obj) = Fields::of_class(entry, "ExcIEEEAC7B") {
            if obj.f64("ExcIEEEAC7B.kia") == Some(0.0) && obj.f64("ExcIEEEAC7B.kpa").is_some_and(|x| x <= 0.0) {
                v.push(dyn_viol(mrid, "dyu:ExcIEEEAC7B.kpa-valueRange", "C:302:DY:ExcIEEEAC7B.kpa:valueRange",
                    "ExcIEEEAC7B", "ExcIEEEAC7B.kpa", "The value negative or zero when ExcIEEEAC7B.kia = 0."));
            }
            if obj.f64("ExcIEEEAC7B.kir") == Some(0.0) && obj.f64("ExcIEEEAC7B.kpr").is_some_and(|x| x <= 0.0) {
                v.push(dyn_viol(mrid, "dyu:ExcIEEEAC7B.kpr-valueRange", "C:302:DY:ExcIEEEAC7B.kpr:valueRange",
                    "ExcIEEEAC7B", "ExcIEEEAC7B.kpr", "The value negative or zero when ExcIEEEAC7B.kir = 0."));
            }
        }
    }
    for mrid in dataset.by_type.get("ExcBBC").into_iter().flatten() {
        let entry = &dataset.entries[mrid];
        if let Some(obj) = Fields::of_class(entry, "ExcBBC")
            && obj.f64("ExcBBC.k") == Some(0.0) {
                v.push(dyn_viol(mrid, "dyu:ExcBBC.k-valueRange", "C:302:DY:ExcBBC.k:valueRange",
                    "ExcBBC", "ExcBBC.k", "The value is 0."));
            }
    }
    for mrid in dataset.by_type.get("ExcIEEEDC4B").into_iter().flatten() {
        let entry = &dataset.entries[mrid];
        if let Some(obj) = Fields::of_class(entry, "ExcIEEEDC4B")
            && obj.f64("ExcIEEEDC4B.kd").is_some_and(|x| x > 0.0) && obj.f64("ExcIEEEDC4B.td").is_some_and(|x| x <= 0.0) {
                v.push(dyn_viol(mrid, "dyu:ExcIEEEDC4B.td-valueRange", "C:302:DY:ExcIEEEDC4B.td:valueRange",
                    "ExcIEEEDC4B", "ExcIEEEDC4B.td", "The value negative or zero when ExcIEEEDC4B.kd > 0."));
            }
    }
    for mrid in dataset.by_type.get("ExcSEXS").into_iter().flatten() {
        let entry = &dataset.entries[mrid];
        if let Some(obj) = Fields::of_class(entry, "ExcSEXS")
            && obj.f64("ExcSEXS.tc").is_some_and(|x| x > 0.0) && obj.f64("ExcSEXS.kc").is_some_and(|x| x <= 0.0) {
                v.push(dyn_viol(mrid, "dyu:ExcSEXS.kc-valueRange", "C:302:DY:ExcSEXS.kc:valueRange",
                    "ExcSEXS", "ExcSEXS.kc", "The value negative or zero when ExcSEXS.tc > 0."));
            }
    }
    v
}

fn check_gov_steam_fv3_t5(dataset: &CimDataset) -> Vec<Violation> {
    let mut v = Vec::new();
    for mrid in dataset.by_type.get("GovSteamFV3").into_iter().flatten() {
        let entry = &dataset.entries[mrid];
        if let Some(obj) = Fields::of_class(entry, "GovSteamFV3")
            && obj.f64("GovSteamFV3.t5").is_some_and(|t5| t5 < 0.0) {
                v.push(dyn_viol(mrid, "dyu:GovSteamFV3.t5-valueRange", "C:302:DY:GovSteamFV3.t5:valueRange",
                    "GovSteamFV3", "GovSteamFV3.t5", "The value is negative."));
            }
    }
    v
}

fn dyn_viol(mrid: &str, rule_id: &str, name: &str, class: &str, property: &str, message: &str) -> Violation {
    Violation {
        object_id: mrid.to_string(), rule_id: rule_id.to_string(), name: name.to_string(),
        class: class.to_string(), property: property.to_string(),
        message: message.to_string(), severity: "sh:Violation".into(), description: String::new(),
    }
}

// -- PSS input signal checks --

fn check_pss_input_signals(dataset: &CimDataset) -> Vec<Violation> {
    let mut v = Vec::new();

    for mrid in dataset.by_type.get("Pss2ST").into_iter().flatten() {
        let entry = &dataset.entries[mrid];
        if let Some(obj) = Fields::of_class(entry, "Pss2ST")
            && let (Some(s1), Some(s2)) = (obj.enumeration("Pss2ST.inputSignal1Type"), obj.enumeration("Pss2ST.inputSignal2Type"))
                && s1 == s2 {
                    v.push(dyn_viol(mrid, "dyu:Pss2ST-inputSignals", "C:302:DY:Pss2ST:inputSignals",
                        "Pss2ST", "Pss2ST.inputSignal1Type", "Input signal #1 and input signal #2 are not different."));
                }
    }
    for mrid in dataset.by_type.get("PssWECC").into_iter().flatten() {
        let entry = &dataset.entries[mrid];
        if let Some(obj) = Fields::of_class(entry, "PssWECC")
            && let (Some(s1), Some(s2)) = (obj.enumeration("PssWECC.inputSignal1Type"), obj.enumeration("PssWECC.inputSignal2Type"))
                && s1 == s2 {
                    v.push(dyn_viol(mrid, "dyu:PssWECC-inputSignals", "C:302:DY:PssWECC:inputSignals",
                        "PssWECC", "PssWECC.inputSignal1Type", "Input signal #1 and input signal #2 are not different."));
                }
    }
    v
}

// -- GovHydro4 gain points --

fn check_gov_hydro4_gain_points(dataset: &CimDataset) -> Vec<Violation> {
    const SIMPLE:         &str = "GovHydro4ModelKind.simple";
    const FRANCIS_PELTON: &str = "GovHydro4ModelKind.francisPelton";
    const KAPLAN:         &str = "GovHydro4ModelKind.kaplan";

    let mut v = Vec::new();
    for mrid in dataset.by_type.get("GovHydro4").into_iter().flatten() {
        let entry = &dataset.entries[mrid];
        let obj = match Fields::of_class(entry, "GovHydro4") {
            Some(o) => o, None => continue,
        };
        let m = match obj.enumeration("GovHydro4.model") { Some(r) => r, None => continue };
        let f = |key: &str| obj.f64(&format!("GovHydro4.{key}"));
        let mut report = |prop: &str, message: &str| {
            v.push(dyn_viol(mrid, &format!("dyu:GovHydro4.{prop}-valueRange"), &format!("C:302:DY:GovHydro4.{prop}:valueRange"),
                "GovHydro4", &format!("GovHydro4.{prop}"), message));
        };
        // Each rule needs its own value given; an absent one is no violation.
        let nonzero = |key: &str| f(key).is_some_and(|x| x != 0.0);

        // Zero under the simple model (and, for bmax and the bgv points, the
        // Francis–Pelton one too).
        let zero_for_fp = ["bmax", "bgv0", "bgv1", "bgv2", "bgv3", "bgv4", "bgv5"];
        let zero_for_simple = ["gv0", "pgv0", "pgv1", "pgv2", "pgv3", "pgv4", "pgv5"];
        for key in zero_for_fp {
            if (m == SIMPLE || m == FRANCIS_PELTON) && nonzero(key) {
                report(key, &format!("The value is not 0 when GovHydro4.model is {}.", if m == SIMPLE { "simple" } else { "francisPelton" }));
            }
        }
        for key in zero_for_simple {
            if m == SIMPLE && nonzero(key) {
                report(key, "The value is not 0 when GovHydro4.model is simple.");
            }
        }
        // gv1–gv5 bind the previous point as a required pattern, under every
        // model: without it, nothing.
        for (key, prev) in [("gv1", "gv0"), ("gv2", "gv1"), ("gv3", "gv2"), ("gv4", "gv3"), ("gv5", "gv4")] {
            let (Some(value), Some(prev_value)) = (f(key), f(prev)) else { continue };
            if m == SIMPLE {
                if value != 0.0 {
                    report(key, "The value is not 0 when GovHydro4.model is simple.");
                }
            } else if m == FRANCIS_PELTON || m == KAPLAN {
                // gv5 also stays below 1. Its SPARQL joins the two bounds
                // with `&&`, which no value can fail; this is the rule its
                // description states.
                if key == "gv5" {
                    if value <= prev_value || value >= 1.0 {
                        report(key, "The value is either not greater than GovHydro4.gv4 or it is not less than 1 when GovHydro4.model is francisPelton or kaplan.");
                    }
                } else if value <= prev_value {
                    report(key, &format!("The value is not greater than GovHydro4.{prev} when GovHydro4.model is francisPelton or kaplan."));
                }
            }
        }
    }
    v
}

// -- LoadStatic model attribute checks --

fn check_load_static_model_attributes(dataset: &CimDataset) -> Vec<Violation> {
    const CONSTANT_Z:  &str = "StaticLoadModelKind.constantZ";
    const EXPONENTIAL: &str = "StaticLoadModelKind.exponential";
    const ZIP1:        &str = "StaticLoadModelKind.zIP1";
    const ZIP2:        &str = "StaticLoadModelKind.zIP2";

    let mut v = Vec::new();
    for mrid in dataset.by_type.get("LoadStatic").into_iter().flatten() {
        let entry = &dataset.entries[mrid];
        let obj = match Fields::of_class(entry, "LoadStatic") {
            Some(o) => o, None => continue,
        };
        let m = match obj.enumeration("LoadStatic.staticLoadModelType") { Some(r) => r, None => continue };
        // Presence, as the SPARQL's `bound(..)`: a coefficient given as 0 is given.
        let has = |keys: &[&str]| keys.iter().filter(|k| obj.has(&format!("LoadStatic.{k}"))).count();
        let any = |keys: &[&str]| has(keys) > 0;
        let all = |keys: &[&str]| has(keys) == keys.len();
        const KP: [&str; 4] = ["kp1", "kp2", "kp3", "kpf"];
        const KQ: [&str; 4] = ["kq1", "kq2", "kq3", "kqf"];
        const E: [&str; 6] = ["ep1", "ep2", "ep3", "eq1", "eq2", "eq3"];
        const K4: [&str; 2] = ["kp4", "kq4"];

        let (rule, name, message, fails) = if m == CONSTANT_Z {
            ("constantZ", "constantZ",
             "The load is represented as a constant impedance but other properties (attributes) are defined.",
             any(&KP) || any(&KQ) || any(&E) || any(&K4))
        } else if m == EXPONENTIAL {
            ("exponental", "exponential",
             "Required properties (attributes) for exponential model type are not defined or there are unnecessary properties defined.",
             !(all(&KP) && all(&KQ) && all(&E)) || any(&K4))
        } else if m == ZIP1 {
            ("zIP1", "zIP1",
             "Required properties (attributes) for zIP1 model type are not defined or there are unnecessary properties defined.",
             !(all(&KP) && all(&KQ)) || any(&E) || any(&K4))
        } else if m == ZIP2 {
            ("zIP2", "zIP2",
             "Required properties (attributes) for zIP2 model type are not defined or there are unnecessary properties defined.",
             !(all(&KP) && all(&KQ) && all(&K4)) || any(&E))
        } else {
            continue;
        };
        if fails {
            v.push(dyn_viol(mrid,
                &format!("dyu:LoadStatic.staticLoadModelType-{rule}"),
                &format!("C:302:DY:StaticLoadModelKind.{name}:requiredAttributes"),
                "LoadStatic", "LoadStatic.staticLoadModelType", message));
        }
    }
    v
}

// -- Rotating machine saturation check --

fn check_rotating_machine_saturation(dataset: &CimDataset) -> Vec<Violation> {
    // Every RotatingMachineDynamics subclass inherits both factors.
    const MACHINES: &[&str] = &[
        "SynchronousMachineTimeConstantReactance", "SynchronousMachineEquivalentCircuit",
        "SynchronousMachineSimplified", "SynchronousMachineUserDefined",
        "AsynchronousMachineEquivalentCircuit", "AsynchronousMachineTimeConstantReactance",
        "AsynchronousMachineUserDefined",
    ];
    let mut v = Vec::new();
    for class in MACHINES {
        for mrid in dataset.by_type.get(*class).into_iter().flatten() {
            let Some(obj) = Fields::of_class(&dataset.entries[mrid], class) else { continue };
            let sf = obj.f64("RotatingMachineDynamics.saturationFactor");
            let sf120 = obj.f64("RotatingMachineDynamics.saturationFactor120");
            if let (Some(s1), Some(s2)) = (sf, sf120)
                && s2 < s1 {
                    v.push(Violation {
                        object_id: mrid.clone(),
                        rule_id:   "dyu:RotatingMachineDynamics.saturationFactor120-valueRange".into(),
                        name:      "C:302:DY:RotatingMachineDynamics.saturationFactor120:valueRange".into(),
                        class:     class.to_string(),
                        property:  "RotatingMachineDynamics.saturationFactor120".into(),
                        message:   "The value is less than RotatingMachineDynamics.saturationFactor.".into(),
                        severity:  "sh:Violation".into(),
                        description: String::new(),
                    });
                }
        }
    }
    v
}

// -- SynchronousMachineSimplified saturation prohibition --

fn check_synchronous_machine_simplified_attributes(dataset: &CimDataset) -> Vec<Violation> {
    let mut v = Vec::new();
    for mrid in dataset.by_type.get("SynchronousMachineSimplified").into_iter().flatten() {
        let entry = &dataset.entries[mrid];
        let obj = match Fields::of_class(entry, "SynchronousMachineSimplified") {
            Some(o) => o, None => continue,
        };
        // Either factor given at all (`bound`), zero included.
        if obj.has("RotatingMachineDynamics.saturationFactor") || obj.has("RotatingMachineDynamics.saturationFactor120") {
            v.push(Violation {
                object_id:   mrid.clone(),
                rule_id:     "dyu:SynchronousMachineSimplified-requiredAttributes".into(),
                name:        "C:302:DY:SynchronousMachineSimplified:requiredAttributes".into(),
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
