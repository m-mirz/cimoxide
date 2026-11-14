//! A model set as views, for whatever draws them: its grid at substation
//! level ([`grid`], laid out by [`layout`]) and its objects as a tree
//! ([`tree`]), with validation's findings counted on each. `cimlsp` serves
//! them to the VS Code extension. Runs on wasm32-unknown-unknown too.
//!
//! The files themselves, as text: where each object and field is written
//! ([`index`], in [`text`]'s positions), and edits to them across a set
//! ([`edit`]): renaming, deleting, adding objects and fields.
//!
//! The views answer in JSON. Where an object is written is not in the data:
//! the caller says, through [`Locations`].

use serde_json::Value;

pub mod edit;
pub mod grid;
pub mod index;
pub mod layout;
pub mod single_line;
pub mod text;
pub mod tree;

/// Where a set's objects are written, which only the caller of a view knows:
/// a language server has ranges in documents, a page only file names.
pub trait Locations {
    /// Where the object `id` is written, in whatever form the view's client
    /// reads, `null` if the set does not write it.
    fn location(&self, id: &str) -> Value;
    /// The set's files, in that same form.
    fn files(&self) -> Vec<Value>;
}

/// The mRID in a reference or an id as written: the text after the last `#`.
pub fn mrid_of(value: &str) -> &str {
    value.rsplit_once('#').map_or(value, |(_, f)| f)
}

/// A model set as the tests need it: files read, validated and located.
#[cfg(test)]
pub(crate) mod testing {
    use std::collections::HashMap;
    use std::path::{Path, PathBuf};
    use std::sync::Arc;

    use cimmodel::CimDataset;
    use serde_json::{Value, json};

    use crate::Locations;
    use crate::grid::{self, Grid, Model};
    use crate::tree::{self, Tree};

    /// The CIM files of `dir`, in name order.
    pub fn collect(dir: &Path) -> Vec<(PathBuf, Arc<str>)> {
        let mut paths: Vec<PathBuf> = std::fs::read_dir(dir)
            .map(|d| d.flatten().map(|e| e.path()).collect())
            .unwrap_or_default();
        paths.sort();
        paths
            .into_iter()
            .filter_map(|p| {
                let text = std::fs::read_to_string(&p).ok()?;
                cimmodel::decode::is_cim(&text).then(|| (p, Arc::from(text)))
            })
            .collect()
    }

    pub struct Set {
        pub grid: Grid,
        pub tree: Tree,
        files: Vec<String>,
        /// mRID → the file of its `rdf:ID`, else of its first `rdf:about`.
        defs: HashMap<String, (usize, bool)>,
    }

    impl Locations for Set {
        fn location(&self, id: &str) -> Value {
            match self.defs.get(id) {
                Some(&(f, _)) => json!({ "uri": self.files[f] }),
                None => Value::Null,
            }
        }
        fn files(&self) -> Vec<Value> {
            self.files.iter().map(|f| json!(f)).collect()
        }
    }

    /// `files` as one model set, under the configuration `cimcli validate
    /// --common --quality` uses.
    pub fn validate(files: &[(PathBuf, Arc<str>)]) -> Set {
        let datasets: Vec<CimDataset> = files.iter().map(|(_, t)| CimDataset::decode_str(t).unwrap()).collect();
        let mut defs: HashMap<String, (usize, bool)> = HashMap::new();
        for (f, ds) in datasets.iter().enumerate() {
            for (id, el) in &ds.entries {
                let slot = defs.entry(id.to_string()).or_insert((f, el.defined()));
                if el.defined() && !slot.1 {
                    *slot = (f, true);
                }
            }
        }
        let cfg = cimvalidation::combined_config(&datasets, None, None, true, true, Vec::new());
        let model = Model::new(&datasets);
        let (mut grid, mut tree) = (grid::build(&model), tree::build(&model));
        let found = cimvalidation::validate_files_traced(datasets, &cfg);
        grid.add_findings(&found);
        tree.add_findings(&found);
        let files = files.iter().map(|(p, _)| p.to_string_lossy().into_owned()).collect();
        Set { grid, tree, files, defs }
    }
}
