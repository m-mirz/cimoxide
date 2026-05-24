"""Round-trip tests against the checked-in testdata/ XML files."""

import os
import pytest
import cimoxide

TESTDATA = os.path.join(os.path.dirname(__file__), "../../testdata")


def td(name: str) -> str:
    return os.path.join(TESTDATA, name)


def test_decode_file_returns_dataset():
    ds = cimoxide.decode_file(td("test_shacl_EQ_001.xml"))
    assert len(ds) > 0


def test_decode_str_matches_file():
    path = td("test_shacl_EQ_001.xml")
    ds_file = cimoxide.decode_file(path)
    with open(path) as f:
        ds_str = cimoxide.decode_str(f.read())
    assert len(ds_file) == len(ds_str)


def test_decode_files_merges():
    paths = [td("test_009_EQ.xml"), td("test_009_TP.xml")]
    ds = cimoxide.decode_files(paths)
    assert len(ds) > 0


def test_by_type_index():
    ds = cimoxide.decode_file(td("test_shacl_EQ_001.xml"))
    idx = ds.by_type()
    assert isinstance(idx, dict)
    for type_name, mrids in idx.items():
        assert isinstance(type_name, str)
        assert isinstance(mrids, list)
        assert all(isinstance(m, str) for m in mrids)


def test_getitem_has_type_key():
    ds = cimoxide.decode_file(td("test_shacl_EQ_001.xml"))
    for mrid in ds:
        obj = ds[mrid]
        assert "_type" in obj
        assert isinstance(obj["_type"], str)
        break


def test_getitem_missing_raises_key_error():
    ds = cimoxide.decode_file(td("test_shacl_EQ_001.xml"))
    with pytest.raises(KeyError):
        _ = ds["nonexistent-mrid"]


def test_contains():
    ds = cimoxide.decode_file(td("test_shacl_EQ_001.xml"))
    mrid = next(iter(ds))
    assert mrid in ds
    assert "nonexistent-mrid" not in ds


def test_get_type_subset():
    ds = cimoxide.decode_file(td("test_shacl_EQ_001.xml"))
    idx = ds.by_type()
    for type_name in list(idx.keys())[:3]:
        objects = ds.get_type(type_name)
        assert len(objects) == len(idx[type_name])
        for obj in objects:
            assert obj["_type"] == type_name


def test_entries_full_dict():
    ds = cimoxide.decode_file(td("test_shacl_EQ_001.xml"))
    all_entries = ds.entries()
    assert len(all_entries) == len(ds)
    for mrid, obj in all_entries.items():
        assert "_type" in obj


def test_mrids_list():
    ds = cimoxide.decode_file(td("test_shacl_EQ_001.xml"))
    mrids = ds.mrids()
    assert isinstance(mrids, list)
    assert len(mrids) == len(ds)
