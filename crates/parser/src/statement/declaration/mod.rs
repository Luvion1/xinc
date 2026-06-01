//! Declaration statement parsers.
//!
//! Parses top-level declarations: `let` bindings and `fn` function
//! definitions. Each statement variant is implemented in its own file
//! and is dispatched from the parent statement module.

pub mod fn_stmt;
pub mod let_stmt;
