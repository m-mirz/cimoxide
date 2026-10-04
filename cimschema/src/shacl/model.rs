use std::collections::HashMap;

/// A single value in a SHACL constraint payload.
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum ShaclValue {
    Str(String),
    Int(i64),
    Float(f64),
    Bool(bool),
    List(Vec<String>),
    /// Nested sub-shape branches for sh:or / sh:and / sh:xone.
    /// Each inner Vec is one branch; each branch is a list of constraints.
    Shapes(Vec<Vec<ConstraintInfo>>),
}

impl ShaclValue {
    pub fn as_str(&self) -> Option<&str> {
        if let ShaclValue::Str(s) = self { Some(s) } else { None }
    }
    pub fn as_int(&self) -> Option<i64> {
        if let ShaclValue::Int(n) = self { Some(*n) } else { None }
    }
    pub fn as_float(&self) -> Option<f64> {
        match self {
            ShaclValue::Float(f) => Some(*f),
            ShaclValue::Int(n) => Some(*n as f64),
            _ => None,
        }
    }
    pub fn as_list(&self) -> Option<&Vec<String>> {
        if let ShaclValue::List(v) = self { Some(v) } else { None }
    }
    pub fn as_shapes(&self) -> Option<&Vec<Vec<ConstraintInfo>>> {
        if let ShaclValue::Shapes(v) = self { Some(v) } else { None }
    }
}

/// One constraint on a property shape (e.g. sh:minCount, sh:in, sh:pattern).
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct ConstraintInfo {
    /// SHACL property path as simplified IRI segments (e.g. ["cim:ACLineSegment.r"]).
    pub path: Vec<String>,
    /// sh:severity simplified (e.g. "sh:Violation", "sh:Warning").
    pub severity: String,
    pub message: String,
    /// sh:name value — structured human-readable rule name (e.g. "C:301:EQ:ACLineSegment.r:valueRange").
    pub name: String,
    pub description: String,
    /// SHACL constraint component (e.g. "sh:RequiredConstraintComponent").
    pub component: String,
    /// Component-specific payload keyed by simplified predicate name.
    pub payload: HashMap<String, ShaclValue>,
    /// Shape IRI used as the machine-readable rule identifier (e.g. "equ:ACLineSegment.r-valueRange").
    pub rule_id: String,
}

/// The target of a NodeShape (what objects it applies to).
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct TargetInfo {
    /// "targetClass", "targetNode", "targetSubjectsOf", "targetObjectsOf", or
    /// "sparqlTarget". Only "targetClass"/"targetNode" resolve to a concrete class for
    /// codegen (see codegen.rs's kind filter in render_file); the others carry no
    /// resolvable class and are recorded so the shape survives parsing instead of being
    /// dropped, with codegen recording an explicit skip entry for them instead.
    pub kind: String,
    /// For "targetClass"/"targetNode": simplified IRI of the target (e.g.
    /// "cim:ACLineSegment"). For "targetSubjectsOf"/"targetObjectsOf": the predicate
    /// IRI. For "sparqlTarget": the blank node id of the `sh:target [a sh:SPARQLTarget;
    /// ..]` object (not a resolvable value on its own).
    pub value: String,
}

/// A parsed and simplified SHACL shape (NodeShape or PropertyShape).
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct ShapeInfo {
    pub id: String,
    pub targets: Vec<TargetInfo>,
    /// Non-empty for PropertyShapes; the property path.
    pub path: Vec<String>,
    pub name: String,
    pub description: String,
    pub constraints: Vec<ConstraintInfo>,
    /// Nested sh:property shapes.
    pub properties: Vec<ShapeInfo>,
    /// `sh:closed true` — the shape's `sh:property` paths, plus
    /// `sh:ignoredProperties`, are the *only* properties the profile allows on
    /// the target class. Simplified IRIs, as [`Self::path`] uses.
    ///
    /// Read straight off the NodeShape rather than from [`Self::properties`]:
    /// the allowed set is written as constraint-less `[ sh:path X ]` blank
    /// nodes, which `build_property_shape` drops for having nothing to check.
    pub closed: Option<Vec<String>>,
    /// `sh:deactivated true` — the schema switched this shape off. Carried
    /// rather than dropped at parse time so the simplification stage can
    /// account for it like any other skip.
    pub deactivated: bool,
}

/// All shapes extracted from one TTL file.
#[derive(Debug)]
pub struct FileResults {
    /// TTL base file name without extension (e.g. "61970-600-2_Equipment-AP-Con-Simple-SHACL").
    pub file_name: String,
    pub shapes: Vec<ShapeInfo>,
    /// The file's own `@prefix` declarations, prefix → IRI.
    ///
    /// Shapes carry simplified IRIs (`"cim:Equipment"`), and the prefix alone
    /// does not identify a class: NCP binds `cim` to `https://cim.ucaiug.io/ns#`
    /// while CGMES binds it to `http://iec.ch/TC57/CIM100#`, and the two
    /// families overlap on 164 local names. Resolving a target or a path
    /// therefore needs the map that was in scope where it was written.
    pub prefixes: HashMap<String, String>,
}
