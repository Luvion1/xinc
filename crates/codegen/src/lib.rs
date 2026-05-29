//! Code generation for Xin.
//!
//! Compiles AST to machine code via HIR->MIR->LIR->LLVM.

mod generator;
mod hir;
mod lir;
mod mir;

pub use generator::{CodegenError, generate};
pub use hir::{HirError, HirExpr, HirStmt, lower_to_hir};
pub use lir::{LirFunction, lower_to_lir};
pub use mir::{MirFunction, lower_to_mir};
