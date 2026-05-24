pub mod violation;
pub use violation::Violation;

pub mod sparql;
pub mod detect;
pub use sparql::{Config, run_validation};
pub use detect::detect_config;

include!("generated_lib.rs");
