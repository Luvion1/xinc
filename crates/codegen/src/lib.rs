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
    clippy::collapsible_if,
    clippy::trivially_copy_pass_by_ref,
    clippy::unnecessary_wraps,
    clippy::use_self,
    clippy::match_same_arms,
    clippy::uninlined_format_args
)]

//! Xin code generation.
//!
//! Lowers a semantically-checked AST into a textual representation. The
//! crate exposes a staged pipeline:
//!
//! ```text
//! AST → [lower_to_hir] → HIR → [lower_to_mir] → MIR → [lower_to_lir] → LIR
//! ```
//!
//! At the top level, [`generate`] takes a slice of statements and returns
//! the generated source as a `String`. Currently only `let` bindings and
//! expression statements are emitted; everything else surfaces as
//! [`CodegenError::InvalidStatement`].
//!
//! # Architecture
//!
//! - [`hir`] — high-level IR, close to the AST. Each `HirStmt` / `HirExpr`
//!   corresponds to a syntactic form. Lowering is mostly a structural
//!   recursion.
//! - [`mir`] — mid-level IR. Functions become [`MirFunction`]s with explicit
//!   locals and a small instruction set. Control flow is not yet modeled.
//! - [`lir`] — low-level IR. [`LirFunction`] is the planned target for an
//!   LLVM backend (not yet wired up).
//! - [`generator`] — the text-level emitter. Walks an AST and produces
//!   parenthesized infix code, e.g. `(1 + (2 * 3))`.
//!
//! # Errors
//!
//! All public functions return [`Result<_, CodegenError>`]. The single
//! variant [`CodegenError::InvalidStatement`] signals a statement kind
//! that has no lowering rule yet (e.g. `if`, `while`, `fn` bodies).
//!
//! # Example
//!
//! ```ignore
//! use xin_codegen::generate;
//! let stmts = /* from xin_parser::parse_statement */;
//! let code = generate(&stmts).unwrap();
//! ```

mod codegen_error;
mod generator;
mod hir;
mod lir;
mod mir;

pub use codegen_error::CodegenError;
pub use generator::{generate, generate_expression};
pub use hir::{HirError, HirExpr, HirStmt, lower_to_hir};
pub use lir::{LirFunction, lower_to_lir};
pub use mir::{MirFunction, lower_to_mir};
