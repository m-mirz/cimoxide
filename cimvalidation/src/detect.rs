use cimmodel::CimDataset;
use crate::Config;

/// The short code for a CGMES profile IRI, from `CGMES/PROF` via
/// [`crate::cgmes_profile_index`].
fn uri_to_short_name(uri: &str) -> Option<&'static str> {
    crate::cgmes_profile_index().0.iter().find(|(iri, _)| *iri == uri).map(|(_, code)| *code)
}

fn collect_profiles_from_type(dataset: &CimDataset, type_name: &str, seen: &mut std::collections::HashSet<&'static str>) {
    for mrid in dataset.by_type.get(type_name).into_iter().flatten() {
        let entry = match dataset.entries.get(mrid) { Some(e) => e, None => continue };
        let Some(model) = crate::sparql::Fields::of_class(entry, type_name) else { continue };
        for p in model.texts("Model.profile") {
            let p = p.trim();
            if !p.is_empty()
                && let Some(short) = uri_to_short_name(p) {
                    seen.insert(short);
                }
        }
    }
}

/// Inspects dataset model headers and returns a Config with profiles, solved, and not_solved populated.
///
/// A CGMES profile is declared by an `md:FullModel`/`md:DifferenceModel`'s
/// `Model.profile`, or by a DCAT header's `dcterms:conformsTo` naming the same
/// profile IRI — the header NC uses, which CGMES datasets may carry instead.
/// A `conformsTo` naming a vocabulary (`http://iec.ch/TC57/CIM100#`) declares
/// no profile.
pub fn detect_config(dataset: &CimDataset) -> Config {
    let mut seen: std::collections::HashSet<&'static str> = std::collections::HashSet::new();
    collect_profiles_from_type(dataset, "FullModel", &mut seen);
    collect_profiles_from_type(dataset, "DifferenceModel", &mut seen);
    for iri in conforms_to(dataset) {
        if let Some(short) = uri_to_short_name(iri.trim()) {
            seen.insert(short);
        }
    }

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
/// `Model.profile` values are matched against `CGMES/PROF`'s index above. NC
/// uses a DCAT header instead: a `dcat:Dataset` with `dcterms:conformsTo`
/// naming profile IRIs, which `NCP/PROF` maps to short codes.
///
/// The field key is the bare `conformsTo`, because the decoder keys fields by
/// the XML local name and the predicate is written `<dcterms:conformsTo>`.
pub fn detect_nc_profiles(dataset: &CimDataset) -> Vec<String> {
    let mut seen: Vec<String> = Vec::new();
    for iri in conforms_to(dataset) {
        if let Some(code) = nc_profile_code(iri)
            && !seen.iter().any(|s| s == code) {
                seen.push(code.to_string());
            }
    }
    seen.sort();
    seen
}

/// Every `dcterms:conformsTo` of the dataset's DCAT headers (`dcat:Dataset`,
/// `dm:DifferenceSet`, which decode as `nc:` classes in either family).
/// Written as `rdf:resource` in practice, but a plain literal is legal too, so
/// both are read.
fn conforms_to(dataset: &CimDataset) -> impl Iterator<Item = &str> {
    ["nc:Dataset", "nc:DifferenceSet"]
        .into_iter()
        .flat_map(|t| dataset.by_type.get(t).into_iter().flatten())
        .filter_map(|mrid| dataset.entries.get(mrid))
        .flat_map(|el| {
            el.get_refs("conformsTo").iter().map(String::as_str).chain(el.get_str("conformsTo"))
        })
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
