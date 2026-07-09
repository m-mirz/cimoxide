use std::collections::HashMap;
use std::fmt;

// ---------------------------------------------------------------------------
// Data types
// ---------------------------------------------------------------------------

pub struct SkipEntry {
    pub class_names: Vec<String>,
    pub prop: String,
    pub component: String,
    pub name: String,
    pub reason: String,
}

impl fmt::Display for SkipEntry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.class_names.len() == 1 {
            write!(f, "{}{} [{}] {:?}: {}",
                self.class_names[0], self.prop, self.component, self.name, self.reason)
        } else {
            write!(f, "{} [{}] {:?} ({}): {}",
                self.prop, self.component, self.name,
                self.class_names.join(", "), self.reason)
        }
    }
}

/// Accumulates skip entries for one file, deduplicating by (prop, component, name).
pub struct SkipCollector {
    pub entries: Vec<SkipEntry>,
    index: HashMap<String, usize>,
}

impl SkipCollector {
    pub fn new() -> Self {
        Self { entries: Vec::new(), index: HashMap::new() }
    }

    pub fn push(&mut self, class_name: &str, prop: &str, component: &str, name: &str, reason: &str) {
        let key = format!("{}\x00{}\x00{}", prop, component, name);
        if let Some(&idx) = self.index.get(&key) {
            let class = class_name.to_string();
            if !self.entries[idx].class_names.contains(&class) {
                self.entries[idx].class_names.push(class);
            }
        } else {
            self.index.insert(key, self.entries.len());
            self.entries.push(SkipEntry {
                class_names: vec![class_name.to_string()],
                prop: prop.to_string(),
                component: component.to_string(),
                name: name.to_string(),
                reason: reason.to_string(),
            });
        }
    }

    pub fn into_entries(self) -> Vec<SkipEntry> {
        self.entries
    }
}

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
    pub section: &'static str, // "simplified" | "skipped" | "cannot_be_conducted" | "sparql" | "other"
    pub match_fn: fn(&SkipEntry) -> bool,
}

static SKIP_CATEGORIES: &[SkipCategory] = &[
    // Simplified — dropped in simplify.rs before codegen; type-system guarantees
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
    // Skipped — codegen-level structural guarantees
    SkipCategory {
        label: "`sh:nodeKind` structurally satisfied",
        section: "skipped",
        match_fn: |e| e.reason.contains("sh:NodeKindConstraintComponent") && e.reason.contains("structurally satisfied"),
    },
    SkipCategory {
        label: "`sh:maxCount` on list field or unsupported value",
        section: "skipped",
        match_fn: |e| e.reason.contains("sh:MaxCount"),
    },
    SkipCategory {
        label: "`sh:orInversePath` structurally satisfied",
        section: "skipped",
        match_fn: |e| e.reason.contains("OrInversePath") && e.reason.contains("structurally satisfied"),
    },
    SkipCategory {
        label: "`sh:class` / `sh:or-class` vacuously true",
        section: "skipped",
        match_fn: |e| e.reason.contains("vacuously true"),
    },
    // Cannot be conducted
    SkipCategory {
        label: "multi-segment path not supported",
        section: "cannot_be_conducted",
        match_fn: |e| e.reason.contains("multi-segment"),
    },
    SkipCategory {
        label: "inverse path component not supported",
        section: "cannot_be_conducted",
        match_fn: |e| e.reason.contains("inverse") && (e.reason.contains("not supported") || e.reason.contains("no class.prop")),
    },
    SkipCategory {
        label: "attribute not found in hierarchy",
        section: "cannot_be_conducted",
        match_fn: |e| e.reason.contains("not found in hierarchy"),
    },
    SkipCategory {
        label: "unused association",
        section: "cannot_be_conducted",
        match_fn: |e| e.reason.contains("unused association"),
    },
    SkipCategory {
        label: "type mismatch (constraint on incompatible field type)",
        section: "cannot_be_conducted",
        match_fn: |e| {
            e.reason.contains("non-string") || e.reason.contains("non-numeric")
                || e.reason.contains("non-association") || e.reason.contains("non-list")
        },
    },
    SkipCategory {
        label: "empty list or missing payload",
        section: "cannot_be_conducted",
        match_fn: |e| e.reason.contains("empty") || e.reason.contains("missing payload"),
    },
    SkipCategory {
        label: "unsupported datatype format",
        section: "cannot_be_conducted",
        match_fn: |e| e.reason.contains("unsupported datatype"),
    },
    SkipCategory {
        label: "compound check branch structure not supported",
        section: "cannot_be_conducted",
        match_fn: |e| e.reason.contains("branch structure not supported") || e.reason.contains("compound check"),
    },
    SkipCategory {
        label: "[field, rdf:type] slice-mrid constraint not applicable",
        section: "cannot_be_conducted",
        match_fn: |e| e.reason.starts_with("slice-mrid"),
    },
    // SPARQL — sh:sparql constraints and sh:target SPARQLTarget targets both require
    // evaluating an arbitrary SPARQL query at runtime; there's no SPARQL evaluator in
    // this codebase, so both are resolved the same way as everything under
    // cimvalidation/src/sparql/: a hand-written Rust implementation, not a general
    // evaluator.
    SkipCategory {
        label: "SPARQL-derived constraint/target (needs a hand-written implementation, not a SPARQL evaluator)",
        section: "sparql",
        match_fn: |e| e.reason.contains("needs a hand-written implementation"),
    },
    // Unsupported SHACL target mechanism (shape recognized, but no concrete class to
    // generate checks against -- see ttl_import.rs's build_node_shape and
    // codegen.rs's push_unsupported_target_skips). Unlike SPARQLTarget above, this one
    // needs no SPARQL evaluator at all -- targetSubjectsOf/targetObjectsOf are plain
    // graph-predicate lookups, just not yet implemented in codegen.rs's per-class model.
    SkipCategory {
        label: "sh:targetSubjectsOf / sh:targetObjectsOf (property-based target, not implemented)",
        section: "unsupported_target",
        match_fn: |e| e.reason.contains("targetSubjectsOf/targetObjectsOf target"),
    },
    // Other
    SkipCategory {
        label: "unknown component",
        section: "other",
        match_fn: |e| e.reason.contains("unknown component"),
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
/// SkipCollector per render_file call) and *not* split on "|" for compound sh:name values --
/// so a repeated constraint pattern across profile-variant files, or a shape whose sh:name
/// bundles several conformance rules, is undercounted relative to ttl_report.rs's
/// sh:name-based, per-profile-group-deduped count.
pub fn print_global_summary(counts: &HashMap<&str, usize>) {
    let sections = [
        ("Simplified (type-system guarantees)", "simplified"),
        ("Skipped", "skipped"),
        ("Cannot be conducted", "cannot_be_conducted"),
        ("SPARQL (see SPARQL Check Coverage below -- not directly comparable, see print_global_summary's doc comment)", "sparql"),
        ("Unsupported SHACL target mechanism", "unsupported_target"),
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
