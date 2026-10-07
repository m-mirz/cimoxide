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
    v.extend(check_dynamics_associations(dataset));
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
                    .is_some_and(|e| e.element.type_name() == "SynchronousMachineSimplified");
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

        if mt == SUBTRANS_SIMPLIFIED && rt == ROUND_ROTOR {
            if obj.f64("RotatingMachineDynamics.statorResistance").unwrap_or(0.0) != 0.0 ||
               obj.f64("SynchronousMachineDetailed.saturationFactorQAxis").unwrap_or(0.0) != 0.0 ||
               obj.f64("SynchronousMachineDetailed.saturationFactor120QAxis").unwrap_or(0.0) != 0.0
            {
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
            if obj.f64("SynchronousMachineDetailed.saturationFactorQAxis").unwrap_or(0.0) == 0.0 ||
               obj.f64("SynchronousMachineDetailed.saturationFactor120QAxis").unwrap_or(0.0) == 0.0 ||
               obj.f64("RotatingMachineDynamics.saturationFactor").unwrap_or(0.0) == 0.0 ||
               obj.f64("RotatingMachineDynamics.saturationFactor120").unwrap_or(0.0) == 0.0 ||
               obj.f64("SynchronousMachineTimeConstantReactance.xQuadTrans").unwrap_or(0.0) == 0.0 ||
               obj.f64("SynchronousMachineTimeConstantReactance.tpqo").unwrap_or(0.0) == 0.0
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
            && (obj.f64("SynchronousMachineDetailed.saturationFactorQAxis").unwrap_or(0.0) != 0.0 ||
               obj.f64("SynchronousMachineDetailed.saturationFactor120QAxis").unwrap_or(0.0) != 0.0)
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
    // Each class declares its own mwbase.
    const GOVERNORS: &[&str] = &[
        "GovCT1", "GovCT2", "GovGAST", "GovGAST1", "GovGAST2", "GovGASTWD",
        "GovHydro1", "GovHydro2", "GovHydro3", "GovHydro4", "GovHydroDD",
        "GovHydroIEEE0", "GovHydroIEEE2", "GovHydroPID", "GovHydroPID2",
        "GovHydroR", "GovHydroWEH", "GovHydroWPID",
        "GovSteam0", "GovSteam1", "GovSteamEU",
        "GovSteamFV2", "GovSteamFV3", "GovSteamIEEE1", "GovSteamSGO",
    ];
    let mut v = Vec::new();
    for class in GOVERNORS {
        let mwbase_key = format!("{class}.mwbase");
        for mrid in dataset.by_type.get(*class).into_iter().flatten() {
            let Some(obj) = Fields::of_class(&dataset.entries[mrid], class) else { continue };
            let mwbase = match obj.f64(&mwbase_key) { Some(v) if v != 0.0 => v, _ => continue };
            let smd_id = match obj.reference("TurbineGovernorDynamics.SynchronousMachineDynamics") {
                Some(r) => r.trim_start_matches('#').to_string(), None => continue,
            };
            let smd_entry = match dataset.entries.get(&smd_id) { Some(e) => e, None => continue };
            let sm_id = match Fields::of(smd_entry).reference("SynchronousMachineDynamics.SynchronousMachine") {
                Some(s) => s.trim_start_matches('#').to_string(),
                None => continue,
            };
            let sm = match Fields::get(dataset, &sm_id, "SynchronousMachine") { Some(o) => o, None => continue };
            let rated_pf = sm.f64("RotatingMachine.ratedPowerFactor").unwrap_or(0.0);
            let rated_s  = sm.f64("RotatingMachine.ratedS").unwrap_or(0.0);
            let expected = rated_pf * rated_s;
            if (mwbase - expected).abs() > 0.001 {
                v.push(Violation {
                    object_id:   mrid.clone(),
                    rule_id:     "dyn457:TurbineGovernorDynamics-mbaseEquation".into(),
                    name:        "C:457:DY:mwbase:equation".into(),
                    class:       class.to_string(),
                    property:    "mwbase".into(),
                    message:     format!("The value {mwbase} does not equal RotatingMachine.ratedPowerFactor * RotatingMachine.ratedS ({expected})."),
                    severity:    "sh:Violation".into(),
                    description: String::new(),
                });
            }
        }
    }
    v
}

// -- ExcitationSystem gain checks --

fn check_excitation_system_gains(dataset: &CimDataset) -> Vec<Violation> {
    let mut v = Vec::new();

    for mrid in dataset.by_type.get("ExcAC8B").into_iter().flatten() {
        let entry = &dataset.entries[mrid];
        if let Some(obj) = Fields::of_class(entry, "ExcAC8B")
            && obj.f64("ExcAC8B.kir").unwrap_or(0.0) == 0.0 && obj.f64("ExcAC8B.kpr").unwrap_or(0.0) <= 0.0 {
                v.push(dyn_viol(mrid, "dyu:ExcAC8B.kpr-valueRange", "C:302:DY:ExcAC8B.kpr:valueRange",
                    "ExcAC8B", "ExcAC8B.kpr", "The value negative or zero when ExcAC8B.kir = 0."));
            }
    }
    for mrid in dataset.by_type.get("ExcIEEEAC8B").into_iter().flatten() {
        let entry = &dataset.entries[mrid];
        if let Some(obj) = Fields::of_class(entry, "ExcIEEEAC8B")
            && obj.f64("ExcIEEEAC8B.kir").unwrap_or(0.0) == 0.0 && obj.f64("ExcIEEEAC8B.kpr").unwrap_or(0.0) <= 0.0 {
                v.push(dyn_viol(mrid, "dyu:ExcIEEEAC8B.kpr-valueRange", "C:302:DY:ExcIEEEAC8B.kpr:valueRange",
                    "ExcIEEEAC8B", "ExcIEEEAC8B.kpr", "The value negative or zero when ExcIEEEAC8B.kir = 0."));
            }
    }
    for mrid in dataset.by_type.get("ExcIEEEAC7B").into_iter().flatten() {
        let entry = &dataset.entries[mrid];
        if let Some(obj) = Fields::of_class(entry, "ExcIEEEAC7B") {
            if obj.f64("ExcIEEEAC7B.kia").unwrap_or(0.0) == 0.0 && obj.f64("ExcIEEEAC7B.kpa").unwrap_or(0.0) <= 0.0 {
                v.push(dyn_viol(mrid, "dyu:ExcIEEEAC7B.kpa-valueRange", "C:302:DY:ExcIEEEAC7B.kpa:valueRange",
                    "ExcIEEEAC7B", "ExcIEEEAC7B.kpa", "The value negative or zero when ExcIEEEAC7B.kia = 0."));
            }
            if obj.f64("ExcIEEEAC7B.kir").unwrap_or(0.0) == 0.0 && obj.f64("ExcIEEEAC7B.kpr").unwrap_or(0.0) <= 0.0 {
                v.push(dyn_viol(mrid, "dyu:ExcIEEEAC7B.kpr-valueRange", "C:302:DY:ExcIEEEAC7B.kpr:valueRange",
                    "ExcIEEEAC7B", "ExcIEEEAC7B.kpr", "The value negative or zero when ExcIEEEAC7B.kir = 0."));
            }
        }
    }
    for mrid in dataset.by_type.get("ExcBBC").into_iter().flatten() {
        let entry = &dataset.entries[mrid];
        if let Some(obj) = Fields::of_class(entry, "ExcBBC")
            && obj.f64("ExcBBC.k").unwrap_or(0.0) == 0.0 {
                v.push(dyn_viol(mrid, "dyu:ExcBBC.k-valueRange", "C:302:DY:ExcBBC.k:valueRange",
                    "ExcBBC", "ExcBBC.k", "The value is 0."));
            }
    }
    for mrid in dataset.by_type.get("ExcIEEEDC4B").into_iter().flatten() {
        let entry = &dataset.entries[mrid];
        if let Some(obj) = Fields::of_class(entry, "ExcIEEEDC4B")
            && obj.f64("ExcIEEEDC4B.kd").unwrap_or(0.0) > 0.0 && obj.f64("ExcIEEEDC4B.td").unwrap_or(0.0) <= 0.0 {
                v.push(dyn_viol(mrid, "dyu:ExcIEEEDC4B.td-valueRange", "C:302:DY:ExcIEEEDC4B.td:valueRange",
                    "ExcIEEEDC4B", "ExcIEEEDC4B.td", "The value negative or zero when ExcIEEEDC4B.kd > 0."));
            }
    }
    for mrid in dataset.by_type.get("ExcSEXS").into_iter().flatten() {
        let entry = &dataset.entries[mrid];
        if let Some(obj) = Fields::of_class(entry, "ExcSEXS")
            && obj.f64("ExcSEXS.tc").unwrap_or(0.0) > 0.0 && obj.f64("ExcSEXS.kc").unwrap_or(0.0) <= 0.0 {
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
            && obj.f64("GovSteamFV3.t5").unwrap_or(0.0) < 0.0 {
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

        let f = |key: &str| obj.f64(&format!("GovHydro4.{key}")).unwrap_or(0.0);
        if m == SIMPLE {
            for (val, prop, rule_id, name) in [
                (f("bmax"),  "bmax",  "dyu:GovHydro4.bmax-valueRange",  "C:302:DY:GovHydro4.bmax:valueRange"),
                (f("bgv0"),  "bgv0",  "dyu:GovHydro4.bgv0-valueRange",  "C:302:DY:GovHydro4.bgv0:valueRange"),
                (f("bgv1"),  "bgv1",  "dyu:GovHydro4.bgv1-valueRange",  "C:302:DY:GovHydro4.bgv1:valueRange"),
                (f("bgv2"),  "bgv2",  "dyu:GovHydro4.bgv2-valueRange",  "C:302:DY:GovHydro4.bgv2:valueRange"),
                (f("bgv3"),  "bgv3",  "dyu:GovHydro4.bgv3-valueRange",  "C:302:DY:GovHydro4.bgv3:valueRange"),
                (f("bgv4"),  "bgv4",  "dyu:GovHydro4.bgv4-valueRange",  "C:302:DY:GovHydro4.bgv4:valueRange"),
                (f("bgv5"),  "bgv5",  "dyu:GovHydro4.bgv5-valueRange",  "C:302:DY:GovHydro4.bgv5:valueRange"),
                (f("gv0"),   "gv0",   "dyu:GovHydro4.gv0-valueRange",   "C:302:DY:GovHydro4.gv0:valueRange"),
                (f("gv1"),   "gv1",   "dyu:GovHydro4.gv1-valueRange",   "C:302:DY:GovHydro4.gv1:valueRange"),
                (f("gv2"),   "gv2",   "dyu:GovHydro4.gv2-valueRange",   "C:302:DY:GovHydro4.gv2:valueRange"),
                (f("gv3"),   "gv3",   "dyu:GovHydro4.gv3-valueRange",   "C:302:DY:GovHydro4.gv3:valueRange"),
                (f("gv4"),   "gv4",   "dyu:GovHydro4.gv4-valueRange",   "C:302:DY:GovHydro4.gv4:valueRange"),
                (f("gv5"),   "gv5",   "dyu:GovHydro4.gv5-valueRange",   "C:302:DY:GovHydro4.gv5:valueRange"),
                (f("pgv0"),  "pgv0",  "dyu:GovHydro4.pgv0-valueRange",  "C:302:DY:GovHydro4.pgv0:valueRange"),
                (f("pgv1"),  "pgv1",  "dyu:GovHydro4.pgv1-valueRange",  "C:302:DY:GovHydro4.pgv1:valueRange"),
                (f("pgv2"),  "pgv2",  "dyu:GovHydro4.pgv2-valueRange",  "C:302:DY:GovHydro4.pgv2:valueRange"),
                (f("pgv3"),  "pgv3",  "dyu:GovHydro4.pgv3-valueRange",  "C:302:DY:GovHydro4.pgv3:valueRange"),
                (f("pgv4"),  "pgv4",  "dyu:GovHydro4.pgv4-valueRange",  "C:302:DY:GovHydro4.pgv4:valueRange"),
                (f("pgv5"),  "pgv5",  "dyu:GovHydro4.pgv5-valueRange",  "C:302:DY:GovHydro4.pgv5:valueRange"),
            ] {
                if val != 0.0 {
                    v.push(dyn_viol(mrid, rule_id, name, "GovHydro4", &format!("GovHydro4.{prop}"),
                        "The value is not 0 when GovHydro4.model is simple."));
                }
            }
        } else if m == FRANCIS_PELTON || m == KAPLAN {
            if m == FRANCIS_PELTON && f("bmax") != 0.0 {
                v.push(dyn_viol(mrid, "dyu:GovHydro4.bmax-valueRange", "C:302:DY:GovHydro4.bmax:valueRange",
                    "GovHydro4", "GovHydro4.bmax",
                    "The value is not 0 when GovHydro4.model is francisPelton."));
            }
            if m == FRANCIS_PELTON {
                for (val, prop, rule_id, name) in [
                    (f("bgv0"), "bgv0", "dyu:GovHydro4.bgv0-valueRange", "C:302:DY:GovHydro4.bgv0:valueRange"),
                    (f("bgv1"), "bgv1", "dyu:GovHydro4.bgv1-valueRange", "C:302:DY:GovHydro4.bgv1:valueRange"),
                    (f("bgv2"), "bgv2", "dyu:GovHydro4.bgv2-valueRange", "C:302:DY:GovHydro4.bgv2:valueRange"),
                    (f("bgv3"), "bgv3", "dyu:GovHydro4.bgv3-valueRange", "C:302:DY:GovHydro4.bgv3:valueRange"),
                    (f("bgv4"), "bgv4", "dyu:GovHydro4.bgv4-valueRange", "C:302:DY:GovHydro4.bgv4:valueRange"),
                    (f("bgv5"), "bgv5", "dyu:GovHydro4.bgv5-valueRange", "C:302:DY:GovHydro4.bgv5:valueRange"),
                ] {
                    if val != 0.0 {
                        v.push(dyn_viol(mrid, rule_id, name, "GovHydro4", &format!("GovHydro4.{prop}"),
                            "The value is not 0 when GovHydro4.model is francisPelton."));
                    }
                }
            }
            for (val, prev, prop, rule_id, name) in [
                (f("gv1"), f("gv0"), "gv1", "dyu:GovHydro4.gv1-valueRange", "C:302:DY:GovHydro4.gv1:valueRange"),
                (f("gv2"), f("gv1"), "gv2", "dyu:GovHydro4.gv2-valueRange", "C:302:DY:GovHydro4.gv2:valueRange"),
                (f("gv3"), f("gv2"), "gv3", "dyu:GovHydro4.gv3-valueRange", "C:302:DY:GovHydro4.gv3:valueRange"),
                (f("gv4"), f("gv3"), "gv4", "dyu:GovHydro4.gv4-valueRange", "C:302:DY:GovHydro4.gv4:valueRange"),
            ] {
                if val <= prev {
                    v.push(dyn_viol(mrid, rule_id, name, "GovHydro4", &format!("GovHydro4.{prop}"),
                        &format!("The value is not greater than GovHydro4.{} when GovHydro4.model is francisPelton or kaplan.", &prop[..prop.len()-1])));
                }
            }
            let gv5 = f("gv5");
            if gv5 <= f("gv4") || gv5 >= 1.0 {
                v.push(dyn_viol(mrid, "dyu:GovHydro4.gv5-valueRange", "C:302:DY:GovHydro4.gv5:valueRange",
                    "GovHydro4", "GovHydro4.gv5",
                    "The value is either not greater than GovHydro4.gv4 or it is not less than 1 when GovHydro4.model is francisPelton or kaplan."));
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
        let f = |key: &str| obj.f64(&format!("LoadStatic.{key}")).unwrap_or(0.0);

        if m == CONSTANT_Z {
            if f("kp1")!=0.0 || f("kp2")!=0.0 || f("kp3")!=0.0 || f("kp4")!=0.0 || f("kpf")!=0.0 ||
               f("kq1")!=0.0 || f("kq2")!=0.0 || f("kq3")!=0.0 || f("kq4")!=0.0 || f("kqf")!=0.0 ||
               f("ep1")!=0.0 || f("ep2")!=0.0 || f("ep3")!=0.0 ||
               f("eq1")!=0.0 || f("eq2")!=0.0 || f("eq3")!=0.0
            {
                v.push(dyn_viol(mrid,
                    "dyu:LoadStatic.staticLoadModelType-constantZ",
                    "C:302:DY:StaticLoadModelKind.constantZ:requiredAttributes",
                    "LoadStatic", "LoadStatic.staticLoadModelType",
                    "The load is represented as a constant impedance but other properties (attributes) are defined."));
            }
        } else if m == EXPONENTIAL {
            if f("kp4")!=0.0 || f("kq4")!=0.0 {
                v.push(dyn_viol(mrid,
                    "dyu:LoadStatic.staticLoadModelType-exponental",
                    "C:302:DY:StaticLoadModelKind.exponential:requiredAttributes",
                    "LoadStatic", "LoadStatic.staticLoadModelType",
                    "Unnecessary properties defined for exponential model type (kp4/kq4)."));
            }
        } else if m == ZIP1 {
            if f("ep1")!=0.0 || f("ep2")!=0.0 || f("ep3")!=0.0 ||
               f("eq1")!=0.0 || f("eq2")!=0.0 || f("eq3")!=0.0 ||
               f("kp4")!=0.0 || f("kq4")!=0.0
            {
                v.push(dyn_viol(mrid,
                    "dyu:LoadStatic.staticLoadModelType-zIP1",
                    "C:302:DY:StaticLoadModelKind.zIP1:requiredAttributes",
                    "LoadStatic", "LoadStatic.staticLoadModelType",
                    "Unnecessary properties defined for zIP1 model type."));
            }
        } else if m == ZIP2
            && (f("ep1")!=0.0 || f("ep2")!=0.0 || f("ep3")!=0.0 ||
               f("eq1")!=0.0 || f("eq2")!=0.0 || f("eq3")!=0.0)
            {
                v.push(dyn_viol(mrid,
                    "dyu:LoadStatic.staticLoadModelType-zIP2",
                    "C:302:DY:StaticLoadModelKind.zIP2:requiredAttributes",
                    "LoadStatic", "LoadStatic.staticLoadModelType",
                    "Unnecessary properties defined for zIP2 model type."));
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
        if obj.f64("RotatingMachineDynamics.saturationFactor").unwrap_or(0.0) != 0.0 ||
           obj.f64("RotatingMachineDynamics.saturationFactor120").unwrap_or(0.0) != 0.0
        {
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

// -- Dynamics associations check --

fn check_dynamics_associations(dataset: &CimDataset) -> Vec<Violation> {
    const GOVERNORS: &[&str] = &[
        "GovCT1", "GovCT2", "GovGAST", "GovGAST1", "GovGAST2", "GovGAST3", "GovGAST4", "GovGASTWD",
        "GovHydro1", "GovHydro2", "GovHydro3", "GovHydro4", "GovHydroDD", "GovHydroFrancis",
        "GovHydroIEEE0", "GovHydroIEEE2", "GovHydroPID", "GovHydroPID2", "GovHydroPelton",
        "GovHydroR", "GovHydroWEH", "GovHydroWPID",
        "GovSteam0", "GovSteam1", "GovSteam2", "GovSteamBB", "GovSteamEU",
        "GovSteamFV2", "GovSteamFV3", "GovSteamFV4", "GovSteamIEEE1", "GovSteamSGO",
    ];
    const MECHANICAL_LOADS: &[&str] = &["MechLoad1", "MechanicalLoadUserDefined"];
    let mut v = Vec::new();
    for (classes, base, rule_id, name) in [
        (GOVERNORS, "TurbineGovernorDynamics", "dyu:TurbineGovernorDynamics",
         "C:302:DY:TurbineGovernorDynamics:associationsCondition"),
        (MECHANICAL_LOADS, "MechanicalLoadDynamics", "dyu:MechanicalLoadDynamics",
         "C:302:DY:MechanicalLoadDynamics:associationsCondition"),
    ] {
        let sync_key = format!("{base}.SynchronousMachineDynamics");
        let async_key = format!("{base}.AsynchronousMachineDynamics");
        for class in classes {
            for mrid in dataset.by_type.get(*class).into_iter().flatten() {
                let Some(obj) = Fields::of_class(&dataset.entries[mrid], class) else { continue };
                if obj.reference(&sync_key).is_none() && obj.reference(&async_key).is_none() {
                    v.push(Violation {
                        object_id:   mrid.clone(),
                        rule_id:     rule_id.into(),
                        name:        name.into(),
                        class:       class.to_string(),
                        property:    "rdf:type".into(),
                        message:     "Required association to either SynchronousMachineDynamics or to AsynchronousMachineDynamics is missing.".into(),
                        severity:    "sh:Violation".into(),
                        description: String::new(),
                    });
                }
            }
        }
    }
    v
}
