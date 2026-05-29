//! Code generation for Xin.
//!
//! Compiles AST to machine code.

mod generator;

pub use generator::{CodegenError, generate};
