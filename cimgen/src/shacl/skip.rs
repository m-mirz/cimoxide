//! Classifies and prints the constraints a consumer could not act on.
//!
//! The [`SkipEntry`] / [`SkipCollector`] types live in `cimschema`, since the
//! simplification stage there produces them too. Everything below — the
//! categories, the counts, the report formatting — is generator-side only.

pub use cimschema::shacl::skip::{SkipCollector, SkipEntry};

use std::collections::HashMap;

pub struct FileSkipInfo {
    pub file_name: String,
    pub check_count: usize,
    pub skips: Vec<SkipEntry>,
}

// ---------------------------------------------------------------------------
// Skip categories
// ---------------------------------------------------------------------------

pub struct SkipCategory {
    pub label: &'static str,
    pub section: &'static str, // "simplified" | "sparql" | "upstream" | "unsupported" | "other"
    pub match_fn: fn(&SkipEntry) -> bool,
}

static SKIP_CATEGORIES: &[SkipCategory] = &[
    // Simplified — dropped in `cimschema::shacl::simplify` before resolution,
    // because the typed decoding already guarantees them.
    SkipCategory {
        label: "`sh:nodeKind` simplified (type-system guarantee)",
        section: "simplified",
        match_fn: |e| e.reason.starts_with("NodeKind") && e.reason.contains("structurally satisfied"),
    },
    SkipCategory {
        label: "`sh:datatype` simplified (native Rust type)",
        section: "simplified",
        match_fn: |e| e.reason.contains("Datatype structurally satisfied"),
    },
    SkipCategory {
        label: "`sh:minCount=0` vacuously true",
        section: "simplified",
        match_fn: |e| e.reason.contains("MinCount=0 vacuously true"),
    },
    // SPARQL: `sh:sparql` constraints and `sh:SPARQLTarget` targets, implemented
    // by hand in cimvalidation/src/sparql/.
    SkipCategory {
        label: "SPARQL-derived constraints (hand-written in cimvalidation/src/sparql)",
        section: "sparql",
        match_fn: |e| e.component == "sh:SPARQLConstraintComponent" || e.component == "sparqlTarget",
    },
    // Upstream defects: names the schema does not define.
    SkipCategory {
        label: "target class not defined by the schema (upstream defect)",
        section: "upstream",
        match_fn: |e| e.reason.contains("target class is not in this family's schema"),
    },
    SkipCategory {
        label: "value list or class that does not resolve (upstream defect)",
        section: "upstream",
        match_fn: |e| {
            matches!(e.component.as_str(),
                "sh:InConstraintComponent" | "sh:HasValueConstraintComponent" | "sh:ClassConstraintComponent")
                && e.reason.contains("not supported")
        },
    },
    // Constraints the table cannot express.
    SkipCategory {
        label: "`sh:nodeKind` on a compound-datatype or `rdf:type` path",
        section: "unsupported",
        match_fn: |e| e.component == "sh:NodeKindConstraintComponent" && e.reason.contains("not supported"),
    },
    SkipCategory {
        label: "`sh:length` (checked by hand in sparql/common.rs)",
        section: "unsupported",
        match_fn: |e| e.component == "sh:LengthConstraintComponent",
    },
    SkipCategory {
        label: "path or logical-combination form not supported",
        section: "unsupported",
        match_fn: |e| e.component == "sh:path" || e.reason.contains("logical"),
    },
];

static SKIP_CATEGORY_OTHER: SkipCategory = SkipCategory {
    label: "other (unclassified)",
    section: "other",
    match_fn: |_| true,
};

pub fn classify(e: &SkipEntry) -> &'static SkipCategory {
    for cat in SKIP_CATEGORIES {
        if (cat.match_fn)(e) { return cat; }
    }
    &SKIP_CATEGORY_OTHER
}

// ---------------------------------------------------------------------------
// Reporting functions
// ---------------------------------------------------------------------------

pub fn accumulate_counts<'a>(counts: &mut HashMap<&'a str, usize>, entries: &[SkipEntry]) {
    for e in entries {
        *counts.entry(classify(e).label).or_insert(0) += 1;
    }
}

pub fn print_file_summary(file_name: &str, checks: usize, entries: &[SkipEntry]) {
    eprintln!("-- {} ({} checks, {} skipped) --", file_name, checks, entries.len());
    if entries.is_empty() { return; }
    let mut counts: HashMap<&str, usize> = HashMap::new();
    accumulate_counts(&mut counts, entries);
    let all_cats = SKIP_CATEGORIES.iter().chain(std::iter::once(&SKIP_CATEGORY_OTHER));
    for cat in all_cats {
        let n = counts.get(cat.label).copied().unwrap_or(0);
        if n > 0 {
            eprintln!("  {:5}  {}", n, cat.label);
        }
    }
}

/// The "sparql" section's total isn't the same number as the SPARQL Check Coverage table's
/// TTL Total, even though both are "how much SPARQL is there" counts: this one is every
/// distinct (property, component, sh:name) skip entry, deduped per TTL file (a fresh
/// SkipCollector per file) and *not* split on "|" for compound sh:name values --
/// so a repeated constraint pattern across profile-variant files, or a shape whose sh:name
/// bundles several conformance rules, is undercounted relative to ttl_report.rs's
/// sh:name-based, per-profile-group-deduped count.
pub fn print_global_summary(counts: &HashMap<&str, usize>) {
    let sections = [
        ("Simplified (type-system guarantees)", "simplified"),
        ("SPARQL (see SPARQL Check Coverage below -- not directly comparable, see print_global_summary's doc comment)", "sparql"),
        ("Upstream schema defects", "upstream"),
        ("Not expressible in the shape table", "unsupported"),
        ("Other", "other"),
    ];
    for (title, key) in &sections {
        let all_cats = SKIP_CATEGORIES.iter().chain(std::iter::once(&SKIP_CATEGORY_OTHER));
        let mut total = 0usize;
        let mut lines: Vec<String> = Vec::new();
        for cat in all_cats {
            if cat.section != *key { continue; }
            let n = counts.get(cat.label).copied().unwrap_or(0);
            if n > 0 {
                lines.push(format!("  {:5}  {}", n, cat.label));
                total += n;
            }
        }
        if total == 0 { continue; }
        eprintln!("\n=== {} ===", title);
        for l in &lines { eprintln!("{}", l); }
        eprintln!("  -----\n  {:5}  total", total);
    }
}
