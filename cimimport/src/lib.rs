//! Importers from other power-grid formats into CGMES 3.0.
//!
//! A conversion takes a file's text and gives the model's files as text, EQ,
//! SSH, TP and SV, with what it had to decide on the case's behalf as
//! [`Converted::notes`]. Nothing here touches the filesystem, so the same code
//! runs in the CLI, the language server and in WebAssembly.
//!
//! The objects' mRIDs are v5 UUIDs of the case's name and each object's place
//! in it (`bus:14`, `branch:4-7:2`): converting a case again gives the same
//! mRIDs, so diagrams, difference models and edits made on one conversion
//! still apply to the next.
//!
//! ```
//! let text = "function mpc = two
//! mpc.version = '2';
//! mpc.baseMVA = 100;
//! mpc.bus = [1 3 0 0 0 0 1 1 0 110 1 1.1 0.9; 2 1 50 10 0 0 1 0.98 -2 110 1 1.1 0.9];
//! mpc.gen = [1 50 10 99 -99 1 100 1 100 0];
//! mpc.branch = [1 2 0.01 0.1 0.02 100 0 0 0 0 1 -360 360];
//! ";
//! let model = cimimport::matpower("two.m", text, &Default::default()).unwrap();
//! let names: Vec<&str> = model.files.iter().map(|(name, _)| name.as_str()).collect();
//! assert_eq!(names, ["two_EQ.xml", "two_SSH.xml", "two_TP.xml", "two_SV.xml"]);
//! ```

pub mod build;
pub mod matpower;

pub use matpower::MatpowerError;

/// A converted model.
#[derive(Clone, Debug)]
pub struct Converted {
    /// `(file name, RDF/XML)`, one per profile: EQ, SSH, TP, SV.
    pub files: Vec<(String, String)>,
    /// What the conversion invented, assumed or left out, one sentence each.
    pub notes: Vec<String>,
}

/// How a conversion builds the model.
#[derive(Clone, Debug)]
pub struct Options {
    /// A breaker at each end of each branch, so that a branch can be switched
    /// and an out-of-service one shows as open. On by default.
    pub breakers: bool,
    /// Buses whose names differ only in a trailing section number, and which a
    /// branch joins, in one substation (`MASON CITY 0` to `MASON CITY 4`). On
    /// by default; a case without bus names is grouped by transformer only.
    pub name_grouping: bool,
    /// The headers' `Model.created` and `Model.scenarioTime`, as
    /// `2026-11-27T12:00:00Z`. Left out, a fixed placeholder, so that
    /// converting a case twice gives the same files.
    pub time: Option<String>,
}

impl Default for Options {
    fn default() -> Self {
        Self {
            breakers: true,
            name_grouping: true,
            time: None,
        }
    }
}

/// Converts the MATPOWER case `text`, read from the file `file_name`, whose
/// stem names the model's files.
pub fn matpower(
    file_name: &str,
    text: &str,
    options: &Options,
) -> Result<Converted, MatpowerError> {
    let case = matpower::parse::parse(text)?;
    matpower::map::convert(&case, stem(file_name), options)
}

/// Whether `file_name` names a file one of the converters reads.
pub fn convertible(file_name: &str) -> bool {
    file_name.to_ascii_lowercase().ends_with(".m")
}

/// `case14` for `dir/case14.m`.
fn stem(file_name: &str) -> &str {
    let base = file_name.rsplit(['/', '\\']).next().unwrap_or(file_name);
    base.rsplit_once('.').map_or(base, |(stem, _)| stem)
}
