//! Profile families.
//!
//! cimoxide reads two ENTSO-E schema trees that are structurally similar but not
//! identical: CGMES (grid models) and NCP (Network Code Profiles). A `Family`
//! captures every place the two diverge, so the importer and generator stay
//! branch-free and the CGMES output is provably unchanged when NC is added.

/// Everything that differs between one profile family and another.
#[derive(Debug)]
pub struct Family {
    /// Default RDFS glob, relative to the workspace root.
    pub default_schema: &'static str,
    /// Profile forced to priority 1 and used as the origin tie-break.
    /// `None` means pure alphabetical ordering.
    pub base_profile: Option<&'static str>,
    /// Namespaces added to `profile_namespaces` even when no class lives in them.
    pub extra_namespaces: &'static [(&'static str, &'static str)],
    /// NCP writes the classifying `cims:stereotype` first and trails profile tags
    /// (`NC`, `profcim`, `deprecated`), so it needs first-match classification.
    /// CGMES relies on the historical last-wins reading.
    pub stereotype_first_wins: bool,
}

pub const CGMES: Family = Family {
    default_schema:
        "application-profiles-library/CGMES/RDFS/61970-600-2_*-AP-Voc-RDFS2020.rdf",
    base_profile: Some("EQ"),
    extra_namespaces: &[
        ("md", "http://iec.ch/TC57/61970-552/ModelDescription/1#"),
        ("rdf", "http://www.w3.org/1999/02/22-rdf-syntax-ns#"),
    ],
    stereotype_first_wins: false,
};

// NCP's DatasetMetadata profile keyword is "DM", which reads like the `dm` XML
// prefix bound to DifferenceModel in CGMES. They are unrelated: profile codes and
// namespace prefixes live in different tables.
// Referenced by the schema tests until NC generation is wired up.
#[allow(dead_code)]
pub const NC: Family = Family {
    default_schema: "application-profiles-library/NCP/RDFS/*-AP-Voc-RDFS2020.rdf",
    base_profile: None,
    extra_namespaces: &[("rdf", "http://www.w3.org/1999/02/22-rdf-syntax-ns#")],
    stereotype_first_wins: true,
};
