//! Semantic analysis.
//!
//! Performs type checking and name resolution.

use crate::symbol::{Symbol, SymbolTable};
use xin_ast::{Expression, Statement};
use super::SemanticError;

/// Semantic analyzer.
pub struct Analyzer {
    symbols: SymbolTable,
}

impl Analyzer {
    /// Create new analyzer.
    pub fn new() -> Self {
        Self {
            symbols: SymbolTable::new(),
        }
    }

    /// Analyze a statement.
    pub fn analyze(&mut self, stmt: &Statement) -> Result<(), SemanticError> {
        match stmt {
            Statement::Let { name, ty, value } => {
                self.symbols.insert(
                    name.clone(),
                    Symbol {
                        ty: ty.clone().map(|t| format!("{:?}", t)),
                        mutable: true,
                    },
                );
                self.check_expr(value)?;
            }
            Statement::Fn { name, params: _, body, ret_ty } => {
                self.symbols.insert(
                    name.clone(),
                    Symbol {
                        ty: ret_ty.as_ref().map(|t| format!("{:?}", t)),
                        mutable: false,
                    },
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
            Expression::Binary { left, right, .. } => {
                self.check_expr(left)?;
                self.check_expr(right)?;
            }
            Expression::Literal(_) => {}
            Expression::Unary { operand, .. } => {
                self.check_expr(operand)?;
            }
            Expression::Call { callee, args } => {
                self.check_expr(callee)?;
                for arg in args {
                    self.check_expr(arg)?;
                }
            }
        }
        Ok(())
    }
}

impl Default for Analyzer {
    fn default() -> Self {
        Self::new()
    }
}