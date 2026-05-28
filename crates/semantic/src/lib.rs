//! Semantic analysis module.
//!
//! Performs type checking and other analyses.

mod analysis;
mod analysis_tests;
mod codegen;
mod error;
mod symbol;

pub use analysis::*;
pub use codegen::*;
pub use error::SemanticError;
pub use symbol::*;