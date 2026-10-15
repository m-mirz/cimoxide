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
    assert_eq!(names, ["list_model_sets", "summary", "find_objects", "get_object", "validate", "conflicts", "sparql", "export", "edit", "describe_class"]);
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

/// A model set listed as files and directories that lie apart (BE's
/// directory and the boundary's) is decoded and validated as those files
/// together, as `cimcli validate` takes them.
#[test]
fn takes_a_model_set_as_a_file_list() {
    const BASE: &str = "CGMES-Test-Configurations/v3.0/MicroGrid/MicroGid-BaseCase";
    let be = format!("{BASE}/MicroGrid-BE-MAS");
    let boundary = format!("{BASE}/MicroGrid-BD-MAS/20171002T0930Z_ENTSO-E_EQ_BD_2.xml");
    let mut files: Vec<PathBuf> = std::fs::read_dir(repo_root().join(&be)).unwrap().flatten().map(|e| e.path()).collect();
    files.sort();
    files.push(repo_root().join(&boundary));
    let paths: Vec<&Path> = files.iter().map(PathBuf::as_path).collect();
    let per_file = cimmodel::CimDataset::decode_files_parallel_separate(&paths).unwrap();
    let cfg = cimvalidation::combined_config(&per_file, None, None, true, true, Vec::new());
    let expected = cimvalidation::validate_files(per_file, &cfg);
    assert!(!expected.is_empty());

    let mut s = Server::start();
    let set = json!([be, boundary]);
    let summary = s.ok("summary", json!({ "files": set }));
    assert!(summary.starts_with(&format!("Model set {be} + {boundary} — 8 file(s)")), "{summary}");
    assert!(summary.contains("MicroGrid-BD-MAS/20171002T0930Z_ENTSO-E_EQ_BD_2.xml [EQBD]"), "{summary}");

    // A BE line ends at a boundary node, which only the boundary file defines.
    let node = s.ok("find_objects", json!({ "files": set, "class": "ConnectivityNode", "text": "Border" }));
    assert!(!node.starts_with("No matching"), "{node}");
    let alone = s.ok("find_objects", json!({ "path": be, "class": "ConnectivityNode", "text": "Border" }));
    assert!(alone.starts_with("No matching"), "{alone}");

    let text = s.ok("validate", json!({ "files": set, "common": true, "quality": true, "limit": 1000 }));
    assert!(text.starts_with(&format!("{} finding(s)", expected.len())), "{text}");
    for v in &expected {
        let line = format!("[{}] {} — {} — {} {}", v.severity, v.rule_id, v.message, v.class, v.object_id);
        assert!(text.contains(&line), "missing {line}\n{text}");
    }

    let (both, err) = s.call("summary", json!({ "path": be, "files": set }));
    assert!(err && both.contains("not both"), "{both}");
    let (missing, err) = s.call("summary", json!({ "files": [be, "nowhere.xml"] }));
    assert!(err && missing.contains("does not exist"), "{missing}");
    let (not_cim, err) = s.call("summary", json!({ "files": [be, "Cargo.toml"] }));
    assert!(err && not_cim.contains("is not a CGMES or NC RDF/XML file"), "{not_cim}");
}

/// An export is what `cimmodel::convert` writes for the same files: returned
/// when short, else written, inside the workspace and over a file only when
/// told to.
#[test]
fn exports_as_cimmodel_does() {
    const BASE: &str = "CGMES-Test-Configurations/v3.0/MicroGrid/MicroGid-BaseCase";
    let be = format!("{BASE}/MicroGrid-BE-MAS");
    let boundary = format!("{BASE}/MicroGrid-BD-MAS/20171002T0930Z_ENTSO-E_EQ_BD_2.xml");
    let mut files: Vec<PathBuf> = std::fs::read_dir(repo_root().join(&be)).unwrap().flatten().map(|e| e.path()).collect();
    files.sort();
    let paths: Vec<&Path> = files.iter().map(PathBuf::as_path).collect();
    let ssh = cimmodel::convert::dataset_to_xml_for_profile(&cimmodel::CimDataset::decode_files_parallel(&paths).unwrap(), "SSH").unwrap();
    files.push(repo_root().join(&boundary));
    let paths: Vec<&Path> = files.iter().map(PathBuf::as_path).collect();
    let whole = cimmodel::convert::dataset_to_xml(&cimmodel::CimDataset::decode_files_parallel(&paths).unwrap()).unwrap();

    let mut s = Server::start();
    assert_eq!(s.ok("export", json!({ "path": be, "profile": "SSH" })), ssh);
    let (long, err) = s.call("export", json!({ "path": be }));
    assert!(err && long.contains("give `output`"), "{long}");

    let dir = format!("target/cimmcp-export-{}", std::process::id());
    let out = format!("{dir}/all.xml");
    let set = json!([be, boundary]);
    let wrote = s.ok("export", json!({ "files": set, "output": out }));
    assert!(wrote.starts_with(&format!("Wrote {out} ")), "{wrote}");
    assert_eq!(std::fs::read_to_string(repo_root().join(&out)).unwrap(), whole);
    let (exists, err) = s.call("export", json!({ "files": set, "output": out }));
    assert!(err && exists.contains("set `overwrite`"), "{exists}");
    s.ok("export", json!({ "files": set, "output": out, "overwrite": true }));
    std::fs::remove_dir_all(repo_root().join(&dir)).unwrap();

    let (unknown, err) = s.call("export", json!({ "path": be, "profile": "XX" }));
    assert!(err && unknown.contains("unknown profile code"), "{unknown}");
    for outside in ["../outside.xml", "/tmp/outside.xml"] {
        let (refused, err) = s.call("export", json!({ "path": be, "output": outside }));
        assert!(err && refused.contains("output"), "{refused}");
    }
}

/// Which files define (`rdf:ID`) or describe (`rdf:about`) an object, and the
/// objects files define again or type with unrelated classes.
#[test]
fn reports_defining_files_and_conflicts() {
    let mut s = Server::start();
    let g1 = s.ok("get_object", json!({ "path": MICROGRID, "mrid": "3a3b27be-b18b-4385-b557-6735d733baf0" }));
    assert!(g1.contains("20210325T1530Z_1D_BE_EQ_001.xml — defines (rdf:ID) as SynchronousMachine"), "{g1}");
    assert!(g1.contains("20210325T1530Z_1D_BE_SSH_001.xml — describes (rdf:about) as SynchronousMachine"), "{g1}");
    let clean = s.ok("conflicts", json!({ "path": MICROGRID }));
    assert!(clean.starts_with("No conflicts across 12 file(s)"), "{clean}");

    // `_x` defined by both files; `_y` a line in one, a breaker in the other.
    let dir = format!("target/cimmcp-conflicts-{}", std::process::id());
    std::fs::create_dir_all(repo_root().join(&dir)).unwrap();
    let doc = |body: &str| {
        format!(
            "<rdf:RDF xmlns:rdf=\"http://www.w3.org/1999/02/22-rdf-syntax-ns#\" xmlns:cim=\"http://iec.ch/TC57/CIM100#\">\n{body}\n</rdf:RDF>\n"
        )
    };
    std::fs::write(
        repo_root().join(&dir).join("a.xml"),
        doc("<cim:Substation rdf:ID=\"_x\"><cim:IdentifiedObject.name>S</cim:IdentifiedObject.name></cim:Substation>\n<cim:ACLineSegment rdf:ID=\"_y\"/>"),
    )
    .unwrap();
    std::fs::write(
        repo_root().join(&dir).join("b.xml"),
        doc("<cim:Substation rdf:ID=\"_x\"/>\n<cim:Breaker rdf:about=\"#_y\"/>"),
    )
    .unwrap();
    let found = s.ok("conflicts", json!({ "path": dir }));
    std::fs::remove_dir_all(repo_root().join(&dir)).unwrap();
    assert!(found.contains("1 object(s) defined by more than one file:\n  Substation _x \"S\" — a.xml, b.xml\n"), "{found}");
    assert!(found.contains("1 object(s) typed with unrelated classes:\n  _y — ACLineSegment in a.xml; Breaker in b.xml\n"), "{found}");
}

/// An edit updates, copies, creates and deletes objects in order and writes
/// the result; the rest stays as read, and the set's files are never written.
#[test]
fn edits_and_writes_the_result() {
    const BASE: &str = "CGMES-Test-Configurations/v3.0/MicroGrid/MicroGid-BaseCase";
    let be = format!("{BASE}/MicroGrid-BE-MAS");
    let mut files: Vec<PathBuf> = std::fs::read_dir(repo_root().join(&be)).unwrap().flatten().map(|e| e.path()).collect();
    files.sort();
    let paths: Vec<&Path> = files.iter().map(PathBuf::as_path).collect();
    let before = cimmodel::CimDataset::decode_files_parallel(&paths).unwrap();
    let eq_file = files.iter().find(|f| f.to_string_lossy().contains("_EQ_")).unwrap();
    let eq = cimmodel::CimDataset::decode_file(eq_file).unwrap();
    let g1 = "_3a3b27be-b18b-4385-b557-6735d733baf0";
    let gone = eq.by_type["Substation"].iter().min().unwrap().clone();

    let dir = format!("target/cimmcp-edit-{}", std::process::id());
    let out = format!("{dir}/EQ.xml");
    let mut s = Server::start();
    let report = s.ok(
        "edit",
        json!({ "path": be, "profile": "EQ", "output": out, "changes": [
            { "id": g1.trim_start_matches('_'), "fields": { "IdentifiedObject.name": "BE-G1 renamed", "IdentifiedObject.description": null } },
            { "id": "_g1-copy", "from": g1, "fields": { "IdentifiedObject.name": "BE-G1 copy" } },
            { "id": "_new-region", "type": "GeographicalRegion", "fields": { "IdentifiedObject.name": "New" } },
            { "id": gone, "delete": true },
        ] }),
    );
    assert!(report.contains(&format!("updated {g1} (SynchronousMachine): IdentifiedObject.description, IdentifiedObject.name")), "{report}");
    assert!(report.contains(&format!("copied {g1} to _g1-copy (SynchronousMachine): IdentifiedObject.name; IdentifiedObject.mRID follows the new id")), "{report}");
    assert!(report.contains("created _new-region (GeographicalRegion): IdentifiedObject.name"), "{report}");
    assert!(report.contains(&format!("deleted {gone}")), "{report}");
    assert!(report.contains(&format!("Wrote {out} ")), "{report}");

    let after = cimmodel::CimDataset::decode_file(&repo_root().join(&out)).unwrap();
    let json = |ds: &cimmodel::CimDataset, id: &str| ds.entries[id].to_json_value();
    let renamed = json(&after, g1);
    assert_eq!(renamed["IdentifiedObject.name"], "BE-G1 renamed");
    assert!(renamed.get("IdentifiedObject.description").is_none());
    let copy = json(&after, "_g1-copy");
    assert_eq!(after.entries["_g1-copy"].type_name(), "SynchronousMachine");
    assert_eq!(copy["IdentifiedObject.mRID"], "g1-copy");
    assert_eq!(copy["RotatingMachine.GeneratingUnit"], renamed["RotatingMachine.GeneratingUnit"]);
    assert_eq!(after.entries["_new-region"].type_name(), "GeographicalRegion");
    assert!(!after.entries.contains_key(&gone));
    // Everything else is the EQ file as read.
    let unchanged = cimmodel::CimDataset::decode_str(&cimmodel::convert::dataset_to_xml_for_profile(&before, "EQ").unwrap()).unwrap();
    for (id, el) in &unchanged.entries {
        if ![g1, gone.as_str()].contains(&id.as_str()) {
            assert_eq!(json(&after, id), el.to_json_value(), "{id}");
        }
    }
    assert_eq!(after.entries.len(), unchanged.entries.len() + 1);

    // Only what changed, as a dataset of its own.
    let only = s.ok("edit", json!({ "path": be, "profile": "EQ", "keep": "changed", "changes": [
        { "id": "_new-region", "type": "GeographicalRegion", "fields": { "IdentifiedObject.name": "New" } },
    ] }));
    assert!(only.contains("kept the 1 changed object(s)"), "{only}");
    let xml = &only[only.find("<?xml").unwrap()..];
    let alone = cimmodel::CimDataset::decode_str(xml).unwrap();
    assert_eq!(alone.entries.keys().filter(|k| !alone.entries[*k].type_name().contains("FullModel")).count(), 1, "{xml}");

    for (change, error) in [
        (json!({ "id": g1, "from": g1 }), "a copy needs a new `id`"),
        (json!({ "id": "_nothing", "fields": { "IdentifiedObject.name": "x" } }), "give `type` to create it"),
        (json!({ "id": "_nothing", "type": "Generatr" }), "unknown class `Generatr`"),
        (json!({ "id": "_nothing", "delete": true }), "no object `_nothing` to delete"),
        (json!({ "id": g1, "fields": { "IdentifiedObject.name": 3 } }), "a string, an array of strings or null"),
    ] {
        let (text, err) = s.call("edit", json!({ "path": be, "profile": "EQ", "output": format!("{dir}/x.xml"), "changes": [change] }));
        assert!(err && text.starts_with("change 1: ") && text.contains(error), "{text}");
    }
    let own = eq_file.strip_prefix(repo_root()).unwrap().to_string_lossy().into_owned();
    let (text, err) = s.call("edit", json!({ "path": be, "output": own, "overwrite": true, "changes": [{ "id": g1 }] }));
    assert!(err && text.contains("a file of the model set"), "{text}");
    std::fs::remove_dir_all(repo_root().join(&dir)).unwrap();
}
