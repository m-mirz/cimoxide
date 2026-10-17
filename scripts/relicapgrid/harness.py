"""cimoxide against ReliCapGrid, with triplets (ReliCapGrid's own CI tool) as the reference.

The file groups and the triplets validation come from ReliCapGrid's
buildScripts/validate_instances.py, imported from the checkout named by
RELICAPGRID_DIR, else cimoxide's relicapgrid submodule, so both tools see the
same files under the same policies.

    python harness.py --report out/
"""

import argparse
import contextlib
import functools
import importlib.metadata
import json
import os
import re
import shutil
import sys
import tempfile
from collections import Counter, defaultdict
from dataclasses import dataclass
from pathlib import Path

import pandas

import cimoxide

CIMOXIDE_ROOT = Path(__file__).resolve().parents[2]
APL_DIR = Path(os.environ.get("CIMOXIDE_APL_DIR", CIMOXIDE_ROOT / "application-profiles-library"))

# triplets keeps these per file; they are bookkeeping, not data
TRIPLETS_META_TYPES = {"Distribution", "NamespaceMap"}
TRIPLETS_META_KEYS = {"label", "xml_base"}


RELICAPGRID_SUBMODULE = CIMOXIDE_ROOT / "relicapgrid"


def relicap_dir():
    """The ReliCapGrid checkout: RELICAPGRID_DIR, else the relicapgrid submodule once
    checked out (it is not by default: `git submodule update --init --checkout relicapgrid`)."""
    path = os.environ.get("RELICAPGRID_DIR")
    if path:
        return Path(path).resolve()
    if (RELICAPGRID_SUBMODULE / "Instance").is_dir():
        return RELICAPGRID_SUBMODULE
    raise RuntimeError("check out the relicapgrid submodule "
                       "(git submodule update --init --checkout relicapgrid) or set RELICAPGRID_DIR")


@contextlib.contextmanager
def in_dir(path):
    previous = Path.cwd()
    os.chdir(path)
    try:
        yield
    finally:
        os.chdir(previous)


@functools.cache
def vi():
    """ReliCapGrid's validate_instances module."""
    sys.path.insert(0, str(relicap_dir() / "buildScripts"))
    import validate_instances
    return validate_instances


# --- identifiers -------------------------------------------------------------

def norm(value):
    """The comparable form of an id or value, applied to both tools alike: text
    after the last '#', without 'urn:uuid:' or a leading '_'; a blank node by its
    label (cimoxide's '_:A1', triplets' 'A1')."""
    value = str(value)
    if "#" in value:
        value = value.rsplit("#", 1)[1]
    return value.removeprefix("_:").removeprefix("urn:uuid:").removeprefix("_")


def bare_type(type_name):
    """A class's local name: after the family prefix (`nc:Dataset`), or after the
    last '#' or '/' of a foreign object's class IRI (`http://www.w3.org/ns/prov#Agent`)."""
    if "://" in type_name:
        return re.split(r"[#/]", type_name)[-1]
    return type_name.split(":", 1)[1] if ":" in type_name else type_name


# --- the reference run -------------------------------------------------------

@dataclass
class Reference:
    frame: pandas.DataFrame       # every instance file, as triplets loads it
    infos: list                   # validate_instances.FileInfo per mapped file
    skipped: list                 # (path, reason)
    groups: dict                  # release -> [validate_instances.Group]


@functools.cache
def reference():
    v = vi()
    with in_dir(relicap_dir()):
        frame, infos, skipped, _fixed = v.scan_instances()
        groups = {}
        for release, config in v.RELEASES.items():
            prof_map, _gaps = v.build_prof_map(APL_DIR)
            build = v.build_cgmes_groups if config["kind"] == "cgmes" else v.build_nc_groups
            groups[release], more = build(infos, prof_map, APL_DIR, config["rdf_map"])
            skipped = skipped + more
    return Reference(frame, infos, skipped, groups)


def all_files():
    """Every instance file, repo-relative, the duplicate GridSituation copies excluded."""
    v = vi()
    root = relicap_dir()
    return sorted(str(p.relative_to(root)) for p in root.glob("Instance/**/*.xml")
                  if not p.relative_to(root).match(v.DUPLICATE_GLOB))


@functools.cache
def instance_ids():
    """Path -> triplets INSTANCE_ID for every loaded file, mapped to a profile or not."""
    frame = reference().frame
    labels = frame[frame["KEY"] == "label"]
    return {str(r.VALUE): str(r.INSTANCE_ID) for r in labels.itertuples()}


# --- import ------------------------------------------------------------------

def triplets_triples(instance_id):
    frame = reference().frame
    rows = frame[frame["INSTANCE_ID"].astype(str) == instance_id]
    meta = set(rows.loc[(rows["KEY"] == "Type") & rows["VALUE"].isin(TRIPLETS_META_TYPES), "ID"])
    rows = rows[~rows["ID"].isin(meta) & ~rows["KEY"].isin(TRIPLETS_META_KEYS)]
    return Counter((norm(r.ID), str(r.KEY), norm(r.VALUE) if pandas.notna(r.VALUE) else "")
                   for r in rows.itertuples())


def cimoxide_triples(entries):
    triples = Counter()
    for mrid, fields in entries.items():
        subject = norm(mrid)
        triples[(subject, "Type", bare_type(fields["_type"]))] += 1
        for key, value in fields.items():
            if key in ("_type", "id", "_lang"):
                continue
            for item in value if isinstance(value, list) else [value]:
                triples[(subject, key, norm(item))] += 1
    return triples


@dataclass
class ImportDiff:
    path: str
    triplets: int
    cimoxide: int
    missing: Counter              # triplets has it, cimoxide does not
    extra: Counter                # cimoxide has it, triplets does not


def classify(diff):
    """Each differing triple by kind: 'whitespace' (cimoxide trimmed the
    literal), 'empty' (an empty element cimoxide dropped) or 'other'."""
    extra = Counter(diff.extra)
    kinds = defaultdict(list)
    for (subject, key, value), count in diff.missing.items():
        trimmed = (subject, key, value.strip())
        if value and value != value.strip() and extra[trimmed]:
            extra[trimmed] -= count
            kinds["whitespace"].append((subject, key, value))
        elif value == "":
            kinds["empty"].append((subject, key, value))
        else:
            kinds["other"].append(("-", subject, key, value))
    kinds["other"] += [("+", *triple) for triple, count in extra.items() if count > 0]
    return {kind: triples for kind, triples in kinds.items() if triples}


# Application profile name (the path segment of a header's conformsTo) → NC profile code.
NC_PROFILE_CODES = {
    "AssessedElement": "AE", "AvailabilitySchedule": "AS", "Contingency": "CO",
    "DatasetMetadata": "DM", "EquipmentReliability": "ER", "GridDisturbance": "GD",
    "ImpactAssessmentMatrix": "IAM", "MonitoringArea": "MA", "ObjectRegistry": "OR",
    "PowerSchedule": "PS", "PowerSystemProject": "PSP", "RemedialAction": "RA",
    "RemedialActionSchedule": "RAS", "SecurityAnalysisResult": "SAR",
    "SteadyStateHypothesisSchedule": "SHS", "StateInstructionSchedule": "SIS",
    "SensitivityMatrix": "SM", "SteadyStateInstruction": "SSI",
}


def nc_profile_code(fi):
    """An NC file's profile code, from the application profile its header conforms
    to (https://ap.cim4.eu/Contingency/2.3 → CO); None for a profile cimoxide lacks
    (Provenance)."""
    for uri in fi.profile_uris:
        parts = uri.split("://", 1)[-1].split("/")
        name = parts[1] if len(parts) > 1 else ""
        if name in NC_PROFILE_CODES:
            return NC_PROFILE_CODES[name]
    return None


def group_files(group):
    return [str(relicap_dir() / fi.path) for fi in group.files]


def all_groups():
    return [g for groups in reference().groups.values() for g in groups]


def group(name):
    return next(g for g in all_groups() if g.name == name)


# --- validation --------------------------------------------------------------

def shape_local(shape):
    """The shape's local name: after '#' of an IRI, after the prefix of a CURIE."""
    shape = str(shape)
    if "#" in shape:
        return shape.rsplit("#", 1)[1]
    return shape.split(":", 1)[1] if ":" in shape and "//" not in shape else shape


CACHE = Path(__file__).resolve().parent / ".cache"


def triplets_findings(grp):
    """ReliCapGrid's own validate_group, cached per dataset commit and group."""
    commit = (relicap_dir() / ".git").exists() and os.popen(
        f"git -C {relicap_dir()} rev-parse --short HEAD").read().strip() or "nogit"
    cached = CACHE / commit / f"{grp.name}.pkl"
    if cached.exists():
        return pandas.read_pickle(cached)
    with in_dir(relicap_dir()):
        _located, reported = vi().validate_group(reference().frame, grp)
    cached.parent.mkdir(parents=True, exist_ok=True)
    reported.to_pickle(cached)
    return reported


@functools.cache
def ids_in(path):
    return frozenset(norm(m) for m in cimoxide.CimDataset.decode_file(str(relicap_dir() / path)).entries())


BOUNDARY_PROFILE = "http://iec.ch/TC57/ns/CIM/EquipmentBoundary-EU/3.0"


@functools.cache
def patched_dir():
    return Path(tempfile.mkdtemp(prefix="cimoxide-relicapgrid-"))


def with_declared_profile(path, boundary_shapes=False):
    """ReliCapGrid's BOUNDARY_PROFILE_FIX for cimoxide: its boundary files' dcat:Dataset
    header names vocabularies only (CIM100#, nc/2.4#), no application profile, so no
    tool knows which shapes apply. ReliCapGrid's CI validates them as CoreEquipment-EU/3.0;
    the copy adds that profile to the header's dcterms:conformsTo, and
    EquipmentBoundary-EU/3.0 with *boundary_shapes*: ReliCapGrid keeps those shapes for
    its boundary group only."""
    prefix, profile = vi().BOUNDARY_PROFILE_FIX
    source = relicap_dir() / path
    if not path.startswith(prefix):
        return str(source)
    text = source.read_text(encoding="utf-8")
    if profile in text:
        return str(source)
    profiles = [profile, BOUNDARY_PROFILE] if boundary_shapes else [profile]
    declared = "".join(f'<dcterms:conformsTo rdf:resource="{p}"/>' for p in profiles)
    header = re.search(r"<dcat:Dataset\b[^>]*>", text)
    target = patched_dir() / ("eqbd" if boundary_shapes else "eq") / path
    target.parent.mkdir(parents=True, exist_ok=True)
    target.write_text(text[:header.end()] + declared + text[header.end():], encoding="utf-8")
    return str(target)


def cimoxide_findings(grp):
    """cimoxide on the group's files, kept to objects of the reported files."""
    reported_ids = frozenset().union(*(ids_in(p) for p in grp.report_paths))
    paths = [with_declared_profile(fi.path, grp.name == "cgmes-boundary") for fi in grp.files]
    violations = cimoxide.validate_files(paths, common=True, quality=True)
    return [v for v in violations if norm(v.object_id) in reported_ids]


@dataclass
class Finding:
    object_id: str
    rule: str                     # shape local name
    severity: str
    property: str
    message: str


def reference_set(grp):
    frame = triplets_findings(grp)
    return [Finding(norm(r.ID), shape_local(r.SOURCE_SHAPE), str(r.SEVERITY).removeprefix("sh:"),
                    str(r.KEY), str(r.MESSAGE)) for r in frame.itertuples()] if len(frame) else []


def cimoxide_set(grp):
    return [Finding(norm(v.object_id), shape_local(v.rule_id), v.severity.removeprefix("sh:"),
                    v.property, v.message) for v in cimoxide_findings(grp)]


@functools.cache
def cimoxide_rules():
    """Local names of every rule cimoxide can report: the generated shape tables'
    rule ids and the hand-written SPARQL rules' literals."""
    src = CIMOXIDE_ROOT / "cimvalidation" / "src"
    pattern = re.compile(r'"[A-Za-z0-9]+:([A-Za-z0-9_.]+-[A-Za-z0-9_.-]+)"')
    return frozenset(m for path in src.rglob("*.rs") for m in pattern.findall(path.read_text()))


@functools.cache
def triplets_rules(grp_name):
    grp = group(grp_name)
    with in_dir(relicap_dir()):
        return frozenset(shape_local(s) for s in vi().shape_ids(vi().all_shapes(grp)))


# How a (object, rule) key differs. Only "disagree" needs a look; the others
# are differences in what each tool runs.
AGREE = "agree"
DISAGREE = "disagree"
NOT_IN_CIMOXIDE = "rule not in cimoxide"
NOT_IN_TRIPLETS = "rule not run by triplets"
TRIPLETS_EXTENSION = "triplets-only check"     # triplets:lexicalForm and the like


# Why each rule the two tools disagree on differs, checked against the data and
# the APL at relicapgrid f09bb34 / APL d8b2d21. "right" names the tool whose
# result follows the shape as written.
TRIAGE = {
    "IdentifiedObject.mRID-cardinality": (
        "cimoxide",
        "SSH describes equipment with rdf:about and no mRID; the EQ file defines it. cimoxide asks "
        "for an mRID only where an object is defined (rdf:ID); triplets' per-file pass cannot see EQ."),
    "description.langTag-presence": (
        "cimoxide", "The data carries xml:lang=\"en\"; triplets drops language tags of this literal."),
    "versionNotes.langTag-presence": (
        "cimoxide", "As description.langTag-presence."),
    "identifier-datatype": (
        "cimoxide", "dcterms:identifier holds a UUID literal; triplets reads it as a reference."),
    "conformsTo-datatype": (
        "neither", "conformsTo URIs ending in '#' (CIM100#, nc/2.4#): both tools keep the text after "
        "'#', an empty string; triplets then fails sh:nodeKind sh:IRI, cimoxide passes."),
    "Model.profile-allowedValues": (
        "cimoxide", "sh:in lists xsd:anyURI literals, the data plain literals; triplets compares terms."),
    "RemedialActionSchedule-proposingEntity": (
        "cimoxide", "ProposingEntity is present; triplets 0.2.0 does not evaluate sh:alternativePath."),
    "Circuit-associations": (
        "cimoxide", "Circuit.IdentifyingTerminal is present; triplets does not evaluate sh:alternativePath."),
    "BaseTimeSeries.interpolationKind-datatype": (
        "cimoxide", "The RAS shape allows 'none'; triplets runs the AS shape of the same name on RAS data "
        "in the group union."),
    "EnergyConnection-containment": (
        "neither", "Container defined (rdf:ID) in several datasets; triplets counts the repeated type twice. "
        "The duplicate definition is a data issue."),
    "BusbarSection-containment": (
        "neither", "VoltageLevel 0c4f41f8 is defined in the boundary, Portheim EQ and Espheim EQ; "
        "triplets counts each definition. The duplication is a data issue."),
    "ACDCConverter.PccTerminal-valueType": (
        "cimoxide", "Two shapes share the local name (eq: Simple, equ: Complex); cimoxide's equ: finding "
        "is right: Britheim's PCC terminal is on an ACLineSegment."),
    "ContingencyWithRemedialAction.RemedialAction-valueType": (
        "triplets", "Data bug: Espheim_CO.xml's dataset header reuses the UUID of GridStateAlteration"
        "RemedialAction 5898c268 (Espheim_RA.xml). cimoxide keys urn:uuid: and _ ids apart."),
    "RemedialActionSchedule.RemedialAction-valueType": (
        "triplets", "As ContingencyWithRemedialAction.RemedialAction-valueType."),
    "RemedialActionScheduleOutcomeValue.RemedialActionSchedule-valueType": (
        "neither", "The IAM files reference the TSOs' RAS objects, which ReliCapGrid's nc-Jotunheim group "
        "does not load; cimoxide reports them as dangling references instead."),
    "EquivalentInjection-containment": (
        "neither", "Container defined in several datasets (boundary and IGMs); triplets counts each definition."),
    "GeneratingUnit-containment": (
        "neither", "Substation defined in several IGMs; triplets counts each definition."),
    "PowerTransformer-containment": (
        "neither", "Substation defined in several IGMs; triplets counts each definition."),
    "Terminal.ConductingEquipment-valueType": (
        "triplets", "EquipmentBoundary shape: boundary terminals must be on a Junction; the data has a "
        "BusbarSection."),
    "SvStatus.ConductingEquipment-valueType": (
        "cimoxide",
        "sv:SvStatus.ConductingEquipment-valueType (SV Simple) lists only CsConverter and VsConverter, "
        "the SV profile's own equipment: CGMES per-profile value types are written for one file, where "
        "EQ objects have no type. triplets runs them on the group union. The cross-profile "
        "sv456cpe: shape (sh:class ConductingEquipment) checks the type on the merged model."),
    "requires-valueType": (
        "triplets", "As ContingencyWithRemedialAction.RemedialAction-valueType: Espheim_RA requires the "
        "Espheim_CO header, whose UUID a remedial action reuses."),
    "ContingencyEquipment.Equipment-valueType": (
        "cimoxide", "Svedala_ER.xml types the machine cim:SynchronousMachine in the NC namespace, "
        "which the sh:in list (cim16:/cim17: machines) omits; on the union that type fails. triplets "
        "matches classes by local name."),
}

# NCP's sh:class shapes name the NC namespace (cim: = https://cim.ucaiug.io/ns#) while ReliCapGrid's
# targets are CGMES 3.0 objects in CIM100; the sh:in value types list both (cim17:...). An APL
# inconsistency: cimoxide checks the IRI as written, triplets matches classes by local name.
for _rule in ("Circuit.IdentifyingTerminal", "CurrentLimitSchedule.CurrentLimit",
              "EquivalentInjectionAction.EquivalentInjection", "OperationalLimit.OperationalLimitSet",
              "OperationalLimit.OperationalLimitType", "PinTerminal.Terminal",
              "ReactiveCapabilityCurve.SynchronousMachine2", "VoltageAngleLimit.AngleReferenceTerminal",
              "SchedulingArea.ControlArea"):
    TRIAGE[f"{_rule}-valueType"] = (
        "cimoxide", "The shape requires an NC-namespace class (cim:, sh:class or sh:in without cim17:); "
        "the target is the CGMES 3.0 (CIM100) object in EQ. APL inconsistency; triplets matches local names.")

# BoundaryPoint.*: the boundary files use nc:BoundaryPoint (NC namespace and NCP 2.5 properties)
# while the CGMES EQ shapes target cim:BoundaryPoint. triplets matches local names.
for _name in ("ConnectivityNode", "fromEndIsoCode", "fromEndName", "fromEndNameTso", "toEndIsoCode",
              "toEndName", "toEndNameTso"):
    TRIAGE[f"BoundaryPoint.{_name}-cardinality"] = (
        "cimoxide", "Data has nc:BoundaryPoint; the CGMES EQ shape targets cim:BoundaryPoint. triplets "
        "matches classes by local name.")


@dataclass
class Difference:
    kind: str
    side: str                     # "both", "triplets", "cimoxide"
    finding: Finding
    violation_type: str = ""


def validation_diff(grp):
    """Every (object, rule) key either tool reported, classified."""
    ours = {(f.object_id, f.rule): f for f in cimoxide_set(grp)}
    frame = triplets_findings(grp)
    theirs, types = {}, {}
    for f, r in zip(reference_set(grp), frame.itertuples() if len(frame) else []):
        theirs[(f.object_id, f.rule)] = f
        types[(f.object_id, f.rule)] = str(r.VIOLATION_TYPE)
    ran = triplets_rules(grp.name)
    differences = []
    for key in sorted(ours.keys() | theirs.keys()):
        if key in ours and key in theirs:
            differences.append(Difference(AGREE, "both", ours[key], types[key]))
        elif key in theirs:
            vtype = types[key]
            kind = (TRIPLETS_EXTENSION if vtype.startswith("triplets:")
                    else NOT_IN_CIMOXIDE if key[1] not in cimoxide_rules() else DISAGREE)
            differences.append(Difference(kind, "triplets", theirs[key], vtype))
        else:
            kind = NOT_IN_TRIPLETS if key[1] not in ran else DISAGREE
            differences.append(Difference(kind, "cimoxide", ours[key]))
    return differences


def import_diff(path):
    instance_id = instance_ids()[path]
    reference_triples = triplets_triples(instance_id)
    dataset = cimoxide.CimDataset.decode_file(str(relicap_dir() / path))
    ours = cimoxide_triples({**dataset.entries(), **dataset.foreign()})
    return ImportDiff(path, sum(reference_triples.values()), sum(ours.values()),
                      reference_triples - ours, ours - reference_triples)


# --- mutations ---------------------------------------------------------------

def reporting_group(path):
    return next(g for g in all_groups() if path in g.report_paths)


@contextlib.contextmanager
def repo_root(path):
    v = vi()
    previous = v.REPO_ROOT
    v.REPO_ROOT = Path(path)
    try:
        yield
    finally:
        v.REPO_ROOT = previous


def group_findings(path):
    """(triplets, cimoxide) findings of the group that reports *path*, keyed (object, rule):
    what mutated() compares against."""
    grp = reporting_group(path)
    return ({(f.object_id, f.rule) for f in reference_set(grp)},
            {(f.object_id, f.rule) for f in cimoxide_set(grp)})


def mutated(path, transform):
    """(triplets, cimoxide) findings that appear when *transform* rewrites the
    text of *path*: both tools run on the group that reports *path*, as
    ReliCapGrid forms it, with the one file changed. Keyed (object, rule)."""
    grp = reporting_group(path)
    source = relicap_dir() / path
    text = source.read_text(encoding="utf-8")
    changed = transform(text)
    assert changed != text, f"mutation left {path} unchanged"
    with tempfile.TemporaryDirectory(prefix="cimoxide-mutation-") as tmp:
        tree = Path(tmp)
        for fi in grp.files:
            target = tree / fi.path
            target.parent.mkdir(parents=True, exist_ok=True)
            shutil.copyfile(relicap_dir() / fi.path, target)
        (tree / path).write_text(changed, encoding="utf-8")
        with repo_root(tree), in_dir(tree):
            frame, infos, _skipped, _fixed = vi().scan_instances()
            release = next(r for r, groups in reference().groups.items() if grp in groups)
            config = vi().RELEASES[release]
            prof_map, _gaps = vi().build_prof_map(APL_DIR)
            build = vi().build_cgmes_groups if config["kind"] == "cgmes" else vi().build_nc_groups
            groups, _ = build(infos, prof_map, APL_DIR, config["rdf_map"])
            twin = next(g for g in groups if g.name == grp.name)
            _located, reported = vi().validate_group(frame, twin)
        paths = [str(tree / path) if fi.path == path else with_declared_profile(fi.path) for fi in grp.files]
        ours = cimoxide.validate_files(paths, common=True, quality=True)
    after_triplets = {(norm(r.ID), shape_local(r.SOURCE_SHAPE)) for r in reported.itertuples()} if len(reported) else set()
    after_ours = {(norm(v.object_id), shape_local(v.rule_id)) for v in ours}
    before_triplets = {(f.object_id, f.rule) for f in reference_set(grp)}
    before_ours = {(norm(v.object_id), shape_local(v.rule_id))
                   for v in cimoxide.validate_files([with_declared_profile(fi.path) for fi in grp.files],
                                                    common=True, quality=True)}
    return after_triplets - before_triplets, after_ours - before_ours


# --- report ------------------------------------------------------------------

def write_import_report(out):
    rows, totals = [], Counter()
    for path in all_files():
        diff = import_diff(path)
        for kind, triples in classify(diff).items():
            totals[kind] += len(triples)
            rows += [{"path": path, "kind": kind, "triple": " | ".join(map(str, t))} for t in triples]
    pandas.DataFrame(rows, columns=["path", "kind", "triple"]).to_csv(out / "import_diff.csv", index=False)
    return totals


def write_validation_report(out):
    rows = []
    for grp in all_groups():
        for d in validation_diff(grp):
            right, why = TRIAGE.get(d.finding.rule, ("", "")) if d.kind == DISAGREE else ("", "")
            rows.append({"group": grp.name, "kind": d.kind, "side": d.side, "rule": d.finding.rule,
                         "object_id": d.finding.object_id, "property": d.finding.property,
                         "severity": d.finding.severity, "violation_type": d.violation_type,
                         "right": right, "why": why, "message": d.finding.message})
    frame = pandas.DataFrame(rows)
    frame.to_csv(out / "validation_diff.csv", index=False)
    return frame


# cimoxide's row in the report's Annex 1 format, from test_import, test_export,
# test_validation and test_use_cases. "import" means the use case's files decode
# and the use-case checks pass on what was decoded; "validate" that cimoxide
# detects the profiles and its findings match triplets' up to TRIAGE; "export" that
# each file exported for its profile is the same RDF graph (test_export).
ANNEX_1 = [
    ("PF_UC1", "import, validate, export", "no power flow; SV covers every energised TopologicalNode"),
    ("BDS_UC1", "import, validate*", "*boundary header declares no profile (EQ forced as in ReliCapGrid CI); "
                "merged into the CGM: 4 boundary points reached from one IGM, Portheim outside the CGM "
                "topology, 8 objects defined twice"),
    ("CD_UC1", "import, validate, export", "3 common data objects defined again by other datasets"),
    ("RD_UC1", "import, export", "SKOS/PROV/DCAT kept apart from CIM objects (foreign()), not validated; "
               "285 of 390 header URIs name reference data the files do not define"),
    ("CO_UC1", "import, validate, edit, export", "all 11 contingency -> equipment pairs resolve; "
               "an N-1 made N-2 exports and validates (test_edits)"),
    ("ER_UC1", "import, validate, export", ""),
    ("RA_UC1..7", "import, validate, edit, export", "RA_UC1's action leaves its switch closed; RA_UC2/5/7 data "
                  "differs from the docs; RA_UC4's "
                  "refusal and counter-proposal built and validated (test_edits), its documented "
                  "kind none contradicts the APL"),
    ("AS_UC1", "import, validate, export", "the schedule is cancelled but has no window (AvailabilityTimePoint)"),
    ("AE_UC1..6", "import, validate, export", ""),
    ("OR_UC1", "import, validate, export", ""),
    ("SAR_UC1", "import, validate", "the report's export means producing a SAR, which needs contingency "
                "analysis; cimoxide only re-exports the supplied one"),
]


def write_summary(out, import_totals, validation):
    lines = ["# cimoxide on ReliCapGrid", "",
             f"Dataset: `{relicap_dir()}`, cimoxide-py {importlib.metadata.version('cimoxide')}", "",
             "## Import (triple diff against triplets)", ""]
    lines += [f"- {kind}: {n}" for kind, n in sorted(import_totals.items())] or ["- identical"]
    lines += ["", "## Validation (per group, keyed by object and rule)", "",
              "| kind | side | findings |", "|---|---|---|"]
    for (kind, side), n in validation.groupby(["kind", "side"]).size().items():
        lines.append(f"| {kind} | {side} | {n} |")
    disagree = validation[validation["kind"] == DISAGREE]
    lines += ["", "### Disagreements by rule", "", "| rule | side | findings | right | why |",
              "|---|---|---|---|---|"]
    for (rule, side), part in disagree.groupby(["rule", "side"]):
        right, why = TRIAGE.get(rule, ("**untriaged**", ""))
        lines.append(f"| {rule} | {side} | {len(part)} | {right} | {why} |")
    lines += ["", "## Annex 1 row", "", "| use case | cimoxide | notes |", "|---|---|---|"]
    lines += [f"| {uc} | {result} | {note} |" for uc, result, note in ANNEX_1]
    (out / "summary.md").write_text("\n".join(lines) + "\n", encoding="utf-8")


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--report", type=Path, default=Path("out"), help="output directory")
    args = parser.parse_args()
    args.report.mkdir(parents=True, exist_ok=True)
    import_totals = write_import_report(args.report)
    validation = write_validation_report(args.report)
    write_summary(args.report, import_totals, validation)
    print(f"wrote {args.report}/import_diff.csv, validation_diff.csv, summary.md")


if __name__ == "__main__":
    main()
