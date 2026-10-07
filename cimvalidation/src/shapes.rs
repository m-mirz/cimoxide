//! The shape table both families are validated against.
//!
//! Elements are property bags ([`cimmodel::Element`]) with no struct to
//! downcast to, so the shapes are data and [`crate::bag`] interprets them, the
//! same way classes are [`cimmodel::base::ClassDef`] rows interpreted by the
//! decoder. `cimgen` emits the tables into `cgmes_shapes.rs` and
//! `nc_shapes.rs`; with the `dynamic-shapes` feature they can be built from the
//! SHACL TTL files at runtime instead.
//!
//! Everything a consumer would have to compute per element is resolved when
//! the table is built: target classes are already family-qualified
//! `CimDataset::by_type` keys with abstract classes expanded to their concrete
//! descendants, and paths are already the field keys the decoder stores.

/// Where a property shape's values come from.
///
/// Field keys are the local XML element name with the prefix stripped, which
/// is exactly what the decoder puts in [`cimmodel::Element::fields`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Path {
    /// A field on the element itself.
    Forward(&'static str),
    /// `sh:inversePath` — the field on whatever points *at* this element.
    /// Needs the reverse index, which is why [`ShapeSet`] tracks whether any
    /// active shape uses one.
    Inverse(&'static str),
    /// `sh:alternativePath` — the union of every branch's values. A branch may
    /// itself be inverse, hence [`AltBranch`] rather than a plain name.
    Alternative(&'static [AltBranch]),
    /// A sequence path: `( ^cim:Terminal.ConductingEquipment
    /// cim:Terminal.phases )`, `( cim:Location.mainAddress
    /// cim:StreetAddress.status cim:Status.dateTime )`, and — 170 of NCP's 178
    /// chains, its main association value-type check — `( nc:X.y rdf:type )`.
    ///
    /// Stepping *from* an element absent from the dataset makes the result
    /// unknown and every check on it silent: phase 1 runs per file, so leaving
    /// the file is normal. A final `rdf:type` step needs nothing from the
    /// elements it reaches, so they count even when absent, and class checks
    /// skip the absent ones, as [`Constraint::RefClass`] describes.
    ///
    /// The referenced element may belong to *either* family: NCP's value-type
    /// lists name CGMES classes (`cim17:ACLineSegment`) alongside NC ones, and
    /// `Element::type_name` answers for both.
    Chain(&'static [Step]),
}

/// One step of a [`Path::Chain`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Step {
    Forward(&'static str),
    /// Needs the reverse index, like [`Path::Inverse`], but as a list of the
    /// elements pointing here rather than a count.
    Inverse(&'static str),
    /// `rdf:type`, last only. Its values are the elements reached; the
    /// constraints on such a path (`RefClass`, counts) read their classes.
    Type,
}

/// One branch of an [`Path::Alternative`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AltBranch {
    Forward(&'static str),
    Inverse(&'static str),
}

/// `sh:nodeKind`, restricted to the values the profiles actually use.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NodeKind {
    /// Written as `rdf:resource`.
    Iri,
    /// Written as element text.
    Literal,
    BlankNode,
}

/// A single check on the values a [`Path`] yields.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Constraint {
    MinCount(u32),
    MaxCount(u32),
    /// The xsd type's local name (`"integer"`, `"dateTime"`, …). A bag holds
    /// strings, so this is a real parse check rather than the tautology it was
    /// against a typed struct.
    Datatype(&'static str),
    NodeKind(NodeKind),
    /// `sh:class` — the referenced element must be an instance of this
    /// family-qualified class, or a descendant of it. Already expanded, so the
    /// check is a set membership.
    Class(&'static [&'static str]),
    /// `sh:in` — the value must be one of these. Enum IRIs are stored as the
    /// decoder stores them: fragment only.
    In(&'static [&'static str]),
    /// `sh:hasValue`, and `sh:in` with exactly one member after
    /// simplification.
    HasValue(&'static str),
    MaxLength(u32),
    MinLength(u32),
    /// The referenced element's class must be one of these family-qualified
    /// names. Produced by `sh:in` on a path ending in `rdf:type`.
    ///
    /// Only checked when the referenced element is **present**. Phase-1
    /// validation runs per file, so a reference out of the current file is
    /// normal and is a different rule's business — reporting it here would make
    /// every cross-file association a value-type violation.
    RefClass(&'static [&'static str]),
    /// `sh:minInclusive` and friends. Compared as `f64`; a value that does not
    /// parse as a number is skipped, the way a typed numeric field that failed
    /// to parse is `None` — malformed numbers are `sh:datatype`'s business.
    MinInclusive(f64),
    MaxInclusive(f64),
    MinExclusive(f64),
    MaxExclusive(f64),
    /// `sh:lessThan` — this property's value must be below the named field's
    /// on the same element. Only checked when both are present and numeric.
    LessThan(&'static str),
    LessThanOrEquals(&'static str),
    /// `sh:not [ sh:class X ]` — the referenced element must not be an
    /// instance of these family-qualified classes. Silent when the referenced
    /// element is absent, like [`Constraint::Class`].
    NotClass(&'static [&'static str]),
    /// `sh:qualifiedValueShape [ sh:in (..) ]` with `sh:qualifiedMinCount` —
    /// at least `min` values must be among `allowed`. NCP uses it to ask
    /// whether a dataset declares conformance to one of a set of profiles.
    QualifiedIn { allowed: &'static [&'static str], min: u32 },
    /// `sh:length` — exactly this many characters, e.g. the 16 of an EIC code.
    Length(u32),
}

/// One constraint together with how to report it.
///
/// SHACL attaches `sh:message`, `sh:name` and `sh:severity` to the constraint,
/// not to the property shape, and one property shape routinely carries several
/// constraints with different messages — a cardinality rule and a value-range
/// rule on the same path. Pairing them here keeps the reported text the one the
/// schema wrote for that specific check.
#[derive(Debug)]
pub struct Check {
    pub constraint: Constraint,
    /// Shape IRI, the stable machine-readable rule id a caller can silence.
    pub rule_id: &'static str,
    pub name: &'static str,
    pub message: &'static str,
    pub description: &'static str,
    /// `"sh:Violation"`, `"sh:Warning"` or `"sh:Info"`. NC leans on `sh:Info`
    /// far more than CGMES does — 842 occurrences against 7 — so a caller that
    /// treats every violation as a failure will report an NC dataset as broken
    /// when it is merely being advised.
    pub severity: &'static str,
}

/// One `sh:property` of a node shape: a path, and what must hold of its values.
#[derive(Debug)]
pub struct PropShape {
    pub path: Path,
    pub checks: &'static [Check],
}

/// What a node shape applies to.
#[derive(Debug)]
pub enum Target {
    /// Family-qualified `by_type` keys. An abstract target is expanded to its
    /// concrete descendants when the table is built — exact-match lookup on an
    /// abstract class would silently check nothing.
    Class(&'static [&'static str]),
    /// `sh:targetSubjectsOf` — every element carrying this field.
    SubjectsOf(&'static str),
}

/// How a [`Logic`] combines its branches.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogicOp {
    And,
    Or,
    Xone,
}

/// One branch of a [`Logic`]. It conforms when none of its checks fail — or,
/// for `[ sh:not X ]` (`negate`), when at least one does. NCP writes material
/// implication that way: `sh:or ( [ sh:not conformsToNCProfile ] [ P required ] )`.
#[derive(Debug)]
pub struct Branch {
    pub props: &'static [PropShape],
    pub negate: bool,
}

/// A node-level `sh:and` / `sh:or` / `sh:xone`. The shape reports once, with
/// its own name and message, when the combination does not hold; the branch
/// checks carry no report text.
#[derive(Debug)]
pub struct Logic {
    pub op: LogicOp,
    pub branches: &'static [Branch],
    pub rule_id: &'static str,
    pub name: &'static str,
    pub message: &'static str,
    pub description: &'static str,
    pub severity: &'static str,
}

/// One `sh:NodeShape`.
#[derive(Debug)]
pub struct ShapeDef {
    pub targets: &'static [Target],
    /// Borrowed rather than owned: NCP defines ~4,200 distinct property shapes
    /// and references them ~14,800 times, because a shape like
    /// `IdentifiedObject.mRID-cardinality` applies to hundreds of classes.
    /// Sharing one definition per shape keeps the generated table a third of
    /// the size it would otherwise be.
    pub props: &'static [&'static PropShape],
    /// `sh:closed` — the allowed field keys. An element carrying any other key
    /// is reported.
    ///
    /// This is the constraint that only a property bag can answer: generated
    /// structs drop unknown properties at decode, leaving nothing to compare
    /// against. 823 NCP shapes use it.
    pub closed: Option<&'static ClosedShape>,
    /// Node-level `sh:and` / `sh:or` / `sh:xone`.
    pub logic: &'static [Logic],
    /// Every NC profile code whose manifest imports the file this shape came
    /// from. Plural because four shared constraint files are imported by 17 or
    /// 18 of the 18 manifests.
    pub profiles: &'static [&'static str],
    /// TTL base file name, for reports.
    pub file: &'static str,
}

/// The `sh:closed` half of a shape, kept separate because it carries its own
/// message and severity.
#[derive(Debug)]
pub struct ClosedShape {
    pub allowed: &'static [&'static str],
    pub rule_id: &'static str,
    pub name: &'static str,
    pub message: &'static str,
    pub description: &'static str,
    pub severity: &'static str,
}
