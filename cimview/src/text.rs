//! Places in a document's text and edits to it, as an editor counts them: lines from 0, and
//! columns in UTF-16 code units, as LSP and JavaScript strings do. They serialize as LSP's do
//! (`{ "line", "character" }`, `{ "start", "end" }`, `{ "range", "newText" }`).

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Position {
    pub line: u32,
    pub character: u32,
}

impl Position {
    pub fn new(line: u32, character: u32) -> Self {
        Self { line, character }
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Range {
    pub start: Position,
    pub end: Position,
}

impl Range {
    pub fn new(start: Position, end: Position) -> Self {
        Self { start, end }
    }
}

/// Whether `pos` is in `r`, its ends included.
pub fn contains(r: Range, pos: Position) -> bool {
    r.start <= pos && pos <= r.end
}

/// `range` of a document replaced by `new_text`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TextEdit {
    pub range: Range,
    pub new_text: String,
}

impl TextEdit {
    pub fn new(range: Range, new_text: String) -> Self {
        Self { range, new_text }
    }
}

/// Byte offsets ↔ positions.
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

/// `text` with `edits` applied, each placed in `text` as it was: they must not overlap.
pub fn apply(text: &str, edits: &[TextEdit]) -> String {
    let lines = LineIndex::new(text);
    let mut spans: Vec<(usize, usize, &str)> = edits
        .iter()
        .map(|e| (lines.offset(text, e.range.start), lines.offset(text, e.range.end), e.new_text.as_str()))
        .collect();
    // From the end, so each offset still holds; at one place, in the order given.
    spans.sort_by_key(|s| std::cmp::Reverse(s.0));
    let mut out = text.to_string();
    let mut i = 0;
    while i < spans.len() {
        let at = spans[i].0;
        let mut j = i;
        while j < spans.len() && spans[j].0 == at {
            j += 1;
        }
        // Inserts at one place keep their order; a replacement among them takes its span.
        let end = spans[i..j].iter().map(|s| s.1).max().unwrap_or(at);
        let joined: String = spans[i..j].iter().map(|s| s.2).collect();
        out.replace_range(at..end, &joined);
        i = j;
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn utf16_positions_round_trip() {
        let text = "a\n€x𝄞y\n";
        let lines = LineIndex::new(text);
        let y = text.find('y').unwrap();
        assert_eq!(lines.position(text, y), Position::new(1, 4));
        assert_eq!(lines.offset(text, Position::new(1, 4)), y);
    }

    #[test]
    fn edits_apply_from_the_end_and_inserts_at_one_place_keep_their_order() {
        let text = "ab\ncd\n";
        let at = |l, c| Position::new(l, c);
        let edits = [
            TextEdit::new(Range::new(at(0, 1), at(0, 2)), "B".into()),
            TextEdit::new(Range::new(at(1, 0), at(1, 0)), "1".into()),
            TextEdit::new(Range::new(at(1, 0), at(1, 0)), "2".into()),
        ];
        assert_eq!(apply(text, &edits), "aB\n12cd\n");
    }

    #[test]
    fn positions_serialize_as_lsp_does() {
        let edit = TextEdit::new(Range::new(Position::new(1, 2), Position::new(3, 4)), "x".into());
        assert_eq!(
            serde_json::to_value(&edit).unwrap(),
            serde_json::json!({ "range": { "start": { "line": 1, "character": 2 }, "end": { "line": 3, "character": 4 } }, "newText": "x" })
        );
    }
}
