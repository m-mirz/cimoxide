//! The tools: their definitions as `tools/list` returns them, and what each
//! does. Every tool reads; `export` and `edit` write, a file inside the
//! workspace the caller names, never over an existing one unless told to, and
//! never a model set's files in place. Output is
//! plain text sized for a model's context — long lists stop at a `limit` and
//! say how many more there are.

use std::fmt::Write as _;
use std::path::{Component, Path, PathBuf};

use cimmodel::base::{AttrKind, ClassDef};
use cimmodel::registry::type_registry;
use cimmodel::{CimDataset, Element, FieldValue};
use cimsparql::{QueryResults, Term};
use serde_json::{Value, json};

use crate::session::{self, ModelSet, Options, Session};

const PATH_DOC: &str = "Model set: a directory of CIM RDF/XML files (or one file), relative to the workspace. \
Default: the workspace directory itself.";

const FORMAT_DOC: &str = "`rdfxml` (default), or `json`: every decoded object, CIM and foreign (reference data's SKOS, PROV, DCAT), \
as `{id: {\"_type\", \"id\", \"Class.attr\": value | [values], \"_lang\"?}}`, as cimoxide-py's `entries()` and `foreign()` give them. \
JSON cannot tell a reference (the target's mRID) from text and takes no `profile`.";

const FILES_DOC: &str = "Instead of `path`: the files that form one model set, relative to the workspace, when they \
are not one directory (an IGM with its boundary and common data). A directory stands for its CIM files. Merged in the order given.";

pub fn definitions() -> Value {
    let path = json!({ "type": "string", "description": PATH_DOC });
    let files = json!({ "type": "array", "items": { "type": "string" }, "description": FILES_DOC });
    let format = json!({ "type": "string", "enum": ["rdfxml", "json"], "description": FORMAT_DOC });
    let limit = |default: u64| json!({ "type": "integer", "minimum": 1, "description": format!("Most rows to return (default {default}).") });
    let read_only = json!({ "readOnlyHint": true, "openWorldHint": false });
    json!([
        {
            "name": "list_model_sets",
            "title": "List model sets",
            "description": "Find the directories holding CGMES/NC RDF/XML files, with each file's profiles (EQ, SSH, SV, TP, …) read from its header.",
            "inputSchema": { "type": "object", "properties": {
                "root": { "type": "string", "description": "Directory to search, relative to the workspace (default: the workspace)." },
                "depth": { "type": "integer", "minimum": 0, "description": "How many directory levels to descend (default 6)." },
            } },
            "annotations": read_only,
        },
        {
            "name": "summary",
            "title": "Summarise a model set",
            "description": "A model set's files, model headers (profiles, scenario time, dependencies) and object counts by class.",
            "inputSchema": { "type": "object", "properties": { "path": path, "files": files } },
            "annotations": read_only,
        },
        {
            "name": "find_objects",
            "title": "Find objects",
            "description": "Search a model set's objects by class (abstract classes include subclasses: `ConductingEquipment`, `Switch`), \
by name/mRID text, and by an attribute's value. Returns mRID, class and name of each match.",
            "inputSchema": { "type": "object", "properties": {
                "path": path,
                "files": files,
                "class": { "type": "string", "description": "Class name, e.g. `ACLineSegment`, `Breaker`, `nc:PowerSchedule`." },
                "text": { "type": "string", "description": "Case-insensitive substring of the name, description or mRID." },
                "attribute": { "type": "string", "description": "Field to filter on: `Switch.open`, or just `open`." },
                "value": { "type": "string", "description": "Value the attribute must have (text as written, a referenced mRID, or an enumeration value such as `PhaseCode.ABC`); omit to require only that it is present." },
                "limit": limit(50),
            } },
            "annotations": read_only,
        },
        {
            "name": "get_object",
            "title": "Get an object",
            "description": "One object by mRID: its class, the files that write it (defining it with `rdf:ID` or describing it with `rdf:about`), \
every field as written (references resolved to class and name) and the objects that reference it.",
            "inputSchema": { "type": "object", "properties": {
                "path": path,
                "files": files,
                "mrid": { "type": "string", "description": "The object's mRID (with or without leading `_` or `#`)." },
                "limit": limit(100),
            }, "required": ["mrid"] },
            "annotations": read_only,
        },
        {
            "name": "validate",
            "title": "Validate a model set",
            "description": "Run the ENTSO-E SHACL and SPARQL rules over a model set, as `cimcli validate` does. \
Returns counts by severity and rule, and the findings (filterable). Results are cached until a file changes.",
            "inputSchema": { "type": "object", "properties": {
                "path": path,
                "files": files,
                "common": { "type": "boolean", "description": "Also run the common cross-profile checks (default false)." },
                "quality": { "type": "boolean", "description": "Also run the modelling-quality checks (default false)." },
                "silence": { "type": "array", "items": { "type": "string" }, "description": "Rule ids not to report." },
                "rule": { "type": "string", "description": "Only findings whose rule id contains this text." },
                "object": { "type": "string", "description": "Only findings about this mRID." },
                "class": { "type": "string", "description": "Only findings about objects of this class." },
                "severity": { "type": "string", "enum": ["Violation", "Warning", "Info"], "description": "Only this severity." },
                "limit": limit(50),
            } },
            "annotations": read_only,
        },
        {
            "name": "conflicts",
            "title": "Merge conflicts",
            "description": "Where a model set's files disagree: objects more than one file defines (`rdf:ID`; common or boundary data \
defined again by a model), and objects the files type with unrelated classes. `get_object` shows which files wrote one object.",
            "inputSchema": { "type": "object", "properties": {
                "path": path,
                "files": files,
                "limit": limit(50),
            } },
            "annotations": read_only,
        },
        {
            "name": "sparql",
            "title": "SPARQL query",
            "description": "Run a SPARQL 1.1 query over a model set's merged CGMES data. Prefixes `cim:`, `eu:`, `md:`, `rdf:`, `xsd:` and \
the other CGMES namespaces are pre-bound. Classes are `cim:ACLineSegment`, properties `cim:IdentifiedObject.name`. \
Object IRIs come back as mRIDs. NC data is not in the graph.",
            "inputSchema": { "type": "object", "properties": {
                "path": path,
                "files": files,
                "query": { "type": "string", "description": "The query (SELECT, ASK, CONSTRUCT or DESCRIBE)." },
                "limit": limit(100),
            }, "required": ["query"] },
            "annotations": read_only,
        },
        {
            "name": "export",
            "title": "Export a model set",
            "description": "Write a model set as RDF/XML: one profile's content with that profile's header (CGMES `EQ`, `SSH`, `TP`, `SV`, …; \
NC `CO`, `AE`, `RA`, …), or with no `profile` the whole merged dataset as read (CGMES, NC and reference data). \
`format: \"json\"` writes every decoded object as JSON instead. Writes `output`; without it, returns the text when it is short.",
            "inputSchema": { "type": "object", "properties": {
                "path": path,
                "files": files,
                "profile": { "type": "string", "description": "Profile code; omit for the whole dataset." },
                "format": format,
                "output": { "type": "string", "description": "File to write, relative to the workspace and inside it. Not in the model set's own directory, or it joins the set." },
                "overwrite": { "type": "boolean", "description": "Replace `output` if it exists (default false)." },
            } },
            "annotations": { "readOnlyHint": false, "destructiveHint": true, "idempotentHint": true, "openWorldHint": false },
        },
        {
            "name": "edit",
            "title": "Edit a model set",
            "description": "Change a model set and write the result as RDF/XML, as `export` does; the set's files stay as they are. \
The changes apply in order, each to one object: update its `fields`, copy an object `from` another to a new `id`, create one of a `type`, \
or `delete` it. `keep: \"changed\"` writes only the objects changed (a dataset of its own, such as a response). To validate the result in place \
of a file, write it outside the set's directories and list it in `validate`'s `files` instead of that file.",
            "inputSchema": { "type": "object", "properties": {
                "path": path,
                "files": files,
                "changes": { "type": "array", "items": { "type": "object", "properties": {
                    "id": { "type": "string", "description": "The object's mRID as the data writes it (`_…`, `urn:uuid:…`): an existing one to update or delete, a new one to create or copy to." },
                    "from": { "type": "string", "description": "Copy this object, class and fields, to the new `id`; a field holding its own mRID as text (`IdentifiedObject.mRID`) follows the new id." },
                    "type": { "type": "string", "description": "Class of a new object (`ACLineSegment`, `nc:RemedialActionScheduleResponse`)." },
                    "fields": { "type": "object", "description": "`Class.attr` → a string, an array of strings for a repeated field, or null to remove it. A reference is the target's mRID, an enumeration value `Enum.value`." },
                    "delete": { "type": "boolean", "description": "Remove the object." },
                }, "required": ["id"] } },
                "keep": { "type": "string", "enum": ["all", "changed"], "description": "Write every object (default) or only those changed." },
                "profile": { "type": "string", "description": "Profile code to write; omit for the whole dataset." },
                "format": format,
                "output": { "type": "string", "description": "File to write, relative to the workspace and inside it." },
                "overwrite": { "type": "boolean", "description": "Replace `output` if it exists (default false)." },
            }, "required": ["changes"] },
            "annotations": { "readOnlyHint": false, "destructiveHint": true, "idempotentHint": true, "openWorldHint": false },
        },
        {
            "name": "describe_class",
            "title": "Describe a CIM class",
            "description": "What the CGMES/NC schema says about a class: its definition from the ENTSO-E vocabulary, superclasses, subclasses, profiles \
and every attribute and association it may carry (type, multiplicity, profiles, definition). Name an attribute (`SvStatus.inService`) for its full definition alone. \
Also describes enumerations (`WindingConnection`: every value with its definition) and enumeration values (`WindingConnection.D`).",
            "inputSchema": { "type": "object", "properties": {
                "class": { "type": "string", "description": "Class name, e.g. `SynchronousMachine`; `nc:` prefix for NC only. Or `Class.attribute` for one attribute, or an enumeration or `Enumeration.value`." },
                "include_unused": { "type": "boolean", "description": "Also list association ends no profile exchanges (default false)." },
            }, "required": ["class"] },
            "annotations": read_only,
        },
    ])
}

/// Run tool `name`: `None` if there is no such tool, else its text or the
/// error to show the model.
pub fn call(session: &mut Session, name: &str, args: &Value) -> Option<Result<String, String>> {
    Some(match name {
        "list_model_sets" => list_model_sets(session, args),
        "summary" => summary(session, args),
        "find_objects" => find_objects(session, args),
        "get_object" => get_object(session, args),
        "validate" => validate(session, args),
        "conflicts" => conflicts(session, args),
        "sparql" => sparql(session, args),
        "export" => export(session, args),
        "edit" => edit(session, args),
        "describe_class" => describe_class(args),
        _ => return None,
    })
}

// --- Arguments --------------------------------------------------------------

fn str_arg<'a>(args: &'a Value, key: &str) -> Option<&'a str> {
    args.get(key).and_then(Value::as_str).map(str::trim).filter(|s| !s.is_empty())
}

fn bool_arg(args: &Value, key: &str) -> bool {
    args.get(key).and_then(Value::as_bool).unwrap_or(false)
}

fn usize_arg(args: &Value, key: &str, default: usize) -> usize {
    args.get(key).and_then(Value::as_u64).map_or(default, |n| n.max(1) as usize)
}

fn list_arg(args: &Value, key: &str) -> Vec<String> {
    match args.get(key) {
        Some(Value::Array(items)) => items.iter().filter_map(Value::as_str).map(str::to_string).collect(),
        Some(Value::String(s)) => s.split(',').map(str::trim).filter(|s| !s.is_empty()).map(str::to_string).collect(),
        _ => Vec::new(),
    }
}

// --- Shared helpers ---------------------------------------------------------

/// The model set `path` or `files` names.
fn model_set<'s>(session: &'s mut Session, args: &Value) -> Result<&'s mut ModelSet, String> {
    session.model_set(str_arg(args, "path"), &list_arg(args, "files"))
}

fn name_of(el: &Element) -> Option<&str> {
    el.get_str("IdentifiedObject.name").filter(|n| !n.is_empty())
}

/// `Class mrid "name"`.
fn label(el: &Element) -> String {
    match name_of(el) {
        Some(name) => format!("{} {} \"{name}\"", el.type_name(), el.mrid()),
        None => format!("{} {}", el.type_name(), el.mrid()),
    }
}

/// The element `mrid` names, tolerating a leading `#`, a missing or extra
/// `_` and a `urn:uuid:` prefix.
fn lookup<'a>(ds: &'a CimDataset, mrid: &str) -> Option<&'a Element> {
    let m = mrid.trim().trim_start_matches('#');
    let bare = m.strip_prefix("urn:uuid:").unwrap_or(m);
    let bare = bare.strip_prefix('_').unwrap_or(bare);
    [m.to_string(), format!("_{bare}"), bare.to_string(), format!("urn:uuid:{bare}")]
        .iter()
        .find_map(|k| ds.entries.get(k))
}

/// Whether `el` is an instance of the class `want` names (by local or
/// qualified name), any of its types and their ancestors considered.
fn is_a(el: &Element, want: &str) -> bool {
    let reg = type_registry();
    let hit = |c: &ClassDef| c.local.eq_ignore_ascii_case(want) || c.qualified.eq_ignore_ascii_case(want);
    el.types().any(|t| hit(t) || reg.chain(t).iter().any(|c| hit(c)))
}

/// Whether field `key` is the one `want` names: the same key, or the same
/// attribute name after the class.
fn key_matches(key: &str, want: &str) -> bool {
    let attr = |k: &str| k.rsplit('.').next().unwrap_or(k).to_string();
    key.eq_ignore_ascii_case(want) || (!want.contains('.') && attr(key).eq_ignore_ascii_case(want))
}

/// The value of a reference or enumeration as compared: after the last `#`.
fn after_hash(v: &str) -> &str {
    v.rsplit('#').next().unwrap_or(v)
}

/// `s`, cut to `max` characters.
fn clip(s: &str, max: usize) -> String {
    match s.char_indices().nth(max) {
        Some((at, _)) => format!("{}… ({} characters)", &s[..at], s.chars().count()),
        None => s.to_string(),
    }
}

fn values(v: &FieldValue) -> Vec<&str> {
    match v {
        FieldValue::Text(s) | FieldValue::Resource(s) => vec![s.as_str()],
        FieldValue::TextList(l) | FieldValue::ResourceList(l) => l.iter().map(String::as_str).collect(),
    }
}

fn value_matches(v: &FieldValue, want: &str) -> bool {
    let want = after_hash(want.trim());
    let want_bare = want.strip_prefix('_').unwrap_or(want);
    values(v).into_iter().any(|x| {
        let x = after_hash(x.trim());
        x.eq_ignore_ascii_case(want) || x.strip_prefix('_').unwrap_or(x).eq_ignore_ascii_case(want_bare)
    })
}

/// Profile codes a file's header declares, read from its first bytes.
fn header_profiles(file: &Path) -> Vec<&'static str> {
    let Some(head) = session::head(file, 64 * 1024) else { return Vec::new() };
    let mut iris: Vec<&str> = Vec::new();
    // CGMES: <md:Model.profile>IRI</md:Model.profile>
    for part in head.split("Model.profile>").skip(1).step_by(2) {
        iris.push(part.split('<').next().unwrap_or("").trim());
    }
    // NC: <dcterms:conformsTo rdf:resource="IRI"/>
    for part in head.split("conformsTo").skip(1) {
        if let Some(rest) = part.split_once("resource=\"").map(|(_, r)| r) {
            iris.push(rest.split('"').next().unwrap_or(""));
        } else if let Some(rest) = part.strip_prefix('>') {
            iris.push(rest.split('<').next().unwrap_or("").trim());
        }
    }
    let mut codes: Vec<&'static str> = Vec::new();
    for iri in iris.into_iter().filter(|i| !i.is_empty()) {
        let code = [cimvalidation::cgmes_profile_index(), cimvalidation::nc_profile_index()]
            .iter()
            .find_map(|(index, _)| index.iter().find(|(i, _)| *i == iri).map(|(_, c)| *c));
        if let Some(code) = code
            && !codes.contains(&code)
        {
            codes.push(code);
        }
    }
    codes
}

fn file_line(display: &str, file: &Path) -> String {
    let profiles = header_profiles(file);
    let size = file.metadata().map(|m| m.len()).unwrap_or(0);
    let size = if size >= 1 << 20 { format!("{:.1} MB", size as f64 / (1 << 20) as f64) } else { format!("{} kB", size.div_ceil(1024)) };
    if profiles.is_empty() { format!("{display} ({size})") } else { format!("{display} [{}] ({size})", profiles.join(", ")) }
}

fn more(out: &mut String, shown: usize, total: usize, hint: &str) {
    if total > shown {
        let _ = writeln!(out, "… {} more not shown{hint}.", total - shown);
    }
}

// --- Tools ------------------------------------------------------------------

fn list_model_sets(session: &mut Session, args: &Value) -> Result<String, String> {
    let root = session.resolve(str_arg(args, "root"));
    if !root.is_dir() {
        return Err(format!("{} is not a directory", root.display()));
    }
    let depth = args.get("depth").and_then(Value::as_u64).map_or(6, |d| d as usize);
    let sets = session::find_sets(&root, depth);
    if sets.is_empty() {
        return Ok(format!("No CGMES or NC RDF/XML files at or below {} (depth {depth}).", session.relative(&root)));
    }
    const MAX: usize = 100;
    let mut out = format!("{} model set(s) below {}:\n", sets.len(), session.relative(&root));
    for dir in sets.iter().take(MAX) {
        let _ = writeln!(out, "\n{}/", session.relative(dir));
        for file in session::cim_files(dir) {
            let name = file.file_name().map(|n| n.to_string_lossy().into_owned()).unwrap_or_default();
            let _ = writeln!(out, "  {}", file_line(&name, &file));
        }
    }
    more(&mut out, MAX.min(sets.len()), sets.len(), "; narrow `root`");
    Ok(out)
}

fn summary(session: &mut Session, args: &Value) -> Result<String, String> {
    let set = model_set(session, args)?;
    let ds = &set.dataset;
    let mut out = String::new();
    let _ = writeln!(out, "Model set {} — {} file(s), {} objects", set.name, set.files.len(), ds.entries.len());
    for file in &set.files {
        let _ = writeln!(out, "  {}", file_line(&set.display(file), file));
    }

    let headers: Vec<&Element> = ["FullModel", "DifferenceModel", "nc:Dataset", "nc:DifferenceSet"]
        .iter()
        .flat_map(|t| ds.by_type.get(*t).into_iter().flatten())
        .filter_map(|m| ds.entries.get(m))
        .collect();
    if !headers.is_empty() {
        let _ = writeln!(out, "\nModel headers:");
        for h in headers {
            let _ = writeln!(out, "  {} {}", h.type_name(), h.mrid());
            let mut fields: Vec<(&&str, &FieldValue)> = h.fields().iter().collect();
            fields.sort_by_key(|(k, _)| **k);
            for (k, v) in fields {
                let _ = writeln!(out, "    {k} = {}", clip(&values(v).join(", "), 160));
            }
        }
    }

    let mut counts: Vec<(&str, usize)> = ds.by_type.iter().map(|(t, m)| (t.as_str(), m.len())).collect();
    counts.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(b.0)));
    let _ = writeln!(out, "\nObjects by class ({} classes):", counts.len());
    for (t, n) in counts {
        let _ = writeln!(out, "  {n:>7}  {t}");
    }
    Ok(out)
}

fn find_objects(session: &mut Session, args: &Value) -> Result<String, String> {
    let class = str_arg(args, "class");
    let text = str_arg(args, "text").map(str::to_lowercase);
    let attribute = str_arg(args, "attribute");
    let value = str_arg(args, "value");
    let limit = usize_arg(args, "limit", 50);
    if value.is_some() && attribute.is_none() {
        return Err("`value` needs `attribute`".into());
    }
    if let Some(c) = class {
        let reg = type_registry();
        if reg.by_type_name(c).is_none() && reg.by_type_name(&format!("nc:{c}")).is_none() && !known_local(c) {
            return Err(format!("unknown class `{c}`{}", suggest(c)));
        }
    }
    let set = model_set(session, args)?;
    let ds = &set.dataset;

    let mut hits: Vec<(&Element, Option<String>)> = ds
        .entries
        .values()
        .filter(|el| class.is_none_or(|c| is_a(el, c)))
        .filter(|el| {
            text.as_deref().is_none_or(|t| {
                el.mrid().to_lowercase().contains(t)
                    || ["IdentifiedObject.name", "IdentifiedObject.description", "IdentifiedObject.shortName"]
                        .iter()
                        .any(|k| el.get_str(k).is_some_and(|s| s.to_lowercase().contains(t)))
            })
        })
        .filter_map(|el| match attribute {
            None => Some((el, None)),
            Some(a) => el
                .fields()
                .iter()
                .find(|(k, v)| key_matches(k, a) && value.is_none_or(|want| value_matches(v, want)))
                .map(|(k, v)| (el, Some(format!("{k} = {}", values(v).join(", "))))),
        })
        .collect();
    hits.sort_by(|a, b| (a.0.type_name(), name_of(a.0), a.0.mrid()).cmp(&(b.0.type_name(), name_of(b.0), b.0.mrid())));

    if hits.is_empty() {
        return Ok("No matching objects.".into());
    }
    let mut out = format!("{} matching object(s):\n", hits.len());
    for (el, field) in hits.iter().take(limit) {
        let _ = write!(out, "{}", label(el));
        if let Some(f) = field {
            let _ = write!(out, "  [{f}]");
        }
        out.push('\n');
    }
    more(&mut out, limit.min(hits.len()), hits.len(), "; raise `limit` or narrow the search");
    Ok(out)
}

fn get_object(session: &mut Session, args: &Value) -> Result<String, String> {
    let mrid = str_arg(args, "mrid").ok_or("`mrid` is required")?;
    let limit = usize_arg(args, "limit", 100);
    let set = model_set(session, args)?;
    let names: Vec<String> = set.files.iter().map(|f| set.display(f)).collect();
    let (ds, origins) = set.origins()?;
    let el = lookup(ds, mrid).ok_or_else(|| format!("no object `{mrid}` in this model set; find_objects searches by name"))?;
    let reg = type_registry();

    let mut out = format!("{}\n", label(el));
    let chain: Vec<&str> = reg.chain(el.class()).iter().map(|c| c.local).collect();
    if chain.len() > 1 {
        let _ = writeln!(out, "class chain: {}", chain.join(" → "));
    }
    let others: Vec<&str> = el.types().skip(1).map(|c| c.qualified).collect();
    if !others.is_empty() {
        let _ = writeln!(out, "also typed: {}", others.join(", "));
    }
    if let Some(written) = origins.get(el.mrid()) {
        let _ = writeln!(out, "\nFiles:");
        for o in written {
            let how = if o.defined { "defines (rdf:ID)" } else { "describes (rdf:about)" };
            let _ = writeln!(out, "  {} — {how} as {}", names[o.file], o.class);
        }
    }

    let _ = writeln!(out, "\nFields:");
    let mut fields: Vec<(&&str, &FieldValue)> = el.fields().iter().collect();
    fields.sort_by_key(|(k, _)| **k);
    for (k, v) in fields {
        let dup = if el.duplicate_fields().contains(*k) { "  (written more than once)" } else { "" };
        let shown: Vec<String> = match v {
            FieldValue::Resource(_) | FieldValue::ResourceList(_) => values(v)
                .into_iter()
                .map(|r| {
                    let is_enum = reg.attr_of(el.class(), k).is_some_and(|a| a.kind == AttrKind::Enum);
                    match lookup(ds, r) {
                        _ if is_enum => match reg.enum_value(el.class(), after_hash(r)) {
                            Some((_, v)) if !v.comment.is_empty() => format!("{} ({})", v.id, clip(v.comment, 120)),
                            _ => after_hash(r).to_string(),
                        },
                        Some(t) => format!("→ {}", label(t)),
                        None => format!("→ {r} (not in this model set)"),
                    }
                })
                .collect(),
            _ => values(v).into_iter().map(|t| clip(t, 500)).collect(),
        };
        let _ = writeln!(out, "  {k} = {}{dup}", shown.join("; "));
    }

    // Objects referencing this one, by any field.
    let id = el.mrid();
    let mut incoming: Vec<(&Element, &str)> = ds
        .entries
        .values()
        .flat_map(|other| {
            other.fields().iter().filter_map(move |(k, v)| match v {
                FieldValue::Resource(_) | FieldValue::ResourceList(_)
                    if values(v).iter().any(|r| r.trim_start_matches('#') == id) =>
                {
                    Some((other, *k))
                }
                _ => None,
            })
        })
        .collect();
    incoming.sort_by(|a, b| (a.1, a.0.type_name(), a.0.mrid()).cmp(&(b.1, b.0.type_name(), b.0.mrid())));
    if !incoming.is_empty() {
        let _ = writeln!(out, "\nReferenced by ({}):", incoming.len());
        for (other, k) in incoming.iter().take(limit) {
            let _ = writeln!(out, "  {} via {k}", label(other));
        }
        more(&mut out, limit.min(incoming.len()), incoming.len(), "; raise `limit`");
    }
    Ok(out)
}

fn conflicts(session: &mut Session, args: &Value) -> Result<String, String> {
    let limit = usize_arg(args, "limit", 50);
    let set = model_set(session, args)?;
    let names: Vec<String> = set.files.iter().map(|f| set.display(f)).collect();
    let (ds, origins) = set.origins()?;

    let mut twice: Vec<(&Element, Vec<&str>)> = origins
        .iter()
        .filter_map(|(mrid, written)| {
            let by: Vec<&str> = written.iter().filter(|o| o.defined).map(|o| names[o.file].as_str()).collect();
            (by.len() > 1).then(|| ds.entries.get(mrid).map(|el| (el, by))).flatten()
        })
        .collect();
    twice.sort_by(|a, b| (a.0.type_name(), a.0.mrid()).cmp(&(b.0.type_name(), b.0.mrid())));
    let typed = ds.type_conflicts();

    if twice.is_empty() && typed.is_empty() {
        return Ok(format!(
            "No conflicts across {} file(s): no object is defined by more than one file, none is typed with unrelated classes.",
            names.len()
        ));
    }
    let mut out = String::new();
    if !twice.is_empty() {
        let _ = writeln!(out, "{} object(s) defined by more than one file:", twice.len());
        for (el, by) in twice.iter().take(limit) {
            let _ = writeln!(out, "  {} — {}", label(el), by.join(", "));
        }
        more(&mut out, limit.min(twice.len()), twice.len(), "; raise `limit`");
    }
    if !typed.is_empty() {
        if !out.is_empty() {
            out.push('\n');
        }
        let _ = writeln!(out, "{} object(s) typed with unrelated classes:", typed.len());
    }
    for (mrid, _) in typed.iter().take(limit) {
        let mut by_class: Vec<(&str, Vec<&str>)> = Vec::new();
        for o in origins.get(*mrid).into_iter().flatten() {
            match by_class.iter_mut().find(|(c, _)| *c == o.class) {
                Some((_, files)) => files.push(&names[o.file]),
                None => by_class.push((o.class, vec![&names[o.file]])),
            }
        }
        let shown: Vec<String> = by_class.iter().map(|(c, files)| format!("{c} in {}", files.join(", "))).collect();
        let _ = writeln!(out, "  {mrid} — {}", shown.join("; "));
    }
    more(&mut out, limit.min(typed.len()), typed.len(), "; raise `limit`");
    Ok(out)
}

fn validate(session: &mut Session, args: &Value) -> Result<String, String> {
    let opts = Options { common: bool_arg(args, "common"), quality: bool_arg(args, "quality"), silence: list_arg(args, "silence") };
    let rule = str_arg(args, "rule").map(str::to_lowercase);
    let object = str_arg(args, "object");
    let class = str_arg(args, "class");
    let severity = str_arg(args, "severity").map(|s| format!("sh:{}", s.trim_start_matches("sh:")));
    let limit = usize_arg(args, "limit", 50);

    let set = model_set(session, args)?;
    // An object filter matches however the mRID is written.
    let object = match object {
        Some(o) => Some(lookup(&set.dataset, o).map_or(o.to_string(), |e| e.mrid().to_string())),
        None => None,
    };
    let (ds, all) = set.findings(&opts)?;
    let total = all.len();
    let found: Vec<&cimvalidation::Violation> = all
        .iter()
        .filter(|v| rule.as_deref().is_none_or(|r| v.rule_id.to_lowercase().contains(r)))
        .filter(|v| object.as_deref().is_none_or(|o| v.object_id == o))
        .filter(|v| severity.as_deref().is_none_or(|s| v.severity.eq_ignore_ascii_case(s)))
        .filter(|v| class.is_none_or(|c| v.class.eq_ignore_ascii_case(c) || v.class.rsplit(':').next() == Some(c)))
        .collect();
    let filtered = found.len() != total;

    let mut out = String::new();
    let count = |sev: &str| found.iter().filter(|v| v.severity == sev).count();
    let _ = writeln!(
        out,
        "{} finding(s){}: {} sh:Violation, {} sh:Warning, {} sh:Info{}",
        found.len(),
        if filtered { format!(" of {total} matching the filters") } else { String::new() },
        count("sh:Violation"),
        count("sh:Warning"),
        count("sh:Info"),
        if opts.common || opts.quality { "" } else { " (common and quality checks off)" },
    );
    if found.is_empty() {
        return Ok(out);
    }

    // By rule, most frequent first.
    let mut rules: Vec<(&str, &str, usize, &str)> = Vec::new();
    for v in &found {
        match rules.iter_mut().find(|r| r.0 == v.rule_id) {
            Some(r) => r.2 += 1,
            None => rules.push((&v.rule_id, &v.severity, 1, if v.description.is_empty() { &v.message } else { &v.description })),
        }
    }
    rules.sort_by(|a, b| b.2.cmp(&a.2).then(a.0.cmp(b.0)));
    let _ = writeln!(out, "\nBy rule:");
    for (id, sev, n, about) in &rules {
        let about = about.lines().next().unwrap_or("");
        let _ = writeln!(out, "  {n:>6} × [{sev}] {id} — {about}");
    }

    let _ = writeln!(out, "\nFindings:");
    for v in found.iter().take(limit) {
        let name = lookup(ds, &v.object_id).and_then(name_of);
        let mut what = format!("{} {}", v.class, v.object_id);
        if let Some(n) = name {
            let _ = write!(what, " \"{n}\"");
        }
        if !v.property.is_empty() {
            let _ = write!(what, " .{}", v.property);
        }
        let _ = writeln!(out, "  [{}] {} — {} — {what}", v.severity, v.rule_id, v.message);
    }
    more(&mut out, limit.min(found.len()), found.len(), "; filter by `rule`, `class` or `object`, or raise `limit`");
    Ok(out)
}

/// Most characters `export` and `edit` return in place of writing a file.
const EXPORT_INLINE: usize = 100_000;

fn export(session: &mut Session, args: &Value) -> Result<String, String> {
    let profile = str_arg(args, "profile");
    let json = json_arg(args, profile)?;
    let output = output_arg(session, args)?;
    let set = model_set(session, args)?;
    let xml = encode(&set.dataset, profile, json)?;
    let what = match profile {
        Some(code) => format!("profile {code} of {}", set.name),
        None if json => format!("all of {} as JSON", set.name),
        None => format!("all of {}", set.name),
    };
    let joins = joins_set(set, output.as_deref())?;
    deliver(session, &xml, output, joins, &what)
}

/// The edited set as a change list leaves it: each change's line, and the
/// dataset.
fn edit(session: &mut Session, args: &Value) -> Result<String, String> {
    let changes = args.get("changes").and_then(Value::as_array).filter(|c| !c.is_empty()).ok_or("`changes` is required: a non-empty array")?;
    let keep_changed = match str_arg(args, "keep") {
        None | Some("all") => false,
        Some("changed") => true,
        Some(other) => return Err(format!("`keep` is `all` or `changed`, not `{other}`")),
    };
    let profile = str_arg(args, "profile");
    let json_out = json_arg(args, profile)?;
    let output = output_arg(session, args)?;
    let set = model_set(session, args)?;
    // The set's own dataset stays as read; the edit works on a copy.
    let paths: Vec<&Path> = set.files.iter().map(PathBuf::as_path).collect();
    let mut ds = CimDataset::decode_files_parallel(&paths).map_err(|e| format!("decoding {}: {e}", set.name))?;
    let joins = joins_set(set, output.as_deref())?;
    let name = set.name.clone();

    let mut changed: Vec<String> = Vec::new();
    let mut report: Vec<String> = Vec::new();
    for (i, change) in changes.iter().enumerate() {
        let at = |e: String| format!("change {}: {e}", i + 1);
        let text = |key: &str| change.get(key).and_then(Value::as_str).map(str::trim).filter(|s| !s.is_empty());
        let id = text("id").ok_or_else(|| at("`id` is required".into()))?;
        let existing = lookup(&ds, id).map(|e| e.mrid().to_string());
        let existed = existing.is_some();

        if change.get("delete").and_then(Value::as_bool) == Some(true) {
            let key = existing.ok_or_else(|| at(format!("no object `{id}` to delete")))?;
            ds.remove(&key);
            changed.retain(|k| *k != key);
            report.push(format!("deleted {key}"));
            continue;
        }
        let (key, mut json, source) = match (text("from"), existing) {
            (Some(_), Some(key)) => return Err(at(format!("`{key}` exists; a copy needs a new `id`"))),
            (Some(from), None) => {
                let src = lookup(&ds, from).ok_or_else(|| at(format!("no object `{from}` to copy")))?;
                (id.to_string(), element_json(src), Some(src.mrid().to_string()))
            }
            (None, Some(key)) => {
                let json = element_json(&ds.entries[&key]);
                (key, json, None)
            }
            (None, None) if text("type").is_some() => (id.to_string(), json!({}), None),
            (None, None) => return Err(at(format!("no object `{id}`; give `type` to create it or `from` to copy one"))),
        };
        let obj = json.as_object_mut().expect("an element's JSON is an object");
        if let Some(t) = text("type") {
            obj.insert("_type".into(), t.into());
        }
        obj.insert("id".into(), key.as_str().into());
        let fields = match change.get("fields") {
            None | Some(Value::Null) => serde_json::Map::new(),
            Some(Value::Object(f)) => f.clone(),
            Some(_) => return Err(at("`fields` must be an object".into())),
        };
        // A copy's own identifier written as text (`IdentifiedObject.mRID`, a
        // DCAT header's `identifier`) follows the new id, unless `fields` sets it.
        let mut followed: Vec<String> = Vec::new();
        if let Some(src) = &source {
            let (old, new) = (bare_id(src), bare_id(&key));
            for (k, v) in obj.iter_mut() {
                if k != "id" && !fields.contains_key(k) && v.as_str() == Some(old) {
                    *v = new.into();
                    followed.push(k.clone());
                }
            }
        }
        for (k, v) in &fields {
            match v {
                Value::Null => {
                    obj.remove(k);
                }
                Value::String(_) | Value::Array(_) => {
                    obj.insert(k.clone(), v.clone());
                }
                other => return Err(at(format!("{k}: a string, an array of strings or null, not {other}"))),
            }
        }
        let type_name = obj.get("_type").and_then(Value::as_str).unwrap_or_default().to_string();
        let class = type_registry().by_type_name(&type_name).ok_or_else(|| at(format!("unknown class `{type_name}`{}", suggest(&type_name))))?;
        let element = ds.element_from_json(class, &json).map_err(at)?;
        ds.set(key.clone(), element);
        if !changed.contains(&key) {
            changed.push(key.clone());
        }
        let keys: Vec<&str> = fields.keys().map(String::as_str).collect();
        let mut line = match &source {
            Some(src) => format!("copied {src} to {key} ({type_name})"),
            None if existed => format!("updated {key} ({type_name})"),
            None => format!("created {key} ({type_name})"),
        };
        if !keys.is_empty() {
            let _ = write!(line, ": {}", keys.join(", "));
        }
        if !followed.is_empty() {
            let _ = write!(line, "; {} follow{} the new id", followed.join(", "), if followed.len() == 1 { "s" } else { "" });
        }
        report.push(line);
    }
    if keep_changed {
        let rest: Vec<String> = ds.entries.keys().filter(|k| !changed.contains(k)).cloned().collect();
        for key in &rest {
            ds.remove(key);
        }
        ds.foreign.clear();
        report.push(format!("kept the {} changed object(s), left out the other {}", changed.len(), rest.len()));
    }

    let xml = encode(&ds, profile, json_out)?;
    let what = match profile {
        Some(code) => format!("profile {code} of {name}, edited"),
        None if json_out => format!("all of {name}, edited, as JSON"),
        None => format!("all of {name}, edited"),
    };
    let delivered = deliver(session, &xml, output, joins, &what)?;
    Ok(format!("{}\n\n{delivered}", report.join("\n")))
}

/// An element as `edit` changes it: its JSON form with its `_type`.
fn element_json(el: &Element) -> Value {
    let mut json = el.to_json_value();
    json["_type"] = el.type_name().into();
    json
}

/// An id without `urn:uuid:` or a leading `_`: how `IdentifiedObject.mRID`
/// writes it.
fn bare_id(id: &str) -> &str {
    let id = id.trim_start_matches('#');
    let id = id.strip_prefix("urn:uuid:").unwrap_or(id);
    id.strip_prefix('_').unwrap_or(id)
}

/// `output`, checked before any work: inside the workspace, and not over a
/// file unless `overwrite`.
fn output_arg(session: &Session, args: &Value) -> Result<Option<PathBuf>, String> {
    let Some(o) = str_arg(args, "output") else { return Ok(None) };
    let out = output_path(&session.base, o)?;
    if out.exists() && !bool_arg(args, "overwrite") {
        return Err(format!("{} exists; set `overwrite` to replace it", session.relative(&out)));
    }
    Ok(Some(out))
}

/// Whether `format` asks for JSON; JSON has no profiles.
fn json_arg(args: &Value, profile: Option<&str>) -> Result<bool, String> {
    match str_arg(args, "format") {
        None | Some("rdfxml") => Ok(false),
        Some("json") if profile.is_some() => Err("`format: \"json\"` writes the whole dataset and takes no `profile`".into()),
        Some("json") => Ok(true),
        Some(other) => Err(format!("`format` is `rdfxml` or `json`, not `{other}`")),
    }
}

/// The dataset as RDF/XML (one profile, or all of it) or as JSON: every CIM
/// and foreign object, in the form cimoxide-py's `entries()` and `foreign()`
/// give.
fn encode(ds: &CimDataset, profile: Option<&str>, json: bool) -> Result<String, String> {
    if json {
        let all: serde_json::Map<String, Value> =
            ds.entries.iter().chain(&ds.foreign).map(|(id, el)| (id.clone(), element_json(el))).collect();
        return serde_json::to_string_pretty(&all).map_err(|e| e.to_string());
    }
    match profile {
        Some(code) => cimmodel::convert::dataset_to_xml_for_profile(ds, code),
        None => cimmodel::convert::dataset_to_xml(ds),
    }
    .map_err(|e| e.to_string())
}

/// Whether `output` lies in a directory the set read files from, and so
/// joins it; an error if it is one of the set's files.
fn joins_set(set: &ModelSet, output: Option<&Path>) -> Result<bool, String> {
    let Some(out) = output else { return Ok(false) };
    if set.files.iter().any(|f| f == out) {
        return Err(format!("`output` is {}, a file of the model set; its files are never written", set.display(out)));
    }
    Ok(set.files.iter().any(|f| f.parent() == out.parent()))
}

/// `xml` written to `output`, or returned when there is none and it is short.
fn deliver(session: &Session, xml: &str, output: Option<PathBuf>, joins: bool, what: &str) -> Result<String, String> {
    let Some(out) = output else {
        let n = xml.chars().count();
        if n > EXPORT_INLINE {
            return Err(format!("{what} is {n} characters; give `output` to write it to a file"));
        }
        return Ok(xml.to_string());
    };
    if let Some(dir) = out.parent() {
        std::fs::create_dir_all(dir).map_err(|e| format!("creating {}: {e}", dir.display()))?;
    }
    std::fs::write(&out, xml).map_err(|e| format!("writing {}: {e}", out.display()))?;
    let mut msg = format!("Wrote {} ({} bytes): {what}.", session.relative(&out), xml.len());
    if joins {
        msg.push_str(" Its directory holds the model set's files, so the set now includes it.");
    }
    Ok(msg)
}

/// `output` against the workspace, which it may not leave.
fn output_path(base: &Path, output: &str) -> Result<PathBuf, String> {
    let p = Path::new(output);
    if p.components().any(|c| c == Component::ParentDir) {
        return Err("`output` may not contain `..`".into());
    }
    let full = if p.is_absolute() { p.to_path_buf() } else { base.join(p) };
    if !full.starts_with(base) || full == base {
        return Err(format!("`output` must be a file inside the workspace {}", base.display()));
    }
    Ok(full)
}

fn sparql(session: &mut Session, args: &Value) -> Result<String, String> {
    let query = str_arg(args, "query").ok_or("`query` is required")?;
    let limit = usize_arg(args, "limit", 100);
    let set = model_set(session, args)?;
    let results = set.store()?.query(query).map_err(|e| format!("query failed: {e}"))?;
    let ds = &set.dataset;
    let prefixes = cimsparql::prefixes();
    let term = |t: &Term| -> String {
        match t {
            Term::NamedNode(n) => {
                let iri = n.as_str();
                if let Some(m) = cimsparql::iri::iri_to_mrid(iri, ds) {
                    return m;
                }
                prefixes
                    .iter()
                    .filter(|(_, ns)| iri.starts_with(ns))
                    .max_by_key(|(_, ns)| ns.len())
                    .map_or_else(|| format!("<{iri}>"), |(p, ns)| format!("{p}:{}", &iri[ns.len()..]))
            }
            Term::Literal(l) => l.value().to_string(),
            other => other.to_string(),
        }
    };

    let mut out = String::new();
    match results {
        QueryResults::Boolean(b) => out.push_str(if b { "true" } else { "false" }),
        QueryResults::Solutions(solutions) => {
            let vars: Vec<String> = solutions.variables().iter().map(|v| v.as_str().to_string()).collect();
            let _ = writeln!(out, "{}", vars.join("\t"));
            let mut rows = 0usize;
            for solution in solutions {
                let solution = solution.map_err(|e| format!("query failed: {e}"))?;
                rows += 1;
                if rows <= limit {
                    let cells: Vec<String> = vars.iter().map(|v| solution.get(v.as_str()).map(&term).unwrap_or_default()).collect();
                    let _ = writeln!(out, "{}", cells.join("\t"));
                }
            }
            let _ = writeln!(out, "({rows} row(s))");
            more(&mut out, limit.min(rows), rows, "; raise `limit` or aggregate in the query");
        }
        QueryResults::Graph(triples) => {
            let mut n = 0usize;
            for triple in triples {
                let triple = triple.map_err(|e| format!("query failed: {e}"))?;
                n += 1;
                if n <= limit {
                    let _ = writeln!(out, "{} {} {} .", term(&triple.subject.into()), term(&triple.predicate.into()), term(&triple.object));
                }
            }
            more(&mut out, limit.min(n), n, "; raise `limit`");
        }
    }
    Ok(out)
}

/// Every class of both families whose local name is `name`, CGMES first.
fn classes_named(name: &str) -> Vec<&'static ClassDef> {
    let reg = type_registry();
    let (family, local) = match name.split_once(':') {
        Some((p, l)) => (Some(if p == "nc" { "nc" } else { "cgmes" }), l),
        None => (None, name),
    };
    ["cgmes", "nc"]
        .into_iter()
        .filter(|f| family.is_none_or(|w| w == *f))
        .filter_map(|f| reg.schema(f))
        .flat_map(|s| s.classes.iter())
        .filter(|c| c.local.eq_ignore_ascii_case(local))
        .collect()
}

fn known_local(name: &str) -> bool {
    !classes_named(name).is_empty()
}

/// Up to a dozen class names containing `name`, as an error's hint.
fn suggest(name: &str) -> String {
    let reg = type_registry();
    let want = name.rsplit(':').next().unwrap_or(name).to_lowercase();
    let mut close: Vec<&str> = ["cgmes", "nc"]
        .into_iter()
        .filter_map(|f| reg.schema(f))
        .flat_map(|s| s.classes.iter())
        .filter(|c| c.local.to_lowercase().contains(&want))
        .map(|c| c.qualified)
        .collect();
    close.sort_unstable();
    close.dedup();
    if close.is_empty() {
        String::new()
    } else {
        close.truncate(12);
        format!("; did you mean {}?", close.join(", "))
    }
}

/// `Literal type (xsd:…)`, `→ Class` or `enum Type`, with multiplicity.
fn attr_type(a: &cimmodel::base::AttrDef) -> String {
    let kind = match a.kind {
        AttrKind::Literal if a.xsd.is_empty() => a.range.to_string(),
        AttrKind::Literal => format!("{} (xsd:{})", a.range, a.xsd),
        AttrKind::Association => format!("→ {}", a.range),
        AttrKind::Enum => format!("enum {}", a.range),
    };
    let many = if a.is_list { " [0..*]" } else { "" };
    let unused = if a.used { "" } else { " (not exchanged)" };
    format!("{kind}{many}{unused}")
}

/// One attribute, `Class.attr`, of every family declaring it.
fn describe_attribute(name: &str) -> Result<String, String> {
    let (class_name, attr) = name.rsplit_once('.').expect("caller checked");
    let classes = classes_named(class_name);
    if classes.is_empty() {
        return Err(format!("unknown class `{class_name}`{}", suggest(class_name)));
    }
    let reg = type_registry();
    let mut out = String::new();
    let mut missing = Vec::new();
    for class in classes {
        let family = reg.family_of(class.qualified).unwrap_or("?").to_uppercase();
        // The attribute as written, else by its name after the class, inherited ones included.
        let id = format!("{}.{attr}", class.local);
        let Some(a) = reg.attr(class, &id).or_else(|| reg.attrs(class).find(|a| key_matches(a.id, attr))) else {
            missing.push(class.qualified);
            continue;
        };
        let _ = writeln!(out, "{} ({family}): {}", a.id, attr_type(a));
        if !a.origins.is_empty() {
            let _ = writeln!(out, "  profiles: {}", a.origins.join(", "));
        }
        let _ = writeln!(out, "  {}\n", if a.comment.is_empty() { "(no definition in the vocabulary)" } else { a.comment });
    }
    if out.is_empty() {
        return Err(format!("{} has no attribute `{attr}`; describe_class `{class_name}` lists them", missing.join(" and ")));
    }
    Ok(out.trim_end().to_string() + "\n")
}

/// The enumeration `name` names, or the enumeration of the value it names,
/// in each family that has it; `None` if no family has it.
fn describe_enum(name: &str) -> Option<String> {
    let reg = type_registry();
    let (family, name) = match name.split_once(':') {
        Some(("nc", n)) => (Some("nc"), n),
        Some((_, n)) => (Some("cgmes"), n),
        None => (None, name),
    };
    let mut out = String::new();
    for f in ["cgmes", "nc"].into_iter().filter(|f| family.is_none_or(|w| w == *f)) {
        let Some(schema) = reg.schema(f) else { continue };
        if let Some((e, v)) = schema.enum_value(name) {
            let _ = writeln!(out, "{} ({}): value of enumeration {}", v.id, f.to_uppercase(), e.local);
            let _ = writeln!(out, "  {}", if v.comment.is_empty() { "(no definition in the vocabulary)" } else { v.comment });
            if !e.comment.is_empty() {
                let _ = writeln!(out, "  {}: {}", e.local, e.comment);
            }
            let _ = writeln!(out, "  written as rdf:resource=\"{}{}\"\n", e.ns, v.id);
        } else if let Some(e) = schema.enumeration(name) {
            let _ = writeln!(out, "{} ({} enumeration) — namespace {}", e.local, f.to_uppercase(), e.ns);
            if !e.comment.is_empty() {
                let _ = writeln!(out, "  {}", e.comment);
            }
            let _ = writeln!(out, "  values ({}):", e.values.len());
            for v in e.values {
                let _ = writeln!(out, "    {}{}", v.id, if v.comment.is_empty() { String::new() } else { format!(" — {}", v.comment) });
            }
            out.push('\n');
        }
    }
    (!out.is_empty()).then(|| out.trim_end().to_string() + "\n")
}

fn describe_class(args: &Value) -> Result<String, String> {
    let name = str_arg(args, "class").ok_or("`class` is required")?;
    if let Some(text) = describe_enum(name) {
        return Ok(text);
    }
    if name.contains('.') {
        return describe_attribute(name);
    }
    let include_unused = bool_arg(args, "include_unused");
    let found = classes_named(name);
    if found.is_empty() {
        return Err(format!("unknown class `{name}`{}", suggest(name)));
    }
    let reg = type_registry();
    let mut out = String::new();
    for class in found {
        let family = reg.family_of(class.qualified).unwrap_or("?");
        let _ = writeln!(
            out,
            "{} ({}{}) — namespace {}",
            class.qualified,
            family.to_uppercase(),
            if class.concrete { "" } else { ", abstract" },
            class.ns
        );
        if !class.comment.is_empty() {
            let _ = writeln!(out, "  {}", class.comment);
        }
        let chain = reg.chain(class);
        let _ = writeln!(out, "  inherits: {}", chain.iter().map(|c| c.local).collect::<Vec<_>>().join(" → "));
        if !class.origins.is_empty() {
            let _ = writeln!(out, "  profiles: {}", class.origins.join(", "));
        }
        let below: Vec<&str> = reg.concrete_descendants(class.qualified).into_iter().filter(|q| *q != class.qualified).collect();
        if !below.is_empty() {
            let shown: Vec<&str> = below.iter().take(40).copied().collect();
            let _ = writeln!(out, "  concrete subclasses ({}): {}{}", below.len(), shown.join(", "), if below.len() > 40 { ", …" } else { "" });
        }
        let mut hidden = 0usize;
        for c in chain {
            let attrs: Vec<_> = c.attrs.iter().filter(|a| include_unused || a.used).collect();
            hidden += c.attrs.len() - attrs.len();
            if attrs.is_empty() {
                continue;
            }
            let _ = writeln!(out, "  from {}:", c.local);
            for a in attrs {
                let profiles = if a.origins.is_empty() { String::new() } else { format!("  {{{}}}", a.origins.join(", ")) };
                let _ = writeln!(out, "    {}: {}{profiles}", a.id, attr_type(a));
                // A long definition is cut here; the attribute alone gives it whole.
                if !a.comment.is_empty() {
                    let _ = writeln!(out, "      {}", clip(a.comment, 200));
                }
            }
        }
        if hidden > 0 {
            let _ = writeln!(out, "  ({hidden} association end(s) no profile exchanges not shown; include_unused lists them)");
        }
        out.push('\n');
    }
    Ok(out.trim_end().to_string() + "\n")
}
