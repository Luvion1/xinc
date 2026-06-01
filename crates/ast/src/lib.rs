#![deny(clippy::all, clippy::pedantic, clippy::nursery)]
#![allow(
    clippy::module_name_repetitions,
    clippy::missing_docs_in_private_items,
    clippy::must_use_candidate,
    clippy::missing_const_for_fn,
    clippy::missing_errors_doc,
    clippy::missing_panics_doc,
    clippy::option_if_let_else
)]

//! Xin AST crate.
//!
//! Contains all AST node definitions.

mod expression;
mod statement;

pub use expression::*;
pub use statement::*;
