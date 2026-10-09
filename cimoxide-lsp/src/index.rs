//! Where things are written in an RDF/XML document.
//!
//! The decoder keeps no positions — it is the hot path, and nothing else
//! needs them — so the server makes its own pass with quick-xml over each
//! document it shows. The index records, as LSP ranges, every top-level
//! element (an object), its `rdf:ID` / `rdf:about`, its field elements and
//! their `rdf:resource` targets. mRIDs follow the decoder: the text after the
//! last `#`.
//!
//! A document being edited is often not well-formed; the pass stops at the
//! first syntax error and keeps what it has.

use lsp_types::{Position, Range};
use quick_xml::events::Event;
use quick_xml::Reader;

/// `rdf:ID` defines an object; `rdf:about` describes one defined elsewhere.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IdKind {
    Id,
    About,
}

#[derive(Debug)]
pub struct Element {
    /// The tag as written, e.g. `cim:ACLineSegment`.
    pub qname: String,
    pub mrid: String,
    pub id_kind: Option<IdKind>,
    /// The start tag.
    pub head: Range,
    /// The value of `rdf:ID` / `rdf:about`, without quotes.
    pub id_range: Option<Range>,
    /// Start tag to end tag.
    pub full: Range,
    pub fields: Vec<Field>,
    /// `IdentifiedObject.name`, for labels.
    pub name: Option<String>,
}

#[derive(Debug)]
pub struct Field {
    /// The tag as written, e.g. `cim:ACLineSegment.r`.
    pub qname: String,
    /// The start tag.
    pub head: Range,
    /// Start tag to end tag.
    pub full: Range,
    /// The `rdf:resource` target as an mRID, and the range of its value.
    pub resource: Option<(String, Range)>,
}

impl Field {
    /// The field key the decoder stores this field under: the local name.
    pub fn key(&self) -> &str {
        local(&self.qname)
    }
}

#[derive(Debug, Default)]
pub struct Index {
    /// `xmlns` bindings of the root element: (prefix, IRI); `""` for the
    /// default namespace.
    pub namespaces: Vec<(String, String)>,
    pub elements: Vec<Element>,
}

/// The local part of a qualified name.
pub fn local(qname: &str) -> &str {
    qname.rsplit_once(':').map_or(qname, |(_, l)| l)
}

/// The prefix of a qualified name, `""` when it has none.
pub fn prefix(qname: &str) -> &str {
    qname.split_once(':').map_or("", |(p, _)| p)
}

/// An IRI or fragment as the decoder keys it: the text after the last `#`.
pub fn mrid_of(value: &str) -> &str {
    value.rsplit_once('#').map_or(value, |(_, f)| f)
}

/// Whether `text` looks like CGMES or NC RDF/XML: it binds one of the two
/// CIM namespaces near the top.
pub fn is_cim(text: &str) -> bool {
    let mut end = text.len().min(8192);
    while !text.is_char_boundary(end) {
        end -= 1;
    }
    let head = &text[..end];
    head.contains("http://iec.ch/TC57/") || head.contains("https://cim.ucaiug.io/ns#")
}

impl Index {
    pub fn build(text: &str) -> Self {
        let lines = LineIndex::new(text);
        let mut ix = Index::default();
        let mut reader = Reader::from_str(text);
        reader.config_mut().check_end_names = false;

        let mut depth = 0u32;
        let mut current: Option<Element> = None;
        let mut field: Option<Field> = None;
        let mut name_text: Option<String> = None;
        loop {
            let start = reader.buffer_position() as usize;
            let event = match reader.read_event() {
                Ok(Event::Eof) | Err(_) => break,
                Ok(e) => e,
            };
            let end = reader.buffer_position() as usize;
            let range = || lines.range(text, start, end);
            let empty = matches!(event, Event::Empty(_));
            match event {
                Event::Start(e) | Event::Empty(e) if depth == 0 => {
                    // The root: keep its namespace bindings.
                    for a in e.attributes().flatten() {
                        let key = String::from_utf8_lossy(a.key.as_ref());
                        let prefix = if key == "xmlns" { Some("") } else { key.strip_prefix("xmlns:") };
                        if let Some(p) = prefix {
                            ix.namespaces.push((p.to_string(), String::from_utf8_lossy(&a.value).into_owned()));
                        }
                    }
                    if !empty {
                        depth = 1;
                    }
                }
                Event::Start(e) | Event::Empty(e) if depth == 1 => {
                    let tag = &text[start..end];
                    let (id_kind, id_range, mrid) = match attr_value(tag, "rdf:ID")
                        .map(|s| (IdKind::Id, s))
                        .or_else(|| attr_value(tag, "rdf:about").map(|s| (IdKind::About, s)))
                    {
                        Some((kind, (vs, ve))) => (
                            Some(kind),
                            Some(lines.range(text, start + vs, start + ve)),
                            mrid_of(&tag[vs..ve]).to_string(),
                        ),
                        None => (None, None, String::new()),
                    };
                    let el = Element {
                        qname: String::from_utf8_lossy(e.name().as_ref()).into_owned(),
                        mrid,
                        id_kind,
                        head: range(),
                        id_range,
                        full: range(),
                        fields: Vec::new(),
                        name: None,
                    };
                    if !empty {
                        current = Some(el);
                        depth = 2;
                    } else {
                        ix.elements.push(el);
                    }
                }
                Event::Start(e) | Event::Empty(e) if depth == 2 => {
                    let tag = &text[start..end];
                    let resource = attr_value(tag, "rdf:resource").map(|(vs, ve)| {
                        (mrid_of(&tag[vs..ve]).to_string(), lines.range(text, start + vs, start + ve))
                    });
                    let f = Field {
                        qname: String::from_utf8_lossy(e.name().as_ref()).into_owned(),
                        head: range(),
                        full: range(),
                        resource,
                    };
                    if let Some(el) = current.as_mut() {
                        if !empty {
                            name_text = (f.key() == "IdentifiedObject.name").then(String::new);
                            field = Some(f);
                        } else {
                            el.fields.push(f);
                        }
                    }
                    if !empty {
                        depth = 3;
                    }
                }
                Event::Start(_) => depth += 1,
                Event::Text(t) if depth == 3 => {
                    if let Some(name) = name_text.as_mut() {
                        name.push_str(&t.unescape().unwrap_or_default());
                    }
                }
                Event::CData(t) if depth == 3 => {
                    if let Some(name) = name_text.as_mut() {
                        name.push_str(&String::from_utf8_lossy(&t));
                    }
                }
                Event::End(_) => {
                    depth = depth.saturating_sub(1);
                    match depth {
                        2 => {
                            if let (Some(el), Some(mut f)) = (current.as_mut(), field.take()) {
                                f.full.end = range().end;
                                if let Some(name) = name_text.take() {
                                    el.name = Some(name.trim().to_string());
                                }
                                // A bare `<` being typed parses as a nameless tag.
                                if !f.qname.is_empty() {
                                    el.fields.push(f);
                                }
                            }
                        }
                        1 => {
                            if let Some(mut el) = current.take() {
                                el.full.end = range().end;
                                ix.elements.push(el);
                            }
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        }
        // A document cut short — or with a tag left half-typed, which
        // swallows what follows — keeps its open element, to the end.
        if let Some(mut el) = current {
            el.full.end = lines.position(text, text.len());
            if let Some(f) = field.filter(|f| !f.qname.is_empty()) {
                el.fields.push(f);
            }
            ix.elements.push(el);
        }
        ix
    }

    /// The namespace IRI `prefix` is bound to at the root.
    pub fn namespace(&self, prefix: &str) -> Option<&str> {
        self.namespaces.iter().find(|(p, _)| p == prefix).map(|(_, ns)| ns.as_str())
    }

    /// The prefix bound to `ns` at the root.
    pub fn prefix_of(&self, ns: &str) -> Option<&str> {
        self.namespaces.iter().find(|(_, n)| n == ns).map(|(p, _)| p.as_str())
    }

    /// The element whose start-to-end span contains `pos`.
    pub fn element_at(&self, pos: Position) -> Option<&Element> {
        // Elements are in document order and do not overlap.
        let i = self.elements.partition_point(|e| e.full.end < pos);
        self.elements.get(i).filter(|e| contains(e.full, pos))
    }
}

pub fn contains(r: Range, pos: Position) -> bool {
    r.start <= pos && pos <= r.end
}

/// The byte span of attribute `name`'s value within a start tag, without its
/// quotes.
fn attr_value(tag: &str, name: &str) -> Option<(usize, usize)> {
    let bytes = tag.as_bytes();
    let mut from = 0;
    while let Some(i) = tag[from..].find(name).map(|i| i + from) {
        from = i + name.len();
        // A whole attribute name: preceded by whitespace, followed by `=`.
        if i == 0 || !bytes[i - 1].is_ascii_whitespace() {
            continue;
        }
        let rest = tag[from..].trim_start();
        let Some(rest) = rest.strip_prefix('=') else { continue };
        let rest = rest.trim_start();
        let quote = rest.chars().next()?;
        if quote != '"' && quote != '\'' {
            continue;
        }
        let vs = tag.len() - rest.len() + 1;
        let ve = vs + tag[vs..].find(quote)?;
        return Some((vs, ve));
    }
    None
}

/// Byte offsets ↔ LSP positions (UTF-16 columns).
pub struct LineIndex {
    starts: Vec<usize>,
}

impl LineIndex {
    pub fn new(text: &str) -> Self {
        let mut starts = vec![0];
        starts.extend(text.bytes().enumerate().filter(|(_, b)| *b == b'\n').map(|(i, _)| i + 1));
        Self { starts }
    }

    pub fn position(&self, text: &str, offset: usize) -> Position {
        let line = self.starts.partition_point(|s| *s <= offset) - 1;
        let start = self.starts[line];
        let character = text.get(start..offset).map_or(0, |s| s.encode_utf16().count());
        Position { line: line as u32, character: character as u32 }
    }

    pub fn range(&self, text: &str, start: usize, end: usize) -> Range {
        Range { start: self.position(text, start), end: self.position(text, end) }
    }

    pub fn offset(&self, text: &str, pos: Position) -> usize {
        let Some(&start) = self.starts.get(pos.line as usize) else { return text.len() };
        let end = self.starts.get(pos.line as usize + 1).copied().unwrap_or(text.len());
        let mut units = 0u32;
        for (i, c) in text[start..end].char_indices() {
            if units >= pos.character {
                return start + i;
            }
            units += c.len_utf16() as u32;
        }
        end
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const DOC: &str = r##"<?xml version="1.0" encoding="UTF-8"?>
<rdf:RDF xmlns:cim="http://iec.ch/TC57/CIM100#" xmlns:rdf="http://www.w3.org/1999/02/22-rdf-syntax-ns#">
  <cim:ACLineSegment rdf:ID="_line">
    <cim:IdentifiedObject.name>Line &amp; 1</cim:IdentifiedObject.name>
    <cim:ACLineSegment.r>0.5</cim:ACLineSegment.r>
    <cim:ConductingEquipment.BaseVoltage rdf:resource="#_bv"/>
  </cim:ACLineSegment>
  <cim:BaseVoltage
      rdf:about="urn:uuid:x#_bv" />
</rdf:RDF>
"##;

    fn pos(line: u32, character: u32) -> Position {
        Position { line, character }
    }

    #[test]
    fn indexes_elements_fields_and_references() {
        let ix = Index::build(DOC);
        assert_eq!(ix.namespace("cim"), Some("http://iec.ch/TC57/CIM100#"));
        assert_eq!(ix.elements.len(), 2);

        let line = &ix.elements[0];
        assert_eq!((line.qname.as_str(), line.mrid.as_str(), line.id_kind), ("cim:ACLineSegment", "_line", Some(IdKind::Id)));
        assert_eq!(line.id_range, Some(Range { start: pos(2, 29), end: pos(2, 34) }));
        assert_eq!(line.full.start, pos(2, 2));
        assert_eq!(line.full.end, pos(6, 22));
        assert_eq!(line.name.as_deref(), Some("Line & 1"));
        let keys: Vec<&str> = line.fields.iter().map(Field::key).collect();
        assert_eq!(keys, ["IdentifiedObject.name", "ACLineSegment.r", "ConductingEquipment.BaseVoltage"]);
        assert_eq!(line.fields[1].full, Range { start: pos(4, 4), end: pos(4, 50) });
        let (target, at) = line.fields[2].resource.clone().unwrap();
        assert_eq!(target, "_bv");
        assert_eq!(at, Range { start: pos(5, 55), end: pos(5, 59) });

        let bv = &ix.elements[1];
        assert_eq!((bv.mrid.as_str(), bv.id_kind), ("_bv", Some(IdKind::About)));
        assert_eq!(bv.head, Range { start: pos(7, 2), end: pos(8, 35) });

        assert_eq!(ix.element_at(pos(4, 10)).map(|e| e.mrid.as_str()), Some("_line"));
        assert!(ix.element_at(pos(1, 3)).is_none());
    }

    #[test]
    fn a_truncated_document_keeps_what_it_has() {
        let cut = &DOC[..DOC.find("<cim:ACLineSegment.r>").unwrap() + 25];
        let ix = Index::build(cut);
        assert_eq!(ix.elements.len(), 1);
        assert_eq!(ix.elements[0].mrid, "_line");
        assert_eq!(ix.elements[0].full.end, LineIndex::new(cut).position(cut, cut.len()));
    }

    #[test]
    fn utf16_positions_round_trip() {
        let text = "a\n€x𝄞y\n";
        let lines = LineIndex::new(text);
        let y = text.find('y').unwrap();
        assert_eq!(lines.position(text, y), pos(1, 4));
        assert_eq!(lines.offset(text, pos(1, 4)), y);
    }

    #[test]
    fn recognises_cim_documents() {
        assert!(is_cim(DOC));
        assert!(!is_cim("<project><modelVersion>4.0.0</modelVersion></project>"));
    }
}
