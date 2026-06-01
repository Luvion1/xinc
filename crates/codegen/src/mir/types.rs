//! MIR types.

use crate::hir::ConstValue;

/// Virtual register.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct VirtualReg(pub usize);

/// MIR binary operator.
#[derive(Debug, Clone, Copy)]
pub enum MirBinaryOp {
    /// Integer add.
    IAdd,
    ISub,
    IMul,
    IDiv,
    IRem,
    IEq,
    INe,
    ILt,
    ILe,
    IGt,
    IGe,
    /// Bitwise ops.
    And,
    Or,
    Xor,
    Shl,
    Ashr,
    /// Logical ops.
    AndAnd,
    OrOr,
}

/// MIR value.
#[derive(Debug, Clone)]
pub enum MirValue {
    /// Constant.
    Const(ConstValue),
    /// Virtual register.
    Reg(VirtualReg),
}

/// MIR instruction.
#[derive(Debug, Clone)]
pub enum Instr {
    /// Assign value to virtual register.
    Assign { dst: VirtualReg, src: MirValue },
    /// Binary operation.
    BinOp { dst: VirtualReg, op: MirBinaryOp, left: VirtualReg, right: VirtualReg },
    /// Jump to block.
    Jump(usize),
    /// Conditional jump.
    Branch { cond: VirtualReg, then_bb: usize, else_bb: usize },
    /// Return value.
    Ret(Option<VirtualReg>),
}
