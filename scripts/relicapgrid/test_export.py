"""Export: each file encoded for its own profile, and whole, compared with the source as RDF graphs."""

import io
import re

import pyoxigraph
import pytest

import cimoxide
import harness

CGMES_PROFILE = {"CoreEquipment": "EQ", "SteadyStateHypothesis": "SSH", "Topology": "TP",
                 "StateVariables": "SV"}


def files(kind):
    try:
        infos = harness.reference().infos
    except RuntimeError:
        return []
    # CGMES boundary and common data declare no profile of their own.
    return [fi for fi in infos
            if fi.kind == kind and (kind == "nc" or fi.area not in ("boundaryData", "commonData"))]


def cgmes_profile(fi):
    match = re.search(r"/CIM/(\w+)-EU/", fi.profile_uris[0])
    return CGMES_PROFILE[match.group(1)]


def graph(xml):
    """The RDF graph of an RDF/XML document, as (subject, predicate, object) strings:
    full IRIs, datatypes and language tags, which cimoxide's own view does not show."""
    triples = pyoxigraph.parse(io.BytesIO(xml.encode("utf-8")), format=pyoxigraph.RdfFormat.RDF_XML,
                               base_iri="http://example.org/document")
    return {(str(t.subject), str(t.predicate), str(t.object)) for t in triples}


RDF_TYPE = "<http://www.w3.org/1999/02/22-rdf-syntax-ns#type>"
EQUIPMENT = "<http://iec.ch/TC57/CIM100#Equipment>"


@pytest.mark.parametrize("fi", files("cgmes"), ids=lambda fi: fi.path)
def test_cgmes_round_trip(fi):
    """Each CGMES file, exported for its own profile, is the same RDF graph."""
    path = harness.relicap_dir() / fi.path
    before = graph(path.read_text(encoding="utf-8"))
    after = graph(cimoxide.CimDataset.decode_file(str(path)).to_xml_for_profile(cgmes_profile(fi)))
    # SSH writes the class it declares Equipment.inService on: an object the file
    # types BusbarSection or PowerTransformer, classes the SSH vocabulary lacks, comes
    # back as cim:Equipment, as the ENTSO-E test configurations write it.
    lost, added = before - after, after - before
    assert all(p == RDF_TYPE for _, p, _ in lost)
    assert added == {(s, RDF_TYPE, EQUIPMENT) for s, _, _ in lost}
    if lost:
        assert cgmes_profile(fi) == "SSH"


# Fields an NC file carries that its profile does not exchange: the export leaves
# them out, and both tools report them (ContingencyPowerFlowResult-AllowedProperties).
NOT_IN_PROFILE = {("Instance/Jotunheim/NetworkCode/2D_SAR_1030Z_Jotunheim.xml", "IdentifiedObject.mRID")}


@pytest.mark.parametrize("fi", files("nc"), ids=lambda fi: fi.path)
def test_nc_round_trip(fi):
    """Each NC file, exported for its own profile, is the same RDF graph, less the
    fields its profile does not exchange."""
    path = harness.relicap_dir() / fi.path
    dataset = cimoxide.CimDataset.decode_file(str(path))
    code = harness.nc_profile_code(fi)
    if code is None:
        with pytest.raises(RuntimeError, match="unknown profile code"):
            dataset.to_xml_for_profile("PV")
        pytest.xfail(f"no NC profile cimoxide knows: {fi.profile_uris}")
    before = graph(path.read_text(encoding="utf-8"))
    after = graph(dataset.to_xml_for_profile(code))
    assert not after - before
    dropped = {key for p, key in NOT_IN_PROFILE if p == fi.path}
    assert {re.split(r"[#/]", predicate.rstrip(">"))[-1] for _, predicate, _ in before - after} <= dropped


def every_file():
    try:
        return harness.reference().infos
    except RuntimeError:
        return []


@pytest.mark.parametrize("fi", every_file(), ids=lambda fi: fi.path)
def test_whole_round_trip(fi):
    """Each file, exported whole (to_xml: CGMES, NC and the objects no CIM table
    knows, as read), is the same RDF graph."""
    path = harness.relicap_dir() / fi.path
    xml = cimoxide.CimDataset.decode_file(str(path)).to_xml()
    assert graph(xml) == graph(path.read_text(encoding="utf-8"))
