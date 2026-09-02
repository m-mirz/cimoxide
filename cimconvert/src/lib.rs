use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fmt::Write as FmtWrite;
use std::sync::OnceLock;

use cimdecoder::CimDataset;
use cimstructs::base::{FieldValue, RdfBlock};
use cimstructs::constants::CIM_NAMESPACES;
use cimstructs::profile_meta::{ATTR_ORIGINS, ATTR_RDF, PROFILE_URIS, TYPE_NS, TYPE_ORIGINS};
use cimstructs::registry::json_registry;

/// Prefix for a class or attribute absent from the generated tables — a third-party
/// extension, or a CIM version skew between the data and the schema it was generated from.
const FALLBACK_PREFIX: &str = "cim";

/// RDF metadata for one attribute, resolved to the prefix this writer emits.
#[derive(Clone, Copy)]
struct AttrMeta {
    prefix: &'static str,
    /// The enum's namespace IRI when `is_enum`; unused otherwise.
    range: &'static str,
    is_enum: bool,
}

/// The generated lookup tables, indexed once.
///
/// These were previously rebuilt from the raw slices on every call — ~450 type rows and
/// ~3600 attribute rows per profile, so `write_xml_files(["EQ", "SSH", ...])` paid for it
/// once per profile.
struct Tables {
    type_origins: HashMap<&'static str, &'static [&'static str]>,
    attr_origins: HashMap<&'static str, &'static [&'static str]>,
    type_prefix: HashMap<&'static str, &'static str>,
    type_ns: HashMap<&'static str, &'static str>,
    attrs: HashMap<&'static str, AttrMeta>,
    /// Profiles that are the dominant origin of no attribute at all — see
    /// [`dataset_to_xml_for_profile`].
    self_defining: HashSet<&'static str>,
}

fn tables() -> &'static Tables {
    static T: OnceLock<Tables> = OnceLock::new();
    T.get_or_init(|| {
        // The decoder throws namespaces away, so `TYPE_NS`/`ATTR_RDF` are the only runtime
        // source of them. Both tables draw on exactly the namespaces `CIM_NAMESPACES`
        // declares, so this reverse map is total over them.
        let prefix_of: HashMap<&str, &str> =
            CIM_NAMESPACES.iter().map(|(p, ns)| (*ns, *p)).collect();
        let lookup = |ns: &str| prefix_of.get(ns).copied().unwrap_or(FALLBACK_PREFIX);

        Tables {
            type_origins: TYPE_ORIGINS.iter().map(|(k, v)| (*k, *v)).collect(),
            attr_origins: ATTR_ORIGINS.iter().map(|(k, v)| (*k, *v)).collect(),
            type_prefix: TYPE_NS.iter().map(|(t, ns)| (*t, lookup(ns))).collect(),
            type_ns: TYPE_NS.iter().copied().collect(),
            attrs: ATTR_RDF
                .iter()
                .map(|(id, ns, range, kind)| {
                    (*id, AttrMeta { prefix: lookup(ns), range, is_enum: *kind == 2 })
                })
                .collect(),
            self_defining: PROFILE_URIS
                .iter()
                .map(|(code, _)| *code)
                .filter(|code| {
                    !ATTR_ORIGINS.iter().any(|(_, o)| o.first() == Some(code))
                })
                .collect(),
        }
    })
}

/// XML prefix for a CIM class, e.g. `eu` for `BoundaryPoint`.
fn prefix_for_type(type_name: &str) -> &'static str {
    tables().type_prefix.get(type_name).copied().unwrap_or(FALLBACK_PREFIX)
}

/// Namespace IRI of a CIM class, for rebuilding a stripped enum value.
fn type_ns_iri(type_name: &str) -> Option<&'static str> {
    tables().type_ns.get(type_name).copied()
}

/// RDF metadata for an `RdfBlock.fields` key owned by `type_name`.
///
/// Most keys are already `Class.attr` and hit directly. A few arrive bare — the decoder's
/// `local_name()` reduces `<dm:forwardDifferences>` to `forwardDifferences`, while `ATTR_RDF`
/// holds `DifferenceModel.forwardDifferences` — so retry qualified before giving up.
fn attr_meta(key: &str, type_name: &str) -> Option<AttrMeta> {
    let t = tables();
    t.attrs
        .get(key)
        .or_else(|| t.attrs.get(format!("{type_name}.{key}").as_str()))
        .copied()
}

pub fn dataset_to_json(ds: &CimDataset) -> serde_json::Value {
    let mut map = serde_json::Map::new();
    for (mrid, entry) in &ds.entries {
        let mut obj = entry
            .element
            .to_json_value()
            .as_object()
            .cloned()
            .unwrap_or_default();
        obj.insert("_type".into(), entry.element.type_name().into());
        map.insert(mrid.clone(), serde_json::Value::Object(obj));
    }
    serde_json::Value::Object(map)
}

pub fn dataset_from_json(json: &str) -> Result<CimDataset, Box<dyn Error>> {
    let root: serde_json::Map<String, serde_json::Value> = serde_json::from_str(json)?;
    let reg = json_registry();
    let mut ds = CimDataset::new();
    for (mrid, val) in root {
        let type_name = val["_type"].as_str().unwrap_or("").to_string();
        if let Some(f) = reg.get(type_name.as_str()) {
            let element = f(val)?;
            ds.set(mrid, element);
        }
    }
    Ok(ds)
}

pub fn dataset_to_xml(ds: &CimDataset) -> Result<String, Box<dyn Error>> {
    let mut out = String::new();
    out.push_str("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n");
    out.push_str("<rdf:RDF");
    for (prefix, uri) in CIM_NAMESPACES {
        write!(out, " xmlns:{prefix}=\"{uri}\"")?;
    }
    out.push_str(">\n");

    let mut mrids: Vec<&str> = ds.entries.keys().map(String::as_str).collect();
    mrids.sort();

    for mrid in mrids {
        let entry = &ds.entries[mrid];
        let block = &entry.block;
        let type_name = block.type_name.as_str();
        let ns = prefix_for_type(type_name);
        write!(out, "  <{ns}:{type_name} rdf:about=\"#{}\">", escape_attr(mrid))?;

        // Emit fields sorted for deterministic output
        let mut fields: Vec<(&String, &FieldValue)> = block.fields.iter().collect();
        fields.sort_by_key(|(k, _)| k.as_str());

        let mut children = String::new();
        for (key, val) in fields {
            write_field(&mut children, key, val, type_name, "#", ds)?;
        }

        if children.is_empty() {
            write!(out, "/>\n")?;
        } else {
            write!(out, "{children}\n  </{ns}:{type_name}>\n")?;
        }
    }

    out.push_str("</rdf:RDF>\n");
    Ok(out)
}

pub fn dataset_to_xml_for_profile(
    ds: &CimDataset,
    profile_code: &str,
) -> Result<String, Box<dyn Error>> {
    let tables = tables();
    let type_map = &tables.type_origins;
    let attr_map = &tables.attr_origins;

    // A profile that is the dominant origin of no attribute cannot select any content under
    // the secondary-element rule below — every element would empty out and be dropped, and
    // the file would contain nothing but its header. EQBD is the only such profile today: the
    // RDFS declares its classes and attributes identically in the Equipment profile, so EQ
    // always outranks it. Real boundary files define their objects outright (all `rdf:ID`,
    // carrying name and mRID), so fall back to plain profile membership, which is all the
    // schema actually asserts.
    let self_defining = tables.self_defining.contains(profile_code);

    let mut out = String::new();
    out.push_str("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n");
    out.push_str("<rdf:RDF");
    for (prefix, uri) in CIM_NAMESPACES {
        write!(out, " xmlns:{prefix}=\"{uri}\"")?;
    }
    out.push_str(">\n");

    // FullModel header — declares the profile this file belongs to. Reuse a real
    // decoded FullModel entry for this profile if the dataset has one (preserves
    // scenarioTime/modelingAuthoritySet/DependentOn/version/etc.), else fall back
    // to a minimal synthetic header.
    if let Some(&(_, uri)) = PROFILE_URIS.iter().find(|(k, _)| *k == profile_code) {
        // `md` via the same tables the body uses: TYPE_NS maps FullModel and ATTR_RDF maps
        // every Model.* field onto the ModelDescription namespace. The header keeps only its
        // genuinely different behaviour — `rdf:about` with no `#`, and no fragment on
        // resource references.
        let hdr = prefix_for_type("FullModel");
        if let Some((mrid, block)) = find_full_model_header(ds, uri) {
            write!(out, "  <{hdr}:FullModel rdf:about=\"{}\">", escape_attr(mrid))?;

            let mut fields: Vec<(&String, &FieldValue)> = block.fields.iter().collect();
            fields.sort_by_key(|(k, _)| k.as_str());

            let mut children = String::new();
            for (key, val) in fields {
                write_field(&mut children, key, val, "FullModel", "", ds)?;
            }

            if children.is_empty() {
                write!(out, "/>\n")?;
            } else {
                write!(out, "{children}\n  </{hdr}:FullModel>\n")?;
            }
        } else {
            let profile_ns = attr_meta("Model.profile", "FullModel")
                .map_or(FALLBACK_PREFIX, |m| m.prefix);
            writeln!(
                out,
                "  <{hdr}:FullModel rdf:about=\"urn:uuid:cimoxide-{profile_code}\">"
            )?;
            writeln!(
                out,
                "    <{profile_ns}:Model.profile>{uri}</{profile_ns}:Model.profile>"
            )?;
            writeln!(out, "  </{hdr}:FullModel>")?;
        }
    }

    let mut mrids: Vec<&str> = ds.entries.keys().map(String::as_str).collect();
    mrids.sort();

    for mrid in mrids {
        let entry = &ds.entries[mrid];
        let type_name = entry.element.type_name();

        let type_origins: &[&str] = type_map.get(type_name).copied().unwrap_or(&[]);
        if !type_origins.contains(&profile_code) {
            continue;
        }

        let is_primary =
            self_defining || type_origins.first().map_or(false, |&o| o == profile_code);
        let block = &entry.block;

        let include_field = |key: &str| -> bool {
            let origins = attr_map.get(key).copied().unwrap_or(&[]);
            if is_primary {
                origins.contains(&profile_code)
            } else {
                origins.first().map_or(false, |&o| o == profile_code)
            }
        };

        let mut fields: Vec<(&String, &FieldValue)> =
            block.fields.iter().filter(|(k, _)| include_field(k)).collect();

        if fields.is_empty() {
            continue;
        }

        let ns = prefix_for_type(type_name);
        if is_primary {
            write!(out, "  <{ns}:{type_name} rdf:ID=\"{}\">", escape_attr(mrid))?;
        } else {
            write!(out, "  <{ns}:{type_name} rdf:about=\"#{}\">", escape_attr(mrid))?;
        }

        fields.sort_by_key(|(k, _)| k.as_str());
        let mut children = String::new();
        for (key, val) in fields {
            write_field(&mut children, key, val, type_name, "#", ds)?;
        }

        if children.is_empty() {
            write!(out, "/>\n")?;
        } else {
            write!(out, "{children}\n  </{ns}:{type_name}>\n")?;
        }
    }

    out.push_str("</rdf:RDF>\n");
    Ok(out)
}

/// Find the decoded `FullModel` entry (if any) whose `Model.profile` field names
/// `profile_uri`. If more than one matches, the lexicographically smallest MRID
/// wins, for deterministic output.
fn find_full_model_header<'a>(ds: &'a CimDataset, profile_uri: &str) -> Option<(&'a str, &'a RdfBlock)> {
    let mut best: Option<(&str, &RdfBlock)> = None;
    for (mrid, entry) in &ds.entries {
        if entry.element.type_name() != "FullModel" {
            continue;
        }
        let matches = match entry.block.fields.get("Model.profile") {
            Some(FieldValue::Text(s)) => s == profile_uri,
            Some(FieldValue::TextList(list)) => list.iter().any(|s| s == profile_uri),
            _ => false,
        };
        if !matches {
            continue;
        }
        if best.is_none_or(|(m, _)| mrid.as_str() < m) {
            best = Some((mrid.as_str(), &entry.block));
        }
    }
    best
}

/// Write one field, with its own namespace prefix taken from `ATTR_RDF`.
///
/// The prefix is per field, never inherited from the owning class: a single
/// `eu:BoundaryPoint` carries both `eu:BoundaryPoint.toEndName` and
/// `cim:IdentifiedObject.description`.
///
/// `resource_prefix` is `"#"` for ordinary fields (local fragment references) and `""` for
/// FullModel header fields (`Model.DependentOn`/`Model.Supersedes` reference another
/// FullModel's full URN, not a local fragment) — matches how the decoder strips at most one
/// leading `#` from `rdf:resource` values on the way in. Enum values ignore it entirely and
/// are written as absolute IRIs, which is what real CGMES files carry:
/// `rdf:resource="http://iec.ch/TC57/CIM100#DCPolarityKind.positive"`. The decoder's
/// `strip_fragment` reduces either form to the same key, so this round-trips unchanged.
fn write_field(
    children: &mut String,
    key: &str,
    val: &FieldValue,
    type_name: &str,
    resource_prefix: &str,
    ds: &CimDataset,
) -> Result<(), Box<dyn Error>> {
    let meta = attr_meta(key, type_name);
    let ns = meta.map_or(FALLBACK_PREFIX, |m| m.prefix);
    let resource = |r: &str| {
        if let Some(m) = meta
            && m.is_enum
        {
            return format!("{}{}", m.range, escape_attr(r));
        }
        // `eu:LimitKind` and `eu:SVCControlMode` are enumerations generated as marker structs
        // — `cims:stereotype` parsing is last-write-wins and their `European` stereotype
        // overwrites the `enumeration` one — so `ATTR_RDF` marks them as plain references. A
        // value naming no entry but matching a known `Type.value` is that case; rebuild the
        // IRI the decoder stripped, as `cimsparql::iri::reference_iri` does on the way in.
        if !ds.entries.contains_key(r)
            && let Some((owner, _)) = r.split_once('.')
            && let Some(ns_iri) = type_ns_iri(owner)
        {
            return format!("{ns_iri}{}", escape_attr(r));
        }
        format!("{resource_prefix}{}", escape_attr(r))
    };

    match val {
        FieldValue::Text(s) => {
            write!(children, "\n    <{ns}:{key}>{}</{ns}:{key}>", escape_text(s))?;
        }
        FieldValue::TextList(ss) => {
            for s in ss {
                write!(children, "\n    <{ns}:{key}>{}</{ns}:{key}>", escape_text(s))?;
            }
        }
        FieldValue::Resource(r) => {
            write!(children, "\n    <{ns}:{key} rdf:resource=\"{}\"/>", resource(r))?;
        }
        FieldValue::ResourceList(rs) => {
            for r in rs {
                write!(children, "\n    <{ns}:{key} rdf:resource=\"{}\"/>", resource(r))?;
            }
        }
    }
    Ok(())
}

fn escape_text(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}

fn escape_attr(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('"', "&quot;")
}
