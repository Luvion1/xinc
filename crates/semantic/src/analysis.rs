//! Semantic analysis.
//!
//! Performs type checking and name resolution.

use super::SemanticError;
use crate::symbol::{Symbol, SymbolTable};
use xin_ast::{Expression, Statement};

/// Semantic analyzer.
pub struct Analyzer {
    symbols: SymbolTable,
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
                    Symbol { ty: ty.clone().map(|t| format!("{:?}", t)), mutable: true },
                );
                self.check_expr(value)?;
            }
            Statement::Fn { name, params: _, body, ret_ty } => {
                self.symbols.insert(
                    name.clone(),
                    Symbol { ty: ret_ty.as_ref().map(|t| format!("{:?}", t)), mutable: false },
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
            Expression::Ternary { cond, then_expr, else_expr } => {
                self.check_expr(cond)?;
                self.check_expr(then_expr)?;
                self.check_expr(else_expr)?;
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

#[cfg(test)]
mod tests {
    use super::*;
    use xin_ast::Type;
    use xin_ast::{BinaryOp, Literal, UnaryOp};

    #[test]
    fn test_analyzer_new() {
        let _analyzer = Analyzer::new();
    }

    #[test]
    fn test_analyzer_default() {
        let analyzer = Analyzer::default();
        let _ = analyzer;
    }

    #[test]
    fn test_analyze_let_ok() {
        let mut analyzer = Analyzer::new();
        let stmt = Statement::Let {
            name: "x".to_string(),
            ty: None,
            value: Expression::Literal(Literal::Number("42".to_string())),
        };
        assert!(analyzer.analyze(&stmt).is_ok());
    }

    #[test]
    fn test_analyze_let_with_type() {
        let mut analyzer = Analyzer::new();
        let stmt = Statement::Let {
            name: "x".to_string(),
            ty: Some(Type::Named("i32".to_string())),
            value: Expression::Literal(Literal::Number("42".to_string())),
        };
        assert!(analyzer.analyze(&stmt).is_ok());
    }

    #[test]
    fn test_analyze_expr_ok() {
        let mut analyzer = Analyzer::new();
        let stmt = Statement::Expr(Expression::Literal(Literal::Number("42".to_string())));
        assert!(analyzer.analyze(&stmt).is_ok());
    }

    #[test]
    fn test_analyze_if_ok() {
        let mut analyzer = Analyzer::new();
        let stmt = Statement::If {
            cond: Expression::Literal(Literal::Boolean(true)),
            then: vec![Statement::Expr(Expression::Literal(Literal::Number("1".to_string())))],
            r#else: None,
        };
        assert!(analyzer.analyze(&stmt).is_ok());
    }

    #[test]
    fn test_analyze_if_else_ok() {
        let mut analyzer = Analyzer::new();
        let stmt = Statement::If {
            cond: Expression::Literal(Literal::Boolean(true)),
            then: vec![Statement::Expr(Expression::Literal(Literal::Number("1".to_string())))],
            r#else: Some(Box::new(Statement::Expr(Expression::Literal(Literal::Number(
                "2".to_string(),
            ))))),
        };
        assert!(analyzer.analyze(&stmt).is_ok());
    }

    #[test]
    fn test_analyze_while_ok() {
        let mut analyzer = Analyzer::new();
        let stmt =
            Statement::While { cond: Expression::Literal(Literal::Boolean(true)), body: vec![] };
        assert!(analyzer.analyze(&stmt).is_ok());
    }

    #[test]
    fn test_analyze_fn_ok() {
        let mut analyzer = Analyzer::new();
        let stmt = Statement::Fn {
            name: "foo".to_string(),
            params: vec![],
            body: vec![Statement::Expr(Expression::Literal(Literal::Number("1".to_string())))],
            ret_ty: None,
        };
        assert!(analyzer.analyze(&stmt).is_ok());
    }

    #[test]
    fn test_analyze_return_ok() {
        let mut analyzer = Analyzer::new();
        let stmt = Statement::Return(Some(Expression::Literal(Literal::Number("42".to_string()))));
        assert!(analyzer.analyze(&stmt).is_ok());
    }

    #[test]
    fn test_analyze_return_void_ok() {
        let mut analyzer = Analyzer::new();
        let stmt = Statement::Return(None);
        assert!(analyzer.analyze(&stmt).is_ok());
    }

    #[test]
    fn test_analyze_block_ok() {
        let mut analyzer = Analyzer::new();
        let stmt = Statement::Block(vec![Statement::Expr(Expression::Literal(Literal::Number(
            "1".to_string(),
        )))]);
        assert!(analyzer.analyze(&stmt).is_ok());
    }

    #[test]
    fn test_analyze_binary_ok() {
        let mut analyzer = Analyzer::new();
        let stmt = Statement::Expr(Expression::Binary {
            left: Box::new(Expression::Literal(Literal::Number("1".to_string()))),
            op: BinaryOp::Add,
            right: Box::new(Expression::Literal(Literal::Number("2".to_string()))),
        });
        assert!(analyzer.analyze(&stmt).is_ok());
    }

    #[test]
    fn test_analyze_unary_ok() {
        let mut analyzer = Analyzer::new();
        let stmt = Statement::Expr(Expression::Unary {
            op: UnaryOp::Neg,
            operand: Box::new(Expression::Literal(Literal::Number("1".to_string()))),
        });
        assert!(analyzer.analyze(&stmt).is_ok());
    }

    #[test]
    fn test_analyze_ternary_ok() {
        let mut analyzer = Analyzer::new();
        let stmt = Statement::Expr(Expression::Ternary {
            cond: Box::new(Expression::Literal(Literal::Boolean(true))),
            then_expr: Box::new(Expression::Literal(Literal::Number("1".to_string()))),
            else_expr: Box::new(Expression::Literal(Literal::Number("2".to_string()))),
        });
        assert!(analyzer.analyze(&stmt).is_ok());
    }

    #[test]
    fn test_analyzer_new_type() {
        let analyzer = Analyzer::new();
        let _: *const SymbolTable = &analyzer.symbols;
    }
}
