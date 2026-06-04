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
    pub name: String,
    pub description: String,
    /// SHACL constraint component (e.g. "sh:RequiredConstraintComponent").
    pub component: String,
    /// Component-specific payload keyed by simplified predicate name.
    pub payload: HashMap<String, ShaclValue>,
}

/// The target of a NodeShape (what objects it applies to).
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct TargetInfo {
    /// "targetClass", "targetImplicitClass", or "targetNode".
    pub kind: String,
    /// Simplified IRI of the target (e.g. "cim:ACLineSegment").
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
}

/// All shapes extracted from one TTL file.
#[derive(Debug)]
pub struct FileResults {
    /// TTL base file name without extension (e.g. "61970-600-2_Equipment-AP-Con-Simple-SHACL").
    pub file_name: String,
    pub shapes: Vec<ShapeInfo>,
}
