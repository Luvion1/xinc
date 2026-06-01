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

//! Code generation for Xin.
//!
//! Compiles AST to machine code via HIR->MIR->LIR->LLVM.

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
