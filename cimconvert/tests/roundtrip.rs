use std::path::Path;

use cimdecoder::CimDataset;
use cimconvert::{dataset_from_json, dataset_to_json, dataset_to_xml, dataset_to_xml_for_profile};

fn test_xml_path() -> &'static Path {
    Path::new("../testdata/test_003.xml")
}

#[test]
fn json_shape() {
    let ds = CimDataset::decode_file(test_xml_path()).expect("decode failed");
    let json = dataset_to_json(&ds);
    let map = json.as_object().expect("root is object");

    // N0 is a TopologicalNode with name "N0"
    let n0 = &map["N0"];
    assert_eq!(n0["_type"].as_str().unwrap(), "TopologicalNode");
    assert_eq!(n0["name"].as_str().unwrap(), "N0");

    // Terminal.N0 MridRef field should be a plain string
    let t = &map["Terminal.N0"];
    assert_eq!(t["_type"].as_str().unwrap(), "Terminal");
    assert!(
        t["topological_node"].is_string(),
        "MridRef should serialize as string"
    );
    assert_eq!(t["topological_node"].as_str().unwrap(), "N0");
}

#[test]
fn json_to_dataset_mrid_set() {
    let ds = CimDataset::decode_file(test_xml_path()).expect("decode failed");
    let original_mrids: std::collections::HashSet<&str> =
        ds.entries.keys().map(String::as_str).collect();

    let json = serde_json::to_string(&dataset_to_json(&ds)).expect("serialize");
    let ds2 = dataset_from_json(&json).expect("from_json failed");

    let round_trip_mrids: std::collections::HashSet<&str> =
        ds2.entries.keys().map(String::as_str).collect();

    assert_eq!(
        original_mrids, round_trip_mrids,
        "MRID set must survive JSON round-trip"
    );
}

#[test]
fn xml_round_trip_mrid_count() {
    let ds = CimDataset::decode_file(test_xml_path()).expect("decode failed");
    let original_count = ds.entries.len();

    let json = serde_json::to_string(&dataset_to_json(&ds)).expect("serialize");
    let ds2 = dataset_from_json(&json).expect("from_json failed");
    let xml = dataset_to_xml(&ds2).expect("to_xml failed");

    let ds3 = CimDataset::decode_str(&xml).expect("decode round-trip XML failed");
    assert_eq!(
        ds3.entries.len(),
        original_count,
        "entry count must survive XML round-trip"
    );
}

#[test]
fn xml_output_structure() {
    let ds = CimDataset::decode_file(test_xml_path()).expect("decode failed");
    let json = serde_json::to_string(&dataset_to_json(&ds)).expect("serialize");
    let ds2 = dataset_from_json(&json).expect("from_json");
    let xml = dataset_to_xml(&ds2).expect("to_xml");

    assert!(xml.contains("xmlns:cim="), "must declare cim namespace");
    assert!(xml.contains("rdf:about="), "must use rdf:about for MRIDs");
    assert!(
        xml.contains("rdf:resource="),
        "must use rdf:resource for references"
    );
}

#[test]
fn numeric_fields_are_numbers() {
    let eq_path = Path::new("../testdata/test_sparql_EQ_001.xml");
    if !eq_path.exists() {
        return; // skip if testdata not present
    }
    let ds = CimDataset::decode_file(eq_path).expect("decode failed");
    let json = dataset_to_json(&ds);
    let map = json.as_object().expect("root is object");

    // BaseVoltage should have a numeric nominalVoltage field
    for (_mrid, obj) in map {
        if obj["_type"].as_str() == Some("BaseVoltage") {
            if let Some(v) = obj.get("nominal_voltage") {
                assert!(v.is_number(), "nominalVoltage must be a number, got {v}");
            }
            break;
        }
    }
}

// ── Profile-aware XML tests ──────────────────────────────────────────────────
//
// test_003.xml contains:
//   - Terminal.N0  (Terminal, EQ-primary, has Terminal.TopologicalNode → TP attr)
//   - N0           (TopologicalNode, SV-primary, has IdentifiedObject.name → EQ/SV attr)

#[test]
fn profile_tp_secondary_uses_rdf_about() {
    let ds = CimDataset::decode_file(test_xml_path()).expect("decode failed");
    let json = serde_json::to_string(&dataset_to_json(&ds)).expect("serialize");
    let ds2 = dataset_from_json(&json).expect("from_json");
    let xml = dataset_to_xml_for_profile(&ds2, "TP").expect("to_xml_for_profile failed");

    // Terminal is EQ-primary, so it appears as secondary in TP → rdf:about
    assert!(
        xml.contains("rdf:about=\"#Terminal.N0\""),
        "Terminal should use rdf:about in TP output, got:\n{xml}"
    );
    // Terminal.TopologicalNode is TP-primary → must appear
    assert!(
        xml.contains("Terminal.TopologicalNode"),
        "TP output must include Terminal.TopologicalNode, got:\n{xml}"
    );
    // No rdf:ID expected (nothing is TP-primary in this fixture)
    assert!(
        !xml.contains("rdf:ID="),
        "TP output must not contain rdf:ID for this fixture, got:\n{xml}"
    );
}

#[test]
fn profile_tp_excludes_eq_only_attrs() {
    let ds = CimDataset::decode_file(test_xml_path()).expect("decode failed");
    let json = serde_json::to_string(&dataset_to_json(&ds)).expect("serialize");
    let ds2 = dataset_from_json(&json).expect("from_json");
    let xml = dataset_to_xml_for_profile(&ds2, "TP").expect("to_xml_for_profile failed");

    // IdentifiedObject.name is EQ-primary — must not appear in TP output for secondary elements
    assert!(
        !xml.contains("IdentifiedObject.name"),
        "TP output must not include EQ-primary IdentifiedObject.name, got:\n{xml}"
    );
}

#[test]
fn profile_eq_skips_tp_only_terminal() {
    let ds = CimDataset::decode_file(test_xml_path()).expect("decode failed");
    let json = serde_json::to_string(&dataset_to_json(&ds)).expect("serialize");
    let ds2 = dataset_from_json(&json).expect("from_json");
    let xml = dataset_to_xml_for_profile(&ds2, "EQ").expect("to_xml_for_profile failed");

    // Terminal.N0 in this fixture only has Terminal.TopologicalNode (TP-primary attr)
    // → no EQ-relevant fields → the element is skipped entirely
    assert!(
        !xml.contains("Terminal.N0"),
        "EQ output must skip Terminal.N0 (no EQ fields), got:\n{xml}"
    );
}

#[test]
fn profile_sv_primary_uses_rdf_id() {
    let ds = CimDataset::decode_file(test_xml_path()).expect("decode failed");
    let json = serde_json::to_string(&dataset_to_json(&ds)).expect("serialize");
    let ds2 = dataset_from_json(&json).expect("from_json");
    let xml = dataset_to_xml_for_profile(&ds2, "SV").expect("to_xml_for_profile failed");

    // TopologicalNode is SV-primary and has IdentifiedObject.name (which is in SV origins)
    // → should appear with rdf:ID
    assert!(
        xml.contains("rdf:ID=\"N0\""),
        "SV output must emit TopologicalNode N0 with rdf:ID (no # prefix), got:\n{xml}"
    );
    // Must not contain rdf:about for N0
    assert!(
        !xml.contains("rdf:about=\"#N0\""),
        "SV output must use rdf:ID, not rdf:about, for primary N0, got:\n{xml}"
    );
}

#[test]
fn profile_sv_xml_parses_back() {
    let ds = CimDataset::decode_file(test_xml_path()).expect("decode failed");
    let json = serde_json::to_string(&dataset_to_json(&ds)).expect("serialize");
    let ds2 = dataset_from_json(&json).expect("from_json");
    let xml = dataset_to_xml_for_profile(&ds2, "SV").expect("to_xml_for_profile failed");

    CimDataset::decode_str(&xml).expect("SV profile XML must be parseable");
}

#[test]
fn profile_unknown_yields_empty_rdf() {
    let ds = CimDataset::decode_file(test_xml_path()).expect("decode failed");
    let json = serde_json::to_string(&dataset_to_json(&ds)).expect("serialize");
    let ds2 = dataset_from_json(&json).expect("from_json");
    let xml = dataset_to_xml_for_profile(&ds2, "UNKNOWN_PROFILE").expect("to_xml failed");

    // No elements should appear for an unknown profile
    assert!(
        !xml.contains("<cim:"),
        "unknown profile must yield empty RDF, got:\n{xml}"
    );
}

// ── FullModel header preservation ────────────────────────────────────────────
//
// PST_Type3_EQ.xml (from CGMES-Test-Configurations) has a real <md:FullModel> header
// with rdf:about="urn:uuid:7b5b1bad-bc28-644c-8416-bc3125789aa3" and a full set of
// Model.* fields, whose Model.profile matches PROFILE_URIS["EQ"] exactly.

fn pst_eq_path() -> &'static Path {
    Path::new("../CGMES-Test-Configurations/v3.0/PST/PST_PhaseTapChangerTable_Type3/PST_Type3_EQ.xml")
}

#[test]
fn full_model_header_preserved_when_present() {
    if !pst_eq_path().exists() {
        return; // skip if submodule not initialized
    }
    let ds = CimDataset::decode_file(pst_eq_path()).expect("decode failed");
    let json = serde_json::to_string(&dataset_to_json(&ds)).expect("serialize");
    let ds2 = dataset_from_json(&json).expect("from_json");
    let xml = dataset_to_xml_for_profile(&ds2, "EQ").expect("to_xml_for_profile failed");

    assert!(
        xml.contains("rdf:about=\"urn:uuid:7b5b1bad-bc28-644c-8416-bc3125789aa3\""),
        "must reuse the original FullModel rdf:about, got:\n{xml}"
    );
    assert!(
        xml.contains("<md:Model.scenarioTime>2021-05-03T05:00:00Z</md:Model.scenarioTime>"),
        "must reuse the original Model.scenarioTime, got:\n{xml}"
    );
    assert!(
        xml.contains("<md:Model.version>1</md:Model.version>"),
        "must reuse the original Model.version, got:\n{xml}"
    );
    assert!(
        !xml.contains("urn:uuid:cimoxide-EQ"),
        "must not fall back to the synthetic header when a real one is present, got:\n{xml}"
    );
}

#[test]
fn full_model_header_synthesized_when_absent() {
    if !pst_eq_path().exists() {
        return; // skip if submodule not initialized
    }
    // This dataset only has an EQ-profile FullModel entry, so requesting SSH must
    // fall back to the synthetic header rather than reusing the EQ one.
    let ds = CimDataset::decode_file(pst_eq_path()).expect("decode failed");
    let json = serde_json::to_string(&dataset_to_json(&ds)).expect("serialize");
    let ds2 = dataset_from_json(&json).expect("from_json");
    let xml = dataset_to_xml_for_profile(&ds2, "SSH").expect("to_xml_for_profile failed");

    assert!(
        xml.contains("urn:uuid:cimoxide-SSH"),
        "must fall back to the synthetic header, got:\n{xml}"
    );
    assert!(
        !xml.contains("7b5b1bad-bc28-644c-8416-bc3125789aa3"),
        "must not leak the EQ FullModel's mrid into the SSH header, got:\n{xml}"
    );
}

// ── Namespace prefixes and the EQBD profile ──────────────────────────────────
//
// These use the FullGrid boundary file from the CGMES-Test-Configurations submodule, which
// is the only fixture carrying eu:-namespaced classes and attributes. The decoder is
// namespace-blind (`local_name()` maps eu:X and cim:X to the same key), so no round-trip
// assertion can catch a wrong prefix — these check the emitted text directly.

fn eqbd_path() -> &'static Path {
    Path::new("../CGMES-Test-Configurations/v3.0/FullGrid/FullGrid-Merged/FullGrid_EQBD.xml")
}

fn decode_eqbd() -> CimDataset {
    let path = eqbd_path();
    assert!(
        path.exists(),
        "missing {} — run `git submodule update --init`",
        path.display()
    );
    CimDataset::decode_file(path).expect("decode failed")
}

/// `eu:` classes and attributes must keep their own namespace, and a `cim:` attribute on the
/// very same element must keep its own. The prefix is per field, never inherited.
#[test]
fn eu_namespace_is_preserved() {
    let ds = decode_eqbd();
    let xml = dataset_to_xml_for_profile(&ds, "EQBD").expect("to_xml_for_profile failed");

    assert!(xml.contains("<eu:BoundaryPoint "), "eu: class prefix lost:\n{xml}");
    assert!(!xml.contains("<cim:BoundaryPoint "), "eu: class written as cim:");

    assert!(
        xml.contains("<eu:BoundaryPoint.toEndName>"),
        "eu: attribute prefix lost"
    );
    assert!(
        !xml.contains("<cim:BoundaryPoint.toEndName>"),
        "eu: attribute leaked into the cim: namespace"
    );
    assert!(
        xml.contains("<eu:IdentifiedObject.shortName>"),
        "eu: attribute on a cim: class lost its prefix"
    );

    // ...while cim: attributes on those same eu: elements stay cim:.
    assert!(
        xml.contains("<cim:IdentifiedObject.description>"),
        "cim: attribute wrongly moved out of the cim: namespace"
    );
}

/// Real CGMES writes enum values as absolute IRIs, not local fragments.
#[test]
fn enum_values_are_absolute_iris() {
    let ds = CimDataset::decode_file(Path::new(
        "../CGMES-Test-Configurations/v3.0/FullGrid/FullGrid-Merged/FullGrid_EQ.xml",
    ))
    .expect("decode failed");
    let xml = dataset_to_xml_for_profile(&ds, "EQ").expect("to_xml_for_profile failed");

    assert!(
        xml.contains("rdf:resource=\"http://iec.ch/TC57/CIM100#UnitSymbol."),
        "enum value should be an absolute IRI"
    );
    assert!(
        !xml.contains("rdf:resource=\"#UnitSymbol."),
        "enum value still written as a local fragment"
    );
    // Ordinary MRID references keep the local fragment form.
    assert!(xml.contains("rdf:resource=\"#"), "MRID references should stay local");
}

/// EQBD is the dominant origin of no attribute, so the secondary-element rule selected
/// nothing and the whole profile exported as a bare header.
#[test]
fn eqbd_exports_its_elements() {
    let ds = decode_eqbd();
    let xml = dataset_to_xml_for_profile(&ds, "EQBD").expect("to_xml_for_profile failed");
    let ds2 = cimdecoder::CimDataset::decode_str(&xml).expect("re-decode failed");

    assert_eq!(
        ds2.entries.len(),
        ds.entries.len(),
        "EQBD round-trip lost entries: {} -> {}",
        ds.entries.len(),
        ds2.entries.len()
    );
    assert_eq!(
        ds2.by_type.get("BoundaryPoint").map_or(0, Vec::len),
        ds.by_type.get("BoundaryPoint").map_or(0, Vec::len),
    );
    // Real boundary files define their objects outright — all rdf:ID, no rdf:about.
    assert!(xml.contains("rdf:ID="), "EQBD elements should be definitions");
    assert!(!xml.contains("rdf:about=\"#"), "EQBD should not emit references");
}

/// The EQBD fallback must stay inert for every other profile: they are each the dominant
/// origin of at least one attribute, so their existing behaviour is untouched.
#[test]
fn every_profile_round_trips_its_own_file() {
    let base = Path::new("../CGMES-Test-Configurations/v3.0/FullGrid/FullGrid-Merged");
    if !base.exists() {
        panic!("missing {} — run `git submodule update --init`", base.display());
    }
    for profile in ["EQ", "SSH", "TP", "SV", "OP", "SC", "EQBD"] {
        let path = base.join(format!("FullGrid_{profile}.xml"));
        let ds = CimDataset::decode_file(&path).expect("decode failed");
        let xml = dataset_to_xml_for_profile(&ds, profile).expect("encode failed");
        let ds2 = cimdecoder::CimDataset::decode_str(&xml).expect("re-decode failed");
        assert_eq!(
            ds2.entries.len(),
            ds.entries.len(),
            "{profile}: {} entries in, {} out",
            ds.entries.len(),
            ds2.entries.len()
        );
    }
}

/// Elements inserted after decoding carry no history, so routing has to come from the
/// schema. The JSON hop rebuilds every field through `to_block()`, exercising that path.
#[test]
fn json_hop_exports_identically() {
    let ds = decode_eqbd();
    let direct = dataset_to_xml_for_profile(&ds, "EQBD").expect("encode failed");

    let json = serde_json::to_string(&dataset_to_json(&ds)).expect("serialize");
    let ds2 = dataset_from_json(&json).expect("from_json");
    let via_json = dataset_to_xml_for_profile(&ds2, "EQBD").expect("encode failed");

    assert_eq!(direct, via_json, "JSON round-trip changed the EQBD export");
}
