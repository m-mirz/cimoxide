use super::model::*;

/// Apply all 7 normalisation rules to every FileResults.
pub fn simplify(results: &mut Vec<FileResults>) {
    for fr in results.iter_mut() {
        for shape in &mut fr.shapes {
            simplify_shape(shape);
        }
    }
}

fn simplify_shape(shape: &mut ShapeInfo) {
    for prop in &mut shape.properties {
        prop.constraints = simplify_constraints(std::mem::take(&mut prop.constraints));
    }
}

/// Apply rules 1–7 to a flat list of constraints on one property shape.
fn simplify_constraints(constraints: Vec<ConstraintInfo>) -> Vec<ConstraintInfo> {
    // Pass 1: determine whether a sh:datatype is present on this shape's constraints.
    let has_datatype = constraints
        .iter()
        .any(|c| c.component == "sh:DatatypeConstraintComponent");

    let mut out: Vec<ConstraintInfo> = Vec::with_capacity(constraints.len());

    for c in constraints {
        match c.component.as_str() {
            // Rule 1: Drop NodeKind=Literal if any sh:datatype is present —
            // the datatype already implies a literal node.
            "sh:NodeKindConstraintComponent" => {
                let nk = c.payload.get("nodeKind").and_then(|v| v.as_str()).unwrap_or("");
                if nk == "sh:Literal" && has_datatype {
                    continue; // drop
                }
                // Rule 2: Drop NodeKind=BlankNodeOrIRI and NodeKind=IRI —
                // MridRef / UriRef already enforce these in Rust.
                if nk == "sh:BlankNodeOrIRI" || nk == "sh:IRI" {
                    continue; // drop
                }
                out.push(c);
            }

            // Rule 7: Drop sh:datatype when Rust's type system already enforces it.
            "sh:DatatypeConstraintComponent" => {
                let dt = c.payload.get("datatype").and_then(|v| v.as_str()).unwrap_or("");
                if is_native_rust_type(dt) {
                    continue; // drop
                }
                out.push(c);
            }

            // Rules 3–5: Normalise cardinality constraints.
            "sh:MinCountConstraintComponent"
            | "sh:MaxCountConstraintComponent"
            | "sh:ExactCountConstraintComponent"
            | "sh:RequiredConstraintComponent" => {
                let min = c.payload.get("minCount").and_then(|v| v.as_int()).unwrap_or(0);
                let max = c.payload.get("maxCount").and_then(|v| v.as_int());

                // Rule 3: Drop minCount=0 — vacuously true.
                if min == 0 && max.is_none() && c.component == "sh:MinCountConstraintComponent" {
                    continue;
                }

                // Rule 4: min=0 + max=1 — keep MaxCountConstraintComponent so the
                // codegen can emit a duplicate-field check; drop MinCountConstraintComponent
                // with min=0 (vacuously true) when paired with an explicit max.
                if min == 0 && max == Some(1) && c.component == "sh:MinCountConstraintComponent" {
                    continue;
                }

                // Rule 5: min=1 + max=1 → Required (emitting Required constraint).
                // Already normalised to sh:RequiredConstraintComponent by the importer.
                out.push(c);
            }

            // Rule 6: Convert sh:in with a single value to sh:HasValue.
            "sh:InConstraintComponent" => {
                let values = c.payload.get("in").and_then(|v| v.as_list());
                if let Some(vals) = values {
                    if vals.len() == 1 {
                        let single = vals[0].clone();
                        let mut payload = std::collections::HashMap::new();
                        payload.insert("hasValue".to_string(), ShaclValue::Str(single));
                        out.push(ConstraintInfo {
                            component: "sh:HasValueConstraintComponent".to_string(),
                            payload,
                            ..c
                        });
                        continue;
                    }
                }
                out.push(c);
            }

            _ => out.push(c),
        }
    }

    out
}

/// Rule 7: datatypes whose validity is guaranteed by the Rust type system.
fn is_native_rust_type(xsd_type: &str) -> bool {
    matches!(
        xsd_type,
        "xsd:string"
            | "xsd:boolean"
            | "xsd:integer"
            | "xsd:int"
            | "xsd:long"
            | "xsd:float"
            | "xsd:double"
            | "xsd:decimal"
            | "<http://www.w3.org/2001/XMLSchema#string>"
            | "<http://www.w3.org/2001/XMLSchema#boolean>"
            | "<http://www.w3.org/2001/XMLSchema#integer>"
            | "<http://www.w3.org/2001/XMLSchema#int>"
            | "<http://www.w3.org/2001/XMLSchema#long>"
            | "<http://www.w3.org/2001/XMLSchema#float>"
            | "<http://www.w3.org/2001/XMLSchema#double>"
            | "<http://www.w3.org/2001/XMLSchema#decimal>"
    )
}
