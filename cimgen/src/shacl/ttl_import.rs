use std::collections::HashMap;
use std::path::Path;

use super::model::*;

// ---------------------------------------------------------------------------
// Public entry point
// ---------------------------------------------------------------------------

pub fn import_ttl_file(path: &Path) -> Result<FileResults, Box<dyn std::error::Error>> {
    let src = std::fs::read_to_string(path)
        .map_err(|e| format!("cannot read {}: {e}", path.display()))?;
    let file_name = path
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("")
        .to_string();

    let graph = parse_turtle(&src)?;
    let shapes = extract_shapes(&graph);
    Ok(FileResults { file_name, shapes })
}

// ---------------------------------------------------------------------------
// RDF value types
// ---------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub enum RdfVal {
    Iri(String),             // simplified IRI: "cim:ACLineSegment" or "<full>"
    Str(String),             // string literal
    Int(i64),
    Float(f64),
    #[allow(dead_code)]
    Bool(bool),
    List(Vec<RdfVal>),       // RDF collection ( ... )
}

impl RdfVal {
    fn as_iri(&self) -> Option<&str> {
        if let RdfVal::Iri(s) = self { Some(s) } else { None }
    }
    fn as_str(&self) -> Option<&str> {
        if let RdfVal::Str(s) = self { Some(s) } else { None }
    }
    fn as_int(&self) -> Option<i64> {
        if let RdfVal::Int(n) = self { Some(*n) } else { None }
    }
    fn as_float(&self) -> Option<f64> {
        match self {
            RdfVal::Float(f) => Some(*f),
            RdfVal::Int(n) => Some(*n as f64),
            _ => None,
        }
    }
    fn as_list(&self) -> Option<&Vec<RdfVal>> {
        if let RdfVal::List(v) = self { Some(v) } else { None }
    }
}

// A subject graph: simplified-IRI subject → [(simplified-IRI predicate, value)]
type Graph = HashMap<String, Vec<(String, RdfVal)>>;

// Return first value for a predicate on a subject, if any.
fn get_one<'g>(g: &'g Graph, subj: &str, pred: &str) -> Option<&'g RdfVal> {
    g.get(subj)?.iter().find(|(p, _)| p == pred).map(|(_, v)| v)
}

fn get_all<'g>(g: &'g Graph, subj: &str, pred: &str) -> Vec<&'g RdfVal> {
    g.get(subj)
        .map(|pairs| pairs.iter().filter(|(p, _)| p == pred).map(|(_, v)| v).collect())
        .unwrap_or_default()
}

// ---------------------------------------------------------------------------
// Lexer
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq)]
enum Token {
    IriRef(String),       // <...>
    PrefixedName(String), // prefix:local or :local
    BlankNodeLabel(String), // _:label
    StringLit(String),    // "..." or '...' or """...""" or '''...'''
    Integer(i64),
    Float(f64),
    Bool(bool),
    At,          // standalone @ (for @prefix / @base as keywords)
    Semicolon,
    Comma,
    Dot,
    LParen,
    RParen,
    LBracket,
    RBracket,
    DoubleCaret, // ^^
    A,           // keyword `a`
    PrefixDecl,  // @prefix
    BaseDecl,    // @base
}

struct Lexer<'a> {
    src: &'a [u8],
    pos: usize,
}

impl<'a> Lexer<'a> {
    fn new(src: &'a str) -> Self {
        Self { src: src.as_bytes(), pos: 0 }
    }

    fn peek(&self) -> Option<u8> {
        self.src.get(self.pos).copied()
    }

    fn peek2(&self) -> Option<u8> {
        self.src.get(self.pos + 1).copied()
    }

    fn advance(&mut self) -> Option<u8> {
        let c = self.peek()?;
        self.pos += 1;
        Some(c)
    }

    fn skip_whitespace_and_comments(&mut self) {
        loop {
            // Skip whitespace
            while self.peek().map_or(false, |c| c.is_ascii_whitespace()) {
                self.advance();
            }
            // Skip comment
            if self.peek() == Some(b'#') {
                while self.peek().map_or(false, |c| c != b'\n') {
                    self.advance();
                }
            } else {
                break;
            }
        }
    }

    fn read_iri_ref(&mut self) -> String {
        self.advance(); // consume <
        let start = self.pos;
        while self.peek().map_or(false, |c| c != b'>') {
            self.advance();
        }
        let s = std::str::from_utf8(&self.src[start..self.pos]).unwrap_or("").to_string();
        self.advance(); // consume >
        s
    }

    fn read_string(&mut self) -> String {
        let q = self.advance().unwrap(); // opening quote char
        // Check for triple quote
        let triple = self.peek() == Some(q) && self.peek2() == Some(q);
        if triple {
            self.advance(); self.advance(); // consume two more
        }
        let mut s = String::new();
        loop {
            if triple {
                if self.peek() == Some(q) && self.peek2() == Some(q)
                    && self.src.get(self.pos + 2).copied() == Some(q)
                {
                    self.advance(); self.advance(); self.advance();
                    break;
                }
            } else if self.peek() == Some(q) {
                self.advance();
                break;
            }
            if self.peek().is_none() { break; }
            let c = self.advance().unwrap();
            if c == b'\\' {
                match self.advance() {
                    Some(b'n') => s.push('\n'),
                    Some(b't') => s.push('\t'),
                    Some(b'r') => s.push('\r'),
                    Some(b'\\') => s.push('\\'),
                    Some(b'"') => s.push('"'),
                    Some(b'\'') => s.push('\''),
                    Some(esc) => { s.push('\\'); s.push(esc as char); }
                    None => break,
                }
            } else {
                s.push(c as char);
            }
        }
        s
    }

    fn read_blank_node_label(&mut self) -> String {
        self.advance(); // _
        self.advance(); // :
        let start = self.pos;
        while self.peek().map_or(false, |c| !c.is_ascii_whitespace() && c != b',' && c != b';' && c != b'.' && c != b')') {
            self.advance();
        }
        std::str::from_utf8(&self.src[start..self.pos]).unwrap_or("").to_string()
    }

    fn read_prefixed_name_or_keyword(&mut self) -> Token {
        let start = self.pos;
        loop {
            match self.peek() {
                None => break,
                Some(c) if c.is_ascii_whitespace() || c == b',' || c == b';'
                    || c == b'(' || c == b')' || c == b'[' || c == b']' || c == b'^' => break,
                Some(b'.') => {
                    // Only stop at '.' if the NEXT char is whitespace/EOF/special
                    // (so we don't chop "cim:Foo.bar" at the dot)
                    let next = self.src.get(self.pos + 1).copied();
                    let stop = match next {
                        None => true,
                        Some(n) => n.is_ascii_whitespace() || n == b'#' || n == b';'
                            || n == b',' || n == b')' || n == b']',
                    };
                    if stop { break; }
                    self.advance();
                }
                _ => { self.advance(); }
            }
        }
        let s = std::str::from_utf8(&self.src[start..self.pos]).unwrap_or("").to_string();
        match s.as_str() {
            "a" => Token::A,
            "true" => Token::Bool(true),
            "false" => Token::Bool(false),
            _ => Token::PrefixedName(s),
        }
    }

    fn read_number(&mut self, first: u8) -> Token {
        let neg = first == b'-';
        let mut s = String::new();
        if neg { s.push('-'); } else { s.push(first as char); }
        let mut is_float = false;
        while let Some(c) = self.peek() {
            if c.is_ascii_digit() { s.push(c as char); self.advance(); }
            else if c == b'.' && self.peek2().map_or(false, |n| n.is_ascii_digit()) {
                is_float = true; s.push('.'); self.advance();
            }
            else if c == b'e' || c == b'E' { is_float = true; s.push(c as char); self.advance(); }
            else { break; }
        }
        if is_float {
            Token::Float(s.parse().unwrap_or(0.0))
        } else {
            Token::Integer(s.parse().unwrap_or(0))
        }
    }

    fn next_token(&mut self) -> Option<Token> {
        self.skip_whitespace_and_comments();
        let c = self.peek()?;
        match c {
            b'<' => {
                Some(Token::IriRef(self.read_iri_ref()))
            }
            b'"' | b'\'' => {
                Some(Token::StringLit(self.read_string()))
            }
            b'_' if self.peek2() == Some(b':') => {
                Some(Token::BlankNodeLabel(self.read_blank_node_label()))
            }
            b'@' => {
                self.advance(); // consume @
                // Read the keyword part
                let start = self.pos;
                while self.peek().map_or(false, |c| c.is_ascii_alphabetic()) {
                    self.advance();
                }
                let kw = std::str::from_utf8(&self.src[start..self.pos]).unwrap_or("");
                match kw {
                    "prefix" => Some(Token::PrefixDecl),
                    "base" => Some(Token::BaseDecl),
                    _ => Some(Token::At),
                }
            }
            b';' => { self.advance(); Some(Token::Semicolon) }
            b',' => { self.advance(); Some(Token::Comma) }
            b'.' => { self.advance(); Some(Token::Dot) }
            b'(' => { self.advance(); Some(Token::LParen) }
            b')' => { self.advance(); Some(Token::RParen) }
            b'[' => { self.advance(); Some(Token::LBracket) }
            b']' => { self.advance(); Some(Token::RBracket) }
            b'^' if self.peek2() == Some(b'^') => {
                self.advance(); self.advance();
                Some(Token::DoubleCaret)
            }
            b'^' => { self.advance(); Some(Token::At) } // treat lone ^ as skip
            b'-' | b'+' => {
                let sign = self.advance().unwrap();
                if self.peek().map_or(false, |c| c.is_ascii_digit()) {
                    Some(self.read_number(sign))
                } else {
                    // treat as prefixed name start (unusual)
                    Some(Token::PrefixedName(String::from(sign as char)))
                }
            }
            c if c.is_ascii_digit() => {
                let d = self.advance().unwrap();
                Some(self.read_number(d))
            }
            _ => {
                Some(self.read_prefixed_name_or_keyword())
            }
        }
    }

    fn tokenize(mut self) -> Vec<Token> {
        let mut tokens = Vec::new();
        while let Some(tok) = self.next_token() {
            tokens.push(tok);
        }
        tokens
    }
}

// ---------------------------------------------------------------------------
// Parser: tokens → RDF graph
// ---------------------------------------------------------------------------

struct Parser {
    tokens: Vec<Token>,
    pos: usize,
    prefixes: HashMap<String, String>, // prefix → expanded IRI (with #)
    base: String,
    graph: Graph,
    bnode_counter: usize,
}

impl Parser {
    fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            pos: 0,
            prefixes: HashMap::new(),
            base: String::new(),
            graph: HashMap::new(),
            bnode_counter: 0,
        }
    }

    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.pos)
    }

    fn advance(&mut self) -> Option<&Token> {
        let t = self.tokens.get(self.pos)?;
        self.pos += 1;
        Some(t)
    }

    fn skip_until_statement_end(&mut self) {
        // Skip tokens until we see a Dot at statement level (heuristic: depth 0).
        let mut depth = 0i32;
        loop {
            match self.peek() {
                None => break,
                Some(Token::LParen) | Some(Token::LBracket) => { depth += 1; self.advance(); }
                Some(Token::RParen) | Some(Token::RBracket) => {
                    if depth > 0 { depth -= 1; }
                    self.advance();
                }
                Some(Token::Dot) if depth == 0 => { self.advance(); break; }
                _ => { self.advance(); }
            }
        }
    }

    fn new_bnode(&mut self) -> String {
        let id = format!("_:b{}", self.bnode_counter);
        self.bnode_counter += 1;
        id
    }

    fn simplify(&self, iri: &str) -> String {
        // If it's already a prefixed name, return as-is
        if !iri.starts_with("http") && !iri.starts_with("urn") && !iri.starts_with("_:") {
            return iri.to_string();
        }
        // Try to match against known prefixes
        for (prefix, expanded) in &self.prefixes {
            if iri.starts_with(expanded.as_str()) {
                return format!("{}:{}", prefix, &iri[expanded.len()..]);
            }
        }
        // Return full IRI in angle brackets
        format!("<{iri}>")
    }

    fn token_to_iri(&self, tok: &Token) -> Option<String> {
        match tok {
            Token::IriRef(s) => Some(self.simplify(s)),
            Token::PrefixedName(s) => Some(s.clone()),
            Token::A => Some("rdf:type".to_string()),
            _ => None,
        }
    }

    fn parse_collection(&mut self) -> RdfVal {
        // Already consumed '('
        let mut items = Vec::new();
        loop {
            match self.peek() {
                None | Some(Token::RParen) => { self.advance(); break; }
                Some(Token::LBracket) => {
                    self.advance();
                    let bnode = self.parse_blank_node_body();
                    items.push(RdfVal::Iri(bnode));
                }
                _ => {
                    if let Some(val) = self.parse_object() {
                        items.push(val);
                    } else {
                        self.advance(); // skip unknown
                    }
                }
            }
        }
        RdfVal::List(items)
    }

    fn parse_blank_node_body(&mut self) -> String {
        // Already consumed '['; parse predicate-object pairs until ']'
        let id = self.new_bnode();
        loop {
            match self.peek() {
                None | Some(Token::RBracket) => { self.advance(); break; }
                Some(Token::Semicolon) => { self.advance(); continue; }
                Some(Token::Dot) => { break; } // shouldn't happen inside []
                _ => {
                    // predicate
                    let pred = match self.advance().cloned() {
                        Some(t) => match self.token_to_iri(&t) {
                            Some(p) => p,
                            None => { self.skip_until_statement_end(); break; }
                        },
                        None => break,
                    };
                    // object(s)
                    loop {
                        if let Some(val) = self.parse_object() {
                            self.graph.entry(id.clone()).or_default().push((pred.clone(), val));
                        }
                        match self.peek() {
                            Some(Token::Comma) => { self.advance(); }
                            _ => break,
                        }
                    }
                }
            }
        }
        id
    }

    fn parse_object(&mut self) -> Option<RdfVal> {
        let tok = self.advance()?.clone();
        match &tok {
            Token::IriRef(s) => Some(RdfVal::Iri(self.simplify(s))),
            Token::PrefixedName(s) => Some(RdfVal::Iri(s.clone())),
            Token::A => Some(RdfVal::Iri("rdf:type".to_string())),
            Token::BlankNodeLabel(s) => Some(RdfVal::Iri(format!("_:{s}"))),
            Token::StringLit(s) => {
                let val = s.clone();
                // Consume optional language tag or datatype
                match self.peek() {
                    Some(Token::At) => { self.advance(); } // @langcode — Token::At already includes the tag
                    Some(Token::DoubleCaret) => {
                        self.advance(); // ^^
                        // Coerce to numeric type based on xsd datatype
                        if let Some(dt_tok) = self.advance().cloned() {
                            let dt = self.token_to_iri(&dt_tok).unwrap_or_default();
                            match dt.as_str() {
                                "xsd:float" | "xsd:double" | "xsd:decimal"
                                | "<http://www.w3.org/2001/XMLSchema#float>"
                                | "<http://www.w3.org/2001/XMLSchema#double>"
                                | "<http://www.w3.org/2001/XMLSchema#decimal>" => {
                                    if let Ok(f) = val.trim().parse::<f64>() {
                                        return Some(RdfVal::Float(f));
                                    }
                                }
                                "xsd:integer" | "xsd:int" | "xsd:long"
                                | "<http://www.w3.org/2001/XMLSchema#integer>"
                                | "<http://www.w3.org/2001/XMLSchema#int>"
                                | "<http://www.w3.org/2001/XMLSchema#long>" => {
                                    if let Ok(n) = val.trim().parse::<i64>() {
                                        return Some(RdfVal::Int(n));
                                    }
                                }
                                _ => {}
                            }
                        }
                    }
                    _ => {}
                }
                Some(RdfVal::Str(val))
            }
            Token::Integer(n) => Some(RdfVal::Int(*n)),
            Token::Float(f) => Some(RdfVal::Float(*f)),
            Token::Bool(b) => Some(RdfVal::Bool(*b)),
            Token::LParen => Some(self.parse_collection()),
            Token::LBracket => {
                let bnode = self.parse_blank_node_body();
                Some(RdfVal::Iri(bnode))
            }
            Token::DoubleCaret => {
                // Typed literal: value was already consumed; skip the type token
                self.advance();
                None
            }
            _ => None,
        }
    }

    fn parse(&mut self) {
        loop {
            match self.peek().cloned() {
                None => break,
                Some(Token::PrefixDecl) => {
                    self.advance();
                    // prefix name: <uri>
                    let prefix = match self.advance().cloned() {
                        Some(Token::PrefixedName(s)) => {
                            s.trim_end_matches(':').to_string()
                        }
                        _ => { self.skip_until_statement_end(); continue; }
                    };
                    let expanded = match self.advance().cloned() {
                        Some(Token::IriRef(s)) => s,
                        _ => { self.skip_until_statement_end(); continue; }
                    };
                    // Consume optional dot
                    if self.peek() == Some(&Token::Dot) { self.advance(); }
                    self.prefixes.insert(prefix, expanded);
                }
                Some(Token::BaseDecl) => {
                    self.advance();
                    if let Some(Token::IriRef(s)) = self.advance().cloned() {
                        self.base = s;
                    }
                    if self.peek() == Some(&Token::Dot) { self.advance(); }
                }
                Some(Token::Dot) => { self.advance(); }
                Some(Token::Semicolon) | Some(Token::Comma) => { self.advance(); }
                _ => {
                    // Parse a subject
                    let subj = match self.advance().cloned() {
                        Some(Token::IriRef(s)) => self.simplify(&s),
                        Some(Token::PrefixedName(s)) => s,
                        Some(Token::BlankNodeLabel(s)) => format!("_:{s}"),
                        Some(Token::LBracket) => self.parse_blank_node_body(),
                        Some(Token::A) => "rdf:type".to_string(),
                        _ => { self.skip_until_statement_end(); continue; }
                    };

                    // Parse predicate-object pairs
                    loop {
                        // End of statement?
                        match self.peek() {
                            None | Some(Token::Dot) => { self.advance(); break; }
                            Some(Token::Semicolon) => { self.advance(); continue; }
                            _ => {}
                        }

                        let pred_tok = match self.advance().cloned() {
                            Some(t) => t,
                            None => break,
                        };
                        let pred = match self.token_to_iri(&pred_tok) {
                            Some(p) => p,
                            None => { self.skip_until_statement_end(); break; }
                        };

                        // Object(s)
                        loop {
                            if let Some(val) = self.parse_object() {
                                self.graph.entry(subj.clone()).or_default().push((pred.clone(), val));
                            }
                            // Typed literal suffix after string
                            match self.peek() {
                                Some(Token::Comma) => { self.advance(); }
                                _ => break,
                            }
                        }

                        // After object(s), expect ; or .
                        match self.peek() {
                            Some(Token::Semicolon) => { self.advance(); }
                            Some(Token::Dot) | None => {}
                            _ => {}
                        }
                    }
                }
            }
        }
    }
}

fn parse_turtle(src: &str) -> Result<Graph, Box<dyn std::error::Error>> {
    let tokens = Lexer::new(src).tokenize();
    let mut parser = Parser::new(tokens);
    parser.parse();
    Ok(parser.graph)
}

// ---------------------------------------------------------------------------
// Shape extraction
// ---------------------------------------------------------------------------

const SH_NODE_SHAPE: &str = "sh:NodeShape";
const SH_PROPERTY_SHAPE: &str = "sh:PropertyShape";
const RDF_TYPE: &str = "rdf:type";

fn extract_shapes(g: &Graph) -> Vec<ShapeInfo> {
    // Find all NodeShape subjects
    let mut node_shapes: Vec<String> = g
        .keys()
        .filter(|s| {
            get_all(g, s, RDF_TYPE)
                .iter()
                .any(|v| v.as_iri().map_or(false, |i| i == SH_NODE_SHAPE))
        })
        .cloned()
        .collect();
    node_shapes.sort();

    node_shapes
        .into_iter()
        .filter_map(|id| build_node_shape(g, &id))
        .collect()
}

fn build_node_shape(g: &Graph, id: &str) -> Option<ShapeInfo> {
    // Targets from sh:targetClass (deduplicated — same subject may appear in multiple statements)
    let mut seen_targets = std::collections::HashSet::new();
    let targets: Vec<TargetInfo> = get_all(g, id, "sh:targetClass")
        .into_iter()
        .filter_map(|v| v.as_iri())
        .filter(|iri| seen_targets.insert(iri.to_string()))
        .map(|iri| TargetInfo {
            kind: "targetClass".to_string(),
            value: iri.to_string(),
        })
        .collect();

    if targets.is_empty() {
        return None; // Skip shapes with no targetClass
    }

    // Nested property shapes from sh:property (may be List or individual IRIs)
    let prop_refs: Vec<String> = collect_iri_list(g, id, "sh:property");

    let mut properties: Vec<ShapeInfo> = prop_refs
        .into_iter()
        .filter_map(|prop_id| build_property_shape(g, &prop_id))
        .collect();

    // Also handle sh:or / sh:and / sh:not / sh:xone directly on a NodeShape
    // by extracting any sh:PropertyShape constraints inside
    for nested_id in nested_shape_ids(g, id) {
        if let Some(ps) = build_property_shape(g, &nested_id) {
            properties.push(ps);
        }
    }

    let name = get_str(g, id, "sh:name").unwrap_or_default();
    let description = get_str(g, id, "sh:description").unwrap_or_default();

    Some(ShapeInfo {
        id: id.to_string(),
        targets,
        path: Vec::new(),
        name,
        description,
        constraints: Vec::new(),
        properties,
    })
}

fn build_property_shape(g: &Graph, id: &str) -> Option<ShapeInfo> {
    // Confirm it's a PropertyShape (or unnamed shape with sh:path)
    let is_prop = get_all(g, id, RDF_TYPE)
        .iter()
        .any(|v| v.as_iri().map_or(false, |i| i == SH_PROPERTY_SHAPE));
    let has_path = g.get(id).map_or(false, |pairs| pairs.iter().any(|(p, _)| p == "sh:path"));
    if !is_prop && !has_path && !id.starts_with("_:") {
        return None;
    }

    let path = extract_path(g, id);
    let name = get_str(g, id, "sh:name").unwrap_or_default();
    let description = get_str(g, id, "sh:description").unwrap_or_default();
    let message = get_str(g, id, "sh:message").unwrap_or_default();
    let severity = get_str(g, id, "sh:severity")
        .or_else(|| get_all(g, id, "sh:severity").into_iter().find_map(|v| v.as_iri().map(str::to_string)))
        .unwrap_or_else(|| "sh:Violation".to_string());

    // Collect constraints from this shape
    let mut constraints = Vec::new();

    // sh:minCount / sh:maxCount
    if let Some(min) = get_one(g, id, "sh:minCount").and_then(|v| v.as_int()) {
        let max = get_one(g, id, "sh:maxCount").and_then(|v| v.as_int());
        let component = match (min, max) {
            (1, Some(1)) => "sh:RequiredConstraintComponent",
            (n, Some(m)) if n == m => "sh:ExactCountConstraintComponent",
            (n, _) if n > 0 => "sh:MinCountConstraintComponent",
            _ => "sh:MinCountConstraintComponent",
        };
        let mut payload = HashMap::new();
        payload.insert("minCount".to_string(), ShaclValue::Int(min));
        if let Some(m) = max {
            payload.insert("maxCount".to_string(), ShaclValue::Int(m));
        }
        constraints.push(ConstraintInfo {
            path: path.clone(),
            severity: severity.clone(),
            message: message.clone(),
            name: name.clone(),
            description: description.clone(),
            component: component.to_string(),
            payload,
        });
    } else if let Some(max) = get_one(g, id, "sh:maxCount").and_then(|v| v.as_int()) {
        let mut payload = HashMap::new();
        payload.insert("maxCount".to_string(), ShaclValue::Int(max));
        constraints.push(ConstraintInfo {
            path: path.clone(),
            severity: severity.clone(),
            message: message.clone(),
            name: name.clone(),
            description: description.clone(),
            component: "sh:MaxCountConstraintComponent".to_string(),
            payload,
        });
    }

    // sh:datatype
    if let Some(dt) = get_one(g, id, "sh:datatype").and_then(|v| v.as_iri()) {
        let mut payload = HashMap::new();
        payload.insert("datatype".to_string(), ShaclValue::Str(dt.to_string()));
        constraints.push(ConstraintInfo {
            path: path.clone(),
            severity: severity.clone(),
            message: message.clone(),
            name: name.clone(),
            description: description.clone(),
            component: "sh:DatatypeConstraintComponent".to_string(),
            payload,
        });
    }

    // sh:nodeKind
    if let Some(nk) = get_one(g, id, "sh:nodeKind").and_then(|v| v.as_iri()) {
        let mut payload = HashMap::new();
        payload.insert("nodeKind".to_string(), ShaclValue::Str(nk.to_string()));
        constraints.push(ConstraintInfo {
            path: path.clone(),
            severity: severity.clone(),
            message: message.clone(),
            name: name.clone(),
            description: description.clone(),
            component: "sh:NodeKindConstraintComponent".to_string(),
            payload,
        });
    }

    // sh:class
    for class_val in get_all(g, id, "sh:class") {
        if let Some(class) = class_val.as_iri() {
            let mut payload = HashMap::new();
            payload.insert("class".to_string(), ShaclValue::Str(class.to_string()));
            constraints.push(ConstraintInfo {
                path: path.clone(),
                severity: severity.clone(),
                message: message.clone(),
                name: name.clone(),
                description: description.clone(),
                component: "sh:ClassConstraintComponent".to_string(),
                payload,
            });
        }
    }

    // sh:in
    if let Some(in_list) = get_one(g, id, "sh:in").and_then(|v| v.as_list()) {
        let values: Vec<String> = in_list
            .iter()
            .filter_map(|v| v.as_iri().map(str::to_string).or_else(|| v.as_str().map(str::to_string)))
            .collect();
        let mut payload = HashMap::new();
        payload.insert("in".to_string(), ShaclValue::List(values));
        constraints.push(ConstraintInfo {
            path: path.clone(),
            severity: severity.clone(),
            message: message.clone(),
            name: name.clone(),
            description: description.clone(),
            component: "sh:InConstraintComponent".to_string(),
            payload,
        });
    }

    // sh:hasValue
    if let Some(v) = get_one(g, id, "sh:hasValue") {
        let val = v.as_iri().map(str::to_string).or_else(|| v.as_str().map(str::to_string));
        if let Some(hv) = val {
            let mut payload = HashMap::new();
            payload.insert("hasValue".to_string(), ShaclValue::Str(hv));
            constraints.push(ConstraintInfo {
                path: path.clone(),
                severity: severity.clone(),
                message: message.clone(),
                name: name.clone(),
                description: description.clone(),
                component: "sh:HasValueConstraintComponent".to_string(),
                payload,
            });
        }
    }

    // sh:pattern
    if let Some(pat) = get_str(g, id, "sh:pattern") {
        let mut payload = HashMap::new();
        payload.insert("pattern".to_string(), ShaclValue::Str(pat));
        if let Some(flags) = get_str(g, id, "sh:flags") {
            payload.insert("flags".to_string(), ShaclValue::Str(flags));
        }
        constraints.push(ConstraintInfo {
            path: path.clone(),
            severity: severity.clone(),
            message: message.clone(),
            name: name.clone(),
            description: description.clone(),
            component: "sh:PatternConstraintComponent".to_string(),
            payload,
        });
    }

    // sh:minLength / sh:maxLength
    if let Some(n) = get_one(g, id, "sh:minLength").and_then(|v| v.as_int()) {
        let mut payload = HashMap::new();
        payload.insert("minLength".to_string(), ShaclValue::Int(n));
        constraints.push(ConstraintInfo {
            path: path.clone(),
            severity: severity.clone(),
            message: message.clone(),
            name: name.clone(),
            description: description.clone(),
            component: "sh:MinLengthConstraintComponent".to_string(),
            payload,
        });
    }
    if let Some(n) = get_one(g, id, "sh:maxLength").and_then(|v| v.as_int()) {
        let mut payload = HashMap::new();
        payload.insert("maxLength".to_string(), ShaclValue::Int(n));
        constraints.push(ConstraintInfo {
            path: path.clone(),
            severity: severity.clone(),
            message: message.clone(),
            name: name.clone(),
            description: description.clone(),
            component: "sh:MaxLengthConstraintComponent".to_string(),
            payload,
        });
    }

    // sh:minExclusive / sh:maxExclusive / sh:minInclusive / sh:maxInclusive
    for (pred, comp) in &[
        ("sh:minExclusive", "sh:MinExclusiveConstraintComponent"),
        ("sh:maxExclusive", "sh:MaxExclusiveConstraintComponent"),
        ("sh:minInclusive", "sh:MinInclusiveConstraintComponent"),
        ("sh:maxInclusive", "sh:MaxInclusiveConstraintComponent"),
    ] {
        if let Some(val) = get_one(g, id, pred).and_then(|v| v.as_float()) {
            let mut payload = HashMap::new();
            payload.insert(pred.trim_start_matches("sh:").to_string(), ShaclValue::Float(val));
            constraints.push(ConstraintInfo {
                path: path.clone(),
                severity: severity.clone(),
                message: message.clone(),
                name: name.clone(),
                description: description.clone(),
                component: comp.to_string(),
                payload,
            });
        }
    }

    // sh:or — collect class values from blank-node items
    for or_val in get_all(g, id, "sh:or") {
        if let Some(list) = or_val.as_list() {
            let classes: Vec<String> = list
                .iter()
                .filter_map(|item| {
                    item.as_iri().and_then(|bnode_id| {
                        get_one(g, bnode_id, "sh:class")
                            .and_then(|v| v.as_iri())
                            .map(str::to_string)
                    })
                })
                .collect();
            if !classes.is_empty() {
                let mut payload = HashMap::new();
                payload.insert("classes".to_string(), ShaclValue::List(classes));
                constraints.push(ConstraintInfo {
                    path: path.clone(),
                    severity: severity.clone(),
                    message: message.clone(),
                    name: name.clone(),
                    description: description.clone(),
                    component: "sh:OrClassConstraintComponent".to_string(),
                    payload,
                });
            }
            // sh:or with inverse path cardinality
            let inv_items: Vec<_> = list
                .iter()
                .filter_map(|item| item.as_iri())
                .filter(|bnode_id| {
                    g.get(*bnode_id).map_or(false, |pairs| {
                        pairs.iter().any(|(p, _)| p == "sh:path")
                    })
                })
                .collect();
            if !inv_items.is_empty() {
                // Complex inverse cardinality — emit as-is for future handling
                let mut payload = HashMap::new();
                payload.insert("complex".to_string(), ShaclValue::Str("sh:or+inversePath".to_string()));
                constraints.push(ConstraintInfo {
                    path: path.clone(),
                    severity: severity.clone(),
                    message: message.clone(),
                    name: name.clone(),
                    description: description.clone(),
                    component: "sh:OrInversePathConstraintComponent".to_string(),
                    payload,
                });
            }
        }
    }

    // sh:lessThan
    if let Some(lt_val) = get_one(g, id, "sh:lessThan").and_then(|v| v.as_iri()) {
        let mut payload = HashMap::new();
        payload.insert("lessThan".to_string(), ShaclValue::Str(lt_val.to_string()));
        constraints.push(ConstraintInfo {
            path: path.clone(),
            severity: severity.clone(),
            message: message.clone(),
            name: name.clone(),
            description: description.clone(),
            component: "sh:LessThanConstraintComponent".to_string(),
            payload,
        });
    }

    // sh:not with sh:class inside → NotClassConstraintComponent
    for not_val in get_all(g, id, "sh:not") {
        if let Some(bnode_id) = not_val.as_iri() {
            if let Some(class_val) = get_one(g, bnode_id, "sh:class") {
                if let Some(class) = class_val.as_iri() {
                    let mut payload = HashMap::new();
                    payload.insert("class".to_string(), ShaclValue::Str(class.to_string()));
                    constraints.push(ConstraintInfo {
                        path: path.clone(),
                        severity: severity.clone(),
                        message: message.clone(),
                        name: name.clone(),
                        description: description.clone(),
                        component: "sh:NotClassConstraintComponent".to_string(),
                        payload,
                    });
                }
            }
        }
    }

    if constraints.is_empty() {
        return None;
    }

    Some(ShapeInfo {
        id: id.to_string(),
        targets: Vec::new(),
        path,
        name,
        description,
        constraints,
        properties: Vec::new(),
    })
}

/// Extract the path from sh:path on a shape — returns simplified IRI segments.
/// Inverse paths are encoded as "~<forward-iri>" (e.g. "~cim:Terminal.TopologicalNode").
fn extract_path(g: &Graph, id: &str) -> Vec<String> {
    let val = match get_one(g, id, "sh:path") {
        Some(v) => v,
        None => return Vec::new(),
    };
    match val {
        RdfVal::Iri(iri) if iri.starts_with("_:") => {
            // Blank node — unpack sh:inversePath if present
            if let Some(RdfVal::Iri(inv)) = get_one(g, iri, "sh:inversePath") {
                vec![format!("~{inv}")]
            } else {
                Vec::new()
            }
        }
        RdfVal::Iri(iri) => vec![iri.clone()],
        RdfVal::List(items) => items
            .iter()
            .filter_map(|item| item.as_iri().map(str::to_string))
            .collect(),
        _ => Vec::new(),
    }
}

/// Collect all IRI values for a predicate, including from List objects.
fn collect_iri_list(g: &Graph, subj: &str, pred: &str) -> Vec<String> {
    let mut result = Vec::new();
    for val in get_all(g, subj, pred) {
        match val {
            RdfVal::Iri(s) => result.push(s.clone()),
            RdfVal::List(items) => {
                for item in items {
                    if let Some(iri) = item.as_iri() {
                        result.push(iri.to_string());
                    }
                }
            }
            _ => {}
        }
    }
    result
}

/// Collect any sh:PropertyShape IDs nested inside sh:or / sh:and / sh:not / sh:xone.
fn nested_shape_ids(g: &Graph, id: &str) -> Vec<String> {
    let mut result = Vec::new();
    for pred in &["sh:or", "sh:and", "sh:not", "sh:xone"] {
        for val in get_all(g, id, pred) {
            match val {
                RdfVal::List(items) => {
                    for item in items {
                        if let Some(bnode) = item.as_iri() {
                            let is_prop = get_all(g, bnode, RDF_TYPE)
                                .iter()
                                .any(|v| v.as_iri().map_or(false, |i| i == SH_PROPERTY_SHAPE));
                            if is_prop || g.get(bnode).map_or(false, |p| p.iter().any(|(k,_)| k == "sh:path")) {
                                result.push(bnode.to_string());
                            }
                        }
                    }
                }
                RdfVal::Iri(bnode) => {
                    if bnode.starts_with("_:") {
                        result.push(bnode.clone());
                    }
                }
                _ => {}
            }
        }
    }
    result
}

fn get_str(g: &Graph, subj: &str, pred: &str) -> Option<String> {
    get_one(g, subj, pred).and_then(|v| v.as_str().map(str::to_string)
        .or_else(|| v.as_iri().map(|s| s.to_string())))
}
