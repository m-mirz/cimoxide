//! `cimmcp`: a Model Context Protocol server for CGMES and NC RDF/XML over
//! stdio, so an LLM chat (VS Code, Claude Code, Claude Desktop, …) can look
//! up, validate and query a model set.
//!
//! The transport is newline-delimited JSON-RPC 2.0 on stdin/stdout; the
//! server answers `initialize`, `ping`, `tools/list` and `tools/call` and
//! ignores notifications. Requests are handled one at a time, in order. The
//! tools ([`tools`]) are read-only.
//!
//! `cimmcp [--dir <path>]`: `--dir` (else `CIMOXIDE_MODEL_DIR`, else the
//! working directory) is where relative `path` arguments resolve and what a
//! tool reads when given none. RDFS and SHACL directories come from
//! `CIMOXIDE_RDFS_DIR` / `CIMOXIDE_SHACL_DIR`, as for `cimcli`.

mod session;
mod tools;

use std::io::{BufRead, Write};
use std::path::PathBuf;

use serde_json::{Value, json};

use session::Session;

/// Protocol revisions this server speaks, newest first.
const PROTOCOL_VERSIONS: &[&str] = &["2025-06-18", "2025-03-26", "2024-11-05"];

const INSTRUCTIONS: &str = "\
Tools over ENTSO-E CGMES and Network Code (NC) RDF/XML. A model set is every CIM XML file in \
one directory (EQ, SSH, SV, TP, … of one model); `path` names that directory, relative to the \
workspace. Start with `list_model_sets` when you do not know where the data is, then `summary`.
- An object is identified by its mRID: the text after `#` in `rdf:ID`/`rdf:about`/`rdf:resource`, \
  often with a leading `_`.
- Fields are keyed `Class.attr` by the class that declares the attribute \
  (`IdentifiedObject.name`, `Equipment.inService` on a Breaker); values are the text as written, \
  references the target mRID, enumerations the value after `#`.
- NC class names carry an `nc:` prefix (`nc:PowerSchedule`).
- Prefer `find_objects` and `get_object`; use `sparql` for joins and aggregates. \
  `describe_class` says what a class may carry.";

fn main() {
    let mut dir: Option<PathBuf> = None;
    let mut args = std::env::args().skip(1);
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--dir" => dir = args.next().map(PathBuf::from),
            "--version" | "-V" => {
                println!("cimmcp {}", env!("CARGO_PKG_VERSION"));
                return;
            }
            _ => {
                eprintln!("usage: cimmcp [--dir <model directory>]");
                eprintln!("An MCP server over stdio; an MCP client starts it.");
                std::process::exit(1);
            }
        }
    }
    let dir = dir
        .or_else(|| std::env::var_os("CIMOXIDE_MODEL_DIR").filter(|v| !v.is_empty()).map(PathBuf::from))
        .or_else(|| std::env::current_dir().ok())
        .unwrap_or_else(|| PathBuf::from("."));
    let mut session = Session::new(dir);

    let stdin = std::io::stdin();
    let mut out = std::io::stdout().lock();
    for line in stdin.lock().lines() {
        let Ok(line) = line else { break };
        if line.trim().is_empty() {
            continue;
        }
        let reply = match serde_json::from_str::<Value>(&line) {
            Ok(msg) => handle(&mut session, &msg),
            Err(e) => Some(error(Value::Null, -32700, &format!("parse error: {e}"))),
        };
        if let Some(reply) = reply {
            // serde_json writes no newlines inside a value, so one line is one message.
            let ok = serde_json::to_writer(&mut out, &reply).is_ok() && writeln!(out).is_ok() && out.flush().is_ok();
            if !ok {
                break;
            }
        }
    }
}

/// The reply to one message; `None` for a notification or a response.
fn handle(session: &mut Session, msg: &Value) -> Option<Value> {
    let method = msg.get("method").and_then(Value::as_str);
    let id = msg.get("id").cloned();
    let (Some(method), Some(id)) = (method, id) else {
        return None;
    };
    let params = msg.get("params").cloned().unwrap_or(Value::Null);
    let result = match method {
        "initialize" => {
            let asked = params.get("protocolVersion").and_then(Value::as_str);
            let version = asked.filter(|v| PROTOCOL_VERSIONS.contains(v)).unwrap_or(PROTOCOL_VERSIONS[0]);
            json!({
                "protocolVersion": version,
                "capabilities": { "tools": { "listChanged": false } },
                "serverInfo": { "name": "cimmcp", "version": env!("CARGO_PKG_VERSION") },
                "instructions": INSTRUCTIONS,
            })
        }
        "ping" => json!({}),
        "tools/list" => json!({ "tools": tools::definitions() }),
        "tools/call" => {
            let Some(name) = params.get("name").and_then(Value::as_str) else {
                return Some(error(id, -32602, "tools/call needs a tool name"));
            };
            let args = params.get("arguments").cloned().unwrap_or_else(|| json!({}));
            match tools::call(session, name, &args) {
                None => return Some(error(id, -32602, &format!("unknown tool: {name}"))),
                // A failing tool is a result the model reads, not a protocol error.
                Some(Ok(text)) => json!({ "content": [{ "type": "text", "text": text }], "isError": false }),
                Some(Err(text)) => json!({ "content": [{ "type": "text", "text": text }], "isError": true }),
            }
        }
        "resources/list" => json!({ "resources": [] }),
        "prompts/list" => json!({ "prompts": [] }),
        _ => return Some(error(id, -32601, &format!("method not found: {method}"))),
    };
    Some(json!({ "jsonrpc": "2.0", "id": id, "result": result }))
}

fn error(id: Value, code: i64, message: &str) -> Value {
    json!({ "jsonrpc": "2.0", "id": id, "error": { "code": code, "message": message } })
}
