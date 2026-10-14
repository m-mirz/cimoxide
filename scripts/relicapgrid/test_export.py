"""Export: each file encoded for its own profile, decoded again, compared triple by triple."""

import re

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
    return [fi for fi in infos if fi.kind == kind and fi.area not in ("boundaryData", "commonData")]


def cgmes_profile(fi):
    match = re.search(r"/CIM/(\w+)-EU/", fi.profile_uris[0])
    return CGMES_PROFILE[match.group(1)]


def round_trip(path, profile):
    original = cimoxide.CimDataset.decode_file(str(harness.relicap_dir() / path))
    before = harness.cimoxide_triples(original.entries())
    after = harness.cimoxide_triples(
        cimoxide.CimDataset.decode_str(original.to_xml_for_profile(profile)).entries())
    return before - after, after - before


@pytest.mark.parametrize("fi", files("cgmes"), ids=lambda fi: fi.path)
def test_cgmes_round_trip(fi):
    lost, added = round_trip(fi.path, cgmes_profile(fi))
    # SSH writes the class it declares Equipment.inService on: an object the file
    # types BusbarSection or PowerTransformer, classes the SSH vocabulary lacks, comes
    # back as cim:Equipment, as the ENTSO-E test configurations write it.
    retyped = {(s, k, v) for (s, k, v) in lost if k == "Type"}
    assert {(s, k, "Equipment") for (s, k, _) in retyped} == set(added)
    assert set(lost) == retyped
    if retyped:
        assert cgmes_profile(fi) == "SSH"


@pytest.mark.xfail(strict=True, reason="cimoxide's encoder skips NC elements")
@pytest.mark.parametrize("fi", files("nc"), ids=lambda fi: fi.path)
def test_nc_round_trip(fi):
    code = harness.nc_profile_code(fi)
    lost, added = round_trip(fi.path, code)
    assert not lost and not added
