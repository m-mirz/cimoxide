use std::collections::hash_map::Entry;
use std::io::BufReader;
use std::path::Path;

use ahash::AHashMap;
use quick_xml::events::Event;
use quick_xml::Reader;

use cimstructs::base::{CimElement, FieldValue, RdfBlock};
use cimstructs::registry::{self, ParseFn};

pub struct CimEntry {
    pub element: Box<dyn CimElement>,
    pub block: RdfBlock,
}

pub struct CimDataset {
    pub entries: AHashMap<String, CimEntry>,
    /// Maps `type_name()` → list of MRIDs of that type. Populated on insert, maintained on merge.
    pub by_type: AHashMap<String, Vec<String>>,
}

impl Default for CimDataset {
    fn default() -> Self {
        Self::new()
    }
}

impl CimDataset {
    pub fn new() -> Self {
        Self {
            entries: AHashMap::new(),
            by_type: AHashMap::new(),
        }
    }

    pub fn decode_str(content: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let mut reader = Reader::from_str(content);
        let raw = parse_to_raw(&mut reader)?;
        Ok(instantiate(raw, registry::registry()))
    }

    pub fn decode_file(path: &Path) -> Result<Self, Box<dyn std::error::Error>> {
        let mut reader = Reader::from_reader(BufReader::new(std::fs::File::open(path)?));
        let raw = parse_to_raw(&mut reader)?;
        Ok(instantiate(raw, registry::registry()))
    }

    pub fn decode_files(paths: &[&Path]) -> Result<Self, Box<dyn std::error::Error>> {
        let mut merged: AHashMap<String, RdfBlock> = AHashMap::new();
        for path in paths {
            let mut reader = Reader::from_reader(BufReader::new(std::fs::File::open(path)?));
            merge_raw(&mut merged, parse_to_raw(&mut reader)?);
        }
        Ok(instantiate(merged, registry::registry()))
    }

    /// Decode files in parallel using one thread per file, merge raw blocks, then instantiate once.
    /// Falls back to `decode_files` for 0–1 paths to avoid thread-spawn overhead.
    pub fn decode_files_parallel(paths: &[&Path]) -> Result<Self, Box<dyn std::error::Error>> {
        if paths.len() <= 1 {
            return Self::decode_files(paths);
        }
        let (ds, _) = Self::decode_files_parallel_with_counts(paths)?;
        Ok(ds)
    }

    /// Like `decode_files_parallel` but also returns the per-file element count (before
    /// deduplication) in the same order as `paths`.
    pub fn decode_files_parallel_with_counts(
        paths: &[&Path],
    ) -> Result<(Self, Vec<usize>), Box<dyn std::error::Error>> {
        let results: Vec<Result<AHashMap<String, RdfBlock>, String>> = std::thread::scope(|s| {
            paths
                .iter()
                .map(|p| s.spawn(|| parse_file_raw(p).map_err(|e| e.to_string())))
                .collect::<Vec<_>>()
                .into_iter()
                .map(|h| h.join().expect("decode thread panicked"))
                .collect()
        });
        let raws: Vec<AHashMap<String, RdfBlock>> = results
            .into_iter()
            .collect::<Result<_, String>>()
            .map_err(|e| -> Box<dyn std::error::Error> { e.into() })?;
        let counts: Vec<usize> = raws.iter().map(|r| r.len()).collect();
        let mut merged: AHashMap<String, RdfBlock> = AHashMap::new();
        for raw in raws {
            merge_raw(&mut merged, raw);
        }
        Ok((instantiate(merged, registry::registry()), counts))
    }

    /// Merge another dataset into self, combining objects with the same MRID.
    /// For conflicting MRIDs: merge RdfBlocks (later scalar wins, lists union),
    /// then re-instantiate the typed element.
    pub fn merge(&mut self, other: CimDataset) {
        let reg = registry::registry();
        for (mrid, incoming) in other.entries {
            if let Some(existing) = self.entries.get_mut(&mrid) {
                existing.block.merge_from(&incoming.block);
                let type_name = existing.block.type_name.clone();
                if let Some(f) = reg.get(type_name.as_str()) {
                    existing.element = f(&existing.block);
                }
            } else {
                let type_name = incoming.element.type_name().to_string();
                self.by_type.entry(type_name).or_default().push(mrid.clone());
                self.entries.insert(mrid, incoming);
            }
        }
    }

    /// Release all RdfBlocks to free memory after the final merge.
    pub fn drop_blocks(&mut self) {
        for entry in self.entries.values_mut() {
            entry.block = RdfBlock::default();
        }
    }
}

// --- raw-block pipeline ------------------------------------------------------

fn parse_file_raw(path: &Path) -> Result<AHashMap<String, RdfBlock>, Box<dyn std::error::Error>> {
    let mut reader = Reader::from_reader(BufReader::new(std::fs::File::open(path)?));
    parse_to_raw(&mut reader)
}

fn merge_raw(base: &mut AHashMap<String, RdfBlock>, other: AHashMap<String, RdfBlock>) {
    for (mrid, incoming) in other {
        match base.entry(mrid) {
            Entry::Occupied(mut e) => e.get_mut().merge_from(&incoming),
            Entry::Vacant(e) => { e.insert(incoming); }
        }
    }
}

fn instantiate(raw: AHashMap<String, RdfBlock>, reg: &AHashMap<&'static str, ParseFn>) -> CimDataset {
    let mut ds = CimDataset {
        entries: AHashMap::with_capacity(raw.len()),
        by_type: AHashMap::new(),
    };
    for (mrid, block) in raw {
        if let Some(f) = reg.get(block.type_name.as_str()) {
            let element = f(&block);
            let type_name = element.type_name().to_string();
            ds.by_type.entry(type_name).or_default().push(mrid.clone());
            ds.entries.insert(mrid, CimEntry { element, block });
        }
    }
    ds
}

// --- XML streaming parser (raw blocks only) ----------------------------------

fn parse_to_raw<R: std::io::BufRead>(
    reader: &mut Reader<R>,
) -> Result<AHashMap<String, RdfBlock>, Box<dyn std::error::Error>> {
    let mut raw: AHashMap<String, RdfBlock> = AHashMap::new();
    let mut buf = Vec::new();

    let mut depth: u32 = 0;
    let mut current: Option<RdfBlock> = None;
    let mut pending_key: Option<String> = None;

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(ref e)) => {
                depth += 1;
                let local = local_name(e.name().as_ref())?;

                match depth {
                    2 => {
                        let mrid = extract_about(e.attributes())?;
                        current = Some(RdfBlock {
                            type_name: local,
                            mrid,
                            fields: AHashMap::new(),
                        });
                    }
                    3 => {
                        if let Some(ref mut block) = current {
                            if let Some(res) = find_resource(e.attributes())? {
                                add_field(block, &local, FieldValue::Resource(res));
                            } else {
                                pending_key = Some(local);
                            }
                        }
                    }
                    _ => {}
                }
            }

            Ok(Event::Empty(ref e)) => {
                let local = local_name(e.name().as_ref())?;
                match depth {
                    1 => {
                        let mrid = extract_about(e.attributes())?;
                        if !mrid.is_empty() {
                            raw.insert(mrid.clone(), RdfBlock {
                                type_name: local,
                                mrid,
                                fields: AHashMap::new(),
                            });
                        }
                    }
                    2 => {
                        if let Some(ref mut block) = current {
                            if let Some(res) = find_resource(e.attributes())? {
                                add_field(block, &local, FieldValue::Resource(res));
                            }
                        }
                    }
                    _ => {}
                }
            }

            Ok(Event::Text(ref e)) => {
                if depth == 3 {
                    if let (Some(block), Some(key)) = (&mut current, pending_key.take()) {
                        let text = e.unescape()?.trim().to_string();
                        if !text.is_empty() {
                            add_field(block, &key, FieldValue::Text(text));
                        }
                    }
                }
            }

            Ok(Event::End(_)) => {
                if depth == 2 {
                    pending_key = None;
                    if let Some(block) = current.take() {
                        if !block.mrid.is_empty() {
                            raw.insert(block.mrid.clone(), block);
                        }
                    }
                }
                depth = depth.saturating_sub(1);
            }

            Ok(Event::Eof) => break,
            Err(e) => return Err(Box::new(e)),
            _ => {}
        }
        buf.clear();
    }

    Ok(raw)
}

// --- helpers ----------------------------------------------------------------

fn local_name(raw: &[u8]) -> Result<String, Box<dyn std::error::Error>> {
    let s = std::str::from_utf8(raw)?;
    Ok(s.find(':').map(|i| &s[i + 1..]).unwrap_or(s).to_string())
}

fn strip_fragment(s: &str) -> String {
    if let Some(i) = s.rfind('#') {
        s[i + 1..].to_string()
    } else {
        s.trim_start_matches('#').to_string()
    }
}

fn extract_about(
    attrs: quick_xml::events::attributes::Attributes<'_>,
) -> Result<String, Box<dyn std::error::Error>> {
    for attr in attrs.flatten() {
        let key = std::str::from_utf8(attr.key.as_ref())?;
        if key == "rdf:about" || key == "rdf:ID" {
            return Ok(strip_fragment(std::str::from_utf8(&attr.value)?));
        }
    }
    Ok(String::new())
}

fn find_resource(
    attrs: quick_xml::events::attributes::Attributes<'_>,
) -> Result<Option<String>, Box<dyn std::error::Error>> {
    for attr in attrs.flatten() {
        let key = std::str::from_utf8(attr.key.as_ref())?;
        if key == "rdf:resource" {
            return Ok(Some(strip_fragment(std::str::from_utf8(&attr.value)?)));
        }
    }
    Ok(None)
}

/// Insert a field value, upgrading Resource → ResourceList on repeated keys.
fn add_field(block: &mut RdfBlock, key: &str, val: FieldValue) {
    if let FieldValue::Resource(ref new_ref) = val {
        match block.fields.get_mut(key) {
            Some(FieldValue::ResourceList(list)) => {
                list.push(new_ref.clone());
                return;
            }
            Some(existing @ FieldValue::Resource(_)) => {
                let old = match existing {
                    FieldValue::Resource(s) => s.clone(),
                    _ => unreachable!(),
                };
                *existing = FieldValue::ResourceList(vec![old, new_ref.clone()]);
                return;
            }
            _ => {}
        }
    }
    block.fields.insert(key.to_string(), val);
}
