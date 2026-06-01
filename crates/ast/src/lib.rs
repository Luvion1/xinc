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

//! Xin abstract syntax tree.
//!
//! Defines the data structures that represent parsed Xin source code.
//! These nodes are produced by [`xin_parser`](https://docs.rs/xin-parser)
//! and consumed by semantic analysis and code generation.
//!
//! # Structure
//!
//! The AST is split into two top-level categories:
//!
//! - [`Expression`] — values and computations that produce a value.
//! - [`Statement`] — top-level program constructs that compose expressions
//!   and control flow.
//!
//! Helper types [`Literal`], [`Type`], [`BuiltinType`], [`Param`], and the
//! type alias [`TypeRef`] live alongside the statement module.
//!
//! # Design notes
//!
//! - All nodes are `Clone` and `PartialEq`/`Eq` so they can be compared
//!   in tests and round-tripped through analysis passes.
//! - Operator kinds ([`BinaryOp`], [`UnaryOp`]) are plain enums without
//!   precedence information; precedence is the parser's concern, not the
//!   AST's.
//! - No source positions are tracked on nodes; if you need spans, wrap the
//!   parser output in a separate span type.

mod expression;
mod statement;

pub use expression::*;
pub use statement::*;
