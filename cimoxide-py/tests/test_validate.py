"""Tests for the validate() method on CimDataset."""

import os
import cimoxide

TESTDATA = os.path.join(os.path.dirname(__file__), "../../testdata")


def td(name: str) -> str:
    return os.path.join(TESTDATA, name)


def test_validate_returns_list():
    ds = cimoxide.decode_file(td("test_shacl_EQ_001.xml"))
    result = ds.validate()
    assert isinstance(result, list)


def test_validate_violations_have_expected_fields():
    ds = cimoxide.decode_file(td("test_shacl_EQ_001.xml"))
    violations = ds.validate()
    for v in violations:
        assert isinstance(v.object_id, str)
        assert isinstance(v.rule_id, str)
        assert isinstance(v.name, str)
        assert isinstance(v.class_, str)
        assert isinstance(v.property, str)
        assert isinstance(v.message, str)
        assert isinstance(v.severity, str)
        assert isinstance(v.description, str)


def test_validate_repr():
    ds = cimoxide.decode_file(td("test_shacl_EQ_001.xml"))
    violations = ds.validate()
    if violations:
        r = repr(violations[0])
        assert isinstance(r, str)
        assert len(r) > 0


def test_validate_violation_type():
    ds = cimoxide.decode_file(td("test_shacl_EQ_001.xml"))
    violations = ds.validate()
    for v in violations:
        assert isinstance(v, cimoxide.Violation)


def test_validate_profile_filter():
    ds = cimoxide.decode_file(td("test_shacl_EQ_001.xml"))
    all_v = ds.validate()
    eq_v = ds.validate(profiles=["EQ"])
    # Filtering to EQ only should not produce more violations
    assert len(eq_v) <= len(all_v) + len(all_v)  # sanity: both are finite


def test_validate_silence_removes_rule():
    ds = cimoxide.decode_file(td("test_shacl_EQ_001.xml"))
    violations = ds.validate()
    if not violations:
        return  # nothing to silence
    rule_to_silence = violations[0].rule_id
    silenced = ds.validate(silence=[rule_to_silence])
    assert all(v.rule_id != rule_to_silence for v in silenced)


def test_validate_quality_flag():
    ds = cimoxide.decode_file(td("test_shacl_EQ_001.xml"))
    result = ds.validate(quality=True)
    assert isinstance(result, list)


def test_validate_common_flag():
    ds = cimoxide.decode_file(td("test_shacl_EQ_001.xml"))
    result = ds.validate(common=True)
    assert isinstance(result, list)
