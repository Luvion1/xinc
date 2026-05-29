//! High-level Intermediate Representation.
//!
//! HIR is close to AST, used for high-level optimizations.

use xin_ast::{BinaryOp, Expression, Literal, Statement, UnaryOp};

/// HIR expression node.
#[derive(Debug, Clone)]
pub enum HirExpr {
    /// Literal value.
    Const(ConstValue),
    /// Variable reference.
    Var(String),
    /// Binary operation.
    BinaryOp(HirBinaryOp, Box<HirExpr>, Box<HirExpr>),
    /// Unary operation.
    UnaryOp(HirUnaryOp, Box<HirExpr>),
    /// Function call.
    Call {
        /// Function name.
        callee: String,
        /// Arguments.
        args: Vec<HirExpr>,
    },
    /// Ternary expression.
    Ternary {
        /// Condition.
        cond: Box<HirExpr>,
        /// Then branch.
        then_expr: Box<HirExpr>,
        /// Else branch.
        else_expr: Box<HirExpr>,
    },
}

/// HIR binary operator.
#[derive(Debug, Clone, Copy)]
pub enum HirBinaryOp {
    /// Arithmetic: +, -, *, /, %.
    Add,
    /// Subtract.
    Sub,
    /// Multiply.
    Mul,
    /// Divide.
    Div,
    /// Modulo.
    Mod,
    /// Equal.
    Eq,
    /// Not equal.
    Neq,
    /// Less than.
    Lt,
    /// Greater than.
    Gt,
    /// Bitwise AND.
    BitAnd,
    /// Bitwise OR.
    BitOr,
    /// Bitwise XOR.
    BitXor,
    /// Shift left.
    Shl,
    /// Shift right.
    Shr,
    /// Logical AND.
    And,
    /// Logical OR.
    Or,
}

/// HIR unary operator.
#[derive(Debug, Clone, Copy)]
pub enum HirUnaryOp {
    /// Negation.
    Neg,
    /// Logical NOT.
    Not,
    /// Bitwise NOT.
    BitNot,
}

/// Constant value.
#[derive(Debug, Clone)]
pub enum ConstValue {
    /// Integer.
    Number(String),
    /// String.
    String(String),
    /// Boolean.
    Boolean(bool),
    /// Null.
    Null,
}

/// HIR statement.
#[derive(Debug, Clone)]
pub enum HirStmt {
    /// Variable declaration.
    Let {
        /// Variable name.
        name: String,
        /// Initializer.
        value: HirExpr,
    },
    /// Expression statement.
    Expr(HirExpr),
    /// Block of statements.
    Block(Vec<HirStmt>),
}

/// Convert AST to HIR.
pub fn lower_to_hir(stmts: &[Statement]) -> Result<Vec<HirStmt>, HirError> {
    stmts.iter().map(stmt_to_hir).collect()
}

/// Convert statement to HIR.
fn stmt_to_hir(stmt: &Statement) -> Result<HirStmt, HirError> {
    match stmt {
        Statement::Let { name, value, .. } => {
            Ok(HirStmt::Let { name: name.clone(), value: expr_to_hir(value)? })
        }
        Statement::Expr(expr) => Ok(HirStmt::Expr(expr_to_hir(expr)?)),
        Statement::Block(stmts) => {
            Ok(HirStmt::Block(stmts.iter().map(stmt_to_hir).collect::<Result<Vec<_>, _>>()?))
        }
        _ => Err(HirError::Unsupported),
    }
}

/// Convert expression to HIR.
fn expr_to_hir(expr: &Expression) -> Result<HirExpr, HirError> {
    match expr {
        Expression::Literal(lit) => Ok(HirExpr::Const(const_to_hir(lit)?)),
        Expression::Identifier(name) => Ok(HirExpr::Var(name.clone())),
        Expression::Binary { left, op, right } => Ok(HirExpr::BinaryOp(
            binary_op_to_hir(op),
            Box::new(expr_to_hir(left)?),
            Box::new(expr_to_hir(right)?),
        )),
        Expression::Unary { op, operand } => {
            Ok(HirExpr::UnaryOp(unary_op_to_hir(op), Box::new(expr_to_hir(operand)?)))
        }
        Expression::Call { callee, args } => {
            let callee_name = match callee.as_ref() {
                Expression::Identifier(name) => name.clone(),
                _ => return Err(HirError::Unsupported),
            };
            Ok(HirExpr::Call {
                callee: callee_name,
                args: args.iter().map(expr_to_hir).collect::<Result<Vec<_>, _>>()?,
            })
        }
        Expression::Ternary { cond, then_expr, else_expr } => Ok(HirExpr::Ternary {
            cond: Box::new(expr_to_hir(cond)?),
            then_expr: Box::new(expr_to_hir(then_expr)?),
            else_expr: Box::new(expr_to_hir(else_expr)?),
        }),
    }
}

/// Convert literal to HIR constant.
fn const_to_hir(lit: &Literal) -> Result<ConstValue, HirError> {
    Ok(match lit {
        Literal::Number(n) => ConstValue::Number(n.clone()),
        Literal::String(s) => ConstValue::String(s.clone()),
        Literal::Boolean(b) => ConstValue::Boolean(*b),
        Literal::Null => ConstValue::Null,
    })
}

/// Convert binary operator to HIR.
fn binary_op_to_hir(op: &BinaryOp) -> HirBinaryOp {
    match op {
        BinaryOp::Add => HirBinaryOp::Add,
        BinaryOp::Sub => HirBinaryOp::Sub,
        BinaryOp::Mul => HirBinaryOp::Mul,
        BinaryOp::Div => HirBinaryOp::Div,
        BinaryOp::Mod => HirBinaryOp::Mod,
        BinaryOp::Eq => HirBinaryOp::Eq,
        BinaryOp::Neq => HirBinaryOp::Neq,
        BinaryOp::Lt => HirBinaryOp::Lt,
        BinaryOp::Gt => HirBinaryOp::Gt,
        BinaryOp::BitAnd => HirBinaryOp::BitAnd,
        BinaryOp::BitOr => HirBinaryOp::BitOr,
        BinaryOp::BitXor => HirBinaryOp::BitXor,
        BinaryOp::Shl => HirBinaryOp::Shl,
        BinaryOp::Shr => HirBinaryOp::Shr,
        BinaryOp::And => HirBinaryOp::And,
        BinaryOp::Or => HirBinaryOp::Or,
    }
}

/// Convert unary operator to HIR.
fn unary_op_to_hir(op: &UnaryOp) -> HirUnaryOp {
    match op {
        UnaryOp::Neg => HirUnaryOp::Neg,
        UnaryOp::Not => HirUnaryOp::Not,
        UnaryOp::BitNot => HirUnaryOp::BitNot,
    }
}

/// HIR lowering error.
#[derive(Debug, thiserror::Error)]
pub enum HirError {
    /// Unsupported statement.
    #[error("Unsupported statement for HIR")]
    Unsupported,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lower_let() {
        let stmt = Statement::Let {
            name: "x".to_string(),
            ty: None,
            value: Expression::Literal(Literal::Number("1".to_string())),
        };
        let hir = lower_to_hir(&[stmt]).unwrap();
        assert_eq!(hir.len(), 1);
    }

    #[test]
    fn test_lower_binary() {
        let expr = Expression::Binary {
            left: Box::new(Expression::Literal(Literal::Number("1".to_string()))),
            op: BinaryOp::Add,
            right: Box::new(Expression::Literal(Literal::Number("2".to_string()))),
        };
        let stmt = Statement::Expr(expr);
        let hir = lower_to_hir(&[stmt]).unwrap();
        assert_eq!(hir.len(), 1);
    }
}
