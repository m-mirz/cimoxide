//! RDF/XML export of one Network Code profile: its DCAT header and the NC
//! elements and fields the profile exchanges.
//!
//! Unlike CGMES, NC writes no `md:FullModel`: a dataset is described by a
//! `dcat:Dataset` (`nc:Dataset` here) whose `dcterms:conformsTo` names the
//! application profile. Most of its properties (`dcterms:`, `prov:`,
//! `adms:`) are not in the NC class table, so their namespaces come from the
//! documents the dataset was decoded from ([`CimDataset::field_forms`]).
//!
//! Identifiers are written back in the form they were read: an mRID that is
//! an IRI (`urn:uuid:…`) as `rdf:about="urn:uuid:…"`, an object a file only
//! described as `rdf:about="#…"`, one it defined as `rdf:ID`.
//!
//! CGMES elements belong to the CGMES profiles and are not written, with one
//! exception: an NC file may type an object with a class of NC's `cim:`
//! namespace that the NC table lacks (`cim:PhaseTapChangerLinear` in an ER
//! file), which the decoder resolves to the CGMES class of that name. When
//! such an element carries fields the profile exchanges, it is written with
//! those, under NC's `cim:` namespace, as the file wrote it.

use std::collections::BTreeMap;
use std::error::Error;
use std::fmt::Write as FmtWrite;

use crate::CimDataset;
use crate::base::{AttrDef, AttrKind, Element, FieldValue, Schema};
use crate::registry::type_registry;

/// Preferred prefixes, after the documents' own ([`CimDataset::prefixes`])
/// and the NC schema's. A namespace in none gets `ns1`, `ns2`, …
const KNOWN_PREFIXES: &[(&str, &str)] = &[
    ("adms", "http://www.w3.org/ns/adms#"),
    ("cim17", "http://iec.ch/TC57/CIM100#"),
    ("dcat-cim", "https://cim4.eu/ns/dcat-cim#"),
    ("dcterms", "http://purl.org/dc/terms/"),
    ("eu", "https://cim.ucaiug.io/ns/eu#"),
    ("eu17", "http://iec.ch/TC57/CIM100-European#"),
    ("eumd", "https://cim4.eu/ns/Metadata-European#"),
    ("md", "http://iec.ch/TC57/61970-552/ModelDescription/1#"),
    ("prov", "http://www.w3.org/ns/prov#"),
    ("skos", "http://www.w3.org/2004/02/skos/core#"),
    ("time", "http://www.w3.org/2006/time#"),
];

/// NC's `cim:` namespace.
const NC_CIM: &str = "https://cim.ucaiug.io/ns#";
const RDF_NS: &str = "http://www.w3.org/1999/02/22-rdf-syntax-ns#";
const DCTERMS_NS: &str = "http://purl.org/dc/terms/";
/// The NC header classes: `dcat:Dataset` and `dcatcim:DifferenceSet`.
const HEADERS: &[&str] = &["nc:Dataset", "nc:DifferenceSet"];

fn schema() -> &'static Schema {
    type_registry().schema("nc").expect("the NC family is registered")
}

/// The profile vocabulary URI an NC profile code stands for
/// (`https://ap-voc.cim4.eu/Contingency/2.3` for `CO`).
pub fn nc_profile_uri(code: &str) -> Option<&'static str> {
    schema().profiles.iter().find(|(c, _)| *c == code).map(|(_, uri)| *uri)
}

/// The application profile's name in a profile URI: `Contingency` in both the
/// vocabulary's `https://ap-voc.cim4.eu/Contingency/2.3` and the
/// `https://ap.cim4.eu/Contingency/2.3` a header's `conformsTo` names.
fn profile_name(uri: &str) -> Option<&str> {
    let path = uri.split_once("://")?.1.split_once('/')?.1;
    path.split('/').next().filter(|s| !s.is_empty())
}

/// Namespace IRI → prefix, allocated as the body uses them.
pub(crate) struct Prefixes {
    pub(crate) bound: BTreeMap<&'static str, String>,
    generated: usize,
    /// (prefix, namespace) the dataset's documents used, sorted.
    documents: Vec<(&'static str, &'static str)>,
}

impl Prefixes {
    /// An allocator for `ds`, whose prefixes start as `bindings` (prefix,
    /// namespace), the ones a document's root already declares.
    pub(crate) fn seeded(ds: &CimDataset, bindings: &[(&'static str, &'static str)]) -> Self {
        let mut documents: Vec<(&'static str, &'static str)> = ds.prefixes.iter().map(|(ns, p)| (*p, *ns)).collect();
        documents.sort_unstable();
        Self { bound: bindings.iter().map(|(p, ns)| (*ns, p.to_string())).collect(), generated: 0, documents }
    }

    pub(crate) fn of(&mut self, ns: &'static str) -> String {
        if let Some(p) = self.bound.get(ns) {
            return p.clone();
        }
        let taken = |p: &str, bound: &BTreeMap<&str, String>| bound.values().any(|b| b == p);
        let preferred = self
            .documents
            .iter()
            .chain(schema().namespaces)
            .chain(KNOWN_PREFIXES)
            .filter(|(_, n)| *n == ns)
            .map(|(p, _)| p.to_string())
            .find(|p| !p.is_empty() && !taken(p, &self.bound));
        let prefix = preferred.unwrap_or_else(|| loop {
            self.generated += 1;
            let p = format!("ns{}", self.generated);
            if !taken(&p, &self.bound) {
                break p;
            }
        });
        self.bound.insert(ns, prefix.clone());
        prefix
    }
}

/// The dataset's header for `profile_uri`: a `dcat:Dataset` or
/// `DifferenceSet` whose `conformsTo` names the same application profile.
/// The smallest mRID wins if several do.
fn header_for<'a>(ds: &'a CimDataset, profile_uri: &str) -> Option<&'a Element> {
    let name = profile_name(profile_uri)?;
    let mut headers: Vec<&Element> = HEADERS
        .iter()
        .flat_map(|t| ds.by_type.get(*t).into_iter().flatten())
        .filter_map(|m| ds.entries.get(m))
        .filter(|e| {
            e.get_refs("conformsTo").iter().map(String::as_str).chain(e.get_str("conformsTo"))
                .any(|c| profile_name(c) == Some(name))
        })
        .collect();
    headers.sort_by_key(|e| e.mrid());
    headers.first().copied()
}

/// Export the elements of NC profile `code` (`CO`, `AE`, …) as RDF/XML.
///
/// - The header is the dataset's own for this profile when it has one, else a
///   minimal `dcat:Dataset` naming the profile, identified by a v5 UUID of the
///   dataset as CGMES's synthesized headers are.
/// - An element whose class the profile exchanges is written with the fields
///   the profile exchanges, and every field the class table does not declare
///   (data may carry what the table lacks).
/// - An element of another class is written only for the fields no other
///   profile exchanges (`TapChanger.TapChangerController` in ER), as the
///   description of an object defined elsewhere.
pub fn dataset_to_xml_for_nc_profile(ds: &CimDataset, code: &str) -> Result<String, Box<dyn Error>> {
    let profile_uri = nc_profile_uri(code).ok_or_else(|| format!("unknown NC profile code {code:?}"))?;
    let reg = type_registry();
    let mut prefixes = Prefixes::seeded(ds, &[]);
    let mut body = String::new();

    let header = header_for(ds, profile_uri);
    match header {
        Some(h) => write_element(&mut body, &mut prefixes, ds, h, h.class().ns, |_| true)?,
        None => write_synthesized_header(&mut body, &mut prefixes, ds, code, profile_uri)?,
    }

    let mut mrids: Vec<&str> = ds.entries.keys().map(String::as_str).collect();
    mrids.sort_unstable();
    let in_profile = |a: Option<&AttrDef>| a.is_some_and(|a| a.origins.contains(&code));
    // What an object of a class outside the profile carries into it: an
    // attribute no other profile exchanges. `IdentifiedObject.name`, which
    // most do, would draw every named object into every profile.
    let only_here = |a: Option<&AttrDef>| a.is_some_and(|a| a.origins == [code]);
    for mrid in mrids {
        let element = &ds.entries[mrid];
        let class = element.class();
        if HEADERS.contains(&class.qualified) {
            continue;
        }
        if !class.qualified.starts_with("nc:") {
            // A class NC's `cim:` namespace names but the NC table lacks: its
            // NC fields are those its CGMES class does not declare, so a CGMES
            // object's `IdentifiedObject.name` never draws it in.
            let nc_attr = |key: &str| reg.attr(class, key).is_none().then(|| reg.family_attr("nc", key)).flatten();
            if element.fields().keys().any(|k| only_here(nc_attr(k))) {
                write_element(&mut body, &mut prefixes, ds, element, NC_CIM, |key| only_here(nc_attr(key)))?;
            }
            continue;
        }
        if class.origins.contains(&code) {
            write_element(&mut body, &mut prefixes, ds, element, class.ns, |key| {
                let a = reg.attr_of(class, key);
                a.is_none() || in_profile(a)
            })?;
        } else if element.fields().keys().any(|k| only_here(reg.attr_of(class, k))) {
            write_element(&mut body, &mut prefixes, ds, element, class.ns, |key| only_here(reg.attr_of(class, key)))?;
        }
    }

    let mut out = String::from("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<rdf:RDF");
    write!(out, " xmlns:rdf=\"{RDF_NS}\"")?;
    let mut bindings: Vec<(&String, &&str)> = prefixes.bound.iter().map(|(ns, p)| (p, ns)).collect();
    bindings.sort();
    for (prefix, ns) in bindings {
        write!(out, " xmlns:{prefix}=\"{}\"", escape_attr(ns))?;
    }
    out.push_str(">\n");
    out.push_str(&body);
    out.push_str("</rdf:RDF>\n");
    Ok(out)
}

/// Write every NC object and every foreign object ([`CimDataset::foreign`])
/// with all its fields, as read: what [`crate::convert::dataset_to_xml`] adds
/// to the CGMES elements. Headers first, then by identifier.
pub(crate) fn write_as_read(out: &mut String, prefixes: &mut Prefixes, ds: &CimDataset) -> Result<(), Box<dyn Error>> {
    let mut nc: Vec<&Element> = ds.entries.values().filter(|e| e.type_name().starts_with("nc:")).collect();
    nc.sort_by_key(|e| (!HEADERS.contains(&e.type_name()), e.mrid()));
    for element in nc {
        write_element(out, prefixes, ds, element, element.class().ns, |_| true)?;
    }
    let mut foreign: Vec<&Element> = ds.foreign.values().collect();
    foreign.sort_by_key(|e| e.mrid());
    for element in foreign {
        write_element(out, prefixes, ds, element, element.class().ns, |_| true)?;
    }
    Ok(())
}

/// The `rdf:ID` / `rdf:about` / `rdf:nodeID` attribute for an element, in the
/// form it was read.
fn identity(element: &Element) -> String {
    let mrid = element.mrid();
    if let Some(label) = mrid.strip_prefix("_:") {
        format!("rdf:nodeID=\"{}\"", escape_attr(label))
    } else if mrid.contains(':') {
        format!("rdf:about=\"{}\"", escape_attr(mrid))
    } else if element.defined() {
        format!("rdf:ID=\"{}\"", escape_attr(mrid))
    } else {
        format!("rdf:about=\"#{}\"", escape_attr(mrid))
    }
}

/// Write `element`, typed in the namespace it was read in
/// ([`CimDataset::type_ns`]) or else `class_ns`, with the fields `keep`
/// selects, sorted by key; nothing if it selects none and the element has
/// fields. A field's namespace is the one it was read in where that is not
/// its declaration's ([`CimDataset::field_forms`]), else its declaration's, in
/// the element's family or else NC's.
fn write_element(
    out: &mut String,
    prefixes: &mut Prefixes,
    ds: &CimDataset,
    element: &Element,
    class_ns: &'static str,
    keep: impl Fn(&str) -> bool,
) -> Result<(), Box<dyn Error>> {
    let class = element.class();
    let mut fields: Vec<(&&'static str, &FieldValue)> = element.fields().iter().filter(|(k, _)| keep(k)).collect();
    if fields.is_empty() && !element.fields().is_empty() {
        return Ok(());
    }
    fields.sort_by_key(|(k, _)| **k);
    let tag_ns = ds.type_ns.get(element.mrid()).copied().unwrap_or(class_ns);
    let tag = format!("{}:{}", prefixes.of(tag_ns), class.local);
    write!(out, "  <{tag} {}>", identity(element))?;
    let reg = type_registry();
    // A foreign object's fields are named only by the documents: a CIM
    // attribute of the same bare name may live in another namespace.
    let foreign = ds.foreign.get(element.mrid()).is_some_and(|f| std::ptr::eq(f, element));
    for (key, value) in fields {
        let attr = if foreign { None } else { reg.attr_of(class, key).or_else(|| reg.family_attr("nc", key)) };
        let form = ds.field_forms.get(&(class.qualified, *key)).copied().unwrap_or_default();
        let ns = form.ns.or(attr.map(|a| a.ns)).unwrap_or(class_ns);
        let datatype = form.datatype.map(|d| format!(" rdf:datatype=\"{}\"", escape_attr(d))).unwrap_or_default();
        let name = format!("{}:{key}", prefixes.of(ns));
        let langs: Vec<&str> = element.langs(key).collect();
        let texts: &[String] = match value {
            FieldValue::Text(s) => std::slice::from_ref(s),
            FieldValue::TextList(v) => v,
            _ => &[],
        };
        // A tag belongs to a value only when every value had one: untagged
        // values leave no entry, so otherwise the pairing is unknown.
        let tagged = langs.len() == texts.len();
        for (i, text) in texts.iter().enumerate() {
            match (tagged, langs.get(i)) {
                (true, Some(lang)) => {
                    write!(out, "\n    <{name} xml:lang=\"{}\">{}</{name}>", escape_attr(lang), escape_text(text))?
                }
                _ => write!(out, "\n    <{name}{datatype}>{}</{name}>", escape_text(text))?,
            }
        }
        let refs: &[String] = match value {
            FieldValue::Resource(r) => std::slice::from_ref(r),
            FieldValue::ResourceList(v) => v,
            _ => &[],
        };
        for r in refs {
            if let Some(label) = r.strip_prefix("_:") {
                write!(out, "\n    <{name} rdf:nodeID=\"{}\"/>", escape_attr(label))?;
            } else {
                let cut_from = form.resource_ns.filter(|_| !ds.entries.contains_key(r) && !ds.foreign.contains_key(r));
                write!(out, "\n    <{name} rdf:resource=\"{}\"/>", escape_attr(&resource(attr, cut_from, r)))?;
            }
        }
    }
    writeln!(out, "\n  </{tag}>")?;
    Ok(())
}

/// The IRI a decoded reference stands for: an enumeration value in its
/// namespace, an IRI or URN as written, a value cut from another namespace
/// back in it ([`crate::decode::FieldForm::resource_ns`]; the caller passes
/// none for an object the dataset holds), an object of the document as
/// `#mrid`.
fn resource(attr: Option<&AttrDef>, cut_from: Option<&str>, value: &str) -> String {
    match attr {
        Some(a) if a.kind == AttrKind::Enum && !a.value_ns.is_empty() && !value.contains(':') => {
            format!("{}{value}", a.value_ns)
        }
        _ if value.contains(':') => value.to_string(),
        _ => match cut_from {
            Some(ns) => format!("{ns}{value}"),
            None => format!("#{value}"),
        },
    }
}

/// A minimal header for a dataset that has none for this profile: the
/// application profile it conforms to and an identifier.
fn write_synthesized_header(
    out: &mut String,
    prefixes: &mut Prefixes,
    ds: &CimDataset,
    code: &str,
    profile_uri: &str,
) -> Result<(), Box<dyn Error>> {
    let urn = crate::convert::header_urn(ds, code);
    let dcat = prefixes.of("http://www.w3.org/ns/dcat#");
    let dcterms = prefixes.of(DCTERMS_NS);
    // The header names the application profile, the vocabulary's sibling.
    let conforms = profile_uri.replacen("://ap-voc.", "://ap.", 1);
    writeln!(out, "  <{dcat}:Dataset rdf:about=\"{urn}\">")?;
    writeln!(out, "    <{dcterms}:conformsTo rdf:resource=\"{}\"/>", escape_attr(&conforms))?;
    writeln!(out, "    <{dcterms}:identifier>{}</{dcterms}:identifier>", urn.trim_start_matches("urn:uuid:"))?;
    writeln!(out, "  </{dcat}:Dataset>")?;
    Ok(())
}

fn escape_text(s: &str) -> String {
    s.replace('&', "&amp;").replace('<', "&lt;").replace('>', "&gt;")
}

fn escape_attr(s: &str) -> String {
    s.replace('&', "&amp;").replace('<', "&lt;").replace('"', "&quot;")
}
