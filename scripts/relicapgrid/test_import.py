"""cimoxide's decoder against triplets, triple by triple, on every instance file."""

import pytest

import harness

# What cimoxide is known to read differently; anything else fails the test.
# - whitespace: cimoxide trims a literal's leading and trailing whitespace.
# - empty: cimoxide drops an element with empty text.
EXPECTED_KINDS = {"whitespace", "empty"}

# Files whose other differences are known: cimoxide keeps CIM/NC classes and
# dataset headers only, so SKOS, PROV and DCAT objects are dropped.
DROPS_NON_CIM = {
    "Instance/Svedala/NetworkCode/cimxml/Svedala_FAP_PV.xml",                 # prov:Activity
    "Instance/referenceData/NetworkCode/cimxml/Activity-Collection_RD.xml",
    "Instance/referenceData/NetworkCode/cimxml/ConfidentialityScheme_RD.xml",
    "Instance/referenceData/NetworkCode/cimxml/ConformTo-Collection-NCP-2-5_RD.xml",
    "Instance/referenceData/NetworkCode/cimxml/LicenseScheme_RD.xml",
    "Instance/referenceData/NetworkCode/cimxml/PropertyReference-ReferenceData.xml",
    "Instance/referenceData/NetworkCode/cimxml/Test-Frame-Collection-ReliCapGrid_RD.xml",
    "Instance/referenceData/NetworkCode/cimxml/Test-Model-Collection-ReliCapGrid_RD.xml",
    "Instance/referenceData/NetworkCode/cimxml/Test-Party-Collection-ReliCapGrid_RD.xml",
}


def files():
    try:
        return harness.all_files()
    except RuntimeError:
        return []


@pytest.mark.parametrize("path", files())
def test_import_matches_triplets(path):
    kinds = harness.classify(harness.import_diff(path))
    other = kinds.get("other", [])
    if path in DROPS_NON_CIM:
        assert other, "cimoxide now keeps this file's non-CIM objects; drop it from DROPS_NON_CIM"
        assert all(side == "-" for side, *_ in other), [t for t in other if t[0] == "+"][:5]
        pytest.xfail("cimoxide drops SKOS/PROV/DCAT objects")
    assert not other, other[:10]
    assert set(kinds) <= EXPECTED_KINDS
