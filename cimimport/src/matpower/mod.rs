//! MATPOWER cases (`.m`, format version 2).
//!
//! [`parse`] reads the matrices as written; [`map`] turns them into a CGMES
//! model, by the rules its documentation gives.

pub mod map;
pub mod parse;

pub use parse::{MatpowerCase, MatpowerError};
