use std::error::Error;
use std::fmt::Write as FmtWrite;

use cimdecoder::{CimDataset, CimEntry};
use cimstructs::base::FieldValue;
use cimstructs::constants::CIM_NAMESPACES;
use cimstructs::registry::json_registry;

pub fn dataset_to_json(ds: &CimDataset) -> serde_json::Value {
    let mut map = serde_json::Map::new();
    for (mrid, entry) in &ds.entries {
        let mut obj = entry
            .element
            .to_json_value()
            .as_object()
            .cloned()
            .unwrap_or_default();
        obj.insert("_type".into(), entry.element.type_name().into());
        map.insert(mrid.clone(), serde_json::Value::Object(obj));
    }
    serde_json::Value::Object(map)
}

pub fn dataset_from_json(json: &str) -> Result<CimDataset, Box<dyn Error>> {
    let root: serde_json::Map<String, serde_json::Value> = serde_json::from_str(json)?;
    let reg = json_registry();
    let mut ds = CimDataset::new();
    for (mrid, val) in root {
        let type_name = val["_type"].as_str().unwrap_or("").to_string();
        if let Some(f) = reg.get(type_name.as_str()) {
            let element = f(val)?;
            let block = element.to_block();
            ds.by_type
                .entry(type_name)
                .or_default()
                .push(mrid.clone());
            ds.entries.insert(mrid, CimEntry { element, block });
        }
    }
    Ok(ds)
}

pub fn dataset_to_xml(ds: &CimDataset) -> Result<String, Box<dyn Error>> {
    let mut out = String::new();
    out.push_str("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n");
    out.push_str("<rdf:RDF");
    for (prefix, uri) in CIM_NAMESPACES {
        write!(out, " xmlns:{prefix}=\"{uri}\"")?;
    }
    out.push_str(">\n");

    let mut mrids: Vec<&str> = ds.entries.keys().map(String::as_str).collect();
    mrids.sort();

    for mrid in mrids {
        let entry = &ds.entries[mrid];
        let block = entry.element.to_block();
        write!(out, "  <cim:{} rdf:about=\"#{}\">", block.type_name, escape_attr(mrid))?;

        // Emit fields sorted for deterministic output
        let mut fields: Vec<(&String, &FieldValue)> = block.fields.iter().collect();
        fields.sort_by_key(|(k, _)| k.as_str());

        let mut children = String::new();
        for (key, val) in fields {
            match val {
                FieldValue::Text(s) => {
                    write!(
                        children,
                        "\n    <cim:{key}>{}</cim:{key}>",
                        escape_text(s)
                    )?;
                }
                FieldValue::Resource(r) => {
                    write!(
                        children,
                        "\n    <cim:{key} rdf:resource=\"#{}\"/>",
                        escape_attr(r)
                    )?;
                }
                FieldValue::ResourceList(rs) => {
                    for r in rs {
                        write!(
                            children,
                            "\n    <cim:{key} rdf:resource=\"#{}\"/>",
                            escape_attr(r)
                        )?;
                    }
                }
            }
        }

        if children.is_empty() {
            write!(out, "/>\n")?;
        } else {
            write!(out, "{children}\n  </cim:{}>\n", block.type_name)?;
        }
    }

    out.push_str("</rdf:RDF>\n");
    Ok(out)
}

fn escape_text(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}

fn escape_attr(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('"', "&quot;")
}
