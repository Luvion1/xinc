//! Low-level Intermediate Representation.
//!
//! LIR is close to machine code, register-based.

use super::hir::ConstValue;
use super::mir::{Instr, MirBinaryOp, MirFunction, MirValue};

/// LIR instruction set.
#[derive(Debug, Clone)]
pub enum LirInstr {
    /// Move immediate.
    MovImm { dst: PhysReg, imm: i64 },
    /// Move register.
    MovReg { dst: PhysReg, src: PhysReg },
    /// Binary ALU operation.
    BinOp { op: LirOp, dst: PhysReg, src: PhysReg },
    /// Register to register binary.
    BinOp3 { op: LirOp, dst: PhysReg, left: PhysReg, right: PhysReg },
    /// Jump to label.
    Jmp(String),
    /// Conditional jump.
    Jcc { cond: PhysReg, label: String },
    /// Return.
    Ret(Option<PhysReg>),
}

/// LIR ALU operation.
#[derive(Debug, Clone, Copy)]
pub enum LirOp {
    /// Add.
    Add,
    /// Subtract.
    Sub,
    /// Multiply.
    Mul,
    /// Divide.
    Div,
    /// Signed remainder.
    Rem,
    /// And.
    And,
    /// Or.
    Or,
    /// Xor.
    Xor,
    /// Shift left.
    Shl,
    /// Arithmetic shift right.
    Sar,
    /// Signed less than.
    Slt,
    /// Signed less or equal.
    Sle,
    /// Signed greater than.
    Sgt,
    /// Signed greater or equal.
    Sge,
}

/// Physical register.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PhysReg(pub usize);

/// LIR function.
#[derive(Debug, Clone)]
pub struct LirFunction {
    /// Function name.
    pub name: String,
    /// Instructions.
    pub instrs: Vec<LirInstr>,
}

/// Convert MIR to LIR.
pub fn lower_to_lir(mir: &MirFunction) -> LirFunction {
    let mut lir = LirFunction::new(&mir.name);
    for block in &mir.blocks {
        for instr in &block.instrs {
            lower_instr(instr, &mut lir);
        }
    }
    lir
}

impl LirFunction {
    /// Create new LIR function.
    pub fn new(name: &str) -> Self {
        Self { name: name.to_string(), instrs: vec![] }
    }
}

/// Lower MIR instruction to LIR.
fn lower_instr(instr: &Instr, lir: &mut LirFunction) {
    match instr {
        Instr::Assign { dst: _, src } => {
            if let MirValue::Const(cv) = src {
                let reg = PhysReg(0);
                let imm = match cv {
                    ConstValue::Number(n) => n.parse::<i64>().unwrap_or(0),
                    _ => 0,
                };
                lir.instrs.push(LirInstr::MovImm { dst: reg, imm });
            }
        }
        Instr::BinOp { dst: _, op, left: _, right: _ } => {
            let lir_op = match op {
                MirBinaryOp::IAdd => LirOp::Add,
                MirBinaryOp::ISub => LirOp::Sub,
                MirBinaryOp::IMul => LirOp::Mul,
                MirBinaryOp::IDiv => LirOp::Div,
                MirBinaryOp::IRem => LirOp::Rem,
                MirBinaryOp::And => LirOp::And,
                MirBinaryOp::Or => LirOp::Or,
                MirBinaryOp::Xor => LirOp::Xor,
                MirBinaryOp::Shl => LirOp::Shl,
                MirBinaryOp::Ashr => LirOp::Sar,
                MirBinaryOp::ILe => LirOp::Sle,
                MirBinaryOp::IGe => LirOp::Sge,
                _ => LirOp::Add,
            };
            lir.instrs.push(LirInstr::BinOp3 {
                op: lir_op,
                dst: PhysReg(0),
                left: PhysReg(1),
                right: PhysReg(2),
            });
        }
        Instr::Ret(_) => lir.instrs.push(LirInstr::Ret(None)),
        Instr::Jump(_) => {}
        Instr::Branch { .. } => {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lir_new() {
        let func = LirFunction::new("main");
        assert_eq!(func.name, "main");
    }

    #[test]
    fn test_lower_mir() {
        let mir = MirFunction::new("test");
        let lir = lower_to_lir(&mir);
        assert_eq!(lir.name, "test");
    }
}
