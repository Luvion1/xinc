//! Mid-level Intermediate Representation.
//!
//! MIR is in SSA form for optimization.

mod function;
mod types;

pub use function::{MirFunction, lower_to_mir};
pub use types::{Instr, MirBinaryOp, MirValue};
