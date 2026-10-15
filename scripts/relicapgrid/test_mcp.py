"""The harness's steps through cimmcp, cimoxide's MCP server, as a chat client drives it:
each result equals what cimoxide-py gives for the same files, so every check the other
tests make on cimoxide-py holds for cimmcp too."""

import json
import os
import re
import shutil
import subprocess
import uuid
from collections import Counter
from pathlib import Path

import pytest

import cimoxide
import harness
import test_edits as te
import test_export

ROOT = Path(__file__).resolve().parents[2]
OUT = "target/relicapgrid-mcp"


def cimmcp():
    """$CIMMCP, else cimmcp built from this checkout."""
    if os.environ.get("CIMMCP"):
        return os.environ["CIMMCP"]
    subprocess.run(["cargo", "build", "-q", "-p", "cimoxide-mcp"], cwd=ROOT, check=True)
    return str(Path(os.environ.get("CARGO_TARGET_DIR", ROOT / "target")) / "debug" / "cimmcp")


class Mcp:
    def __init__(self):
        self.proc = subprocess.Popen([cimmcp(), "--dir", str(ROOT)], stdin=subprocess.PIPE,
                                     stdout=subprocess.PIPE, text=True, encoding="utf-8")
        self.next_id = 0
        self.request("initialize", {"protocolVersion": "2025-06-18", "capabilities": {}})

    def request(self, method, params):
        self.next_id += 1
        self.proc.stdin.write(json.dumps({"jsonrpc": "2.0", "id": self.next_id, "method": method,
                                          "params": params}) + "\n")
        self.proc.stdin.flush()
        return json.loads(self.proc.stdout.readline())

    def call(self, tool, **args):
        result = self.request("tools/call", {"name": tool, "arguments": args})["result"]
        text = result["content"][0]["text"]
        assert not result["isError"], f"{tool}: {text}"
        return text

    def written(self, tool, name, **args):
        """What *tool* writes to a file of its own under target/."""
        self.call(tool, output=f"{OUT}/{name}", overwrite=True, **args)
        return (ROOT / OUT / name).read_bytes().decode("utf-8")      # as written: no newline translation


@pytest.fixture(scope="module")
def mcp():
    server = Mcp()
    yield server
    server.proc.kill()
    shutil.rmtree(ROOT / OUT, ignore_errors=True)


def source(path):
    return str(harness.relicap_dir() / path)


def files(paths):
    try:
        return paths()
    except RuntimeError:
        return []


# --- import ------------------------------------------------------------------

@pytest.mark.parametrize("path", files(harness.all_files))
def test_import(mcp, path):
    """The JSON dump of a file is cimoxide-py's entries() and foreign(), which
    test_import compares with triplets."""
    dataset = cimoxide.CimDataset.decode_file(source(path))
    dump = json.loads(mcp.written("export", "import.json", path=source(path), format="json"))
    assert dump == {**dataset.entries(), **dataset.foreign()}


# --- validation --------------------------------------------------------------

def group_names():
    return [g.name for g in harness.all_groups()]


@pytest.mark.parametrize("name", files(group_names))
def test_validation(mcp, name):
    """validate on a group, listed as files, reports cimoxide-py's findings, each once
    per (severity, rule, message, class, object, property)."""
    grp = harness.group(name)
    paths = [harness.with_declared_profile(fi.path, name == "cgmes-boundary") for fi in grp.files]
    expected = cimoxide.validate_files(paths, common=True, quality=True)
    text = mcp.call("validate", files=paths, common=True, quality=True, limit=1_000_000)
    assert text.startswith(f"{len(expected)} finding(s)")
    findings = text[text.find("\nFindings:\n"):] + "\n"
    for (severity, rule, message, cls, obj, prop), n in Counter(
            (v.severity, v.rule_id, v.message, v.class_, v.object_id, v.property) for v in expected).items():
        line = re.escape(f"  [{severity}] {rule} — {message} — {cls} {obj}") + r'(?: "[^"\n]*")?'
        line += (r" \." + re.escape(prop) if prop else "") + r"\n"
        assert len(re.findall(line, findings)) == n, (rule, obj, prop)


# --- export ------------------------------------------------------------------

def profile_exports():
    cgmes = [(fi.path, test_export.cgmes_profile(fi)) for fi in test_export.files("cgmes")]
    nc = [(fi.path, harness.nc_profile_code(fi)) for fi in test_export.files("nc") if harness.nc_profile_code(fi)]
    return cgmes + nc


@pytest.mark.parametrize("path, profile", files(profile_exports), ids=lambda x: str(x))
def test_export_profile(mcp, path, profile):
    expected = cimoxide.CimDataset.decode_file(source(path)).to_xml_for_profile(profile)
    assert mcp.written("export", "profile.xml", path=source(path), profile=profile) == expected


def whole_exports():
    return [fi.path for fi in test_export.every_file()]


@pytest.mark.parametrize("path", files(whole_exports))
def test_export_whole(mcp, path):
    expected = cimoxide.CimDataset.decode_file(source(path)).to_xml()
    assert mcp.written("export", "whole.xml", path=source(path)) == expected


# --- use cases: defining files and type conflicts ----------------------------

@pytest.mark.parametrize("name", files(group_names))
def test_conflicts(mcp, name):
    """conflicts lists the objects more than one file defines and those typed with
    unrelated classes, as test_use_cases finds them through cimoxide-py."""
    grp = harness.group(name)
    paths = [source(fi.path) for fi in grp.files]
    defined_by = Counter(m for p in paths for m in cimoxide.CimDataset.decode_file(p).defined_mrids())
    twice = sorted(m for m, n in defined_by.items() if n > 1)
    typed = sorted(m for m, _ in cimoxide.CimDataset.decode_files(paths).type_conflicts())
    text = mcp.call("conflicts", files=paths, limit=100_000)
    sections = dict.fromkeys(("defined", "typed"), [])
    for block in text.split("\n\n"):
        head, *rows = block.strip().splitlines()
        key = "defined" if "defined by more than one file" in head else "typed" if "unrelated classes" in head else None
        if key:
            sections[key] = sorted(re.match(r"\s+(?:\S+ )?(\S+)(?: \"[^\"]*\")? — ", r).group(1)
                                   if key == "defined" else r.split()[0] for r in rows)
    assert sections["defined"] == twice
    assert sections["typed"] == typed


# --- edits -------------------------------------------------------------------

def test_co_uc1(mcp):
    """CO_UC1's update through edit is test_edits.co_update's, byte for byte."""
    entries = te.decode(te.CO).entries()
    [first] = te.of_type(entries, "nc:ContingencyEquipment", "ContingencyElement.Contingency", te.CONTINGENCY)
    changes = [
        {"id": te.CONTINGENCY, "fields": {"IdentifiedObject.name": te.NEW_NAME}},
        {"id": te.new_id("CO_UC1 equipment"), "from": first, "fields": {
            "IdentifiedObject.name": "Belgovia-Svedala tie-line", "ContingencyEquipment.Equipment": te.SECOND_LINE}},
    ]
    dataset, _ = te.co_update()
    assert mcp.written("edit", "CO.xml", path=source(te.CO), profile="CO", changes=changes) == \
        dataset.to_xml_for_profile("CO")


def test_ra_uc4(mcp):
    """RA_UC4's counter-proposal through edit is test_edits.counter_proposal's, byte for
    byte: copies take their new identifiers, and only what changed is kept."""
    entries = te.decode(te.PROPOSED).entries()
    refused = te.REFUSED
    response, schedule, dependency = (te.new_id(f"RA_UC4 {n}") for n in ("response", "schedule", "dependency"))
    changes = [
        {"id": response, "type": "nc:RemedialActionScheduleResponse", "fields": {
            "RemedialActionScheduleResponse.mRID": response[1:],
            "RemedialActionScheduleResponse.kind": "RemedialActionScheduleResponseKind.refused",
            "RemedialActionScheduleResponse.rejectionReasonKind": "RejectionReasonKind.madeAlternative",
            "RemedialActionScheduleResponse.RemedialActionSchedule": refused,
            "RemedialActionScheduleResponse.RespondingEntity": te.RESPONDING}},
        {"id": schedule, "from": refused},
    ]
    for old in te.of_type(entries, "nc:EventSchedule", "EventSchedule.RemedialActionSchedule", refused):
        event_schedule = te.new_id(f"RA_UC4 {old}")
        changes.append({"id": event_schedule, "from": old,
                        "fields": {"EventSchedule.RemedialActionSchedule": schedule}})
        for point in te.of_type(entries, "nc:EventTimePoint", "EventTimePoint.EventSchedule", old):
            changes.append({"id": te.new_id(f"RA_UC4 {point}"), "from": point, "fields": {
                "EventTimePoint.EventSchedule": event_schedule, "EventTimePoint.isActive": "false"}})
    changes.append({"id": dependency, "type": "nc:RemedialActionScheduleDependency", "fields": {
        "RemedialActionScheduleDependency.mRID": dependency[1:],
        "RemedialActionScheduleDependency.ProposedRemedialActionSchedule": schedule,
        "RemedialActionScheduleDependency.ReplacedRemedialActionSchedule": refused}})
    [header] = te.of_type(entries, "nc:Dataset")
    changes.append({"id": f"urn:uuid:{uuid.uuid5(te.EDITS, 'RA_UC4 dataset')}", "from": header})
    dataset, _, _ = te.counter_proposal()
    assert mcp.written("edit", "RAS.xml", path=source(te.PROPOSED), profile="RAS", keep="changed",
                       changes=changes) == dataset.to_xml_for_profile("RAS")
