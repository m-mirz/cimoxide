"""The Annex 1 edit steps, replayed through cimoxide-py: CO_UC1's update of a
contingency and RA_UC4's refusal and counter-proposal of a remedial action
schedule. Each edit is exported for its profile, read back, and validated by
both tools in place of the file ReliCapGrid supplies."""

import uuid

import pytest

import cimoxide
import harness

# Deterministic identifiers for the objects an edit creates.
EDITS = uuid.UUID("6f1c2a54-3c1e-4c55-9a43-6b0f0e6c1d20")


def new_id(name):
    return f"_{uuid.uuid5(EDITS, name)}"


def decode(path):
    return cimoxide.CimDataset.decode_file(str(harness.relicap_dir() / path))


def of_type(entries, type_name, key=None, value=None):
    return sorted(k for k, v in entries.items()
                  if v["_type"] == type_name and (key is None or v.get(key) == value))


def replaced(path, xml):
    """(triplets, cimoxide) findings that appear when *xml* replaces *path* in the
    group ReliCapGrid validates it in, keyed (object, rule)."""
    return harness.mutated(path, lambda _text: xml)


def unexplained(new, before, sources):
    """The findings in *new* not explained by an existing object: one on a created
    object is explained when its source in *sources* (created → existing id) had
    the same rule before."""
    sources = {harness.norm(k): harness.norm(v) for k, v in sources.items()}
    return {(obj, rule) for obj, rule in new if (sources.get(obj), rule) not in before}


# --- CO_UC1 ------------------------------------------------------------------

CO = "Instance/Belgovia/NetworkCode/cimxml/Belgovia_CO.xml"
CONTINGENCY = "_7e31c67d-67ba-4592-8ac1-9e806d697c8e"   # loss of the Belgovia-Galia tie-line
SECOND_LINE = "_ed0c5d75-4a54-43c8-b782-b20d7431630b"   # the Belgovia-Svedala tie-line
NEW_NAME = "Loss of TieLines Belgovia-Galia and Belgovia-Svedala"


def co_update(drop=()):
    """The contingency becomes an N-2: renamed, and a second tie-line added as
    contingency equipment shaped like its first, less the fields in *drop*."""
    dataset = decode(CO)
    entries = dataset.entries()
    contingency = dict(entries[CONTINGENCY], **{"IdentifiedObject.name": NEW_NAME})
    dataset[CONTINGENCY] = contingency
    [first] = of_type(entries, "nc:ContingencyEquipment", "ContingencyElement.Contingency", CONTINGENCY)
    element = new_id("CO_UC1 equipment")
    equipment = dict(entries[first], **{
        "id": element,
        "IdentifiedObject.mRID": element[1:],
        "IdentifiedObject.name": "Belgovia-Svedala tie-line",
        "ContingencyEquipment.Equipment": SECOND_LINE,
    })
    dataset[element] = {k: v for k, v in equipment.items() if k not in drop}
    return dataset, element


def test_co_uc1_update_is_exported():
    dataset, element = co_update()
    again = cimoxide.CimDataset.decode_str(dataset.to_xml_for_profile("CO")).entries()
    assert again[CONTINGENCY]["IdentifiedObject.name"] == NEW_NAME
    assert again[element]["ContingencyElement.Contingency"] == CONTINGENCY
    assert again[element]["ContingencyEquipment.Equipment"] == SECOND_LINE
    original = decode(CO).entries()
    assert {k: v for k, v in again.items() if k not in (CONTINGENCY, element)} == \
        {k: v for k, v in original.items() if k != CONTINGENCY}, "the rest is unchanged"


# The first equipment's own findings explain the second's: its line is typed both
# ACLineSegment (EQ) and cim:Equipment (SSH), which both tools flag. A broken update
# must show something else: the contingent status the profile requires.
@pytest.mark.parametrize("drop, broken", [((), False), (("ContingencyEquipment.contingentStatus",), True)],
                         ids=["complete", "no contingent status"])
def test_co_uc1_update_validates(drop, broken):
    """Both tools report nothing on the update that its first equipment did not
    have, and both notice a missing contingent status."""
    dataset, element = co_update(drop)
    entries = decode(CO).entries()
    [first] = of_type(entries, "nc:ContingencyEquipment", "ContingencyElement.Contingency", CONTINGENCY)
    triplets_new, cimoxide_new = replaced(CO, dataset.to_xml_for_profile("CO"))
    triplets_before, cimoxide_before = harness.group_findings(CO)
    assert bool(unexplained(triplets_new, triplets_before, {element: first})) == broken
    assert bool(unexplained(cimoxide_new, cimoxide_before, {element: first})) == broken


# --- RA_UC4 ------------------------------------------------------------------

SVEDALA = "Instance/Svedala/NetworkCode/cimxml/"
PROPOSED = SVEDALA + "Svedala_RAS_Jotunheim_proposed.xml"        # the schedules RAO proposed
COUNTER = SVEDALA + "Svedala_RAS_acceptance_proposal.xml"        # ReliCapGrid's own answer
REFUSED = "_17527637-6099-4ee6-8b21-fea8dfec4c75"
RESPONDING = "_b9fbb881-573b-4e98-9b7b-31f00a45ebe0"            # Svedala, as in ReliCapGrid's answer
NC = "https://cim4.eu/ns/nc#"


def counter_proposal(kind=None, replaced_schedule=REFUSED):
    """Svedala's answer to the RAO proposal, built from it: a response refusing
    the whole schedule, a new complete schedule (new schedule, event schedule
    and time points, all with new identifiers), and a dependency from the new to
    the refused one, in a dataset of its own. The dependency's *kind* is left
    out by default: see test_ra_uc4_dependency_kind_conflicts_with_the_apl."""
    dataset = decode(PROPOSED)
    entries = dataset.entries()
    ids = {}

    def add(name, obj):
        ids[name] = obj["id"]
        dataset[obj["id"]] = obj

    response = new_id("RA_UC4 response")
    add("response", {
        "_type": "nc:RemedialActionScheduleResponse", "id": response,
        "RemedialActionScheduleResponse.mRID": response[1:],
        "RemedialActionScheduleResponse.kind": "RemedialActionScheduleResponseKind.refused",
        "RemedialActionScheduleResponse.rejectionReasonKind": "RejectionReasonKind.madeAlternative",
        "RemedialActionScheduleResponse.RemedialActionSchedule": REFUSED,
        "RemedialActionScheduleResponse.RespondingEntity": RESPONDING,
    })
    schedule = new_id("RA_UC4 schedule")
    add("schedule", dict(entries[REFUSED], id=schedule, **{"IdentifiedObject.mRID": schedule[1:]}))
    points = []
    for old in of_type(entries, "nc:EventSchedule", "EventSchedule.RemedialActionSchedule", REFUSED):
        event_schedule = new_id(f"RA_UC4 {old}")
        add(old, dict(entries[old], id=event_schedule, **{
            "IdentifiedObject.mRID": event_schedule[1:],
            "EventSchedule.RemedialActionSchedule": schedule}))
        for point in of_type(entries, "nc:EventTimePoint", "EventTimePoint.EventSchedule", old):
            copy = new_id(f"RA_UC4 {point}")
            points.append(copy)
            # The counter-proposal: not active where the RAO wanted the action.
            add(point, dict(entries[point], id=copy, **{
                "EventTimePoint.EventSchedule": event_schedule, "EventTimePoint.isActive": "false"}))
    dependency = new_id("RA_UC4 dependency")
    add("dependency", {
        "_type": "nc:RemedialActionScheduleDependency", "id": dependency,
        "RemedialActionScheduleDependency.mRID": dependency[1:],
        "RemedialActionScheduleDependency.ProposedRemedialActionSchedule": schedule,
        "RemedialActionScheduleDependency.ReplacedRemedialActionSchedule": replaced_schedule,
        **({"RemedialActionScheduleDependency.kind": f"RemedialActionScheduleDependencyKind.{kind}"}
           if kind else {}),
    })
    [header] = of_type(entries, "nc:Dataset")
    dataset_id = f"urn:uuid:{uuid.uuid5(EDITS, 'RA_UC4 dataset')}"
    add("header", dict(entries[header], id=dataset_id, identifier=dataset_id.removeprefix("urn:uuid:")))
    for mrid in entries:          # the RAO proposal stays in its own file
        del dataset[mrid]
    return dataset, ids, points


def test_ra_uc4_counter_proposal_is_exported():
    dataset, ids, points = counter_proposal()
    again = cimoxide.CimDataset.decode_str(dataset.to_xml_for_profile("RAS")).entries()
    assert sorted(again) == sorted(ids.values())
    response = again[ids["response"]]
    assert response["RemedialActionScheduleResponse.kind"].endswith("refused")
    assert response["RemedialActionScheduleResponse.RemedialActionSchedule"] == REFUSED
    dependency = again[ids["dependency"]]
    assert dependency["RemedialActionScheduleDependency.ProposedRemedialActionSchedule"] == ids["schedule"]
    assert dependency["RemedialActionScheduleDependency.ReplacedRemedialActionSchedule"] == REFUSED
    # Complete: every time point of the refused schedule, under new identifiers.
    refused_points = [p for es in of_type(decode(PROPOSED).entries(), "nc:EventSchedule",
                                          "EventSchedule.RemedialActionSchedule", REFUSED)
                      for p in of_type(decode(PROPOSED).entries(), "nc:EventTimePoint",
                                       "EventTimePoint.EventSchedule", es)]
    assert len(points) == len(refused_points) > 0
    assert not set(points) & set(refused_points)
    header = again[ids["header"]]
    assert header["_type"] == "nc:Dataset" and header["identifier"] == ids["header"].removeprefix("urn:uuid:")


# Each created object's counterpart: what it was copied from, or the object ReliCapGrid's
# own answer (Svedala_RAS_acceptance_proposal.xml) has in its place.
COUNTERPARTS = {
    "header": "urn:uuid:baf8c34a-4944-4309-8d9f-8f39c3e4bac1",         # copied from the RAO proposal
    "response": "_96f93b71-420f-456f-932e-9ac3f724b683",
    "schedule": "_51361821-0a40-4e5a-a49d-29c266ea2ecc",
    "dependency": "_f7231aa5-d497-41f3-a5d5-9d4af74444f0",
}


def sources(ids):
    copied = {ids[old]: old for old in ids if old.startswith("_")}     # event schedules, time points
    return {**copied, **{ids[name]: COUNTERPARTS[name] for name in COUNTERPARTS}}


# (triplets notices, cimoxide notices): a dependency "replacing" the response, an
# object of the same file, fails ras:...ReplacedRemedialActionSchedule-valueType;
# triplets 0.2.0 misses it.
@pytest.mark.parametrize("replaced_schedule, broken", [(REFUSED, (False, False)), ("response", (False, True))],
                         ids=["refused schedule", "response"])
def test_ra_uc4_counter_proposal_validates(replaced_schedule, broken):
    """In place of ReliCapGrid's answer, both tools report nothing on ours that its
    counterpart did not have; cimoxide notices a dependency replacing the
    response rather than the schedule."""
    if replaced_schedule == "response":
        replaced_schedule = new_id("RA_UC4 response")
    dataset, ids, _ = counter_proposal(replaced_schedule=replaced_schedule)
    triplets_new, cimoxide_new = replaced(COUNTER, dataset.to_xml_for_profile("RAS"))
    triplets_before, cimoxide_before = harness.group_findings(COUNTER)
    assert bool(unexplained(triplets_new, triplets_before, sources(ids))) == broken[0]
    assert bool(unexplained(cimoxide_new, cimoxide_before, sources(ids))) == broken[1]


def test_ra_uc4_dependency_kind_conflicts_with_the_apl():
    """The report's step sets RemedialActionScheduleDependencyKind none; the APL
    (rasc:RemedialActionScheduleDependency.kind-applicability) forbids a kind on a
    dependency outside a RemedialActionScheduleGroup. ReliCapGrid's data follows
    the APL; both tools flag the documented step."""
    dataset, ids, _ = counter_proposal(kind="none")
    triplets_new, cimoxide_new = replaced(COUNTER, dataset.to_xml_for_profile("RAS"))
    rule = "RemedialActionScheduleDependency.kind-applicability"
    dependency = harness.norm(ids["dependency"])
    assert (dependency, rule) in triplets_new
    assert (dependency, rule) in cimoxide_new
