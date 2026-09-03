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


def test_count_type_matches_by_type():
    ds = cimoxide.decode_file(td("test_shacl_EQ_001.xml"))
    for type_name, mrids in ds.by_type().items():
        assert ds.count_type(type_name) == len(mrids)


def test_count_type_unknown_returns_zero():
    ds = cimoxide.decode_file(td("test_shacl_EQ_001.xml"))
    assert ds.count_type("NoSuchCimType") == 0


def test_count_type_tracks_mutation():
    ds = cimoxide.decode_file(td("test_shacl_EQ_001.xml"))
    before = ds.count_type("ACLineSegment")
    del ds["ACLineSegment.OK"]
    assert ds.count_type("ACLineSegment") == before - 1


def test_decode_str_bad_xml_raises():
    with pytest.raises(Exception):
        cimoxide.decode_str("not xml at all <<<")


def test_decode_file_missing_raises():
    with pytest.raises(Exception):
        cimoxide.decode_file("/nonexistent/path/to/file.xml")


# ── Mutation: __setitem__ / __delitem__ ──────────────────────────────────────


def test_setitem_updates_existing_field():
    ds = cimoxide.decode_file(td("test_shacl_EQ_001.xml"))
    obj = ds["ACLineSegment.OK"]
    obj["r"] = 999.5
    ds["ACLineSegment.OK"] = obj
    assert ds["ACLineSegment.OK"]["r"] == 999.5


def test_setitem_new_mrid_adds_entry():
    ds = cimoxide.decode_file(td("test_shacl_EQ_001.xml"))
    before = len(ds)
    obj = dict(ds["ACLineSegment.OK"])
    obj["id"] = "ACLineSegment.NEW"
    obj["m_rid"] = "ACLineSegment.NEW"
    ds["ACLineSegment.NEW"] = obj
    assert len(ds) == before + 1
    assert "ACLineSegment.NEW" in ds.by_type()["ACLineSegment"]


def test_delitem_removes_entry():
    ds = cimoxide.decode_file(td("test_shacl_EQ_001.xml"))
    before = len(ds)
    del ds["ACLineSegment.OK"]
    assert len(ds) == before - 1
    assert "ACLineSegment.OK" not in ds.by_type()["ACLineSegment"]
    with pytest.raises(KeyError):
        _ = ds["ACLineSegment.OK"]


def test_delitem_missing_raises_key_error():
    ds = cimoxide.decode_file(td("test_shacl_EQ_001.xml"))
    with pytest.raises(KeyError):
        del ds["nonexistent-mrid"]


def test_setitem_missing_type_raises_value_error():
    ds = cimoxide.decode_file(td("test_shacl_EQ_001.xml"))
    with pytest.raises(ValueError):
        ds["ACLineSegment.OK"] = {}


def test_setitem_unknown_type_raises_value_error():
    ds = cimoxide.decode_file(td("test_shacl_EQ_001.xml"))
    with pytest.raises(ValueError):
        ds["ACLineSegment.OK"] = {"_type": "NoSuchCimType"}


def test_setitem_then_encode_reflects_change():
    ds = cimoxide.decode_file(td("test_shacl_EQ_001.xml"))
    obj = ds["ACLineSegment.OK"]
    obj["r"] = 12345.5
    ds["ACLineSegment.OK"] = obj
    xml = ds.to_xml_for_profile("EQ")
    assert "12345.5" in xml
