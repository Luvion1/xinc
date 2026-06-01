#![deny(clippy::all, clippy::pedantic, clippy::nursery)]
#![allow(
    clippy::module_name_repetitions,
    clippy::missing_docs_in_private_items,
    clippy::must_use_candidate,
    clippy::missing_const_for_fn,
    clippy::missing_errors_doc,
    clippy::missing_panics_doc,
    clippy::option_if_let_else,
    clippy::unnested_or_patterns,
    clippy::redundant_closure_for_method_calls,
    clippy::collapsible_if
)]

//! Semantic analysis module.
//!
//! Performs type checking and other analyses.

mod analysis;
mod error;
mod symbol;

pub use analysis::Analyzer;
pub use error::SemanticError;
pub use symbol::{Symbol, SymbolTable};
