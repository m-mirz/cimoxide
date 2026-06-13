mod common;

use cimvalidation::validate_generated;

#[test]
fn shacl_gl_001() {
    let ds = common::load_dataset("../testdata/test_shacl_GL_001.xml");
    let vs = validate_generated(&ds, &["GL"]);
    let by_id = common::violations_by_id(&vs);
    assert_eq!(by_id.get("CoordinateSystem.WGS84").map_or(0, |v| v.len()), 0,
        "CoordinateSystem.WGS84 (default crsUrn): expected 0 violations, got: {:?}", by_id.get("CoordinateSystem.WGS84"));
    assert_eq!(by_id.get("CoordinateSystem.ETRS89").map_or(0, |v| v.len()), 1,
        "CoordinateSystem.ETRS89 (non-default crsUrn): expected 1 violation, got: {:?}", by_id.get("CoordinateSystem.ETRS89"));
}

#[test]
fn shacl_dl_001_diagram_style_name() {
    // DiagramStyle.name must be one of the allowed values (C:453:DL:DiagramStyle:name).
    let ds = common::load_dataset("../testdata/test_shacl_DL_001.xml");
    let vs = validate_generated(&ds, &["DL"]);
    let by_id = common::violations_by_id(&vs);
    assert_eq!(by_id.get("DiagramStyle.OK").map_or(0, |v| v.len()), 0,
        "DiagramStyle.OK (name=node-breaker): expected 0 violations, got: {:?}", by_id.get("DiagramStyle.OK"));
    assert_eq!(by_id.get("DiagramStyle.BAD").map_or(0, |v| v.len()), 1,
        "DiagramStyle.BAD (name=invalid-style): expected 1 violation, got: {:?}", by_id.get("DiagramStyle.BAD"));
}

#[test]
fn shacl_dl_002_sequence_number() {
    // DiagramObjectPoint.sequenceNumber must be > 0 (sh:minExclusive 0).
    let ds = common::load_dataset("../testdata/test_shacl_DL_002.xml");
    let vs = validate_generated(&ds, &["DL"]);
    let by_id = common::violations_by_id(&vs);
    assert_eq!(by_id.get("DiagramObjectPoint.OK").map_or(0, |v| v.len()), 0,
        "DiagramObjectPoint.OK (sequenceNumber=1): expected 0 violations, got: {:?}", by_id.get("DiagramObjectPoint.OK"));
    assert_eq!(by_id.get("DiagramObjectPoint.NEG").map_or(0, |v| v.len()), 1,
        "DiagramObjectPoint.NEG (sequenceNumber=-1): expected 1 violation, got: {:?}", by_id.get("DiagramObjectPoint.NEG"));
}

#[test]
fn shacl_eq_001() {
    // ACLineSegment.length >= 0 (sh:minInclusive 0).
    // BaseVoltage.nominalVoltage > 0 (sh:minExclusive 0).
    let ds = common::load_dataset("../testdata/test_shacl_EQ_001.xml");
    let vs = validate_generated(&ds, &["EQ"]);
    let by_id = common::violations_by_id(&vs);
    assert_eq!(by_id.get("ACLineSegment.OK").map_or(0, |v| v.len()), 0,
        "ACLineSegment.OK (length=5): expected 0 violations, got: {:?}", by_id.get("ACLineSegment.OK"));
    assert_eq!(by_id.get("ACLineSegment.BAD").map_or(0, |v| v.len()), 1,
        "ACLineSegment.BAD (length=-1): expected 1 violation, got: {:?}", by_id.get("ACLineSegment.BAD"));
    assert_eq!(by_id.get("BaseVoltage.OK").map_or(0, |v| v.len()), 0,
        "BaseVoltage.OK (nominalVoltage=110): expected 0 violations, got: {:?}", by_id.get("BaseVoltage.OK"));
    assert_eq!(by_id.get("BaseVoltage.BAD").map_or(0, |v| v.len()), 1,
        "BaseVoltage.BAD (nominalVoltage=-1): expected 1 violation, got: {:?}", by_id.get("BaseVoltage.BAD"));
}

#[test]
fn shacl_ssh_001_battery_unit() {
    // BatteryUnit.storedE must be < ratedE (sh:lessThan).
    let ds = common::load_dataset("../testdata/test_shacl_SSH_001.xml");
    let vs = validate_generated(&ds, &["SSH"]);
    let by_id = common::violations_by_id(&vs);
    assert_eq!(by_id.get("BatteryUnit.OK").map_or(0, |v| v.len()), 0,
        "BatteryUnit.OK (storedE=50 < ratedE=100): expected 0 violations, got: {:?}", by_id.get("BatteryUnit.OK"));
    assert_eq!(by_id.get("BatteryUnit.BAD").map_or(0, |v| v.len()), 1,
        "BatteryUnit.BAD (storedE=150 >= ratedE=100): expected 1 violation, got: {:?}", by_id.get("BatteryUnit.BAD"));
}

#[test]
fn shacl_ssh_002_energy_consumer() {
    // EnergyConsumer.p must be >= 0 (sh:minInclusive 0).
    let ds = common::load_dataset("../testdata/test_shacl_SSH_002.xml");
    let vs = validate_generated(&ds, &["SSH"]);
    let by_id = common::violations_by_id(&vs);
    assert_eq!(by_id.get("EnergyConsumer.OK").map_or(0, |v| v.len()), 0,
        "EnergyConsumer.OK (p=100): expected 0 violations, got: {:?}", by_id.get("EnergyConsumer.OK"));
    assert_eq!(by_id.get("EnergyConsumer.BAD").map_or(0, |v| v.len()), 1,
        "EnergyConsumer.BAD (p=-10): expected 1 violation, got: {:?}", by_id.get("EnergyConsumer.BAD"));
}

#[test]
fn shacl_sc_001() {
    // PowerTransformerEnd.phaseAngleClock must be in [0, 11] (sh:maxInclusive 11).
    let ds = common::load_dataset("../testdata/test_shacl_SC_001.xml");
    let vs = validate_generated(&ds, &["SC"]);
    let by_id = common::violations_by_id(&vs);
    assert_eq!(by_id.get("PowerTransformerEnd.OK").map_or(0, |v| v.len()), 0,
        "PowerTransformerEnd.OK (phaseAngleClock=5): expected 0 violations, got: {:?}", by_id.get("PowerTransformerEnd.OK"));
    assert_eq!(by_id.get("PowerTransformerEnd.BAD").map_or(0, |v| v.len()), 1,
        "PowerTransformerEnd.BAD (phaseAngleClock=12): expected 1 violation, got: {:?}", by_id.get("PowerTransformerEnd.BAD"));
}

#[test]
fn shacl_sv_001() {
    // SvVoltage.v must be > 0 (sh:minExclusive 0).
    let ds = common::load_dataset("../testdata/test_shacl_SV_001.xml");
    let vs = validate_generated(&ds, &["SV"]);
    let by_id = common::violations_by_id(&vs);
    assert_eq!(by_id.get("SvVoltage.OK").map_or(0, |v| v.len()), 0,
        "SvVoltage.OK (v=110): expected 0 violations, got: {:?}", by_id.get("SvVoltage.OK"));
    assert_eq!(by_id.get("SvVoltage.BAD").map_or(0, |v| v.len()), 1,
        "SvVoltage.BAD (v=-1): expected 1 violation, got: {:?}", by_id.get("SvVoltage.BAD"));
}

#[test]
fn shacl_dy_001() {
    // AsynchronousMachineTimeConstantReactance.tppo must be < tpo (sh:lessThan).
    let ds = common::load_dataset("../testdata/test_shacl_DY_001.xml");
    let vs = validate_generated(&ds, &["DY"]);
    let by_id = common::violations_by_id(&vs);
    assert_eq!(by_id.get("AsynchronousMachineTimeConstantReactance.OK").map_or(0, |v| v.len()), 0,
        "AMTCR.OK (tppo=0.01 < tpo=0.1): expected 0 violations, got: {:?}", by_id.get("AsynchronousMachineTimeConstantReactance.OK"));
    assert_eq!(by_id.get("AsynchronousMachineTimeConstantReactance.BAD").map_or(0, |v| v.len()), 1,
        "AMTCR.BAD (tppo=0.1 >= tpo=0.05): expected 1 violation, got: {:?}", by_id.get("AsynchronousMachineTimeConstantReactance.BAD"));
}

#[test]
fn shacl_op_001() {
    // AccumulatorLimit.value must be > 0 (sh:minExclusive 0).
    let ds = common::load_dataset("../testdata/test_shacl_OP_001.xml");
    let vs = validate_generated(&ds, &["OP"]);
    let by_id = common::violations_by_id(&vs);
    assert_eq!(by_id.get("AccumulatorLimit.OK").map_or(0, |v| v.len()), 0,
        "AccumulatorLimit.OK (value=5): expected 0 violations, got: {:?}", by_id.get("AccumulatorLimit.OK"));
    assert_eq!(by_id.get("AccumulatorLimit.BAD").map_or(0, |v| v.len()), 1,
        "AccumulatorLimit.BAD (value=-1): expected 1 violation, got: {:?}", by_id.get("AccumulatorLimit.BAD"));
}

#[test]
fn shacl_tp_001() {
    // TopologicalNode.name is required (sh:required).
    let ds = common::load_dataset("../testdata/test_shacl_TP_001.xml");
    let vs = validate_generated(&ds, &["TP"]);
    let by_id = common::violations_by_id(&vs);
    assert_eq!(by_id.get("TopologicalNode.OK").map_or(0, |v| v.len()), 0,
        "TopologicalNode.OK (name present): expected 0 violations, got: {:?}", by_id.get("TopologicalNode.OK"));
    assert_eq!(by_id.get("TopologicalNode.BAD").map_or(0, |v| v.len()), 1,
        "TopologicalNode.BAD (name absent): expected 1 violation, got: {:?}", by_id.get("TopologicalNode.BAD"));
}

#[test]
fn shacl_eqbd_001() {
    // BoundaryPoint.fromEndIsoCode must be a valid European ISO-3166-1-alpha-2 code (sh:in).
    let ds = common::load_dataset("../testdata/test_shacl_EQBD_001.xml");
    let vs = validate_generated(&ds, &["EQBD"]);
    let by_id = common::violations_by_id(&vs);
    assert_eq!(by_id.get("BoundaryPoint.OK").map_or(0, |v| v.len()), 0,
        "BoundaryPoint.OK (fromEndIsoCode=DE): expected 0 violations, got: {:?}", by_id.get("BoundaryPoint.OK"));
    assert_eq!(by_id.get("BoundaryPoint.BAD").map_or(0, |v| v.len()), 1,
        "BoundaryPoint.BAD (fromEndIsoCode=XX): expected 1 violation, got: {:?}", by_id.get("BoundaryPoint.BAD"));
}
