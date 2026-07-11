mod common;

use cimvalidation::Config;
use cimvalidation::Violation;

fn validate(ds: &cimdecoder::CimDataset, cfg: &Config) -> Vec<Violation> {
    let mut v = Vec::new();
    for profile in &cfg.profiles {
        v.extend(cimvalidation::sparql::validate_profile_local(ds, profile, cfg));
    }
    v.extend(cimvalidation::sparql::validate_crossprofile(ds, cfg));
    v
}

#[test]
fn sparql_dl_001() {
    // DiagramObject.IdentifiedObject must NOT point to Diagram/DiagramObject/etc.
    let ds = common::load_dataset("../testdata/test_sparql_DL_001.xml");
    let cfg = Config { profiles: vec!["DL".into()], ..Default::default() };
    let vs = validate(&ds, &cfg);
    let by_id = common::violations_by_id(&vs);
    assert_eq!(by_id.get("DiagramObject.OK").map_or(0, |v| v.len()), 0,
        "DiagramObject.OK: expected 0 violations, got: {:?}", by_id.get("DiagramObject.OK"));
    assert_eq!(by_id.get("DiagramObject.BAD").map_or(0, |v| v.len()), 1,
        "DiagramObject.BAD: expected 1 violation, got: {:?}", by_id.get("DiagramObject.BAD"));
    assert_eq!(by_id.get("TextDiagramObject.BAD").map_or(0, |v| v.len()), 1,
        "TextDiagramObject.BAD: expected 1 violation, got: {:?}", by_id.get("TextDiagramObject.BAD"));
}

#[test]
fn sparql_eqbd_001() {
    // isExcludedFromAreaInterchange=false requires TieFlow; true forbids TieFlow.
    let ds = common::load_dataset("../testdata/test_sparql_EQBD_001.xml");
    let cfg = Config { profiles: vec!["EQBD".into()], ..Default::default() };
    let vs = validate(&ds, &cfg);
    let by_id = common::violations_by_id(&vs);
    assert_eq!(by_id.get("BP.OK1").map_or(0, |v| v.len()), 0,
        "BP.OK1: expected 0 violations, got: {:?}", by_id.get("BP.OK1"));
    assert_eq!(by_id.get("BP.OK2").map_or(0, |v| v.len()), 0,
        "BP.OK2: expected 0 violations, got: {:?}", by_id.get("BP.OK2"));
    assert_eq!(by_id.get("BP.BAD1").map_or(0, |v| v.len()), 1,
        "BP.BAD1: expected 1 violation, got: {:?}", by_id.get("BP.BAD1"));
    assert_eq!(by_id.get("BP.BAD2").map_or(0, |v| v.len()), 1,
        "BP.BAD2: expected 1 violation, got: {:?}", by_id.get("BP.BAD2"));
}

#[test]
fn sparql_sc_notsolved_001() {
    // MutualCoupling.First_Terminal and Second_Terminal must point to different ACLineSegments.
    let ds = common::load_dataset("../testdata/test_sparql_SC_NOTSOLVED_001.xml");
    let cfg = Config { profiles: vec!["SC".into()], not_solved: true, ..Default::default() };
    let vs = validate(&ds, &cfg);
    let by_id = common::violations_by_id(&vs);
    assert_eq!(by_id.get("MC.OK").map_or(0, |v| v.len()), 0,
        "MC.OK: expected 0 violations, got: {:?}", by_id.get("MC.OK"));
    assert_eq!(by_id.get("MC.BAD.SAME").map_or(0, |v| v.len()), 1,
        "MC.BAD.SAME: expected 1 violation, got: {:?}", by_id.get("MC.BAD.SAME"));
    assert_eq!(by_id.get("MC.BAD.TYPE").map_or(0, |v| v.len()), 1,
        "MC.BAD.TYPE: expected 1 violation, got: {:?}", by_id.get("MC.BAD.TYPE"));
}

#[test]
fn sparql_sc_001_varistor() {
    // varistorRatedCurrent/VoltageThreshold only exchanged if varistorPresent is true.
    let ds = common::load_dataset("../testdata/test_sparql_SC_001.xml");
    let cfg = Config { profiles: vec!["SC".into()], ..Default::default() };
    let vs = validate(&ds, &cfg);
    let by_id = common::violations_by_id(&vs);
    assert_eq!(by_id.get("SC.OK.1").map_or(0, |v| v.len()), 0,
        "SC.OK.1: expected 0 violations, got: {:?}", by_id.get("SC.OK.1"));
    assert_eq!(by_id.get("SC.OK.2").map_or(0, |v| v.len()), 0,
        "SC.OK.2: expected 0 violations, got: {:?}", by_id.get("SC.OK.2"));
    assert_eq!(by_id.get("SC.BAD.1").map_or(0, |v| v.len()), 1,
        "SC.BAD.1: expected 1 violation, got: {:?}", by_id.get("SC.BAD.1"));
    assert_eq!(by_id.get("SC.BAD.2").map_or(0, |v| v.len()), 1,
        "SC.BAD.2: expected 1 violation, got: {:?}", by_id.get("SC.BAD.2"));
}

#[test]
fn sparql_sc_002_452() {
    let ds = common::load_dataset("../testdata/test_sparql_SC_002.xml");
    let cfg = Config { profiles: vec!["SC".into()], ..Default::default() };
    let vs = validate(&ds, &cfg);
    let by_id = common::violations_by_id(&vs);
    assert_eq!(by_id.get("SM.OK").map_or(0, |v| v.len()), 0,
        "SM.OK: expected 0 violations, got: {:?}", by_id.get("SM.OK"));
    assert_eq!(by_id.get("SM.BAD").map_or(0, |v| v.len()), 1,
        "SM.BAD: expected 1 violation, got: {:?}", by_id.get("SM.BAD"));
    assert_eq!(by_id.get("PTE.OK").map_or(0, |v| v.len()), 0,
        "PTE.OK: expected 0 violations, got: {:?}", by_id.get("PTE.OK"));
    assert_eq!(by_id.get("PTE.BAD").map_or(0, |v| v.len()), 1,
        "PTE.BAD: expected 1 violation, got: {:?}", by_id.get("PTE.BAD"));
}

#[test]
fn sparql_sc_003_6002() {
    // varistorRatedCurrent and varistorVoltageThreshold are required if varistorPresent is true.
    let ds = common::load_dataset("../testdata/test_sparql_SC_003.xml");
    let cfg = Config { profiles: vec!["SC".into()], ..Default::default() };
    let vs = validate(&ds, &cfg);
    let by_id = common::violations_by_id(&vs);
    assert_eq!(by_id.get("SC.OK.1").map_or(0, |v| v.len()), 0,
        "SC.OK.1: expected 0 violations, got: {:?}", by_id.get("SC.OK.1"));
    assert_eq!(by_id.get("SC.OK.2").map_or(0, |v| v.len()), 0,
        "SC.OK.2: expected 0 violations, got: {:?}", by_id.get("SC.OK.2"));
    assert_eq!(by_id.get("SC.BAD.REQUIRED").map_or(0, |v| v.len()), 2,
        "SC.BAD.REQUIRED: expected 2 violations (both fields missing), got: {:?}", by_id.get("SC.BAD.REQUIRED"));
}

#[test]
fn sparql_sv_001() {
    // alpha [10, 18] for rectifier, gamma [17, 20] for inverter.
    let ds = common::load_dataset("../testdata/test_sparql_SV_001.xml");
    let cfg = Config { profiles: vec!["SV".into()], ..Default::default() };
    let vs = validate(&ds, &cfg);
    let by_id = common::violations_by_id(&vs);
    assert_eq!(by_id.get("CSC.RECT.OK").map_or(0, |v| v.len()), 0,
        "CSC.RECT.OK: expected 0 violations, got: {:?}", by_id.get("CSC.RECT.OK"));
    assert_eq!(by_id.get("CSC.INV.OK").map_or(0, |v| v.len()), 0,
        "CSC.INV.OK: expected 0 violations, got: {:?}", by_id.get("CSC.INV.OK"));
    assert_eq!(by_id.get("CSC.RECT.BAD").map_or(0, |v| v.len()), 1,
        "CSC.RECT.BAD: expected 1 violation, got: {:?}", by_id.get("CSC.RECT.BAD"));
    assert_eq!(by_id.get("CSC.INV.BAD").map_or(0, |v| v.len()), 1,
        "CSC.INV.BAD: expected 1 violation, got: {:?}", by_id.get("CSC.INV.BAD"));
}

#[test]
fn sparql_sv_solved_001_tap() {
    // SvTapStep.position must be within [lowStep, highStep] of the associated TapChanger.
    let ds = common::load_dataset("../testdata/test_sparql_SV_SOLVED_001.xml");
    let cfg = Config { profiles: vec!["SV".into()], solved: true, ..Default::default() };
    let vs = validate(&ds, &cfg);
    let by_id = common::violations_by_id(&vs);
    assert_eq!(by_id.get("SV.OK.1").map_or(0, |v| v.len()), 0,
        "SV.OK.1: expected 0 violations, got: {:?}", by_id.get("SV.OK.1"));
    assert_eq!(by_id.get("SV.BAD.LOW").map_or(0, |v| v.len()), 1,
        "SV.BAD.LOW: expected 1 violation, got: {:?}", by_id.get("SV.BAD.LOW"));
    assert_eq!(by_id.get("SV.BAD.HIGH").map_or(0, |v| v.len()), 1,
        "SV.BAD.HIGH: expected 1 violation, got: {:?}", by_id.get("SV.BAD.HIGH"));
}

#[test]
fn sparql_sv_solved_002_angle_ref() {
    // Priority 1 SM must be at the AngleRefTopologicalNode.
    let ds = common::load_dataset("../testdata/test_sparql_SV_SOLVED_002.xml");
    let cfg = Config { profiles: vec!["SV".into()], solved: true, common: true, ..Default::default() };
    let vs = validate(&ds, &cfg);
    let by_id = common::violations_by_id(&vs);
    let sm_ok_non_uuid = by_id.get("SM.OK").map_or(0, |vs| vs.iter()
        .filter(|v| !v.rule_id.starts_with("all600:All-GENC"))
        .count());
    assert_eq!(sm_ok_non_uuid, 0,
        "SM.OK: expected 0 non-UUID violations, got: {:?}", by_id.get("SM.OK"));
    assert!(by_id.get("SM.BAD.NODE").map_or(0, |v| v.len()) >= 1,
        "SM.BAD.NODE: expected violation, got none");
    assert!(
        by_id.get("global").map_or(false, |vs| vs.iter().any(|v| v.message.contains("Multiple machines"))),
        "global: expected violation for duplicate priority 1 machines, got: {:?}", by_id.get("global")
    );
    assert!(by_id.get("TN.OTHER").map_or(0, |v| v.len()) >= 1,
        "TN.OTHER: expected violation for missing SvVoltage, got none");
}

#[test]
fn sparql_sv_solved_003_456() {
    let ds = common::load_dataset("../testdata/test_sparql_SV_SOLVED_003.xml");
    let cfg = Config { profiles: vec!["SV".into()], solved: true, ..Default::default() };
    let vs = validate(&ds, &cfg);
    let by_id = common::violations_by_id(&vs);
    for id in &["SM.ENERGIZED", "SW.1", "SVSC.BAD", "SVTS.BAD", "SVV.BAD"] {
        assert!(by_id.get(*id).map_or(0, |v| v.len()) >= 1,
            "{}: expected violation, got none", id);
    }
}

#[test]
fn sparql_sv_solved_004_600_1() {
    let ds = common::load_dataset("../testdata/test_sparql_SV_SOLVED_004.xml");
    let cfg = Config { profiles: vec!["SV".into()], solved: true, common: true, ..Default::default() };
    let vs = validate(&ds, &cfg);
    let by_id = common::violations_by_id(&vs);
    for id in &["S1", "LSC.SYNC.BAD", "RTC.SYNC.BAD", "SM.ENERGIZED.NO_STATUS", "LSC.ENERGIZED.NO_SVSC"] {
        assert!(by_id.get(*id).map_or(0, |v| v.len()) >= 1,
            "{}: expected violation, got none", id);
    }
}

#[test]
fn sparql_sv_solved_005_rc() {
    let ds = common::load_dataset("../testdata/test_sparql_SV_SOLVED_005.xml");
    let cfg = Config { profiles: vec!["SV".into()], solved: true, common: true, ..Default::default() };
    let vs = validate(&ds, &cfg);
    let by_id = common::violations_by_id(&vs);
    let rc_v2_samepoint = by_id.get("RC.V.2").map_or(0, |vs| vs.iter()
        .filter(|v| v.rule_id.contains("samePoint"))
        .count());
    assert_eq!(rc_v2_samepoint, 1,
        "RC.V.2: expected 1 samePoint violation for contradictory target, got: {:?}", by_id.get("RC.V.2"));
    assert!(by_id.get("RC.V.1").map_or(0, |v| v.len()) >= 1,
        "RC.V.1: expected violation for machine/tap island mismatch, got none");
}

#[test]
fn sparql_ssh_notsolved_001() {
    let ds = common::load_dataset("../testdata/test_sparql_SSH_NOTSOLVED_001.xml");
    let cfg = Config { profiles: vec!["SSH".into()], not_solved: true, ..Default::default() };
    let vs = validate(&ds, &cfg);
    let by_id = common::violations_by_id(&vs);
    for id in &[
        "CA.INTERCHANGE.BAD",
        "CSC.INV.BAD.ALPHA",
        "CSC.RECT.BAD.GAMMA",
        "LSC.BAD.SECTIONS",
        "LSC.NONINT.SECTIONS",
        "NSC.BAD.SECTIONS",
        "RC.PF.BAD",
        "RTC.BAD.STEP",
    ] {
        assert!(by_id.get(*id).map_or(0, |v| v.len()) >= 1,
            "{}: expected violation, got none", id);
    }
    // C:456:SSH:TapChanger.step:value (discrete AND enabled) is a distinct, stricter rule
    // than C:301's discrete-only check above — assert its name shows up specifically.
    let step_enabled = by_id.get("RTC.BAD.STEP.ENABLED").map_or(0, |vs| vs.iter()
        .filter(|v| v.name == "C:456:SSH:TapChanger.step:value")
        .count());
    assert_eq!(step_enabled, 1,
        "RTC.BAD.STEP.ENABLED: expected 1 C:456:SSH:TapChanger.step:value violation, got: {:?}",
        by_id.get("RTC.BAD.STEP.ENABLED"));
}

#[test]
fn sparql_ssh_001() {
    let ds = common::load_dataset("../testdata/test_sparql_SSH_001.xml");
    let cfg = Config { profiles: vec!["SSH".into()], ..Default::default() };
    let vs = validate(&ds, &cfg);
    let by_id = common::violations_by_id(&vs);
    for id in &[
        "ES.CONSUMER",
        "RC.CONT.WITH.DEAD",
        "RC.DISC.WITHOUT.DEAD",
        "CSC.RECT.BAD.RANGE",
        "VSC.P.BAD.DROOP",
    ] {
        assert!(by_id.get(*id).map_or(0, |v| v.len()) >= 1,
            "{}: expected violation, got none", id);
    }
}

#[test]
fn sparql_tp_001_phase_code() {
    // Terminals at the same TopologicalNode must have consistent phase codes.
    let ds = common::load_dataset("../testdata/test_sparql_TP_001.xml");
    let cfg = Config { profiles: vec!["TP".into()], not_solved: true, ..Default::default() };
    let vs = validate(&ds, &cfg);
    let by_id = common::violations_by_id(&vs);
    assert_eq!(by_id.get("TN.OK").map_or(0, |v| v.len()), 0,
        "TN.OK: expected 0 violations, got: {:?}", by_id.get("TN.OK"));
    assert_eq!(by_id.get("TN.BAD").map_or(0, |v| v.len()), 1,
        "TN.BAD: expected 1 violation, got: {:?}", by_id.get("TN.BAD"));
}

#[test]
fn sparql_tp_002_exch8() {
    // Terminal.TopologicalNode is required if a RegulatingControl is associated.
    let ds = common::load_dataset("../testdata/test_sparql_TP_002.xml");
    let cfg = Config { profiles: vec!["TP".into()], not_solved: true, ..Default::default() };
    let vs = validate(&ds, &cfg);
    let by_id = common::violations_by_id(&vs);
    assert_eq!(by_id.get("Term.OK").map_or(0, |v| v.len()), 0,
        "Term.OK: expected 0 violations, got: {:?}", by_id.get("Term.OK"));
    assert_eq!(by_id.get("Term.BAD").map_or(0, |v| v.len()), 1,
        "Term.BAD: expected 1 violation, got: {:?}", by_id.get("Term.BAD"));
}

#[test]
fn sparql_tp_003_same_tn() {
    // Terminals of a retained Switch shall not be connected to the same TopologicalNode.
    let ds = common::load_dataset("../testdata/test_sparql_TP_003.xml");
    let cfg = Config { profiles: vec!["TP".into()], not_solved: true, ..Default::default() };
    let vs = validate(&ds, &cfg);
    let by_id = common::violations_by_id(&vs);
    assert_eq!(by_id.get("SW.OK").map_or(0, |v| v.len()), 0,
        "SW.OK: expected 0 violations, got: {:?}", by_id.get("SW.OK"));
    assert_eq!(by_id.get("SW.NOT_RETAINED.OK").map_or(0, |v| v.len()), 0,
        "SW.NOT_RETAINED.OK: expected 0 violations, got: {:?}", by_id.get("SW.NOT_RETAINED.OK"));
    assert_eq!(by_id.get("SW.BAD").map_or(0, |v| v.len()), 1,
        "SW.BAD: expected 1 violation, got: {:?}", by_id.get("SW.BAD"));
}

#[test]
fn sparql_dy_001_mbase() {
    // mwbase must equal RotatingMachine.ratedPowerFactor * RotatingMachine.ratedS.
    let ds = common::load_dataset("../testdata/test_sparql_DY_001.xml");
    let cfg = Config { profiles: vec!["DY".into()], ..Default::default() };
    let vs = validate(&ds, &cfg);
    let by_id = common::violations_by_id(&vs);
    assert_eq!(by_id.get("GOV.OK").map_or(0, |v| v.len()), 0,
        "GOV.OK: expected 0 violations, got: {:?}", by_id.get("GOV.OK"));
    assert_eq!(by_id.get("GOV.BAD").map_or(0, |v| v.len()), 1,
        "GOV.BAD: expected 1 violation, got: {:?}", by_id.get("GOV.BAD"));
}

#[test]
fn sparql_dy_002_exc() {
    // ExcitationSystemDynamics.SynchronousMachineDynamics shall not point to SynchronousMachineSimplified.
    let ds = common::load_dataset("../testdata/test_sparql_DY_002.xml");
    let cfg = Config { profiles: vec!["DY".into()], ..Default::default() };
    let vs = validate(&ds, &cfg);
    let by_id = common::violations_by_id(&vs);
    assert_eq!(by_id.get("EXC.OK").map_or(0, |v| v.len()), 0,
        "EXC.OK: expected 0 violations, got: {:?}", by_id.get("EXC.OK"));
    assert_eq!(by_id.get("EXC.BAD").map_or(0, |v| v.len()), 1,
        "EXC.BAD: expected 1 violation, got: {:?}", by_id.get("EXC.BAD"));
}

#[test]
fn sparql_dy_003_302() {
    let ds = common::load_dataset("../testdata/test_sparql_DY_003.xml");
    let cfg = Config { profiles: vec!["DY".into()], ..Default::default() };
    let vs = validate(&ds, &cfg);
    let by_id = common::violations_by_id(&vs);
    for id in &[
        "EXC.AC8B.BAD",
        "EXC.BBC.BAD",
        "EXC.DC4B.BAD",
        "PSS.2ST.BAD",
        "GOV.H4.SIMPLE.BAD",
        "GOV.H4.KAPLAN.BAD",
        "GOV.H4.BGV.BAD",
        "LOAD.STATIC.Z.BAD",
        "SM.SAT.BAD",
        "SMS.BAD",
        "MECH.BAD",
    ] {
        assert!(by_id.get(*id).map_or(0, |v| v.len()) >= 1,
            "{}: expected violation, got none", id);
    }
}

#[test]
fn sparql_common_001() {
    // Common CGMES rules: model header, UUID syntax, duplicate mRID, NaN, string lengths, EIC.
    let ds = common::load_dataset("../testdata/test_sparql_COMMON_001.xml");
    let cfg = Config { common: true, ..Default::default() };
    let vs = validate(&ds, &cfg);
    let by_id = common::violations_by_id(&vs);
    for id in &[
        "urn:uuid:header-1",
        "Substation-Not-A-UUID",
        "_7336d36e-d917-4e54-9469-8730b200b3d5",
        "_6336d36e-d917-4e54-9469-8730b200b3d5",
        "_5336d36e-d917-4e54-9469-8730b200b3d5",
        "_4336d36e-d917-4e54-9469-8730b200b3d5",
    ] {
        assert!(by_id.get(*id).map_or(0, |v| v.len()) >= 1,
            "{}: expected violation, got none", id);
    }
    // Duplicate mRID: reported on one or both of the pair
    assert!(
        by_id.get("_8336d36e-d917-4e54-9469-8730b200b3d5").map_or(0, |v| v.len()) >= 1
        || by_id.get("_9336d36e-d917-4e54-9469-8730b200b3d5").map_or(0, |v| v.len()) >= 1,
        "duplicate mRID: expected violation on _833... or _933..., got none"
    );
}

#[test]
fn sparql_eq_notsolved_001() {
    // RegulatingControl.targetValue must be within TapChanger capability limits.
    let ds = common::load_dataset("../testdata/test_sparql_EQ_NOTSOLVED_001.xml");
    let cfg = Config { profiles: vec!["EQ".into()], not_solved: true, ..Default::default() };
    let vs = validate(&ds, &cfg);
    let by_id = common::violations_by_id(&vs);
    assert_eq!(by_id.get("TCC.OK").map_or(0, |v| v.len()), 0,
        "TCC.OK: expected 0 violations, got: {:?}", by_id.get("TCC.OK"));
    assert_eq!(by_id.get("TCC.BAD").map_or(0, |v| v.len()), 1,
        "TCC.BAD: expected 1 violation, got: {:?}", by_id.get("TCC.BAD"));
}

#[test]
fn sparql_eq_001_452() {
    // Switch terminals must share BaseVoltage; ACLineSegment terminals must use different ConnectivityNodes.
    let ds = common::load_dataset("../testdata/test_sparql_EQ_001.xml");
    let cfg = Config { profiles: vec!["EQ".into()], ..Default::default() };
    let vs = validate(&ds, &cfg);
    let by_id = common::violations_by_id(&vs);
    assert_eq!(by_id.get("SW.OK.SAME_VL").map_or(0, |v| v.len()), 0,
        "SW.OK.SAME_VL: expected 0 violations, got: {:?}", by_id.get("SW.OK.SAME_VL"));
    assert_eq!(by_id.get("SW.BAD.DIFF_BV").map_or(0, |v| v.len()), 1,
        "SW.BAD.DIFF_BV: expected 1 violation, got: {:?}", by_id.get("SW.BAD.DIFF_BV"));
    assert_eq!(by_id.get("Line.BAD.SAME_CN").map_or(0, |v| v.len()), 1,
        "Line.BAD.SAME_CN: expected 1 violation, got: {:?}", by_id.get("Line.BAD.SAME_CN"));
}

#[test]
fn sparql_eq_002_6002() {
    let ds = common::load_dataset("../testdata/test_sparql_EQ_002.xml");
    let cfg = Config { profiles: vec!["EQ".into()], ..Default::default() };
    let vs = validate(&ds, &cfg);
    let by_id = common::violations_by_id(&vs);
    assert!(by_id.get("global").map_or(0, |v| v.len()) >= 1,
        "global: expected violation for substation count, got none");
    assert_eq!(by_id.get("RCC1").map_or(0, |v| v.len()), 1,
        "RCC1: expected 1 violation for units, got: {:?}", by_id.get("RCC1"));
    assert_eq!(by_id.get("RTC1").map_or(0, |v| v.len()), 1,
        "RTC1: expected 1 violation for neutralU sync, got: {:?}", by_id.get("RTC1"));
}

#[test]
fn quality_001_rc_target_voltage_mismatch() {
    // quality:RegulatingControl.targetVoltageMismatch — regression test for a bug where the
    // mode comparison used a full CIM100 namespace URI constant, but cimdecoder strips every
    // rdf:resource down to its bare local name, so the check never fired.
    let ds = common::load_dataset("../testdata/test_quality_001.xml");
    let cfg = Config { quality: true, ..Default::default() };
    let vs = validate(&ds, &cfg);
    let by_id = common::violations_by_id(&vs);
    assert_eq!(by_id.get("RC.V.OK").map_or(0, |v| v.len()), 0,
        "RC.V.OK: expected 0 violations, got: {:?}", by_id.get("RC.V.OK"));
    let bad = by_id.get("RC.V.BAD").map_or(0, |vs| vs.iter()
        .filter(|v| v.rule_id == "quality:RegulatingControl.targetVoltageMismatch")
        .count());
    assert_eq!(bad, 1,
        "RC.V.BAD: expected 1 targetVoltageMismatch violation, got: {:?}", by_id.get("RC.V.BAD"));
}

#[test]
fn sparql_eq_003_lrc() {
    // LoadResponseCharacteristic.exponentModel: exponent/coefficient/coefficientSum.
    let ds = common::load_dataset("../testdata/test_sparql_EQ_003.xml");
    let cfg = Config { profiles: vec!["EQ".into()], ..Default::default() };
    let vs = validate(&ds, &cfg);
    let by_id = common::violations_by_id(&vs);

    let name_of = |id: &str| -> Vec<String> {
        by_id.get(id).map_or(Vec::new(), |vs| vs.iter().map(|v| v.name.clone()).collect())
    };

    assert_eq!(by_id.get("LRC.EXP.OK").map_or(0, |v| v.len()), 0,
        "LRC.EXP.OK: expected 0 violations, got: {:?}", by_id.get("LRC.EXP.OK"));
    assert_eq!(by_id.get("LRC.COEF.OK").map_or(0, |v| v.len()), 0,
        "LRC.COEF.OK: expected 0 violations, got: {:?}", by_id.get("LRC.COEF.OK"));

    assert!(name_of("LRC.EXP.BAD").iter().any(|n| n == "C:301:EQ:LoadResponseCharacteristic.exponentModel:exponent"),
        "LRC.EXP.BAD: expected an :exponent violation, got: {:?}", by_id.get("LRC.EXP.BAD"));
    assert!(name_of("LRC.COEF.BAD").iter().any(|n| n == "C:301:EQ:LoadResponseCharacteristic.exponentModel:coefficient"),
        "LRC.COEF.BAD: expected a :coefficient violation, got: {:?}", by_id.get("LRC.COEF.BAD"));
    assert!(name_of("LRC.SUM.BAD").iter().any(|n| n == "C:301:EQ:LoadResponseCharacteristic.exponentModel:coefficientSum"),
        "LRC.SUM.BAD: expected a :coefficientSum violation, got: {:?}", by_id.get("LRC.SUM.BAD"));
}

#[test]
fn sparql_sv_solved_006_operational_limits() {
    // SvVoltage.v:limits, C:456 — value must be within [VoltageLimit(low), VoltageLimit(high)]
    // for OperationalLimitSets on terminals connected to the SvVoltage's TopologicalNode.
    let ds = common::load_dataset("../testdata/test_sparql_SV_SOLVED_006.xml");
    let cfg = Config { profiles: vec!["SV".into()], solved: true, ..Default::default() };
    let vs = validate(&ds, &cfg);
    let by_id = common::violations_by_id(&vs);
    assert_eq!(by_id.get("SVV.OK").map_or(0, |v| v.len()), 0,
        "SVV.OK: expected 0 violations, got: {:?}", by_id.get("SVV.OK"));
    let bad_limits = by_id.get("SVV.BAD").map_or(0, |vs| vs.iter()
        .filter(|v| v.name == "C:456:SV:SvVoltage.v:limits")
        .count());
    assert_eq!(bad_limits, 1,
        "SVV.BAD: expected 1 C:456:SV:SvVoltage.v:limits violation, got: {:?}", by_id.get("SVV.BAD"));
}

#[test]
fn sparql_op_001() {
    // Measurement.Terminal must reference a Terminal of Measurement.PowerSystemResource,
    // unless measurementType is TapPosition or SwitchPosition.
    let ds = common::load_dataset("../testdata/test_sparql_OP_001.xml");
    let cfg = Config { profiles: vec!["OP".into()], ..Default::default() };
    let vs = validate(&ds, &cfg);
    let by_id = common::violations_by_id(&vs);
    assert_eq!(by_id.get("MEAS.OK").map_or(0, |v| v.len()), 0,
        "MEAS.OK: expected 0 violations, got: {:?}", by_id.get("MEAS.OK"));
    assert_eq!(by_id.get("MEAS.TAP.OK").map_or(0, |v| v.len()), 0,
        "MEAS.TAP.OK: expected 0 violations, got: {:?}", by_id.get("MEAS.TAP.OK"));
    assert_eq!(by_id.get("MEAS.BAD.TERMINAL").map_or(0, |v| v.len()), 1,
        "MEAS.BAD.TERMINAL: expected 1 violation, got: {:?}", by_id.get("MEAS.BAD.TERMINAL"));
    assert_eq!(by_id.get("MEAS.TAP.BAD").map_or(0, |v| v.len()), 1,
        "MEAS.TAP.BAD: expected 1 violation, got: {:?}", by_id.get("MEAS.TAP.BAD"));
    assert_eq!(by_id.get("MEAS.VOLT.BAD.ABSENT").map_or(0, |v| v.len()), 1,
        "MEAS.VOLT.BAD.ABSENT: expected 1 violation, got: {:?}", by_id.get("MEAS.VOLT.BAD.ABSENT"));
}
