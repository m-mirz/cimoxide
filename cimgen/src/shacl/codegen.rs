use std::fmt::Write as FmtWrite;
use std::fs;
use std::path::Path;

use crate::generator::rust_gen::{sanitize_field, to_snake_case};
use crate::schema::model::{CimAttribute, CimSpecification};

use super::model::*;
use super::skip;

// ---------------------------------------------------------------------------
// Public entry point
// ---------------------------------------------------------------------------

pub fn generate_validation(
    results: &[FileResults],
    spec: &CimSpecification,
    output_dir: &Path,
) -> Result<(usize, Vec<skip::FileSkipInfo>), Box<dyn std::error::Error>> {
    fs::create_dir_all(output_dir)?;

    let mut total_checks = 0usize;
    let mut generated_modules: Vec<String> = Vec::new();
    let mut file_skips: Vec<skip::FileSkipInfo> = Vec::new();

    for fr in results {
        let (code, count, skips) = render_file(fr, spec);
        let mod_name = file_to_mod_name(&fr.file_name);
        file_skips.push(skip::FileSkipInfo { file_name: fr.file_name.clone(), check_count: count, skips });
        if count == 0 {
            continue;
        }
        total_checks += count;
        let out_path = output_dir.join(format!("generated_{mod_name}.rs"));
        fs::write(&out_path, code)?;
        generated_modules.push(mod_name);
    }

    Ok((total_checks, file_skips))
}

// ---------------------------------------------------------------------------
// Per-file rendering
// ---------------------------------------------------------------------------

fn render_file(fr: &FileResults, spec: &CimSpecification) -> (String, usize, Vec<skip::SkipEntry>) {
    let file_snake = file_to_mod_name(&fr.file_name);

    let mut regex_decls: Vec<String> = Vec::new();
    let mut regex_counter = 0usize;
    let mut check_names: Vec<String> = Vec::new();
    let mut checks_body = String::new();
    let mut used_fn_names: std::collections::HashSet<String> = std::collections::HashSet::new();
    let mut collector = skip::SkipCollector::new();
    // Tracks (path, component, name) triples -- deliberately *not* including the target
    // class -- so `count` below reports one entry per distinct rule pattern regardless of
    // how many concrete classes it's generated against, matching skip::SkipCollector's
    // existing dedup key for the Skipped side (see its doc comment) and cimgo's
    // `uniqueCheckPatterns`. `check_names`/`checks_body` above are unaffected by this: every
    // class still gets its own generated function and orchestrator call, since Rust's static
    // typing requires one function per concrete class.
    let mut unique_patterns: std::collections::HashSet<(String, String, String)> = std::collections::HashSet::new();

    for node_shape in &fr.shapes {
        for target in &node_shape.targets {
            if target.kind != "targetClass" && target.kind != "targetNode" {
                // targetSubjectsOf/targetObjectsOf/sparqlTarget carry no concrete class
                // to generate per-class checks against (see model.rs's TargetInfo::kind
                // doc). Record why instead of silently generating nothing.
                push_unsupported_target_skips(node_shape, &target.kind, &mut collector);
                continue;
            }
            let class_name = local_name(&target.value);
            if spec.types.get(&class_name).is_none() {
                continue; // class not in schema (e.g. md:FullModel)
            }

            // Compound node-level constraints (sh:xone, sh:or, sh:and)
            for node_constraint in &node_shape.constraints {
                match gen_compound_check(&file_snake, &class_name, spec, node_constraint) {
                    Ok(fn_code) => {
                        let raw_name = extract_fn_name(&fn_code);
                        let fn_name = dedup_fn_name(raw_name.clone(), &used_fn_names);
                        let patched = if fn_name != raw_name {
                            fn_code.replacen(&format!("pub fn {raw_name}("), &format!("pub fn {fn_name}("), 1)
                        } else { fn_code };
                        used_fn_names.insert(fn_name.clone());
                        check_names.push(fn_name);
                        checks_body.push_str(&patched);
                        checks_body.push('\n');
                        unique_patterns.insert((String::new(), node_constraint.component.clone(), node_constraint.name.clone()));
                    }
                    Err(reason) => {
                        collector.push(&class_name, "", &node_constraint.component, &node_constraint.name, &reason);
                    }
                }
            }

            for prop_shape in &node_shape.properties {
                if prop_shape.path.is_empty() {
                    continue;
                }
                for constraint in &prop_shape.constraints {
                    let (opt_code, opt_regex) = render_check(
                        &file_snake,
                        &class_name,
                        spec,
                        &prop_shape.path,
                        constraint,
                        &mut regex_counter,
                        &mut collector,
                    );
                    if let Some(fn_code) = opt_code {
                        if let Some(decl) = opt_regex {
                            regex_decls.push(decl);
                        }
                        let raw_name = extract_fn_name(&fn_code);
                        let fn_name = dedup_fn_name(raw_name.clone(), &used_fn_names);
                        let patched = if fn_name != raw_name {
                            fn_code.replacen(&format!("pub fn {raw_name}("), &format!("pub fn {fn_name}("), 1)
                        } else { fn_code };
                        used_fn_names.insert(fn_name.clone());
                        check_names.push(fn_name);
                        checks_body.push_str(&patched);
                        checks_body.push('\n');
                        unique_patterns.insert((constraint.path.join("/"), constraint.component.clone(), constraint.name.clone()));
                    }
                }
            }
        }
    }

    let count = unique_patterns.len();
    if check_names.is_empty() {
        return (String::new(), 0, collector.into_entries());
    }

    let mut s = String::new();
    writeln!(s, "// Generated by cimgen — do not edit by hand.").unwrap();
    writeln!(s, "#![allow(clippy::all, dead_code, unused)]").unwrap();
    writeln!(s, "use cimdecoder::CimDataset;").unwrap();
    writeln!(s, "use crate::Violation;").unwrap();
    writeln!(s, "use crate::helpers::*;").unwrap();
    writeln!(s).unwrap();

    if !regex_decls.is_empty() {
        writeln!(s, "use std::sync::LazyLock;").unwrap();
        for decl in &regex_decls {
            s.push_str(decl);
            s.push('\n');
        }
        writeln!(s).unwrap();
    }

    let orchestrator = format!("validate_{file_snake}");
    writeln!(s, "pub fn {orchestrator}(dataset: &CimDataset) -> Vec<Violation> {{").unwrap();
    writeln!(s, "    let mut v = Vec::new();").unwrap();
    for name in &check_names {
        writeln!(s, "    v.extend({name}(dataset));").unwrap();
    }
    writeln!(s, "    v").unwrap();
    writeln!(s, "}}").unwrap();
    writeln!(s).unwrap();

    s.push_str(&checks_body);

    (s, count, collector.into_entries())
}

/// Records why every constraint under `node_shape` isn't code-generated when the shape's
/// target is a kind with no resolvable concrete class (targetSubjectsOf/targetObjectsOf/
/// sparqlTarget) -- otherwise these would resolve to zero checks with no skip entry at
/// all, an invisible gap one level down from the shape being dropped entirely at parse
/// time (which ttl_import.rs no longer does).
fn push_unsupported_target_skips(node_shape: &ShapeInfo, kind: &str, collector: &mut skip::SkipCollector) {
    let label = format!("({kind})");
    let reason = match kind {
        "targetSubjectsOf" | "targetObjectsOf" => {
            "sh:targetSubjectsOf/targetObjectsOf target: property-based target, no concrete class to generate checks against".to_string()
        }
        _ => "sh:target SPARQLTarget: needs a hand-written implementation, not a SPARQL evaluator".to_string(),
    };
    for node_constraint in &node_shape.constraints {
        collector.push(&label, "", &node_constraint.component, &node_constraint.name, &reason);
    }
    for prop_shape in &node_shape.properties {
        let full_path_key = prop_shape.path.join("/");
        for constraint in &prop_shape.constraints {
            collector.push(&label, &full_path_key, &constraint.component, &constraint.name, &reason);
        }
    }
}

// ---------------------------------------------------------------------------
// Per-constraint rendering
// ---------------------------------------------------------------------------

fn render_check(
    file_snake: &str,
    class_name: &str,
    spec: &CimSpecification,
    full_path: &[String],
    constraint: &ConstraintInfo,
    regex_counter: &mut usize,
    collector: &mut skip::SkipCollector,
) -> (Option<String>, Option<String>) {
    let path_seg = full_path[0].as_str();
    let attr_id = local_name(path_seg);
    let is_multi_seg = full_path.len() > 1;
    // Full path, for skip-entry dedup only -- unlike path_seg (used for fn naming
    // and multi-segment routing), this must distinguish constraints that share a
    // sh:name and first path segment but diverge deeper in the path (e.g. two
    // property shapes both named "...:containment", one checking
    // Equipment.EquipmentContainer, the other Equipment.EquipmentContainer's own
    // DCConverterUnit.Substation) -- otherwise they'd wrongly collapse into one
    // skip entry, undercounting relative to the Generated-side dedup below (which
    // already keys on the full constraint.path) and cimgo's equivalent PathKey.
    let full_path_key = full_path.join("/");

    let comp = constraint.component.as_str();

    // Compute fn_name early so all branches can use it.
    let attr_part = to_snake_case(&attr_id.replace('.', "_"));
    let comp_suffix = component_suffix(comp);
    let raw_name = format!(
        "check_{file_snake}_{class_snake}_{attr_part}_{comp_suffix}",
        class_snake = to_snake_case(class_name),
    );
    let fn_name = safe_fn_name(&raw_name);

    // Multi-segment paths: chains of 0..1 reference hops, resolved against the
    // dataset one link at a time (mirroring cimgo's chain-walker semantics: any
    // missing/unresolvable link means the path yields no value and the object is
    // skipped, except sh:Required which collapses onto the first link's presence).
    if is_multi_seg {
        let is_inverse_head = path_seg.starts_with('^');
        let ends_rdf_type = full_path.last().map(|s| s == "rdf:type").unwrap_or(false);

        // [field, rdf:type] sh:In / sh:HasValue — single-hop reference type check.
        if full_path.len() == 2 && ends_rdf_type && !is_inverse_head
            && matches!(comp, "sh:InConstraintComponent" | "sh:HasValueConstraintComponent")
        {
            return match gen_slice_mrid_rdf_type_check(&fn_name, class_name, spec, &attr_id, constraint) {
                Ok((code, regex)) => (Some(code), regex),
                Err(reason) => {
                    collector.push(class_name, &full_path_key, comp, &constraint.name, &reason);
                    (None, None)
                }
            };
        }

        // Difference-model header paths into rdf:Statement members: subject/
        // predicate/object are mandated by the RDF specification for every
        // rdf:Statement, and rdf:Statement resources are not decoded into the
        // dataset — the constraint can neither fire nor be violated.
        if full_path.iter().any(|s| s.starts_with("rdf:Statements.")) {
            collector.push(class_name, &full_path_key, comp, &constraint.name,
                "multi-segment path into rdf:Statement members: guaranteed by the RDF specification, rdf:Statement resources are not decoded");
            return (None, None);
        }

        // Every hop in a forward CIM reference chain is a 0..1 field, so the
        // value count can never exceed 1 — sh:maxCount 1 is structurally satisfied.
        if comp == "sh:MaxCountConstraintComponent" {
            collector.push(class_name, &full_path_key, comp, &constraint.name,
                "multi-segment MaxCount=1 structurally satisfied: every hop is a 0..1 field");
            return (None, None);
        }
        // The decoded Rust type fixes the value kind of every path step.
        if comp == "sh:NodeKindConstraintComponent" {
            collector.push(class_name, &full_path_key, comp, &constraint.name,
                "multi-segment NodeKind structurally satisfied: value kind is fixed by the decoded Rust type");
            return (None, None);
        }

        let result = if !is_inverse_head {
            match comp {
                // sh:Required over a chain ending in rdf:type collapses onto the
                // first link's presence (cimgo parity): later links that don't
                // resolve are indistinguishable from data split across files.
                "sh:RequiredConstraintComponent" | "sh:MinCountConstraintComponent" if ends_rdf_type => {
                    Some(gen_chain_required_first_link(&fn_name, class_name, spec, &attr_id, constraint))
                }
                // sh:HasValue with an rdf:type tail: walk the chain, the final
                // entry's class must be the hasValue class (or a subclass).
                "sh:HasValueConstraintComponent" if ends_rdf_type => {
                    let allowed = match constraint.payload.get("hasValue").and_then(|v| v.as_str()) {
                        Some(v) => class_and_subclasses(spec, &local_name(v)),
                        None => Vec::new(),
                    };
                    if allowed.is_empty() {
                        Some(Err("chain sh:HasValue: value class not decodable".to_string()))
                    } else {
                        Some(gen_forward_chain_type_check(&fn_name, class_name, spec,
                            &full_path[..full_path.len() - 1], &allowed, constraint))
                    }
                }
                // sh:or of sh:class alternatives on an association tail: walk the
                // chain, the final referenced entry's class must be in the allow-list.
                "sh:OrClassConstraintComponent" if !ends_rdf_type => {
                    let raw: Vec<String> = constraint.payload.get("classes").and_then(|v| v.as_list())
                        .map(|v| v.iter().map(|s| local_name(s)).collect())
                        .unwrap_or_default();
                    let mut set = std::collections::BTreeSet::new();
                    for cls in &raw {
                        for t in class_and_subclasses(spec, cls) { set.insert(t); }
                    }
                    let allowed: Vec<String> = set.into_iter().collect();
                    if allowed.is_empty() || allowed.len() >= spec.types.len() {
                        Some(Err("chain sh:OrClass: empty or vacuous class list".to_string()))
                    } else {
                        Some(gen_forward_chain_type_check(&fn_name, class_name, spec,
                            full_path, &allowed, constraint))
                    }
                }
                // sh:datatype on a primitive leaf at the end of a chain.
                "sh:DatatypeConstraintComponent" if !ends_rdf_type => {
                    Some(gen_forward_chain_datatype_check(&fn_name, class_name, spec, full_path, constraint))
                }
                _ => None,
            }
        } else if full_path.len() == 2 && comp == "sh:HasValueConstraintComponent" {
            // [^forward-ref, attr] sh:HasValue: at least one referrer must carry
            // the expected attribute value.
            Some(gen_inverse_chain_has_value(&fn_name, class_name, spec, path_seg, &full_path[1], constraint))
        } else {
            None
        };

        return match result {
            Some(Ok((code, regex))) => (Some(code), regex),
            Some(Err(reason)) => {
                collector.push(class_name, &full_path_key, comp, &constraint.name, &reason);
                (None, None)
            }
            None => {
                collector.push(class_name, &full_path_key, comp, &constraint.name, "multi-segment path not supported");
                (None, None)
            }
        };
    }

    // Inverse path (encoded as "^<forward-iri>" by the parser) — route to dedicated generator
    // before the component skip list so MaxCount is not dropped for inverse shapes.
    if path_seg.starts_with('^') {
        let forward_pred = &path_seg[1..]; // strip "^"
        return match gen_inverse_count(&fn_name, class_name, forward_pred, spec, constraint) {
            Ok((code, regex)) => (Some(code), regex),
            Err(reason) => {
                collector.push(class_name, &full_path_key, comp, &constraint.name, &reason);
                (None, None)
            }
        };
    }

    match comp {
        "sh:NodeKindConstraintComponent" => {
            collector.push(class_name, &full_path_key, comp, &constraint.name,
                "sh:NodeKindConstraintComponent structurally satisfied by Rust type system");
            return (None, None);
        }
        "sh:SPARQLConstraintComponent" => {
            collector.push(class_name, &full_path_key, comp, &constraint.name,
                "sh:SPARQLConstraintComponent: needs a hand-written implementation, not a SPARQL evaluator");
            return (None, None);
        }
        "sh:OrInversePathConstraintComponent" => {
            collector.push(class_name, &full_path_key, comp, &constraint.name,
                "sh:OrInversePathConstraintComponent structurally satisfied");
            return (None, None);
        }
        _ => {}
    }

    let (accessor_prefix, attr) = match find_attr_in_hierarchy(spec, class_name, &attr_id) {
        Some(pair) => pair,
        None => {
            collector.push(class_name, &full_path_key, comp, &constraint.name,
                &format!("attribute {} not found in hierarchy", attr_id));
            return (None, None);
        }
    };
    if !attr.is_association_used {
        collector.push(class_name, &full_path_key, comp, &constraint.name, "unused association");
        return (None, None);
    }

    let field_name = sanitize_field(to_snake_case(&attr.label));
    let full_accessor = format!("{accessor_prefix}.{field_name}");

    let result = match comp {
        "sh:RequiredConstraintComponent" | "sh:MinCountConstraintComponent" => {
            gen_required(&fn_name, class_name, &full_accessor, &attr, constraint)
        }
        "sh:MaxCountConstraintComponent" => {
            gen_max_count_one(&fn_name, class_name, &attr_id, &attr, constraint)
        }
        "sh:ExactCountConstraintComponent" => {
            gen_exact_count(&fn_name, class_name, &full_accessor, &attr, constraint)
        }
        "sh:InConstraintComponent" => {
            gen_in(&fn_name, class_name, &full_accessor, &attr, constraint)
        }
        "sh:HasValueConstraintComponent" => {
            gen_has_value(&fn_name, class_name, &full_accessor, &attr, constraint)
        }
        "sh:DatatypeConstraintComponent" => {
            gen_datatype(&fn_name, class_name, &full_accessor, &attr, constraint)
        }
        "sh:PatternConstraintComponent" => {
            gen_pattern(&fn_name, class_name, &full_accessor, &attr, constraint, regex_counter)
        }
        "sh:MinLengthConstraintComponent" => {
            gen_length(&fn_name, class_name, &full_accessor, &attr, constraint, true)
        }
        "sh:MaxLengthConstraintComponent" => {
            gen_length(&fn_name, class_name, &full_accessor, &attr, constraint, false)
        }
        "sh:MinExclusiveConstraintComponent"
        | "sh:MaxExclusiveConstraintComponent"
        | "sh:MinInclusiveConstraintComponent"
        | "sh:MaxInclusiveConstraintComponent" => {
            gen_numeric_range(&fn_name, class_name, &full_accessor, &attr, constraint)
        }
        "sh:LessThanConstraintComponent" => {
            gen_less_than(&fn_name, class_name, spec, &full_accessor, &attr, constraint)
        }
        "sh:LessThanOrEqualsConstraintComponent" => {
            gen_less_than_or_equals(&fn_name, class_name, spec, &full_accessor, &attr, constraint)
        }
        "sh:ClassConstraintComponent" => {
            gen_class(&fn_name, class_name, spec, &full_accessor, &attr, constraint)
        }
        "sh:OrClassConstraintComponent" => {
            gen_or_class(&fn_name, class_name, spec, &full_accessor, &attr, constraint)
        }
        "sh:NotClassConstraintComponent" => {
            gen_not_class(&fn_name, class_name, &full_accessor, &attr, constraint)
        }
        _ => Err(format!("unknown component: {comp}")),
    };
    match result {
        Ok((code, regex)) => (Some(code), regex),
        Err(reason) => {
            collector.push(class_name, &full_path_key, comp, &constraint.name, &reason);
            (None, None)
        }
    }
}

// ---------------------------------------------------------------------------
// Constraint generators
// ---------------------------------------------------------------------------

fn gen_required(
    fn_name: &str,
    class_name: &str,
    accessor: &str,
    attr: &CimAttribute,
    c: &ConstraintInfo,
) -> Result<(String, Option<String>), String> {
    let condition = if attr.is_list {
        format!("{accessor}.is_empty()")
    } else if attr.is_primitive || attr.is_cim_datatype {
        match attr.lang_type.as_str() {
            "String" => format!("{accessor}.is_empty()"),
            "bool" | "i64" | "f64" => format!("{accessor}.is_none()"),
            _ => format!("{accessor}.is_empty()"),
        }
    } else if attr.is_enum_value {
        format!("{accessor}.is_none()")
    } else {
        format!("{accessor}.is_none()")
    };
    Ok((build_fn(fn_name, class_name, None, None, &condition, c, attr), None))
}

fn gen_max_count_one(
    fn_name: &str,
    class_name: &str,
    attr_id: &str,
    attr: &CimAttribute,
    c: &ConstraintInfo,
) -> Result<(String, Option<String>), String> {
    let max = c.payload.get("maxCount").and_then(|v| v.as_int()).unwrap_or(1);
    if max != 1 {
        return Err(format!("sh:MaxCount={max} on non-list field (only maxCount=1 is supported)"));
    }
    if attr.is_list {
        return Err("sh:MaxCount=1 on list field — use sh:ExactCount instead".to_string());
    }
    fn esc(s: &str) -> String { s.replace('\\', "\\\\").replace('"', "\\\"") }
    let message  = esc(&c.message);
    let severity = &c.severity;
    let name_str    = esc(&c.name);
    let rule_id_str = esc(&c.rule_id);
    let desc_str = esc(&c.description);
    let prop = &attr.id;

    let mut s = String::new();
    writeln!(s, "pub fn {fn_name}(dataset: &CimDataset) -> Vec<Violation> {{").unwrap();
    writeln!(s, "    let mut violations = Vec::new();").unwrap();
    writeln!(s, "    for mrid in dataset.by_type.get(\"{class_name}\").into_iter().flatten() {{").unwrap();
    writeln!(s, "        if dataset.entries[mrid].block.duplicate_fields.contains(\"{attr_id}\") {{").unwrap();
    writeln!(s, "            violations.push(Violation {{").unwrap();
    writeln!(s, "                object_id:   mrid.clone(),").unwrap();
    writeln!(s, "                rule_id:     \"{rule_id_str}\".to_string(),").unwrap();
    writeln!(s, "                class:       \"{class_name}\".to_string(),").unwrap();
    writeln!(s, "                property:    \"{prop}\".to_string(),").unwrap();
    writeln!(s, "                message:     \"{message}\".to_string(),").unwrap();
    writeln!(s, "                severity:    \"{severity}\".to_string(),").unwrap();
    writeln!(s, "                name:        \"{name_str}\".to_string(),").unwrap();
    writeln!(s, "                description: \"{desc_str}\".to_string(),").unwrap();
    writeln!(s, "            }});").unwrap();
    writeln!(s, "        }}").unwrap();
    writeln!(s, "    }}").unwrap();
    writeln!(s, "    violations").unwrap();
    writeln!(s, "}}").unwrap();
    Ok((s, None))
}

fn gen_exact_count(
    fn_name: &str,
    class_name: &str,
    accessor: &str,
    attr: &CimAttribute,
    c: &ConstraintInfo,
) -> Result<(String, Option<String>), String> {
    if !attr.is_list {
        return Err("sh:ExactCount on non-list field".to_string());
    }
    let n = c.payload.get("minCount").and_then(|v| v.as_int()).unwrap_or(1);
    let condition = format!("{accessor}.len() != {n}");
    Ok((build_fn(fn_name, class_name, None, None, &condition, c, attr), None))
}

/// Literal form an enum value takes after decoding: the decoder strips enum
/// resource URIs to their fragment (`http://...#PhaseCode.ABC` → `PhaseCode.ABC`),
/// so comparison values from the TTL (prefix-simplified, e.g. `cim:PhaseCode.ABC`)
/// must be reduced the same way or the check can never match.
fn enum_literal(v: &str) -> String {
    if let Some(i) = v.rfind('#') {
        v[i + 1..].to_string()
    } else {
        local_name(v)
    }
}

fn gen_in(
    fn_name: &str,
    class_name: &str,
    accessor: &str,
    attr: &CimAttribute,
    c: &ConstraintInfo,
) -> Result<(String, Option<String>), String> {
    if attr.is_list {
        if (attr.is_primitive || attr.is_cim_datatype) && attr.lang_type == "Vec<String>" {
            return gen_slice_string_in(fn_name, class_name, accessor, attr, c);
        }
        return Err("sh:In on non-string list field".to_string());
    }
    if (attr.is_primitive || attr.is_cim_datatype) && attr.lang_type != "String" {
        return Err(format!("sh:In on non-string primitive field ({})", attr.lang_type));
    }
    let values = match c.payload.get("in").and_then(|v| v.as_list()) {
        Some(v) if !v.is_empty() => v,
        _ => return Err("sh:In: empty payload".to_string()),
    };

    // Enum URIs and MridRef resources are both fragment-stripped by the decoder;
    // only plain string literals are stored verbatim.
    let is_string_literal = (attr.is_primitive || attr.is_cim_datatype) && attr.lang_type == "String";
    let allowed_items: Vec<String> = values
        .iter()
        .map(|v| {
            let lit = if is_string_literal { v.clone() } else { enum_literal(v) };
            format!("\"{}\"", lit.replace('"', "\\\""))
        })
        .collect();
    let allowed_str = allowed_items.join(", ");

    let (field_expr, guard) = field_expr_and_guard(accessor, attr);
    let prelude = format!("    const ALLOWED: &[&str] = &[{allowed_str}];\n");
    let condition = format!("!ALLOWED.contains(&{field_expr})");

    Ok((build_fn(fn_name, class_name, guard.as_deref(), Some(&prelude), &condition, c, attr), None))
}

fn gen_has_value(
    fn_name: &str,
    class_name: &str,
    accessor: &str,
    attr: &CimAttribute,
    c: &ConstraintInfo,
) -> Result<(String, Option<String>), String> {
    if attr.is_list { return Err("sh:HasValue on list field".to_string()); }
    if (attr.is_primitive || attr.is_cim_datatype) && attr.lang_type != "String" {
        return Err(format!("sh:HasValue on non-string primitive field ({})", attr.lang_type));
    }
    let expected = match c.payload.get("hasValue").and_then(|v| v.as_str()) {
        Some(v) => v.to_string(),
        None => return Err("sh:HasValue: missing payload".to_string()),
    };
    let esc = expected.replace('"', "\\\"");

    let (condition, guard) = if attr.is_enum_value {
        let lit = enum_literal(&expected).replace('"', "\\\"");
        (format!("{accessor}.as_ref().map_or(true, |u| u.uri != \"{lit}\")"), None)
    } else if attr.is_primitive || attr.is_cim_datatype {
        (format!("{accessor} != \"{esc}\""),
         Some(format!("if {accessor}.is_empty() {{ continue; }}")))
    } else {
        // MridRef resources are fragment-stripped by the decoder, like enum URIs.
        let lit = enum_literal(&expected).replace('"', "\\\"");
        (format!("{accessor}.as_ref().map_or(true, |r| r.mrid != \"{lit}\")"), None)
    };

    Ok((build_fn(fn_name, class_name, guard.as_deref(), None, &condition, c, attr), None))
}

fn gen_datatype(
    fn_name: &str,
    class_name: &str,
    accessor: &str,
    attr: &CimAttribute,
    c: &ConstraintInfo,
) -> Result<(String, Option<String>), String> {
    if attr.lang_type != "String" && attr.lang_type != "Vec<String>" {
        return Err(format!("sh:Datatype on non-string field ({})", attr.lang_type));
    }
    let dt = c.payload.get("datatype").and_then(|v| v.as_str()).unwrap_or("");
    let check_fn = match dt {
        "xsd:dateTime" | "<http://www.w3.org/2001/XMLSchema#dateTime>" => "is_xsd_datetime",
        "xsd:date" | "<http://www.w3.org/2001/XMLSchema#date>" => "is_xsd_date",
        "xsd:gMonthDay" | "<http://www.w3.org/2001/XMLSchema#gMonthDay>" => "is_xsd_gmonthday",
        "xsd:anyURI" | "<http://www.w3.org/2001/XMLSchema#anyURI>" => "is_xsd_anyuri",
        _ => return Err(format!("sh:Datatype: unsupported datatype {:?}", dt)),
    };
    if attr.lang_type == "Vec<String>" {
        return Ok((gen_slice_string_datatype(fn_name, class_name, accessor, attr, c, check_fn), None));
    }
    let guard = format!("if {accessor}.is_empty() {{ continue; }}");
    let condition = format!("!{check_fn}(&{accessor})");
    Ok((build_fn(fn_name, class_name, Some(&guard), None, &condition, c, attr), None))
}

/// sh:Datatype format check applied per-element to a `Vec<String>` field
/// (e.g. md:Model.profile, one xsd:anyURI value per declared profile).
fn gen_slice_string_datatype(
    fn_name: &str,
    class_name: &str,
    accessor: &str,
    attr: &CimAttribute,
    c: &ConstraintInfo,
    check_fn: &str,
) -> String {
    fn esc(x: &str) -> String { x.replace('\\', "\\\\").replace('"', "\\\"") }
    let message  = esc(&c.message);
    let severity = &c.severity;
    let name_str    = esc(&c.name);
    let rule_id_str = esc(&c.rule_id);
    let desc_str = esc(&c.description);
    let prop = &attr.id;

    let mut s = String::new();
    writeln!(s, "pub fn {fn_name}(dataset: &CimDataset) -> Vec<Violation> {{").unwrap();
    writeln!(s, "    let mut violations = Vec::new();").unwrap();
    writeln!(s, "    for mrid in dataset.by_type.get(\"{class_name}\").into_iter().flatten() {{").unwrap();
    writeln!(s, "        let entry = &dataset.entries[mrid];").unwrap();
    writeln!(s, "        let obj = match entry.element.as_any()").unwrap();
    writeln!(s, "            .downcast_ref::<cimstructs::{class_name}>() {{").unwrap();
    writeln!(s, "            Some(o) => o, None => continue,").unwrap();
    writeln!(s, "        }};").unwrap();
    writeln!(s, "        for val in &{accessor} {{").unwrap();
    writeln!(s, "            if val.is_empty() {{ continue; }}").unwrap();
    writeln!(s, "            if !{check_fn}(val) {{").unwrap();
    writeln!(s, "                violations.push(Violation {{").unwrap();
    writeln!(s, "                    object_id:   mrid.clone(),").unwrap();
    writeln!(s, "                    rule_id:     \"{rule_id_str}\".to_string(),").unwrap();
    writeln!(s, "                    class:       \"{class_name}\".to_string(),").unwrap();
    writeln!(s, "                    property:    \"{prop}\".to_string(),").unwrap();
    writeln!(s, "                    message:     \"{message}\".to_string(),").unwrap();
    writeln!(s, "                    severity:    \"{severity}\".to_string(),").unwrap();
    writeln!(s, "                    name:        \"{name_str}\".to_string(),").unwrap();
    writeln!(s, "                    description: \"{desc_str}\".to_string(),").unwrap();
    writeln!(s, "                }});").unwrap();
    writeln!(s, "            }}").unwrap();
    writeln!(s, "        }}").unwrap();
    writeln!(s, "    }}").unwrap();
    writeln!(s, "    violations").unwrap();
    writeln!(s, "}}").unwrap();
    s
}

fn gen_pattern(
    fn_name: &str,
    class_name: &str,
    accessor: &str,
    attr: &CimAttribute,
    c: &ConstraintInfo,
    regex_counter: &mut usize,
) -> Result<(String, Option<String>), String> {
    if attr.lang_type != "String" {
        return Err(format!("sh:Pattern on non-string field ({})", attr.lang_type));
    }
    let pattern = match c.payload.get("pattern").and_then(|v| v.as_str()) {
        Some(p) => p.to_string(),
        None => return Err("sh:Pattern: missing pattern payload".to_string()),
    };
    let idx = *regex_counter;
    *regex_counter += 1;
    let static_name = format!("REGEX_{idx}");
    let escaped = pattern.replace('\\', "\\\\").replace('"', "\\\"");
    let regex_decl = format!(
        "static {static_name}: LazyLock<regex::Regex> = LazyLock::new(|| regex::Regex::new(\"{escaped}\").expect(\"SHACL regex\"));\n"
    );
    let guard = format!("if {accessor}.is_empty() {{ continue; }}");
    let condition = format!("!{static_name}.is_match(&{accessor})");
    Ok((build_fn(fn_name, class_name, Some(&guard), None, &condition, c, attr), Some(regex_decl)))
}

fn gen_length(
    fn_name: &str,
    class_name: &str,
    accessor: &str,
    attr: &CimAttribute,
    c: &ConstraintInfo,
    is_min: bool,
) -> Result<(String, Option<String>), String> {
    if attr.lang_type != "String" {
        return Err(format!("sh:Length on non-string field ({})", attr.lang_type));
    }
    let key = if is_min { "minLength" } else { "maxLength" };
    let n = match c.payload.get(key).and_then(|v| v.as_int()) {
        Some(n) => n,
        None => return Err("sh:Length: missing payload".to_string()),
    };
    let op = if is_min { "<" } else { ">" };
    let guard = format!("if {accessor}.is_empty() {{ continue; }}");
    let condition = format!("{accessor}.chars().count() {op} {n}");
    Ok((build_fn(fn_name, class_name, Some(&guard), None, &condition, c, attr), None))
}

fn gen_numeric_range(
    fn_name: &str,
    class_name: &str,
    accessor: &str,
    attr: &CimAttribute,
    c: &ConstraintInfo,
) -> Result<(String, Option<String>), String> {
    let comp = c.component.as_str();
    let (key, op) = match comp {
        "sh:MinExclusiveConstraintComponent" => ("minExclusive", "<="),
        "sh:MaxExclusiveConstraintComponent" => ("maxExclusive", ">="),
        "sh:MinInclusiveConstraintComponent" => ("minInclusive", "<"),
        "sh:MaxInclusiveConstraintComponent" => ("maxInclusive", ">"),
        _ => return Err(format!("unknown numeric range component: {comp}")),
    };
    let val = match c.payload.get(key).and_then(|v| v.as_float()) {
        Some(f) => f,
        None => return Err("sh:NumericRange: missing payload".to_string()),
    };
    let (guard, condition) = match attr.lang_type.as_str() {
        "f64" => (
            format!("if {accessor}.is_none() {{ continue; }}"),
            format!("{accessor}.unwrap() {op} {val}_f64"),
        ),
        "i64" => (
            format!("if {accessor}.is_none() {{ continue; }}"),
            format!("({accessor}.unwrap() as f64) {op} {val}_f64"),
        ),
        _ => return Err(format!("sh:NumericRange on non-numeric field ({})", attr.lang_type)),
    };
    Ok((build_fn(fn_name, class_name, Some(&guard), None, &condition, c, attr), None))
}

fn gen_less_than(
    fn_name: &str,
    class_name: &str,
    spec: &CimSpecification,
    accessor: &str,
    attr: &CimAttribute,
    c: &ConstraintInfo,
) -> Result<(String, Option<String>), String> {
    gen_less_than_cmp(fn_name, class_name, spec, accessor, attr, c, "lessThan", "sh:LessThan", "<")
}

fn gen_less_than_or_equals(
    fn_name: &str,
    class_name: &str,
    spec: &CimSpecification,
    accessor: &str,
    attr: &CimAttribute,
    c: &ConstraintInfo,
) -> Result<(String, Option<String>), String> {
    gen_less_than_cmp(fn_name, class_name, spec, accessor, attr, c, "lessThanOrEquals", "sh:LessThanOrEquals", "<=")
}

/// Shared generator for sh:LessThan / sh:LessThanOrEquals: both compare two
/// numeric fields on the same decoded object, differing only in the payload
/// key and the comparison operator.
fn gen_less_than_cmp(
    fn_name: &str,
    class_name: &str,
    spec: &CimSpecification,
    accessor: &str,
    attr: &CimAttribute,
    c: &ConstraintInfo,
    payload_key: &str,
    label: &str,
    op: &str,
) -> Result<(String, Option<String>), String> {
    if !matches!(attr.lang_type.as_str(), "f64" | "i64") {
        return Err(format!("{label} on non-numeric field ({})", attr.lang_type));
    }
    let other_iri = match c.payload.get(payload_key).and_then(|v| v.as_str()) {
        Some(v) => v.to_string(),
        None => return Err(format!("{label}: missing {payload_key} payload")),
    };
    let other_attr_id = local_name(&other_iri);
    let (other_prefix, other_attr) = match find_attr_in_hierarchy(spec, class_name, &other_attr_id) {
        Some(p) => p,
        None => return Err(format!("{label}: other attribute {} not found in hierarchy", other_attr_id)),
    };
    if !matches!(other_attr.lang_type.as_str(), "f64" | "i64") {
        return Err(format!("{label}: other attribute {} is non-numeric ({})", other_attr_id, other_attr.lang_type));
    }
    let other_field = sanitize_field(to_snake_case(&other_attr.label));
    let other_accessor = format!("{other_prefix}.{other_field}");

    let mut s = String::new();
    fn esc(x: &str) -> String { x.replace('\\', "\\\\").replace('"', "\\\"") }
    let message  = esc(&c.message);
    let severity = &c.severity;
    let name_str    = esc(&c.name);
    let rule_id_str = esc(&c.rule_id);
    let desc_str = esc(&c.description);
    let prop = &attr.id;

    writeln!(s, "pub fn {fn_name}(dataset: &CimDataset) -> Vec<Violation> {{").unwrap();
    writeln!(s, "    let mut violations = Vec::new();").unwrap();
    writeln!(s, "    for mrid in dataset.by_type.get(\"{class_name}\").into_iter().flatten() {{").unwrap();
    writeln!(s, "        let entry = &dataset.entries[mrid];").unwrap();
    writeln!(s, "        let obj = match entry.element.as_any()").unwrap();
    writeln!(s, "            .downcast_ref::<cimstructs::{class_name}>() {{").unwrap();
    writeln!(s, "            Some(o) => o, None => continue,").unwrap();
    writeln!(s, "        }};").unwrap();
    writeln!(s, "        if {accessor}.is_none() || {other_accessor}.is_none() {{ continue; }}").unwrap();
    writeln!(s, "        if !(({accessor}.unwrap() as f64) {op} {other_accessor}.unwrap() as f64) {{").unwrap();
    writeln!(s, "            violations.push(Violation {{").unwrap();
    writeln!(s, "                object_id:   mrid.clone(),").unwrap();
    writeln!(s, "                rule_id:     \"{rule_id_str}\".to_string(),").unwrap();
    writeln!(s, "                class:       \"{class_name}\".to_string(),").unwrap();
    writeln!(s, "                property:    \"{prop}\".to_string(),").unwrap();
    writeln!(s, "                message:     \"{message}\".to_string(),").unwrap();
    writeln!(s, "                severity:    \"{severity}\".to_string(),").unwrap();
    writeln!(s, "                name:        \"{name_str}\".to_string(),").unwrap();
    writeln!(s, "                description: \"{desc_str}\".to_string(),").unwrap();
    writeln!(s, "            }});").unwrap();
    writeln!(s, "        }}").unwrap();
    writeln!(s, "    }}").unwrap();
    writeln!(s, "    violations").unwrap();
    writeln!(s, "}}").unwrap();
    Ok((s, None))
}

fn gen_not_class(
    fn_name: &str,
    class_name: &str,
    accessor: &str,
    attr: &CimAttribute,
    c: &ConstraintInfo,
) -> Result<(String, Option<String>), String> {
    if attr.is_primitive || attr.is_cim_datatype || attr.is_enum_value || attr.is_list {
        return Err("sh:NotClass on non-association field".to_string());
    }
    let forbidden = match c.payload.get("class").and_then(|v| v.as_str()) {
        Some(v) => local_name(v),
        None => return Err("sh:NotClass: missing class payload".to_string()),
    };

    let mut s = String::new();
    fn esc(x: &str) -> String { x.replace('\\', "\\\\").replace('"', "\\\"") }
    let message  = esc(&c.message);
    let severity = &c.severity;
    let name_str    = esc(&c.name);
    let rule_id_str = esc(&c.rule_id);
    let desc_str = esc(&c.description);
    let prop = &attr.id;

    writeln!(s, "pub fn {fn_name}(dataset: &CimDataset) -> Vec<Violation> {{").unwrap();
    writeln!(s, "    let mut violations = Vec::new();").unwrap();
    writeln!(s, "    for mrid in dataset.by_type.get(\"{class_name}\").into_iter().flatten() {{").unwrap();
    writeln!(s, "        let entry = &dataset.entries[mrid];").unwrap();
    writeln!(s, "        let obj = match entry.element.as_any()").unwrap();
    writeln!(s, "            .downcast_ref::<cimstructs::{class_name}>() {{").unwrap();
    writeln!(s, "            Some(o) => o, None => continue,").unwrap();
    writeln!(s, "        }};").unwrap();
    writeln!(s, "        let ref_mrid = match {accessor}.as_ref() {{").unwrap();
    writeln!(s, "            Some(r) => &r.mrid, None => continue,").unwrap();
    writeln!(s, "        }};").unwrap();
    writeln!(s, "        if let Some(ref_entry) = dataset.entries.get(ref_mrid) {{").unwrap();
    writeln!(s, "            if ref_entry.element.type_name() == \"{forbidden}\" {{").unwrap();
    writeln!(s, "                violations.push(Violation {{").unwrap();
    writeln!(s, "                    object_id:   mrid.clone(),").unwrap();
    writeln!(s, "                    rule_id:     \"{rule_id_str}\".to_string(),").unwrap();
    writeln!(s, "                    class:       \"{class_name}\".to_string(),").unwrap();
    writeln!(s, "                    property:    \"{prop}\".to_string(),").unwrap();
    writeln!(s, "                    message:     \"{message}\".to_string(),").unwrap();
    writeln!(s, "                    severity:    \"{severity}\".to_string(),").unwrap();
    writeln!(s, "                    name:        \"{name_str}\".to_string(),").unwrap();
    writeln!(s, "                    description: \"{desc_str}\".to_string(),").unwrap();
    writeln!(s, "                }});").unwrap();
    writeln!(s, "            }}").unwrap();
    writeln!(s, "        }}").unwrap();
    writeln!(s, "    }}").unwrap();
    writeln!(s, "    violations").unwrap();
    writeln!(s, "}}").unwrap();
    Ok((s, None))
}

fn gen_class(
    fn_name: &str,
    class_name: &str,
    spec: &CimSpecification,
    accessor: &str,
    attr: &CimAttribute,
    c: &ConstraintInfo,
) -> Result<(String, Option<String>), String> {
    if attr.is_primitive || attr.is_cim_datatype || attr.is_enum_value {
        return Err("sh:Class on non-association field".to_string());
    }
    let wanted = match c.payload.get("class").and_then(|v| v.as_str()) {
        Some(v) => local_name(v),
        None => return Err("sh:Class: missing class payload".to_string()),
    };
    // Expand to all types that ARE or inherit from `wanted`. If everything
    // qualifies, the constraint is vacuously true and can be skipped.
    let allowed_types = class_and_subclasses(spec, &wanted);
    if allowed_types.is_empty() || allowed_types.len() >= spec.types.len() {
        return Err(format!("sh:Class vacuously true: all subtypes of {} qualify", wanted));
    }
    gen_ref_type_check(fn_name, class_name, accessor, attr, c, &allowed_types)
}

fn gen_or_class(
    fn_name: &str,
    class_name: &str,
    spec: &CimSpecification,
    accessor: &str,
    attr: &CimAttribute,
    c: &ConstraintInfo,
) -> Result<(String, Option<String>), String> {
    if attr.is_primitive || attr.is_cim_datatype || attr.is_enum_value {
        return Err("sh:OrClass on non-association field".to_string());
    }
    let raw_classes: Vec<String> = match c.payload.get("classes").and_then(|v| v.as_list()) {
        Some(v) if !v.is_empty() => v.iter().map(|s| local_name(s)).collect(),
        _ => return Err("sh:OrClass: empty class list".to_string()),
    };
    // Expand each listed class to include its subclasses.
    let mut allowed_set = std::collections::BTreeSet::new();
    for cls in &raw_classes {
        for t in class_and_subclasses(spec, cls) {
            allowed_set.insert(t);
        }
    }
    let allowed_types: Vec<String> = allowed_set.into_iter().collect();
    if allowed_types.is_empty() || allowed_types.len() >= spec.types.len() {
        return Err("sh:OrClass vacuously true: all subtypes qualify".to_string());
    }
    gen_ref_type_check(fn_name, class_name, accessor, attr, c, &allowed_types)
}

fn gen_ref_type_check(
    fn_name: &str,
    class_name: &str,
    accessor: &str,
    attr: &CimAttribute,
    c: &ConstraintInfo,
    allowed_types: &[String],
) -> Result<(String, Option<String>), String> {
    fn esc(x: &str) -> String { x.replace('\\', "\\\\").replace('"', "\\\"") }
    let message  = esc(&c.message);
    let severity = &c.severity;
    let name_str    = esc(&c.name);
    let rule_id_str = esc(&c.rule_id);
    let desc_str = esc(&c.description);
    let prop = &attr.id;
    let allowed_str = allowed_types.iter()
        .map(|t| format!("\"{}\"", t))
        .collect::<Vec<_>>()
        .join(", ");

    let mut s = String::new();
    writeln!(s, "pub fn {fn_name}(dataset: &CimDataset) -> Vec<Violation> {{").unwrap();
    writeln!(s, "    let mut violations = Vec::new();").unwrap();
    writeln!(s, "    for mrid in dataset.by_type.get(\"{class_name}\").into_iter().flatten() {{").unwrap();
    writeln!(s, "        let entry = &dataset.entries[mrid];").unwrap();
    writeln!(s, "        let obj = match entry.element.as_any()").unwrap();
    writeln!(s, "            .downcast_ref::<cimstructs::{class_name}>() {{").unwrap();
    writeln!(s, "            Some(o) => o, None => continue,").unwrap();
    writeln!(s, "        }};").unwrap();

    if attr.is_list {
        writeln!(s, "        for r in &{accessor} {{").unwrap();
        writeln!(s, "            let ref_id = r.mrid.trim_start_matches('#');").unwrap();
        writeln!(s, "            if ref_id.is_empty() {{ continue; }}").unwrap();
        writeln!(s, "            if let Some(ref_entry) = dataset.entries.get(ref_id) {{").unwrap();
        writeln!(s, "                let allowed: &[&str] = &[{allowed_str}];").unwrap();
        writeln!(s, "                if !allowed.contains(&ref_entry.element.type_name()) {{").unwrap();
        writeln!(s, "                    violations.push(Violation {{").unwrap();
        writeln!(s, "                        object_id:   mrid.clone(),").unwrap();
        writeln!(s, "                        rule_id:     \"{rule_id_str}\".to_string(),").unwrap();
        writeln!(s, "                        class:       \"{class_name}\".to_string(),").unwrap();
        writeln!(s, "                        property:    \"{prop}\".to_string(),").unwrap();
        writeln!(s, "                        message:     \"{message}\".to_string(),").unwrap();
        writeln!(s, "                        severity:    \"{severity}\".to_string(),").unwrap();
        writeln!(s, "                        name:        \"{name_str}\".to_string(),").unwrap();
        writeln!(s, "                        description: \"{desc_str}\".to_string(),").unwrap();
        writeln!(s, "                    }});").unwrap();
        writeln!(s, "                }}").unwrap();
        writeln!(s, "            }}").unwrap();
        writeln!(s, "        }}").unwrap();
    } else {
        writeln!(s, "        let ref_mrid = match {accessor}.as_ref() {{").unwrap();
        writeln!(s, "            Some(r) => r.mrid.trim_start_matches('#'), None => continue,").unwrap();
        writeln!(s, "        }};").unwrap();
        writeln!(s, "        if let Some(ref_entry) = dataset.entries.get(ref_mrid) {{").unwrap();
        writeln!(s, "            let allowed: &[&str] = &[{allowed_str}];").unwrap();
        writeln!(s, "            if !allowed.contains(&ref_entry.element.type_name()) {{").unwrap();
        writeln!(s, "                violations.push(Violation {{").unwrap();
        writeln!(s, "                    object_id:   mrid.clone(),").unwrap();
        writeln!(s, "                    rule_id:     \"{rule_id_str}\".to_string(),").unwrap();
        writeln!(s, "                    class:       \"{class_name}\".to_string(),").unwrap();
        writeln!(s, "                    property:    \"{prop}\".to_string(),").unwrap();
        writeln!(s, "                    message:     \"{message}\".to_string(),").unwrap();
        writeln!(s, "                    severity:    \"{severity}\".to_string(),").unwrap();
        writeln!(s, "                    name:        \"{name_str}\".to_string(),").unwrap();
        writeln!(s, "                    description: \"{desc_str}\".to_string(),").unwrap();
        writeln!(s, "                }});").unwrap();
        writeln!(s, "            }}").unwrap();
        writeln!(s, "        }}").unwrap();
    }

    writeln!(s, "    }}").unwrap();
    writeln!(s, "    violations").unwrap();
    writeln!(s, "}}").unwrap();
    Ok((s, None))
}

fn gen_slice_mrid_rdf_type_check(
    fn_name: &str,
    class_name: &str,
    spec: &CimSpecification,
    attr_id: &str,
    c: &ConstraintInfo,
) -> Result<(String, Option<String>), String> {
    let (accessor_prefix, attr) = match find_attr_in_hierarchy(spec, class_name, attr_id) {
        Some(p) => p,
        None => return Err(format!("slice-mrid: attribute {} not found in hierarchy", attr_id)),
    };
    if attr.is_primitive || attr.is_cim_datatype || attr.is_enum_value || !attr.is_association_used {
        return Err("slice-mrid: field is not a usable association".to_string());
    }
    let field_name = sanitize_field(to_snake_case(&attr.label));
    let accessor = format!("{accessor_prefix}.{field_name}");

    let comp = c.component.as_str();
    let allowed_types: Vec<String> = match comp {
        "sh:InConstraintComponent" => {
            match c.payload.get("in").and_then(|v| v.as_list()) {
                Some(v) if !v.is_empty() => v.iter().map(|s| local_name(s)).collect(),
                _ => return Err("slice-mrid sh:In: empty payload".to_string()),
            }
        }
        "sh:HasValueConstraintComponent" => {
            match c.payload.get("hasValue").and_then(|v| v.as_str()) {
                Some(v) => vec![local_name(v)],
                None => return Err("slice-mrid sh:HasValue: missing payload".to_string()),
            }
        }
        _ => return Err(format!("slice-mrid: component {} not supported", comp)),
    };
    if allowed_types.is_empty() {
        return Err("slice-mrid: empty allowed types list".to_string());
    }
    gen_ref_type_check(fn_name, class_name, &accessor, &attr, c, &allowed_types)
}

fn gen_slice_string_in(
    fn_name: &str,
    class_name: &str,
    accessor: &str,
    attr: &CimAttribute,
    c: &ConstraintInfo,
) -> Result<(String, Option<String>), String> {
    let values = match c.payload.get("in").and_then(|v| v.as_list()) {
        Some(v) if !v.is_empty() => v,
        _ => return Err("sh:SliceStringIn: empty list".to_string()),
    };
    let allowed_items: Vec<String> = values
        .iter()
        .map(|v| format!("\"{}\"", v.replace('"', "\\\"")))
        .collect();
    let allowed_str = allowed_items.join(", ");

    fn esc(x: &str) -> String { x.replace('\\', "\\\\").replace('"', "\\\"") }
    let message  = esc(&c.message);
    let severity = &c.severity;
    let name_str    = esc(&c.name);
    let rule_id_str = esc(&c.rule_id);
    let desc_str = esc(&c.description);
    let prop = &attr.id;

    let mut s = String::new();
    writeln!(s, "pub fn {fn_name}(dataset: &CimDataset) -> Vec<Violation> {{").unwrap();
    writeln!(s, "    let mut violations = Vec::new();").unwrap();
    writeln!(s, "    for mrid in dataset.by_type.get(\"{class_name}\").into_iter().flatten() {{").unwrap();
    writeln!(s, "        let entry = &dataset.entries[mrid];").unwrap();
    writeln!(s, "        let obj = match entry.element.as_any()").unwrap();
    writeln!(s, "            .downcast_ref::<cimstructs::{class_name}>() {{").unwrap();
    writeln!(s, "            Some(o) => o, None => continue,").unwrap();
    writeln!(s, "        }};").unwrap();
    writeln!(s, "        let allowed: &[&str] = &[{allowed_str}];").unwrap();
    writeln!(s, "        for val in &{accessor} {{").unwrap();
    writeln!(s, "            if val.is_empty() {{ continue; }}").unwrap();
    writeln!(s, "            if !allowed.contains(&val.as_str()) {{").unwrap();
    writeln!(s, "                violations.push(Violation {{").unwrap();
    writeln!(s, "                    object_id:   mrid.clone(),").unwrap();
    writeln!(s, "                    rule_id:     \"{rule_id_str}\".to_string(),").unwrap();
    writeln!(s, "                    class:       \"{class_name}\".to_string(),").unwrap();
    writeln!(s, "                    property:    \"{prop}\".to_string(),").unwrap();
    writeln!(s, "                    message:     \"{message}\".to_string(),").unwrap();
    writeln!(s, "                    severity:    \"{severity}\".to_string(),").unwrap();
    writeln!(s, "                    name:        \"{name_str}\".to_string(),").unwrap();
    writeln!(s, "                    description: \"{desc_str}\".to_string(),").unwrap();
    writeln!(s, "                }});").unwrap();
    writeln!(s, "            }}").unwrap();
    writeln!(s, "        }}").unwrap();
    writeln!(s, "    }}").unwrap();
    writeln!(s, "    violations").unwrap();
    writeln!(s, "}}").unwrap();
    Ok((s, None))
}

// ---------------------------------------------------------------------------
// Inverse path generator
// ---------------------------------------------------------------------------

/// sh:targetNode <cim:Class> with sh:path [ sh:inversePath rdf:type ] and
/// sh:minCount/sh:maxCount: a single dataset-wide check on how many instances
/// of `target_class` exist (e.g. "at least one TopologicalIsland", "at most
/// one GeographicalRegion"), not a per-instance check. Emits at most one
/// violation, matching cimgo's generated form (empty object_id).
fn gen_instance_count(
    fn_name: &str,
    target_class: &str,
    c: &ConstraintInfo,
) -> Result<(String, Option<String>), String> {
    let (op, threshold) = match c.component.as_str() {
        "sh:MinCountConstraintComponent" | "sh:RequiredConstraintComponent" =>
            ("<", c.payload.get("minCount").and_then(|v| v.as_int()).unwrap_or(1)),
        "sh:MaxCountConstraintComponent" =>
            (">", c.payload.get("maxCount").and_then(|v| v.as_int()).unwrap_or(1)),
        _ => return Err(format!("inverse rdf:type count: component {} not supported", c.component)),
    };

    fn esc(x: &str) -> String { x.replace('\\', "\\\\").replace('"', "\\\"") }
    let message  = esc(&c.message);
    let severity = &c.severity;
    let name_str    = esc(&c.name);
    let rule_id_str = esc(&c.rule_id);
    let desc_str = esc(&c.description);
    let prop = &c.name;

    let mut s = String::new();
    writeln!(s, "pub fn {fn_name}(dataset: &CimDataset) -> Vec<Violation> {{").unwrap();
    writeln!(s, "    let count = dataset.by_type.get(\"{target_class}\").map_or(0, |v| v.len());").unwrap();
    writeln!(s, "    let mut violations = Vec::new();").unwrap();
    writeln!(s, "    if count {op} {threshold} {{").unwrap();
    writeln!(s, "        violations.push(Violation {{").unwrap();
    writeln!(s, "            object_id:   String::new(),").unwrap();
    writeln!(s, "            rule_id:     \"{rule_id_str}\".to_string(),").unwrap();
    writeln!(s, "            class:       \"{target_class}\".to_string(),").unwrap();
    writeln!(s, "            property:    \"{prop}\".to_string(),").unwrap();
    writeln!(s, "            message:     \"{message}\".to_string(),").unwrap();
    writeln!(s, "            severity:    \"{severity}\".to_string(),").unwrap();
    writeln!(s, "            name:        \"{name_str}\".to_string(),").unwrap();
    writeln!(s, "            description: \"{desc_str}\".to_string(),").unwrap();
    writeln!(s, "        }});").unwrap();
    writeln!(s, "    }}").unwrap();
    writeln!(s, "    violations").unwrap();
    writeln!(s, "}}").unwrap();
    Ok((s, None))
}

fn gen_inverse_count(
    fn_name: &str,
    target_class: &str,
    forward_pred: &str,
    spec: &CimSpecification,
    c: &ConstraintInfo,
) -> Result<(String, Option<String>), String> {
    // sh:targetNode <cim:Class> with sh:path [ sh:inversePath rdf:type ]: the
    // inverse of rdf:type from the class node reaches every instance of that
    // class, so min/maxCount here means "N instances of Class must exist" —
    // a single dataset-wide check, not one CIM association per instance.
    if local_name(forward_pred) == "type" {
        return gen_instance_count(fn_name, target_class, c);
    }
    let local = local_name(forward_pred);
    let (src_class, src_prop) = match local.split_once('.') {
        Some(p) => p,
        None => return Err("inverse path has no class.prop shape".to_string()),
    };

    let (_, attr) = match find_attr_in_hierarchy(spec, src_class, &local) {
        Some(p) => p,
        None => return Err(format!("inverse attribute {} not found in hierarchy", local)),
    };
    if !attr.is_association_used {
        return Err("inverse unused association".to_string());
    }
    // Inverse path requires an association field (Option<MridRef> or Vec<MridRef>)
    if attr.is_primitive || attr.is_cim_datatype || attr.is_enum_value {
        return Err("inverse path on non-association field".to_string());
    }

    // The referrer class can be abstract (e.g. RegulatingCondEq) — dataset.by_type
    // only holds concrete type names, so the index must scan every subclass that
    // can carry the forward field, or the count is always 0 and every target is
    // falsely flagged.
    let field_name = sanitize_field(to_snake_case(src_prop));
    let mut scan_classes: Vec<(String, String)> = Vec::new();
    for cls in class_and_subclasses(spec, src_class) {
        if let Some((prefix, _)) = find_attr_in_hierarchy(spec, &cls, &local) {
            let src_prefix = prefix.replacen("obj", "src", 1);
            scan_classes.push((cls, format!("{src_prefix}.{field_name}")));
        }
    }
    if scan_classes.is_empty() {
        return Err(format!("inverse attribute {} has no concrete referrer class", local));
    }

    let index_snippets: Vec<(String, String)> = scan_classes
        .into_iter()
        .map(|(cls, src_field)| {
            let snippet = if attr.is_list {
                format!("for r in &{src_field} {{ *ref_counts.entry(r.mrid.as_str()).or_insert(0) += 1; }}")
            } else {
                format!("if let Some(r) = &{src_field} {{ *ref_counts.entry(r.mrid.as_str()).or_insert(0) += 1; }}")
            };
            (cls, snippet)
        })
        .collect();

    let (op, threshold) = match c.component.as_str() {
        "sh:MinCountConstraintComponent" | "sh:RequiredConstraintComponent" =>
            ("<", c.payload.get("minCount").and_then(|v| v.as_int()).unwrap_or(1)),
        "sh:MaxCountConstraintComponent" =>
            (">", c.payload.get("maxCount").and_then(|v| v.as_int()).unwrap_or(1)),
        // The prelude's downcast_ref::<src_class> already type-asserts every scanned
        // referrer, so an sh:class constraint on the inverse path can never fire.
        "sh:ClassConstraintComponent" =>
            return Err("sh:Class vacuously true: inverse index already type-asserts the referrer class".to_string()),
        _ => return Err(format!("inverse path: component {} not supported", c.component)),
    };

    Ok((build_inverse_fn(fn_name, target_class, &index_snippets, threshold, op, c), None))
}

fn build_inverse_fn(
    fn_name: &str,
    target_class: &str,
    index_snippets: &[(String, String)],
    threshold: i64,
    op: &str,
    c: &ConstraintInfo,
) -> String {
    let mut s = String::new();
    fn esc(x: &str) -> String { x.replace('\\', "\\\\").replace('"', "\\\"") }
    let message  = esc(&c.message);
    let severity = &c.severity;
    let name_str    = esc(&c.name);
    let rule_id_str = esc(&c.rule_id);
    let desc_str = esc(&c.description);
    let prop     = &c.name;

    writeln!(s, "pub fn {fn_name}(dataset: &CimDataset) -> Vec<Violation> {{").unwrap();
    writeln!(s, "    let mut ref_counts: std::collections::HashMap<&str, usize> = std::collections::HashMap::new();").unwrap();
    for (src_class, index_snippet) in index_snippets {
        writeln!(s, "    for src_mrid in dataset.by_type.get(\"{src_class}\").into_iter().flatten() {{").unwrap();
        writeln!(s, "        if let Some(src) = dataset.entries.get(src_mrid)").unwrap();
        writeln!(s, "            .and_then(|e| e.element.as_any().downcast_ref::<cimstructs::{src_class}>()) {{").unwrap();
        writeln!(s, "            {index_snippet}").unwrap();
        writeln!(s, "        }}").unwrap();
        writeln!(s, "    }}").unwrap();
    }
    writeln!(s, "    let mut violations = Vec::new();").unwrap();
    writeln!(s, "    for mrid in dataset.by_type.get(\"{target_class}\").into_iter().flatten() {{").unwrap();
    writeln!(s, "        let count = ref_counts.get(mrid.as_str()).copied().unwrap_or(0);").unwrap();
    writeln!(s, "        if count {op} {threshold} {{").unwrap();
    writeln!(s, "            violations.push(Violation {{").unwrap();
    writeln!(s, "                object_id:   mrid.clone(),").unwrap();
    writeln!(s, "                rule_id:     \"{rule_id_str}\".to_string(),").unwrap();
    writeln!(s, "                class:       \"{target_class}\".to_string(),").unwrap();
    writeln!(s, "                property:    \"{prop}\".to_string(),").unwrap();
    writeln!(s, "                message:     \"{message}\".to_string(),").unwrap();
    writeln!(s, "                severity:    \"{severity}\".to_string(),").unwrap();
    writeln!(s, "                name:        \"{name_str}\".to_string(),").unwrap();
    writeln!(s, "                description: \"{desc_str}\".to_string(),").unwrap();
    writeln!(s, "            }});").unwrap();
    writeln!(s, "        }}").unwrap();
    writeln!(s, "    }}").unwrap();
    writeln!(s, "    violations").unwrap();
    writeln!(s, "}}").unwrap();
    s
}

// ---------------------------------------------------------------------------
// Forward chain generators (multi-segment paths)
// ---------------------------------------------------------------------------

/// Splits a path segment like "cim:StreetAddress.status" into ("StreetAddress",
/// full local name "StreetAddress.status").
fn seg_class_and_local(seg: &str) -> Result<(String, String), String> {
    let local = local_name(seg);
    match local.split_once('.') {
        Some((class, _)) => Ok((class.to_string(), local.clone())),
        None => Err(format!("chain segment {} has no Class.prop shape", seg)),
    }
}

/// Resolves a chain hop attribute and enforces that it decodes to a single
/// 0..1 `MridRef` field. Returns (accessor relative to `var`, attr).
fn chain_hop_accessor(
    spec: &CimSpecification,
    owner_class: &str,
    attr_local: &str,
    var: &str,
) -> Result<(String, CimAttribute), String> {
    let (prefix, attr) = find_attr_in_hierarchy(spec, owner_class, attr_local)
        .ok_or_else(|| format!("chain attribute {} not found in hierarchy", attr_local))?;
    if attr.is_primitive || attr.is_cim_datatype || attr.is_enum_value {
        return Err(format!("chain hop {} is not a reference field", attr_local));
    }
    if attr.is_list {
        return Err(format!("chain hop {} is a list field (not supported)", attr_local));
    }
    let field = sanitize_field(to_snake_case(&attr.label));
    Ok((format!("{}.{field}", prefix.replacen("obj", var, 1)), attr))
}

/// Emits the reference walk for the association segments `segs` into `s` (inside
/// the per-object loop opened by `write_fn_header`, i.e. with `obj` in scope).
/// Any missing or unresolvable link `continue`s — the path yields no value there,
/// matching cimgo's chain-walker. Returns the variable naming the final entry.
fn emit_forward_chain_walk(
    s: &mut String,
    class_name: &str,
    spec: &CimSpecification,
    segs: &[String],
) -> Result<String, String> {
    let (acc0, _) = chain_hop_accessor(spec, class_name, &local_name(&segs[0]), "obj")?;
    writeln!(s, "        let ref0 = match {acc0}.as_ref() {{").unwrap();
    writeln!(s, "            Some(r) => r.mrid.trim_start_matches('#'), None => continue,").unwrap();
    writeln!(s, "        }};").unwrap();
    writeln!(s, "        let entry0 = match dataset.entries.get(ref0) {{").unwrap();
    writeln!(s, "            Some(e) => e, None => continue,").unwrap();
    writeln!(s, "        }};").unwrap();

    for (i, seg) in segs.iter().enumerate().skip(1) {
        let (hop_class, attr_local) = seg_class_and_local(seg)?;
        if !spec.types.contains_key(&hop_class) {
            return Err(format!("chain hop class {} not in schema", hop_class));
        }
        let hop_var = format!("hop{i}");
        let (acc, _) = chain_hop_accessor(spec, &hop_class, &attr_local, &hop_var)?;
        let prev = i - 1;
        writeln!(s, "        let {hop_var} = match entry{prev}.element.as_any()").unwrap();
        writeln!(s, "            .downcast_ref::<cimstructs::{hop_class}>() {{").unwrap();
        writeln!(s, "            Some(o) => o, None => continue,").unwrap();
        writeln!(s, "        }};").unwrap();
        writeln!(s, "        let ref{i} = match {acc}.as_ref() {{").unwrap();
        writeln!(s, "            Some(r) => r.mrid.trim_start_matches('#'), None => continue,").unwrap();
        writeln!(s, "        }};").unwrap();
        writeln!(s, "        let entry{i} = match dataset.entries.get(ref{i}) {{").unwrap();
        writeln!(s, "            Some(e) => e, None => continue,").unwrap();
        writeln!(s, "        }};").unwrap();
    }
    Ok(format!("entry{}", segs.len() - 1))
}

/// sh:Required over a multi-segment forward chain: the presence requirement
/// collapses onto the first link (cimgo parity — a later link that doesn't
/// resolve is indistinguishable from data legitimately split across files).
fn gen_chain_required_first_link(
    fn_name: &str,
    class_name: &str,
    spec: &CimSpecification,
    attr_local: &str,
    c: &ConstraintInfo,
) -> Result<(String, Option<String>), String> {
    let (prefix, attr) = find_attr_in_hierarchy(spec, class_name, attr_local)
        .ok_or_else(|| format!("chain attribute {} not found in hierarchy", attr_local))?;
    if attr.is_primitive || attr.is_cim_datatype || attr.is_enum_value {
        return Err(format!("chain Required first link {} is not a reference field", attr_local));
    }
    let field = sanitize_field(to_snake_case(&attr.label));
    let accessor = format!("{prefix}.{field}");
    gen_required(fn_name, class_name, &accessor, &attr, c)
}

/// Walks the association segments and requires the final resolved entry's class
/// to be in `allowed` (already subclass-expanded). Backs sh:HasValue with an
/// rdf:type tail and multi-hop sh:or-of-sh:class.
fn gen_forward_chain_type_check(
    fn_name: &str,
    class_name: &str,
    spec: &CimSpecification,
    segs: &[String],
    allowed: &[String],
    c: &ConstraintInfo,
) -> Result<(String, Option<String>), String> {
    let mut walk = String::new();
    let final_entry = emit_forward_chain_walk(&mut walk, class_name, spec, segs)?;
    let (message, severity, name_str, rule_id_str, desc_str, _) = extract_violation_fields(c);
    let prop = local_name(&segs[0]);
    let allowed_str = allowed.iter().map(|t| format!("\"{t}\"")).collect::<Vec<_>>().join(", ");

    let mut s = String::new();
    write_fn_header(&mut s, fn_name, class_name);
    s.push_str(&walk);
    writeln!(s, "        let allowed: &[&str] = &[{allowed_str}];").unwrap();
    writeln!(s, "        if !allowed.contains(&{final_entry}.element.type_name()) {{").unwrap();
    write_violation(&mut s, class_name, &prop, &message, &severity, &name_str, &rule_id_str, &desc_str, "            ");
    writeln!(s, "        }}").unwrap();
    write_fn_footer(&mut s);
    Ok((s, None))
}

/// Walks the association segments up to the last one, then applies an
/// sh:datatype format check to the primitive leaf field named by the final segment.
fn gen_forward_chain_datatype_check(
    fn_name: &str,
    class_name: &str,
    spec: &CimSpecification,
    segs: &[String],
    c: &ConstraintInfo,
) -> Result<(String, Option<String>), String> {
    let (mid, leaf_seg) = segs.split_at(segs.len() - 1);
    if mid.is_empty() {
        return Err("chain datatype: no hop segments".to_string());
    }
    let (leaf_class, leaf_local) = seg_class_and_local(&leaf_seg[0])?;
    if !spec.types.contains_key(&leaf_class) {
        return Err(format!("chain leaf class {} not in schema", leaf_class));
    }
    let (leaf_prefix, leaf_attr) = find_attr_in_hierarchy(spec, &leaf_class, &leaf_local)
        .ok_or_else(|| format!("chain attribute {} not found in hierarchy", leaf_local))?;
    if leaf_attr.lang_type != "String" || leaf_attr.is_list {
        return Err(format!("chain sh:Datatype on non-string leaf {}", leaf_local));
    }
    let dt = c.payload.get("datatype").and_then(|v| v.as_str()).unwrap_or("");
    let check_fn = match dt {
        "xsd:dateTime" | "<http://www.w3.org/2001/XMLSchema#dateTime>" => "is_xsd_datetime",
        "xsd:date" | "<http://www.w3.org/2001/XMLSchema#date>" => "is_xsd_date",
        "xsd:gMonthDay" | "<http://www.w3.org/2001/XMLSchema#gMonthDay>" => "is_xsd_gmonthday",
        "xsd:anyURI" | "<http://www.w3.org/2001/XMLSchema#anyURI>" => "is_xsd_anyuri",
        _ => return Err(format!("chain sh:Datatype: unsupported datatype {:?}", dt)),
    };

    let mut walk = String::new();
    let final_entry = emit_forward_chain_walk(&mut walk, class_name, spec, mid)?;
    let (message, severity, name_str, rule_id_str, desc_str, _) = extract_violation_fields(c);
    let prop = local_name(&segs[0]);
    let leaf_field = sanitize_field(to_snake_case(&leaf_attr.label));
    let leaf_acc = format!("{}.{leaf_field}", leaf_prefix.replacen("obj", "leaf", 1));

    let mut s = String::new();
    write_fn_header(&mut s, fn_name, class_name);
    s.push_str(&walk);
    writeln!(s, "        let leaf = match {final_entry}.element.as_any()").unwrap();
    writeln!(s, "            .downcast_ref::<cimstructs::{leaf_class}>() {{").unwrap();
    writeln!(s, "            Some(o) => o, None => continue,").unwrap();
    writeln!(s, "        }};").unwrap();
    writeln!(s, "        if {leaf_acc}.is_empty() {{ continue; }}").unwrap();
    writeln!(s, "        if !{check_fn}(&{leaf_acc}) {{").unwrap();
    write_violation(&mut s, class_name, &prop, &message, &severity, &name_str, &rule_id_str, &desc_str, "            ");
    writeln!(s, "        }}").unwrap();
    write_fn_footer(&mut s);
    Ok((s, None))
}

/// [^forward-ref, enum-attr] sh:HasValue: the target object must be referenced by
/// at least one source object whose enum attribute carries the expected value
/// (e.g. every Ground needs a Terminal with Terminal.phases = PhaseCode.N).
fn gen_inverse_chain_has_value(
    fn_name: &str,
    target_class: &str,
    spec: &CimSpecification,
    inv_seg: &str,
    value_seg: &str,
    c: &ConstraintInfo,
) -> Result<(String, Option<String>), String> {
    let forward = &inv_seg[1..];
    let (src_class, ref_local) = seg_class_and_local(forward)?;
    if !spec.types.contains_key(&src_class) {
        return Err(format!("inverse chain source class {} not in schema", src_class));
    }
    let (ref_prefix, ref_attr) = find_attr_in_hierarchy(spec, &src_class, &ref_local)
        .ok_or_else(|| format!("chain attribute {} not found in hierarchy", ref_local))?;
    if ref_attr.is_primitive || ref_attr.is_cim_datatype || ref_attr.is_enum_value {
        return Err(format!("inverse chain link {} is not a reference field", ref_local));
    }
    let value_local = local_name(value_seg);
    let (val_prefix, val_attr) = find_attr_in_hierarchy(spec, &src_class, &value_local)
        .ok_or_else(|| format!("chain attribute {} not found in hierarchy", value_local))?;
    if !val_attr.is_enum_value {
        return Err(format!("inverse chain sh:HasValue on non-enum attribute {}", value_local));
    }
    let expected = enum_literal(
        c.payload.get("hasValue").and_then(|v| v.as_str())
            .ok_or_else(|| "inverse chain sh:HasValue: missing payload".to_string())?,
    )
    .replace('"', "\\\"");

    let ref_field = sanitize_field(to_snake_case(&ref_attr.label));
    let ref_acc = format!("{}.{ref_field}", ref_prefix.replacen("obj", "src", 1));
    let val_field = sanitize_field(to_snake_case(&val_attr.label));
    let val_acc = format!("{}.{val_field}", val_prefix.replacen("obj", "src", 1));

    let (message, severity, name_str, rule_id_str, desc_str, _) = extract_violation_fields(c);
    let prop = ref_local;

    let mut s = String::new();
    writeln!(s, "pub fn {fn_name}(dataset: &CimDataset) -> Vec<Violation> {{").unwrap();
    writeln!(s, "    let mut has_value: std::collections::HashSet<&str> = std::collections::HashSet::new();").unwrap();
    writeln!(s, "    for src_mrid in dataset.by_type.get(\"{src_class}\").into_iter().flatten() {{").unwrap();
    writeln!(s, "        let src = match dataset.entries.get(src_mrid)").unwrap();
    writeln!(s, "            .and_then(|e| e.element.as_any().downcast_ref::<cimstructs::{src_class}>()) {{").unwrap();
    writeln!(s, "            Some(o) => o, None => continue,").unwrap();
    writeln!(s, "        }};").unwrap();
    writeln!(s, "        if {val_acc}.as_ref().map_or(true, |u| u.uri != \"{expected}\") {{ continue; }}").unwrap();
    if ref_attr.is_list {
        writeln!(s, "        for r in &{ref_acc} {{ has_value.insert(r.mrid.trim_start_matches('#')); }}").unwrap();
    } else {
        writeln!(s, "        if let Some(r) = {ref_acc}.as_ref() {{ has_value.insert(r.mrid.trim_start_matches('#')); }}").unwrap();
    }
    writeln!(s, "    }}").unwrap();
    writeln!(s, "    let mut violations = Vec::new();").unwrap();
    writeln!(s, "    for mrid in dataset.by_type.get(\"{target_class}\").into_iter().flatten() {{").unwrap();
    writeln!(s, "        if !has_value.contains(mrid.as_str()) {{").unwrap();
    write_violation(&mut s, target_class, &prop, &message, &severity, &name_str, &rule_id_str, &desc_str, "            ");
    writeln!(s, "        }}").unwrap();
    writeln!(s, "    }}").unwrap();
    writeln!(s, "    violations").unwrap();
    writeln!(s, "}}").unwrap();
    Ok((s, None))
}

// ---------------------------------------------------------------------------
// Code-building helpers
// ---------------------------------------------------------------------------

fn field_expr_and_guard(accessor: &str, attr: &CimAttribute) -> (String, Option<String>) {
    if attr.is_enum_value {
        (
            format!("{accessor}.as_ref().map_or(\"\", |u| u.uri.as_str())"),
            Some(format!("if {accessor}.is_none() {{ continue; }}")),
        )
    } else if attr.is_primitive || attr.is_cim_datatype {
        (
            format!("{accessor}.as_str()"),
            Some(format!("if {accessor}.is_empty() {{ continue; }}")),
        )
    } else {
        (
            format!("{accessor}.as_ref().map_or(\"\", |r| r.mrid.as_str())"),
            Some(format!("if {accessor}.is_none() {{ continue; }}")),
        )
    }
}

fn build_fn(
    fn_name: &str,
    class_name: &str,
    guard: Option<&str>,
    prelude: Option<&str>,     // constant decls outside the loop
    condition: &str,
    c: &ConstraintInfo,
    attr: &CimAttribute,
) -> String {
    let mut s = String::new();
    let prop = &attr.id;
    fn esc(s: &str) -> String { s.replace('\\', "\\\\").replace('"', "\\\"") }
    let message = esc(&c.message);
    let severity = &c.severity;
    let name_str    = esc(&c.name);
    let rule_id_str = esc(&c.rule_id);
    let desc_str = esc(&c.description);

    writeln!(s, "pub fn {fn_name}(dataset: &CimDataset) -> Vec<Violation> {{").unwrap();
    writeln!(s, "    let mut violations = Vec::new();").unwrap();
    // Prelude goes BEFORE the loop (constants, regex etc.)
    if let Some(pre) = prelude {
        s.push_str(pre);
    }
    writeln!(s, "    for mrid in dataset.by_type.get(\"{class_name}\").into_iter().flatten() {{").unwrap();
    writeln!(s, "        let entry = &dataset.entries[mrid];").unwrap();
    writeln!(s, "        let obj = match entry.element.as_any()").unwrap();
    writeln!(s, "            .downcast_ref::<cimstructs::{class_name}>() {{").unwrap();
    writeln!(s, "            Some(o) => o, None => continue,").unwrap();
    writeln!(s, "        }};").unwrap();
    if let Some(g) = guard {
        writeln!(s, "        {g}").unwrap();
    }
    writeln!(s, "        if {condition} {{").unwrap();
    writeln!(s, "            violations.push(Violation {{").unwrap();
    writeln!(s, "                object_id:   mrid.clone(),").unwrap();
    writeln!(s, "                rule_id:     \"{rule_id_str}\".to_string(),").unwrap();
    writeln!(s, "                class:       \"{class_name}\".to_string(),").unwrap();
    writeln!(s, "                property:    \"{prop}\".to_string(),").unwrap();
    writeln!(s, "                message:     \"{message}\".to_string(),").unwrap();
    writeln!(s, "                severity:    \"{severity}\".to_string(),").unwrap();
    writeln!(s, "                name:        \"{name_str}\".to_string(),").unwrap();
    writeln!(s, "                description: \"{desc_str}\".to_string(),").unwrap();
    writeln!(s, "            }});").unwrap();
    writeln!(s, "        }}").unwrap();
    writeln!(s, "    }}").unwrap();
    writeln!(s, "    violations").unwrap();
    writeln!(s, "}}").unwrap();
    s
}

// ---------------------------------------------------------------------------
// Field lookup
// ---------------------------------------------------------------------------

fn find_attr_in_hierarchy(
    spec: &CimSpecification,
    class_name: &str,
    attr_id: &str,
) -> Option<(String, CimAttribute)> {
    let mut depth = 0usize;
    let mut current_id = class_name.to_string();
    loop {
        let t = spec.types.get(&current_id)?;
        if let Some(attr) = t.attributes.iter().find(|a| a.id == attr_id) {
            // depth 0 → "obj", depth 1 → "obj.base", depth 2 → "obj.base.base"
            let prefix = format!("obj{}", ".base".repeat(depth));
            return Some((prefix, attr.clone()));
        }
        if t.super_type.is_empty() || t.super_type == current_id {
            break;
        }
        depth += 1;
        current_id = t.super_type.clone();
    }
    None
}

// ---------------------------------------------------------------------------
// Name helpers
// ---------------------------------------------------------------------------

fn local_name(simplified_iri: &str) -> String {
    if let Some(colon) = simplified_iri.find(':') {
        simplified_iri[colon + 1..].to_string()
    } else {
        simplified_iri.to_string()
    }
}

/// Convert a TTL file name to a valid Rust module name.
fn file_to_mod_name(file_name: &str) -> String {
    let raw: String = file_name
        .chars()
        .map(|c| if c.is_alphanumeric() { c.to_ascii_lowercase() } else { '_' })
        .collect();
    // Module names can't start with a digit
    let raw = raw.trim_matches('_');
    if raw.starts_with(|c: char| c.is_ascii_digit()) {
        format!("p{raw}")
    } else {
        raw.to_string()
    }
}

fn component_suffix(component: &str) -> &str {
    match component {
        "sh:RequiredConstraintComponent" => "required",
        "sh:MinCountConstraintComponent" => "min_count",
        "sh:MaxCountConstraintComponent" => "max_count",
        "sh:ExactCountConstraintComponent" => "exact_count",
        "sh:InConstraintComponent" => "in",
        "sh:HasValueConstraintComponent" => "has_value",
        "sh:DatatypeConstraintComponent" => "datatype",
        "sh:ClassConstraintComponent" => "class_check",
        "sh:OrClassConstraintComponent" => "or_class",
        "sh:NotClassConstraintComponent" => "not_class",
        "sh:LessThanConstraintComponent" => "less_than",
        "sh:LessThanOrEqualsConstraintComponent" => "less_than_or_equals",
        "sh:PatternConstraintComponent" => "pattern",
        "sh:MinLengthConstraintComponent" => "min_length",
        "sh:MaxLengthConstraintComponent" => "max_length",
        "sh:MinExclusiveConstraintComponent" => "min_excl",
        "sh:MaxExclusiveConstraintComponent" => "max_excl",
        "sh:MinInclusiveConstraintComponent" => "min_incl",
        "sh:MaxInclusiveConstraintComponent" => "max_incl",
        _ => "constraint",
    }
}

fn safe_fn_name(raw: &str) -> String {
    // Collapse runs of underscores, ensure doesn't start with digit
    let collapsed: String = {
        let mut prev = '_';
        let mut s = String::new();
        for c in raw.chars() {
            if c == '_' && prev == '_' { continue; }
            s.push(c);
            prev = c;
        }
        s.trim_matches('_').to_string()
    };
    if collapsed.starts_with(|c: char| c.is_ascii_digit()) {
        format!("check_{collapsed}")
    } else if collapsed.len() > 100 {
        format!("{}_etc", collapsed[..97].trim_end_matches('_'))
    } else {
        collapsed
    }
}

fn extract_fn_name(code: &str) -> String {
    code.lines()
        .find(|l| l.starts_with("pub fn "))
        .and_then(|l| l.strip_prefix("pub fn "))
        .and_then(|l| l.split('(').next())
        .unwrap_or("")
        .to_string()
}

// ---------------------------------------------------------------------------
// Compound constraint generators (sh:xone, sh:or, sh:and)
// ---------------------------------------------------------------------------

fn gen_compound_check(
    file_snake: &str,
    class_name: &str,
    spec: &CimSpecification,
    c: &ConstraintInfo,
) -> Result<String, String> {
    let branches = c.payload.get("branches").and_then(|v| v.as_shapes())
        .ok_or_else(|| "compound check: missing branches payload".to_string())?;

    let comp_suffix = match c.component.as_str() {
        "sh:XoneConstraintComponent" => "xone",
        "sh:OrConstraintComponent" => "or",
        "sh:AndConstraintComponent" => "and",
        _ => return Err(format!("unknown compound component: {}", c.component)),
    };
    let raw_name = format!(
        "check_{file_snake}_{class_snake}_{comp_suffix}",
        class_snake = to_snake_case(class_name),
    );
    let fn_name = safe_fn_name(&raw_name);

    match c.component.as_str() {
        "sh:XoneConstraintComponent" => gen_xone_check(&fn_name, class_name, spec, c, branches)
            .ok_or_else(|| "sh:xone: branch structure not supported".to_string()),
        "sh:OrConstraintComponent"   => gen_or_compound_check(&fn_name, class_name, spec, c, branches)
            .ok_or_else(|| "sh:or: branch structure not supported".to_string()),
        "sh:AndConstraintComponent"  => gen_and_compound_check(&fn_name, class_name, spec, c, branches)
            .ok_or_else(|| "sh:and: branch structure not supported".to_string()),
        _ => Err(format!("unknown compound component: {}", c.component)),
    }
}

/// sh:xone: exactly one of N forward-pointer fields must be non-None.
/// Each branch must have a single MinCount=1 on a forward association path.
fn gen_xone_check(
    fn_name: &str,
    class_name: &str,
    spec: &CimSpecification,
    c: &ConstraintInfo,
    branches: &[Vec<ConstraintInfo>],
) -> Option<String> {
    let mut field_accessors: Vec<String> = Vec::new();
    for branch in branches {
        let min1 = branch.iter().find(|ci|
            ci.component == "sh:MinCountConstraintComponent"
            && ci.payload.get("minCount").and_then(|v| v.as_int()) == Some(1)
            && ci.path.len() == 1 && !ci.path[0].starts_with('^'))?;
        let attr_id = local_name(&min1.path[0]);
        let (acc_prefix, attr) = find_attr_in_hierarchy(spec, class_name, &attr_id)?;
        if !attr.is_association_used || attr.is_list { return None; }
        let field = sanitize_field(to_snake_case(&attr.label));
        field_accessors.push(format!("{acc_prefix}.{field}"));
    }
    if field_accessors.len() < 2 { return None; }

    let (message, severity, name_str, rule_id_str, desc_str, prop) = extract_violation_fields(c);
    let mut s = String::new();
    write_fn_header(&mut s, fn_name, class_name);
    writeln!(s, "        let mut pass_count = 0usize;").unwrap();
    for acc in &field_accessors {
        writeln!(s, "        if {acc}.is_some() {{ pass_count += 1; }}").unwrap();
    }
    writeln!(s, "        if pass_count != 1 {{").unwrap();
    write_violation(&mut s, class_name, &prop, &message, &severity, &name_str, &rule_id_str, &desc_str, "            ");
    writeln!(s, "        }}").unwrap();
    write_fn_footer(&mut s);
    Some(s)
}

/// sh:or: violation when ALL branches fail their inverse-count range.
/// Each branch must have exactly one inverse-path with min/max count.
fn gen_or_compound_check(
    fn_name: &str,
    class_name: &str,
    spec: &CimSpecification,
    c: &ConstraintInfo,
    branches: &[Vec<ConstraintInfo>],
) -> Option<String> {
    struct InvBranch { prelude: String, fail_cond: String }
    let mut parsed: Vec<InvBranch> = Vec::new();

    for (i, branch) in branches.iter().enumerate() {
        // Must have at least one constraint with inverse path
        let inv = branch.iter().find(|ci|
            !ci.path.is_empty() && ci.path[0].starts_with('^'))?;
        if inv.path.len() != 1 { return None; } // multi-segment: skip

        let prelude = build_inverse_count_prelude(spec, &inv.path[0], i)?;
        let min = branch.iter().find(|ci| ci.component == "sh:MinCountConstraintComponent")
            .and_then(|ci| ci.payload.get("minCount").and_then(|v| v.as_int()))
            .unwrap_or(0);
        let max = branch.iter().find(|ci| ci.component == "sh:MaxCountConstraintComponent")
            .and_then(|ci| ci.payload.get("maxCount").and_then(|v| v.as_int()))
            .unwrap_or(i64::MAX / 2);
        let fail_cond = inverse_fail_cond(i, min, max);
        parsed.push(InvBranch { prelude, fail_cond });
    }
    if parsed.len() < 2 { return None; }

    let (message, severity, name_str, rule_id_str, desc_str, prop) = extract_violation_fields(c);
    let all_fail = parsed.iter().map(|b| b.fail_cond.as_str()).collect::<Vec<_>>().join(" && ");

    let mut s = String::new();
    writeln!(s, "pub fn {fn_name}(dataset: &CimDataset) -> Vec<Violation> {{").unwrap();
    for b in &parsed { s.push_str(&b.prelude); }
    writeln!(s, "    let mut violations = Vec::new();").unwrap();
    writeln!(s, "    for mrid in dataset.by_type.get(\"{class_name}\").into_iter().flatten() {{").unwrap();
    writeln!(s, "        if {all_fail} {{").unwrap();
    write_violation(&mut s, class_name, &prop, &message, &severity, &name_str, &rule_id_str, &desc_str, "            ");
    writeln!(s, "        }}").unwrap();
    writeln!(s, "    }}").unwrap();
    writeln!(s, "    violations").unwrap();
    writeln!(s, "}}").unwrap();
    Some(s)
}

/// sh:and: violation when ANY branch fails.
/// Branches can be:
///   - forward MaxCount=0 (field must be absent)
///   - forward MinCount=1+MaxCount=1 (field must be present, scalar only)
///   - inverse single-segment path with min/max count
///   - multi-segment (skipped — returns None for whole check)
fn gen_and_compound_check(
    fn_name: &str,
    class_name: &str,
    spec: &CimSpecification,
    c: &ConstraintInfo,
    branches: &[Vec<ConstraintInfo>],
) -> Option<String> {
    let mut prelude_parts: Vec<String> = Vec::new();
    let mut fail_conds: Vec<String> = Vec::new();
    let mut needs_obj = false;
    let mut inv_idx = 0usize;

    for branch in branches {
        // Find a path from any constraint in this branch
        let any_path = branch.iter().find(|ci| !ci.path.is_empty()).map(|ci| &ci.path[0]);
        let path0 = any_path?;

        if path0.starts_with('^') {
            // Inverse path branch, single-segment (^Class.field) or two-segment
            // (^Class.field / Class.otherField): the latter counts, per outer
            // object, how many referrers ALSO have the second field set (e.g.
            // "how many referring Terminals also have a ConnectivityNode").
            let inv_ci = branch.iter().find(|ci| !ci.path.is_empty())?;
            let prelude = match inv_ci.path.len() {
                1 => build_inverse_count_prelude(spec, &inv_ci.path[0], inv_idx)?,
                2 => build_inverse_forward_count_prelude(spec, &inv_ci.path, inv_idx)?,
                _ => return None, // longer chains: skip
            };
            let min = branch.iter().find(|ci| ci.component == "sh:MinCountConstraintComponent")
                .and_then(|ci| ci.payload.get("minCount").and_then(|v| v.as_int()))
                .unwrap_or(0);
            let max = branch.iter().find(|ci| ci.component == "sh:MaxCountConstraintComponent")
                .and_then(|ci| ci.payload.get("maxCount").and_then(|v| v.as_int()))
                .unwrap_or(i64::MAX / 2);
            prelude_parts.push(prelude);
            fail_conds.push(inverse_fail_cond(inv_idx, min, max));
            inv_idx += 1;
        } else {
            // Forward field branch — determine what kind
            if branch.iter().any(|ci| ci.path.len() > 1) { return None; } // multi-seg: skip
            let attr_id = local_name(path0);
            let (acc_prefix, attr) = find_attr_in_hierarchy(spec, class_name, &attr_id)?;
            let field = sanitize_field(to_snake_case(&attr.label));
            let accessor = format!("{acc_prefix}.{field}");

            let max = branch.iter().find(|ci| ci.component == "sh:MaxCountConstraintComponent")
                .and_then(|ci| ci.payload.get("maxCount").and_then(|v| v.as_int()));
            let min = branch.iter().find(|ci| ci.component == "sh:MinCountConstraintComponent")
                .and_then(|ci| ci.payload.get("minCount").and_then(|v| v.as_int()));

            let fail_cond = if max == Some(0) {
                // Field must be absent
                field_absent_cond(&attr, &accessor)?
            } else if min == Some(1) && (max == Some(1) || max.is_none()) {
                // Field must be present
                field_required_cond(&attr, &accessor)?
            } else {
                return None; // unsupported pattern
            };
            needs_obj = true;
            fail_conds.push(fail_cond);
        }
    }

    if fail_conds.is_empty() { return None; }
    let any_fail = fail_conds.join(" || ");

    let (message, severity, name_str, rule_id_str, desc_str, prop) = extract_violation_fields(c);

    let mut s = String::new();
    writeln!(s, "pub fn {fn_name}(dataset: &CimDataset) -> Vec<Violation> {{").unwrap();
    for p in &prelude_parts { s.push_str(p); }
    writeln!(s, "    let mut violations = Vec::new();").unwrap();
    writeln!(s, "    for mrid in dataset.by_type.get(\"{class_name}\").into_iter().flatten() {{").unwrap();
    if needs_obj {
        writeln!(s, "        let entry = &dataset.entries[mrid];").unwrap();
        writeln!(s, "        let obj = match entry.element.as_any()").unwrap();
        writeln!(s, "            .downcast_ref::<cimstructs::{class_name}>() {{").unwrap();
        writeln!(s, "            Some(o) => o, None => continue,").unwrap();
        writeln!(s, "        }};").unwrap();
    }
    writeln!(s, "        if {any_fail} {{").unwrap();
    write_violation(&mut s, class_name, &prop, &message, &severity, &name_str, &rule_id_str, &desc_str, "            ");
    writeln!(s, "        }}").unwrap();
    writeln!(s, "    }}").unwrap();
    writeln!(s, "    violations").unwrap();
    writeln!(s, "}}").unwrap();
    Some(s)
}

// ---------------------------------------------------------------------------
// Compound generator helpers
// ---------------------------------------------------------------------------

/// Build the prelude code that counts inverse references for one branch.
/// `path0` is the raw inverse path segment like "^cim:Terminal.ConductingEquipment".
fn build_inverse_count_prelude(spec: &CimSpecification, path0: &str, idx: usize) -> Option<String> {
    let forward_pred = &path0[1..]; // strip leading '^'
    let local = local_name(forward_pred);
    let (src_class, _) = local.split_once('.')?;
    let (_, attr) = find_attr_in_hierarchy(spec, src_class, &local)?;
    if !attr.is_association_used { return None; }
    let field = sanitize_field(to_snake_case(&attr.label));

    let mut s = String::new();
    writeln!(s, "    let mut counts{idx}: std::collections::HashMap<String, usize> = std::collections::HashMap::new();").unwrap();
    // The referrer class can be abstract — scan every concrete subclass that
    // carries the forward field (dataset.by_type only holds concrete type names).
    let mut emitted = false;
    for cls in class_and_subclasses(spec, src_class) {
        let Some((acc_prefix, _)) = find_attr_in_hierarchy(spec, &cls, &local) else { continue };
        let src_prefix = acc_prefix.replacen("obj", "src", 1);
        let src_field = format!("{src_prefix}.{field}");
        emitted = true;
        writeln!(s, "    for src_mrid in dataset.by_type.get(\"{cls}\").into_iter().flatten() {{").unwrap();
        writeln!(s, "        if let Some(src) = dataset.entries.get(src_mrid)").unwrap();
        writeln!(s, "            .and_then(|e| e.element.as_any().downcast_ref::<cimstructs::{cls}>()) {{").unwrap();
        if attr.is_list {
            writeln!(s, "            for r in &{src_field} {{").unwrap();
            writeln!(s, "                *counts{idx}.entry(r.mrid.trim_start_matches('#').to_string()).or_insert(0) += 1;").unwrap();
            writeln!(s, "            }}").unwrap();
        } else {
            writeln!(s, "            if let Some(r) = &{src_field} {{").unwrap();
            writeln!(s, "                *counts{idx}.entry(r.mrid.trim_start_matches('#').to_string()).or_insert(0) += 1;").unwrap();
            writeln!(s, "            }}").unwrap();
        }
        writeln!(s, "        }}").unwrap();
        writeln!(s, "    }}").unwrap();
    }
    if !emitted { return None; }
    Some(s)
}

/// Build the prelude for a two-segment inverse-then-forward branch:
/// `[^Class.field, Class.otherField]` (e.g. "how many Terminals refer to this
/// Clamp via Terminal.ConductingEquipment AND also have Terminal.ConnectivityNode
/// set" — used by C:452:EQ:Clamp:numberOfTerminals's sh:and). Counts, per outer
/// object, referrers for which the SECOND field also resolves — a referrer
/// missing the second field doesn't count.
fn build_inverse_forward_count_prelude(spec: &CimSpecification, path: &[String], idx: usize) -> Option<String> {
    if path.len() != 2 { return None; }
    let inv_local = local_name(&path[0][1..]); // strip leading '^'
    let (src_class, _) = inv_local.split_once('.')?;
    let fwd_local = local_name(&path[1]);
    if !fwd_local.contains('.') { return None; }

    let mut s = String::new();
    writeln!(s, "    let mut counts{idx}: std::collections::HashMap<String, usize> = std::collections::HashMap::new();").unwrap();
    let mut emitted = false;
    for cls in class_and_subclasses(spec, src_class) {
        let Some((inv_prefix, inv_attr)) = find_attr_in_hierarchy(spec, &cls, &inv_local) else { continue };
        let Some((fwd_prefix, fwd_attr)) = find_attr_in_hierarchy(spec, &cls, &fwd_local) else { continue };
        if !inv_attr.is_association_used || !fwd_attr.is_association_used { continue; }
        let inv_field = sanitize_field(to_snake_case(&inv_attr.label));
        let inv_src_field = format!("{}.{inv_field}", inv_prefix.replacen("obj", "src", 1));
        let fwd_field = sanitize_field(to_snake_case(&fwd_attr.label));
        let fwd_src_field = format!("{}.{fwd_field}", fwd_prefix.replacen("obj", "src", 1));
        let fwd_present = if fwd_attr.is_list {
            format!("!{fwd_src_field}.is_empty()")
        } else {
            format!("{fwd_src_field}.is_some()")
        };
        emitted = true;
        writeln!(s, "    for src_mrid in dataset.by_type.get(\"{cls}\").into_iter().flatten() {{").unwrap();
        writeln!(s, "        if let Some(src) = dataset.entries.get(src_mrid)").unwrap();
        writeln!(s, "            .and_then(|e| e.element.as_any().downcast_ref::<cimstructs::{cls}>()) {{").unwrap();
        writeln!(s, "            if {fwd_present} {{").unwrap();
        if inv_attr.is_list {
            writeln!(s, "                for r in &{inv_src_field} {{").unwrap();
            writeln!(s, "                    *counts{idx}.entry(r.mrid.trim_start_matches('#').to_string()).or_insert(0) += 1;").unwrap();
            writeln!(s, "                }}").unwrap();
        } else {
            writeln!(s, "                if let Some(r) = &{inv_src_field} {{").unwrap();
            writeln!(s, "                    *counts{idx}.entry(r.mrid.trim_start_matches('#').to_string()).or_insert(0) += 1;").unwrap();
            writeln!(s, "                }}").unwrap();
        }
        writeln!(s, "            }}").unwrap();
        writeln!(s, "        }}").unwrap();
        writeln!(s, "    }}").unwrap();
    }
    if !emitted { return None; }
    Some(s)
}

fn inverse_fail_cond(idx: usize, min: i64, max: i64) -> String {
    let count = format!("counts{idx}.get(mrid.as_str()).copied().unwrap_or(0)");
    if min == max {
        format!("{count} != {min}")
    } else if max == i64::MAX / 2 {
        format!("{count} < {min}")
    } else if min == 0 {
        format!("{count} > {max}")
    } else {
        format!("{{ let c = {count}; c < {min} || c > {max} }}")
    }
}

/// Violation condition for "field must be absent" (sh:maxCount 0).
fn field_absent_cond(attr: &CimAttribute, accessor: &str) -> Option<String> {
    if attr.is_list {
        Some(format!("!{accessor}.is_empty()"))
    } else if attr.is_primitive || attr.is_cim_datatype {
        Some(match attr.lang_type.as_str() {
            "String" => format!("!{accessor}.is_empty()"),
            _ => format!("{accessor}.is_some()"),  // Option<f64>, Option<i64>, Option<bool>
        })
    } else if attr.is_enum_value {
        Some(format!("{accessor}.is_some()"))
    } else {
        Some(format!("{accessor}.is_some()"))  // Option<MridRef>
    }
}

/// Violation condition for "field must be present" (sh:minCount 1 + maxCount 1).
fn field_required_cond(attr: &CimAttribute, accessor: &str) -> Option<String> {
    if attr.is_list {
        Some(format!("{accessor}.is_empty()"))
    } else if attr.is_primitive || attr.is_cim_datatype {
        Some(match attr.lang_type.as_str() {
            "String" => format!("{accessor}.is_empty()"),
            _ => format!("{accessor}.is_none()"),
        })
    } else if attr.is_enum_value {
        Some(format!("{accessor}.is_none()"))
    } else {
        Some(format!("{accessor}.is_none()"))
    }
}

fn extract_violation_fields(c: &ConstraintInfo) -> (String, String, String, String, String, String) {
    fn esc(x: &str) -> String { x.replace('\\', "\\\\").replace('"', "\\\"") }
    let message     = esc(&c.message);
    let severity    = if c.severity.is_empty() { "sh:Violation".to_string() } else { c.severity.clone() };
    let name_str    = esc(&c.name);
    let rule_id_str = esc(&c.rule_id);
    let desc_str    = esc(&c.description);
    let prop        = esc(&c.name);
    (message, severity, name_str, rule_id_str, desc_str, prop)
}

fn write_fn_header(s: &mut String, fn_name: &str, class_name: &str) {
    writeln!(s, "pub fn {fn_name}(dataset: &CimDataset) -> Vec<Violation> {{").unwrap();
    writeln!(s, "    let mut violations = Vec::new();").unwrap();
    writeln!(s, "    for mrid in dataset.by_type.get(\"{class_name}\").into_iter().flatten() {{").unwrap();
    writeln!(s, "        let entry = &dataset.entries[mrid];").unwrap();
    writeln!(s, "        let obj = match entry.element.as_any()").unwrap();
    writeln!(s, "            .downcast_ref::<cimstructs::{class_name}>() {{").unwrap();
    writeln!(s, "            Some(o) => o, None => continue,").unwrap();
    writeln!(s, "        }};").unwrap();
}

fn write_fn_footer(s: &mut String) {
    writeln!(s, "    }}").unwrap();
    writeln!(s, "    violations").unwrap();
    writeln!(s, "}}").unwrap();
}

fn write_violation(s: &mut String, class_name: &str, prop: &str, message: &str, severity: &str, name_str: &str, rule_id_str: &str, desc_str: &str, indent: &str) {
    writeln!(s, "{indent}violations.push(Violation {{").unwrap();
    writeln!(s, "{indent}    object_id:   mrid.clone(),").unwrap();
    writeln!(s, "{indent}    rule_id:     \"{rule_id_str}\".to_string(),").unwrap();
    writeln!(s, "{indent}    class:       \"{class_name}\".to_string(),").unwrap();
    writeln!(s, "{indent}    property:    \"{prop}\".to_string(),").unwrap();
    writeln!(s, "{indent}    message:     \"{message}\".to_string(),").unwrap();
    writeln!(s, "{indent}    severity:    \"{severity}\".to_string(),").unwrap();
    writeln!(s, "{indent}    name:        \"{name_str}\".to_string(),").unwrap();
    writeln!(s, "{indent}    description: \"{desc_str}\".to_string(),").unwrap();
    writeln!(s, "{indent}}});").unwrap();
}

fn dedup_fn_name(raw_name: String, used: &std::collections::HashSet<String>) -> String {
    if !used.contains(&raw_name) {
        return raw_name;
    }
    let mut n = 2usize;
    loop {
        let candidate = format!("{raw_name}_{n}");
        if !used.contains(&candidate) { return candidate; }
        n += 1;
    }
}

// ---------------------------------------------------------------------------
// Class hierarchy helpers
// ---------------------------------------------------------------------------

fn is_subtype_of(spec: &CimSpecification, type_name: &str, ancestor: &str) -> bool {
    let mut current = type_name.to_string();
    let mut visited = std::collections::HashSet::new();
    loop {
        if !visited.insert(current.clone()) {
            return false;
        }
        match spec.types.get(&current) {
            Some(t) => {
                if t.super_type.is_empty() || t.super_type == current {
                    return false;
                }
                if t.super_type == ancestor {
                    return true;
                }
                current = t.super_type.clone();
            }
            None => return false,
        }
    }
}

fn class_and_subclasses(spec: &CimSpecification, class: &str) -> Vec<String> {
    let mut result: Vec<String> = spec.types
        .keys()
        .filter(|name| name.as_str() == class || is_subtype_of(spec, name, class))
        .cloned()
        .collect();
    result.sort();
    result
}
