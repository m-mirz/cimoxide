//! Where things are written in a document: cimoxide-view's index, in its own
//! positions, and their conversions to and from LSP's, which count alike
//! (lines from 0, UTF-16 columns).

pub use cimmodel::decode::is_cim;
pub use cimview::index::{Element, Field, IdKind, Index, local};
pub use cimview::text::{LineIndex, contains};

/// A range as LSP writes it.
pub fn lsp(r: cimview::text::Range) -> lsp_types::Range {
    lsp_types::Range::new(lsp_pos(r.start), lsp_pos(r.end))
}

pub fn lsp_pos(p: cimview::text::Position) -> lsp_types::Position {
    lsp_types::Position::new(p.line, p.character)
}

/// An LSP position, as the index counts it.
pub fn ours(p: lsp_types::Position) -> cimview::text::Position {
    cimview::text::Position::new(p.line, p.character)
}

/// An edit as LSP writes it.
pub fn lsp_edit(e: cimview::text::TextEdit) -> lsp_types::TextEdit {
    lsp_types::TextEdit::new(lsp(e.range), e.new_text)
}
