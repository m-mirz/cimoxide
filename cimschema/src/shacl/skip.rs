//! Records constraints the consumer could not act on.
//!
//! The simplification stage and `cimgen`'s codegen both drop constraints, and a
//! dropped constraint that leaves no trace is indistinguishable from one that
//! was checked. These types carry the reason; classifying and printing them is
//! the generator's job, in `cimgen::shacl::skip`.

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
