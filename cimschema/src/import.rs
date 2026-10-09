use std::collections::HashMap;
use std::fs;

use quick_xml::events::Event;
use quick_xml::Reader;

use super::family::Family;
use super::model::*;

// Collects all fields from one rdf:Description block during XML parsing.
#[derive(Debug, Default)]
struct Block {
    about: String,
    rdf_types: Vec<String>,
    label: String,
    comment: String,
    sub_class_of: String,
    domain: String,
    range: String,
    // Every `cims:stereotype` value on the block, in document order. Which one
    // classifies the block depends on the family (see `Block::classifier`).
    stereotypes: Vec<String>,
    /// Any of the block's `cims:stereotype` values is `concrete`.
    concrete: bool,
    multiplicity: String,
    association_used: String,
    inverse_role: String,
    data_type: String,
    is_fixed: String,
    category: String,
    version_info: String,
    version_iri: String,
    keyword: String,
    title: String,
}

/// Stereotypes that decide which bucket a block lands in. Everything else
/// (`NC`, `profcim`, `deprecated`, `concrete`, ...) is an annotation.
const CLASSIFIERS: &[&str] = &["enumeration", "Primitive", "CIMDatatype", "Compound"];

impl Block {
    /// The stereotype that classifies this block.
    ///
    /// CGMES blocks carry at most one meaningful stereotype, so the historical
    /// last-wins reading is kept. NCP writes the classifier first and trails
    /// profile tags, which last-wins would misread: 94 enumeration blocks and
    /// 72+ Primitive blocks end on `NC`/`profcim`/`deprecated` and would be
    /// classified as plain classes.
    fn classifier(&self, family: &Family) -> String {
        if family.stereotype_first_wins {
            if let Some(s) = self
                .stereotypes
                .iter()
                .map(|s| uri_end(s))
                .find(|s| CLASSIFIERS.contains(&s.as_str()))
            {
                return s;
            }
        }
        self.stereotypes.last().map(|s| uri_end(s)).unwrap_or_default()
    }

    fn set(&mut self, tag: &str, value: String) {
        match tag {
            "rdf:type" => self.rdf_types.push(value),
            "rdfs:label" => self.label = value,
            "rdfs:comment" => self.comment = value,
            "rdfs:subClassOf" => self.sub_class_of = value,
            "rdfs:domain" => self.domain = value,
            "rdfs:range" => self.range = value,
            "cims:stereotype" => {
                self.concrete |= uri_end(&value) == "concrete";
                self.stereotypes.push(value);
            }
            "cims:multiplicity" => self.multiplicity = value,
            "cims:AssociationUsed" => self.association_used = value,
            "cims:inverseRoleName" => self.inverse_role = value,
            "cims:dataType" => self.data_type = value,
            "cims:isFixed" => self.is_fixed = value,
            "cims:belongsToCategory" => self.category = value,
            "owl:versionInfo" => self.version_info = value,
            "owl:versionIRI" => self.version_iri = value,
            "dcat:keyword" => self.keyword = value,
            "dcterms:title" => self.title = value,
            _ => {}
        }
    }
}

pub fn import_schema_files(
    pattern: &str,
    family: &'static Family,
    verbose: bool,
) -> Result<CimSpecification, Box<dyn std::error::Error>> {
    let mut paths: Vec<String> = glob::glob(pattern)?
        .filter_map(Result::ok)
        .map(|p| p.to_string_lossy().into_owned())
        .collect();

    if paths.is_empty() {
        return Err(format!("no schema files matched: {pattern}").into());
    }
    paths.sort();

    let mut spec = CimSpecification::new(family);
    for path in &paths {
        if verbose {
            eprintln!("parsing {path}");
        }
        parse_file(path, family, &mut spec)?;
    }

    super::processing::postprocess(&mut spec);
    Ok(spec)
}

fn parse_file(
    path: &str,
    family: &'static Family,
    spec: &mut CimSpecification,
) -> Result<(), Box<dyn std::error::Error>> {
    let content = fs::read_to_string(path)
        .map_err(|e| format!("cannot read {path}: {e}"))?;

    let mut reader = Reader::from_str(&content);
    let mut buf = Vec::new();

    let mut ns_map: HashMap<String, String> = HashMap::new();
    let mut blocks: Vec<Block> = Vec::new();
    let mut current: Option<Block> = None;
    let mut current_tag: Option<String> = None;

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(ref e)) => {
                let name = bytes_to_str(e.name().as_ref())?;

                if name == "rdf:RDF" {
                    for attr in e.attributes().flatten() {
                        let key = bytes_to_str(attr.key.as_ref())?;
                        let val = std::str::from_utf8(&attr.value)?.to_string();
                        if let Some(prefix) = key.strip_prefix("xmlns:").or_else(|| key.strip_prefix("xml:")) {
                            let ns = ensure_terminated(val);
                            ns_map.insert(prefix.to_string(), ns);
                        }
                    }
                } else if name == "rdf:Description" {
                    let mut b = Block::default();
                    for attr in e.attributes().flatten() {
                        let key = bytes_to_str(attr.key.as_ref())?;
                        if key == "rdf:about" {
                            b.about = std::str::from_utf8(&attr.value)?.to_string();
                        }
                    }
                    current = Some(b);
                } else if current.is_some() {
                    let resource = find_resource(e.attributes());
                    if let Some(res) = resource {
                        if let Some(ref mut b) = current {
                            b.set(&name, res);
                        }
                        current_tag = None;
                    } else {
                        current_tag = Some(name.to_string());
                    }
                }
            }

            Ok(Event::Empty(ref e)) => {
                let name = bytes_to_str(e.name().as_ref())?;

                if name == "rdf:Description" {
                    let mut b = Block::default();
                    for attr in e.attributes().flatten() {
                        let key = bytes_to_str(attr.key.as_ref())?;
                        if key == "rdf:about" {
                            b.about = std::str::from_utf8(&attr.value)?.to_string();
                        }
                    }
                    blocks.push(b);
                } else if current.is_some() {
                    if let Some(res) = find_resource(e.attributes()) {
                        if let Some(ref mut b) = current {
                            b.set(&name, res);
                        }
                    }
                }
            }

            Ok(Event::Text(ref e)) => {
                if let Some(tag) = current_tag.take() {
                    let text = e.unescape()?.into_owned();
                    let text = text.trim().to_string();
                    if !text.is_empty() {
                        if let Some(ref mut b) = current {
                            b.set(&tag, text);
                        }
                    }
                }
            }

            Ok(Event::End(ref e)) => {
                let name = bytes_to_str(e.name().as_ref())?;
                if name == "rdf:Description" {
                    if let Some(b) = current.take() {
                        blocks.push(b);
                    }
                }
                current_tag = None;
            }

            Ok(Event::Eof) => break,
            Err(e) => return Err(Box::new(e)),
            _ => {}
        }
        buf.clear();
    }

    // Find ontology keyword first (needed to set origin on all other objects)
    let keyword = blocks
        .iter()
        .find(|b| b.rdf_types.iter().any(|t| t.contains("owl#Ontology")))
        .map(|b| b.keyword.clone())
        .unwrap_or_default();

    // Classify and collect per-file results
    let mut types: HashMap<String, CimType> = HashMap::new();
    let mut enums: HashMap<String, CimEnum> = HashMap::new();
    let mut datatypes: HashMap<String, CimDatatype> = HashMap::new();
    let mut primitives: HashMap<String, CimPrimitive> = HashMap::new();
    let mut attributes: Vec<CimAttribute> = Vec::new();
    let mut enum_values: Vec<CimEnumValue> = Vec::new();
    let mut ontology = CimOntology::default();

    for b in blocks {
        let type_str = b.rdf_types.join(" ");
        if type_str.contains("rdf-schema#Class") {
            let stereo = b.classifier(family);
            if stereo == "enumeration" {
                let mut e = make_enum(&b, family);
                e.origin = keyword.clone();
                e.origins = vec![keyword.clone()];
                enums.insert(e.id.clone(), e);
            } else if stereo == "CIMDatatype" {
                let d = make_datatype(&b, family);
                datatypes.insert(d.id.clone(), d);
            } else if stereo == "Primitive" {
                let p = make_primitive(&b, family);
                primitives.insert(p.id.clone(), p);
            } else {
                let mut t = make_type(&b, family);
                t.origin = keyword.clone();
                t.origins = vec![keyword.clone()];
                if b.concrete {
                    t.concrete_in = vec![keyword.clone()];
                }
                types.insert(t.id.clone(), t);
            }
        } else if type_str.contains("rdf-syntax-ns#Property") {
            let mut attr = make_attribute(&b, family);
            attr.origin = keyword.clone();
            attr.origins = vec![keyword.clone()];
            attributes.push(attr);
        } else if type_str.contains("owl#Ontology") {
            ontology = make_ontology(&b);
        } else if type_str.contains("ClassCategory") {
            // skip package category nodes
        } else if !type_str.is_empty() {
            // Enum value: rdf:type points to the owning enum class
            enum_values.push(make_enum_value(&b, family));
        }
    }

    assign_attrs_to_types(&mut types, &attributes);
    assign_attrs_to_datatypes(&mut datatypes, &attributes);
    assign_values_to_enums(&mut enums, &enum_values);

    merge_types(&mut spec.types, types);
    merge_enums(&mut spec.enums, enums);
    merge_datatypes(&mut spec.cim_datatypes, datatypes);
    merge_primitives(&mut spec.primitive_types, primitives);

    if !ontology.keyword.is_empty() {
        spec.ontologies.insert(ontology.keyword.clone(), ontology);
    }

    for (k, v) in ns_map {
        spec.specification_namespaces.entry(k).or_insert(v);
    }

    Ok(())
}

// --- constructors -----------------------------------------------------------

fn make_type(b: &Block, family: &Family) -> CimType {
    CimType {
        id: uri_end(&b.about),
        label: b.label.clone(),
        comment: clean_text(&b.comment),
        namespace: uri_path(&b.about),
        super_type: uri_end(&b.sub_class_of),
        cim_stereotype: b.classifier(family),
        rdf_type: b.rdf_types.first().map(|s| uri_end(s)).unwrap_or_default(),
        cim_categories: vec![uri_end(&b.category)],
        ..Default::default()
    }
}

fn make_primitive(b: &Block, family: &Family) -> CimPrimitive {
    CimPrimitive {
        id: uri_end(&b.about),
        label: b.label.clone(),
        comment: clean_text(&b.comment),
        namespace: uri_path(&b.about),
        cim_stereotype: b.classifier(family),
        rdf_type: b.rdf_types.first().map(|s| uri_end(s)).unwrap_or_default(),
        ..Default::default()
    }
}

fn make_datatype(b: &Block, family: &Family) -> CimDatatype {
    CimDatatype {
        id: uri_end(&b.about),
        label: b.label.clone(),
        comment: clean_text(&b.comment),
        namespace: uri_path(&b.about),
        cim_stereotype: b.classifier(family),
        rdf_type: b.rdf_types.first().map(|s| uri_end(s)).unwrap_or_default(),
        cim_category: uri_end(&b.category),
        ..Default::default()
    }
}

fn make_enum(b: &Block, family: &Family) -> CimEnum {
    CimEnum {
        id: uri_end(&b.about),
        label: b.label.clone(),
        comment: clean_text(&b.comment),
        namespace: uri_path(&b.about),
        cim_stereotype: b.classifier(family),
        rdf_type: b.rdf_types.first().map(|s| uri_end(s)).unwrap_or_default(),
        ..Default::default()
    }
}

fn make_enum_value(b: &Block, family: &Family) -> CimEnumValue {
    CimEnumValue {
        id: uri_end(&b.about),
        label: b.label.clone(),
        comment: clean_text(&b.comment),
        cim_stereotype: b.classifier(family),
        rdf_type: b.rdf_types.first().map(|s| uri_end(s)).unwrap_or_default(),
    }
}

fn make_ontology(b: &Block) -> CimOntology {
    CimOntology {
        id: uri_end(&b.about),
        namespace: uri_path(&b.about),
        rdf_type: b.rdf_types.first().map(|s| uri_end(s)).unwrap_or_default(),
        owl_version_iri: b.version_iri.clone(),
        owl_version_info: b.version_info.clone(),
        keyword: b.keyword.clone(),
        name: b.title.trim_end_matches(" Vocabulary").to_string(),
        priority: 0,
    }
}

fn make_attribute(b: &Block, family: &Family) -> CimAttribute {
    let assoc = b.association_used.to_lowercase();
    let is_assoc = assoc == "yes" || assoc.is_empty();
    let is_list = is_list_multiplicity(&b.multiplicity);
    CimAttribute {
        id: uri_end(&b.about),
        label: b.label.clone(),
        comment: clean_text(&b.comment),
        namespace: uri_path(&b.about),
        cim_stereotype: b.classifier(family),
        rdf_domain: uri_end(&b.domain),
        rdf_range: uri_end(&b.range),
        cim_data_type: uri_end(&b.data_type),
        rdf_type: b.rdf_types.first().map(|s| uri_end(s)).unwrap_or_default(),
        cim_association_used: assoc,
        is_association_used: is_assoc,
        cim_inverse_role: uri_end(&b.inverse_role),
        cim_multiplicity: b.multiplicity.clone(),
        is_list,
        cim_is_fixed: b.is_fixed.clone(),
        ..Default::default()
    }
}

// --- assignment -------------------------------------------------------------

fn assign_attrs_to_types(types: &mut HashMap<String, CimType>, attrs: &[CimAttribute]) {
    for attr in attrs {
        if let Some(t) = types.get_mut(&attr.rdf_domain) {
            let mut a = attr.clone();
            a.cim_categories = t.cim_categories.clone();
            t.attributes.push(a);
        }
    }
}

fn assign_attrs_to_datatypes(dt: &mut HashMap<String, CimDatatype>, attrs: &[CimAttribute]) {
    for attr in attrs {
        if let Some(d) = dt.get_mut(&attr.rdf_domain) {
            d.attributes.push(attr.clone());
        }
    }
}

fn assign_values_to_enums(enums: &mut HashMap<String, CimEnum>, values: &[CimEnumValue]) {
    for val in values {
        if let Some(e) = enums.get_mut(&val.rdf_type) {
            e.values.push(val.clone());
        }
    }
}

// --- merge ------------------------------------------------------------------

fn merge_types(target: &mut HashMap<String, CimType>, source: HashMap<String, CimType>) {
    for (k, src) in source {
        if let Some(existing) = target.get_mut(&k) {
            if !src.super_type.is_empty() {
                existing.super_type = src.super_type;
            }
            if !src.cim_stereotype.is_empty() {
                existing.cim_stereotype = src.cim_stereotype;
            }
            // The first profile (in file-name order) to describe a class or
            // attribute gives its comment.
            if existing.comment.is_empty() {
                existing.comment = src.comment;
            }
            existing.cim_categories.extend(src.cim_categories);
            if !src.origin.is_empty() {
                existing.origins.push(src.origin);
            }
            existing.concrete_in.extend(src.concrete_in);
            for attr in src.attributes {
                if let Some(idx) = existing.attributes.iter().position(|a| a.id == attr.id) {
                    if existing.attributes[idx].comment.is_empty() {
                        existing.attributes[idx].comment = attr.comment;
                    }
                    existing.attributes[idx].origins.extend(attr.origins);
                    existing.attributes[idx]
                        .cim_categories
                        .extend(attr.cim_categories);
                } else {
                    existing.attributes.push(attr);
                }
            }
        } else {
            target.insert(k, src);
        }
    }
}

fn merge_enums(target: &mut HashMap<String, CimEnum>, source: HashMap<String, CimEnum>) {
    for (k, src) in source {
        if let Some(existing) = target.get_mut(&k) {
            if !src.cim_stereotype.is_empty() {
                existing.cim_stereotype = src.cim_stereotype;
            }
            if existing.comment.is_empty() {
                existing.comment = src.comment;
            }
            if !src.origin.is_empty() {
                existing.origins.push(src.origin);
            }
            for val in src.values {
                match existing.values.iter_mut().find(|v| v.id == val.id) {
                    Some(v) if v.comment.is_empty() => v.comment = val.comment,
                    Some(_) => {}
                    None => existing.values.push(val),
                }
            }
        } else {
            target.insert(k, src);
        }
    }
}

fn merge_datatypes(
    target: &mut HashMap<String, CimDatatype>,
    source: HashMap<String, CimDatatype>,
) {
    for (k, src) in source {
        if let Some(existing) = target.get_mut(&k) {
            if !src.cim_stereotype.is_empty() {
                existing.cim_stereotype = src.cim_stereotype;
            }
        } else {
            target.insert(k, src);
        }
    }
}

fn merge_primitives(
    target: &mut HashMap<String, CimPrimitive>,
    source: HashMap<String, CimPrimitive>,
) {
    for (k, src) in source {
        if let Some(existing) = target.get_mut(&k) {
            if !src.cim_stereotype.is_empty() {
                existing.cim_stereotype = src.cim_stereotype;
            }
        } else {
            target.insert(k, src);
        }
    }
}

// --- helpers ----------------------------------------------------------------

fn bytes_to_str(b: &[u8]) -> Result<String, Box<dyn std::error::Error>> {
    Ok(std::str::from_utf8(b)?.to_string())
}

fn find_resource(
    attrs: quick_xml::events::attributes::Attributes,
) -> Option<String> {
    for attr in attrs.flatten() {
        if let Ok(key) = std::str::from_utf8(attr.key.as_ref()) {
            if key == "rdf:resource" {
                if let Ok(s) = std::str::from_utf8(&attr.value) {
                    return Some(s.to_string());
                }
            }
        }
    }
    None
}

/// Local name of a URI. `#` wins; `/` is the fallback for slash-terminated
/// vocabularies such as `http://purl.org/dc/terms/title`, which NCP's
/// DatasetMetadata profile uses and CGMES never does.
pub fn uri_end(uri: &str) -> String {
    if let Some(pos) = uri.rfind('#') {
        uri[pos + 1..].to_string()
    } else if let Some(pos) = uri.rfind('/') {
        uri[pos + 1..].to_string()
    } else {
        uri.to_string()
    }
}

/// Namespace of a URI, including its trailing delimiter.
pub fn uri_path(uri: &str) -> String {
    if let Some(pos) = uri.rfind('#') {
        uri[..=pos].to_string()
    } else if let Some(pos) = uri.rfind('/') {
        uri[..=pos].to_string()
    } else {
        String::new()
    }
}

/// True when the multiplicity's upper bound admits more than one value.
///
/// Reproduces the CGMES set (`0..n`, `1..n`, `2..n`, `0..2`) exactly and also
/// covers NCP's `0..3`, `1..2` and `2..2`, which a literal match would silently
/// treat as scalar.
fn is_list_multiplicity(m: &str) -> bool {
    let token = match m.rfind("#M:") {
        Some(pos) => &m[pos + 3..],
        None => return false,
    };
    let upper = token.rsplit("..").next().unwrap_or(token);
    upper == "n" || upper.parse::<u32>().is_ok_and(|n| n > 1)
}

/// Namespaces are compared verbatim downstream, so they must carry a delimiter.
/// A slash-terminated namespace is already terminated.
fn ensure_terminated(mut ns: String) -> String {
    if !ns.ends_with('#') && !ns.ends_with('/') {
        ns.push('#');
    }
    ns
}

pub fn clean_text(s: &str) -> String {
    // Strip HTML tags with a simple state machine
    let mut out = String::with_capacity(s.len());
    let mut in_tag = false;
    for ch in s.chars() {
        match ch {
            '<' => in_tag = true,
            '>' => in_tag = false,
            _ if !in_tag => out.push(ch),
            _ => {}
        }
    }
    let out = out
        .replace("&amp;", "&")
        .replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&quot;", "'")
        .replace("&apos;", "'")
        .replace('"', "'")
        .replace('\u{2013}', "-"); // en dash

    out.split_whitespace().collect::<Vec<_>>().join(" ")
}

// ---------------------------------------------------------------------------
// Profile index
// ---------------------------------------------------------------------------

/// One application profile's identity: which IRIs a dataset can declare
/// conformance to, and the short code the constraint manifests use.
#[derive(Debug)]
pub struct ProfileInfo {
    /// `dcat:keyword` — the short code (`"CO"`, `"ER"`, …).
    pub keyword: String,
    /// The profile IRI and every version IRI of it. A dataset's
    /// `dcterms:conformsTo` names one of these.
    pub iris: Vec<String>,
}

/// Read the profile descriptors in a `PROF` directory (`NCP/PROF`,
/// `CGMES/PROF`).
///
/// These are the authority on profile identity. Without them the profile IRI a
/// dataset declares — NC's `dcterms:conformsTo`, CGMES's `md:Model.profile` —
/// is opaque, and nothing can say which shapes apply.
pub fn import_profile_index(
    dir: &std::path::Path,
) -> Result<Vec<ProfileInfo>, Box<dyn std::error::Error>> {
    let mut paths: Vec<std::path::PathBuf> = std::fs::read_dir(dir)?
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .filter(|p| p.extension().and_then(|x| x.to_str()) == Some("rdf"))
        .collect();
    paths.sort();

    let mut out = Vec::new();
    for path in &paths {
        if let Some(info) = read_profile(path)? {
            out.push(info);
        }
    }
    out.sort_by(|a, b| a.keyword.cmp(&b.keyword));
    Ok(out)
}

fn read_profile(
    path: &std::path::Path,
) -> Result<Option<ProfileInfo>, Box<dyn std::error::Error>> {
    use quick_xml::events::Event;

    let mut reader = quick_xml::Reader::from_file(path)?;
    reader.config_mut().trim_text(true);

    let mut buf = Vec::new();
    let mut keyword: Option<String> = None;
    // A PROF file holds several sibling rdf:Description blocks: the profile
    // itself, then one per artifact. Only the profile carries dcat:keyword, so
    // the current subject is tracked per block and kept when the keyword
    // arrives — taking the last subject seen would name an artifact instead.
    let mut current: Option<String> = None;
    let mut subject: Option<String> = None;
    let mut iris: Vec<String> = Vec::new();
    let mut versions: Vec<(String, String)> = Vec::new();
    let mut in_keyword = false;
    let mut depth = 0usize;

    loop {
        match reader.read_event_into(&mut buf)? {
            Event::Eof => break,
            Event::Start(e) | Event::Empty(e) => {
                let name = e.name();
                let local = local_name(name.as_ref());
                match local {
                    b"Description" => {
                        depth += 1;
                        if depth == 1 {
                            current = attr(&e, b"about")?;
                        }
                    }
                    b"keyword" if depth == 1 => in_keyword = true,
                    // Recorded against the block that declares it; which
                    // block was the profile is only known once the keyword
                    // turns up, and it can appear either side of this.
                    b"versionIRI" if depth == 1 => {
                        if let (Some(subj), Some(v)) = (current.clone(), attr(&e, b"resource")?) {
                            versions.push((subj, v));
                        }
                    }
                    _ => {}
                }
            }
            Event::End(e) => {
                let name = e.name();
                match local_name(name.as_ref()) {
                    b"Description" => depth = depth.saturating_sub(1),
                    b"keyword" => in_keyword = false,
                    _ => {}
                }
            }
            Event::Text(t) if in_keyword => {
                let s = t.unescape()?.trim().to_string();
                if !s.is_empty() && keyword.is_none() {
                    keyword = Some(s);
                    subject = current.clone();
                }
            }
            _ => {}
        }
        buf.clear();
    }

    let Some(keyword) = keyword else { return Ok(None) };
    let Some(subject) = subject else { return Ok(None) };
    iris.extend(
        versions
            .into_iter()
            .filter(|(subj, _)| *subj == subject)
            .map(|(_, v)| v),
    );
    iris.push(subject);
    iris.sort();
    iris.dedup();
    if iris.is_empty() {
        return Ok(None);
    }
    Ok(Some(ProfileInfo { keyword, iris }))
}

fn local_name(qname: &[u8]) -> &[u8] {
    match qname.iter().position(|b| *b == b':') {
        Some(i) => &qname[i + 1..],
        None => qname,
    }
}

fn attr(
    e: &quick_xml::events::BytesStart<'_>,
    want: &[u8],
) -> Result<Option<String>, Box<dyn std::error::Error>> {
    for a in e.attributes().flatten() {
        if local_name(a.key.as_ref()) == want {
            return Ok(Some(String::from_utf8(a.value.to_vec())?));
        }
    }
    Ok(None)
}
