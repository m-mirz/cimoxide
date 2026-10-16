//! `cimlsp`: a language server for CGMES and NC RDF/XML over stdio.
//!
//! - Diagnostics: the SHACL shape tables and SPARQL rules `cimcli validate`
//!   runs, over the model set a document belongs to (see [`validate`]), on
//!   open and save, and while typing if `onType` is set.
//! - Hover, go to definition, references, document outline and completion
//!   from the class tables ([`features`]).
//! - Rename of an mRID across the files of its model set, quick fixes for
//!   missing fields and references to nothing ([`edit`]); deleting and adding
//!   objects is the model tree's (`cimoxide/delete`, `cimoxide/add`).
//! - `cimoxide/grid`: the model set's grid at substation level, for the
//!   extension's grid view ([`grid`]); `cimoxide/substation`: what one of its
//!   substations holds, for the view's drill-down; `cimoxide/tree`: its
//!   objects by container, area and class ([`tree`]), a level at a time;
//!   `cimoxide/treePath`: the keys to expand to reach the object at a position.
//!
//! Initialization options (the VS Code extension sends its settings):
//! `{ "common": bool, "quality": bool, "silence": [rule ids], "onType": bool }`.
//! RDFS and SHACL directories come from `CIMOXIDE_RDFS_DIR` /
//! `CIMOXIDE_SHACL_DIR` in the server's environment, as for `cimcli`.

mod edit;
mod features;
mod grid;
mod index;
mod layout;
mod tree;
mod validate;

use std::collections::{HashMap, HashSet};
use std::path::PathBuf;
use std::sync::{Arc, Mutex, OnceLock};
use std::time::{Duration, Instant};

use crossbeam_channel::{Receiver, RecvTimeoutError, Sender};
use lsp_server::{Connection, ErrorCode, Message, Notification, Request, RequestId, Response};
use lsp_types::notification::{
    DidChangeTextDocument, DidChangeWatchedFiles, DidCloseTextDocument, DidOpenTextDocument, DidSaveTextDocument, LogMessage,
    Notification as _, PublishDiagnostics,
};
use lsp_types::request::{
    CodeActionRequest, Completion, DocumentSymbolRequest, ExecuteCommand, GotoDefinition,
    HoverRequest, PrepareRenameRequest, References, Rename, Request as _,
};
use lsp_types::{
    CodeAction, CodeActionKind, CodeActionOptions, CodeActionOrCommand, CodeActionProviderCapability, CompletionOptions,
    DocumentSymbolResponse, ExecuteCommandOptions, GotoDefinitionResponse, HoverProviderCapability, LogMessageParams,
    MessageType, OneOf, PrepareRenameResponse, PublishDiagnosticsParams, RenameOptions, SaveOptions, ServerCapabilities,
    TextDocumentContentChangeEvent, TextDocumentSyncCapability, TextDocumentSyncKind, TextDocumentSyncOptions,
    TextDocumentSyncSaveOptions, Url,
};

use features::View;
use index::{Index, LineIndex};
use validate::{ModelSet, Options};

const REVALIDATE: &str = "cimoxide.revalidate";

/// `cimoxide/grid` `{ "uri" }`: the grid of the document's model set as last
/// validated ([`grid::Grid::json`]), or `null` before its first validation.
const GRID: &str = "cimoxide/grid";

/// `cimoxide/substation` `{ "uri", "id" }`: what substation `id` of the
/// document's model set holds ([`grid::Grid::inside_json`]), or `null`.
const SUBSTATION: &str = "cimoxide/substation";

/// `cimoxide/tree` `{ "uri", "key"?, "onlyFindings"? }`: the children of
/// node `key` of the document's model set's tree, else its sections
/// ([`tree::Tree::children_json`]); `null` before its first validation or
/// for a key it no longer has.
const TREE: &str = "cimoxide/tree";

/// `cimoxide/treePath` `{ "uri", "position", "onlyFindings"? }`: the object
/// at `position` and the keys of the tree's nodes down to it
/// ([`tree::Tree::path`]), `{ "id", "keys" }`; `null` off an object or
/// before the set's first validation.
const TREE_PATH: &str = "cimoxide/treePath";

/// `cimoxide/delete` `{ "uri", "id", "dependents"? }`: the edit deleting
/// object `id` of the document's model set, and with `dependents` what it
/// holds ([`edit::delete`]): `{ "edit", "label", "objects", "references" }`.
const DELETE: &str = "cimoxide/delete";

/// `cimoxide/addable` `{ "uri", "id" }`: the classes whose objects can be
/// added in object `id` ([`edit::addable`]): `[{ "class", "key", "comment" }]`.
const ADDABLE: &str = "cimoxide/addable";

/// `cimoxide/add` `{ "uri", "parent", "class", "name"? }`: the edit adding an
/// object of `class` in `parent` ([`edit::add`]): `{ "edit", "uri", "select",
/// "id" }`.
const ADD: &str = "cimoxide/add";

/// `cimoxide/fixes` `{ "uri", "id" }`: the fixes for object `id`'s findings
/// that a field is missing, as quick fixes offer them, from the files as they
/// are (open or not): `[{ "title", "detail", "edit", "uri", "select"? }]`.
const FIXES: &str = "cimoxide/fixes";

/// `cimoxide/rename` `{ "uri", "id", "newName" }`: the edit renaming mRID `id`
/// in the document's model set ([`edit::rename`]), from the files as they are,
/// so also where VS Code hands the server no document (over 50 MB).
const RENAME: &str = "cimoxide/rename";

/// `cimoxide/search` `{ "uri", "query" }`: the document's model set's
/// objects whose name or mRID holds `query` ([`tree::Tree::search`]):
/// `[{ "id", "label", "class", "place", "location" }]`.
const SEARCH: &str = "cimoxide/search";

/// `cimoxide/treePathOf` `{ "uri", "id", "onlyFindings"? }`: as
/// `cimoxide/treePath`, for object `id` of the document's model set, which
/// need not be open.
const TREE_PATH_OF: &str = "cimoxide/treePathOf";

/// `cimoxide/referrers` `{ "uri", "id" }`: the objects of the document's
/// model set that refer to object `id`, and by which field
/// ([`edit::referrers`]): `[{ "id", "label", "class", "key", "place", "location" }]`.
const REFERRERS: &str = "cimoxide/referrers";

/// How long a burst of triggers is gathered before a set is validated.
const DEBOUNCE: Duration = Duration::from_millis(300);

#[derive(Debug, Default, Clone, serde::Deserialize)]
#[serde(default, rename_all = "camelCase")]
struct InitOptions {
    #[serde(flatten)]
    validation: Options,
    on_type: bool,
}

/// A document's line index and index, built on first use.
type Built = Arc<OnceLock<(LineIndex, Index)>>;

/// An open document. Its index is built on first use after each change.
struct Doc {
    text: Arc<str>,
    built: Built,
}

impl Doc {
    fn new(text: Arc<str>) -> Self {
        Self { text, built: Arc::default() }
    }
}

#[derive(Default)]
struct State {
    docs: HashMap<Url, Doc>,
    /// The last validation of each model set, by directory.
    sets: HashMap<PathBuf, Arc<ModelSet>>,
}

type Shared = Arc<Mutex<State>>;

fn main() {
    if std::env::args().any(|a| a == "--version" || a == "-V") {
        println!("cimlsp {}", env!("CARGO_PKG_VERSION"));
        return;
    }
    let (connection, io_threads) = Connection::stdio();
    if let Err(e) = run(connection) {
        eprintln!("cimlsp: {e}");
        std::process::exit(1);
    }
    io_threads.join().ok();
}

fn capabilities() -> ServerCapabilities {
    ServerCapabilities {
        text_document_sync: Some(TextDocumentSyncCapability::Options(TextDocumentSyncOptions {
            open_close: Some(true),
            change: Some(TextDocumentSyncKind::INCREMENTAL),
            save: Some(TextDocumentSyncSaveOptions::SaveOptions(SaveOptions { include_text: Some(false) })),
            ..Default::default()
        })),
        hover_provider: Some(HoverProviderCapability::Simple(true)),
        definition_provider: Some(OneOf::Left(true)),
        references_provider: Some(OneOf::Left(true)),
        document_symbol_provider: Some(OneOf::Left(true)),
        rename_provider: Some(OneOf::Right(RenameOptions { prepare_provider: Some(true), work_done_progress_options: Default::default() })),
        code_action_provider: Some(CodeActionProviderCapability::Options(CodeActionOptions {
            code_action_kinds: Some(vec![CodeActionKind::QUICKFIX, CodeActionKind::REFACTOR_REWRITE]),
            ..Default::default()
        })),
        completion_provider: Some(CompletionOptions { trigger_characters: Some(vec!["<".into(), "#".into()]), ..Default::default() }),
        execute_command_provider: Some(ExecuteCommandOptions { commands: vec![REVALIDATE.into()], ..Default::default() }),
        ..Default::default()
    }
}

fn run(connection: Connection) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let params: lsp_types::InitializeParams = serde_json::from_value(connection.initialize(serde_json::to_value(capabilities())?)?)?;
    let opts: InitOptions = params
        .initialization_options
        .and_then(|v| serde_json::from_value(v).ok())
        .unwrap_or_default();

    let state: Shared = Arc::default();
    let (jobs, queue) = crossbeam_channel::unbounded::<PathBuf>();
    let worker = {
        let (state, sender, validation) = (state.clone(), connection.sender.clone(), opts.validation.clone());
        std::thread::spawn(move || validate_loop(&queue, &state, &sender, &validation))
    };

    for msg in &connection.receiver {
        match msg {
            Message::Request(req) => {
                if connection.handle_shutdown(&req)? {
                    break;
                }
                let resp = handle_request(&state, &jobs, req);
                connection.sender.send(Message::Response(resp))?;
            }
            Message::Notification(n) => handle_notification(&state, &jobs, &opts, n),
            Message::Response(_) => {}
        }
    }
    drop(jobs);
    worker.join().ok();
    Ok(())
}

/// The directory of a `file:` document, if it is a CIM document.
fn set_dir(url: &Url, text: &str) -> Option<PathBuf> {
    let path = url.to_file_path().ok()?;
    index::is_cim(text).then(|| path.parent().map(PathBuf::from)).flatten()
}

fn handle_notification(state: &Shared, jobs: &Sender<PathBuf>, opts: &InitOptions, n: Notification) {
    let mut st = state.lock().unwrap();
    match n.method.as_str() {
        DidOpenTextDocument::METHOD => {
            let Ok(p) = serde_json::from_value::<lsp_types::DidOpenTextDocumentParams>(n.params) else { return };
            let text: Arc<str> = Arc::from(p.text_document.text);
            if let Some(dir) = set_dir(&p.text_document.uri, &text) {
                jobs.send(dir).ok();
            }
            st.docs.insert(p.text_document.uri, Doc::new(text));
        }
        DidChangeTextDocument::METHOD => {
            let Ok(p) = serde_json::from_value::<lsp_types::DidChangeTextDocumentParams>(n.params) else { return };
            let Some(doc) = st.docs.get_mut(&p.text_document.uri) else { return };
            let mut text = doc.text.to_string();
            for change in p.content_changes {
                apply_change(&mut text, change);
            }
            *doc = Doc::new(Arc::from(text));
            if opts.on_type
                && let Some(dir) = set_dir(&p.text_document.uri, &doc.text)
            {
                jobs.send(dir).ok();
            }
        }
        DidSaveTextDocument::METHOD => {
            let Ok(p) = serde_json::from_value::<lsp_types::DidSaveTextDocumentParams>(n.params) else { return };
            if let Some(dir) = st.docs.get(&p.text_document.uri).and_then(|d| set_dir(&p.text_document.uri, &d.text)) {
                jobs.send(dir).ok();
            }
        }
        DidCloseTextDocument::METHOD => {
            let Ok(p) = serde_json::from_value::<lsp_types::DidCloseTextDocumentParams>(n.params) else { return };
            // Its diagnostics stay: the file is still part of its model set.
            st.docs.remove(&p.text_document.uri);
        }
        // A file changed on disk, whoever wrote it: a save VS Code never told
        // the server of (a file over 50 MB, one an edit changed unopened),
        // another tool, a checkout. Its set, if validated or open, again.
        DidChangeWatchedFiles::METHOD => {
            let Ok(p) = serde_json::from_value::<lsp_types::DidChangeWatchedFilesParams>(n.params) else { return };
            let dirs: HashSet<PathBuf> = p
                .changes
                .iter()
                .filter_map(|c| c.uri.to_file_path().ok()?.parent().map(PathBuf::from))
                .filter(|d| st.sets.contains_key(d) || st.docs.keys().any(|u| u.to_file_path().ok().and_then(|p| p.parent().map(PathBuf::from)).as_ref() == Some(d)))
                .collect();
            for dir in dirs {
                jobs.send(dir).ok();
            }
        }
        _ => {}
    }
}

/// Apply one incremental (or whole-document) change.
fn apply_change(text: &mut String, change: TextDocumentContentChangeEvent) {
    match change.range {
        Some(range) => {
            let lines = LineIndex::new(text);
            let start = lines.offset(text, range.start);
            let end = lines.offset(text, range.end).max(start);
            text.replace_range(start..end, &change.text);
        }
        None => *text = change.text,
    }
}

fn handle_request(state: &Shared, jobs: &Sender<PathBuf>, req: Request) -> Response {
    let id = req.id.clone();
    let result = match req.method.as_str() {
        HoverRequest::METHOD => with_params::<lsp_types::HoverParams, _>(req, state, |p| {
            (p.text_document_position_params.text_document.uri, p.text_document_position_params.position)
        }, |v, pos| serde_json::to_value(features::hover(v, pos))),
        GotoDefinition::METHOD => with_params::<lsp_types::GotoDefinitionParams, _>(req, state, |p| {
            (p.text_document_position_params.text_document.uri, p.text_document_position_params.position)
        }, |v, pos| serde_json::to_value(features::definition(v, pos).map(GotoDefinitionResponse::Scalar))),
        References::METHOD => {
            let decl = serde_json::from_value::<lsp_types::ReferenceParams>(req.params.clone())
                .is_ok_and(|p| p.context.include_declaration);
            with_params::<lsp_types::ReferenceParams, _>(req, state, |p| {
                (p.text_document_position.text_document.uri, p.text_document_position.position)
            }, |v, pos| serde_json::to_value(features::references(v, pos, decl)))
        }
        DocumentSymbolRequest::METHOD => with_params::<lsp_types::DocumentSymbolParams, _>(req, state, |p| {
            (p.text_document.uri, Default::default())
        }, |v, _| serde_json::to_value(DocumentSymbolResponse::Nested(features::symbols(v)))),
        Completion::METHOD => with_params::<lsp_types::CompletionParams, _>(req, state, |p| {
            (p.text_document_position.text_document.uri, p.text_document_position.position)
        }, |v, pos| serde_json::to_value(features::completion(v, pos))),
        PrepareRenameRequest::METHOD => {
            let p: lsp_types::TextDocumentPositionParams = match serde_json::from_value(req.params) {
                Ok(p) => p,
                Err(e) => return error(id, ErrorCode::InvalidParams, e.to_string()),
            };
            match with_set_files(state, &p.text_document.uri, |_, files| edit::prepare_rename(files, &p.text_document.uri, p.position)).flatten() {
                Some((range, placeholder)) => serde_json::to_value(PrepareRenameResponse::RangeWithPlaceholder { range, placeholder }),
                None => return error(id, ErrorCode::RequestFailed, "not an mRID: rename an `rdf:ID`, `rdf:about` or `rdf:resource`".into()),
            }
        }
        Rename::METHOD => {
            let p: lsp_types::RenameParams = match serde_json::from_value(req.params) {
                Ok(p) => p,
                Err(e) => return error(id, ErrorCode::InvalidParams, e.to_string()),
            };
            let (uri, pos) = (&p.text_document_position.text_document.uri, p.text_document_position.position);
            let renamed = with_set_files(state, uri, |_, files| {
                let (_, old) = edit::prepare_rename(files, uri, pos).ok_or("not an mRID")?;
                edit::rename(files, &old, &p.new_name)
            });
            match renamed {
                Some(Ok(edit)) => serde_json::to_value(edit),
                Some(Err(e)) => return error(id, ErrorCode::RequestFailed, e),
                None => return error(id, ErrorCode::RequestFailed, "the model set has not been validated yet".into()),
            }
        }
        CodeActionRequest::METHOD => {
            let p: lsp_types::CodeActionParams = match serde_json::from_value(req.params) {
                Ok(p) => p,
                Err(e) => return error(id, ErrorCode::InvalidParams, e.to_string()),
            };
            let uri = p.text_document.uri;
            let actions = with_set_files(state, &uri, |set, files| {
                let mut out = quick_fixes(set, files, &uri, &p.context.diagnostics);
                let Some(el) = files.iter().find(|f| f.url == &uri).and_then(|f| f.index.element_at(p.range.start)) else { return out };
                if el.mrid.is_empty() {
                    return out;
                }
                // A reference to nothing, or to no object of the set: what it could name.
                for f in edit::reference_fixes(files, &set.tree, &el.mrid, Some((&uri, p.range.start))) {
                    out.push(CodeActionOrCommand::CodeAction(CodeAction {
                        title: f.title,
                        kind: Some(if f.empty { CodeActionKind::QUICKFIX } else { CodeActionKind::REFACTOR_REWRITE }),
                        edit: Some(change(&f.url, f.edit)),
                        ..Default::default()
                    }));
                }
                out
            });
            serde_json::to_value(actions.unwrap_or_default())
        }
        ADDABLE => {
            #[derive(serde::Deserialize)]
            struct Params {
                uri: Url,
                id: String,
            }
            let p: Params = match serde_json::from_value(req.params) {
                Ok(p) => p,
                Err(e) => return error(id, ErrorCode::InvalidParams, e.to_string()),
            };
            let classes = with_set_files(state, &p.uri, |_, files| {
                edit::addable(files, &p.id)
                    .into_iter()
                    .map(|a| serde_json::json!({ "class": a.class.local, "key": a.key, "comment": a.class.comment }))
                    .collect::<Vec<_>>()
            });
            Ok(serde_json::json!(classes.unwrap_or_default()))
        }
        ADD => {
            #[derive(serde::Deserialize)]
            struct Params {
                uri: Url,
                parent: String,
                class: String,
                name: Option<String>,
            }
            let p: Params = match serde_json::from_value(req.params) {
                Ok(p) => p,
                Err(e) => return error(id, ErrorCode::InvalidParams, e.to_string()),
            };
            match with_set_files(state, &p.uri, |set, files| edit::add(files, &set.tree, &p.parent, &p.class, p.name.as_deref())) {
                Some(Ok(a)) => Ok(serde_json::json!({ "edit": a.edit, "uri": a.url, "select": a.select, "id": a.id })),
                Some(Err(e)) => return error(id, ErrorCode::RequestFailed, e),
                None => return error(id, ErrorCode::RequestFailed, "the model set has not been validated yet".into()),
            }
        }
        REFERRERS => {
            #[derive(serde::Deserialize)]
            struct Params {
                uri: Url,
                id: String,
            }
            let p: Params = match serde_json::from_value(req.params) {
                Ok(p) => p,
                Err(e) => return error(id, ErrorCode::InvalidParams, e.to_string()),
            };
            let found = with_set_files(state, &p.uri, |set, files| {
                edit::referrers(files, &p.id)
                    .into_iter()
                    .map(|(r, key)| {
                        let (label, class) = set.tree.describe(r).unwrap_or((r, ""));
                        let location = set.definition(r).map(|(file, el)| serde_json::json!({ "uri": file.url, "range": el.head }));
                        serde_json::json!({
                            "id": r, "label": label, "class": class, "key": key,
                            "place": set.tree.containers(r).join(" › "), "location": location,
                        })
                    })
                    .collect::<Vec<_>>()
            });
            Ok(serde_json::json!(found.unwrap_or_default()))
        }
        FIXES => {
            #[derive(serde::Deserialize)]
            struct Params {
                uri: Url,
                id: String,
            }
            let p: Params = match serde_json::from_value(req.params) {
                Ok(p) => p,
                Err(e) => return error(id, ErrorCode::InvalidParams, e.to_string()),
            };
            let found = with_set_files(state, &p.uri, |set, files| {
                let findings = set.findings.get(&p.id).cloned().unwrap_or_default();
                let missing = findings.into_iter().map(|(property, rule, message)| (p.id.clone(), property, (rule, message))).collect();
                // Add where the object is defined, as it is written there.
                let home = set.definition(&p.id).map(|(f, _)| f.url.clone()).unwrap_or_else(|| p.uri.clone());
                let mut out: Vec<serde_json::Value> = fixes(set, files, &home, missing)
                    .into_iter()
                    .map(|f| {
                        let detail: Vec<&str> = f.fixes.iter().map(|(_, m)| m.as_str()).collect();
                        serde_json::json!({
                            "title": f.title, "detail": detail.join("; "), "edit": change(&f.url, f.edit),
                            "uri": f.url, "select": f.select.map(|(r, _)| r),
                        })
                    })
                    .collect();
                // References to nothing, or to what the set lacks, which no finding may report.
                out.extend(edit::reference_fixes(files, &set.tree, &p.id, None).into_iter().map(|f| {
                    let detail = if f.empty { format!("{} names nothing", f.key) } else { format!("{} names an object the set does not have", f.key) };
                    serde_json::json!({ "title": f.title, "detail": detail, "edit": change(&f.url, f.edit), "uri": f.url, "select": null })
                }));
                out
            });
            Ok(serde_json::json!(found.unwrap_or_default()))
        }
        RENAME => {
            #[derive(serde::Deserialize)]
            #[serde(rename_all = "camelCase")]
            struct Params {
                uri: Url,
                id: String,
                new_name: String,
            }
            let p: Params = match serde_json::from_value(req.params) {
                Ok(p) => p,
                Err(e) => return error(id, ErrorCode::InvalidParams, e.to_string()),
            };
            match with_set_files(state, &p.uri, |_, files| edit::rename(files, &p.id, &p.new_name)) {
                Some(Ok(edit)) => serde_json::to_value(edit),
                Some(Err(e)) => return error(id, ErrorCode::RequestFailed, e),
                None => return error(id, ErrorCode::RequestFailed, "the model set has not been validated yet".into()),
            }
        }
        DELETE => {
            let p: DeleteParams = match serde_json::from_value(req.params) {
                Ok(p) => p,
                Err(e) => return error(id, ErrorCode::InvalidParams, e.to_string()),
            };
            Ok(deletion(state, &p).map_or(serde_json::Value::Null, |(d, label)| {
                serde_json::json!({ "edit": d.edit, "label": label, "objects": d.objects, "references": d.references })
            }))
        }
        ExecuteCommand::METHOD => {
            let p: lsp_types::ExecuteCommandParams = match serde_json::from_value(req.params) {
                Ok(p) => p,
                Err(e) => return error(id, ErrorCode::InvalidParams, e.to_string()),
            };
            if p.command == REVALIDATE {
                revalidate(state, jobs, &p.arguments);
            }
            Ok(serde_json::Value::Null)
        }
        GRID => {
            #[derive(serde::Deserialize)]
            struct Params {
                uri: Url,
            }
            let p: Params = match serde_json::from_value(req.params) {
                Ok(p) => p,
                Err(e) => return error(id, ErrorCode::InvalidParams, e.to_string()),
            };
            let set = p.uri.to_file_path().ok().and_then(|f| {
                let st = state.lock().unwrap();
                f.parent().and_then(|d| st.sets.get(d)).cloned()
            });
            Ok(set.map_or(serde_json::Value::Null, |s| s.grid.json(&s)))
        }
        SUBSTATION => {
            #[derive(serde::Deserialize)]
            struct Params {
                uri: Url,
                id: String,
            }
            let p: Params = match serde_json::from_value(req.params) {
                Ok(p) => p,
                Err(e) => return error(id, ErrorCode::InvalidParams, e.to_string()),
            };
            let set = p.uri.to_file_path().ok().and_then(|f| {
                let st = state.lock().unwrap();
                f.parent().and_then(|d| st.sets.get(d)).cloned()
            });
            Ok(set.and_then(|s| s.grid.inside_json(&s, &p.id)).unwrap_or(serde_json::Value::Null))
        }
        TREE => {
            #[derive(serde::Deserialize)]
            #[serde(rename_all = "camelCase")]
            struct Params {
                uri: Url,
                key: Option<String>,
                #[serde(default)]
                only_findings: bool,
            }
            let p: Params = match serde_json::from_value(req.params) {
                Ok(p) => p,
                Err(e) => return error(id, ErrorCode::InvalidParams, e.to_string()),
            };
            let set = p.uri.to_file_path().ok().and_then(|f| {
                let st = state.lock().unwrap();
                f.parent().and_then(|d| st.sets.get(d)).cloned()
            });
            Ok(set.and_then(|s| s.tree.children_json(&s, &s.grid, p.key.as_deref(), p.only_findings)).unwrap_or(serde_json::Value::Null))
        }
        SEARCH => {
            #[derive(serde::Deserialize)]
            struct Params {
                uri: Url,
                query: String,
            }
            let p: Params = match serde_json::from_value(req.params) {
                Ok(p) => p,
                Err(e) => return error(id, ErrorCode::InvalidParams, e.to_string()),
            };
            let set = p.uri.to_file_path().ok().and_then(|f| {
                let st = state.lock().unwrap();
                f.parent().and_then(|d| st.sets.get(d)).cloned()
            });
            let hits = set.map(|s| {
                s.tree
                    .search(&p.query, 100)
                    .into_iter()
                    .map(|(oid, label, class)| {
                        let location = s.definition(oid).map(|(file, el)| serde_json::json!({ "uri": file.url, "range": el.head }));
                        serde_json::json!({ "id": oid, "label": label, "class": class, "place": s.tree.containers(oid).join(" › "), "location": location })
                    })
                    .collect::<Vec<_>>()
            });
            Ok(serde_json::json!(hits.unwrap_or_default()))
        }
        TREE_PATH_OF => {
            #[derive(serde::Deserialize)]
            #[serde(rename_all = "camelCase")]
            struct Params {
                uri: Url,
                id: String,
                #[serde(default)]
                only_findings: bool,
            }
            let p: Params = match serde_json::from_value(req.params) {
                Ok(p) => p,
                Err(e) => return error(id, ErrorCode::InvalidParams, e.to_string()),
            };
            let set = p.uri.to_file_path().ok().and_then(|f| {
                let st = state.lock().unwrap();
                f.parent().and_then(|d| st.sets.get(d)).cloned()
            });
            let found = set.and_then(|s| s.tree.path(&p.id, p.only_findings)).map(|keys| serde_json::json!({ "id": p.id, "keys": keys }));
            Ok(found.unwrap_or(serde_json::Value::Null))
        }
        TREE_PATH => {
            #[derive(serde::Deserialize)]
            #[serde(rename_all = "camelCase")]
            struct Params {
                uri: Url,
                position: lsp_types::Position,
                #[serde(default)]
                only_findings: bool,
            }
            let only_findings = std::cell::Cell::new(false);
            with_params::<Params, _>(req, state, |p| {
                only_findings.set(p.only_findings);
                (p.uri, p.position)
            }, |v, pos| {
                let found = v.index.element_at(pos).zip(v.set).and_then(|(el, set)| {
                    set.tree.path(&el.mrid, only_findings.get()).map(|keys| serde_json::json!({ "id": el.mrid, "keys": keys }))
                });
                Ok(found.unwrap_or(serde_json::Value::Null))
            })
        }
        _ => return error(id, ErrorCode::MethodNotFound, format!("unhandled method {}", req.method)),
    };
    match result {
        Ok(v) => Response::new_ok(id, v),
        Err(e) => error(id, ErrorCode::InternalError, e.to_string()),
    }
}

/// `cimoxide.revalidate [uri]`: the set of the given document, else of
/// every open CIM document. The given one need not be open: VS Code hands no
/// file over 50 MB to an extension, so the client asks for its set by name.
fn revalidate(state: &Shared, jobs: &Sender<PathBuf>, args: &[serde_json::Value]) {
    let st = state.lock().unwrap();
    let wanted: Option<Url> = args.first().and_then(|a| a.as_str()).and_then(|s| Url::parse(s).ok());
    if let Some(w) = &wanted
        && !st.docs.contains_key(w)
        && let Some(dir) = w.to_file_path().ok().and_then(|p| p.parent().map(PathBuf::from))
    {
        jobs.send(dir).ok();
        return;
    }
    let dirs: HashSet<PathBuf> = st
        .docs
        .iter()
        .filter(|(url, _)| wanted.as_ref().is_none_or(|w| w == *url))
        .filter_map(|(url, doc)| set_dir(url, &doc.text))
        .collect();
    for dir in dirs {
        jobs.send(dir).ok();
    }
}

fn error(id: RequestId, code: ErrorCode, message: String) -> Response {
    Response::new_err(id, code as i32, message)
}

/// Decode a request's params, find its document and run `f` on a view of it.
/// A document that is not open, or not CIM, answers `null`.
fn with_params<P, F>(
    req: Request,
    state: &Shared,
    locate: impl FnOnce(P) -> (Url, lsp_types::Position),
    f: F,
) -> Result<serde_json::Value, serde_json::Error>
where
    P: serde::de::DeserializeOwned,
    F: FnOnce(&View, lsp_types::Position) -> Result<serde_json::Value, serde_json::Error>,
{
    let (url, pos) = locate(serde_json::from_value(req.params)?);
    let (text, built, set) = {
        let st = state.lock().unwrap();
        let Some(doc) = st.docs.get(&url) else { return Ok(serde_json::Value::Null) };
        let set = url
            .to_file_path()
            .ok()
            .and_then(|p| p.parent().and_then(|d| st.sets.get(d)).cloned());
        (doc.text.clone(), doc.built.clone(), set)
    };
    if !index::is_cim(&text) {
        return Ok(serde_json::Value::Null);
    }
    let (lines, index) = built.get_or_init(|| (LineIndex::new(&text), Index::build(&text)));
    f(&View { url: &url, text: &text, lines, index, set: set.as_deref() }, pos)
}

/// A fix for findings that a field is missing ([`edit::add_missing`]): its
/// title, the edit, the value to select after it (and whether to list what a
/// reference can name), and the findings it fixes.
struct Fix<D> {
    title: String,
    url: Url,
    edit: lsp_types::TextEdit,
    select: Option<(lsp_types::Range, bool)>,
    preferred: bool,
    fixes: Vec<D>,
}

/// The fixes for `missing` — (object, property, finding) — preferring to add
/// in `prefer`: each field alone, a reference with the five likeliest objects
/// to name and one to choose later; several on one object, also all at once.
fn fixes<D: Clone>(set: &ModelSet, files: &[edit::File], prefer: &Url, missing: Vec<(String, String, D)>) -> Vec<Fix<D>> {
    let indexes: Vec<&Index> = files.iter().map(|f| f.index).collect();
    // The additions to each object, by the file they add to.
    type Adds<D> = Vec<(String, D, edit::Addition)>;
    let mut by_object: Vec<(String, Url, Adds<D>)> = Vec::new();
    for (object, property, finding) in missing {
        if object.is_empty() || property.starts_with('^') {
            continue;
        }
        let Some(add) = edit::add_missing(files, prefer, &object, &property) else { continue };
        let at = by_object.iter().position(|(o, u, adds)| *o == object && *u == add.url && !adds.iter().any(|(_, _, a)| a.edit == add.edit));
        match at {
            Some(i) => by_object[i].2.push((property, finding, add)),
            None => by_object.push((object, add.url.clone(), vec![(property, finding, add)])),
        }
    }
    let mut out = Vec::new();
    for (object, url, adds) in by_object {
        for (property, finding, add) in &adds {
            let named = if add.reference { edit::candidates(&indexes, &set.tree, &object, property, 5) } else { Vec::new() };
            for (i, c) in named.iter().enumerate() {
                let place = if c.place.is_empty() { String::new() } else { format!(" in {}", c.place) };
                out.push(Fix {
                    title: format!("Add {property}: {} {}{place}", c.class, c.label),
                    url: url.clone(),
                    edit: add.naming(c),
                    select: None,
                    preferred: i == 0 && adds.len() == 1,
                    fixes: vec![finding.clone()],
                });
            }
            out.push(Fix {
                title: if add.reference { format!("Add {property}, choosing the object…") } else { format!("Add {property}") },
                url: url.clone(),
                edit: add.edit.clone(),
                select: Some((add.value, add.reference)),
                preferred: adds.len() == 1 && named.is_empty(),
                fixes: vec![finding.clone()],
            });
        }
        if adds.len() > 1 {
            // One insertion, the fields in the order of the findings.
            let at = adds[0].2.edit.range;
            let text: String = adds.iter().map(|(_, _, a)| a.edit.new_text.as_str()).collect();
            out.push(Fix {
                title: format!("Add the {} missing fields", adds.len()),
                url: url.clone(),
                edit: lsp_types::TextEdit::new(at, text),
                select: None,
                preferred: true,
                fixes: adds.iter().map(|(_, f, _)| f.clone()).collect(),
            });
        }
    }
    out
}

/// A field-sized edit to one file.
fn change(url: &Url, e: lsp_types::TextEdit) -> lsp_types::WorkspaceEdit {
    lsp_types::WorkspaceEdit { changes: Some(HashMap::from([(url.clone(), vec![e])])), ..Default::default() }
}

/// The command a fix runs after its edit: select the value it wrote.
fn select_command(url: &Url, (range, suggest): (lsp_types::Range, bool)) -> lsp_types::Command {
    lsp_types::Command {
        title: "Select the value".into(),
        command: "cimoxide.select".into(),
        // For a reference, also list the objects it can name (completion).
        arguments: Some(vec![serde_json::json!(url), serde_json::json!(range), serde_json::json!(suggest)]),
    }
}

/// For each finding of `diagnostics` that a field is missing, the actions
/// adding it ([`fixes`]).
fn quick_fixes(set: &ModelSet, files: &[edit::File], uri: &Url, diagnostics: &[lsp_types::Diagnostic]) -> Vec<CodeActionOrCommand> {
    let missing = diagnostics
        .iter()
        .filter(|d| d.source.as_deref() == Some("cimoxide"))
        .filter_map(|d| {
            let data = d.data.as_ref()?;
            Some((data["object_id"].as_str()?.to_string(), data["property"].as_str()?.to_string(), d.clone()))
        })
        .collect();
    fixes(set, files, uri, missing)
        .into_iter()
        .map(|f| {
            CodeActionOrCommand::CodeAction(CodeAction {
                title: f.title,
                kind: Some(CodeActionKind::QUICKFIX),
                diagnostics: Some(f.fixes),
                edit: Some(change(&f.url, f.edit)),
                command: f.select.map(|s| select_command(&f.url, s)),
                is_preferred: Some(f.preferred),
                ..Default::default()
            })
        })
        .collect()
}

#[derive(serde::Deserialize)]
struct DeleteParams {
    uri: Url,
    id: String,
    #[serde(default)]
    dependents: bool,
}

/// The deletion `p` asks for, and its label.
fn deletion(state: &Shared, p: &DeleteParams) -> Option<(edit::Deletion, String)> {
    with_set_files(state, &p.uri, |set, files| {
        let mut ids: HashSet<&str> = HashSet::from([p.id.as_str()]);
        if p.dependents {
            ids.extend(set.tree.descendants(&p.id));
        }
        let what = edit::describe(files, &p.id);
        let label = if p.dependents { format!("Delete {what} and what it holds") } else { format!("Delete {what}") };
        (edit::delete(files, &ids, &label), label)
    })
}

/// Run `f` over the files of `url`'s model set as they are now: an open
/// document's buffer, else the file on disk with the set's index of it.
/// `None` before the set's first validation.
fn with_set_files<R>(state: &Shared, url: &Url, f: impl FnOnce(&ModelSet, &[edit::File]) -> R) -> Option<R> {
    enum Source {
        Open(Built),
        Disk(LineIndex),
    }
    let dir = url.to_file_path().ok()?.parent()?.to_path_buf();
    let (set, held) = {
        let st = state.lock().unwrap();
        let set = st.sets.get(&dir)?.clone();
        let held: Vec<(Arc<str>, Option<Built>)> = set
            .files
            .iter()
            .map(|file| match st.docs.get(&file.url) {
                Some(doc) => (doc.text.clone(), Some(doc.built.clone())),
                None => (Arc::from(std::fs::read_to_string(&file.path).unwrap_or_default()), None),
            })
            .collect();
        (set, held)
    };
    let sources: Vec<(Arc<str>, Source)> = held
        .into_iter()
        .map(|(text, built)| {
            let source = match built {
                Some(b) => Source::Open(b),
                None => Source::Disk(LineIndex::new(&text)),
            };
            (text, source)
        })
        .collect();
    let files: Vec<edit::File> = sources
        .iter()
        .zip(&set.files)
        .map(|((text, source), file)| {
            let (lines, index) = match source {
                Source::Open(b) => {
                    let (l, i) = b.get_or_init(|| (LineIndex::new(text), Index::build(text)));
                    (l, i)
                }
                Source::Disk(l) => (l, &file.index),
            };
            edit::File { url: &file.url, text, lines, index }
        })
        .collect();
    Some(f(&set, &files))
}

/// Validate model sets as they are asked for. Requests arriving within
/// [`DEBOUNCE`] of each other are gathered, and each set runs once.
fn validate_loop(queue: &Receiver<PathBuf>, state: &Shared, sender: &Sender<Message>, opts: &Options) {
    // What was last published for each set, to clear files that left it.
    let mut published: HashMap<PathBuf, Vec<Url>> = HashMap::new();
    while let Ok(first) = queue.recv() {
        let mut dirs = vec![first];
        loop {
            match queue.recv_timeout(DEBOUNCE) {
                Ok(d) => {
                    if !dirs.contains(&d) {
                        dirs.push(d);
                    }
                }
                Err(RecvTimeoutError::Timeout) => break,
                Err(RecvTimeoutError::Disconnected) => return,
            }
        }
        for dir in dirs {
            let open: HashMap<PathBuf, Arc<str>> = {
                let st = state.lock().unwrap();
                st.docs.iter().filter_map(|(u, d)| Some((u.to_file_path().ok()?, d.text.clone()))).collect()
            };
            let started = Instant::now();
            let files = validate::collect(&dir, &open);
            let (set, diags) = validate::validate(&files, opts);
            let findings: usize = diags.iter().map(|(_, d)| d.len()).sum();

            let urls: Vec<Url> = diags.iter().map(|(u, _)| u.clone()).collect();
            for stale in published.remove(&dir).unwrap_or_default().into_iter().filter(|u| !urls.contains(u)) {
                publish(sender, stale, Vec::new());
            }
            for (url, d) in diags {
                publish(sender, url, d);
            }
            published.insert(dir.clone(), urls);
            log(sender, format!(
                "validated {} file(s) in {} in {:.1?}: {findings} finding(s)",
                files.len(), dir.display(), started.elapsed(),
            ));
            state.lock().unwrap().sets.insert(dir, Arc::new(set));
        }
    }
}

fn publish(sender: &Sender<Message>, uri: Url, diagnostics: Vec<lsp_types::Diagnostic>) {
    let params = PublishDiagnosticsParams { uri, diagnostics, version: None };
    sender.send(Message::Notification(Notification::new(PublishDiagnostics::METHOD.into(), params))).ok();
}

fn log(sender: &Sender<Message>, message: String) {
    let params = LogMessageParams { typ: MessageType::INFO, message };
    sender.send(Message::Notification(Notification::new(LogMessage::METHOD.into(), params))).ok();
}

#[cfg(test)]
mod tests {
    use super::*;
    use lsp_types::{Position, Range};

    #[test]
    fn applies_incremental_changes() {
        let mut text = "ab\ncd€e\n".to_string();
        apply_change(&mut text, TextDocumentContentChangeEvent {
            range: Some(Range::new(Position::new(1, 2), Position::new(1, 3))),
            range_length: None,
            text: "X".into(),
        });
        assert_eq!(text, "ab\ncdXe\n");
        apply_change(&mut text, TextDocumentContentChangeEvent { range: None, range_length: None, text: "new".into() });
        assert_eq!(text, "new");
    }
}
