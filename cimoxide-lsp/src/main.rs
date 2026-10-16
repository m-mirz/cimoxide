//! `cimlsp`: a language server for CGMES and NC RDF/XML over stdio.
//!
//! - Diagnostics: the SHACL shape tables and SPARQL rules `cimcli validate`
//!   runs, over the model set a document belongs to (see [`validate`]), on
//!   open and save, and while typing if `onType` is set.
//! - Hover, go to definition, references, document outline and completion
//!   from the class tables ([`features`]).
//! - Rename of an mRID, and code actions deleting an object (with what it
//!   holds), across the files of its model set ([`edit`]).
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
    CodeActionRequest, CodeActionResolveRequest, Completion, DocumentSymbolRequest, ExecuteCommand, GotoDefinition,
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
            code_action_kinds: Some(vec![CodeActionKind::QUICKFIX, CodeActionKind::REFACTOR, CodeActionKind::REFACTOR_REWRITE]),
            resolve_provider: Some(true),
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
                let field = el.fields.iter().find(|f| index::contains(f.full, p.range.start));
                if let Some((f, (target, value))) = field.and_then(|f| f.resource.as_ref().map(|r| (f, r)))
                    && (target.is_empty() || !files.iter().any(|x| x.index.elements.iter().any(|e| e.mrid == *target)))
                {
                    let indexes: Vec<&Index> = files.iter().map(|f| f.index).collect();
                    for c in edit::candidates(&indexes, &set.tree, &el.mrid, f.key(), 5) {
                        let place = if c.place.is_empty() { String::new() } else { format!(" in {}", c.place) };
                        out.push(CodeActionOrCommand::CodeAction(CodeAction {
                            title: format!("Set {} to {} {}{place}", f.key(), c.class, c.label),
                            kind: Some(if target.is_empty() { CodeActionKind::QUICKFIX } else { CodeActionKind::REFACTOR_REWRITE }),
                            edit: Some(lsp_types::WorkspaceEdit {
                                changes: Some(HashMap::from([(uri.clone(), vec![lsp_types::TextEdit::new(*value, c.reference())])])),
                                ..Default::default()
                            }),
                            ..Default::default()
                        }));
                    }
                }
                let what = edit::describe(files, &el.mrid);
                let held = set.tree.descendants(&el.mrid).len();
                let action = |title: String, dependents: bool| {
                    CodeActionOrCommand::CodeAction(CodeAction {
                        title,
                        kind: Some(CodeActionKind::REFACTOR),
                        data: Some(serde_json::json!({ "uri": uri, "id": el.mrid, "dependents": dependents })),
                        ..Default::default()
                    })
                };
                // Adding in it is the extension's dialogue: class, then name.
                if !edit::addable(files, &el.mrid).is_empty() {
                    out.push(CodeActionOrCommand::CodeAction(CodeAction {
                        title: format!("Add… in {what}"),
                        kind: Some(CodeActionKind::REFACTOR),
                        command: Some(lsp_types::Command {
                            title: "Add".into(),
                            command: "cimoxide.model.add".into(),
                            arguments: Some(vec![serde_json::json!({ "uri": uri, "id": el.mrid, "label": what })]),
                        }),
                        ..Default::default()
                    }));
                }
                out.push(action(format!("Delete {what}"), false));
                if held > 0 {
                    out.push(action(format!("Delete {what} and the {held} objects it holds"), true));
                }
                out
            });
            serde_json::to_value(actions.unwrap_or_default())
        }
        CodeActionResolveRequest::METHOD => {
            let mut action: CodeAction = match serde_json::from_value(req.params) {
                Ok(p) => p,
                Err(e) => return error(id, ErrorCode::InvalidParams, e.to_string()),
            };
            // VS Code resolves every action without an edit: one that runs a
            // command (Add…) or has its edit already needs nothing more.
            let Some(p) = action.data.clone().and_then(|d| serde_json::from_value::<DeleteParams>(d).ok()) else {
                return Response::new_ok(id, action);
            };
            action.edit = deletion(state, &p).map(|(d, _)| d.edit);
            // Then open what it changed, for review: a file it changed unopened
            // would otherwise stay unsaved, unseen.
            if let Some(lsp_types::DocumentChanges::Edits(edits)) = action.edit.as_ref().and_then(|e| e.document_changes.as_ref()) {
                let files: Vec<&Url> = edits.iter().map(|e| &e.text_document.uri).collect();
                action.command = Some(lsp_types::Command {
                    title: "Open the files it changes".into(),
                    command: "cimoxide.openEdited".into(),
                    arguments: Some(vec![serde_json::json!(files)]),
                });
            }
            serde_json::to_value(action)
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

/// For each finding of `diagnostics` that a field is missing, the action
/// adding it ([`edit::add_missing`]), which selects the value it writes;
/// with several for one object, also one adding them all.
fn quick_fixes(set: &ModelSet, files: &[edit::File], uri: &Url, diagnostics: &[lsp_types::Diagnostic]) -> Vec<CodeActionOrCommand> {
    let mut out = Vec::new();
    let indexes: Vec<&Index> = files.iter().map(|f| f.index).collect();
    let change = |url: &Url, e: lsp_types::TextEdit| lsp_types::WorkspaceEdit { changes: Some(HashMap::from([(url.clone(), vec![e])])), ..Default::default() };
    // The fixes of each object, by the file they add to.
    type Fixes = Vec<(lsp_types::Diagnostic, edit::Addition)>;
    let mut by_object: Vec<(String, Url, Fixes)> = Vec::new();
    for d in diagnostics.iter().filter(|d| d.source.as_deref() == Some("cimoxide")) {
        let Some(data) = &d.data else { continue };
        let (Some(object), Some(property)) = (data["object_id"].as_str(), data["property"].as_str()) else { continue };
        if object.is_empty() || property.starts_with('^') {
            continue;
        }
        let Some(add) = edit::add_missing(files, uri, object, property) else { continue };
        let at = by_object.iter().position(|(o, u, adds)| o == object && *u == add.url && !adds.iter().any(|(_, a)| a.edit == add.edit));
        match at {
            Some(i) => by_object[i].2.push((d.clone(), add)),
            None => by_object.push((object.to_string(), add.url.clone(), vec![(d.clone(), add)])),
        }
    }
    for (object, url, adds) in by_object {
        for (d, add) in &adds {
            let property = d.data.as_ref().and_then(|x| x["property"].as_str()).unwrap_or_default();
            // A reference: the likeliest objects to name, then one to choose later.
            let named = if add.reference { edit::candidates(&indexes, &set.tree, &object, property, 5) } else { Vec::new() };
            for (i, c) in named.iter().enumerate() {
                let place = if c.place.is_empty() { String::new() } else { format!(" in {}", c.place) };
                out.push(CodeActionOrCommand::CodeAction(CodeAction {
                    title: format!("Add {property}: {} {}{place}", c.class, c.label),
                    kind: Some(CodeActionKind::QUICKFIX),
                    diagnostics: Some(vec![d.clone()]),
                    edit: Some(change(&url, add.naming(c))),
                    is_preferred: Some(i == 0 && adds.len() == 1),
                    ..Default::default()
                }));
            }
            out.push(CodeActionOrCommand::CodeAction(CodeAction {
                title: if add.reference { format!("Add {property}, choosing the object…") } else { format!("Add {property}") },
                kind: Some(CodeActionKind::QUICKFIX),
                diagnostics: Some(vec![d.clone()]),
                edit: Some(change(&url, add.edit.clone())),
                command: Some(lsp_types::Command {
                    title: "Select the value".into(),
                    command: "cimoxide.select".into(),
                    // For a reference, also list the objects it can name (completion).
                    arguments: Some(vec![serde_json::json!(url), serde_json::json!(add.value), serde_json::json!(add.reference)]),
                }),
                is_preferred: Some(adds.len() == 1 && named.is_empty()),
                ..Default::default()
            }));
        }
        if adds.len() > 1 {
            // One insertion, the fields in the order of the findings.
            let at = adds[0].1.edit.range;
            let text: String = adds.iter().map(|(_, a)| a.edit.new_text.as_str()).collect();
            out.push(CodeActionOrCommand::CodeAction(CodeAction {
                title: format!("Add the {} missing fields", adds.len()),
                kind: Some(CodeActionKind::QUICKFIX),
                diagnostics: Some(adds.iter().map(|(d, _)| d.clone()).collect()),
                edit: Some(change(&url, lsp_types::TextEdit::new(at, text))),
                is_preferred: Some(true),
                ..Default::default()
            }));
        }
    }
    out
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
