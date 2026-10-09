//! Hover, navigation, outline and completion over one document, with the
//! model set it belongs to for what other files define.
//!
//! Everything about classes and attributes comes from the class tables
//! through [`cimmodel::registry::type_registry`].

use cimmodel::base::{AttrDef, AttrKind, ClassDef};
use cimmodel::registry::type_registry;
use lsp_types::{
    CompletionItem, CompletionItemKind, CompletionTextEdit, DocumentSymbol, Hover, HoverContents,
    InsertTextFormat, Location, MarkupContent, MarkupKind, Position, Range, SymbolKind, TextEdit, Url,
};

use crate::index::{contains, local, prefix, Element, IdKind, Index, LineIndex};
use crate::validate::ModelSet;

/// An open document, indexed.
pub struct View<'a> {
    pub url: &'a Url,
    pub text: &'a str,
    pub lines: &'a LineIndex,
    pub index: &'a Index,
    /// The model set the document was last validated in, if it was.
    pub set: Option<&'a ModelSet>,
}

/// The class an element's tag names: by the namespace its prefix is bound
/// to, else by bare name — as the decoder resolves it.
pub fn class_of(index: &Index, qname: &str) -> Option<&'static ClassDef> {
    let reg = type_registry();
    index
        .namespace(prefix(qname))
        .and_then(|ns| reg.ns_table(ns))
        .and_then(|t| t.get(local(qname)).copied())
        .or_else(|| reg.bare().get(local(qname)).copied())
}

/// What the cursor is on.
enum Target<'a> {
    /// An element's start tag; `id` when on its `rdf:ID` / `rdf:about` value.
    Element { el: &'a Element, id: bool },
    Field { el: &'a Element, key: &'a str, head: Range },
    Resource { mrid: &'a str, range: Range },
}

fn target_at(index: &Index, pos: Position) -> Option<Target<'_>> {
    let el = index.element_at(pos)?;
    if el.id_range.is_some_and(|r| contains(r, pos)) {
        return Some(Target::Element { el, id: true });
    }
    if contains(el.head, pos) {
        return Some(Target::Element { el, id: false });
    }
    let f = el.fields.iter().find(|f| contains(f.full, pos))?;
    if let Some((mrid, range)) = f.resource.as_ref().filter(|(_, r)| contains(*r, pos)) {
        return Some(Target::Resource { mrid, range: *range });
    }
    contains(f.head, pos).then(|| Target::Field { el, key: f.key(), head: f.head })
}

pub fn hover(view: &View, pos: Position) -> Option<Hover> {
    let (text, range) = match target_at(view.index, pos)? {
        Target::Element { el, id } => {
            if id && el.id_kind == Some(IdKind::About) {
                (object_summary(view, &el.mrid)?, el.id_range)
            } else {
                (class_summary(class_of(view.index, &el.qname)?), Some(el.head))
            }
        }
        Target::Field { el, key, head } => {
            let class = class_of(view.index, &el.qname)?;
            (attr_summary(type_registry().attr_of(class, key)?, class), Some(head))
        }
        Target::Resource { mrid, range } => (object_summary(view, mrid)?, Some(range)),
    };
    Some(Hover {
        contents: HoverContents::Markup(MarkupContent { kind: MarkupKind::Markdown, value: text }),
        range,
    })
}

fn class_summary(class: &ClassDef) -> String {
    let reg = type_registry();
    let chain: Vec<&str> = reg.chain(class).iter().map(|c| c.local).collect();
    let mut s = format!("**{}**{}\n\n", class.local, if class.concrete { "" } else { " *(abstract)*" });
    s.push_str(&format!("`{}`\n\n", chain.join(" → ")));
    if !class.origins.is_empty() {
        s.push_str(&format!("Profiles: {}", class.origins.join(", ")));
    }
    s
}

fn attr_summary(a: &AttrDef, class: &ClassDef) -> String {
    let kind = match a.kind {
        AttrKind::Literal => "attribute",
        AttrKind::Association => "association",
        AttrKind::Enum => "enumeration",
    };
    let mut s = format!("**{}** — {kind} of type `{}`", a.id, a.range);
    if !a.xsd.is_empty() {
        s.push_str(&format!(" (`xsd:{}`)", a.xsd));
    }
    s.push_str(if a.is_list { ", multi-valued" } else { ", single-valued" });
    if type_registry().attr(class, a.id).is_none() {
        s.push_str(&format!("\n\n*Not declared for `{}`.*", class.local));
    }
    if !a.origins.is_empty() {
        s.push_str(&format!("\n\nProfiles: {}", a.origins.join(", ")));
    }
    s
}

/// An object by mRID: its class and name, and the file that defines it.
fn object_summary(view: &View, mrid: &str) -> Option<String> {
    let (el, index, file) = match definition_of(view, mrid) {
        Some(Def::Here(el)) => (el, view.index, None),
        Some(Def::Set(f, el)) => (el, &f.index, f.path.file_name().map(|n| n.to_string_lossy().into_owned())),
        None => return Some(format!("`{mrid}` — not defined in this model set")),
    };
    let class = class_of(index, &el.qname).map_or(local(&el.qname), |c| c.local);
    let mut s = format!("**{class}**");
    if let Some(name) = el.name.as_deref().filter(|n| !n.is_empty()) {
        s.push_str(&format!(" {name}"));
    }
    s.push_str(&format!("\n\n`{mrid}`"));
    if let Some(file) = file {
        s.push_str(&format!(" in {file}"));
    }
    Some(s)
}

enum Def<'a> {
    Here(&'a Element),
    Set(&'a crate::validate::SetFile, &'a Element),
}

/// Where `mrid` is defined: by `rdf:ID` in this document, else by the model
/// set, else by the first `rdf:about` in this document.
fn definition_of<'a>(view: &View<'a>, mrid: &str) -> Option<Def<'a>> {
    let here = || view.index.elements.iter().filter(|e| e.mrid == mrid);
    if let Some(el) = here().find(|e| e.id_kind == Some(IdKind::Id)) {
        return Some(Def::Here(el));
    }
    if let Some((f, el)) = view.set.and_then(|s| s.definition(mrid))
        && f.url != *view.url
    {
        return Some(Def::Set(f, el));
    }
    here().next().map(Def::Here)
}

fn mrid_at<'a>(view: &View<'a>, pos: Position) -> Option<&'a str> {
    match target_at(view.index, pos)? {
        Target::Element { el, .. } => Some(&el.mrid),
        Target::Resource { mrid, .. } => Some(mrid),
        Target::Field { .. } => None,
    }
}

pub fn definition(view: &View, pos: Position) -> Option<Location> {
    let mrid = mrid_at(view, pos)?;
    Some(match definition_of(view, mrid)? {
        Def::Here(el) => Location::new(view.url.clone(), el.id_range.unwrap_or(el.head)),
        Def::Set(f, el) => Location::new(f.url.clone(), el.id_range.unwrap_or(el.head)),
    })
}

/// Every `rdf:resource` naming the object under the cursor, in this document
/// and the rest of its model set; with `declaration`, also the elements that
/// write the object.
pub fn references(view: &View, pos: Position, declaration: bool) -> Vec<Location> {
    let Some(mrid) = mrid_at(view, pos) else { return Vec::new() };
    let mut out = Vec::new();
    let mut scan = |url: &Url, index: &Index| {
        for el in &index.elements {
            if declaration && el.mrid == mrid {
                out.push(Location::new(url.clone(), el.id_range.unwrap_or(el.head)));
            }
            for f in &el.fields {
                if let Some((m, r)) = &f.resource
                    && m == mrid
                {
                    out.push(Location::new(url.clone(), *r));
                }
            }
        }
    };
    scan(view.url, view.index);
    for f in view.set.into_iter().flat_map(|s| &s.files) {
        if f.url != *view.url {
            scan(&f.url, &f.index);
        }
    }
    out
}

#[allow(deprecated)] // `DocumentSymbol::deprecated` must still be written.
pub fn symbols(view: &View) -> Vec<DocumentSymbol> {
    view.index
        .elements
        .iter()
        .map(|el| {
            let class = local(&el.qname);
            let name = match el.name.as_deref().filter(|n| !n.is_empty()) {
                Some(n) => n.to_string(),
                None if !el.mrid.is_empty() => el.mrid.clone(),
                None => class.to_string(),
            };
            let children = el
                .fields
                .iter()
                .map(|f| DocumentSymbol {
                    name: f.key().to_string(),
                    detail: f.resource.as_ref().map(|(m, _)| m.clone()),
                    kind: if f.resource.is_some() { SymbolKind::FIELD } else { SymbolKind::PROPERTY },
                    tags: None,
                    deprecated: None,
                    range: f.full,
                    selection_range: f.head,
                    children: None,
                })
                .collect();
            DocumentSymbol {
                name,
                detail: Some(class.to_string()),
                kind: SymbolKind::OBJECT,
                tags: None,
                deprecated: None,
                range: el.full,
                selection_range: el.id_range.unwrap_or(el.head),
                children: Some(children),
            }
        })
        .collect()
}

/// Inside an object: the fields its class declares (inherited ones
/// included, association ends no profile exchanges left out). Between
/// objects: the concrete classes of the document's family.
pub fn completion(view: &View, pos: Position) -> Vec<CompletionItem> {
    let offset = view.lines.offset(view.text, pos);
    let line_start = view.text[..offset].rfind('\n').map_or(0, |i| i + 1);
    let typed = &view.text[line_start..offset];
    // Replace from an opening `<` the user has typed, if any.
    let lt = typed.rfind('<').filter(|i| !typed[*i..].contains(['>', ' ', '"']));
    let replace = Range::new(lt.map_or(pos, |i| view.lines.position(view.text, line_start + i)), pos);
    let reg = type_registry();

    if let Some(el) = view.index.element_at(pos) {
        if contains(el.head, pos) || el.fields.iter().any(|f| contains(f.full, pos) && f.full.end != pos) {
            return Vec::new();
        }
        let Some(class) = class_of(view.index, &el.qname) else { return Vec::new() };
        let mut attrs: Vec<&AttrDef> = reg.attrs(class).filter(|a| a.used).collect();
        attrs.sort_by_key(|a| a.id);
        return attrs
            .into_iter()
            .map(|a| {
                let tag = format!("{}:{}", view.index.prefix_of(a.ns).unwrap_or("cim"), a.id);
                let snippet = match a.kind {
                    AttrKind::Literal => format!("<{tag}>$1</{tag}>"),
                    AttrKind::Association => format!("<{tag} rdf:resource=\"#$1\"/>"),
                    AttrKind::Enum => format!("<{tag} rdf:resource=\"{}$1\"/>", a.value_ns),
                };
                item(tag, a.range.to_string(), CompletionItemKind::FIELD, snippet, replace)
            })
            .collect();
    }

    // Between objects: classes of the namespace the document's `cim` binds.
    let Some(table) = view.index.namespace("cim").and_then(|ns| reg.ns_table(ns)) else { return Vec::new() };
    let mut classes: Vec<&ClassDef> = table.values().copied().filter(|c| c.concrete).collect();
    classes.sort_by_key(|c| c.local);
    classes
        .into_iter()
        .map(|c| {
            let tag = format!("cim:{}", c.local);
            let snippet = format!("<{tag} rdf:ID=\"$1\">\n\t$0\n</{tag}>");
            item(tag, c.origins.join(", "), CompletionItemKind::CLASS, snippet, replace)
        })
        .collect()
}

fn item(label: String, detail: String, kind: CompletionItemKind, snippet: String, replace: Range) -> CompletionItem {
    CompletionItem {
        filter_text: Some(format!("<{label}")),
        label,
        detail: Some(detail),
        kind: Some(kind),
        insert_text_format: Some(InsertTextFormat::SNIPPET),
        text_edit: Some(CompletionTextEdit::Edit(TextEdit::new(replace, snippet))),
        ..Default::default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const DOC: &str = r##"<?xml version="1.0" encoding="UTF-8"?>
<rdf:RDF xmlns:cim="http://iec.ch/TC57/CIM100#" xmlns:rdf="http://www.w3.org/1999/02/22-rdf-syntax-ns#">
  <cim:BaseVoltage rdf:ID="_bv">
    <cim:IdentifiedObject.name>380 kV</cim:IdentifiedObject.name>
  </cim:BaseVoltage>
  <cim:ACLineSegment rdf:ID="_line">
    <cim:ACLineSegment.r>0.5</cim:ACLineSegment.r>
    <cim:ConductingEquipment.BaseVoltage rdf:resource="#_bv"/>
    <
  </cim:ACLineSegment>
</rdf:RDF>
"##;

    fn with_view<R>(f: impl FnOnce(&View) -> R) -> R {
        let url = Url::parse("file:///tmp/eq.xml").unwrap();
        let lines = LineIndex::new(DOC);
        let index = Index::build(DOC);
        f(&View { url: &url, text: DOC, lines: &lines, index: &index, set: None })
    }

    fn hover_text(h: Option<Hover>) -> String {
        match h.expect("a hover").contents {
            HoverContents::Markup(m) => m.value,
            _ => unreachable!(),
        }
    }

    #[test]
    fn hovers_classes_attributes_and_references() {
        with_view(|v| {
            let class = hover_text(hover(v, Position::new(5, 8)));
            assert!(class.starts_with("**ACLineSegment**"), "{class}");
            assert!(class.contains("IdentifiedObject → "), "{class}");

            let attr = hover_text(hover(v, Position::new(6, 10)));
            assert!(attr.starts_with("**ACLineSegment.r** — attribute"), "{attr}");

            let target = hover_text(hover(v, Position::new(7, 58)));
            assert!(target.starts_with("**BaseVoltage** 380 kV"), "{target}");
        });
    }

    #[test]
    fn navigates_references() {
        with_view(|v| {
            let def = definition(v, Position::new(7, 58)).expect("a definition");
            assert_eq!(def.range.start, Position::new(2, 27));
            let refs = references(v, Position::new(2, 28), true);
            assert_eq!(refs.len(), 2, "{refs:?}");
        });
    }

    #[test]
    fn completes_fields_of_the_class() {
        with_view(|v| {
            let items = completion(v, Position::new(8, 5));
            let labels: Vec<&str> = items.iter().map(|i| i.label.as_str()).collect();
            assert!(labels.contains(&"cim:ACLineSegment.r"), "{labels:?}");
            assert!(labels.contains(&"cim:IdentifiedObject.name"), "inherited");
            let Some(CompletionTextEdit::Edit(edit)) = &items[0].text_edit else { panic!() };
            assert_eq!(edit.range.start, Position::new(8, 4), "replaces the typed `<`");
        });
    }

    #[test]
    fn outlines_objects() {
        with_view(|v| {
            let s = symbols(v);
            assert_eq!(s.len(), 2);
            assert_eq!(s[0].name, "380 kV");
            assert_eq!(s[1].name, "_line");
            assert_eq!(s[1].children.as_ref().unwrap().len(), 2);
        });
    }
}
