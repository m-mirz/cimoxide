"""The 2026 SV-IOP use cases (report Annex 1), checked on what cimoxide decodes.

Steps and mRIDs come from ReliCapGrid's TestUseCases/*.md. Each check returns
the problems it finds. A test asserts the problems on the real data equal the
known ones in KNOWN (data or documentation issues, each explained), then breaks
the data in memory and asserts the check notices. Steps that need a power flow,
a contingency analysis or an RAO are out of scope; the data they act on is checked.
"""

import copy
import functools
import re
from dataclasses import dataclass, field

import pytest

import cimoxide
import harness

UUID = re.compile(r"^[0-9a-fA-F]{8}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{12}$")
AREAS = ("Belgovia", "Britheim", "DC-Espheim-Svedala", "DC-Nordheim-Galia", "Espheim", "Galia",
         "Nordheim", "Portheim", "Svedala")


class Model:
    """What cimoxide decoded from a set of files, keyed by normalised id."""

    def __init__(self, objects, **facts):
        self.objects = objects
        # What a check needs beyond the objects (a merge's duplicates, ...),
        # kept by the mutations.
        self.facts = facts

    @classmethod
    def load(cls, globs):
        root = harness.relicap_dir() / "Instance"
        paths = sorted({str(p) for g in globs for p in root.glob(g)
                        if "GridSituation" not in str(p) and "Dataset_version_dependency" not in str(p)})
        assert paths, globs
        dataset = cimoxide.CimDataset.decode_files(paths)
        objects = {**dataset.entries(), **dataset.foreign()}
        return cls({harness.norm(k): v for k, v in objects.items()})

    def get(self, mrid):
        return self.objects.get(harness.norm(mrid))

    def type(self, mrid):
        obj = self.get(mrid)
        return harness.bare_type(obj["_type"]) if obj else None

    def values(self, mrid, key):
        obj = self.get(mrid) or {}
        value = obj.get(key)
        return value if isinstance(value, list) else [value] if value is not None else []

    def value(self, mrid, key):
        values = self.values(mrid, key)
        return values[-1] if values else None

    def refs(self, mrid, key):
        return [harness.norm(v) for v in self.values(mrid, key)]

    def ref(self, mrid, key):
        refs = self.refs(mrid, key)
        return refs[-1] if refs else None

    def referrers(self, key, target):
        target = harness.norm(target)
        return sorted(k for k, v in self.objects.items()
                      if target in [harness.norm(x) for x in (v.get(key) if isinstance(v.get(key), list)
                                                               else [v.get(key)]) if x])

    def of_type(self, type_name):
        return sorted(k for k, v in self.objects.items() if harness.bare_type(v["_type"]) == type_name)

    def unresolved(self, mrids):
        """Associations (Class.Upper) from *mrids* to an id-like value not in the model."""
        missing = []
        for mrid in mrids:
            for key, value in (self.get(mrid) or {}).items():
                attr = key.rsplit(".", 1)[-1]
                if "." not in key or not attr[:1].isupper():
                    continue
                for v in value if isinstance(value, list) else [value]:
                    target = harness.norm(v)
                    if UUID.match(target) and target not in self.objects:
                        missing.append(f"{mrid} {key} -> {target}")
        return missing

    # mutations, on a copy
    def without(self, *mrids):
        drop = {harness.norm(m) for m in mrids}
        return Model({k: v for k, v in self.objects.items() if k not in drop}, **self.facts)

    def with_value(self, mrid, key, value):
        objects = copy.copy(self.objects)
        obj = dict(objects[harness.norm(mrid)])
        if value is None:
            obj.pop(key, None)
        else:
            obj[key] = value
        objects[harness.norm(mrid)] = obj
        return Model(objects, **self.facts)


@functools.cache
def model(*globs):
    return Model.load(globs)


def grid(*areas):
    return tuple(f"{a}/Grid/cimxml/*.xml" for a in areas)


def nc(*areas):
    return tuple(f"{a}/NetworkCode/cimxml/*.xml" for a in areas)


COMMON = ("commonData/Grid/cimxml/*.xml", "commonData/NetworkCode/cimxml/*.xml")
BOUNDARY = ("boundaryData/Grid/cimxml/*.xml",)


def expect(problems, condition, message):
    if not condition:
        problems.append(message)


def expect_type(problems, m, mrid, *types):
    actual = m.type(mrid)
    expect(problems, actual in types, f"{mrid}: {actual or 'absent'}, expected {' or '.join(types)}")


# --- PF, BDS, CD, RD ---------------------------------------------------------

def check_pf(m):
    """Power-flow input per IGM: EQ, SSH, TP and SV headers; every TopologicalNode
    in a TopologicalIsland has an SvVoltage (nodes outside every island are
    de-energised and have none in Espheim and Svedala)."""
    problems = []
    profiles = {p.rsplit("/", 2)[-2] for h in m.of_type("FullModel") for p in m.values(h, "Model.profile")}
    for needed in ("CoreEquipment-EU", "SteadyStateHypothesis-EU", "Topology-EU", "StateVariables-EU"):
        expect(problems, needed in profiles, f"no {needed} dataset")
    voltages = {m.ref(v, "SvVoltage.TopologicalNode") for v in m.of_type("SvVoltage")}
    energised = {n for i in m.of_type("TopologicalIsland") for n in m.refs(i, "TopologicalIsland.TopologicalNodes")}
    for node in sorted(energised):
        expect(problems, node in voltages, f"TopologicalNode {node} has no SvVoltage")
    expect(problems, m.of_type("TopologicalIsland"), "no TopologicalIsland")
    return problems


def check_bds(m):
    """The boundary merged with every IGM into the CGM, as ReliCapGrid assembles it
    (group cgmes-CGM-Jotunheim): every BoundaryPoint's ConnectivityNode is reached
    by terminals of two IGMs, and all of them sit on one TopologicalNode of the
    CGM's topology that has a voltage; no object is defined by two datasets, and
    none typed with unrelated classes."""
    problems = []
    voltages = {m.ref(v, "SvVoltage.TopologicalNode") for v in m.of_type("SvVoltage")}
    for bp in sorted(m.of_type("BoundaryPoint"), key=lambda bp: m.value(bp, "IdentifiedObject.name")):
        name = m.value(bp, "IdentifiedObject.name")
        node = m.ref(bp, "CommonResponsibilityPoint.ConnectivityNode")
        expect(problems, node in m.objects, f"{name}: node {node} absent")
        terminals = m.referrers("Terminal.ConnectivityNode", node)
        areas = sorted({m.value(t, "_area") for t in terminals} - {None})
        expect(problems, len(areas) >= 2, f"{name}: reached from {areas} only")
        topological = {m.ref(t, "Terminal.TopologicalNode") for t in terminals} | {
            m.ref(node, "ConnectivityNode.TopologicalNode")}
        placed = topological - {None}
        expect(problems, len(placed) == 1 and None not in topological,
               f"{name}: {len(placed)} TopologicalNode(s) in the CGM, "
               f"{sum(m.ref(t, 'Terminal.TopologicalNode') is None for t in terminals)} of "
               f"{len(terminals)} terminals on none")
        expect(problems, not placed or placed <= voltages, f"{name}: no SvVoltage in the CGM")
    for mrid, files in sorted(m.facts["defined_by"].items()):
        expect(problems, len(files) == 1, f"{m.type(mrid)} {mrid} defined by {', '.join(sorted(files))}")
    for mrid, types in m.facts["type_conflicts"]:
        problems.append(f"{mrid} typed {' and '.join(types)}")
    return problems


def bds_model():
    """ReliCapGrid's CGM (group cgmes-CGM-Jotunheim: the boundary, every IGM's EQ and
    updated SSH, Jotunheim's CGM topology and state, the common data) merged by
    cimoxide; each terminal tagged with the IGM whose EQ defines it, and each object
    with the datasets that define it."""
    root = harness.relicap_dir()
    files = harness.group("cgmes-CGM-Jotunheim").files
    cgm = cimoxide.CimDataset.decode_files([str(root / f.path) for f in files])
    objects = {harness.norm(k): v for k, v in cgm.entries().items()}
    defined_by = {}
    for f in files:
        dataset = cimoxide.CimDataset.decode_file(str(root / f.path))
        for mrid in dataset.defined_mrids():
            defined_by.setdefault(harness.norm(mrid), []).append(f.path.rsplit("/", 1)[-1])
            if "_EQ_" in f.path and objects[harness.norm(mrid)]["_type"] == "Terminal":
                objects[harness.norm(mrid)] = dict(objects[harness.norm(mrid)], _area=f.area)
    return Model(objects, defined_by=defined_by,
                 type_conflicts=[(harness.norm(k), t) for k, t in cgm.type_conflicts()])


CD_CLASSES = ("BiddingZone", "BiddingZoneBorder", "CapacityCalculationRegion",
              "CoordinatedCapacityCalculator", "Organisation", "TransmissionSystemOperator",
              "SecurityCoordinator", "LoadFrequencyControlArea", "SynchronousArea", "SchedulingArea",
              "LoadFrequencyControlBlock", "TieCorridor")


def check_cd(m):
    """The classes CD_UC1 lists are present (the report's "CapacityCalculationCalculator"
    is CoordinatedCapacityCalculator); common data is defined once: no other dataset
    defines a common data object again; and its associations resolve."""
    problems = [f"no {c}" for c in CD_CLASSES if not m.of_type(c)]
    for mrid, files in sorted(m.facts["defined_by"].items()):
        common = [f for f in files if f in m.facts["common_files"]]
        others = sorted(set(files) - set(common))
        if common and others:
            problems.append(f"{m.type(mrid)} {mrid} of {', '.join(common)} also defined by {', '.join(others)}")
    return problems + m.unresolved(sorted(m.facts["common"]))


def cd_model():
    """Every dataset merged, each object tagged with the datasets that define it, and
    which of them are common data."""
    root = harness.relicap_dir() / "Instance"
    globs = COMMON + BOUNDARY + ALL_GRID + ALL_NC
    paths = sorted({p for g in globs for p in root.glob(g)
                    if "GridSituation" not in str(p) and "Dataset_version_dependency" not in str(p)})
    common_files = {p.name for g in COMMON for p in root.glob(g)}
    defined_by = {}
    for path in paths:
        for mrid in cimoxide.CimDataset.decode_file(str(path)).defined_mrids():
            defined_by.setdefault(harness.norm(mrid), []).append(path.name)
    return Model(model(*globs).objects, defined_by=defined_by, common_files=common_files,
                 common={k for k, fs in defined_by.items() if set(fs) & common_files})


RD_KEYS = ("publisher", "isVersionOf", "spatial", "wasGeneratedBy", "license", "accessRights")


def check_rd(m):
    """Every NC header's reference-data URI is defined in referenceData: per header
    property, how many of the URIs the headers name are not."""
    defined = {v["id"] for v in m.objects.values()} | set(m.objects)
    problems = []
    for key in RD_KEYS:
        uris = [uri for header in m.of_type("Dataset") for uri in m.values(header, key)]
        missing = [uri for uri in uris if uri not in defined]
        expect(problems, not missing, f"{key}: {len(missing)} of {len(uris)} not in reference data")
    return problems


# --- CO, ER ------------------------------------------------------------------

# Contingencies_use_cases.md: contingency -> equipment, and the equipment's class
CONTINGENCIES = [
    ("7e31c67d-67ba-4592-8ac1-9e806d697c8e", "d9622e7f-5bf0-4e7e-b766-b8596c6fe4ae", "ACLineSegment"),
    ("e9eab3fe-c328-4f78-9bc1-77adb59f6ba7", "ed0c5d75-4a54-43c8-b782-b20d7431630b", "ACLineSegment"),
    ("8cdec4c6-10c3-40c1-9eeb-7f6ae8d9b3fe", "b85e5fb8-7e2b-4264-a059-edab9a838116", "ACLineSegment"),
    ("e05bbe20-9d4a-40da-9777-8424d216785d", "f1c13f90-6d89-4a37-a51c-94742ad2dd72", "PowerTransformer"),
    ("62eac668-82a1-4739-8cfc-de929b78ef7e", "0489d903-c766-11e1-8775-005056c00008", "LinearShuntCompensator"),
    ("ab92defa-ef8e-4d13-a242-5b1a9fe956f6", "8464a151-ec39-4e7a-8321-06c41e63958c", "NonlinearShuntCompensator"),
    ("66627b36-aecd-4141-ab99-4bd8a74d18a8", "038ce404-30dc-4289-b9db-7076cb870b8e", "CsConverter"),
    ("504b48fb-de1f-4f7e-928f-8585daaf7b72", "be09fc02-de3f-49e4-aa84-94803bcc5d76", "DCLineSegment"),
    ("37997e71-cb7d-4a8c-baa6-2a1594956da9", "550ebe0d-f2b2-48c1-991f-cebea43a21aa", "SynchronousMachine"),
    ("f3a8c241-7b5e-4d92-b1c6-8e3f092a4d71", "ebf0cd8b-a357-4e97-aa67-de5b263733d6", "ConformLoad"),
    ("9d17b84c-33b5-4a68-b8b9-ed5b31038d40", "04566cf8-c766-11e1-8775-005056c00008", "ACLineSegment"),
    ("13334fdf-9cc2-4341-adb6-1281269040b4", "0475dbd8-c766-11e1-8775-005056c00008", "ACLineSegment"),
]


def check_co(m):
    problems = []
    for contingency, equipment, cls in CONTINGENCIES:
        expect_type(problems, m, contingency, "OrdinaryContingency", "OutOfRangeContingency",
                    "ExceptionalContingency")
        linked = {m.ref(ce, "ContingencyEquipment.Equipment")
                  for ce in m.referrers("ContingencyElement.Contingency", contingency)}
        expect(problems, equipment in linked, f"{contingency} does not name {equipment}")
        expect_type(problems, m, equipment, cls)
    return problems


def check_er(m):
    """Every association from an ER dataset's objects resolves."""
    er = [k for k in m.objects if m.get(k).get("_file_kind") == "ER"]
    return m.unresolved(er)


def er_model(area):
    """The area's grid, NC and common data, ER objects tagged."""
    objects = dict(model(*grid(area), *nc(area), *COMMON, *BOUNDARY).objects)
    for mrid in model(f"{area}/NetworkCode/cimxml/*_ER.xml").objects:
        objects[mrid] = dict(objects[mrid], _file_kind="ER")
    return Model(objects)


# --- RA ----------------------------------------------------------------------

def topology_actions(m, remedial_action):
    return [ga for ga in m.referrers("GridStateAlteration.GridStateAlterationRemedialAction", remedial_action)
            if m.type(ga) == "TopologyAction"]


def applied(m, remedial_action):
    """The switch states applying *remedial_action* sets: each TopologyAction's switch,
    open when its StaticPropertyRange on Switch.open gives an absolute value other than 0.
    None for a switch whose action gives no such value."""
    states = {}
    for action in topology_actions(m, remedial_action):
        values = [m.value(r, "RangeConstraint.normalValue")
                  for r in m.referrers("RangeConstraint.GridStateAlteration", action)
                  if m.value(r, "StaticPropertyRange.PropertyReference") == "Switch.open"
                  and m.value(r, "RangeConstraint.valueKind") == "ValueOffsetKind.absolute"]
        states[m.ref(action, "TopologyAction.Switch")] = (float(values[-1]) != 0) if values else None
    return states


def check_ra1(m):
    """The remedial action has one topology action on the switch, which is closed in SSH;
    applying it opens the switch."""
    ra, switch = "6d2c8901-d068-4d22-8ce9-7379644b4f17", "176d262c-701c-4ced-99b2-a155c136e787"
    problems = []
    expect_type(problems, m, ra, "GridStateAlterationRemedialAction")
    actions = topology_actions(m, ra)
    expect(problems, len(actions) == 1, f"{ra}: {len(actions)} topology actions, expected 1")
    expect(problems, [m.ref(a, "TopologyAction.Switch") for a in actions] == [switch],
           f"{ra}: switches {[m.ref(a, 'TopologyAction.Switch') for a in actions]}, expected {switch}")
    expect(problems, m.value(switch, "Switch.open") == "false", f"switch {switch} not closed in SSH")
    expect(problems, applied(m, ra).get(switch) is True,
           f"applying {ra}: switch {switch} open = {applied(m, ra).get(switch)}, expected True")
    return problems


def check_ra2(m):
    """Two exclusive dependencies, each on a remedial action with several topology
    actions; the two act on the same four switches and, applied, set different
    configurations."""
    problems, configurations, actions_of = [], [], []
    for dependency in ("598ab84e-a575-40ad-bf66-c1ab41c65093", "13c29cc3-3d99-4c5b-abac-085f0d319eab"):
        expect(problems, m.value(dependency, "RemedialActionDependency.kind")
               == "RemedialActionDependencyKind.exclusive", f"{dependency} not exclusive")
        ra = m.ref(dependency, "RemedialActionDependency.RemedialAction")
        expect_type(problems, m, ra, "GridStateAlterationRemedialAction")
        actions = topology_actions(m, ra)
        expect(problems, len(actions) > 1, f"{ra}: {len(actions)} topology actions, expected several")
        actions_of.append(ra)
        configurations.append(applied(m, ra))
    switch_sets = [set(c) for c in configurations]
    expect(problems, len(switch_sets) == 2 and switch_sets[0] == switch_sets[1] and len(switch_sets[0]) == 4,
           f"switch sets {[sorted(s) for s in switch_sets]}, expected the same four")
    expect(problems, len(set(actions_of)) == 2, f"both dependencies on {actions_of[0]}")
    expect(problems, len(configurations) == 2 and configurations[0] != configurations[1],
           "both remedial actions set the same configuration")
    return problems


def check_ra3(m):
    ra, action, schedule = ("d856a2a2-3de4-4a7b-aea4-d363c13d9014", "4d3757fd-b40a-4d7b-be47-6553935234ff",
                            "f5205bd4-637a-4eb7-b0d8-e03638bc570b")
    problems = []
    expect_type(problems, m, ra, "GridStateAlterationRemedialAction")
    expect(problems, m.ref(action, "GridStateAlteration.GridStateAlterationRemedialAction") == ra,
           f"{action} not in {ra}")
    expect(problems, m.ref(schedule, "GridStateAlterationSchedule.GridStateAlteration") == action,
           f"{schedule} not on {action}")
    points = m.referrers("GridStateAlterationTimePoint.GridStateAlterationSchedule", schedule)
    states = {m.value(p, "GridStateAlterationTimePoint.enabled") for p in points}
    expect(problems, len(points) > 1 and states == {"true", "false"},
           f"{schedule}: {len(points)} time points, enabled {sorted(states - {None})}")
    # SSI: the action described with its state, without a schedule.
    expect(problems, m.value(action, "GridStateAlteration.enabled") in ("true", "false"),
           f"{action}: no GridStateAlteration.enabled from SSI")
    return problems


def check_ra4(m):
    response, dependency, proposal = ("96f93b71-420f-456f-932e-9ac3f724b683",
                                      "f7231aa5-d497-41f3-a5d5-9d4af74444f0",
                                      "51361821-0a40-4e5a-a49d-29c266ea2ecc")
    problems = []
    expect(problems, m.value(response, "RemedialActionScheduleResponse.kind")
           == "RemedialActionScheduleResponseKind.refused", f"{response} does not refuse")
    refused = m.ref(response, "RemedialActionScheduleResponse.RemedialActionSchedule")
    expect_type(problems, m, refused, "RemedialActionSchedule")
    expect(problems, m.ref(dependency, "RemedialActionScheduleDependency.ReplacedRemedialActionSchedule")
           == refused, f"{dependency} does not replace the refused {refused}")
    expect(problems, m.ref(dependency, "RemedialActionScheduleDependency.ProposedRemedialActionSchedule")
           == proposal, f"{dependency} does not propose {proposal}")
    expect(problems, m.value(dependency, "RemedialActionScheduleDependency.kind")
           == "RemedialActionScheduleDependencyKind.none",
           f"{dependency}: kind {m.value(dependency, 'RemedialActionScheduleDependency.kind')}, expected none")
    expect_type(problems, m, proposal, "RemedialActionSchedule")
    new_events = m.referrers("EventSchedule.RemedialActionSchedule", proposal)
    old_events = m.referrers("EventSchedule.RemedialActionSchedule", refused)
    expect(problems, new_events and not set(new_events) & set(old_events),
           f"proposal's EventSchedules {new_events}, refused schedule's {old_events}")
    return problems


def check_ra5(m):
    group = "6a9b5221-bb3f-421b-89c1-886b100aa68a"
    pairs = [("1c1e39cf-15a1-4874-9739-d9e6dc90ee11", "279bfb00-1b2a-4592-af3a-dba9403d1abd"),
             ("023512a8-150d-4300-8159-807275eafd48", "87e90c47-43d8-4e5c-b2ad-5c67d3a74842")]
    problems, activated = [], {}
    for ra in ("7e422768-207d-455f-976e-0b1cb2338509", "fa3e3533-345a-4ef1-a46d-3ab6d251924a"):
        expect_type(problems, m, ra, "GridStateAlterationRemedialAction")
        dependencies = [d for d in m.referrers("RemedialActionDependency.RemedialAction", ra)
                        if m.ref(d, "RemedialActionDependency.DependingRemedialActionGroup") == group]
        expect(problems, any(m.value(d, "RemedialActionDependency.kind")
                             == "RemedialActionDependencyKind.exclusive" for d in dependencies),
               f"{ra}: no exclusive dependency in group {group}")
    for event_schedule, schedule in pairs:
        expect(problems, m.ref(event_schedule, "EventSchedule.RemedialActionSchedule") == schedule,
               f"{event_schedule} not on {schedule}")
        points = m.referrers("EventTimePoint.EventSchedule", event_schedule)
        expect(problems, points, f"{event_schedule}: no EventTimePoint")
        for p in points:
            if m.value(p, "EventTimePoint.isActive") == "true":
                activated.setdefault(m.value(p, "EventTimePoint.atTime"), set()).add(schedule)
    both = sorted(t for t, s in activated.items() if len(s) > 1)
    expect(problems, not both, f"both activated at {both}")
    return problems


def check_ra6(m):
    ra, power_schedule, unit, bids = ("30f6377d-5e35-4dd0-a53d-c1992043855e",
                                      "27940387-6935-47b6-ab25-8742bac2a266",
                                      "413bbc8a-683c-7e07-4fb9-aa6ec83278e0",
                                      "ae6a5ddd-fe7e-40b2-bcb6-cbd2882434e7")
    problems = []
    expect_type(problems, m, ra, "CountertradeRemedialAction")
    expect_type(problems, m, unit, "GeneratingUnit")
    expect(problems, m.ref(power_schedule, "PowerSchedule.PowerRemedialAction") == ra,
           f"{power_schedule} not on {ra}")
    expect(problems, m.ref(power_schedule, "PowerSchedule.GeneratingUnit") == unit,
           f"{power_schedule} not on {unit}")
    expect(problems, m.ref(bids, "PowerBidSchedule.PowerRemedialAction") == ra, f"{bids} not on {ra}")
    problems += complete_points(m, power_schedule, "PowerTimePoint", "PowerSchedule", ("activatedP",))
    problems += complete_points(m, bids, "PowerBidScheduleTimePoint", "PowerBidSchedule", ("p", "price"))
    return problems


def complete_points(m, schedule, point_class, schedule_key, values):
    """The time points of *schedule* exist, each with a distinct atTime and *values*."""
    points = m.referrers(f"{point_class}.{schedule_key}", schedule)
    problems = [] if points else [f"{schedule}: no {point_class}"]
    times = [m.value(p, f"{point_class}.atTime") for p in points]
    expect(problems, None not in times and len(set(times)) == len(times), f"{schedule}: times {sorted(map(str, times))}")
    for p in points:
        for value in values:
            expect(problems, m.value(p, f"{point_class}.{value}") is not None, f"{p}: no {value}")
    return problems


def check_ra7(m):
    machine = "3a3b27be-b18b-4385-b557-6735d733baf0"
    bids, point, shift_schedule, distribution = ("6c0ba4c5-5ac0-43f8-87d9-ec2fb41a3dec",
                                                 "250e37a5-50e0-44b7-9bd7-7284c04116db",
                                                 "3b5748de-39f2-45d7-9a66-65819c667e91",
                                                 "7ab20445-576c-4162-aad5-d097d894a8e0")
    variant1, variant2 = "fa73cfee-ff54-43cd-816b-51394cd76f0f", "9d72d12a-f135-437a-83fa-ef3e5b23f41f"
    problems = []
    expect_type(problems, m, machine, "SynchronousMachine")
    for ra in (variant1, variant2):
        expect_type(problems, m, ra, "RedispatchRemedialAction")
    expect(problems, m.ref(bids, "PowerBidSchedule.PowerRemedialAction") == variant1, f"{bids} not on {variant1}")
    expect(problems, m.ref(point, "PowerBidScheduleTimePoint.PowerBidSchedule") == bids, f"{point} not on {bids}")
    expect(problems, m.ref(distribution, "PowerShiftKeyDistribution.PowerShiftKeySchedule") == shift_schedule,
           f"{distribution} not on {shift_schedule}")
    unit = m.ref(machine, "RotatingMachine.GeneratingUnit")
    expect(problems, m.ref(shift_schedule, "PowerShiftKeySchedule.GeneratingUnit") == unit,
           f"{shift_schedule} not on the machine's unit {unit}")
    distribution_ra = m.ref(m.ref(distribution, "PowerShiftKeyDistribution.PowerBidSchedule"),
                            "PowerBidSchedule.PowerRemedialAction")
    expect(problems, distribution_ra == variant2,
           f"variant 2: {distribution} reaches {distribution_ra}, expected {variant2}")
    problems += complete_points(m, bids, "PowerBidScheduleTimePoint", "PowerBidSchedule", ("p", "price"))
    return problems


# --- AS, AE ------------------------------------------------------------------

def check_as(m):
    """The availability remedial action has the schedule, which is cancelled; the
    resulting window is the schedule's AvailabilityTimePoints."""
    ra, schedule = "14b2b671-e92b-40e6-abf8-ad37811b33c7", "d394295f-9d18-4d9f-9808-bd23282bf60f"
    problems = []
    expect_type(problems, m, ra, "AvailabilityRemedialAction")
    expect_type(problems, m, schedule, "AvailabilitySchedule")
    expect(problems, m.ref(schedule, "AvailabilitySchedule.RemedialAction") == ra, f"{schedule} not on {ra}")
    expect(problems, m.value(schedule, "AvailabilitySchedule.cancelledDateTime"),
           f"{schedule}: neither cancelled, shortened nor rescheduled")
    points = m.referrers("AvailabilityTimePoint.AvailabilitySchedule", schedule)
    expect(problems, points, f"{schedule}: no AvailabilityTimePoint, so no window to cancel")
    return problems


REGION = "56d81c91-4fb8-47a2-9d7c-baa13dedd605"


def check_ae1(m):
    ae = "992c2de6-e206-45b3-a76a-f4a691e8839a"
    problems = []
    expect(problems, m.value(ae, "AssessedElement.inBaseCase") == "true", f"{ae} not in base case")
    expect(problems, m.ref(ae, "AssessedElement.SecuredForRegion") == REGION, f"{ae} not secured for {REGION}")
    return problems


def check_ae2(m):
    ae, scanned_only = "1eb2eb03-dda6-4e59-b7c8-a2edb117d676", "d463cbba-c89c-4199-bbb9-1a33d90cae2c"
    problems = []
    expect(problems, m.value(ae, "AssessedElement.inBaseCase") == "false", f"{ae} in base case")
    expect(problems, m.ref(ae, "AssessedElement.ScannedForRegion"), f"{ae} not scanned")
    expect(problems, m.ref(ae, "AssessedElement.SecuredForRegion"), f"{ae} not secured")
    expect(problems, m.value(ae, "AssessedElement.exclusionReason"), f"{ae} has no exclusionReason")
    expect(problems, m.ref(scanned_only, "AssessedElement.ScannedForRegion"), f"{scanned_only} not scanned")
    expect(problems, not m.ref(scanned_only, "AssessedElement.SecuredForRegion"), f"{scanned_only} secured")
    return problems


def check_ae3(m):
    schedule, point = "7221ae31-f736-4ae2-8ce1-f3dd605cefd6", "a26e3ae0-0a7d-4f42-ad64-e9105ec3cd41"
    problems = []
    expect(problems, m.ref(schedule, "AssessedElementSchedule.AssessedElement")
           == "992c2de6-e206-45b3-a76a-f4a691e8839a", f"{schedule} not on the secured AE")
    expect(problems, m.ref(point, "AssessedElementTimePoint.AssessedElementSchedule") == schedule,
           f"{point} not on {schedule}")
    expect(problems, m.value(point, "AssessedElementTimePoint.enabled") == "false", f"{point} does not disable")
    expect(problems, m.value(point, "AssessedElementTimePoint.atTime"), f"{point}: no atTime")
    return problems


def check_ae4(m):
    ae = "1eb2eb03-dda6-4e59-b7c8-a2edb117d676"
    return [] if m.ref(ae, "AssessedElement.ScannedForRegion") else [f"{ae} not excluded via ScannedForRegion"]


def check_ae5(m):
    problems = []
    full = "d17943b5-9d10-4f8b-bb07-18c3a3822348"
    expect(problems, m.value(full, "AssessedElement.inBaseCase") == "true", f"{full} not in base case")
    expect(problems, m.value(full, "AssessedElement.isCombinableWithContingency") == "true",
           f"{full} not combinable")
    for ae, link, kind in (("992c2de6-e206-45b3-a76a-f4a691e8839a", "1f38d403-a822-4c24-93c0-0f18ac699ef1",
                            "excluded"),
                           ("663c6c3c-1777-4eb7-98b0-c23233ce0f71", "deb4c65f-c3fc-414e-baa3-692829e3eec2",
                            "included")):
        expect(problems, m.ref(link, "AssessedElementWithContingency.AssessedElement") == ae, f"{link} not on {ae}")
        expect(problems, m.value(link, "AssessedElementWithContingency.combinationConstraintKind")
               == f"ElementCombinationConstraintKind.{kind}", f"{link} not {kind}")
        expect(problems, m.ref(link, "AssessedElementWithContingency.Contingency") in m.objects,
               f"{link}: contingency absent")
    expect(problems, m.value("663c6c3c-1777-4eb7-98b0-c23233ce0f71", "AssessedElement.isCombinableWithContingency")
           == "false", "663c6c3c combinable")
    return problems


def check_ae6(m):
    ae, link, ra = ("13d17257-977e-43bc-971a-25022282688a", "66f786ce-5e2a-427d-9eb8-4e1462045c27",
                    "70b696ac-c38f-4528-8d65-ba707c0b72e1")
    problems = []
    expect(problems, m.ref(link, "AssessedElementWithRemedialAction.AssessedElement") == ae, f"{link} not on {ae}")
    expect(problems, m.ref(link, "AssessedElementWithRemedialAction.RemedialAction") == ra, f"{link} not on {ra}")
    expect(problems, m.value(link, "AssessedElementWithRemedialAction.combinationConstraintKind")
           in ("ElementCombinationConstraintKind.included", "ElementCombinationConstraintKind.excluded",
               "ElementCombinationConstraintKind.considered"), f"{link}: no inclusion kind")
    expect_type(problems, m, ra, "GridStateAlterationRemedialAction")
    return problems


# --- OR, SAR, SIPS -----------------------------------------------------------

def check_resolves(type_names):
    def check(m):
        objects = [k for t in type_names for k in m.of_type(t)]
        return ([f"no {' / '.join(type_names)}"] if not objects else []) + m.unresolved(objects)
    return check


SIPS = ("SchemeRemedialAction", "StageTrigger", "Stage", "GateInputPin", "PinTerminal",
        "PinEquipmentTripping", "Gate", "RemedialActionScheme", "TriggerCondition")


# --- the table ---------------------------------------------------------------

@dataclass
class UseCase:
    name: str
    globs: tuple
    check: object
    mutate: object                       # Model -> Model the check must reject
    known: list = field(default_factory=list)   # problems in the data or the docs, explained in KNOWN
    load: object = None                  # () -> Model, instead of globs


def svedala():
    return grid("Svedala") + nc("Svedala") + COMMON


def belgovia():
    return grid("Belgovia") + nc("Belgovia") + COMMON


def espheim():
    return grid("Espheim") + nc("Espheim") + COMMON


ALL_NC = tuple(f"{a}/NetworkCode/cimxml/*.xml" for a in AREAS + ("Jotunheim",)) + (
    "Jotunheim/NetworkCode/*.xml",)
ALL_GRID = tuple(f"{a}/Grid/cimxml/*.xml" for a in AREAS)

USE_CASES = [
    *[UseCase(f"PF_UC1[{a}]", grid(a), check_pf, lambda m: m.without(*m.of_type("SvVoltage")[:1]))
      for a in AREAS],
    UseCase("BDS_UC1", (), check_bds,
            lambda m: m.without(*[k for k, v in m.objects.items() if v.get("_area") == "Belgovia"]),
            load=bds_model),
    UseCase("CD_UC1", (), check_cd, lambda m: m.without(*m.of_type("TieCorridor")), load=cd_model),
    UseCase("RD_UC1", ALL_NC + ("referenceData/NetworkCode/cimxml/*.xml",), check_rd,
            lambda m: m.without(m.value(m.of_type("Dataset")[0], "accessRights"))),
    UseCase("CO_UC1", ALL_NC + ALL_GRID, check_co,
            lambda m: m.without(*m.referrers("ContingencyElement.Contingency", CONTINGENCIES[0][0]))),
    *[UseCase(f"ER_UC1[{a}]", (), check_er,
              lambda m: m.without(*m.of_type("Terminal"), *m.of_type("Substation")),
              load=functools.partial(er_model, a))
      for a in ("Belgovia", "Britheim", "Espheim", "Galia", "Svedala")],
    UseCase("RA_UC1", svedala(), check_ra1,
            lambda m: m.with_value("176d262c-701c-4ced-99b2-a155c136e787", "Switch.open", "true")),
    UseCase("RA_UC2", svedala(), check_ra2,
            lambda m: m.with_value("598ab84e-a575-40ad-bf66-c1ab41c65093", "RemedialActionDependency.kind",
                                   "RemedialActionDependencyKind.inclusive")),
    UseCase("RA_UC3", espheim(), check_ra3,
            lambda m: m.without(*m.referrers("GridStateAlterationTimePoint.GridStateAlterationSchedule",
                                             "f5205bd4-637a-4eb7-b0d8-e03638bc570b"))),
    UseCase("RA_UC4", svedala(), check_ra4,
            lambda m: m.with_value("96f93b71-420f-456f-932e-9ac3f724b683", "RemedialActionScheduleResponse.kind",
                                   "RemedialActionScheduleResponseKind.accepted")),
    UseCase("RA_UC5", svedala(), check_ra5,
            lambda m: m.without(*m.referrers("EventTimePoint.EventSchedule",
                                             "1c1e39cf-15a1-4874-9739-d9e6dc90ee11"))),
    UseCase("RA_UC6", belgovia(), check_ra6, lambda m: m.without("27940387-6935-47b6-ab25-8742bac2a266")),
    UseCase("RA_UC7", belgovia(), check_ra7, lambda m: m.without("7ab20445-576c-4162-aad5-d097d894a8e0")),
    UseCase("AS_UC1", svedala(), check_as,
            lambda m: m.with_value("d394295f-9d18-4d9f-9808-bd23282bf60f",
                                   "AvailabilitySchedule.cancelledDateTime", None)),
    UseCase("AE_UC1", belgovia(), check_ae1,
            lambda m: m.with_value("992c2de6-e206-45b3-a76a-f4a691e8839a", "AssessedElement.inBaseCase", "false")),
    UseCase("AE_UC2", belgovia(), check_ae2,
            lambda m: m.with_value("1eb2eb03-dda6-4e59-b7c8-a2edb117d676", "AssessedElement.exclusionReason", None)),
    UseCase("AE_UC3", belgovia(), check_ae3,
            lambda m: m.with_value("a26e3ae0-0a7d-4f42-ad64-e9105ec3cd41", "AssessedElementTimePoint.enabled",
                                   "true")),
    UseCase("AE_UC4", belgovia(), check_ae4,
            lambda m: m.with_value("1eb2eb03-dda6-4e59-b7c8-a2edb117d676", "AssessedElement.ScannedForRegion",
                                   None)),
    UseCase("AE_UC5", ALL_NC + COMMON, check_ae5,
            lambda m: m.with_value("1f38d403-a822-4c24-93c0-0f18ac699ef1",
                                   "AssessedElementWithContingency.combinationConstraintKind",
                                   "ElementCombinationConstraintKind.included")),
    UseCase("AE_UC6", belgovia(), check_ae6, lambda m: m.without("70b696ac-c38f-4528-8d65-ba707c0b72e1")),
    UseCase("OR_UC1", espheim() + COMMON, check_resolves(("Designation", "DesignationType",
                                                           "DesignationAuthority")),
            lambda m: m.without(*m.of_type("DesignationType"))),
    UseCase("SAR_UC1", ALL_NC + ALL_GRID + COMMON + ("Jotunheim/Grid/cimxml/*.xml",),
            check_resolves(("BaseCasePowerFlowResult", "ContingencyPowerFlowResult")),
            lambda m: m.without(*m.of_type("OrdinaryContingency"), *m.of_type("ExceptionalContingency"),
                                *m.of_type("OutOfRangeContingency"))),
    UseCase("SIPS[Belgovia]", belgovia() + BOUNDARY, check_resolves(SIPS),
            lambda m: m.without(*m.of_type("Terminal"))),
    UseCase("SIPS[Svedala]", svedala() + BOUNDARY, check_resolves(SIPS),
            lambda m: m.without(*m.of_type("Terminal"))),
]

# Problems the data or the TestUseCases docs have, by use case: (why, problems).
KNOWN = {
    "CD_UC1": (
        "Common data is not defined once: Belgovia_ER.xml defines the common data's SchedulingArea and "
        "BiddingZone again, Nordheim's EQ its GeographicalRegion.",
        ["SchedulingArea 297f533e-77a7-42d9-a683-32babcf6c82f of Org-NineRealms_CD.xml also defined by "
         "Belgovia_ER.xml",
         "BiddingZone 87fb9f56-fd86-446f-b35d-c0abba19925f of Org-NineRealms_CD.xml also defined by "
         "Belgovia_ER.xml",
         "GeographicalRegion 9356077b-f716-0d6c-0cac-32c074bd8fa3 of Grid_CommonData_CGM-CD.xml also defined "
         "by 20220615T2230Z__Nordheim_EQ_1.xml"]),
    "RA_UC1": (
        "The description has the remedial action open the switch; its one GridStateAlteration sets "
        "Switch.open through StaticPropertyRange caaa9307 to the absolute value 0.0, not open, and the "
        "switch is already closed: applying it changes nothing.",
        ["applying 6d2c8901-d068-4d22-8ce9-7379644b4f17: switch 176d262c-701c-4ced-99b2-a155c136e787 "
         "open = False, expected True"]),
    "AS_UC1": (
        "The availability schedule has no AvailabilityTimePoint: the outage window exists only as the "
        "text 'Maintenance performed from 10:00h to 18:00h', so the resulting window cannot be verified.",
        ["d394295f-9d18-4d9f-9808-bd23282bf60f: no AvailabilityTimePoint, so no window to cancel"]),
    "RD_UC1": (
        "The headers name reference data the referenceData files do not define: licences at "
        "creativecommons.org (they define .../License/...), frames as Test/Frame/X-Power-Transmission-"
        "System (test/frame/X-Transmission), activities as Activity/CGM-IAM (activity/AC-IAM), no "
        "model of the model collection, and parties RCC-Jotunheim, Britheim and www.entsoe.eu.",
        ["publisher: 37 of 71 not in reference data",
         "isVersionOf: 71 of 71 not in reference data",
         "spatial: 41 of 41 not in reference data",
         "wasGeneratedBy: 65 of 65 not in reference data",
         "license: 71 of 71 not in reference data"]),
    "BDS_UC1": (
        "In ReliCapGrid's CGM: Madgar1-3/Lysaborg1-3 (Boundary_Border-Espheim-Portheim.xml) have no "
        "Portheim terminal, and Madgar4-Leyva1 (Boundary_Border-Svedala-Espheim.xml) only the HVDC IGM's; "
        "the CGM's topology leaves Portheim out (1 of its 121 terminals on a TopologicalNode), so BP "
        "Portheim-Espheim is joined only on Espheim's side; and the boundary's containers and one "
        "ConnectivityNode are defined again, identically, by Espheim's, Portheim's and Nordheim's EQ, "
        "one GeographicalRegion by Nordheim's EQ and the common data.",
        ["BP Portheim-Espheim: 1 TopologicalNode(s) in the CGM, 9 of 11 terminals on none",
         "Madgar1-Lysaborg1: reached from ['Espheim'] only",
         "Madgar2-Lysaborg2: reached from ['Espheim'] only",
         "Madgar3-Lysaborg3: reached from ['Espheim'] only",
         "Madgar4-Leyva1: reached from ['DC-Espheim-Svedala'] only",
         "VoltageLevel 0c4f41f8-9797-44eb-bdbe-1a28efa083b9 defined by 20220615T2230Z__Espheim_EQ_1.xml, "
         "20241223T0642Z_2D_Portheim_EQ_1.xml, Boundary_Border-Espheim-Portheim.xml",
         "Substation 42b7f03a-db7a-415a-9e15-25f634f5ea9c defined by 20220615T2230Z__Espheim_EQ_1.xml, "
         "20241223T0642Z_2D_Portheim_EQ_1.xml, Boundary_Border-Espheim-Portheim.xml",
         "SubGeographicalRegion 77467625-a8fe-8792-7cf2-06c5dfd5fba9 defined by 20220615T2230Z__Nordheim_EQ_1.xml, "
         "Boundary_Border-Galia-Nordheim.xml",
         "ConnectivityNode 892ef502-162b-469f-b93f-266aae828227 defined by 20220615T2230Z__Nordheim_EQ_1.xml, "
         "Boundary_Border-Galia-Nordheim.xml",
         "GeographicalRegion 9356077b-f716-0d6c-0cac-32c074bd8fa3 defined by 20220615T2230Z__Nordheim_EQ_1.xml, "
         "Grid_CommonData_CGM-CD.xml",
         "Substation c39c3eb8-9cce-4778-9069-6952a117ba2b defined by 20220615T2230Z__Nordheim_EQ_1.xml, "
         "Boundary_Border-Galia-Nordheim.xml",
         "VoltageLevel e61f8ecf-8129-4702-a3a2-826bdf5ad13d defined by 20220615T2230Z__Nordheim_EQ_1.xml, "
         "Boundary_Border-Galia-Nordheim.xml",
         "SubGeographicalRegion f69de45f-9681-4147-9d1f-9e0336fc8635 defined by 20220615T2230Z__Espheim_EQ_1.xml, "
         "20241223T0642Z_2D_Portheim_EQ_1.xml, Boundary_Border-Espheim-Portheim.xml"]),
    "RA_UC2": (
        "The docs describe two remedial actions on the same four switches of Substation A; the data's "
        "are 'RA SubstationB_2N' and 'RA SubstationA_quarterbar1B', four different switches each.",
        None),
    "RA_UC4": (
        "The docs set RemedialActionScheduleDependencyKind none; the dependency carries no kind, as the "
        "APL requires outside a RemedialActionScheduleGroup (rasc:...kind-applicability): the docs "
        "contradict the APL, not the data (test_edits).",
        ["f7231aa5-d497-41f3-a5d5-9d4af74444f0: kind None, expected none"]),
    "RA_UC5": (
        "The two remedial actions are exclusive, but both proposed schedules are active at the same "
        "three time points.",
        ["both activated at ['2024-11-20T00:30:00Z', '2024-11-20T01:30:00Z', '2024-11-20T02:30:00Z']"]),
    "RA_UC7": (
        "Variant 2's RedispatchRemedialAction 9d72d12a is not linked to the power shift key objects: "
        "PowerShiftKeyDistribution 7ab20445 goes through PowerBidSchedule 6c0ba4c5 to variant 1's fa73cfee.",
        ["variant 2: 7ab20445-576c-4162-aad5-d097d894a8e0 reaches fa73cfee-ff54-43cd-816b-51394cd76f0f, "
         "expected 9d72d12a-f135-437a-83fa-ef3e5b23f41f"]),
}


def load(case):
    return case.load() if case.load else model(*case.globs)


def known(case):
    _why, problems = KNOWN.get(case.name, ("", []))
    return problems


@pytest.mark.parametrize("case", USE_CASES, ids=lambda c: c.name)
def test_use_case(case):
    problems = case.check(load(case))
    expected = known(case)
    if expected is None:      # a docs mismatch: any problem, but there must be one
        assert problems
    else:
        assert problems == expected


@pytest.mark.parametrize("case", USE_CASES, ids=lambda c: c.name)
def test_use_case_notices_breakage(case):
    real = load(case)
    assert case.check(case.mutate(real)) != case.check(real)
