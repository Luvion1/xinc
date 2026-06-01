//! Semantic analyzer implementation.
//!
//! This module houses the [`Analyzer`] struct and its traversal logic. The
//! analyzer is a single-pass visitor that walks a slice of statements in
//! order, populating a [`SymbolTable`] and enforcing a small set of type
//! rules along the way.
//!
//! # Pass structure
//!
//! [`Analyzer::analyze`] dispatches on the statement kind:
//!
//! | Statement | Action |
//! |-----------|--------|
//! | `Let`     | Insert binding with `mutable = true`, validate the initializer. |
//! | `Fn`      | Insert binding with `mutable = false`, recurse into the body. |
//! | `Expr`    | Recurse into the expression. |
//! | `Return`  | Validate the optional value expression. |
//! | `Block`   | Recurse into each statement. |
//! | `If`      | Recurse into condition, then-branch, optional else. |
//! | `While`   | Recurse into condition and body. |
//! | `Assign`  | Require the target to already be in the symbol table, validate value. |
//!
//! Expressions are handled by [`Analyzer::check_expr`], which performs
//! name resolution for identifiers and type-checks bitwise / shift
//! operands through [`Analyzer::check_integer_operand`].
//!
//! # Scope
//!
//! The analyzer does not yet implement block scoping. A `let` inside a
//! block is therefore visible for the rest of the program, including after
//! the block ends. This is documented as a limitation in the parent module.

mod tests;

use super::SemanticError;
use crate::symbol::{Symbol, SymbolTable};
use xin_ast::{BinaryOp, Expression, Literal, Statement, UnaryOp};

/// Semantic analyzer.
///
/// Holds a [`SymbolTable`] of every binding seen so far. Reuse a single
/// instance for a complete program; the analyzer mutates the table as it
/// walks. Use [`Analyzer::new`] to construct one with an empty table.
pub struct Analyzer {
    /// Symbol table populated as statements are analyzed.
    pub symbols: SymbolTable,
}

impl Analyzer {
    /// Create a new analyzer with an empty symbol table.
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
