//! `cimlsp`: a language server for CGMES and NC RDF/XML over stdio.
//!
//! - Diagnostics: the SHACL shape tables and SPARQL rules `cimcli validate`
//!   runs, over the model set a document belongs to (see [`validate`]), on
//!   open and save, and while typing if `onType` is set.
//! - Hover, go to definition, references, document outline and completion
//!   from the class tables ([`features`]).
//!
//! Initialization options (the VS Code extension sends its settings):
//! `{ "common": bool, "quality": bool, "silence": [rule ids], "onType": bool }`.
//! RDFS and SHACL directories come from `CIMOXIDE_RDFS_DIR` /
//! `CIMOXIDE_SHACL_DIR` in the server's environment, as for `cimcli`.

mod features;
mod index;
mod validate;

use std::collections::{HashMap, HashSet};
use std::path::PathBuf;
use std::sync::{Arc, Mutex, OnceLock};
use std::time::{Duration, Instant};

use crossbeam_channel::{Receiver, RecvTimeoutError, Sender};
use lsp_server::{Connection, ErrorCode, Message, Notification, Request, RequestId, Response};
use lsp_types::notification::{
    DidChangeTextDocument, DidCloseTextDocument, DidOpenTextDocument, DidSaveTextDocument, LogMessage,
    Notification as _, PublishDiagnostics,
};
use lsp_types::request::{
    Completion, DocumentSymbolRequest, ExecuteCommand, GotoDefinition, HoverRequest, References, Request as _,
};
use lsp_types::{
    CompletionOptions, DocumentSymbolResponse, ExecuteCommandOptions, GotoDefinitionResponse, HoverProviderCapability,
    LogMessageParams, MessageType, OneOf, PublishDiagnosticsParams, SaveOptions, ServerCapabilities,
    TextDocumentContentChangeEvent, TextDocumentSyncCapability, TextDocumentSyncKind, TextDocumentSyncOptions,
    TextDocumentSyncSaveOptions, Url,
};

use features::View;
use index::{Index, LineIndex};
use validate::{ModelSet, Options};

const REVALIDATE: &str = "cimoxide.revalidate";

/// How long a burst of triggers is gathered before a set is validated.
const DEBOUNCE: Duration = Duration::from_millis(300);

#[derive(Debug, Default, Clone, serde::Deserialize)]
#[serde(default, rename_all = "camelCase")]
struct InitOptions {
    #[serde(flatten)]
    validation: Options,
    on_type: bool,
}

/// An open document. Its index is built on first use after each change.
struct Doc {
    text: Arc<str>,
    built: Arc<OnceLock<(LineIndex, Index)>>,
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
        completion_provider: Some(CompletionOptions { trigger_characters: Some(vec!["<".into()]), ..Default::default() }),
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
        _ => return error(id, ErrorCode::MethodNotFound, format!("unhandled method {}", req.method)),
    };
    match result {
        Ok(v) => Response::new_ok(id, v),
        Err(e) => error(id, ErrorCode::InternalError, e.to_string()),
    }
}

/// `cimoxide.revalidate [uri]`: the set of the given document, else of
/// every open CIM document.
fn revalidate(state: &Shared, jobs: &Sender<PathBuf>, args: &[serde_json::Value]) {
    let st = state.lock().unwrap();
    let wanted: Option<Url> = args.first().and_then(|a| a.as_str()).and_then(|s| Url::parse(s).ok());
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
