"""Tests for encoding a CimDataset back to CGMES profile XML."""

import os
import cimoxide

TESTDATA = os.path.join(os.path.dirname(__file__), "../../testdata")
PST_EQ = os.path.join(
    os.path.dirname(__file__),
    "..",
    "..",
    "CGMES-Test-Configurations",
    "v3.0",
    "PST",
    "PST_PhaseTapChangerTable_Type3",
    "PST_Type3_EQ.xml",
)


def td(name: str) -> str:
    return os.path.join(TESTDATA, name)


def test_to_xml_for_profile_structure():
    ds = cimoxide.decode_file(td("test_shacl_EQ_001.xml"))
    xml = ds.to_xml_for_profile("EQ")
    assert "xmlns:cim=" in xml
    assert "rdf:about=" in xml or "rdf:ID=" in xml
    assert "rdf:resource=" in xml


def test_to_xml_for_profile_round_trip():
    ds = cimoxide.decode_file(td("test_shacl_EQ_001.xml"))
    xml = ds.to_xml_for_profile("EQ")
    ds2 = cimoxide.decode_str(xml)
    assert len(ds2) > 0


def test_to_xml_for_profile_unknown_yields_empty_rdf():
    ds = cimoxide.decode_file(td("test_shacl_EQ_001.xml"))
    xml = ds.to_xml_for_profile("UNKNOWN_PROFILE")
    assert "<cim:" not in xml


def test_write_xml_files_writes_and_reloads(tmp_path):
    ds = cimoxide.decode_file(td("test_shacl_EQ_001.xml"))
    ds.write_xml_files(str(tmp_path), ["EQ", "SSH"])

    eq_path = tmp_path / "EQ.xml"
    ssh_path = tmp_path / "SSH.xml"
    assert eq_path.exists()
    assert ssh_path.exists()

    ds_eq = cimoxide.decode_file(str(eq_path))
    assert len(ds_eq) > 0


def test_write_xml_files_creates_missing_dir(tmp_path):
    ds = cimoxide.decode_file(td("test_shacl_EQ_001.xml"))
    out_dir = tmp_path / "nested" / "out"
    ds.write_xml_files(str(out_dir), ["EQ"])
    assert (out_dir / "EQ.xml").exists()


def test_full_model_header_preserved():
    if not os.path.exists(PST_EQ):
        return  # skip if CGMES-Test-Configurations submodule not initialized

    ds = cimoxide.decode_file(PST_EQ)
    xml = ds.to_xml_for_profile("EQ")

    assert 'rdf:about="urn:uuid:7b5b1bad-bc28-644c-8416-bc3125789aa3"' in xml
    assert "<md:Model.scenarioTime>2021-05-03T05:00:00Z</md:Model.scenarioTime>" in xml
    assert "<md:Model.version>1</md:Model.version>" in xml
    assert "urn:uuid:cimoxide-EQ" not in xml


def test_full_model_header_synthesized_when_absent():
    if not os.path.exists(PST_EQ):
        return  # skip if CGMES-Test-Configurations submodule not initialized

    # This dataset only decoded the EQ file, so it has no SSH FullModel entry.
    ds = cimoxide.decode_file(PST_EQ)
    xml = ds.to_xml_for_profile("SSH")

    assert "urn:uuid:cimoxide-SSH" in xml
    assert "7b5b1bad-bc28-644c-8416-bc3125789aa3" not in xml
