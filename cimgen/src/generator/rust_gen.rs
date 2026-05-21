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
    let spec: &CimSpecification = &*spec;

    let mut module_ids: Vec<String> = Vec::new();

    // Structs
    for (id, t) in &spec.types {
        let mut code = render_struct(t);
        code.push_str(&render_from_block(spec, t));
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

    // base.rs — shared runtime types
    fs::write(output_dir.join("base.rs"), BASE_RS)?;

    // constants.rs
    fs::write(output_dir.join("constants.rs"), render_constants(spec))?;

    // registry.rs — type-name → from_block dispatch table
    fs::write(output_dir.join("registry.rs"), render_registry(spec))?;

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
    writeln!(s, "#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]").unwrap();
    writeln!(s, "pub struct {} {{", t.id).unwrap();

    if t.super_type.is_empty() {
        writeln!(s, "    pub id: String,").unwrap();
    } else {
        writeln!(s, "    #[serde(flatten)]").unwrap();
        writeln!(s, "    pub base: super::{},", t.super_type).unwrap();
    }

    for attr in t.attributes.iter().filter(|a| a.is_association_used) {
        if !attr.comment.is_empty() {
            writeln!(s, "    /// {}", attr.comment).unwrap();
        }
        let fname = sanitize_field(to_snake_case(&attr.label));
        let ftype = field_type(attr);
        if ftype.starts_with("Option<") {
            writeln!(s, "    #[serde(skip_serializing_if = \"Option::is_none\")]").unwrap();
        }
        writeln!(s, "    pub {fname}: {ftype},").unwrap();
    }

    writeln!(s, "}}").unwrap();
    s
}

fn field_type(attr: &CimAttribute) -> String {
    if attr.is_primitive || attr.is_cim_datatype {
        if !attr.is_list {
            match attr.lang_type.as_str() {
                "f64" => return "Option<f64>".to_string(),
                "i64" => return "Option<i64>".to_string(),
                "bool" => return "Option<bool>".to_string(),
                _ => {}
            }
        }
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
    writeln!(s, "#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]").unwrap();
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
    writeln!(s, "pub mod registry;").unwrap();
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

pub fn sanitize_field(name: String) -> String {
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

// --- from_block code generation ---------------------------------------------

fn id_obj_path(spec: &CimSpecification, t: &CimType, obj: &str) -> String {
    let mut depth = 0usize;
    let mut current = t.super_type.clone();
    while !current.is_empty() {
        depth += 1;
        current = spec
            .types
            .get(&current)
            .map(|p| p.super_type.clone())
            .unwrap_or_default();
    }
    if depth == 0 {
        format!("{obj}.id")
    } else {
        format!("{obj}.{}id", "base.".repeat(depth))
    }
}

fn emit_attr_arm(s: &mut String, prefix: &str, attr: &CimAttribute) {
    let fname = sanitize_field(to_snake_case(&attr.label));
    writeln!(s, "                \"{}\" => {{", attr.id).unwrap();

    if attr.is_primitive || attr.is_cim_datatype {
        if attr.is_list {
            writeln!(s, "                    if let crate::base::FieldValue::Text(sv) = val {{").unwrap();
            if attr.lang_type == "Vec<String>" || attr.lang_type == "String" {
                writeln!(s, "                        {prefix}.{fname}.push(sv.trim().to_string());").unwrap();
            } else {
                writeln!(s, "                        if let Ok(v) = sv.trim().parse() {{ {prefix}.{fname}.push(v); }}").unwrap();
            }
            writeln!(s, "                    }}").unwrap();
        } else {
            match attr.lang_type.as_str() {
                "String" => {
                    writeln!(s, "                    if let crate::base::FieldValue::Text(sv) = val {{").unwrap();
                    writeln!(s, "                        {prefix}.{fname}.clone_from(sv);").unwrap();
                    writeln!(s, "                    }}").unwrap();
                }
                "bool" => {
                    writeln!(s, "                    if let crate::base::FieldValue::Text(sv) = val {{").unwrap();
                    writeln!(s, "                        {prefix}.{fname} = Some(sv.trim() == \"true\");").unwrap();
                    writeln!(s, "                    }}").unwrap();
                }
                _ => {
                    writeln!(s, "                    if let crate::base::FieldValue::Text(sv) = val {{").unwrap();
                    writeln!(s, "                        if let Ok(v) = sv.trim().parse() {{ {prefix}.{fname} = Some(v); }}").unwrap();
                    writeln!(s, "                    }}").unwrap();
                }
            }
        }
    } else if attr.is_enum_value {
        writeln!(s, "                    if let crate::base::FieldValue::Resource(sv) = val {{").unwrap();
        writeln!(s, "                        {prefix}.{fname} = Some(crate::base::UriRef {{ uri: sv.clone() }});").unwrap();
        writeln!(s, "                    }}").unwrap();
    } else if attr.is_list {
        writeln!(s, "                    match val {{").unwrap();
        writeln!(s, "                        crate::base::FieldValue::Resource(sv) => {prefix}.{fname}.push(crate::base::MridRef {{ mrid: sv.clone() }}),").unwrap();
        writeln!(s, "                        crate::base::FieldValue::ResourceList(svs) => {{").unwrap();
        writeln!(s, "                            for sv in svs {{ {prefix}.{fname}.push(crate::base::MridRef {{ mrid: sv.clone() }}); }}").unwrap();
        writeln!(s, "                        }}").unwrap();
        writeln!(s, "                        _ => {{}}").unwrap();
        writeln!(s, "                    }}").unwrap();
    } else {
        writeln!(s, "                    if let crate::base::FieldValue::Resource(sv) = val {{").unwrap();
        writeln!(s, "                        {prefix}.{fname} = Some(crate::base::MridRef {{ mrid: sv.clone() }});").unwrap();
        writeln!(s, "                    }}").unwrap();
    }

    writeln!(s, "                }}").unwrap();
}

fn emit_to_block_field(s: &mut String, attr: &CimAttribute) {
    let fname = sanitize_field(to_snake_case(&attr.label));
    if attr.is_primitive || attr.is_cim_datatype {
        if attr.is_list {
            writeln!(s, "        for v in &self.{fname} {{").unwrap();
            writeln!(s, "            block.fields.insert(\"{}\".into(), crate::base::FieldValue::Text(v.to_string()));", attr.id).unwrap();
            writeln!(s, "        }}").unwrap();
        } else {
            match attr.lang_type.as_str() {
                "String" => {
                    writeln!(s, "        if !self.{fname}.is_empty() {{").unwrap();
                    writeln!(s, "            block.fields.insert(\"{}\".into(), crate::base::FieldValue::Text(self.{fname}.clone()));", attr.id).unwrap();
                    writeln!(s, "        }}").unwrap();
                }
                _ => {
                    writeln!(s, "        if let Some(v) = self.{fname} {{").unwrap();
                    writeln!(s, "            block.fields.insert(\"{}\".into(), crate::base::FieldValue::Text(v.to_string()));", attr.id).unwrap();
                    writeln!(s, "        }}").unwrap();
                }
            }
        }
    } else if attr.is_enum_value {
        writeln!(s, "        if let Some(ref v) = self.{fname} {{").unwrap();
        writeln!(s, "            block.fields.insert(\"{}\".into(), crate::base::FieldValue::Resource(v.uri.clone()));", attr.id).unwrap();
        writeln!(s, "        }}").unwrap();
    } else if attr.is_list {
        writeln!(s, "        if !self.{fname}.is_empty() {{").unwrap();
        writeln!(s, "            block.fields.insert(\"{}\".into(), crate::base::FieldValue::ResourceList(self.{fname}.iter().map(|r| r.mrid.clone()).collect()));", attr.id).unwrap();
        writeln!(s, "        }}").unwrap();
    } else {
        writeln!(s, "        if let Some(ref v) = self.{fname} {{").unwrap();
        writeln!(s, "            block.fields.insert(\"{}\".into(), crate::base::FieldValue::Resource(v.mrid.clone()));", attr.id).unwrap();
        writeln!(s, "        }}").unwrap();
    }
}

fn render_from_block(spec: &CimSpecification, t: &CimType) -> String {
    let mut s = String::new();

    // CimElement trait impl
    let self_id = id_obj_path(spec, t, "self");
    writeln!(s, "impl crate::base::CimElement for {id} {{", id = t.id).unwrap();
    writeln!(s, "    fn mrid(&self) -> &str {{ &{self_id} }}").unwrap();
    writeln!(s, "    fn type_name(&self) -> &'static str {{ \"{}\" }}", t.id).unwrap();
    writeln!(s, "    fn as_any(&self) -> &dyn std::any::Any {{ self }}").unwrap();
    writeln!(s, "    fn to_json_value(&self) -> serde_json::Value {{").unwrap();
    writeln!(s, "        serde_json::to_value(self).unwrap_or(serde_json::Value::Null)").unwrap();
    writeln!(s, "    }}").unwrap();
    writeln!(s, "    fn to_block(&self) -> crate::base::RdfBlock {{").unwrap();
    let has_own = t.attributes.iter().any(|a| a.is_association_used);
    // root types: mut only if fields to insert; inherited types: always mut (type_name update)
    let needs_mut = has_own || !t.super_type.is_empty();
    let mut_kw = if needs_mut { "mut " } else { "" };
    if t.super_type.is_empty() {
        writeln!(s, "        let {mut_kw}block = crate::base::RdfBlock {{").unwrap();
        writeln!(s, "            type_name: \"{}\".to_string(),", t.id).unwrap();
        writeln!(s, "            mrid: self.id.clone(),").unwrap();
        writeln!(s, "            fields: std::collections::HashMap::new(),").unwrap();
        writeln!(s, "        }};").unwrap();
    } else {
        writeln!(s, "        let {mut_kw}block = self.base.to_block();").unwrap();
        writeln!(s, "        block.type_name = \"{}\".to_string();", t.id).unwrap();
    }
    for attr in t.attributes.iter().filter(|a| a.is_association_used) {
        emit_to_block_field(&mut s, attr);
    }
    writeln!(s, "        block").unwrap();
    writeln!(s, "    }}").unwrap();
    writeln!(s, "}}").unwrap();
    writeln!(s).unwrap();

    // Collect all (field-access-prefix, attrs) layers: own first, then ancestors
    let mut layers: Vec<(String, Vec<CimAttribute>)> = Vec::new();
    layers.push(("obj".to_string(), t.attributes.clone()));
    let mut prefix = "obj.base".to_string();
    let mut current_id = t.super_type.clone();
    while let Some(ancestor) = spec.types.get(&current_id) {
        layers.push((prefix.clone(), ancestor.attributes.clone()));
        prefix = format!("{prefix}.base");
        current_id = ancestor.super_type.clone();
    }

    let has_fields = layers
        .iter()
        .any(|(_, attrs)| attrs.iter().any(|a| a.is_association_used));

    let obj_id = id_obj_path(spec, t, "obj");
    writeln!(s, "impl {id} {{", id = t.id).unwrap();
    writeln!(s, "    pub fn from_block(b: &crate::base::RdfBlock) -> Self {{").unwrap();
    writeln!(s, "        let mut obj = Self::default();").unwrap();
    writeln!(s, "        {obj_id}.clone_from(&b.mrid);").unwrap();

    if has_fields {
        writeln!(s, "        for (key, val) in &b.fields {{").unwrap();
        writeln!(s, "            match key.as_str() {{").unwrap();
        for (pfx, attrs) in &layers {
            for attr in attrs.iter().filter(|a| a.is_association_used) {
                emit_attr_arm(&mut s, pfx, attr);
            }
        }
        writeln!(s, "                _ => {{}}").unwrap();
        writeln!(s, "            }}").unwrap();
        writeln!(s, "        }}").unwrap();
    }

    writeln!(s, "        obj").unwrap();
    writeln!(s, "    }}").unwrap();
    writeln!(s, "}}").unwrap();
    s
}

fn render_registry(spec: &CimSpecification) -> String {
    let mut s = String::new();
    writeln!(s, "// Generated by cimgen — do not edit by hand.").unwrap();
    writeln!(s, "use std::collections::HashMap;").unwrap();
    writeln!(s, "use crate::base::{{CimElement, RdfBlock}};").unwrap();
    writeln!(s).unwrap();
    writeln!(s, "pub type ParseFn = fn(&RdfBlock) -> Box<dyn CimElement>;").unwrap();
    writeln!(s).unwrap();
    writeln!(s, "pub fn registry() -> HashMap<&'static str, ParseFn> {{").unwrap();
    writeln!(s, "    let mut m: HashMap<&'static str, ParseFn> = HashMap::new();").unwrap();

    let mut ids: Vec<&String> = spec.types.keys().collect();
    ids.sort();
    for id in &ids {
        writeln!(s, "    m.insert(\"{id}\", |b| Box::new(super::{id}::from_block(b)));").unwrap();
    }

    writeln!(s, "    m").unwrap();
    writeln!(s, "}}").unwrap();

    writeln!(s).unwrap();
    writeln!(s, "pub type JsonParseFn = fn(serde_json::Value) -> Result<Box<dyn CimElement>, serde_json::Error>;").unwrap();
    writeln!(s).unwrap();
    writeln!(s, "pub fn json_registry() -> HashMap<&'static str, JsonParseFn> {{").unwrap();
    writeln!(s, "    let mut m: HashMap<&'static str, JsonParseFn> = HashMap::new();").unwrap();
    for id in &ids {
        writeln!(s, "    m.insert(\"{id}\", |v| Ok(Box::new(serde_json::from_value::<super::{id}>(v)?)));").unwrap();
    }
    writeln!(s, "    m").unwrap();
    writeln!(s, "}}").unwrap();
    s
}

// --- static content ---------------------------------------------------------

const BASE_RS: &str = r#"use std::collections::HashMap;

pub trait CimElement {
    fn mrid(&self) -> &str;
    fn type_name(&self) -> &'static str;
    fn as_any(&self) -> &dyn std::any::Any;
    fn to_json_value(&self) -> serde_json::Value;
    fn to_block(&self) -> RdfBlock;
}

#[derive(Debug, Clone)]
pub enum FieldValue {
    Text(String),
    Resource(String),
    ResourceList(Vec<String>),
}

#[derive(Debug, Default, Clone)]
pub struct RdfBlock {
    pub type_name: String,
    pub mrid: String,
    pub fields: HashMap<String, FieldValue>,
}

impl RdfBlock {
    pub fn merge_from(&mut self, other: &RdfBlock) {
        for (k, v) in &other.fields {
            match v {
                FieldValue::ResourceList(new_list) => {
                    match self.fields.get_mut(k) {
                        Some(FieldValue::ResourceList(existing)) => {
                            existing.extend(new_list.iter().cloned())
                        }
                        _ => {
                            self.fields.insert(k.clone(), v.clone());
                        }
                    }
                }
                _ => {
                    self.fields.insert(k.clone(), v.clone());
                }
            }
        }
    }
}

/// A reference to another CIM object by MRID.
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct MridRef {
    pub mrid: String,
}

/// A reference to a CIM enum value by URI.
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct UriRef {
    pub uri: String,
}
"#;
