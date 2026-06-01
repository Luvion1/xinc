//! Semantic analysis.
//!
//! Performs type checking and name resolution.

mod tests;

use super::SemanticError;
use crate::symbol::{Symbol, SymbolTable};
use xin_ast::{BinaryOp, Expression, Literal, Statement, UnaryOp};

/// Semantic analyzer.
pub struct Analyzer {
    pub symbols: SymbolTable,
}

impl Analyzer {
    /// Create new analyzer.
    pub fn new() -> Self {
        Self { symbols: SymbolTable::new() }
    }

    /// Analyze a statement.
    pub fn analyze(&mut self, stmt: &Statement) -> Result<(), SemanticError> {
        match stmt {
            Statement::Let { name, ty, value } => {
                self.symbols.insert(
                    name.clone(),
                    Symbol { ty: ty.clone().map(|t| format!("{t:?}")), mutable: true },
                );
                self.check_expr(value)?;
            }
            Statement::Fn { name, params: _, body, ret_ty } => {
                self.symbols.insert(
                    name.clone(),
                    Symbol { ty: ret_ty.as_ref().map(|t| format!("{t:?}")), mutable: false },
                );
                for stmt in body {
                    self.analyze(stmt)?;
                }
            }
            Statement::Expr(expr) => {
                self.check_expr(expr)?;
            }
            Statement::Return(expr) => {
                if let Some(e) = expr {
                    self.check_expr(e)?;
                }
            }
            Statement::Block(stmts) => {
                for stmt in stmts {
                    self.analyze(stmt)?;
                }
            }
            Statement::If { cond, then, r#else } => {
                self.check_expr(cond)?;
                for stmt in then {
                    self.analyze(stmt)?;
                }
                if let Some(else_stmt) = r#else {
                    self.analyze(else_stmt)?;
                }
            }
            Statement::While { cond, body } => {
                self.check_expr(cond)?;
                for stmt in body {
                    self.analyze(stmt)?;
                }
            }
            Statement::Assign { target, value } => {
                if self.symbols.lookup(target).is_none() {
                    return Err(SemanticError::UndefinedVariable(target.clone()));
                }
                self.check_expr(value)?;
            }
        }
        Ok(())
    }

    /// Check expression types.
    fn check_expr(&self, expr: &Expression) -> Result<(), SemanticError> {
        match expr {
            Expression::Identifier(name) => {
                if self.symbols.lookup(name).is_none() {
                    return Err(SemanticError::UndefinedVariable(name.clone()));
                }
            }
            Expression::Binary { left, right, op } => {
                self.check_expr(left)?;
                self.check_expr(right)?;
                match op {
                    BinaryOp::BitAnd | BinaryOp::BitOr | BinaryOp::BitXor => {
                        self.check_integer_operand(left, "bitwise")?;
                        self.check_integer_operand(right, "bitwise")?;
                    }
                    BinaryOp::Shl | BinaryOp::Shr => {
                        self.check_integer_operand(left, "shift")?;
                        self.check_integer_operand(right, "shift")?;
                    }
                    _ => {}
                }
            }
            Expression::Literal(_) => {}
            Expression::Unary { operand, op } => {
                self.check_expr(operand)?;
                if *op == UnaryOp::BitNot {
                    self.check_integer_operand(operand, "bitwise NOT")?;
                }
            }
            Expression::Call { callee, args } => {
                self.check_expr(callee)?;
                for arg in args {
                    self.check_expr(arg)?;
                }
            }
            Expression::Ternary { cond, then_expr, else_expr } => {
                self.check_expr(cond)?;
                self.check_expr(then_expr)?;
                self.check_expr(else_expr)?;
            }
        }
        Ok(())
    }

    #[allow(clippy::unused_self)]
    fn check_integer_operand(&self, expr: &Expression, op: &str) -> Result<(), SemanticError> {
        match expr {
            Expression::Literal(Literal::Number(_)) | Expression::Identifier(_) => Ok(()),
            _ => Err(SemanticError::TypeError(format!(
                "operand for {op} operator must be integer",
            ))),
        }
    }
}

impl Default for Analyzer {
    fn default() -> Self {
        Self::new()
    }
}
