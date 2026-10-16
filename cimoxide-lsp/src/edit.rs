//! Edits across a model set, as text edits for the client to apply: renaming
//! an mRID, deleting objects and adding a field a finding says is missing;
//! and the objects a reference can name, the likeliest first; adding an
//! object in a container. They change the files as written, keeping
//! everything else of them, and reach every file of the set: an object's
//! EQ, SSH and SV elements and every reference to it.
//!
//! The files are read as they are now: an open document's buffer, else the
//! file on disk with the index of the set's last validation.

use std::collections::{HashMap, HashSet};

use lsp_types::{
    AnnotatedTextEdit, ChangeAnnotation, DocumentChanges, OneOf, OptionalVersionedTextDocumentIdentifier, Position, Range,
    TextDocumentEdit, TextEdit, Url, WorkspaceEdit,
};

use cimmodel::base::AttrKind;
use cimmodel::registry::type_registry;

use crate::features::class_of;
use crate::index::{IdKind, Index, LineIndex, contains, local};

/// One file of the set, as it is now.
pub struct File<'a> {
    pub url: &'a Url,
    pub text: &'a str,
    pub lines: &'a LineIndex,
    pub index: &'a Index,
}

impl File<'_> {
    /// The last `len` bytes of `r`: an mRID at the end of the value written
    /// (after `#`, or the whole of `_x` or `urn:uuid:x`).
    fn suffix(&self, r: Range, len: usize) -> Range {
        let end = self.lines.offset(self.text, r.end);
        Range::new(self.lines.position(self.text, end.saturating_sub(len)), r.end)
    }

    /// `r` widened to whole lines when nothing else is written on them.
    fn lines_of(&self, r: Range) -> Range {
        let (text, start, end) = (self.text, self.lines.offset(self.text, r.start), self.lines.offset(self.text, r.end));
        let line_start = text[..start].rfind('\n').map_or(0, |i| i + 1);
        let line_end = text[end..].find('\n').map_or(text.len(), |i| end + i + 1);
        if text[line_start..start].trim().is_empty() && text[end..line_end].trim().is_empty() {
            self.lines.range(text, line_start, line_end)
        } else {
            r
        }
    }

    /// The text of a field written `<q>text</q>`, and its range.
    fn field_text(&self, head: Range, full: Range, qname: &str) -> Option<(&str, Range)> {
        let start = self.lines.offset(self.text, head.end);
        let end = self.lines.offset(self.text, full.end).checked_sub(qname.len() + 3)?;
        let value = self.text.get(start..end)?;
        (!value.contains('<')).then(|| (value, self.lines.range(self.text, start, end)))
    }
}

/// The mRID at `pos` in `url` — an element's `rdf:ID`/`rdf:about`, or an
/// `rdf:resource` — and the range of it as written; anywhere else in an
/// element's start tag, that element's.
pub fn prepare_rename(files: &[File], url: &Url, pos: Position) -> Option<(Range, String)> {
    let file = files.iter().find(|f| f.url == url)?;
    let el = file.index.element_at(pos)?;
    if let Some((mrid, r)) = el.fields.iter().filter_map(|f| f.resource.as_ref()).find(|(_, r)| contains(*r, pos)) {
        return Some((file.suffix(*r, mrid.len()), mrid.clone()));
    }
    let id = el.id_range?;
    (contains(el.head, pos) && !el.mrid.is_empty()).then(|| (file.suffix(id, el.mrid.len()), el.mrid.clone()))
}

/// Rename mRID `old` to `new` in every file: each element's `rdf:ID` or
/// `rdf:about`, each `rdf:resource` to it, and an `IdentifiedObject.mRID`
/// that writes it (without its leading `_`, as CGMES 3 does, if so).
pub fn rename(files: &[File], old: &str, new: &str) -> Result<WorkspaceEdit, String> {
    let new = new.trim();
    if new.is_empty() || new.contains(|c: char| c.is_whitespace() || "\"'<>&#".contains(c)) {
        return Err(format!("`{new}` cannot be an mRID: no spaces, quotes, `<`, `>`, `&` or `#`"));
    }
    let defined = files.iter().flat_map(|f| &f.index.elements).any(|e| e.mrid == old && e.id_kind == Some(IdKind::Id));
    let ncname = new.starts_with(|c: char| c.is_alphabetic() || c == '_') && !new.contains(':');
    if defined && !ncname {
        return Err(format!("`{new}` cannot be an `rdf:ID`: it must start with a letter or `_` and hold no `:`"));
    }
    if files.iter().flat_map(|f| &f.index.elements).any(|e| e.mrid == new) {
        return Err(format!("the model set already has an object `{new}`"));
    }
    let mut changes = HashMap::new();
    for file in files {
        let mut edits = Vec::new();
        for el in &file.index.elements {
            if el.mrid == old
                && let Some(id) = el.id_range
            {
                edits.push(TextEdit::new(file.suffix(id, old.len()), new.into()));
                for f in el.fields.iter().filter(|f| f.key() == "IdentifiedObject.mRID") {
                    let Some((text, r)) = file.field_text(f.head, f.full, &f.qname) else { continue };
                    let bare = |s: &str| s.strip_prefix('_').unwrap_or(s).to_string();
                    if text.trim() == old {
                        edits.push(TextEdit::new(r, new.into()));
                    } else if text.trim() == bare(old) {
                        edits.push(TextEdit::new(r, bare(new)));
                    }
                }
            }
            for (mrid, r) in el.fields.iter().filter_map(|f| f.resource.as_ref()) {
                if mrid == old {
                    edits.push(TextEdit::new(file.suffix(*r, old.len()), new.into()));
                }
            }
        }
        if !edits.is_empty() {
            changes.insert(file.url.clone(), edits);
        }
    }
    Ok(WorkspaceEdit { changes: Some(changes), ..Default::default() })
}

/// What deleting objects `ids` changes: the elements writing them, in every
/// file, and the fields referring to them from what stays.
pub struct Deletion {
    pub edit: WorkspaceEdit,
    pub objects: usize,
    pub references: usize,
}

/// Delete objects `ids`; the client asks before applying it (`label`).
pub fn delete(files: &[File], ids: &HashSet<&str>, label: &str) -> Deletion {
    let annotation = "delete".to_string();
    let mut changes = Vec::new();
    let (mut objects, mut references) = (HashSet::new(), 0);
    for file in files {
        let mut edits: Vec<OneOf<TextEdit, AnnotatedTextEdit>> = Vec::new();
        let mut edit = |r: Range| {
            edits.push(OneOf::Right(AnnotatedTextEdit { text_edit: TextEdit::new(file.lines_of(r), String::new()), annotation_id: annotation.clone() }))
        };
        for el in &file.index.elements {
            if ids.contains(el.mrid.as_str()) {
                objects.insert(el.mrid.as_str());
                edit(el.full);
                continue;
            }
            for f in &el.fields {
                if f.resource.as_ref().is_some_and(|(m, _)| ids.contains(m.as_str())) {
                    references += 1;
                    edit(f.full);
                }
            }
        }
        if !edits.is_empty() {
            changes.push(TextDocumentEdit { text_document: OptionalVersionedTextDocumentIdentifier { uri: file.url.clone(), version: None }, edits });
        }
    }
    let description = format!("{} object(s), {references} reference(s) to them", objects.len());
    let annotations = HashMap::from([(annotation, ChangeAnnotation { label: label.into(), needs_confirmation: Some(true), description: Some(description) })]);
    Deletion {
        edit: WorkspaceEdit { document_changes: Some(DocumentChanges::Edits(changes)), change_annotations: Some(annotations), ..Default::default() },
        objects: objects.len(),
        references,
    }
}

/// A field to add: where, and where its value is once added.
pub struct Addition {
    pub url: Url,
    pub edit: TextEdit,
    /// The inserted value (or where it goes), to select after the edit.
    pub value: Range,
    /// A reference, inserted empty: `rdf:resource="#"`.
    pub reference: bool,
}

impl Addition {
    /// The edit with the reference to `c` filled in.
    pub fn naming(&self, c: &Candidate) -> TextEdit {
        TextEdit::new(self.edit.range, self.edit.new_text.replacen("rdf:resource=\"#\"", &format!("rdf:resource=\"{}\"", c.reference()), 1))
    }
}

/// The profile codes `file`'s header declares (`md:Model.profile`,
/// `dcterms:conformsTo`), by the schemas' profile tables.
fn profiles(file: &File) -> Vec<&'static str> {
    let reg = type_registry();
    let tables: Vec<&[(&str, &str)]> = ["cgmes", "nc"].iter().filter_map(|f| reg.schema(f)).map(|s| s.profiles).collect();
    let Some(header) = file.index.elements.iter().find(|e| matches!(local(&e.qname), "FullModel" | "Dataset" | "DifferenceModel")) else { return Vec::new() };
    header
        .fields
        .iter()
        .filter(|f| matches!(f.key(), "Model.profile" | "conformsTo"))
        .filter_map(|f| match &f.resource {
            Some((uri, _)) => Some(uri.as_str()),
            None => file.field_text(f.head, f.full, &f.qname).map(|(t, _)| t.trim()),
        })
        .filter_map(|uri| tables.iter().flat_map(|t| t.iter()).find(|(_, u)| *u == uri).map(|(code, _)| *code))
        .collect()
}

/// Add field `key` to object `id`, which writes it nowhere: in the file
/// (`prefer` first) whose profile carries it and that writes the object,
/// after its last field, with a value to type over — the object's mRID for
/// `IdentifiedObject.mRID`, `false`, `0`, an enumeration's first value, or
/// none. `None` if a file writes it already, its class does not declare
/// it, or no file fits.
pub fn add_missing(files: &[File], prefer: &Url, id: &str, key: &str) -> Option<Addition> {
    let reg = type_registry();
    let sites: Vec<(&File, &crate::index::Element)> =
        files.iter().flat_map(|f| f.index.elements.iter().filter(|e| e.mrid == id).map(move |e| (f, e))).collect();
    if sites.iter().any(|(_, e)| e.fields.iter().any(|f| f.key() == key)) {
        return None;
    }
    let (def_file, def) = sites.iter().max_by_key(|(_, e)| e.id_kind == Some(IdKind::Id))?;
    let class = class_of(def_file.index, &def.qname)?;
    let attr = reg.attr(class, key).filter(|a| a.used)?;
    let fits = |f: &File| {
        let declared = profiles(f);
        declared.is_empty() || attr.origins.is_empty() || declared.iter().any(|p| attr.origins.contains(p))
    };
    let (file, el) = sites
        .iter()
        .filter(|(f, _)| fits(f))
        .min_by_key(|(f, e)| (f.url != prefer, e.id_kind != Some(IdKind::Id)))?;
    let tag = format!("{}:{}", file.index.prefix_of(attr.ns)?, attr.id);
    let (open, value, close) = match attr.kind {
        AttrKind::Literal => {
            let value = match (attr.id, attr.xsd) {
                ("IdentifiedObject.mRID", _) => id.strip_prefix('_').unwrap_or(id).to_string(),
                (_, "boolean") => "false".into(),
                (_, "double" | "float" | "decimal" | "integer" | "int" | "long" | "short") => "0".into(),
                _ => String::new(),
            };
            (format!("<{tag}>"), value, format!("</{tag}>"))
        }
        AttrKind::Association => (format!("<{tag} rdf:resource=\"#"), String::new(), "\" />".into()),
        AttrKind::Enum => {
            let first = reg.family_of(class.qualified).and_then(|f| reg.schema(f)).and_then(|s| s.enumeration(attr.range)).and_then(|e| e.values.first());
            (format!("<{tag} rdf:resource=\"{}", attr.value_ns), first.map_or(String::new(), |v| v.id.to_string()), "\" />".into())
        }
    };
    // On a line of its own after the last field, indented as it is; else
    // inside the element, a step in from it.
    let (text, lines) = (file.text, file.lines);
    let indent_of = |r: Range| {
        let start = lines.offset(text, Position::new(r.start.line, 0));
        let at = lines.offset(text, r.start);
        text[start..at].chars().take_while(|c| c.is_whitespace()).collect::<String>()
    };
    let (at, indent) = match el.fields.last() {
        Some(f) => (f.full.end, indent_of(f.head)),
        None => {
            let end = lines.offset(text, el.full.end);
            let closing = end.checked_sub(el.qname.len() + 3).filter(|&c| text.get(c..end) == Some(&format!("</{}>", el.qname)))?;
            (lines.position(text, closing), format!("{}  ", indent_of(el.head)))
        }
    };
    // Lines end as the file's do.
    let eol = if text.contains("\r\n") { "\r\n" } else { "\n" };
    let new_text = if el.fields.is_empty() {
        format!("  {open}{value}{close}{eol}{}", indent_of(el.head))
    } else {
        format!("{eol}{indent}{open}{value}{close}")
    };
    let utf16 = |s: &str| s.encode_utf16().count() as u32;
    let start = if el.fields.is_empty() { Position::new(at.line, at.character + 2 + utf16(&open)) } else { Position::new(at.line + 1, utf16(&indent) + utf16(&open)) };
    let value_range = Range::new(start, Position::new(start.line, start.character + utf16(&value)));
    let reference = attr.kind == AttrKind::Association;
    Some(Addition { url: file.url.clone(), edit: TextEdit::new(Range::new(at, at), new_text), value: value_range, reference })
}

/// An object a reference can name.
pub struct Candidate {
    pub id: String,
    pub label: String,
    pub class: &'static str,
    /// Its containers' labels, as the outline shows them.
    pub place: String,
}

impl Candidate {
    /// The reference to it as written: `#_id`, or an IRI as it is.
    pub fn reference(&self) -> String {
        if self.id.contains(':') { self.id.clone() } else { format!("#{}", self.id) }
    }
}

/// The objects field `key` of object `id` can name — of the class it ranges
/// over, or below — at most `limit`, the likeliest first: those linked to
/// the object, naming what it names with its other fields or named by it
/// (a terminal's connectivity node, of its topological node; equipment's
/// base voltage, its voltage level's); then those that share the most
/// containers with it, by where they are or by where the objects are that
/// name them with the same field (a line's base voltage is the one the
/// equipment of its voltage level names); then the most named.
/// Empty if `key` is not a reference of the object's class.
pub fn candidates(indexes: &[&Index], tree: &crate::tree::Tree, id: &str, key: &str, limit: usize) -> Vec<Candidate> {
    let reg = type_registry();
    let class = indexes
        .iter()
        .flat_map(|ix| ix.elements.iter().filter(|e| e.mrid == id).map(move |e| (ix, e)))
        .max_by_key(|(_, e)| e.id_kind == Some(IdKind::Id))
        .and_then(|(ix, e)| class_of(ix, &e.qname));
    let Some(attr) = class.and_then(|c| reg.attr(c, key)).filter(|a| a.kind == AttrKind::Association) else { return Vec::new() };
    // What the object names with its other fields.
    let own: HashSet<&str> = indexes
        .iter()
        .flat_map(|ix| ix.elements.iter().filter(|e| e.mrid == id))
        .flat_map(|e| e.fields.iter().filter(|f| f.key() != key).filter_map(|f| f.resource.as_ref().map(|(t, _)| t.as_str())))
        .filter(|t| !t.is_empty())
        .collect();
    let mut linked: HashSet<&str> = HashSet::new();
    let mut found: HashMap<&str, (Option<&str>, &'static str)> = HashMap::new();
    let mut named: HashMap<&str, Vec<&str>> = HashMap::new();
    for ix in indexes {
        for el in &ix.elements {
            let targets = || el.fields.iter().filter_map(|f| f.resource.as_ref().map(|(t, _)| t.as_str()));
            if own.contains(el.mrid.as_str()) {
                linked.extend(targets());
            } else if targets().any(|t| own.contains(t)) {
                linked.insert(el.mrid.as_str());
            }
            for f in el.fields.iter().filter(|f| f.key() == key && el.mrid != id) {
                if let Some((target, _)) = &f.resource {
                    named.entry(target.as_str()).or_default().push(el.mrid.as_str());
                }
            }
            if el.mrid.is_empty() || el.mrid == id {
                continue;
            }
            let Some(c) = class_of(ix, &el.qname).filter(|c| reg.chain(c).iter().any(|a| a.local == attr.range)) else { continue };
            let entry = found.entry(el.mrid.as_str()).or_insert((None, c.local));
            if entry.0.is_none() {
                entry.0 = el.name.as_deref().filter(|n| !n.is_empty());
            }
        }
    }
    let here = {
        let mut chain = tree.chain(id);
        chain.pop();
        chain
    };
    let shared = |chain: &[u32]| here.iter().zip(chain).take_while(|(a, b)| a == b).count();
    let mut ranked: Vec<(bool, usize, usize, String, &str, &'static str)> = found
        .into_iter()
        .map(|(c, (name, class))| {
            let by = named.get(c).map_or(&[][..], Vec::as_slice);
            let near = by.iter().map(|r| {
                let mut chain = tree.chain(r);
                chain.pop();
                shared(&chain)
            });
            let score = near.chain(std::iter::once(shared(&tree.chain(c)))).max().unwrap_or(0);
            (linked.contains(c), score, by.len(), name.unwrap_or(c).to_string(), c, class)
        })
        .collect();
    ranked.sort_by(|a, b| b.0.cmp(&a.0).then(b.1.cmp(&a.1)).then(b.2.cmp(&a.2)).then_with(|| a.3.to_lowercase().cmp(&b.3.to_lowercase())));
    ranked
        .into_iter()
        .take(limit)
        .map(|(_, _, _, label, c, class)| Candidate { id: c.to_string(), label, class, place: tree.containers(c).join(" › ") })
        .collect()
}

/// A class that can be added in an object, and the field naming it there.
pub struct Addable {
    pub class: &'static cimmodel::base::ClassDef,
    pub key: &'static str,
}

/// The class of object `id` as the file defining it writes it.
fn class_in(files: &[File], id: &str) -> Option<&'static cimmodel::base::ClassDef> {
    let (file, el) = files
        .iter()
        .flat_map(|f| f.index.elements.iter().filter(|e| e.mrid == id).map(move |e| (f, e)))
        .max_by_key(|(_, e)| e.id_kind == Some(IdKind::Id))?;
    class_of(file.index, &el.qname)
}

/// The concrete classes of `parent`'s family whose objects the containment
/// puts in it: the first of [`crate::tree::CONTAINED`] they declare that
/// ranges over its class. Only classes of the profiles the set's files
/// declare (all, if none declares one).
pub fn addable(files: &[File], parent: &str) -> Vec<Addable> {
    let reg = type_registry();
    let Some(class) = class_in(files, parent) else { return Vec::new() };
    let Some(schema) = reg.family_of(class.qualified).and_then(|f| reg.schema(f)) else { return Vec::new() };
    let is = |range: &str| reg.chain(class).iter().any(|c| c.local == range);
    let declared: HashSet<&str> = files.iter().flat_map(profiles).collect();
    let mut out: Vec<Addable> = schema
        .classes
        .iter()
        .filter(|c| c.concrete && (declared.is_empty() || c.origins.iter().any(|o| declared.contains(o))))
        .filter_map(|c| {
            let key = crate::tree::CONTAINED.iter().find_map(|k| reg.attr(c, k).filter(|a| a.used && a.kind == AttrKind::Association))?;
            is(key.range).then_some(Addable { class: c, key: key.id })
        })
        .collect();
    out.sort_by_key(|a| a.class.local);
    out
}

/// A random (version 4) UUID, from the standard library's hasher keys and the clock.
fn uuid() -> String {
    use std::hash::{BuildHasher, Hasher};
    let now = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map_or(0, |d| d.as_nanos());
    let mut b = [0u8; 16];
    for (i, chunk) in b.chunks_mut(8).enumerate() {
        let mut h = std::collections::hash_map::RandomState::new().build_hasher();
        h.write_u128(now);
        h.write_usize(i);
        chunk.copy_from_slice(&h.finish().to_le_bytes());
    }
    b[6] = (b[6] & 0x0f) | 0x40;
    b[8] = (b[8] & 0x3f) | 0x80;
    let hex: String = b.iter().map(|x| format!("{x:02x}")).collect();
    format!("{}-{}-{}-{}-{}", &hex[..8], &hex[8..12], &hex[12..16], &hex[16..20], &hex[20..])
}

/// An object added: the edit, and where its name (else its start tag) is.
pub struct Added {
    pub edit: WorkspaceEdit,
    pub url: Url,
    pub select: Range,
    pub id: String,
}

/// Add an object of `class` in `parent`, named `name` if given: an
/// `rdf:ID`, its mRID, name and the field naming `parent`, in the file
/// whose profile carries the class (the parent's own first), after the last
/// of the parent's objects there, else after the parent, else at the end;
/// and where the set has an SSH file and the class SSH attributes, an empty
/// `rdf:about` there for them. What else it needs, validation asks for.
pub fn add(files: &[File], tree: &crate::tree::Tree, parent: &str, class: &str, name: Option<&str>) -> Result<Added, String> {
    let reg = type_registry();
    let found = addable(files, parent);
    let Addable { class, key } = found.into_iter().find(|a| a.class.local == class).ok_or_else(|| format!("a {class} cannot be added there"))?;
    let fits = |f: &File| {
        let declared = profiles(f);
        declared.is_empty() || declared.iter().any(|p| class.origins.contains(p))
    };
    let defines = |f: &File, id: &str| f.index.elements.iter().any(|e| e.mrid == id && e.id_kind == Some(IdKind::Id));
    let main = files
        .iter()
        .filter(|f| fits(f) && !profiles(f).contains(&"SSH"))
        .max_by_key(|f| defines(f, parent))
        .ok_or_else(|| format!("no file of the model set has a profile for {}", class.local))?;
    let mut id = format!("_{}", uuid());
    while files.iter().any(|f| f.index.elements.iter().any(|e| e.mrid == id)) {
        id = format!("_{}", uuid());
    }

    let prefix = |f: &File, ns: &str| f.index.prefix_of(ns).map(str::to_string);
    let tag = format!("{}:{}", prefix(main, class.ns).ok_or("the file binds no prefix to the class's namespace")?, class.local);
    let field = |f: &File, key: &str| reg.attr(class, key).and_then(|a| Some((a, format!("{}:{}", prefix(f, a.ns)?, a.id))));
    let carries = |f: &File, key: &str| {
        let declared = profiles(f);
        reg.attr(class, key).is_some_and(|a| declared.is_empty() || a.origins.iter().any(|o| declared.contains(o)))
    };
    let mut lines = vec![format!("<{tag} rdf:ID=\"{id}\">")];
    let mut select_line = None;
    if carries(main, "IdentifiedObject.mRID")
        && let Some((_, t)) = field(main, "IdentifiedObject.mRID")
    {
        lines.push(format!("  <{t}>{}</{t}>", &id[1..]));
    }
    if let Some(name) = name.map(str::trim).filter(|n| !n.is_empty())
        && let Some((_, t)) = field(main, "IdentifiedObject.name")
    {
        select_line = Some((lines.len(), t.len() + 4, name.encode_utf16().count()));
        let escaped = name.replace('&', "&amp;").replace('<', "&lt;");
        lines.push(format!("  <{t}>{escaped}</{t}>"));
    }
    let (_, container) = field(main, key).ok_or("the file binds no prefix to the field's namespace")?;
    let target = if parent.contains(':') { parent.to_string() } else { format!("#{parent}") };
    lines.push(format!("  <{container} rdf:resource=\"{target}\" />"));
    lines.push(format!("</{tag}>"));

    let mut changes = HashMap::new();
    let (at, line_no) = insertion(main, tree, parent, &lines)?;
    let select = match select_line {
        Some((l, col, len)) => {
            let start = Position::new(line_no + l as u32, at.1 + col as u32);
            Range::new(start, Position::new(start.line, start.character + len as u32))
        }
        None => Range::new(Position::new(line_no, at.1), Position::new(line_no, at.1)),
    };
    changes.insert(main.url.clone(), vec![at.0]);
    // Its steady-state part.
    if class.origins.contains(&"SSH")
        && let Some(ssh) = files.iter().find(|f| profiles(f).contains(&"SSH"))
        && let Some(p) = prefix(ssh, class.ns)
    {
        let tag = format!("{p}:{}", class.local);
        let ssh_lines = vec![format!("<{tag} rdf:about=\"#{id}\">"), format!("</{tag}>")];
        let (edit, _) = insertion(ssh, tree, parent, &ssh_lines)?;
        changes.entry(ssh.url.clone()).or_insert_with(Vec::new).push(edit.0);
    }
    Ok(Added { edit: WorkspaceEdit { changes: Some(changes), ..Default::default() }, url: main.url.clone(), select, id })
}

/// Where to insert `lines` (an element, unindented) in `file`: after the
/// last element there of an object `parent` holds, else after `parent`'s,
/// else before the root's end tag; indented as the elements are, in the
/// file's line endings. The edit with the indent's width, and the line the
/// element starts on.
fn insertion(file: &File, tree: &crate::tree::Tree, parent: &str, lines: &[String]) -> Result<((TextEdit, u32), u32), String> {
    let (text, li) = (file.text, file.lines);
    let eol = if text.contains("\r\n") { "\r\n" } else { "\n" };
    let siblings: HashSet<&str> = tree.children(parent).into_iter().collect();
    let anchor = file.index.elements.iter().rev().find(|e| siblings.contains(e.mrid.as_str()))
        .or_else(|| file.index.elements.iter().rev().find(|e| e.mrid == parent));
    let (offset, indent) = match anchor {
        Some(el) => {
            let end = li.offset(text, el.full.end);
            let line_end = text[end..].find('\n').map_or(text.len(), |i| end + i + 1);
            let head = li.offset(text, el.head.start);
            let line_start = text[..head].rfind('\n').map_or(0, |i| i + 1);
            (line_end, text[line_start..head].to_string())
        }
        None => {
            let root_end = text.rfind("</").ok_or("the file has no root end tag")?;
            let line_start = text[..root_end].rfind('\n').map_or(0, |i| i + 1);
            let indent = file.index.elements.last().map_or("  ".to_string(), |el| {
                let head = li.offset(text, el.head.start);
                text[text[..head].rfind('\n').map_or(0, |i| i + 1)..head].to_string()
            });
            (line_start, indent)
        }
    };
    // An anchor ending the file without a newline: start a line first.
    let lead = if offset == text.len() && !text.ends_with('\n') { eol } else { "" };
    let body: String = lines.iter().map(|l| format!("{indent}{l}{eol}")).collect();
    let at = li.position(text, offset);
    let first = if lead.is_empty() { at.line } else { at.line + 1 };
    Ok(((TextEdit::new(Range::new(at, at), format!("{lead}{body}")), indent.encode_utf16().count() as u32), first))
}

/// A fix for a reference to nothing (`rdf:resource="#"`) or to an object the
/// set lacks: name one of the likeliest objects instead.
pub struct ReferenceFix {
    pub key: String,
    pub url: Url,
    pub edit: TextEdit,
    pub title: String,
    /// It names nothing; else something the set does not have.
    pub empty: bool,
}

/// The fixes for object `id`'s references to nothing or to objects the set
/// lacks, in every file that writes it, or only the field at `at`: each the
/// five likeliest objects ([`candidates`]). An enumeration value is not a
/// reference to an object, so it has none.
pub fn reference_fixes(files: &[File], tree: &crate::tree::Tree, id: &str, at: Option<(&Url, Position)>) -> Vec<ReferenceFix> {
    let sites: Vec<(&File, &crate::index::Field, &str, Range)> = files
        .iter()
        .filter(|f| at.is_none_or(|(u, _)| f.url == u))
        .flat_map(|f| f.index.elements.iter().filter(|e| e.mrid == id).map(move |e| (f, e)))
        .flat_map(|(f, e)| e.fields.iter().map(move |x| (f, x)))
        .filter(|(_, x)| at.is_none_or(|(_, pos)| contains(x.full, pos)))
        .filter_map(|(f, x)| x.resource.as_ref().map(|(t, r)| (f, x, t.as_str(), *r)))
        .collect();
    if sites.is_empty() {
        return Vec::new();
    }
    let known: HashSet<&str> = files.iter().flat_map(|f| f.index.elements.iter().map(|e| e.mrid.as_str())).collect();
    let indexes: Vec<&Index> = files.iter().map(|f| f.index).collect();
    let mut out = Vec::new();
    for (file, field, target, value) in sites {
        if !target.is_empty() && known.contains(target) {
            continue;
        }
        for c in candidates(&indexes, tree, id, field.key(), 5) {
            let place = if c.place.is_empty() { String::new() } else { format!(" in {}", c.place) };
            out.push(ReferenceFix {
                key: field.key().to_string(),
                url: file.url.clone(),
                edit: TextEdit::new(value, c.reference()),
                title: format!("Set {} to {} {}{place}", field.key(), c.class, c.label),
                empty: target.is_empty(),
            });
        }
    }
    out
}

/// The objects that refer to object `id`, and by which field, as the files
/// write them: (referrer, field), each once.
pub fn referrers<'a>(files: &[File<'a>], id: &str) -> Vec<(&'a str, &'a str)> {
    let mut out: Vec<(&str, &str)> = files
        .iter()
        .flat_map(|f| f.index.elements.iter())
        .flat_map(|e| e.fields.iter().filter(|x| x.resource.as_ref().is_some_and(|(t, _)| t == id)).map(move |x| (e.mrid.as_str(), x.key())))
        .filter(|(r, _)| !r.is_empty() && *r != id)
        .collect();
    out.sort_unstable();
    out.dedup();
    out
}

/// How an object is named in a title: its class and name, else mRID.
pub fn describe(files: &[File], id: &str) -> String {
    let el = files.iter().flat_map(|f| &f.index.elements).filter(|e| e.mrid == id).max_by_key(|e| e.id_kind == Some(IdKind::Id));
    match el {
        Some(el) => format!("{} {}", local(&el.qname), el.name.as_deref().filter(|n| !n.is_empty()).unwrap_or(id)),
        None => id.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EQ: &str = r##"<rdf:RDF xmlns:cim="http://iec.ch/TC57/CIM100#" xmlns:rdf="http://www.w3.org/1999/02/22-rdf-syntax-ns#">
  <cim:Breaker rdf:ID="_b1">
    <cim:IdentifiedObject.mRID>b1</cim:IdentifiedObject.mRID>
    <cim:IdentifiedObject.name>B1</cim:IdentifiedObject.name>
  </cim:Breaker>
  <cim:Terminal rdf:ID="_t1">
    <cim:Terminal.ConductingEquipment rdf:resource="#_b1" />
    <cim:IdentifiedObject.name>T1</cim:IdentifiedObject.name>
  </cim:Terminal>
</rdf:RDF>"##;
    const SSH: &str = r##"<rdf:RDF xmlns:cim="http://iec.ch/TC57/CIM100#" xmlns:rdf="http://www.w3.org/1999/02/22-rdf-syntax-ns#">
  <cim:Breaker rdf:about="#_b1">
    <cim:Switch.open>false</cim:Switch.open>
  </cim:Breaker>
</rdf:RDF>"##;

    /// Apply `edits` to `text`, last first.
    fn apply(text: &str, mut edits: Vec<TextEdit>) -> String {
        let lines = LineIndex::new(text);
        edits.sort_by_key(|e| std::cmp::Reverse(lines.offset(text, e.range.start)));
        let mut out = text.to_string();
        for e in edits {
            let (s, t) = (lines.offset(text, e.range.start), lines.offset(text, e.range.end));
            out.replace_range(s..t, &e.new_text);
        }
        out
    }

    fn with_files<R>(f: impl FnOnce(&[File]) -> R) -> R {
        let urls = [Url::parse("file:///m/EQ.xml").unwrap(), Url::parse("file:///m/SSH.xml").unwrap()];
        let texts = [EQ, SSH];
        let lines: Vec<LineIndex> = texts.iter().map(|t| LineIndex::new(t)).collect();
        let indexes: Vec<Index> = texts.iter().map(|t| Index::build(t)).collect();
        let files: Vec<File> = (0..2).map(|i| File { url: &urls[i], text: texts[i], lines: &lines[i], index: &indexes[i] }).collect();
        f(&files)
    }

    #[test]
    fn renames_an_mrid_everywhere() {
        with_files(|files| {
            // On the reference, and on the start tag.
            let (r, old) = prepare_rename(files, files[0].url, Position::new(6, 55)).unwrap();
            assert_eq!((old.as_str(), r), ("_b1", Range::new(Position::new(6, 53), Position::new(6, 56))));
            assert_eq!(prepare_rename(files, files[1].url, Position::new(1, 4)).unwrap().1, "_b1");
            assert!(prepare_rename(files, files[0].url, Position::new(2, 10)).is_none());

            let edit = rename(files, "_b1", "_b2").unwrap();
            let changes = edit.changes.unwrap();
            let eq = apply(EQ, changes[files[0].url].clone());
            assert!(eq.contains(r#"rdf:ID="_b2""#) && eq.contains(">b2</") && eq.contains(r##"rdf:resource="#_b2""##), "{eq}");
            assert!(!eq.contains("b1"), "{eq}");
            assert_eq!(apply(SSH, changes[files[1].url].clone()), SSH.replace("#_b1", "#_b2"));

            assert!(rename(files, "_b1", "_t1").is_err());
            assert!(rename(files, "_b1", "a b").is_err());
            assert!(rename(files, "_b1", "1b").is_err());
        });
    }

    #[test]
    fn adds_a_missing_field_where_its_profile_is() {
        let header = |p: &str| format!(r#"  <md:FullModel rdf:about="urn:uuid:{p}">
    <md:Model.profile>http://iec.ch/TC57/ns/CIM/{p}/3.0</md:Model.profile>
  </md:FullModel>
"#);
        let with_header = |text: &str, p: &str| {
            text.replacen(r#"xmlns:rdf"#, r#"xmlns:md="http://iec.ch/TC57/61970-552/ModelDescription/1#" xmlns:rdf"#, 1)
                .replacen(">\n", &format!(">\n{}", header(p)), 1)
        };
        let texts = [with_header(EQ, "CoreEquipment-EU"), with_header(SSH, "SteadyStateHypothesis-EU")];
        let urls = [Url::parse("file:///m/EQ.xml").unwrap(), Url::parse("file:///m/SSH.xml").unwrap()];
        let lines: Vec<LineIndex> = texts.iter().map(|t| LineIndex::new(t)).collect();
        let indexes: Vec<Index> = texts.iter().map(|t| Index::build(t)).collect();
        let files: Vec<File> = (0..2).map(|i| File { url: &urls[i], text: &texts[i], lines: &lines[i], index: &indexes[i] }).collect();
        let added = |id: &str, key: &str| {
            let a = add_missing(&files, &urls[0], id, key)?;
            let i = urls.iter().position(|u| *u == a.url).unwrap();
            let out = apply(&texts[i], vec![a.edit.clone()]);
            let selected = apply(&out, vec![TextEdit::new(a.value, "|".into())]);
            Some((i, out, selected))
        };

        // An SSH attribute goes to the SSH file, though asked from the EQ one.
        let (i, out, selected) = added("_b1", "Equipment.inService").unwrap();
        assert_eq!(i, 1);
        assert!(out.contains("<cim:Switch.open>false</cim:Switch.open>\n    <cim:Equipment.inService>false</cim:Equipment.inService>\n  </cim:Breaker>"), "{out}");
        assert!(selected.contains("<cim:Equipment.inService>|</cim:Equipment.inService>"), "{selected}");
        // An EQ one to the EQ file; a reference left to fill in, an enumeration its first value.
        let (i, _, selected) = added("_t1", "Terminal.ConnectivityNode").unwrap();
        assert_eq!(i, 0);
        assert!(selected.contains(r##"<cim:Terminal.ConnectivityNode rdf:resource="#|" />"##), "{selected}");
        let (_, out, _) = added("_t1", "Terminal.phases").unwrap();
        assert!(out.contains(r#"<cim:Terminal.phases rdf:resource="http://iec.ch/TC57/CIM100#PhaseCode."#), "{out}");
        // Written already, or not the class's.
        assert!(added("_b1", "Switch.open").is_none());
        assert!(added("_b1", "ACLineSegment.r").is_none());
    }

    #[test]
    fn lists_what_refers_to_an_object() {
        with_files(|files| {
            assert_eq!(referrers(files, "_b1"), [("_t1", "Terminal.ConductingEquipment")]);
            assert!(referrers(files, "_t1").is_empty());
        });
    }

    #[test]
    fn deletes_an_object_from_every_file() {
        with_files(|files| {
            let d = delete(files, &HashSet::from(["_b1"]), "Delete");
            assert_eq!((d.objects, d.references), (1, 1));
            let Some(DocumentChanges::Edits(changes)) = d.edit.document_changes else { panic!() };
            let plain = |i: usize| changes.iter().find(|c| c.text_document.uri == *files[i].url).unwrap().edits.iter()
                .map(|e| match e { OneOf::Left(t) => t.clone(), OneOf::Right(a) => a.text_edit.clone() }).collect::<Vec<_>>();
            let eq = apply(EQ, plain(0));
            assert!(!eq.contains("Breaker") && !eq.contains("ConductingEquipment") && eq.contains("<cim:Terminal rdf:ID=\"_t1\">\n    <cim:IdentifiedObject.name>T1"), "{eq}");
            assert!(!apply(SSH, plain(1)).contains("Breaker"));
        });
    }

    /// On MicroGrid, the object each reference names is among the
    /// candidates offered for it, mostly first.
    #[test]
    fn candidates_rank_what_the_data_names() {
        use std::path::Path;
        use crate::validate::{Options, collect, validate};
        let dir = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../CGMES-Test-Configurations/v3.0/MicroGrid/MicroGid-BaseCase/MicroGrid-BaseCase-Merged");
        let files = collect(&dir, &Default::default());
        let (set, _) = validate(&files, &Options::default());
        let indexes: Vec<&Index> = set.files.iter().map(|f| &f.index).collect();
        let mut stats: HashMap<&str, (usize, usize, usize)> = HashMap::new();
        for ix in &indexes {
            for el in &ix.elements {
                for f in &el.fields {
                    let key = f.key();
                    let Some((target, _)) = &f.resource else { continue };
                    // Not the containment's own fields: the object is where they put it.
                    if !["ConductingEquipment.BaseVoltage", "Terminal.ConnectivityNode", "Terminal.TopologicalNode", "RegulatingControl.Terminal"].contains(&key) {
                        continue;
                    }
                    let c = candidates(&indexes, &set.tree, &el.mrid, key, 5);
                    let s = stats.entry(key).or_default();
                    s.0 += 1;
                    if c.first().is_some_and(|c| c.id == *target) { s.1 += 1 }
                    if c.iter().any(|c| c.id == *target) { s.2 += 1 }
                }
            }
        }
        for (key, (n, first, top5)) in &stats {
            eprintln!("{key}: {n} references, first {first}, top 5 {top5}");
        }
        // Linked through the topological node: always first. A base voltage:
        // a line's, among its sub-region's, not always.
        assert_eq!(stats["Terminal.ConnectivityNode"], (73, 73, 73));
        assert_eq!(stats["Terminal.TopologicalNode"], (73, 73, 73));
        assert_eq!(stats["RegulatingControl.Terminal"], (7, 7, 7));
        assert_eq!(stats["ConductingEquipment.BaseVoltage"], (23, 20, 23));
    }

    /// On MicroGrid: what a voltage level and a transformer can hold; a breaker
    /// added in a voltage level, in EQ with an SSH part, which validates
    /// with findings for what it lacks and sits in the voltage level.
    #[test]
    fn adds_an_object_in_a_container() {
        use std::path::Path;
        use crate::validate::{Options, collect, validate};
        let dir = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../CGMES-Test-Configurations/v3.0/MicroGrid/MicroGid-BaseCase/MicroGrid-BaseCase-Merged");
        let read = collect(&dir, &Default::default());
        let (set, _) = validate(&read, &Options::default());
        let texts: Vec<String> = set.files.iter().map(|f| std::fs::read_to_string(&f.path).unwrap()).collect();
        let lines: Vec<LineIndex> = texts.iter().map(|t| LineIndex::new(t)).collect();
        let files: Vec<File> = set.files.iter().enumerate().map(|(i, f)| File { url: &f.url, text: &texts[i], lines: &lines[i], index: &f.index }).collect();
        let find = |class: &str, name: &str| set.files.iter().flat_map(|f| &f.index.elements)
            .find(|e| local(&e.qname) == class && e.name.as_deref() == Some(name)).unwrap().mrid.clone();
        let vl = find("VoltageLevel", "110.0");
        let classes: Vec<&str> = addable(&files, &vl).iter().map(|a| a.class.local).collect();
        for c in ["Breaker", "Bay", "ConnectivityNode", "LinearShuntCompensator"] {
            assert!(classes.contains(&c), "{classes:?}");
        }
        assert!(!classes.contains(&"Terminal"), "{classes:?}");
        let transformer = find("PowerTransformer", "BE-TR2_1");
        let classes: Vec<&str> = addable(&files, &transformer).iter().map(|a| a.class.local).collect();
        assert!(classes.contains(&"Terminal") && classes.contains(&"PowerTransformerEnd"), "{classes:?}");

        let added = add(&files, &set.tree, &vl, "Breaker", Some("B-new")).unwrap();
        let changes = added.edit.changes.unwrap();
        assert_eq!(changes.len(), 2, "EQ and SSH");
        let edited: Vec<(std::path::PathBuf, std::sync::Arc<str>)> = set.files.iter().enumerate().map(|(i, f)| {
            let text = match changes.get(&f.url) { Some(e) => apply(&texts[i], e.clone()), None => texts[i].clone() };
            (f.path.clone(), std::sync::Arc::from(text))
        }).collect();
        let main = set.files.iter().position(|f| f.url == added.url).unwrap();
        assert!(edited[main].0.to_string_lossy().contains("_EQ_"));
        let text = &edited[main].1;
        let selected = apply(text, vec![TextEdit::new(added.select, "|".into())]);
        assert!(selected.contains("<cim:IdentifiedObject.name>|</cim:IdentifiedObject.name>"), "the name is selected");

        let (set2, _) = validate(&edited, &Options::default());
        assert_eq!(set2.tree.containers(&added.id), ["PP_Brussels", "110.0"]);
        let ssh = edited.iter().find(|(p, _)| p.to_string_lossy().contains("_BE_SSH_")).unwrap();
        assert!(ssh.1.contains(&format!("rdf:about=\"#{}\"", added.id)));
    }
}

