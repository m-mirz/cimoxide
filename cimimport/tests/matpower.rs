//! MATPOWER cases from benchmark-grids (the `benchmark-grids/` submodule)
//! converted to CGMES: read back, validated, and checked against the case.

use std::collections::{HashMap, HashSet};
use std::path::PathBuf;

use cimimport::matpower::parse::{self, MatpowerCase};
use cimimport::{Converted, Options};
use cimmodel::CimDataset;

fn grids() -> PathBuf {
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../benchmark-grids");
    assert!(
        dir.join("matpower").is_dir(),
        "benchmark-grids is a submodule: git submodule update --init benchmark-grids"
    );
    dir
}

fn text(case: &str) -> String {
    std::fs::read_to_string(grids().join(case)).unwrap()
}

fn convert_with(case: &str, options: &Options) -> Converted {
    cimimport::matpower(case, &text(case), options).unwrap()
}

fn convert(case: &str) -> Converted {
    convert_with(case, &Options::default())
}

/// The files decoded, each on its own (as validation reads them) and merged.
fn decode(model: &Converted) -> (Vec<CimDataset>, CimDataset) {
    let files: Vec<CimDataset> = model
        .files
        .iter()
        .map(|(name, xml)| {
            CimDataset::decode_str(xml).unwrap_or_else(|e| panic!("{name} does not decode: {e}"))
        })
        .collect();
    let mut merged = CimDataset::new();
    for f in &files {
        merged.merge(f.clone());
    }
    (files, merged)
}

fn of_type<'a>(ds: &'a CimDataset, class: &str) -> Vec<&'a cimmodel::Element> {
    let mut out: Vec<_> = ds
        .entries
        .values()
        .filter(|e| e.type_name() == class)
        .collect();
    out.sort_by_key(|e| e.mrid().to_string());
    out
}

/// The SHACL violations by rule, warnings and advisories left out.
fn violations(files: Vec<CimDataset>) -> HashMap<String, usize> {
    let cfg = cimvalidation::combined_config(&files, None, None, false, false, Vec::new());
    let mut by_rule = HashMap::new();
    for v in cimvalidation::validate_files(files, &cfg) {
        if v.severity.ends_with("Violation") {
            *by_rule.entry(v.rule_id.clone()).or_default() += 1;
        }
    }
    by_rule
}

/// What CGMES's value checks reject in what the case gives, kept on purpose
/// (see `map.rs`): a transformer's positive `b`, or its negative impedance.
const KEPT_AS_GIVEN: [&str; 3] = [
    "PowerTransformerEnd.b-valueRange",
    "PowerTransformerEnd.x-value",
    "PowerTransformerEnd.r-valueRange",
];

fn kept_as_given(rule: &str) -> bool {
    KEPT_AS_GIVEN.iter().any(|k| rule.ends_with(k))
}

#[test]
fn every_case_that_states_its_data_converts_and_reads_back() {
    let mut refused = Vec::new();
    for dir in ["matpower", "matpower-plain", "pglib"] {
        let mut names: Vec<PathBuf> = std::fs::read_dir(grids().join(dir))
            .unwrap()
            .map(|e| e.unwrap().path())
            .filter(|p| p.extension().is_some_and(|e| e == "m"))
            .collect();
        names.sort();
        for path in names {
            let name = format!("{dir}/{}", path.file_name().unwrap().to_str().unwrap());
            let text = std::fs::read_to_string(&path).unwrap();
            // The five largest take most of a minute without optimisation;
            // `cargo test --release` converts them too, and the tests below
            // cover a 2,869- and a 9,241-bus case on their own.
            if cfg!(debug_assertions) && parse::parse(&text).is_ok_and(|c| c.bus.len() > 3000) {
                continue;
            }
            match cimimport::matpower(&name, &text, &Options::default()) {
                Ok(model) => {
                    assert_eq!(model.files.len(), 4, "{name}");
                    let case = parse::parse(&text).unwrap();
                    // Reading back the largest is slow without optimisation,
                    // and the smaller ones cover what they have.
                    if case.bus.len() > 3000 {
                        continue;
                    }
                    let (_, merged) = decode(&model);
                    assert_eq!(
                        of_type(&merged, "BusbarSection").len(),
                        case.bus.len(),
                        "{name}"
                    );
                }
                // The distribution originals convert their units in MATLAB
                // code; matpower-plain holds them evaluated.
                Err(e) => {
                    assert!(e.to_string().contains("MATLAB code"), "{name}: {e}");
                    assert!(
                        grids()
                            .join("matpower-plain")
                            .join(path.file_name().unwrap())
                            .exists(),
                        "{name}"
                    );
                    refused.push(name);
                }
            }
        }
    }
    assert!(
        refused.iter().all(|n| n.starts_with("matpower/")),
        "{refused:?}"
    );
}

#[test]
fn the_models_validate() {
    for case in [
        "matpower/case14.m",
        "matpower/case118.m",
        "matpower/case300.m",
        "matpower/case4_dist.m",
        "matpower/case_ACTIVSg200.m",
        "matpower-plain/case33bw.m",
        "matpower-plain/case69.m",
        "pglib/pglib_opf_case30_ieee.m",
        "pglib/pglib_opf_case300_ieee.m",
    ] {
        let model = convert(case);
        let (files, _) = decode(&model);
        let found = violations(files);
        let unexpected: Vec<_> = found
            .iter()
            .filter(|(rule, _)| !kept_as_given(rule))
            .collect();
        assert!(unexpected.is_empty(), "{case}: {unexpected:?}");
        // What is kept is said.
        if !found.is_empty() {
            assert!(
                model
                    .notes
                    .iter()
                    .any(|n| n.contains("kept as the case gives")),
                "{case}: {found:?} without a note"
            );
        }
    }
}

#[test]
fn converting_twice_gives_the_same_files() {
    for case in ["matpower/case118.m", "matpower/case_ACTIVSg200.m"] {
        let (a, b) = (convert(case), convert(case));
        assert_eq!(a.files, b.files, "{case}");
        assert_eq!(a.notes, b.notes, "{case}");
    }
}

#[test]
fn every_row_becomes_its_objects() {
    let model = convert("matpower/case118.m");
    let (_, ds) = decode(&model);
    let case = parse::parse(&text("matpower/case118.m")).unwrap();
    let count = |c: &str| of_type(&ds, c).len();
    // 118 buses, 186 branches (175 lines, 11 transformers), 54 generators.
    assert_eq!(count("BusbarSection"), 118);
    assert_eq!(count("ACLineSegment"), 175);
    assert_eq!(count("PowerTransformer"), 11);
    assert_eq!(count("PowerTransformerEnd"), 22);
    assert_eq!(count("SynchronousMachine"), case.gens.len());
    assert_eq!(count("RegulatingControl"), 54);
    let loads = case
        .bus
        .iter()
        .filter(|b| b[2] != 0.0 || b[3] != 0.0)
        .count();
    assert_eq!(
        count("EnergyConsumer") + count("EquivalentInjection"),
        loads
    );
    let shunts = case
        .bus
        .iter()
        .filter(|b| b[4] != 0.0 || b[5] != 0.0)
        .count();
    assert_eq!(count("LinearShuntCompensator"), shunts);
    // A breaker at each end of each branch.
    assert_eq!(count("Breaker"), 2 * 186);
    assert_eq!(count("SvSwitch"), 2 * 186);
}

#[test]
fn impedances_are_converted_from_per_unit() {
    let model = convert("matpower/case118.m");
    let (_, ds) = decode(&model);
    // Branch 1 to 2: r 0.0303, x 0.0999, b 0.0254 per unit on 100 MVA at 138 kV,
    // whose impedance base is 138² / 100 = 190.44 Ω.
    let line = of_type(&ds, "ACLineSegment")
        .into_iter()
        .find(|e| e.get_str("IdentifiedObject.name") == Some("Riversde V2 - Pokagon V2"))
        .expect("the line from bus 1 to bus 2");
    let close = |attr: &str, want: f64| {
        let got = line.get_f64(attr).unwrap();
        assert!(
            (got - want).abs() < 1e-9 * want.abs().max(1.0),
            "{attr}: {got} vs {want}"
        );
    };
    close("ACLineSegment.r", 0.0303 * 190.44);
    close("ACLineSegment.x", 0.0999 * 190.44);
    close("ACLineSegment.bch", 0.0254 / 190.44);
    // Its limit: rateA 0 is none.
    let limits = of_type(&ds, "CurrentLimit");
    assert!(limits.is_empty(), "case118 has no ratings");
}

#[test]
fn ratings_are_limits_in_amperes() {
    let model = convert("pglib/pglib_opf_case30_ieee.m");
    let (_, ds) = decode(&model);
    let case = parse::parse(&text("pglib/pglib_opf_case30_ieee.m")).unwrap();
    let rated = case.branch.iter().filter(|r| r[5] > 0.0).count();
    // A permanent limit at each end of each rated branch.
    let limits = of_type(&ds, "CurrentLimit");
    assert_eq!(limits.len(), 2 * rated);
    // Branch 1 to 2: rateA 138 MVA at 132 kV.
    let want = 138.0e3 / (3f64.sqrt() * 132.0);
    assert!(
        limits.iter().any(|l| {
            l.get_f64("CurrentLimit.normalValue")
                .is_some_and(|v| (v - want).abs() < 1e-9)
        }),
        "no {want} A limit"
    );
}

#[test]
fn a_transformers_end_one_is_its_highest_voltage_winding() {
    for case in ["matpower/case300.m", "matpower/case2869pegase.m"] {
        let model = convert(case);
        let (_, ds) = decode(&model);
        let mut ends: HashMap<&str, Vec<(i64, f64)>> = HashMap::new();
        for e in of_type(&ds, "PowerTransformerEnd") {
            ends.entry(e.get_ref("PowerTransformerEnd.PowerTransformer").unwrap())
                .or_default()
                .push((
                    e.get_str("TransformerEnd.endNumber")
                        .unwrap()
                        .parse()
                        .unwrap(),
                    e.get_f64("PowerTransformerEnd.ratedU").unwrap(),
                ));
        }
        for (pt, mut e) in ends {
            e.sort_by_key(|(n, _)| *n);
            assert_eq!(e.len(), 2, "{case} {pt}");
            assert!(e[0].1 >= e[1].1, "{case} {pt}: {e:?}");
        }
    }
}

#[test]
fn an_out_of_service_branch_has_its_breakers_open() {
    // case33bw's five tie switches are branches out of service.
    let model = convert("matpower-plain/case33bw.m");
    let (_, ds) = decode(&model);
    let case = parse::parse(&text("matpower-plain/case33bw.m")).unwrap();
    let out = case.branch.iter().filter(|r| r[10] == 0.0).count();
    assert_eq!(out, 5);
    let open = of_type(&ds, "Breaker")
        .into_iter()
        .filter(|b| b.get_str("Switch.open") == Some("true"))
        .count();
    assert_eq!(open, 2 * out);
    // The TP: a node per bus, and one for each end of each branch that is out.
    assert_eq!(
        of_type(&ds, "TopologicalNode").len(),
        case.bus.len() + 2 * out
    );
    // The buses' nodes, and only theirs, are in an island.
    let in_islands: usize = of_type(&ds, "TopologicalIsland")
        .iter()
        .map(|i| i.get_refs("TopologicalIsland.TopologicalNodes").len())
        .sum();
    assert_eq!(in_islands, case.bus.len());
}

#[test]
fn without_breakers_the_branches_connect_to_the_buses() {
    let options = Options {
        breakers: false,
        ..Options::default()
    };
    let model = convert_with("matpower-plain/case33bw.m", &options);
    let (files, ds) = decode(&model);
    assert!(of_type(&ds, "Breaker").is_empty());
    assert_eq!(of_type(&ds, "TopologicalNode").len(), 33);
    // Out of service, their terminals are disconnected instead.
    let disconnected = of_type(&ds, "Terminal")
        .into_iter()
        .filter(|t| t.get_str("ACDCTerminal.connected") == Some("false"))
        .count();
    assert_eq!(disconnected, 2 * 5);
    assert!(violations(files).is_empty());
}

#[test]
fn the_sv_holds_the_cases_voltages() {
    let path = "matpower/case300.m";
    let case: MatpowerCase = parse::parse(&text(path)).unwrap();
    let model = convert(path);
    let (_, ds) = decode(&model);
    // A topological node's voltage, by the node's name: case300 names its
    // buses by number only, so each node is `Bus <n>`.
    let names: HashMap<&str, &str> = of_type(&ds, "TopologicalNode")
        .into_iter()
        .map(|tn| (tn.mrid(), tn.get_str("IdentifiedObject.name").unwrap()))
        .collect();
    let sv: HashMap<&str, (f64, f64)> = of_type(&ds, "SvVoltage")
        .into_iter()
        .map(|v| {
            (
                names[v.get_ref("SvVoltage.TopologicalNode").unwrap()],
                (
                    v.get_f64("SvVoltage.v").unwrap(),
                    v.get_f64("SvVoltage.angle").unwrap(),
                ),
            )
        })
        .collect();
    assert_eq!(sv.len(), case.bus.len());
    for b in &case.bus {
        let (v, angle) = sv[format!("Bus {}", b[0]).as_str()];
        assert_eq!(v, b[7] * b[9], "bus {}", b[0]);
        assert_eq!(angle, b[8], "bus {}", b[0]);
    }
    assert!(!model.notes.iter().any(|n| n.contains("flat start")));
}

#[test]
fn a_flat_start_is_said() {
    let model = convert("matpower/case4_dist.m");
    assert!(
        model.notes.iter().any(|n| n.contains("flat start")),
        "{:?}",
        model.notes
    );
}

/// Data CGMES has no place for is named, table by table and column by column.
#[test]
fn what_is_not_converted_is_said() {
    let column_notes = |case: &str| -> Vec<String> {
        convert(case)
            .notes
            .into_iter()
            .filter(|n| n.contains("not converted") && !n.starts_with("mpc."))
            .collect()
    };
    assert_eq!(
        column_notes("pglib/pglib_opf_case30_ieee.m"),
        ["branch angle limits (ANGMIN, ANGMAX) of 41 row(s) not converted"]
    );
    assert_eq!(
        column_notes("matpower/case_ACTIVSg200.m"),
        [
            "bus ZONE not converted (6 zones)",
            "OPF results (LAM_P to MU_ANGMAX) of 200 row(s) not converted"
        ]
    );
    // One zone, and angle limits at ±360 degrees, which limit nothing.
    assert!(column_notes("matpower/case118.m").is_empty());
    assert!(
        convert("matpower/case118.m")
            .notes
            .iter()
            .any(|n| n == "mpc.gencost not converted")
    );
}

/// The substations, each as the sorted names of its busbars.
fn substations(ds: &CimDataset) -> Vec<Vec<String>> {
    let level_of: HashMap<&str, &str> = of_type(ds, "VoltageLevel")
        .into_iter()
        .map(|vl| (vl.mrid(), vl.get_ref("VoltageLevel.Substation").unwrap()))
        .collect();
    let mut by: HashMap<&str, Vec<String>> = HashMap::new();
    for bbs in of_type(ds, "BusbarSection") {
        let sub = level_of[bbs.get_ref("Equipment.EquipmentContainer").unwrap()];
        by.entry(sub)
            .or_default()
            .push(bbs.get_str("IdentifiedObject.name").unwrap().to_string());
    }
    let mut out: Vec<Vec<String>> = by
        .into_values()
        .map(|mut v| {
            v.sort();
            v
        })
        .collect();
    out.sort();
    out
}

#[test]
fn buses_are_grouped_by_transformer_and_by_name() {
    let model = convert("matpower/case_ACTIVSg200.m");
    let (_, ds) = decode(&model);
    let subs = substations(&ds);
    assert_eq!(subs.len(), 111);
    // Mason City: buses 17 to 21, joined by short lines, one substation.
    let mason: Vec<String> = (0..5).map(|k| format!("MASON CITY {k}")).collect();
    assert!(
        subs.contains(&mason),
        "{:?}",
        subs.iter().find(|s| s[0].starts_with("MASON"))
    );
    // Peoria 8 and Peoria 9 are two stations.
    let of = |name: &str| {
        subs.iter()
            .position(|s| s.iter().any(|b| b == name))
            .unwrap()
    };
    assert_ne!(of("PEORIA 8 0"), of("PEORIA 9 0"));
    assert_eq!(
        of_type(&ds, "Substation")
            .iter()
            .filter(|s| s.get_str("IdentifiedObject.name") == Some("MASON CITY"))
            .count(),
        1
    );
    assert!(
        model
            .notes
            .iter()
            .any(|n| n == "4 line(s) inside substation MASON CITY")
    );

    // By transformer alone.
    let options = Options {
        name_grouping: false,
        ..Options::default()
    };
    let (_, ds) = decode(&convert_with("matpower/case_ACTIVSg200.m", &options));
    assert_eq!(substations(&ds).len(), 134);
}

/// Each generator and branch is described by its row, so the case's own data on it (costs,
/// angle limits) can be found again.
#[test]
fn generators_and_branches_keep_their_rows() {
    let (_, ds) = decode(&convert("matpower/case118.m"));
    let described = |classes: &[&str], prefix: &str| -> HashMap<usize, String> {
        classes
            .iter()
            .flat_map(|class| of_type(&ds, class))
            .map(|e| {
                let row = e
                    .get_str("IdentifiedObject.description")
                    .and_then(|d| d.strip_prefix(prefix))
                    .and_then(|n| n.parse().ok())
                    .unwrap_or_else(|| panic!("{e:?} has no {prefix:?} description"));
                (row, e.get_str("IdentifiedObject.name").unwrap().to_string())
            })
            .collect()
    };
    let gens = described(&["SynchronousMachine"], "MATPOWER gen ");
    assert_eq!(gens.len(), 54);
    assert_eq!(gens[&1], "Riversde V2 G1");
    let branches = described(
        &[
            "ACLineSegment",
            "PowerTransformer",
            "SeriesCompensator",
            "EquivalentBranch",
        ],
        "MATPOWER branch ",
    );
    assert_eq!(branches.len(), 186);
    assert_eq!(branches[&1], "Riversde V2 - Pokagon V2");
    assert!((1..=186).all(|row| branches.contains_key(&row)));
}

#[test]
fn a_named_bus_keeps_its_number() {
    let (_, ds) = decode(&convert("matpower/case_ACTIVSg200.m"));
    for class in ["TopologicalNode", "ConnectivityNode"] {
        let named = |number: u32| {
            let description = format!("MATPOWER bus {number}");
            let found: Vec<_> = of_type(&ds, class)
                .into_iter()
                .filter(|e| e.get_str("IdentifiedObject.description") == Some(&description))
                .collect();
            assert_eq!(found.len(), 1, "{class} {description}");
            found[0]
                .get_str("IdentifiedObject.name")
                .unwrap()
                .to_string()
        };
        assert_eq!(named(17), "MASON CITY 0", "{class}");
        assert_eq!(named(21), "MASON CITY 4", "{class}");
    }
}

#[test]
fn a_ratio_of_one_is_a_line() {
    // RTE writes TAP 1 on its lines: they stay lines, and the substations stay
    // apart.
    let (_, ds) = decode(&convert("matpower/case1888rte.m"));
    let case = parse::parse(&text("matpower/case1888rte.m")).unwrap();
    let kv: HashMap<u64, f64> = case.bus.iter().map(|b| (b[0] as u64, b[9])).collect();
    let lines = case
        .branch
        .iter()
        .filter(|r| {
            (r[8] == 0.0 || r[8] == 1.0) && r[9] == 0.0 && kv[&(r[0] as u64)] == kv[&(r[1] as u64)]
        })
        .count();
    let written = ["ACLineSegment", "SeriesCompensator", "EquivalentBranch"]
        .iter()
        .map(|c| of_type(&ds, c).len())
        .sum::<usize>();
    assert_eq!(written, lines);
}

#[test]
fn negative_line_impedances_become_compensators_and_equivalents() {
    let model = convert("matpower/case9241pegase.m");
    let (_, ds) = decode(&model);
    assert_eq!(of_type(&ds, "SeriesCompensator").len(), 16);
    assert_eq!(of_type(&ds, "EquivalentBranch").len(), 14);
    for l in of_type(&ds, "ACLineSegment") {
        assert!(l.get_f64("ACLineSegment.x").unwrap() > 0.0);
        assert!(l.get_f64("ACLineSegment.r").unwrap() >= 0.0);
    }
}

#[test]
fn mrids_are_stable_and_distinct() {
    let (_, a) = decode(&convert("matpower/case118.m"));
    let ids: HashSet<&str> = a.entries.keys().map(String::as_str).collect();
    // The same bus of another case is another object.
    let other = cimimport::build::Builder::new("case14").id("bus:1");
    let this = cimimport::build::Builder::new("case118").id("bus:1");
    assert_ne!(other, this);
    assert!(ids.contains(this.as_str()));
    assert!(!ids.contains(other.as_str()));
}
