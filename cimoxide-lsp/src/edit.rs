//! cimoxide-view's edits across a model set, as LSP workspace edits: its
//! files are named by their URLs.

use std::collections::{HashMap, HashSet};

use lsp_types::{
    AnnotatedTextEdit, ChangeAnnotation, DocumentChanges, OneOf, OptionalVersionedTextDocumentIdentifier, TextDocumentEdit,
    Url, WorkspaceEdit,
};

pub use cimview::edit::{Addition, File, addable, candidates, describe, prepare_rename, reference_fixes, referrers};

use crate::index::lsp_edit;

/// `edits`, by the URL each file is named by, as plain changes.
pub fn workspace(edits: cimview::edit::Edits) -> WorkspaceEdit {
    let changes: HashMap<Url, Vec<lsp_types::TextEdit>> = edits
        .into_iter()
        .filter_map(|(name, e)| Some((Url::parse(name).ok()?, e.into_iter().map(lsp_edit).collect())))
        .collect();
    WorkspaceEdit { changes: Some(changes), ..Default::default() }
}

/// Rename mRID `old` to `new` in every file ([`cimview::edit::rename`]).
pub fn rename(files: &[File], old: &str, new: &str) -> Result<WorkspaceEdit, String> {
    cimview::edit::rename(files, old, new).map(workspace)
}

/// What deleting objects changes ([`cimview::edit::delete`]), annotated for the client to ask
/// before applying it (`label`).
pub struct Deletion {
    pub edit: WorkspaceEdit,
    pub objects: usize,
    pub references: usize,
}

pub fn delete(files: &[File], ids: &HashSet<&str>, label: &str) -> Deletion {
    let found = cimview::edit::delete(files, ids);
    let annotation = "delete".to_string();
    let changes = found
        .edits
        .into_iter()
        .filter_map(|(name, edits)| {
            Some(TextDocumentEdit {
                text_document: OptionalVersionedTextDocumentIdentifier { uri: Url::parse(name).ok()?, version: None },
                edits: edits
                    .into_iter()
                    .map(|e| OneOf::Right(AnnotatedTextEdit { text_edit: lsp_edit(e), annotation_id: annotation.clone() }))
                    .collect(),
            })
        })
        .collect();
    let description = format!("{} object(s), {} reference(s) to them", found.objects, found.references);
    let annotations = HashMap::from([(annotation, ChangeAnnotation { label: label.into(), needs_confirmation: Some(true), description: Some(description) })]);
    Deletion {
        edit: WorkspaceEdit { document_changes: Some(DocumentChanges::Edits(changes)), change_annotations: Some(annotations), ..Default::default() },
        objects: found.objects,
        references: found.references,
    }
}

/// Add field `key` to object `id` ([`cimview::edit::add_missing`]), `prefer` first.
pub fn add_missing(files: &[File], prefer: &Url, id: &str, key: &str) -> Option<Addition> {
    cimview::edit::add_missing(files, prefer.as_str(), id, key)
}

/// An object added: the edit, and where its name (else its start tag) is.
pub struct Added {
    pub edit: WorkspaceEdit,
    pub url: Url,
    pub select: lsp_types::Range,
    pub id: String,
}

/// 16 random bytes, from the standard library's hasher keys and the clock.
fn random() -> [u8; 16] {
    use std::hash::{BuildHasher, Hasher};
    let now = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map_or(0, |d| d.as_nanos());
    let mut b = [0u8; 16];
    for (i, chunk) in b.chunks_mut(8).enumerate() {
        let mut h = std::collections::hash_map::RandomState::new().build_hasher();
        h.write_u128(now);
        h.write_usize(i);
        chunk.copy_from_slice(&h.finish().to_le_bytes());
    }
    b
}

/// Add an object of `class` in `parent` ([`cimview::edit::add`]).
pub fn add(files: &[File], tree: &cimview::tree::Tree, parent: &str, class: &str, name: Option<&str>) -> Result<Added, String> {
    let added = cimview::edit::add(files, tree, parent, class, name, &mut random)?;
    let url = Url::parse(&added.file).map_err(|e| e.to_string())?;
    Ok(Added { edit: workspace(added.edits), url, select: crate::index::lsp(added.select), id: added.id })
}
