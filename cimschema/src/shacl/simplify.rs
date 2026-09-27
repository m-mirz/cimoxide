use super::model::*;
use super::skip;

/// Apply all 7 normalisation rules to every FileResults.
/// Returns per-file skip entries for every constraint dropped during simplification.
pub fn simplify(results: &mut Vec<FileResults>) -> Vec<(String, Vec<skip::SkipEntry>)> {
    let mut all_skips = Vec::new();
    for fr in results.iter_mut() {
        let mut collector = skip::SkipCollector::new();
        for shape in &mut fr.shapes {
            // Only targetClass/targetNode carry an actual class-like name; the newer
            // targetSubjectsOf/targetObjectsOf/sparqlTarget kinds hold a predicate IRI or
            // blank node id (see model.rs's TargetInfo::kind doc), which would otherwise
            // show up as a bogus "class" label on skip entries below.
            let class_names: Vec<String> = shape.targets.iter()
                .filter(|t| t.kind == "targetClass" || t.kind == "targetNode")
                .map(|t| local_name(&t.value))
                .filter(|n| !n.is_empty())
                .collect();
            simplify_shape(shape, &class_names, &mut collector);
        }
        all_skips.push((fr.file_name.clone(), collector.into_entries()));
    }
    all_skips
}

fn local_name(iri: &str) -> String {
    iri.find(':').map(|i| iri[i + 1..].to_string()).unwrap_or_else(|| iri.to_string())
}

fn simplify_shape(shape: &mut ShapeInfo, class_names: &[String], collector: &mut skip::SkipCollector) {
    for prop in &mut shape.properties {
        let path = prop.path.first().map(|s| s.as_str()).unwrap_or("");
        prop.constraints = simplify_constraints(
            std::mem::take(&mut prop.constraints),
            class_names,
            path,
            collector,
        );
    }
}

/// Apply rules 1–7 to a flat list of constraints on one property shape.
fn simplify_constraints(
    constraints: Vec<ConstraintInfo>,
    class_names: &[String],
    path: &str,
    collector: &mut skip::SkipCollector,
) -> Vec<ConstraintInfo> {
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
                    push_for_classes(collector, class_names, path, &c.component, &c.name,
                        "NodeKind=Literal structurally satisfied: datatype constraint implies literal");
                    continue;
                }
                // Rule 2: Drop NodeKind=BlankNodeOrIRI and NodeKind=IRI —
                // MridRef / UriRef already enforce these in Rust.
                if nk == "sh:BlankNodeOrIRI" || nk == "sh:IRI" {
                    push_for_classes(collector, class_names, path, &c.component, &c.name,
                        "NodeKind structurally satisfied by Rust type system (MridRef/UriRef)");
                    continue;
                }
                out.push(c);
            }

            // Rule 7: Drop sh:datatype when Rust's type system already enforces it.
            "sh:DatatypeConstraintComponent" => {
                let dt = c.payload.get("datatype").and_then(|v| v.as_str()).unwrap_or("");
                if is_native_rust_type(dt) {
                    push_for_classes(collector, class_names, path, &c.component, &c.name,
                        "Datatype structurally satisfied by Rust type system");
                    continue;
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
                    push_for_classes(collector, class_names, path, &c.component, &c.name,
                        "MinCount=0 vacuously true");
                    continue;
                }

                // Rule 4: min=0 + max=1 — keep MaxCountConstraintComponent so the
                // codegen can emit a duplicate-field check; drop MinCountConstraintComponent
                // with min=0 (vacuously true) when paired with an explicit max.
                if min == 0 && max == Some(1) && c.component == "sh:MinCountConstraintComponent" {
                    push_for_classes(collector, class_names, path, &c.component, &c.name,
                        "MinCount=0 vacuously true (paired with MaxCount=1)");
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

fn push_for_classes(
    collector: &mut skip::SkipCollector,
    class_names: &[String],
    prop: &str,
    component: &str,
    name: &str,
    reason: &str,
) {
    for class in class_names {
        collector.push(class, prop, component, name, reason);
    }
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
