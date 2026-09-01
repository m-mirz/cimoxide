"""SPARQL querying over a decoded dataset."""

import os
import pytest
import cimoxide

FULLGRID = os.path.join(
    os.path.dirname(__file__), "../../CGMES-Test-Configurations/v3.0/FullGrid/FullGrid-Merged"
)


def fg(name: str) -> str:
    return os.path.join(FULLGRID, name)


@pytest.fixture(scope="module")
def eq():
    if not os.path.exists(fg("FullGrid_EQ.xml")):
        pytest.skip("CGMES-Test-Configurations submodule not initialised")
    return cimoxide.decode_file(fg("FullGrid_EQ.xml"))


def test_select_returns_dicts(eq):
    rows = eq.query(
        "SELECT ?name ?r WHERE { ?s a cim:ACLineSegment ;"
        " cim:IdentifiedObject.name ?name ; cim:ACLineSegment.r ?r } ORDER BY ?name"
    )
    assert rows, "expected ACLineSegments in FullGrid"
    assert all(set(row) == {"name", "r"} for row in rows)
    # Values come back as plain strings, not N-Triples literals.
    assert all(not row["r"].startswith('"') for row in rows)


def test_select_agrees_with_by_type(eq):
    rows = eq.query("SELECT ?s WHERE { ?s a cim:ACLineSegment }")
    assert len(rows) == len(eq.by_type()["ACLineSegment"])


def test_ask_returns_bool(eq):
    assert eq.query("ASK { ?s a cim:ACLineSegment }") is True
    assert eq.query("ASK { ?s a cim:NoSuchClass }") is False


def test_construct_returns_triples(eq):
    triples = eq.query("CONSTRUCT { ?s a cim:Line } WHERE { ?s a cim:ACLineSegment }")
    assert len(triples) == len(eq.by_type()["ACLineSegment"])
    subject, predicate, obj = triples[0]
    # IRIs are bare, with no angle brackets, in every position.
    for value in (subject, predicate, obj):
        assert not value.startswith("<")
    assert predicate.endswith("#type")
    assert obj == "http://iec.ch/TC57/CIM100#Line"


def test_namespaces_are_prebound(eq):
    """cim:, eu:, md: and xsd: resolve without a prologue."""
    assert eq.query("ASK { ?s a md:FullModel }") is True
    # eu:-namespaced attributes must not be reachable under cim:.
    assert eq.query("ASK { ?s cim:IdentifiedObject.energyIdentCodeEic ?v }") is False
    assert eq.query("ASK { ?s eu:IdentifiedObject.energyIdentCodeEic ?v }") is True


def test_literals_are_numerically_typed(eq):
    rows = eq.query(
        "SELECT ?t WHERE { ?s cim:ACLineSegment.r ?r BIND(DATATYPE(?r) AS ?t) } LIMIT 1"
    )
    assert rows[0]["t"] == "http://www.w3.org/2001/XMLSchema#double"


def test_invalid_query_raises(eq):
    with pytest.raises(Exception):
        eq.query("SELECT ?s WHERE {")
