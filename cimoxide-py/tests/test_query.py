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


# ── Store caching: the graph is built once and dropped on mutation ───────────

TESTDATA = os.path.join(os.path.dirname(__file__), "../../testdata")


def td(name: str) -> str:
    return os.path.join(TESTDATA, name)


LINES = "SELECT ?s WHERE { ?s a cim:ACLineSegment }"


def line_count(ds) -> int:
    return len(ds.query(LINES))


def test_repeated_query_is_stable():
    """Second query hits the cache and must agree with the first."""
    ds = cimoxide.decode_file(td("test_shacl_EQ_001.xml"))
    assert ds.query(LINES) == ds.query(LINES)


def test_query_cache_sees_setitem():
    ds = cimoxide.decode_file(td("test_shacl_EQ_001.xml"))
    before = line_count(ds)  # populates the cache
    obj = dict(ds["ACLineSegment.OK"])
    obj["id"] = obj["m_rid"] = "ACLineSegment.NEW"
    ds["ACLineSegment.NEW"] = obj
    assert line_count(ds) == before + 1


def test_query_cache_sees_delitem():
    ds = cimoxide.decode_file(td("test_shacl_EQ_001.xml"))
    before = line_count(ds)
    del ds["ACLineSegment.OK"]
    assert line_count(ds) == before - 1


def test_query_cache_sees_merge():
    """TopologicalNode exists only in the TP profile, so it must appear in the
    EQ dataset's results only after the merge -- i.e. the cache was dropped."""
    ds1 = cimoxide.decode_file(td("test_009_EQ.xml"))
    ds2 = cimoxide.decode_file(td("test_009_TP.xml"))
    assert ds1.query("ASK { ?s a cim:TopologicalNode }") is False  # populates cache
    ds1.merge(ds2)
    assert ds1.query("ASK { ?s a cim:TopologicalNode }") is True


def test_merge_invalidates_source():
    """merge() empties `other`, so its cached graph must go too."""
    ds1 = cimoxide.decode_file(td("test_009_EQ.xml"))
    ds2 = cimoxide.decode_file(td("test_009_TP.xml"))
    assert ds2.query("ASK { ?s ?p ?o }") is True  # populates ds2's cache
    ds1.merge(ds2)
    assert len(ds2) == 0
    assert ds2.query("ASK { ?s ?p ?o }") is False


def test_drop_sparql_store_then_query():
    ds = cimoxide.decode_file(td("test_shacl_EQ_001.xml"))
    first = ds.query(LINES)
    ds.drop_sparql_store()
    assert ds.query(LINES) == first
    ds.drop_sparql_store()  # idempotent
    ds.drop_sparql_store()
