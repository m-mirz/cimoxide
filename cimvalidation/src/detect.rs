use cimmodel::CimDataset;
use crate::Config;

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
        let profiles: &[String] = if let Some(fm) = entry.element.as_any().downcast_ref::<cimmodel::FullModel>() {
            &fm.base.profile
        } else if let Some(dm) = entry.element.as_any().downcast_ref::<cimmodel::DifferenceModel>() {
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

// ── NC / property-bag families ─────────────────────────────────────────────

/// Profile codes an NC dataset declares conformance to.
///
/// CGMES announces its profiles in an `md:FullModel` header whose
/// `Model.profile` values are matched against a hardcoded URI table above. NC
/// uses a DCAT header instead: a `dcat:Dataset` with `dcterms:conformsTo`
/// naming profile IRIs, which `NCP/PROF` maps to short codes — read from the
/// descriptors rather than written out by hand.
///
/// The field key is the bare `conformsTo`, because the decoder keys fields by
/// the XML local name and the predicate is written `<dcterms:conformsTo>`.
pub fn detect_nc_profiles(dataset: &CimDataset) -> Vec<String> {
    let mut seen: Vec<String> = Vec::new();
    for type_name in ["nc:Dataset", "nc:DifferenceSet"] {
        for mrid in dataset.by_type.get(type_name).into_iter().flatten() {
            let Some(entry) = dataset.entries.get(mrid) else { continue };
            let Some(el) = entry
                .element
                .as_any()
                .downcast_ref::<cimmodel::base::GenericElement>()
            else {
                continue;
            };
            // Written as rdf:resource in practice, but a plain literal is
            // legal too, so both are read.
            let refs: Vec<&str> = el.get_refs("conformsTo").iter().map(String::as_str).collect();
            let text = el.get_str("conformsTo").into_iter().collect::<Vec<_>>();
            for iri in refs.into_iter().chain(text) {
                if let Some(code) = nc_profile_code(iri) {
                    if !seen.iter().any(|s| s == code) {
                        seen.push(code.to_string());
                    }
                }
            }
        }
    }
    seen.sort();
    seen
}

/// The short code for a profile IRI, if it names one.
///
/// A dataset may declare the base IRI or a version IRI; the descriptors list
/// both. An IRI the descriptors do not know is not an NC profile.
pub fn nc_profile_code(iri: &str) -> Option<&'static str> {
    let iri = iri.trim();
    crate::nc_profile_index()
        .0
        .iter()
        .find(|(known, _)| *known == iri)
        .map(|(_, code)| *code)
}
