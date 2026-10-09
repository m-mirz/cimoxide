use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fmt::Write as FmtWrite;
use std::sync::OnceLock;

use crate::CimDataset;
use crate::base::{Element, FieldValue};
use crate::base::{AttrKind, Schema};
use crate::registry::type_registry;

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

/// The CGMES schema, indexed once for writing: prefixes, profile membership and the
/// namespaces the decoder dropped.
struct Tables {
    schema: &'static Schema,
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
        let schema = type_registry().schema("cgmes").expect("the CGMES family is registered");
        // The decoder throws namespaces away, so the schema is the only runtime source of
        // them; its classes and attributes draw on exactly the namespaces it binds, so this
        // reverse map is total over them.
        let prefix_of: HashMap<&str, &'static str> =
            schema.namespaces.iter().map(|(p, ns)| (*ns, *p)).collect();
        let lookup = |ns: &str| prefix_of.get(ns).copied().unwrap_or(FALLBACK_PREFIX);

        // An attribute's profiles over every class that declares it, the export's dominant
        // profile first: EQ, then the rest alphabetically.
        let mut attr_origins: HashMap<&'static str, Vec<&'static str>> = HashMap::new();
        for a in schema.classes.iter().flat_map(|c| c.attrs).filter(|a| a.used) {
            let merged = attr_origins.entry(a.id).or_default();
            for o in a.origins {
                if !merged.contains(o) {
                    merged.push(o);
                }
            }
        }
        attr_origins.retain(|_, o| !o.is_empty());
        let attr_origins: HashMap<&'static str, &'static [&'static str]> = attr_origins
            .into_iter()
            .map(|(id, mut o)| {
                o.sort_by(|a, b| (*a != "EQ", *a).cmp(&(*b != "EQ", *b)));
                (id, &*Vec::leak(o))
            })
            .collect();

        let mut attrs: HashMap<&'static str, AttrMeta> = HashMap::new();
        for a in schema.classes.iter().flat_map(|c| c.attrs).filter(|a| !a.ns.is_empty()) {
            attrs.entry(a.id).or_insert(AttrMeta {
                prefix: lookup(a.ns),
                range: a.value_ns,
                is_enum: a.kind == AttrKind::Enum,
            });
        }

        Tables {
            schema,
            type_origins: schema
                .classes
                .iter()
                .filter(|c| !c.origins.is_empty())
                .map(|c| (c.local, c.origins))
                .collect(),
            self_defining: schema
                .profiles
                .iter()
                .map(|(code, _)| *code)
                .filter(|code| !attr_origins.values().any(|o| o.first() == Some(code)))
                .collect(),
            attr_origins,
            type_prefix: schema.classes.iter().map(|c| (c.local, lookup(c.ns))).collect(),
            type_ns: schema.classes.iter().filter(|c| !c.ns.is_empty()).map(|c| (c.local, c.ns)).collect(),
            attrs,
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

/// RDF metadata for an `Element::fields` key owned by `type_name`.
///
/// Most keys are already `Class.attr` and hit directly. A few arrive bare — the decoder's
/// `local_name()` reduces `<dm:forwardDifferences>` to `forwardDifferences`, while the schema
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
        let mut obj = entry.to_json_value().as_object().cloned().unwrap_or_default();
        obj.insert("_type".into(), entry.type_name().into());
        map.insert(mrid.clone(), serde_json::Value::Object(obj));
    }
    serde_json::Value::Object(map)
}

pub fn dataset_from_json(json: &str) -> Result<CimDataset, Box<dyn Error>> {
    let root: serde_json::Map<String, serde_json::Value> = serde_json::from_str(json)?;
    let reg = type_registry();
    let mut ds = CimDataset::new();
    for (mrid, val) in root {
        let type_name = val["_type"].as_str().unwrap_or("");
        if let Some(class) = reg.by_type_name(type_name) {
            ds.set(mrid, Element::from_json(class, reg, &val)?);
        }
    }
    Ok(ds)
}

/// True when the element belongs to a profile family this encoder cannot write.
///
/// The writer serves the CGMES schema — its profiles, its prefixes, its
/// `md:FullModel` header. Other families
/// carry a qualified type name (`nc:Contingency`); emitting one as
/// `<cim:nc:Contingency>` would produce a malformed document, so they are
/// skipped and reported instead.
fn is_foreign_family(type_name: &str) -> bool {
    type_name.contains(':')
}

pub fn dataset_to_xml(ds: &CimDataset) -> Result<String, Box<dyn Error>> {
    let mut out = String::new();
    out.push_str("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n");
    out.push_str("<rdf:RDF");
    for (prefix, uri) in tables().schema.namespaces {
        write!(out, " xmlns:{prefix}=\"{uri}\"")?;
    }
    out.push_str(">\n");

    let mut mrids: Vec<&str> = ds.entries.keys().map(String::as_str).collect();
    mrids.sort();

    let mut skipped = 0usize;
    for mrid in mrids {
        let entry = &ds.entries[mrid];
        let block = entry;
        let type_name = block.type_name();
        if is_foreign_family(type_name) {
            skipped += 1;
            continue;
        }
        let ns = prefix_for_type(type_name);
        write!(out, "  <{ns}:{type_name} rdf:about=\"#{}\">", escape_attr(mrid))?;

        // Emit fields sorted for deterministic output
        let mut fields: Vec<(&&'static str, &FieldValue)> = block.fields().iter().collect();
        fields.sort_by_key(|(k, _)| **k);

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
    if skipped > 0 {
        eprintln!(
            "warning: skipped {skipped} element(s) from a profile family this encoder cannot write yet"
        );
    }
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
    for (prefix, uri) in tables.schema.namespaces {
        write!(out, " xmlns:{prefix}=\"{uri}\"")?;
    }
    out.push_str(">\n");

    // FullModel header — declares the profile this file belongs to. Reuse a real
    // decoded FullModel entry for this profile if the dataset has one (preserves
    // scenarioTime/modelingAuthoritySet/DependentOn/version/etc.), else fall back
    // to a minimal synthetic header.
    if let Some(&(_, uri)) = tables.schema.profiles.iter().find(|(k, _)| *k == profile_code) {
        // `md` via the same tables the body uses: the schema puts FullModel and every
        // Model.* field in the ModelDescription namespace. The header keeps only its
        // genuinely different behaviour — `rdf:about` with no `#`, and no fragment on
        // resource references.
        let hdr = prefix_for_type("FullModel");
        if let Some((mrid, block)) = find_full_model_header(ds, uri) {
            write!(out, "  <{hdr}:FullModel rdf:about=\"{}\">", escape_attr(mrid))?;

            let mut fields: Vec<(&&'static str, &FieldValue)> = block.fields().iter().collect();
            fields.sort_by_key(|(k, _)| **k);

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
            write_synthesized_header(&mut out, ds, profile_code, uri)?;
        }
    }

    let mut mrids: Vec<&str> = ds.entries.keys().map(String::as_str).collect();
    mrids.sort();

    for mrid in mrids {
        let entry = &ds.entries[mrid];
        let type_name = entry.type_name();

        // Another family's element has no CGMES profile membership and no entry
        // in these tables, so it would fall through to `write_carried_fields`
        // and emit nothing. Skip it outright rather than rely on that.
        if is_foreign_family(type_name) {
            continue;
        }

        let type_origins: &[&str] = type_map.get(type_name).copied().unwrap_or(&[]);
        if !type_origins.contains(&profile_code) {
            write_carried_fields(&mut out, mrid, entry, profile_code, type_map, attr_map, ds)?;
            continue;
        }

        let is_primary =
            self_defining || type_origins.first().map_or(false, |&o| o == profile_code);
        let block = entry;

        let include_field = |key: &str| -> bool {
            let origins = attr_map.get(key).copied().unwrap_or(&[]);
            if is_primary {
                origins.contains(&profile_code)
            } else {
                origins.first().map_or(false, |&o| o == profile_code)
            }
        };

        let mut fields: Vec<(&&'static str, &FieldValue)> =
            block.fields().iter().filter(|(k, _)| include_field(k)).collect();

        if fields.is_empty() {
            continue;
        }

        let ns = prefix_for_type(type_name);
        if is_primary {
            write!(out, "  <{ns}:{type_name} rdf:ID=\"{}\">", escape_attr(mrid))?;
        } else {
            write!(out, "  <{ns}:{type_name} rdf:about=\"#{}\">", escape_attr(mrid))?;
        }

        fields.sort_by_key(|(k, _)| **k);
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

/// Namespace for the v5 UUIDs of synthesized FullModel headers.
const HEADER_UUID_NAMESPACE: [u8; 16] = [
    0x3b, 0x8e, 0x5f, 0x0a, 0x9c, 0x41, 0x4d, 0x2e, 0xa7, 0x16, 0x5d, 0x0b, 0x6f, 0x93, 0xc2, 0x48,
];
/// Placeholders for mandatory header values the dataset does not provide.
const UNKNOWN_TIME: &str = "1970-01-01T00:00:00Z";
const UNKNOWN_AUTHORITY: &str = "urn:cimoxide:unknown-modeling-authority-set";

/// Writes a FullModel header for a profile the dataset has no header for.
///
/// The Header profile (IEC 61970-552, `Header-AP-Voc-RDFS2020`) makes `Model.created`,
/// `Model.description`, `Model.modelingAuthoritySet`, `Model.scenarioTime` and
/// `Model.version` mandatory (1..1) and `Model.profile` 1..n, and a model is identified by
/// a `urn:uuid:` URN. Readers rely on this: PowSyBl, for one, ignores an SSH file whose
/// header has no `Model.modelingAuthoritySet`, silently importing no loads, setpoints or
/// switch states.
///
/// - The identifier is a v5 UUID of the profile and the dataset's mRIDs, so encoding stays a
///   pure function of the dataset and different datasets get different model ids.
/// - `scenarioTime`, `modelingAuthoritySet` and `created` come from another FullModel
///   decoded into the dataset when there is one (one dataset describes one scenario from
///   one authority), else from recognisable placeholders. Callers that know the real values
///   add a FullModel entry for the profile, which is then written as is.
fn write_synthesized_header(
    out: &mut String,
    ds: &CimDataset,
    profile_code: &str,
    profile_uri: &str,
) -> Result<(), Box<dyn Error>> {
    let hdr = prefix_for_type("FullModel");
    let md = |field: &str| {
        attr_meta(&format!("Model.{field}"), "FullModel").map_or(FALLBACK_PREFIX, |m| m.prefix)
    };
    let inherited = |field: &str| -> Option<String> {
        let mut headers: Vec<(&String, &Element)> = ds
            .entries
            .iter()
            .filter(|(_, e)| e.type_name() == "FullModel")
            .collect();
        headers.sort_by_key(|(m, _)| m.as_str());
        headers.into_iter().find_map(|(_, b)| match b.fields().get(format!("Model.{field}").as_str()) {
            Some(FieldValue::Text(v)) if !v.is_empty() => Some(v.clone()),
            _ => None,
        })
    };
    let scenario_time = inherited("scenarioTime").unwrap_or_else(|| UNKNOWN_TIME.to_string());
    let created = inherited("created").unwrap_or_else(|| scenario_time.clone());
    let authority = inherited("modelingAuthoritySet").unwrap_or_else(|| UNKNOWN_AUTHORITY.to_string());

    writeln!(out, "  <{hdr}:FullModel rdf:about=\"{}\">", header_urn(ds, profile_code))?;
    let text = |out: &mut String, field: &str, value: &str| -> std::fmt::Result {
        let p = md(field);
        writeln!(out, "    <{p}:Model.{field}>{}</{p}:Model.{field}>", escape_text(value))
    };
    text(out, "created", &created)?;
    text(out, "description", &format!("{profile_code} profile, header synthesized by cimoxide"))?;
    text(out, "modelingAuthoritySet", &authority)?;
    text(out, "profile", profile_uri)?;
    text(out, "scenarioTime", &scenario_time)?;
    text(out, "version", "1")?;
    writeln!(out, "  </{hdr}:FullModel>")?;
    Ok(())
}

/// `urn:uuid:` + a v5 UUID (RFC 9562, SHA-1 name-based) of the profile code and the
/// dataset's sorted mRIDs.
fn header_urn(ds: &CimDataset, profile_code: &str) -> String {
    use sha1::{Digest, Sha1};
    let mut mrids: Vec<&str> = ds.entries.keys().map(String::as_str).collect();
    mrids.sort_unstable();
    let mut h = Sha1::new();
    h.update(HEADER_UUID_NAMESPACE);
    h.update(profile_code.as_bytes());
    for m in mrids {
        h.update([0u8]);
        h.update(m.as_bytes());
    }
    let d = h.finalize();
    let mut b = [0u8; 16];
    b.copy_from_slice(&d[..16]);
    b[6] = (b[6] & 0x0f) | 0x50; // version 5
    b[8] = (b[8] & 0x3f) | 0x80; // RFC 4122 variant
    let hex: String = b.iter().map(|x| format!("{x:02x}")).collect();
    format!("urn:uuid:{}-{}-{}-{}-{}", &hex[0..8], &hex[8..12], &hex[12..16], &hex[16..20], &hex[20..32])
}

/// Find the decoded `FullModel` entry (if any) whose `Model.profile` field names
/// `profile_uri`. If more than one matches, the lexicographically smallest MRID
/// wins, for deterministic output.
fn find_full_model_header<'a>(ds: &'a CimDataset, profile_uri: &str) -> Option<(&'a str, &'a Element)> {
    let mut best: Option<(&str, &Element)> = None;
    for (mrid, entry) in &ds.entries {
        if entry.type_name() != "FullModel" {
            continue;
        }
        let matches = match entry.fields().get("Model.profile") {
            Some(FieldValue::Text(s)) => s == profile_uri,
            Some(FieldValue::TextList(list)) => list.iter().any(|s| s == profile_uri),
            _ => false,
        };
        if !matches {
            continue;
        }
        if best.is_none_or(|(m, _)| mrid.as_str() < m) {
            best = Some((mrid.as_str(), entry));
        }
    }
    best
}

/// Writes the fields of an element whose own class does not belong to `profile_code`,
/// but which carries attributes of a superclass that does, typed as that superclass.
///
/// CGMES 3.0 states `Equipment.inService` in SSH for equipment whose class SSH does not
/// list (ACLineSegment, PowerTransformer, BusbarSection, ...) as
/// `<cim:Equipment rdf:about="#...">`; that is why the SSH RDFS declares Equipment concrete.
/// Without this, those elements were skipped entirely: re-encoding the SmallGrid fixture
/// dropped all 314 of them, and readers that require the statement (cgmes2pgm converts no
/// line without it) lost the equipment.
///
/// A field qualifies when its attribute exists in `profile_code` only (so no other profile
/// can state it) and the class declaring it (the `Class.` part of its key) belongs to
/// `profile_code`. Attributes shared across profiles, such as `IdentifiedObject.name`, never
/// qualify: they are stated where the element's own class is. Always `rdf:about`: the
/// element itself is defined by its own class's profile.
fn write_carried_fields(
    out: &mut String,
    mrid: &str,
    block: &Element,
    profile_code: &str,
    type_map: &HashMap<&'static str, &'static [&'static str]>,
    attr_map: &HashMap<&'static str, &'static [&'static str]>,
    ds: &CimDataset,
) -> Result<(), Box<dyn Error>> {
    let mut by_class: std::collections::BTreeMap<&str, Vec<(&&'static str, &FieldValue)>> =
        std::collections::BTreeMap::new();
    for (key, val) in block.fields() {
        let only_here = attr_map.get(*key).is_some_and(|o| *o == [profile_code]);
        let Some((class, _)) = key.split_once('.') else { continue };
        let class_in_profile = type_map.get(class).is_some_and(|o| o.contains(&profile_code));
        if only_here && class_in_profile {
            by_class.entry(class).or_default().push((key, val));
        }
    }
    for (class, mut fields) in by_class {
        let ns = prefix_for_type(class);
        write!(out, "  <{ns}:{class} rdf:about=\"#{}\">", escape_attr(mrid))?;
        fields.sort_by_key(|(k, _)| **k);
        let mut children = String::new();
        for (key, val) in fields {
            write_field(&mut children, key, val, class, "#", ds)?;
        }
        write!(out, "{children}\n  </{ns}:{class}>\n")?;
    }
    Ok(())
}

/// Write one field, with its own namespace prefix taken from the schema.
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
        // overwrites the `enumeration` one — so the schema marks them as plain references. A
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
