//! `cimmcp` driven over stdio, as an MCP client drives it, against the
//! MicroGrid test configuration.

use std::io::{BufRead, BufReader, Write};
use std::path::{Path, PathBuf};
use std::process::{Child, ChildStdin, ChildStdout, Command, Stdio};

use serde_json::{Value, json};

const MICROGRID: &str = "CGMES-Test-Configurations/v3.0/MicroGrid/MicroGid-BaseCase/MicroGrid-BaseCase-Merged";

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap().to_path_buf()
}

struct Server {
    child: Child,
    stdin: ChildStdin,
    stdout: BufReader<ChildStdout>,
    next_id: u64,
}

impl Server {
    fn start() -> Self {
        let dir = repo_root().join(MICROGRID);
        assert!(dir.is_dir(), "missing test data {} — run `git submodule update --init`", dir.display());
        let mut child = Command::new(env!("CARGO_BIN_EXE_cimmcp"))
            .arg("--dir")
            .arg(repo_root())
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .expect("start cimmcp");
        let stdin = child.stdin.take().unwrap();
        let stdout = BufReader::new(child.stdout.take().unwrap());
        let mut server = Server { child, stdin, stdout, next_id: 1 };
        let init = server.request("initialize", json!({ "protocolVersion": "2025-06-18", "capabilities": {} }));
        assert_eq!(init["result"]["protocolVersion"], "2025-06-18");
        server.send(&json!({ "jsonrpc": "2.0", "method": "notifications/initialized" }));
        server
    }

    fn send(&mut self, msg: &Value) {
        writeln!(self.stdin, "{msg}").unwrap();
        self.stdin.flush().unwrap();
    }

    fn request(&mut self, method: &str, params: Value) -> Value {
        let id = self.next_id;
        self.next_id += 1;
        self.send(&json!({ "jsonrpc": "2.0", "id": id, "method": method, "params": params }));
        let mut line = String::new();
        self.stdout.read_line(&mut line).unwrap();
        let reply: Value = serde_json::from_str(&line).expect("one JSON message per line");
        assert_eq!(reply["id"], id);
        reply
    }

    /// A tool's text, and whether it is an error.
    fn call(&mut self, name: &str, args: Value) -> (String, bool) {
        let reply = self.request("tools/call", json!({ "name": name, "arguments": args }));
        let result = &reply["result"];
        (result["content"][0]["text"].as_str().unwrap().to_string(), result["isError"].as_bool().unwrap())
    }

    fn ok(&mut self, name: &str, args: Value) -> String {
        let (text, err) = self.call(name, args);
        assert!(!err, "{name} failed: {text}");
        text
    }
}

impl Drop for Server {
    fn drop(&mut self) {
        let _ = self.child.kill();
    }
}

#[test]
fn lists_tools_and_rejects_unknown_ones() {
    let mut s = Server::start();
    let list = s.request("tools/list", json!({}));
    let names: Vec<&str> = list["result"]["tools"].as_array().unwrap().iter().map(|t| t["name"].as_str().unwrap()).collect();
    assert_eq!(names, ["list_model_sets", "summary", "find_objects", "get_object", "validate", "sparql", "describe_class"]);
    let reply = s.request("tools/call", json!({ "name": "nope", "arguments": {} }));
    assert_eq!(reply["error"]["code"], -32602);
    let reply = s.request("no/such/method", json!({}));
    assert_eq!(reply["error"]["code"], -32601);
}

#[test]
fn finds_and_describes_objects() {
    let mut s = Server::start();
    let sets = s.ok("list_model_sets", json!({ "root": "CGMES-Test-Configurations/v3.0/MicroGrid/MicroGid-BaseCase" }));
    assert!(sets.contains("MicroGrid-BaseCase-Merged/"), "{sets}");
    assert!(sets.contains("20210325T1530Z_1D_BE_EQ_001.xml [EQ, SC]"), "{sets}");

    let summary = s.ok("summary", json!({ "path": MICROGRID }));
    assert!(summary.contains("12 file(s)"), "{summary}");
    assert!(summary.contains("SynchronousMachine"), "{summary}");

    // An abstract class finds its concrete descendants.
    let machines = s.ok("find_objects", json!({ "path": MICROGRID, "class": "RotatingMachine", "text": "BE-G1" }));
    assert!(machines.starts_with("1 matching object(s)"), "{machines}");
    assert!(machines.contains("SynchronousMachine _3a3b27be-b18b-4385-b557-6735d733baf0 \"BE-G1\""), "{machines}");

    // An attribute named without its class, a value as written.
    let terminals = s.ok("find_objects", json!({ "path": MICROGRID, "attribute": "connected", "value": "true", "limit": 1 }));
    assert!(terminals.contains("[ACDCTerminal.connected = true]"), "{terminals}");

    // The mRID without its `_`; references resolved both ways.
    let g1 = s.ok("get_object", json!({ "path": MICROGRID, "mrid": "3a3b27be-b18b-4385-b557-6735d733baf0" }));
    assert!(g1.contains("RotatingMachine.p = -90"), "{g1}");
    assert!(g1.contains("RotatingMachine.GeneratingUnit = → GeneratingUnit"), "{g1}");
    assert!(g1.contains("SynchronousMachine.operatingMode = SynchronousMachineOperatingMode.generator (Operating as generator.)"), "{g1}");
    assert!(g1.contains("via Terminal.ConductingEquipment"), "{g1}");

    let (missing, err) = s.call("get_object", json!({ "path": MICROGRID, "mrid": "_nothing" }));
    assert!(err, "{missing}");

    let breaker = s.ok("describe_class", json!({ "class": "Breaker" }));
    assert!(breaker.contains("Switch.open: Boolean (xsd:boolean)  {SSH}"), "{breaker}");
    assert!(breaker.contains("nc:Breaker (NC)"), "{breaker}");
    // The vocabulary's definitions: the class's, and each attribute's.
    assert!(breaker.contains("A mechanical switching device capable of making"), "{breaker}");
    assert!(breaker.contains("The attribute tells if the switch is considered open"), "{breaker}");
    let open = s.ok("describe_class", json!({ "class": "Breaker.open" }));
    assert!(open.starts_with("Switch.open (CGMES): Boolean (xsd:boolean)"), "{open}");
    let mode = s.ok("describe_class", json!({ "class": "SynchronousMachineOperatingMode" }));
    assert!(mode.contains("SynchronousMachineOperatingMode.condenser — Operating as condenser."), "{mode}");
    let generator = s.ok("describe_class", json!({ "class": "SynchronousMachineOperatingMode.generator" }));
    assert!(generator.contains("Operating as generator."), "{generator}");
    assert!(generator.contains("rdf:resource=\"http://iec.ch/TC57/CIM100#SynchronousMachineOperatingMode.generator\""), "{generator}");
    let (none, err) = s.call("describe_class", json!({ "class": "Breaker.bogus" }));
    assert!(err && none.contains("no attribute `bogus`"), "{none}");
    let (unknown, err) = s.call("find_objects", json!({ "path": MICROGRID, "class": "Generatr" }));
    assert!(err && unknown.starts_with("unknown class"), "{unknown}");
}

#[test]
fn queries_with_sparql() {
    let mut s = Server::start();
    let rows = s.ok(
        "sparql",
        json!({ "path": MICROGRID, "query": "SELECT ?g ?p WHERE { ?g a cim:SynchronousMachine ; cim:IdentifiedObject.name \"BE-G1\" ; cim:RotatingMachine.p ?p }" }),
    );
    assert_eq!(rows, "g\tp\n_3a3b27be-b18b-4385-b557-6735d733baf0\t-90\n(1 row(s))\n");
    let (bad, err) = s.call("sparql", json!({ "path": MICROGRID, "query": "SELEC" }));
    assert!(err && bad.starts_with("query failed"), "{bad}");
}

/// The findings, by rule, are those `cimcli validate --common --quality`
/// reports.
#[test]
fn validates_as_cimcli_does() {
    let dir = repo_root().join(MICROGRID);
    let mut files: Vec<PathBuf> = std::fs::read_dir(&dir).unwrap().flatten().map(|e| e.path()).filter(|p| p.extension().is_some_and(|x| x == "xml")).collect();
    files.sort();
    let paths: Vec<&Path> = files.iter().map(PathBuf::as_path).collect();
    let per_file = cimmodel::CimDataset::decode_files_parallel_separate(&paths).unwrap();
    let cfg = cimvalidation::combined_config(&per_file, None, None, true, true, Vec::new());
    let expected = cimvalidation::validate_files(per_file, &cfg);
    assert!(!expected.is_empty());

    let mut s = Server::start();
    let text = s.ok("validate", json!({ "path": MICROGRID, "common": true, "quality": true, "limit": 1000 }));
    assert!(text.starts_with(&format!("{} finding(s)", expected.len())), "{text}");
    for v in &expected {
        let line = format!("[{}] {} — {} — {} {}", v.severity, v.rule_id, v.message, v.class, v.object_id);
        assert!(text.contains(&line), "missing {line}\n{text}");
    }

    let one = s.ok("validate", json!({ "path": MICROGRID, "common": true, "quality": true, "rule": "Substation-count" }));
    assert!(one.starts_with(&format!("1 finding(s) of {} matching the filters", expected.len())), "{one}");
}
