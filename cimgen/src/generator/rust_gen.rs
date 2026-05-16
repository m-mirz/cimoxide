use std::collections::HashSet;
use std::fmt::Write as FmtWrite;
use std::fs;
use std::path::Path;

use crate::schema::model::*;

pub fn generate_rust(
    spec: &mut CimSpecification,
    output_dir: &Path,
) -> Result<(), Box<dyn std::error::Error>> {
    fs::create_dir_all(output_dir)?;

    set_lang_types(spec);

    let mut module_ids: Vec<String> = Vec::new();

    // Structs
    for (id, t) in &spec.types {
        let code = render_struct(t);
        let file = output_dir.join(format!("{id}.rs"));
        fs::write(&file, code)?;
        module_ids.push(id.clone());
    }

    // Enums
    for (id, e) in &spec.enums {
        if e.values.is_empty() {
            continue;
        }
        let code = render_enum(e);
        let file = output_dir.join(format!("{id}.rs"));
        fs::write(&file, code)?;
        module_ids.push(id.clone());
    }

    // Type aliases (only non-Compound CIMDatatypes with a resolved primitive type)
    for (id, dt) in &spec.cim_datatypes {
        if dt.cim_stereotype == "Compound" || dt.lang_type.is_empty() {
            continue;
        }
        let code = render_type_alias(dt);
        let file = output_dir.join(format!("{id}.rs"));
        fs::write(&file, code)?;
        module_ids.push(id.clone());
    }

    // base.rs — helper reference types
    fs::write(output_dir.join("base.rs"), BASE_RS)?;

    // constants.rs
    fs::write(output_dir.join("constants.rs"), render_constants(spec))?;

    // lib.rs / mod.rs
    module_ids.sort();
    fs::write(output_dir.join("lib.rs"), render_lib(&module_ids))?;

    Ok(())
}

// --- type mapping -----------------------------------------------------------

fn map_lang_type(data_type: &str) -> &'static str {
    match data_type {
        DATA_TYPE_STRING | DATA_TYPE_DATE | DATA_TYPE_DATE_TIME | DATA_TYPE_MONTH_DAY | "URI" => {
            "String"
        }
        DATA_TYPE_BOOLEAN => "bool",
        DATA_TYPE_INTEGER => "i64",
        DATA_TYPE_FLOAT | DATA_TYPE_DECIMAL => "f64",
        _ => "String",
    }
}

fn set_lang_types(spec: &mut CimSpecification) {
    for dt in spec.cim_datatypes.values_mut() {
        dt.lang_type = map_lang_type(&dt.primitive_type).to_string();
    }
    for p in spec.primitive_types.values_mut() {
        p.lang_type = map_lang_type(&p.data_type).to_string();
    }
    for t in spec.types.values_mut() {
        for attr in &mut t.attributes {
            let base = map_lang_type(&attr.data_type);
            attr.lang_type = if attr.is_list && (attr.is_primitive || attr.is_cim_datatype) {
                format!("Vec<{base}>")
            } else {
                base.to_string()
            };
        }
    }
}

// --- renderers --------------------------------------------------------------

fn render_struct(t: &CimType) -> String {
    let mut s = String::new();
    if !t.comment.is_empty() {
        writeln!(s, "/// {}", t.comment).unwrap();
    }
    writeln!(s, "#[derive(Debug, Default, Clone)]").unwrap();
    writeln!(s, "pub struct {} {{", t.id).unwrap();

    if t.super_type.is_empty() {
        writeln!(s, "    pub id: String,").unwrap();
    } else {
        writeln!(s, "    pub base: super::{},", t.super_type).unwrap();
    }

    for attr in t.attributes.iter().filter(|a| a.is_association_used) {
        if !attr.comment.is_empty() {
            writeln!(s, "    /// {}", attr.comment).unwrap();
        }
        let fname = sanitize_field(to_snake_case(&attr.label));
        let ftype = field_type(attr);
        writeln!(s, "    pub {fname}: {ftype},").unwrap();
    }

    writeln!(s, "}}").unwrap();
    s
}

fn field_type(attr: &CimAttribute) -> String {
    if attr.is_primitive || attr.is_cim_datatype {
        attr.lang_type.clone()
    } else if attr.is_enum_value {
        format!("Option<super::base::UriRef>")
    } else if attr.is_list {
        "Vec<super::base::MridRef>".to_string()
    } else {
        "Option<super::base::MridRef>".to_string()
    }
}

fn render_enum(e: &CimEnum) -> String {
    let mut s = String::new();
    if !e.comment.is_empty() {
        writeln!(s, "/// {}", e.comment).unwrap();
    }
    writeln!(s, "#[derive(Debug, Clone, PartialEq)]").unwrap();
    writeln!(s, "pub enum {} {{", e.id).unwrap();
    for v in &e.values {
        if !v.comment.is_empty() {
            writeln!(s, "    /// {}", v.comment).unwrap();
        }
        writeln!(s, "    {},", sanitize_variant(&v.label)).unwrap();
    }
    writeln!(s, "}}").unwrap();
    writeln!(s).unwrap();
    writeln!(s, "impl {} {{", e.id).unwrap();
    writeln!(s, "    pub fn uri(&self) -> &'static str {{").unwrap();
    writeln!(s, "        match self {{").unwrap();
    let ns = if e.namespace.is_empty() {
        "http://iec.ch/TC57/CIM100".to_string()
    } else {
        e.namespace.trim_end_matches('#').to_string()
    };
    for v in &e.values {
        writeln!(
            s,
            "            {}::{} => \"{}#{}\",",
            e.id,
            sanitize_variant(&v.label),
            ns,
            v.id
        )
        .unwrap();
    }
    writeln!(s, "        }}").unwrap();
    writeln!(s, "    }}").unwrap();
    writeln!(s, "}}").unwrap();
    s
}

fn render_type_alias(dt: &CimDatatype) -> String {
    let mut s = String::new();
    if !dt.comment.is_empty() {
        writeln!(s, "/// {}", dt.comment).unwrap();
    }
    writeln!(s, "pub type {} = {};", dt.id, dt.lang_type).unwrap();
    s
}

fn render_constants(spec: &CimSpecification) -> String {
    let mut s = String::new();
    writeln!(s, "pub const CIM_VERSION: &str = \"{}\";", spec.cgmes_version).unwrap();
    writeln!(s).unwrap();
    writeln!(
        s,
        "pub static CIM_NAMESPACES: &[(&str, &str)] = &["
    )
    .unwrap();
    let mut pairs: Vec<(&String, &String)> = spec.profile_namespaces.iter().collect();
    pairs.sort_by_key(|(k, _)| k.as_str());
    for (k, v) in pairs {
        writeln!(s, "    (\"{k}\", \"{v}\"),").unwrap();
    }
    writeln!(s, "];").unwrap();
    s
}

fn render_lib(ids: &[String]) -> String {
    let mut s = String::new();
    writeln!(s, "// Generated by cimgen — do not edit by hand.").unwrap();
    writeln!(s, "#![allow(non_snake_case, non_camel_case_types, dead_code)]").unwrap();
    writeln!(s).unwrap();
    writeln!(s, "pub mod base;").unwrap();
    writeln!(s, "pub mod constants;").unwrap();
    writeln!(s).unwrap();

    // Collect unique IDs (struct/enum/alias files may share names in edge cases)
    let unique: Vec<&String> = {
        let mut seen = HashSet::new();
        ids.iter().filter(|id| seen.insert(id.as_str())).collect()
    };

    for id in unique {
        let mod_name = to_snake_case(id);
        writeln!(s, "#[path = \"{id}.rs\"]").unwrap();
        writeln!(s, "mod {mod_name}_mod;").unwrap();
        writeln!(s, "pub use {mod_name}_mod::*;").unwrap();
    }
    s
}

// --- name helpers -----------------------------------------------------------

pub fn to_snake_case(s: &str) -> String {
    let chars: Vec<char> = s.chars().collect();
    let mut out = String::with_capacity(s.len() + 4);
    for (i, &c) in chars.iter().enumerate() {
        if c.is_uppercase() {
            let prev_lower = i > 0 && chars[i - 1].is_lowercase();
            let next_lower = i + 1 < chars.len() && chars[i + 1].is_lowercase();
            let prev_upper = i > 0 && chars[i - 1].is_uppercase();
            if i > 0 && (prev_lower || (next_lower && prev_upper)) {
                out.push('_');
            }
            out.extend(c.to_lowercase());
        } else {
            out.push(c);
        }
    }
    out
}

fn sanitize_field(name: String) -> String {
    match name.as_str() {
        "type" | "where" | "in" | "for" | "let" | "pub" | "use" | "mod" | "match" | "if"
        | "else" | "fn" | "struct" | "enum" | "impl" | "trait" | "return" | "break"
        | "continue" | "loop" | "while" | "move" | "ref" | "mut" | "const" | "static"
        | "self" | "super" | "crate" | "extern" | "unsafe" | "async" | "await" | "dyn"
        | "box" | "true" | "false" | "abstract" | "become" | "do" | "final" | "macro"
        | "override" | "priv" | "typeof" | "unsized" | "virtual" | "yield" => {
            format!("{name}_")
        }
        _ => name,
    }
}

fn sanitize_variant(label: &str) -> String {
    if label.is_empty() {
        return "_Empty".to_string();
    }
    let first = label.chars().next().unwrap();
    if first.is_ascii_digit() {
        format!("_{label}")
    } else {
        label.to_string()
    }
}

// --- static content ---------------------------------------------------------

const BASE_RS: &str = r#"
/// A reference to another CIM object by MRID.
#[derive(Debug, Default, Clone)]
pub struct MridRef {
    pub mrid: String,
}

/// A reference to a CIM enum value by URI.
#[derive(Debug, Default, Clone)]
pub struct UriRef {
    pub uri: String,
}
"#;
