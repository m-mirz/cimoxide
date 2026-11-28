//! The profiles' SPARQL constraints, run as written, against cimoxide's hand-written rules.
//!
//! Both run on the hand-written rules' fixtures and on the CGMES conformance sets, and their
//! findings are compared by rule and object. A rule where they differ fails the test unless
//! [`KNOWN`] gives the reason, and an entry that no longer differs fails it too, so the table
//! stays the list of what is true.

#![cfg(feature = "sparql-shapes")]

use std::collections::{BTreeMap, BTreeSet, HashSet};
use std::path::{Path, PathBuf};

use cimmodel::CimDataset;
use cimvalidation::sparql_shapes::{self, SparqlRules};

/// Where the two differ, and why. `true` marks a difference only RealGrid or FullGrid shows.
const KNOWN: &[(&str, bool, &str)] = &[
    // The query cannot report what its description says.
    (
        "svs456:SvVoltage.v-absoluteLimit",
        false,
        "reads cim:SvVoltage.ToplogicalNode (sic), which no data carries",
    ),
    (
        "svs456:SvVoltage.v-limits",
        false,
        "reads cim:SvVoltage.ToplogicalNode (sic), which no data carries",
    ),
    (
        "sshn301:TapChanger.step-valueType",
        false,
        "a whole number is tested as STRENDS(str(?value), \".\"), which no number meets",
    ),
    (
        "sshn456:ShuntCompensator.sections-value",
        false,
        "a whole number is tested as STRENDS(str(?value), \".\"), which no number meets",
    ),
    (
        "sshn456:TapChanger.step-value",
        false,
        "a whole number is tested as STRENDS(str(?value), \".\"), which no number meets",
    ),
    (
        "svs456:SvShuntCompensatorSections.sections-value",
        false,
        "a whole number is tested as STRENDS(str(?value), \".\"), which no number meets",
    ),
    (
        "svs456:SvTapStep.position-value",
        false,
        "a whole number is tested as STRENDS(str(?value), \".\"), which no number meets",
    ),
    (
        "equ:OperationalLimitType.isInfiniteDuration-usage",
        false,
        "requires the path's value, then filters on there being none",
    ),
    (
        "eq600:ReactiveCapabilityCurve-units",
        false,
        "requires a y2Unit, then filters on there being none",
    ),
    (
        "all600:All-GENC5",
        false,
        "filters on an id longer than 59 characters that is also empty",
    ),
    (
        "all600:Float-specialValues",
        false,
        "tests isLiteral($this), and $this is the placeholder node cim:FloatSpecialValues",
    ),
    (
        "equ:Terminal.phases-consistencyConnectivityNode",
        false,
        "groups by the terminal it counts, so HAVING (?terms>1) never holds",
    ),
    (
        "tpn301:Terminal.phases-consistencyTopologicalNode",
        false,
        "groups by the terminal it counts, so HAVING (?terms>1) never holds",
    ),
    (
        "tpn456:Switch-sameTopologicalNode",
        false,
        "requires both terminals' ConnectivityNodes even where it compares their \
         TopologicalNodes",
    ),
    (
        "sm600:SvTapStep.position-SV__4",
        true,
        "steps cim:TransformerEnd.PowerTransformer, which CGMES 3.0 does not define",
    ),
    (
        "opn452:Measurement.Terminal-requiredCases",
        false,
        "checks only measurements whose equipment has a terminal numbered 1",
    ),
    // Topology read through ConnectivityNodes only: bus-branch data has none.
    (
        "sm600:SvShuntCompensatorSections-SV__4",
        false,
        "reaches islands only through ConnectivityNodes; the fixture is bus-branch",
    ),
    (
        "sm600:SvStatus-SV__4",
        false,
        "reaches islands only through ConnectivityNodes; the fixture is bus-branch",
    ),
    (
        "svs456:SvPowerFlow-instance",
        false,
        "reaches islands only through ConnectivityNodes; the fixture is bus-branch",
    ),
    (
        "sm6002:RegulatingControl-point",
        false,
        "reaches islands only through ConnectivityNodes; the fixture is bus-branch",
    ),
    (
        "sm456:Model-angleReference",
        false,
        "counts the model's reference machines regardless of its IGMs, so a merged model \
         with one per IGM fails it, and reaches islands only through ConnectivityNodes",
    ),
    // The same check, read differently.
    (
        "sm6002:RegulatingControl-samePoint",
        false,
        "reports the TopologicalNode; the hand-written rule each control on it, the \
         query's ?rcfail",
    ),
    (
        "dyn457:TurbineGovernorDynamics-mbaseEquation",
        false,
        "its file is NotSolvedMAS, so it runs on unsolved models only; the hand-written \
         rule in either state",
    ),
    (
        "eq600:Substation-count",
        false,
        "the hand-written rule counts per EQ file, the query over the union of the model",
    ),
    (
        "all600:All-GENC1",
        false,
        "an id written twice is one element to the decoder and one node in the union, so \
         the query cannot see the duplicate",
    ),
];

fn root() -> &'static Path {
    Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap()
}

fn rules() -> SparqlRules {
    sparql_shapes::from_dirs(&[root().join("CGMES-AP"), root().join("NC-AP")])
        .expect("reading the profiles")
}

/// The findings, by rule: `(hand-written, SPARQL)` objects, each as `run|object`, so a
/// finding of one run does not stand in for another's.
type Findings = BTreeMap<String, (BTreeSet<String>, BTreeSet<String>)>;

fn compare(
    run: &str,
    per_file: &[CimDataset],
    cfg: &cimvalidation::Config,
    rules: &SparqlRules,
    hand: Vec<cimvalidation::Violation>,
    into: &mut Findings,
) {
    let ids: HashSet<&str> = rules
        .rules
        .iter()
        .map(|r| r.constraint.rule_id.as_str())
        .collect();
    let report = sparql_shapes::validate_files(per_file, cfg, rules);
    assert!(report.failed.is_empty(), "{:?}", report.failed);
    for v in hand
        .into_iter()
        .filter(|v| ids.contains(v.rule_id.as_str()))
    {
        into.entry(v.rule_id)
            .or_default()
            .0
            .insert(format!("{run}|{}", v.object_id));
    }
    for (_, v) in report.violations {
        into.entry(v.rule_id)
            .or_default()
            .1
            .insert(format!("{run}|{}", v.object_id));
    }
}

/// The hand-written rules' fixtures, each a model of its own, unsolved and solved. A fixture
/// without a header takes its profile from its name (`test_sparql_EQ_001` → `EQ`).
fn fixtures(rules: &SparqlRules, into: &mut Findings) {
    let mut paths: Vec<PathBuf> = std::fs::read_dir(root().join("testdata"))
        .unwrap()
        .filter_map(|e| e.ok().map(|e| e.path()))
        .filter(|p| {
            p.extension().is_some_and(|x| x == "xml") && !p.to_string_lossy().ends_with(".out.xml")
        })
        .collect();
    paths.sort();
    for path in paths {
        let Ok(ds) = CimDataset::decode_file(&path) else {
            continue;
        };
        let set = vec![ds];
        for solved in [false, true] {
            let mut cfg =
                cimvalidation::combined_config(&set, None, Some(solved), true, false, vec![]);
            if cfg.profiles.is_empty() {
                let stem = path.file_stem().unwrap().to_string_lossy().into_owned();
                cfg.profiles = stem
                    .split('_')
                    .nth(2)
                    .map(String::from)
                    .into_iter()
                    .collect();
            }
            let mut hand = Vec::new();
            for profile in &cfg.profiles {
                hand.extend(cimvalidation::validate_profile_local(
                    &set[0], profile, &cfg,
                ));
            }
            hand.extend(cimvalidation::validate_crossprofile(&set[0], &cfg));
            let run = format!(
                "{} solved={solved}",
                path.file_name().unwrap().to_string_lossy()
            );
            compare(&run, &set, &cfg, rules, hand, into);
        }
    }
}

/// A conformance set, every file of its directory, with the configuration its headers give.
fn conformance(set: &str, rules: &SparqlRules, into: &mut Findings) {
    let dir = root().join("CGMES-Test-Configurations/v3.0").join(set);
    let mut paths: Vec<PathBuf> = std::fs::read_dir(&dir)
        .unwrap_or_else(|e| panic!("{}: {e}", dir.display()))
        .filter_map(|e| e.ok().map(|e| e.path()))
        .filter(|p| p.extension().is_some_and(|x| x == "xml"))
        .collect();
    paths.sort();
    let per_file: Vec<CimDataset> = paths
        .iter()
        .map(|p| CimDataset::decode_file(p).unwrap())
        .collect();
    let cfg = cimvalidation::combined_config(&per_file, None, None, true, false, vec![]);
    let hand = cimvalidation::validate_files(per_file.clone(), &cfg);
    compare(set, &per_file, &cfg, rules, hand, into);
}

/// The rules where the two differ.
fn differing(findings: &Findings) -> BTreeSet<&str> {
    findings
        .iter()
        .filter(|(_, (hand, sparql))| hand != sparql)
        .map(|(rule, _)| rule.as_str())
        .collect()
}

fn check(findings: &Findings, large: bool) {
    let differ = differing(findings);
    let known: BTreeSet<&str> = KNOWN.iter().map(|(r, _, _)| *r).collect();
    let new: Vec<String> = differ
        .iter()
        .filter(|r| !known.contains(*r))
        .map(|r| {
            let (hand, sparql) = &findings[*r];
            format!(
                "{r}: hand-written only {:?}, SPARQL only {:?}",
                hand.difference(sparql).take(3).collect::<Vec<_>>(),
                sparql.difference(hand).take(3).collect::<Vec<_>>()
            )
        })
        .collect();
    assert!(
        new.is_empty(),
        "differences without a reason:\n{}",
        new.join("\n")
    );
    let stale: Vec<&str> = KNOWN
        .iter()
        .filter(|(rule, only_large, _)| *only_large == large && !differ.contains(rule))
        .map(|(rule, _, _)| *rule)
        .collect();
    assert!(
        stale.is_empty(),
        "known divergences that no longer occur: {stale:?}"
    );
}

/// Every constraint in ENTSO-E's files is read and prepared.
#[test]
fn every_constraint_is_prepared() {
    let rules = rules();
    assert_eq!(rules.rules.len(), 220);
    assert!(rules.skipped.is_empty(), "{:?}", rules.skipped);
}

#[test]
fn the_queries_and_the_hand_written_rules_agree_but_where_known() {
    let rules = rules();
    let mut findings = Findings::new();
    fixtures(&rules, &mut findings);
    for set in [
        "MicroGrid/MicroGid-BaseCase/MicroGrid-BaseCase-Merged",
        "MicroGrid/MicroGrid-Type1/MicroGrid-Type1-Merged",
        "MicroGrid/MicroGrid-Type2/MicroGrid-Type2-Merged",
        "MiniGrid/MiniGrid-Merged",
        "SmallGrid/SmallGrid-Merged",
        "Svedala/Svedala-Merged",
        "PST/PST_PhaseTapChangerLinear_Type1",
        "PST/PST_PhaseTapChangerLinear_Type2",
        "PST/PST_PhaseTapChangerTable_Type3",
        "PowerFlow/PowerFlow",
    ] {
        conformance(set, &rules, &mut findings);
    }
    // Most rules find nothing in this corpus; those that do, agree.
    let agree = findings.len() - differing(&findings).len();
    assert!(agree >= 50, "only {agree} rules agree");
    check(&findings, false);
}

/// RealGrid and FullGrid. Ignored for their size: run with `--release -- --ignored`.
#[test]
#[ignore]
fn the_large_models_agree_but_where_known() {
    let rules = rules();
    let mut findings = Findings::new();
    for set in ["RealGrid/RealGrid-Merged", "FullGrid/FullGrid-Merged"] {
        conformance(set, &rules, &mut findings);
    }
    // The differences of the default set show up here too; only those marked large have
    // to.
    let differ = differing(&findings);
    let known: BTreeSet<&str> = KNOWN.iter().map(|(r, _, _)| *r).collect();
    let new: Vec<&&str> = differ.iter().filter(|r| !known.contains(**r)).collect();
    assert!(new.is_empty(), "differences without a reason: {new:?}");
    let missing: Vec<&str> = KNOWN
        .iter()
        .filter(|(rule, large, _)| *large && !differ.contains(rule))
        .map(|(rule, _, _)| *rule)
        .collect();
    assert!(
        missing.is_empty(),
        "known divergences that no longer occur: {missing:?}"
    );
}
