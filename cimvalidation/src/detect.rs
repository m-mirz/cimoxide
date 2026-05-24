use cimdecoder::CimDataset;
use crate::sparql::Config;

const PROF_BASE: &str = "http://iec.ch/TC57/ns/CIM/";
const PROF_EQ:   &str = "http://iec.ch/TC57/ns/CIM/CoreEquipment-EU/3.0";
const PROF_EQBD: &str = "http://iec.ch/TC57/ns/CIM/EquipmentBoundary-EU/3.0";
const PROF_DY:   &str = "http://iec.ch/TC57/ns/CIM/Dynamics-EU/1.0";
const PROF_DL:   &str = "http://iec.ch/TC57/ns/CIM/DiagramLayout-EU/3.0";
const PROF_SC:   &str = "http://iec.ch/TC57/ns/CIM/ShortCircuit-EU/3.0";
const PROF_OP:   &str = "http://iec.ch/TC57/ns/CIM/Operation-EU/3.0";
const PROF_GL:   &str = "http://iec.ch/TC57/ns/CIM/GeographicalLocation-EU/3.0";
const PROF_SV:   &str = "http://iec.ch/TC57/ns/CIM/StateVariables-EU/3.0";
const PROF_TP:   &str = "http://iec.ch/TC57/ns/CIM/Topology-EU/3.0";
const PROF_SSH:  &str = "http://iec.ch/TC57/ns/CIM/SteadyStateHypothesis-EU/3.0";

fn uri_to_short_name(uri: &str) -> Option<&'static str> {
    match uri {
        PROF_EQ   => Some("EQ"),
        PROF_SSH  => Some("SSH"),
        PROF_TP   => Some("TP"),
        PROF_SV   => Some("SV"),
        PROF_DY   => Some("DY"),
        PROF_SC   => Some("SC"),
        PROF_DL   => Some("DL"),
        PROF_GL   => Some("GL"),
        PROF_OP   => Some("OP"),
        PROF_EQBD => Some("EQBD"),
        _         => None,
    }
}

fn collect_profiles_from_type(dataset: &CimDataset, type_name: &str, seen: &mut std::collections::HashSet<&'static str>) {
    for mrid in dataset.by_type.get(type_name).into_iter().flatten() {
        let entry = match dataset.entries.get(mrid) { Some(e) => e, None => continue };
        let profiles: &[String] = if let Some(fm) = entry.element.as_any().downcast_ref::<cimstructs::FullModel>() {
            &fm.base.profile
        } else if let Some(dm) = entry.element.as_any().downcast_ref::<cimstructs::DifferenceModel>() {
            &dm.base.profile
        } else {
            continue;
        };
        for p in profiles {
            let p = p.trim();
            if !p.is_empty() {
                if let Some(short) = uri_to_short_name(p) {
                    seen.insert(short);
                }
            }
        }
    }
}

/// Inspects dataset model headers and returns a Config with profiles, solved, and not_solved populated.
pub fn detect_config(dataset: &CimDataset) -> Config {
    let mut seen: std::collections::HashSet<&'static str> = std::collections::HashSet::new();
    collect_profiles_from_type(dataset, "FullModel", &mut seen);
    collect_profiles_from_type(dataset, "DifferenceModel", &mut seen);

    let mut profiles: Vec<String> = seen.iter().map(|s| s.to_string()).collect();
    profiles.sort();

    let is_solved = seen.contains("SV");
    Config {
        profiles,
        solved: is_solved,
        not_solved: !is_solved,
        ..Config::default()
    }
}
