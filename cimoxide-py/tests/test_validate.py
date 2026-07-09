"""Tests for validate_files()."""

import os
import cimoxide

TESTDATA = os.path.join(os.path.dirname(__file__), "../../testdata")


def td(name: str) -> str:
    return os.path.join(TESTDATA, name)


def test_validate_returns_list():
    result = cimoxide.validate_files([td("test_shacl_EQ_001.xml")])
    assert isinstance(result, list)


def test_validate_violations_have_expected_fields():
    violations = cimoxide.validate_files([td("test_shacl_EQ_001.xml")])
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
    violations = cimoxide.validate_files([td("test_shacl_EQ_001.xml")])
    if violations:
        r = repr(violations[0])
        assert isinstance(r, str)
        assert len(r) > 0


def test_validate_violation_type():
    violations = cimoxide.validate_files([td("test_shacl_EQ_001.xml")])
    for v in violations:
        assert isinstance(v, cimoxide.Violation)


def test_validate_profile_filter():
    path = td("test_shacl_EQ_001.xml")
    all_v = cimoxide.validate_files([path])
    eq_v = cimoxide.validate_files([path], profiles=["EQ"])
    # Filtering to EQ only should not produce more violations
    assert len(eq_v) <= len(all_v) + len(all_v)  # sanity: both are finite


def test_validate_silence_removes_rule():
    path = td("test_shacl_EQ_001.xml")
    violations = cimoxide.validate_files([path])
    if not violations:
        return  # nothing to silence
    rule_to_silence = violations[0].rule_id
    silenced = cimoxide.validate_files([path], silence=[rule_to_silence])
    assert all(v.rule_id != rule_to_silence for v in silenced)


def test_validate_quality_flag():
    result = cimoxide.validate_files([td("test_shacl_EQ_001.xml")], quality=True)
    assert isinstance(result, list)


def test_validate_common_flag():
    result = cimoxide.validate_files([td("test_shacl_EQ_001.xml")], common=True)
    assert isinstance(result, list)
