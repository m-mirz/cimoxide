//! Which CGMES constraint file applies to which profile, and when.
//!
//! NCP ships one manifest per profile (`NCP/SHACL/Validation/`) whose
//! `owl:imports` name the files that apply. CGMES ships none, so the mapping is
//! written out here — the data form of the dispatch `cimvalidation` used to
//! hard-code around the generated validators, with the same NotSolved gating
//! and local/cross-profile split.
//!
//! A tag is one of:
//!
//! * `"EQ"` — a local rule of that profile, run on each file declaring it
//! * `"EQ!NS"` — a NotSolvedMAS file's rule, run once on the merged dataset
//!   when no SV is present (not solved): those files are written for a model
//!   authority set and read across its profiles
//! * `"X:SV"` — a cross-profile rule, run once on the merged dataset
//! * `"HDR"` — a model header rule, run on every file
//! * `"COMMON"` — a rule for every profile, run once on the merged dataset
//!   when common checks are enabled
//!
//! A file absent from this list is not run. That covers the files whose every
//! constraint is `sh:sparql` — those are implemented by hand in
//! `cimvalidation::sparql` — and the files no profile dispatch ever ran.

/// `(TTL file stem, tag)`. A file may carry several tags.
pub const MANIFEST: &[(&str, &str)] = &[
    ("61970-552-Header-AP-Con-Simple-SHACL", "HDR"),
    // IdentifiedObject string lengths, for every profile
    ("61970-600-2_IdentifiedObjectCommon_AP-Con-Complex-SHACL", "COMMON"),
    // DL
    ("61970-301_DiagramLayout-AP-Con-Complex-NotSolvedMAS-SHACL", "DL!NS"),
    ("61970-301_DiagramLayout-AP-Con-Complex-SHACL", "DL"),
    ("61970-453_DiagramLayout-AP-Con-Complex-SHACL", "DL"),
    ("61970-600-2_DiagramLayout-AP-Con-Simple-SHACL", "DL"),
    ("61970-453_DiagramLayout-AP-Con-Complex-Implicit-CrossProfile-SHACL", "X:DL"),
    // DY
    ("61970-302_Dynamics-AP-Con-Complex-SHACL", "DY"),
    ("61970-600-2_Dynamics-AP-Con-Simple-SHACL", "DY"),
    ("61970-457_Dynamics-AP-Con-Complex-Implicit-CrossProfile-SHACL", "X:DY"),
    // EQ
    ("61970-301_Equipment-AP-Con-Complex-SHACL", "EQ"),
    ("61970-452_Equipment-AP-Con-Complex-SHACL", "EQ"),
    ("61970-600-1_Equipment-AP-Con-Complex-SHACL", "EQ"),
    ("61970-600-2_Equipment-AP-Con-Complex-SHACL", "EQ"),
    ("61970-600-2_Equipment-AP-Con-Simple-SHACL", "EQ"),
    // EQBD
    ("61970-301_EquipmentBoundary-AP-Con-Complex-SHACL", "EQBD"),
    ("61970-600-2_EquipmentBoundary-AP-Con-Simple-SHACL", "EQBD"),
    // GL
    ("61968-13_GeographicalLocation-AP-Con-Complex-SHACL", "GL"),
    ("61970-600-2_GeographicalLocation-AP-Con-Simple-SHACL", "GL"),
    ("61970-600-2_GeographicalLocation-AP-Con-Complex-Implicit-CrossProfile-SHACL", "X:GL"),
    // OP
    ("61970-301_Operation-AP-Con-Complex-SHACL", "OP"),
    ("61970-452_Operation-AP-Con-Complex-SHACL", "OP"),
    ("61970-600-2_Operation-AP-Con-Simple-SHACL", "OP"),
    ("61970-600-2_Operation-AP-Con-Complex-Implicit-CrossProfile-SHACL", "X:OP"),
    // SC
    ("61970-301_ShortCircuit-AP-Con-Complex-SHACL", "SC"),
    ("61970-600-2_ShortCircuit-AP-Con-Simple-SHACL", "SC"),
    // SSH
    ("61970-301_SteadyStateHypothesis-AP-Con-Complex-NotSolvedMAS-SHACL", "SSH!NS"),
    ("61970-301_SteadyStateHypothesis-AP-Con-Complex-SHACL", "SSH"),
    ("61970-456_SteadyStateHypothesis-AP-Con-Complex-NotSolvedMAS-SHACL", "SSH!NS"),
    ("61970-456_SteadyStateHypothesis-AP-Con-Complex-SHACL", "SSH"),
    ("61970-600-2_SteadyStateHypothesis-AP-Con-Simple-SHACL", "SSH"),
    // SV
    ("61970-301_StateVariables-AP-Con-Complex-SHACL", "SV"),
    ("61970-600-2_StateVariables-AP-Con-Simple-SHACL", "SV"),
    ("61970-456_StateVariables-AP-Con-Complex-Explicit-CrossProfile-SHACL", "X:SV"),
    ("61970-456_StateVariables-AP-Con-Complex-Implicit-CrossProfile-SHACL", "X:SV"),
    // TP
    ("61970-456_Topology-AP-Con-Complex-SHACL", "TP"),
    ("61970-600-2_Topology-AP-Con-Simple-SHACL", "TP"),
    ("61970-456_Topology-AP-Con-Complex-Implicit-CrossProfile-SHACL", "X:TP"),
];

/// The profiles with cross-profile rules, in the order they run.
pub const CROSS_PROFILES: &[&str] = &["DL", "DY", "GL", "OP", "SV", "TP"];
