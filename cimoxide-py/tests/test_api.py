"""API contract tests: merge, drop_blocks, iteration, error handling."""

import os
import pytest
import cimoxide

TESTDATA = os.path.join(os.path.dirname(__file__), "../../testdata")


def td(name: str) -> str:
    return os.path.join(TESTDATA, name)


def test_merge_combines_datasets():
    ds1 = cimoxide.decode_file(td("test_009_EQ.xml"))
    ds2 = cimoxide.decode_file(td("test_009_TP.xml"))
    expected_mrids = set(ds1.mrids()) | set(ds2.mrids())
    ds1.merge(ds2)
    # After merge: ds1 contains the union of both MRID sets, ds2 is empty.
    assert set(ds1.mrids()) == expected_mrids
    assert len(ds2) == 0


def test_drop_blocks_doesnt_lose_entries():
    ds = cimoxide.decode_file(td("test_shacl_EQ_001.xml"))
    count = len(ds)
    ds.drop_blocks()
    assert len(ds) == count
    for mrid in ds:
        obj = ds[mrid]
        assert "_type" in obj


def test_iteration_covers_all():
    ds = cimoxide.decode_file(td("test_shacl_EQ_001.xml"))
    seen = set()
    for mrid in ds:
        seen.add(mrid)
    assert seen == set(ds.mrids())


def test_get_returns_none_for_missing():
    ds = cimoxide.decode_file(td("test_shacl_EQ_001.xml"))
    assert ds.get("nonexistent-mrid") is None


def test_get_returns_dict_for_present():
    ds = cimoxide.decode_file(td("test_shacl_EQ_001.xml"))
    mrid = next(iter(ds))
    obj = ds.get(mrid)
    assert obj is not None
    assert "_type" in obj


def test_get_type_unknown_returns_empty():
    ds = cimoxide.decode_file(td("test_shacl_EQ_001.xml"))
    assert ds.get_type("NoSuchCimType") == []


def test_decode_str_bad_xml_raises():
    with pytest.raises(Exception):
        cimoxide.decode_str("not xml at all <<<")


def test_decode_file_missing_raises():
    with pytest.raises(Exception):
        cimoxide.decode_file("/nonexistent/path/to/file.xml")
