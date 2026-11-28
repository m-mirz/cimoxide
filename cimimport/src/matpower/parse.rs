//! Reading a MATPOWER `.m` case: the matrices as written, nothing converted.
//!
//! Copied from gridoxide's `matpower::parse` (gridoxide depends on
//! cimoxide-model, so cimoxide cannot call it), and extended with what a
//! CGMES model has room for and a solver does not need: the case's name
//! (`function mpc = case14`) and the cell arrays `mpc.bus_name` and
//! `mpc.genfuel`.
//!
//! A case file is MATLAB code, and some cases finish with code that edits the
//! matrices they just wrote: `case33bw` converts its loads from kW and its
//! impedances from ohms that way. Reading the literals and ignoring the code
//! would convert a different network without a word, so an assignment to
//! `mpc.bus`, `mpc.gen`, `mpc.branch` or `mpc.baseMVA` that is not a literal
//! is an error (benchmark-grids' `matpower-plain/` holds such cases with the
//! code evaluated). So is a version-1 case, whose layout differs.

use std::fmt;

// Column indices, from MATPOWER's `idx_bus`, `idx_gen` and `idx_brch`.
pub const BUS_I: usize = 0;
pub const BUS_TYPE: usize = 1;
pub const PD: usize = 2;
pub const QD: usize = 3;
pub const GS: usize = 4;
pub const BS: usize = 5;
pub const BUS_AREA: usize = 6;
pub const VM: usize = 7;
pub const VA: usize = 8;
pub const BASE_KV: usize = 9;
pub const ZONE: usize = 10;
pub const VMAX: usize = 11;
pub const VMIN: usize = 12;
/// `LAM_P`, the first of the OPF results `LAM_P`, `LAM_Q`, `MU_VMAX`, `MU_VMIN`.
pub const LAM_P: usize = 13;
pub const MU_VMIN: usize = 16;
const BUS_COLUMNS: usize = 10;

pub const GEN_BUS: usize = 0;
pub const PG: usize = 1;
pub const QG: usize = 2;
pub const QMAX: usize = 3;
pub const QMIN: usize = 4;
pub const VG: usize = 5;
pub const MBASE: usize = 6;
pub const GEN_STATUS: usize = 7;
pub const PMAX: usize = 8;
pub const PMIN: usize = 9;
/// The capability curve, `PC1`, `PC2`, `QC1MIN`, `QC1MAX`, `QC2MIN`, `QC2MAX`.
pub const PC1: usize = 10;
pub const QC2MAX: usize = 15;
/// The ramp rates, `RAMP_AGC`, `RAMP_10`, `RAMP_30`, `RAMP_Q`.
pub const RAMP_AGC: usize = 16;
pub const RAMP_Q: usize = 19;
pub const APF: usize = 20;
/// The OPF results `MU_PMAX`, `MU_PMIN`, `MU_QMAX`, `MU_QMIN`.
pub const MU_PMAX: usize = 21;
pub const MU_QMIN: usize = 24;
const GEN_COLUMNS: usize = 8;

pub const F_BUS: usize = 0;
pub const T_BUS: usize = 1;
pub const BR_R: usize = 2;
pub const BR_X: usize = 3;
pub const BR_B: usize = 4;
pub const RATE_A: usize = 5;
pub const RATE_B: usize = 6;
pub const RATE_C: usize = 7;
pub const TAP: usize = 8;
pub const SHIFT: usize = 9;
pub const BR_STATUS: usize = 10;
pub const ANGMIN: usize = 11;
pub const ANGMAX: usize = 12;
pub const PF: usize = 13;
pub const QF: usize = 14;
pub const PT: usize = 15;
pub const QT: usize = 16;
/// The OPF results `MU_SF`, `MU_ST`, `MU_ANGMIN`, `MU_ANGMAX`.
pub const MU_SF: usize = 17;
pub const MU_ANGMAX: usize = 20;
const BRANCH_COLUMNS: usize = 11;

pub const PQ: f64 = 1.0;
pub const PV: f64 = 2.0;
pub const REF: f64 = 3.0;
pub const NONE: f64 = 4.0;

#[derive(Debug, Clone, PartialEq)]
pub enum MatpowerError {
    /// Text that is not what a MATPOWER case can contain at that point.
    Parse { line: usize, message: String },
    /// One of the four fields a power flow needs was never assigned.
    Missing(&'static str),
    /// A row with fewer columns than the format requires.
    ShortRow {
        matrix: &'static str,
        row: usize,
        columns: usize,
        required: usize,
    },
    /// A generator or branch naming a bus the bus matrix does not define.
    UnknownBus {
        matrix: &'static str,
        row: usize,
        bus: f64,
    },
    /// Something the file says that is refused rather than read wrongly.
    Unsupported(String),
}

impl fmt::Display for MatpowerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MatpowerError::Parse { line, message } => write!(f, "line {line}: {message}"),
            MatpowerError::Missing(field) => write!(f, "the case never assigns mpc.{field}"),
            MatpowerError::ShortRow {
                matrix,
                row,
                columns,
                required,
            } => write!(
                f,
                "mpc.{matrix} row {row} has {columns} columns, at least {required} are required"
            ),
            MatpowerError::UnknownBus { matrix, row, bus } => write!(
                f,
                "mpc.{matrix} row {row} names bus {bus}, which mpc.bus does not define"
            ),
            MatpowerError::Unsupported(what) => write!(f, "{what}"),
        }
    }
}

impl std::error::Error for MatpowerError {}

/// The matrices of a case, as written. Rows keep every column the file gave.
#[derive(Clone, Debug, Default)]
pub struct MatpowerCase {
    /// The function's name (`function mpc = case14`), if the file has one.
    pub name: Option<String>,
    pub base_mva: f64,
    pub bus: Vec<Vec<f64>>,
    pub gens: Vec<Vec<f64>>,
    pub branch: Vec<Vec<f64>>,
    /// `mpc.bus_name`, one per bus row, if the case has it.
    pub bus_name: Option<Vec<String>>,
    /// `mpc.genfuel`, one per generator row, if the case has it.
    pub genfuel: Option<Vec<String>>,
    /// Fields assigned that the conversion does not map (`gencost`, `dcline`,
    /// …), in file order.
    pub unmapped: Vec<String>,
}

/// Parses the text of a MATPOWER `.m` case.
pub fn parse(text: &str) -> Result<MatpowerCase, MatpowerError> {
    let code = strip_comments(text);
    let mut case = MatpowerCase {
        name: function_name(&code),
        ..MatpowerCase::default()
    };
    let (mut base_mva, mut bus, mut gens, mut branch) = (None, None, None, None);

    let bytes = code.as_bytes();
    let mut i = 0;
    while let Some(found) = code[i..].find("mpc.") {
        let start = i + found;
        i = start + 4;
        // Only where a statement begins: `xmpc.` or `a = mpc.bus(…)` is not
        // an assignment to the case.
        let before = code[..start].trim_end_matches([' ', '\t']);
        if !(before.is_empty() || before.ends_with(['\n', ';', ',', CONTINUED])) {
            continue;
        }
        let name_end = code[i..]
            .find(|c: char| !(c.is_ascii_alphanumeric() || c == '_'))
            .map_or(code.len(), |n| i + n);
        let name = &code[i..name_end];
        let line = line_of(&code, start);
        let mut j = skip_blanks(bytes, name_end);
        let read = matches!(name, "bus" | "gen" | "branch" | "baseMVA" | "version");
        if j >= bytes.len() || bytes[j] != b'=' || bytes.get(j + 1) == Some(&b'=') {
            if read {
                return Err(MatpowerError::Unsupported(format!(
                    "line {line}: mpc.{name} is modified by MATLAB code, which this converter \
                     does not run; use a case that states the matrices as plain data"
                )));
            }
            continue;
        }
        j = skip_blanks(bytes, j + 1);
        match name {
            "bus" | "gen" | "branch" => {
                if bytes.get(j) != Some(&b'[') {
                    return Err(MatpowerError::Unsupported(format!(
                        "line {line}: mpc.{name} is assigned an expression, not a literal \
                         matrix, and this converter does not run MATLAB code"
                    )));
                }
                let close =
                    code[j..]
                        .find(']')
                        .map(|n| j + n)
                        .ok_or_else(|| MatpowerError::Parse {
                            line,
                            message: format!("mpc.{name} has no closing ']'"),
                        })?;
                let rows = parse_matrix(&code[j + 1..close], line_of(&code, j))?;
                i = close + 1;
                match name {
                    "bus" => bus = Some(rows),
                    "gen" => gens = Some(rows),
                    _ => branch = Some(rows),
                }
            }
            "bus_name" | "genfuel" if bytes.get(j) == Some(&b'{') => {
                let close = cell_end(&code, j + 1).ok_or_else(|| MatpowerError::Parse {
                    line,
                    message: format!("mpc.{name} has no closing '}}'"),
                })?;
                let cells = parse_cells(&code[j + 1..close]);
                i = close + 1;
                if name == "bus_name" {
                    case.bus_name = Some(cells);
                } else {
                    case.genfuel = Some(cells);
                }
            }
            "baseMVA" => {
                let end = statement_end(&code, j);
                let value = code[j..end].trim();
                base_mva = Some(value.parse::<f64>().map_err(|_| {
                    MatpowerError::Unsupported(format!(
                        "line {line}: mpc.baseMVA = {value} is not a number"
                    ))
                })?);
                i = end;
            }
            "version" => {
                let end = statement_end(&code, j);
                let value = code[j..end].trim().trim_matches(['\'', '"']);
                if value != "2" {
                    return Err(MatpowerError::Unsupported(format!(
                        "line {line}: MATPOWER case format version {value}; only version 2 is read"
                    )));
                }
                i = end;
            }
            other => {
                if !case.unmapped.iter().any(|u| u == other) {
                    case.unmapped.push(other.to_string());
                }
            }
        }
    }

    case.base_mva = base_mva.ok_or(MatpowerError::Missing("baseMVA"))?;
    case.bus = bus.ok_or(MatpowerError::Missing("bus"))?;
    case.gens = gens.ok_or(MatpowerError::Missing("gen"))?;
    case.branch = branch.ok_or(MatpowerError::Missing("branch"))?;
    if case.base_mva.is_nan() || case.base_mva <= 0.0 {
        return Err(MatpowerError::Unsupported(format!(
            "mpc.baseMVA = {} is not positive",
            case.base_mva
        )));
    }
    check_columns("bus", &case.bus, BUS_COLUMNS)?;
    check_columns("gen", &case.gens, GEN_COLUMNS)?;
    check_columns("branch", &case.branch, BRANCH_COLUMNS)?;
    for (field, cells, rows) in [
        ("bus_name", &case.bus_name, case.bus.len()),
        ("genfuel", &case.genfuel, case.gens.len()),
    ] {
        if let Some(cells) = cells
            && cells.len() != rows
        {
            return Err(MatpowerError::Unsupported(format!(
                "mpc.{field} has {} entries for {rows} rows",
                cells.len()
            )));
        }
    }
    Ok(case)
}

/// A bus number as MATPOWER writes it: a positive whole number.
pub fn bus_number(value: f64) -> Option<u64> {
    (value >= 1.0 && value.fract() == 0.0 && value < u64::MAX as f64).then_some(value as u64)
}

/// `case14` from `function mpc = case14`.
fn function_name(code: &str) -> Option<String> {
    let rest = code.trim_start().strip_prefix("function")?;
    let after = rest.split_once('=').map_or(rest, |(_, r)| r);
    let name: String = after
        .trim_start()
        .chars()
        .take_while(|c| c.is_ascii_alphanumeric() || *c == '_')
        .collect();
    (!name.is_empty()).then_some(name)
}

fn check_columns(
    matrix: &'static str,
    rows: &[Vec<f64>],
    required: usize,
) -> Result<(), MatpowerError> {
    match rows.iter().position(|r| r.len() < required) {
        Some(row) => Err(MatpowerError::ShortRow {
            matrix,
            row: row + 1,
            columns: rows[row].len(),
            required,
        }),
        None => Ok(()),
    }
}

/// The text with every `%` comment removed and every `...` continuation
/// joined, line structure otherwise kept so positions still map to lines.
fn strip_comments(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    for line in text.lines() {
        let mut quoted = false;
        let mut end = line.len();
        for (k, c) in line.char_indices() {
            match c {
                '\'' => quoted = !quoted,
                '%' if !quoted => {
                    end = k;
                    break;
                }
                _ => {}
            }
        }
        let code = &line[..end];
        match code.find("...") {
            // A continuation: the next line carries on this one, and the rest
            // of this one is a comment. A vertical tab stands in for the line
            // break: blank to the tokenizer, still a line to `line_of`.
            Some(dots) => {
                out.push_str(&code[..dots]);
                out.push(CONTINUED);
            }
            None => {
                out.push_str(code);
                out.push('\n');
            }
        }
    }
    out
}

/// The line break of a `...` continuation, after [`strip_comments`].
const CONTINUED: char = '\x0b';

/// Rows of a matrix literal: separated by `;` or a newline, columns by
/// blanks or commas.
fn parse_matrix(body: &str, first_line: usize) -> Result<Vec<Vec<f64>>, MatpowerError> {
    let mut rows = Vec::new();
    let mut line = first_line;
    for (k, chunk) in body.split('\n').enumerate() {
        if k > 0 {
            line += 1;
        }
        let continued = chunk.matches(CONTINUED).count();
        for row in chunk.split(';') {
            let values = row
                .split(|c: char| c.is_whitespace() || c == ',')
                .filter(|t| !t.is_empty())
                .map(|t| {
                    t.parse::<f64>().map_err(|_| MatpowerError::Parse {
                        line,
                        message: format!("`{t}` is not a number"),
                    })
                })
                .collect::<Result<Vec<f64>, _>>()?;
            if !values.is_empty() {
                rows.push(values);
            }
        }
        line += continued;
    }
    Ok(rows)
}

/// The `}` closing a cell array whose body starts at `from`, quotes skipped.
fn cell_end(code: &str, from: usize) -> Option<usize> {
    let mut quoted = false;
    for (k, c) in code[from..].char_indices() {
        match c {
            '\'' => quoted = !quoted,
            '}' if !quoted => return Some(from + k),
            _ => {}
        }
    }
    None
}

/// The quoted strings of a cell array body, in order. MATLAB escapes a quote
/// inside one by doubling it.
fn parse_cells(body: &str) -> Vec<String> {
    let mut cells = Vec::new();
    let mut chars = body.chars().peekable();
    while let Some(c) = chars.next() {
        if c != '\'' {
            continue;
        }
        let mut cell = String::new();
        while let Some(c) = chars.next() {
            if c == '\'' {
                if chars.peek() == Some(&'\'') {
                    chars.next();
                    cell.push('\'');
                    continue;
                }
                break;
            }
            cell.push(c);
        }
        cells.push(cell);
    }
    cells
}

fn skip_blanks(bytes: &[u8], mut j: usize) -> usize {
    while j < bytes.len() && (bytes[j] == b' ' || bytes[j] == b'\t') {
        j += 1;
    }
    j
}

/// Where the statement starting at `j` ends: its `;` or the end of its line.
fn statement_end(code: &str, j: usize) -> usize {
    code[j..].find([';', '\n']).map_or(code.len(), |n| j + n)
}

fn line_of(code: &str, pos: usize) -> usize {
    code[..pos].matches(['\n', CONTINUED]).count() + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    pub(crate) const CASE: &str = "function mpc = tiny
%% a three-bus case
mpc.version = '2';
mpc.baseMVA = 100;
mpc.bus = [ % bus_i type Pd Qd Gs Bs area Vm Va baseKV zone Vmax Vmin
\t1\t3\t0\t0\t0\t0\t1\t1.02\t5\t230\t1\t1.1\t0.9;
\t2\t2\t20\t10\t0\t5\t1\t1\t0\t230\t1\t1.1\t0.9;
\t3\t1\t50, 20, 1, 0, 1, 1, 0, 230, 1, 1.1, 0.9
\t4\t4\t10\t0\t0\t0\t1\t1\t0\t230\t1\t1.1\t0.9;
];
mpc.gen = [
\t1\t0\t0\t300\t-300\t1.04\t100\t1;
\t2\t40\t5\t100\t-50\t1.01\t100\t1;
];
mpc.branch = [
\t1\t2\t0.01\t0.1\t0.02\t0\t0\t0\t0\t0\t1;
\t2\t3\t0.01\t0.1\t0\t0\t0\t0\t0.98\t-3\t1;
];
mpc.gencost = [
\t2\t0\t0\t2\t1\t0;
];
mpc.bus_name = {
\t'North';
\t'O''Hare 1';
\t'South';
\t'Spare';
};
";

    #[test]
    fn reads_the_matrices_names_and_what_it_leaves_out() {
        let case = parse(CASE).unwrap();
        assert_eq!(case.name.as_deref(), Some("tiny"));
        assert_eq!(case.base_mva, 100.0);
        assert_eq!(case.bus.len(), 4);
        assert_eq!(case.bus[2][..4], [3.0, 1.0, 50.0, 20.0]);
        assert_eq!(case.gens.len(), 2);
        assert_eq!(case.branch.len(), 2);
        assert_eq!(
            case.bus_name.as_deref(),
            Some(&["North", "O'Hare 1", "South", "Spare"].map(String::from)[..])
        );
        assert_eq!(case.genfuel, None);
        assert_eq!(case.unmapped, ["gencost"]);
    }

    #[test]
    fn code_that_edits_the_matrices_is_refused() {
        let text = format!("{CASE}\nmpc.bus(:, [PD, QD]) = mpc.bus(:, [PD, QD]) / 1e3;\n");
        let err = parse(&text).unwrap_err().to_string();
        assert!(err.contains("mpc.bus is modified by MATLAB code"), "{err}");
    }

    #[test]
    fn a_continued_row_and_a_commented_bracket_are_read() {
        let text = CASE.replace(
            "\t2\t3\t0.01\t0.1\t0\t0\t0\t0\t0.98\t-3\t1;",
            "\t2\t3\t0.01\t0.1\t0\t0 ... ]\n\t0\t0\t0.98\t-3\t1; % ]",
        );
        let case = parse(&text).unwrap();
        assert_eq!(case.branch[1][TAP], 0.98);
    }

    #[test]
    fn version_one_and_a_short_name_list_are_refused() {
        let text = CASE.replace("mpc.version = '2';", "mpc.version = '1';");
        assert!(parse(&text).unwrap_err().to_string().contains("version 1"));
        let text = CASE.replace("\t'Spare';\n", "");
        let err = parse(&text).unwrap_err().to_string();
        assert!(
            err.contains("mpc.bus_name has 3 entries for 4 rows"),
            "{err}"
        );
    }
}
