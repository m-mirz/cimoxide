//! `cimcli validate --format sarif`: SARIF 2.1.0, one result per finding.
//!
//! Each result names its rule (with the rule's metadata in `tool.driver.rules`),
//! the object as a logical location, and every input file and line that writes
//! the object as a physical location — an EQ file and an SSH file may both
//! describe one. The decoder keeps no positions, so those come from a text
//! scan of the inputs for `rdf:ID` / `rdf:about`, made only here and only for
//! the objects reported.

use std::collections::{BTreeMap, HashMap, HashSet};
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};

use cimvalidation::Violation;
use serde_json::{json, Value};

const SCHEMA: &str = "https://docs.oasis-open.org/sarif/sarif/v2.1.0/errata01/os/schemas/sarif-schema-2.1.0.json";

/// The SARIF level of a SHACL severity: `sh:Info` is advisory, as in the text
/// output's exit code.
fn level(severity: &str) -> &'static str {
    match severity {
        "sh:Violation" => "error",
        "sh:Warning" => "warning",
        _ => "note",
    }
}

/// Where an object is written: (input file, 1-based line, the line's text).
type Sites = HashMap<String, Vec<(usize, usize, String)>>;

/// The element starts writing each of `wanted`, by the mRID the decoder gives
/// them: the text after the last `#` of `rdf:ID` or `rdf:about`.
fn sites(files: &[PathBuf], wanted: &HashSet<&str>) -> Sites {
    let mut out: Sites = HashMap::new();
    if wanted.is_empty() {
        return out;
    }
    for (fi, path) in files.iter().enumerate() {
        // Line by line: a whole EQ file held at once costs more than the scan.
        let Ok(file) = std::fs::File::open(path) else { continue };
        for (li, line) in BufReader::new(file).lines().enumerate() {
            let Ok(line) = line else { break };
            for attr in ["rdf:ID=\"", "rdf:about=\""] {
                let Some(start) = line.find(attr).map(|i| i + attr.len()) else { continue };
                let Some(len) = line[start..].find('"') else { continue };
                let value = &line[start..start + len];
                let mrid = value.rsplit_once('#').map_or(value, |(_, f)| f);
                if wanted.contains(mrid) {
                    out.entry(mrid.to_string()).or_default().push((fi, li + 1, line.trim().to_string()));
                }
            }
        }
    }
    out
}

/// An artifact URI: a relative path as given, with `/` separators; an absolute
/// one as a `file://` URI.
fn uri(path: &Path) -> String {
    let s = path.to_string_lossy().replace('\\', "/");
    if path.is_absolute() {
        format!("file://{}{s}", if s.starts_with('/') { "" } else { "/" })
    } else {
        s
    }
}

pub fn render(violations: &[Violation], files: &[PathBuf]) -> Value {
    // Rules in id order, so the output does not depend on finding order.
    let mut rules: BTreeMap<&str, &Violation> = BTreeMap::new();
    for v in violations {
        rules.entry(v.rule_id.as_str()).or_insert(v);
    }
    let index: HashMap<&str, usize> = rules.keys().enumerate().map(|(i, id)| (*id, i)).collect();
    let rule_defs: Vec<Value> = rules
        .values()
        .map(|v| {
            let mut r = json!({
                "id": v.rule_id,
                "name": v.name,
                "shortDescription": { "text": if v.name.is_empty() { &v.rule_id } else { &v.name } },
                "defaultConfiguration": { "level": level(&v.severity) },
            });
            if !v.description.is_empty() {
                r["fullDescription"] = json!({ "text": v.description });
            }
            r
        })
        .collect();

    let wanted: HashSet<&str> = violations.iter().map(|v| v.object_id.as_str()).collect();
    let sites = sites(files, &wanted);
    let uris: Vec<String> = files.iter().map(|p| uri(p)).collect();

    let results: Vec<Value> = violations
        .iter()
        .map(|v| {
            let logical = json!({
                "name": v.object_id,
                "fullyQualifiedName": if v.property.is_empty() { v.object_id.clone() } else { format!("{}/{}", v.object_id, v.property) },
                "kind": "object",
            });
            let mut locations: Vec<Value> = sites
                .get(&v.object_id)
                .into_iter()
                .flatten()
                .map(|(fi, line, text)| {
                    json!({
                        "physicalLocation": {
                            "artifactLocation": { "uri": uris[*fi], "index": fi },
                            "region": { "startLine": line, "snippet": { "text": text } },
                        },
                        "logicalLocations": [logical.clone()],
                    })
                })
                .collect();
            if locations.is_empty() {
                // A dataset-wide finding ("global") or an object no input writes.
                locations.push(json!({ "logicalLocations": [logical] }));
            }
            json!({
                "ruleId": v.rule_id,
                "ruleIndex": index[v.rule_id.as_str()],
                "level": level(&v.severity),
                "message": { "text": v.message },
                "locations": locations,
                "properties": { "class": v.class, "property": v.property, "severity": v.severity },
            })
        })
        .collect();

    json!({
        "$schema": SCHEMA,
        "version": "2.1.0",
        "runs": [{
            "tool": { "driver": {
                "name": "cimoxide",
                "version": env!("CARGO_PKG_VERSION"),
                "informationUri": env!("CARGO_PKG_REPOSITORY"),
                "rules": rule_defs,
            }},
            "artifacts": uris.iter().map(|u| json!({ "location": { "uri": u } })).collect::<Vec<_>>(),
            "invocations": [{ "executionSuccessful": true }],
            "results": results,
        }],
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn finding(object_id: &str, rule_id: &str, severity: &str) -> Violation {
        Violation {
            object_id: object_id.into(), rule_id: rule_id.into(), class: "ACLineSegment".into(),
            property: "ACLineSegment.r".into(), message: "m".into(), severity: severity.into(),
            name: "n".into(), description: String::new(),
        }
    }

    #[test]
    fn findings_point_at_every_file_writing_the_object() {
        let dir = std::env::temp_dir().join(format!("cimcli-sarif-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let eq = dir.join("eq.xml");
        let ssh = dir.join("ssh.xml");
        std::fs::write(&eq, "<rdf:RDF>\n  <cim:ACLineSegment rdf:ID=\"_x\">\n</rdf:RDF>\n").unwrap();
        std::fs::write(&ssh, "<rdf:RDF>\n\n  <cim:Equipment rdf:about=\"#_x\">\n  <cim:Line rdf:about=\"urn:uuid:y\">\n").unwrap();
        let files = [eq, ssh];
        let log = render(
            &[finding("_x", "b:rule", "sh:Violation"), finding("urn:uuid:y", "a:rule", "sh:Info"), finding("global", "a:rule", "sh:Warning")],
            &files,
        );
        std::fs::remove_dir_all(&dir).ok();

        let run = &log["runs"][0];
        // Rules sorted by id, and each result indexes its rule.
        assert_eq!(run["tool"]["driver"]["rules"][0]["id"], "a:rule");
        let results = run["results"].as_array().unwrap();
        assert_eq!(results[0]["ruleIndex"], 1);
        assert_eq!(results[0]["level"], "error");
        assert_eq!(results[1]["level"], "note");
        assert_eq!(results[2]["level"], "warning");

        let lines = |r: &Value| -> Vec<(u64, u64)> {
            r["locations"].as_array().unwrap().iter().filter_map(|l| {
                let p = &l["physicalLocation"];
                Some((p["artifactLocation"]["index"].as_u64()?, p["region"]["startLine"].as_u64()?))
            }).collect()
        };
        assert_eq!(lines(&results[0]), [(0, 2), (1, 3)], "both files, at the element's line");
        assert_eq!(lines(&results[1]), [(1, 4)]);
        assert!(lines(&results[2]).is_empty(), "a dataset-wide finding has no file");
        assert_eq!(results[2]["locations"][0]["logicalLocations"][0]["name"], "global");
    }
}
