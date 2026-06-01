//! MIR function representation.
//!
//! The mid-level IR organizes every function as a linear (for now) list
//! of [`BasicBlock`]s. Each block holds a [`Vec<Instr>`]; control-flow
//! edges between blocks are not yet represented — branches are emitted
//! as terminator instructions only when the lowering pass adds them.
//!
//! # Virtual registers
//!
//! [`MirFunction::next_vreg`] is a monotonic counter. Every
//! [`VirtualReg`] allocated by [`MirFunction::alloc_vreg`] takes the
//! current value and bumps it. There is no deallocation: virtual
//! registers are infinite-arity and only ever needed during the LIR
//! pass.
//!
//! # Lowering
//!
//! The HIR → MIR pass is in [`super`] (the parent `mir` module). It
//! walks a [`crate::hir::HirStmt`] sequence and produces a
//! [`MirFunction`] per `fn` declaration; top-level expression
//! statements share a single synthetic "main" function.

use super::types::{Instr, MirBinaryOp, MirValue, VirtualReg};
use crate::hir::{HirBinaryOp, HirExpr, HirStmt};

/// A linear sequence of MIR instructions.
///
/// Control-flow is not modeled at the block level; an `if` lowers to a
/// branch instruction at the end of a block, followed by a fresh block
/// for the join point.
#[derive(Debug, Clone)]
pub struct BasicBlock {
    /// Instructions in this block, in execution order.
    pub instrs: Vec<Instr>,
}

/// A MIR-level function.
///
/// Constructed via [`MirFunction::new`]. The first [`BasicBlock`] is
/// pre-allocated and serves as the entry block.
#[derive(Debug, Clone)]
pub struct MirFunction {
    /// Function name.
    pub name: String,
    /// Basic blocks, in insertion order. `blocks[0]` is the entry block.
    pub blocks: Vec<BasicBlock>,
    /// Next virtual register ID to allocate.
    pub next_vreg: usize,
}

impl MirFunction {
    /// Create a new MIR function with a single empty entry block.
    pub fn new(name: impl Into<String>) -> Self {
        Self { name: name.into(), blocks: vec![BasicBlock { instrs: vec![] }], next_vreg: 0 }
    }

    /// Allocate virtual register.
    pub fn new_reg(&mut self) -> VirtualReg {
        let reg = VirtualReg(self.next_vreg);
        self.next_vreg += 1;
        reg
    }
}

/// Convert HIR to MIR.
pub fn lower_to_mir(hir: &[HirStmt]) -> MirFunction {
    let mut func = MirFunction::new("main");
    for stmt in hir {
        lower_stmt(stmt, &mut func);
    }
    func
}

/// Lower HIR statement to MIR.
fn lower_stmt(stmt: &HirStmt, func: &mut MirFunction) -> VirtualReg {
    match stmt {
        HirStmt::Let { name: _, value } => lower_expr(value, func),
        HirStmt::Expr(expr) => lower_expr(expr, func),
        HirStmt::Block(stmts) => {
            let mut reg = VirtualReg(0);
            for s in stmts {
                reg = lower_stmt(s, func);
            }
            reg
        }
    }
}

/// Lower HIR expression to MIR.
fn lower_expr(expr: &HirExpr, func: &mut MirFunction) -> VirtualReg {
    match expr {
        HirExpr::Const(cv) => {
            let reg = func.new_reg();
            func.blocks
                .last_mut()
                .unwrap()
                .instrs
                .push(Instr::Assign { dst: reg, src: MirValue::Const(cv.clone()) });
            reg
        }
        HirExpr::Var(_) => func.new_reg(),
        HirExpr::BinaryOp(op, left, right) => {
            let dst = func.new_reg();
            let left_reg = lower_expr(left, func);
            let right_reg = lower_expr(right, func);
            let mir_op = match op {
                HirBinaryOp::Add => MirBinaryOp::IAdd,
                HirBinaryOp::Sub => MirBinaryOp::ISub,
                HirBinaryOp::Mul => MirBinaryOp::IMul,
                HirBinaryOp::Div => MirBinaryOp::IDiv,
                HirBinaryOp::Mod => MirBinaryOp::IRem,
                HirBinaryOp::Eq => MirBinaryOp::IEq,
                HirBinaryOp::Neq => MirBinaryOp::INe,
                HirBinaryOp::Lt => MirBinaryOp::ILt,
                HirBinaryOp::Gt => MirBinaryOp::IGt,
                HirBinaryOp::BitAnd => MirBinaryOp::And,
                HirBinaryOp::BitOr => MirBinaryOp::Or,
                HirBinaryOp::BitXor => MirBinaryOp::Xor,
                HirBinaryOp::Shl => MirBinaryOp::Shl,
                HirBinaryOp::Shr => MirBinaryOp::Ashr,
                HirBinaryOp::And => MirBinaryOp::AndAnd,
                HirBinaryOp::Or => MirBinaryOp::OrOr,
            };
            func.blocks.last_mut().unwrap().instrs.push(Instr::BinOp {
                dst,
                op: mir_op,
                left: left_reg,
                right: right_reg,
            });
            dst
        }
        HirExpr::UnaryOp(_op, operand) => lower_expr(operand, func),
        HirExpr::Call { callee, .. } => {
            let _ = callee;
            func.new_reg()
        }
        HirExpr::Ternary { .. } => func.new_reg(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mir_new() {
        let func = MirFunction::new("test");
        assert_eq!(func.name, "test");
        assert_eq!(func.blocks.len(), 1);
    }

    #[test]
    fn test_lower_empty() {
        let mir = lower_to_mir(&[]);
        assert!(mir.blocks.is_empty() || mir.blocks[0].instrs.is_empty());
    }
}
