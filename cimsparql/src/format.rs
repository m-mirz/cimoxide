//! Serialising query results.
//!
//! SELECT results follow the SPARQL 1.1 Query Results formats so the CLI's output is
//! interoperable. CONSTRUCT/DESCRIBE results have no counterpart in those formats and are
//! always written as N-Triples.

use std::io::Write;

use oxigraph::model::Term;
use oxigraph::sparql::QueryResults;

use crate::CimSparqlError;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Format {
    /// SPARQL 1.1 Query Results JSON.
    Json,
    /// SPARQL 1.1 Query Results CSV.
    Csv,
    /// SPARQL 1.1 Query Results TSV.
    Tsv,
    /// Aligned columns for a terminal.
    Text,
}

impl Format {
    pub fn parse(s: &str) -> Option<Self> {
        match s.to_ascii_lowercase().as_str() {
            "json" => Some(Self::Json),
            "csv" => Some(Self::Csv),
            "tsv" => Some(Self::Tsv),
            "text" | "table" => Some(Self::Text),
            _ => None,
        }
    }
}

pub fn write(
    results: QueryResults<'_>,
    format: Format,
    out: &mut impl Write,
) -> Result<(), CimSparqlError> {
    match results {
        QueryResults::Boolean(b) => write_boolean(b, format, out),
        QueryResults::Solutions(solutions) => {
            let variables: Vec<String> =
                solutions.variables().iter().map(|v| v.as_str().to_string()).collect();
            let mut rows: Vec<Vec<Option<Term>>> = Vec::new();
            for solution in solutions {
                let solution = solution?;
                rows.push(
                    variables
                        .iter()
                        .map(|v| solution.get(v.as_str()).cloned())
                        .collect(),
                );
            }
            write_solutions(&variables, &rows, format, out)
        }
        QueryResults::Graph(triples) => {
            for triple in triples {
                writeln!(out, "{} .", triple?)?;
            }
            Ok(())
        }
    }
}

fn write_boolean(b: bool, format: Format, out: &mut impl Write) -> Result<(), CimSparqlError> {
    match format {
        Format::Json => writeln!(out, "{{\"head\":{{}},\"boolean\":{b}}}")?,
        _ => writeln!(out, "{b}")?,
    }
    Ok(())
}

fn write_solutions(
    variables: &[String],
    rows: &[Vec<Option<Term>>],
    format: Format,
    out: &mut impl Write,
) -> Result<(), CimSparqlError> {
    match format {
        Format::Json => {
            let bindings: Vec<serde_json::Value> = rows
                .iter()
                .map(|row| {
                    let mut obj = serde_json::Map::new();
                    for (name, term) in variables.iter().zip(row) {
                        if let Some(term) = term {
                            obj.insert(name.clone(), term_to_json(term));
                        }
                    }
                    serde_json::Value::Object(obj)
                })
                .collect();
            let doc = serde_json::json!({
                "head": { "vars": variables },
                "results": { "bindings": bindings },
            });
            writeln!(out, "{}", serde_json::to_string(&doc).unwrap())?;
        }
        Format::Csv => {
            writeln!(out, "{}", variables.join(","))?;
            for row in rows {
                let cells: Vec<String> =
                    row.iter().map(|t| csv_escape(&plain_value(t))).collect();
                writeln!(out, "{}", cells.join(","))?;
            }
        }
        Format::Tsv => {
            writeln!(out, "{}", variables.iter().map(|v| format!("?{v}")).collect::<Vec<_>>().join("\t"))?;
            for row in rows {
                let cells: Vec<String> = row
                    .iter()
                    .map(|t| t.as_ref().map(ToString::to_string).unwrap_or_default())
                    .collect();
                writeln!(out, "{}", cells.join("\t"))?;
            }
        }
        Format::Text => {
            let cells: Vec<Vec<String>> =
                rows.iter().map(|r| r.iter().map(plain_value).collect()).collect();
            let widths: Vec<usize> = variables
                .iter()
                .enumerate()
                .map(|(i, v)| {
                    cells
                        .iter()
                        .map(|r| r[i].chars().count())
                        .chain(std::iter::once(v.chars().count() + 1))
                        .max()
                        .unwrap_or(0)
                })
                .collect();
            let header: Vec<String> = variables
                .iter()
                .zip(&widths)
                .map(|(v, w)| format!("{:<w$}", format!("?{v}"), w = w))
                .collect();
            writeln!(out, "{}", header.join("  ").trim_end())?;
            writeln!(out, "{}", widths.iter().map(|w| "-".repeat(*w)).collect::<Vec<_>>().join("  "))?;
            for row in &cells {
                let line: Vec<String> = row
                    .iter()
                    .zip(&widths)
                    .map(|(c, w)| format!("{c:<w$}", w = w))
                    .collect();
                writeln!(out, "{}", line.join("  ").trim_end())?;
            }
            writeln!(out, "\n{} row(s)", rows.len())?;
        }
    }
    Ok(())
}

/// The lexical value of a term, without Turtle/N-Triples decoration.
fn plain_value(term: &Option<Term>) -> String {
    match term {
        None => String::new(),
        Some(Term::NamedNode(n)) => n.as_str().to_string(),
        Some(Term::BlankNode(b)) => format!("_:{}", b.as_str()),
        Some(Term::Literal(l)) => l.value().to_string(),
    }
}

fn term_to_json(term: &Term) -> serde_json::Value {
    match term {
        Term::NamedNode(n) => serde_json::json!({ "type": "uri", "value": n.as_str() }),
        Term::BlankNode(b) => serde_json::json!({ "type": "bnode", "value": b.as_str() }),
        Term::Literal(l) => {
            let mut obj = serde_json::Map::new();
            obj.insert("type".into(), "literal".into());
            obj.insert("value".into(), l.value().into());
            if let Some(lang) = l.language() {
                obj.insert("xml:lang".into(), lang.into());
            } else if l.datatype().as_str() != "http://www.w3.org/2001/XMLSchema#string" {
                obj.insert("datatype".into(), l.datatype().as_str().into());
            }
            serde_json::Value::Object(obj)
        }
    }
}

fn csv_escape(value: &str) -> String {
    if value.contains([',', '"', '\n', '\r']) {
        format!("\"{}\"", value.replace('"', "\"\""))
    } else {
        value.to_string()
    }
}
