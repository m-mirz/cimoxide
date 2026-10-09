use super::model::*;
use super::skip;

/// Apply the normalisation rules to every FileResults.
///
/// Returns per-file skip entries for every constraint dropped here.
///
/// `sh:nodeKind` and `sh:datatype` are always kept. Rules 1, 2 and 7 used to
/// drop them for CGMES, because a generated struct field could not hold a
/// violating value; validation now reads the text as written, where a malformed
/// literal is real and only these constraints report it.
pub fn simplify(results: &mut [FileResults]) -> Vec<(String, Vec<skip::SkipEntry>)> {
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

fn simplify_shape(
    shape: &mut ShapeInfo,
    class_names: &[String],
    collector: &mut skip::SkipCollector,
) {
    // A shape the schema switched off contributes nothing, but it must be
    // accounted for rather than vanishing — an unreported drop is
    // indistinguishable from a rule that was checked and passed.
    if shape.deactivated {
        for prop in &shape.properties {
            let path = prop.path.first().map(|s| s.as_str()).unwrap_or("");
            for c in &prop.constraints {
                push_for_classes(collector, class_names, path, &c.component, &c.name,
                    "sh:deactivated — switched off by the schema");
            }
        }
        shape.properties.clear();
        shape.constraints.clear();
        shape.closed = None;
        return;
    }
    shape.properties.retain(|prop| {
        if !prop.deactivated {
            return true;
        }
        let path = prop.path.first().map(|s| s.as_str()).unwrap_or("");
        for c in &prop.constraints {
            push_for_classes(collector, class_names, path, &c.component, &c.name,
                "sh:deactivated — switched off by the schema");
        }
        false
    });
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
    let mut out: Vec<ConstraintInfo> = Vec::with_capacity(constraints.len());

    for c in constraints {
        match c.component.as_str() {
            // sh:nodeKind and sh:datatype are kept, for both families. The
            // generated validators read parsed struct fields, against which
            // both were tautologies (rules 1, 2 and 7, now gone); the shape
            // table reads the text as written, where a malformed number or a
            // literal in place of a reference is real and only these report.
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
                if let Some(vals) = values
                    && vals.len() == 1 {
                        let single = vals[0].clone();
                        let mut payload = std::collections::HashMap::new();
                        payload.insert("hasValue".to_string(), ShaclValue::Str(single));
                        if let Some(lits) = c.payload.get(LITERALS) {
                            payload.insert(LITERALS.to_string(), lits.clone());
                        }
                        out.push(ConstraintInfo {
                            component: "sh:HasValueConstraintComponent".to_string(),
                            payload,
                            ..c
                        });
                        continue;
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
