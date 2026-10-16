//! Edits across a model set, as text edits for the client to apply: renaming
//! an mRID and deleting objects. They change the files as written, keeping
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
}
