//! The shape table a property-bag family is validated against.
//!
//! CGMES validation is generated code: one function per check, each
//! downcasting to a concrete struct and reading a typed field. NC classes have
//! no struct to downcast to — they decode into
//! [`cimstructs::base::GenericElement`] property bags — so there is nothing for
//! that strategy to generate against.
//!
//! So the shapes become data and [`crate::bag`] interprets them, the same way
//! NC classes became [`cimstructs::base::ClassDef`] rows interpreted by the
//! decoder. `cimgen` emits the table into `nc_shapes.rs`; with the
//! `dynamic-shapes` feature it can be built from the SHACL TTL files at
//! runtime instead.
//!
//! Everything a consumer would have to compute per element is resolved when
//! the table is built: target classes are already family-qualified
//! `CimDataset::by_type` keys with abstract classes expanded to their concrete
//! descendants, and paths are already the field keys the decoder stores.

/// Where a property shape's values come from.
///
/// Field keys are the local XML element name with the prefix stripped, which
/// is exactly what the decoder puts in [`cimstructs::base::RdfBlock::fields`].
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
    /// `sh:path ( nc:X.y rdf:type )` — follow the association, then look at the
    /// referenced element's class. 170 of NCP's 178 property chains are this
    /// shape, and they are its main association value-type check.
    ///
    /// The referenced element may belong to *either* family: these shapes list
    /// CGMES classes (`cim17:ACLineSegment`) alongside NC ones, and
    /// `CimElement::type_name` answers for both.
    RefType(&'static str),
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
    /// strings, so this is a real parse check rather than the tautology it is
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
    /// names. Produced by `sh:in` on a [`Path::RefType`].
    ///
    /// Only checked when the referenced element is **present**. Phase-1
    /// validation runs per file, so a reference out of the current file is
    /// normal and is a different rule's business — reporting it here would make
    /// every cross-file association a value-type violation.
    RefClass(&'static [&'static str]),
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
