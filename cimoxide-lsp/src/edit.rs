//! cimoxide-view's edits across a model set, as LSP workspace edits: its
//! files are named by their URLs.

use std::collections::{HashMap, HashSet};

use lsp_types::{DocumentChanges, OneOf, OptionalVersionedTextDocumentIdentifier, TextDocumentEdit, Url, WorkspaceEdit};

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

/// What deleting objects changes ([`cimview::edit::delete`]): a plain edit, which the client
/// applies and saves once the user has confirmed the deletion.
pub struct Deletion {
    pub edit: WorkspaceEdit,
    pub objects: usize,
    pub references: usize,
}

pub fn delete(files: &[File], ids: &HashSet<&str>) -> Deletion {
    let found = cimview::edit::delete(files, ids);
    Deletion { edit: workspace(found.edits), objects: found.objects, references: found.references }
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
pub fn random() -> [u8; 16] {
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

/// Each file `texts` changes from what `files` hold, replaced whole: an edit made in steps, the
/// later ones on the texts the first left.
pub fn replaced(files: &[File], texts: &[(String, String)]) -> WorkspaceEdit {
    let changes: HashMap<Url, Vec<lsp_types::TextEdit>> = files
        .iter()
        .zip(texts)
        .filter(|(f, (_, t))| f.text != t.as_str())
        .filter_map(|(f, (_, t))| {
            let end = f.lines.position(f.text, f.text.len());
            let range = cimview::text::Range::new(cimview::text::Position::new(0, 0), end);
            Some((Url::parse(f.name).ok()?, vec![lsp_edit(cimview::text::TextEdit::new(range, t.clone()))]))
        })
        .collect();
    WorkspaceEdit { changes: Some(changes), ..Default::default() }
}

/// A topology edit inside `substation` ([`cimview::topology::apply`]); with `diagram`, that DL
/// diagram drawn anew with it ([`cimview::topology::redraw`]). The edit, and what it added.
pub fn topology(
    files: &[File],
    tree: &cimview::tree::Tree,
    grid: &cimview::grid::Grid,
    substation: &str,
    op: &serde_json::Value,
    diagram: Option<&str>,
    at: Option<(f64, f64)>,
) -> Result<(WorkspaceEdit, Option<String>), String> {
    let inside = grid.inside_of(substation).ok_or_else(|| format!("no substation `{substation}`"))?;
    let done = cimview::topology::apply(files, tree, inside, op, &mut random)?;
    let Some(diagram) = diagram else { return Ok((workspace(done.edits), done.select)) };
    let mut texts = cimview::topology::after(files, &done.edits);
    for (name, edits) in cimview::topology::redraw(&texts, substation, diagram, at, &mut random)? {
        if let Some((_, t)) = texts.iter_mut().find(|(n, _)| *n == name) {
            *t = cimview::text::apply(t, &edits);
        }
    }
    Ok((replaced(files, &texts), done.select))
}

/// Substation `substation`'s layout `layout` (positions in the diagram's coordinates) written as
/// a DL diagram: into `existing`, else as a new diagram `name` in the set's DL file, else in a new
/// DL file beside the EQ file defining the substation. The edit, and the diagram's mRID.
pub fn diagram(
    files: &[File],
    grid: &cimview::grid::Grid,
    substation: &str,
    layout: &serde_json::Value,
    existing: Option<&str>,
    name: &str,
) -> Result<(WorkspaceEdit, String), String> {
    let inside = grid.inside_of(substation).ok_or_else(|| format!("no substation `{substation}`"))?;
    let l = inside.layout_from_json(layout);
    let objects: Vec<cimview::edit::Drawn> = cimview::single_line::diagram_objects(inside, &l)
        .into_iter()
        .filter(|(m, _, _)| existing.is_none() || !inside.items.iter().any(|it| it.id == *m && !it.far.is_empty()))
        .collect();
    if objects.is_empty() {
        return Err("nothing to draw".into());
    }
    let says = |text: &str, what: &str| {
        let head = text.find("FullModel").map_or("", |s| {
            let rest = &text[s..];
            &rest[..rest[1..].find("FullModel").map_or(rest.len(), |e| e + 1)]
        });
        head.contains(what)
    };
    let dl = files.iter().find(|f| says(f.text, "DiagramLayout"));
    if existing.is_some() || dl.is_some() {
        let into = dl.map_or("", |f| f.name);
        let written = cimview::edit::diagram(files, into, existing, name, &objects, &mut random)?;
        return Ok((workspace(written.edits), written.id));
    }
    // A new DL file, beside the EQ file defining the substation.
    let like = files
        .iter()
        .find(|f| f.index.elements.iter().any(|e| e.mrid == substation && e.id_kind == Some(cimview::index::IdKind::Id)))
        .ok_or_else(|| format!("no file defines `{substation}`"))?;
    let text = cimview::edit::companion(like, "DL", &cimview::edit::uuid(random()))?;
    let url = Url::parse(like.name).map_err(|e| e.to_string())?;
    let path = url.to_file_path().map_err(|_| "not a file".to_string())?;
    let stem = path.file_stem().and_then(|s| s.to_str()).unwrap_or("model");
    let base = if stem.contains("_EQ") { stem.replacen("_EQ", "_DL", 1) } else { format!("{stem}_DL") };
    let mut target = path.with_file_name(format!("{base}.xml"));
    let mut n = 2;
    while target.exists() {
        target = path.with_file_name(format!("{base}_{n}.xml"));
        n += 1;
    }
    let new_url = Url::from_file_path(&target).map_err(|_| "no URL for the new file".to_string())?;
    let (lines, index) = (cimview::text::LineIndex::new(&text), cimview::index::Index::build(&text));
    let new_file = File { name: new_url.as_str(), text: &text, lines: &lines, index: &index };
    let written = cimview::edit::diagram(std::slice::from_ref(&new_file), new_url.as_str(), None, name, &objects, &mut random)?;
    let edits: Vec<cimview::text::TextEdit> = written.edits.into_iter().flat_map(|(_, e)| e).collect();
    let content = cimview::text::apply(&text, &edits);
    let start = lsp_types::Position::new(0, 0);
    let operations = vec![
        lsp_types::DocumentChangeOperation::Op(lsp_types::ResourceOp::Create(lsp_types::CreateFile { uri: new_url.clone(), options: None, annotation_id: None })),
        lsp_types::DocumentChangeOperation::Edit(TextDocumentEdit {
            text_document: OptionalVersionedTextDocumentIdentifier { uri: new_url.clone(), version: None },
            edits: vec![OneOf::Left(lsp_types::TextEdit { range: lsp_types::Range::new(start, start), new_text: content })],
        }),
    ];
    Ok((WorkspaceEdit { document_changes: Some(DocumentChanges::Operations(operations)), ..Default::default() }, written.id))
}

#[cfg(test)]
mod tests {
    use super::*;

    /// MicroGrid's Type1 files, named by their URLs; `dl` whether its DL file is among them.
    fn micro_grid(dl: bool) -> Vec<(String, String)> {
        let dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../CGMES-Test-Configurations/v3.0/MicroGrid/MicroGrid-Type1/MicroGrid-Type1-Merged");
        let mut paths: Vec<_> = std::fs::read_dir(&dir).unwrap().flatten().map(|e| e.path()).filter(|p| p.extension().is_some_and(|x| x == "xml")).collect();
        paths.sort();
        paths
            .into_iter()
            .filter(|p| dl || !p.to_string_lossy().contains("_DL_"))
            .map(|p| (Url::from_file_path(&p).unwrap().to_string(), std::fs::read_to_string(&p).unwrap()))
            .collect()
    }

    fn with<R>(texts: &[(String, String)], f: impl FnOnce(&[File], &cimview::tree::Tree, &cimview::grid::Grid) -> R) -> R {
        let datasets: Vec<cimmodel::CimDataset> = texts.iter().map(|(_, t)| cimmodel::CimDataset::decode_str(t).unwrap()).collect();
        let model = cimview::grid::Model::new(&datasets);
        let (grid, tree) = (cimview::grid::build(&model), cimview::tree::build(&model));
        let lines: Vec<cimview::text::LineIndex> = texts.iter().map(|(_, t)| cimview::text::LineIndex::new(t)).collect();
        let indexes: Vec<cimview::index::Index> = texts.iter().map(|(_, t)| cimview::index::Index::build(t)).collect();
        let files: Vec<File> = texts.iter().zip(&lines).zip(&indexes).map(|(((n, t), l), i)| File { name: n, text: t, lines: l, index: i }).collect();
        f(&files, &tree, &grid)
    }

    fn anvers(grid: &cimview::grid::Grid) -> (String, &cimview::grid::Inside) {
        let n = grid.nodes.iter().position(|n| n.label == "Anvers").unwrap();
        (grid.nodes[n].id.clone(), &grid.inside[&n])
    }

    #[test]
    fn a_topology_edit_in_a_diagram_changes_eq_ssh_and_dl() {
        with(&micro_grid(true), |files, tree, grid| {
            let (substation, inside) = anvers(grid);
            let diagram = inside.layouts.iter().find(|l| l.kind == "diagram").unwrap().id.trim_start_matches("dl:").to_string();
            let op = serde_json::json!({ "op": "add", "class": "Breaker", "level": inside.groups[0].id });
            let (edit, select) = topology(files, tree, grid, &substation, &op, Some(&diagram), Some((0.0, 0.0))).unwrap();
            assert!(select.is_some());
            let changed: Vec<String> = edit.changes.unwrap().keys().map(|u| u.to_string()).collect();
            for profile in ["_EQ_", "_SSH_", "_DL_"] {
                assert!(changed.iter().any(|u| u.contains(profile)), "{profile}: {changed:?}");
            }
        });
    }

    #[test]
    fn a_layout_without_a_dl_file_goes_into_a_new_one() {
        with(&micro_grid(false), |files, _, grid| {
            let (substation, inside) = anvers(grid);
            let auto = inside.layout_json(&cimview::single_line::single_line(inside));
            let (edit, id) = diagram(files, grid, &substation, &auto, None, "Anvers single-line").unwrap();
            assert!(id.starts_with('_'));
            let Some(DocumentChanges::Operations(ops)) = edit.document_changes else { panic!("operations") };
            let lsp_types::DocumentChangeOperation::Op(lsp_types::ResourceOp::Create(created)) = &ops[0] else { panic!("a file created") };
            assert!(created.uri.as_str().ends_with("_BE_DL_1.xml"), "{}", created.uri);
            let lsp_types::DocumentChangeOperation::Edit(e) = &ops[1] else { panic!("its text") };
            let OneOf::Left(text) = &e.edits[0] else { panic!() };
            assert!(text.new_text.contains("Anvers single-line") && text.new_text.contains("DiagramLayout"));
        });
    }
}
