//! Semantic analysis tests.

#[cfg(test)]
mod tests {
    use crate::{Analyzer, SymbolTable, Symbol};
    use xin_ast::{Statement, Expression, Literal};

    #[test]
    fn test_analyzer_let() {
        let mut analyzer = Analyzer::new();
        let stmt = Statement::Let {
            name: "x".to_string(),
            ty: None,
            value: Expression::Literal(Literal::Number("42".to_string())),
        };
        analyzer.analyze(&stmt).unwrap();
    }

    #[test]
    fn test_analyzer_let_with_type() {
        let mut analyzer = Analyzer::new();
        let stmt = Statement::Let {
            name: "x".to_string(),
            ty: Some(xin_ast::Type::Builtin(xin_ast::BuiltinType::I32)),
            value: Expression::Literal(Literal::Number("42".to_string())),
        };
        analyzer.analyze(&stmt).unwrap();
    }

    #[test]
    fn test_analyzer_expr_statement() {
        let mut analyzer = Analyzer::new();
        let stmt = Statement::Expr(Expression::Literal(Literal::Boolean(true)));
        analyzer.analyze(&stmt).unwrap();
    }

    #[test]
    fn test_analyzer_block() {
        let mut analyzer = Analyzer::new();
        let stmt = Statement::Block(vec![]);
        analyzer.analyze(&stmt).unwrap();
    }

    #[test]
    fn test_analyzer_return() {
        let mut analyzer = Analyzer::new();
        let stmt = Statement::Return(None);
        analyzer.analyze(&stmt).unwrap();
    }

    #[test]
    fn test_symbol_table() {
        let mut table = SymbolTable::new();
        table.insert("x".to_string(), Symbol {
            ty: Some("i32".to_string()),
            mutable: true,
        });
        assert!(table.lookup("x").is_some());
    }

    #[test]
    fn test_symbol_table_not_found() {
        let table = SymbolTable::new();
        assert!(table.lookup("missing").is_none());
    }

    #[test]
    fn test_symbol_table_mutable() {
        let mut table = SymbolTable::new();
        table.insert("x".to_string(), Symbol {
            ty: Some("i32".to_string()),
            mutable: true,
        });
        assert!(table.lookup("x").unwrap().mutable);
    }

    #[test]
    fn test_undefined_variable() {
        let mut analyzer = Analyzer::new();
        let stmt = Statement::Expr(Expression::Identifier("undefined".to_string()));
        assert!(analyzer.analyze(&stmt).is_err());
    }

    #[test]
    fn test_analyzer_if() {
        let mut analyzer = Analyzer::new();
        let stmt = Statement::If {
            cond: Expression::Literal(Literal::Boolean(true)),
            then: vec![],
            r#else: None,
        };
        analyzer.analyze(&stmt).unwrap();
    }

    #[test]
    fn test_analyzer_if_else() {
        let mut analyzer = Analyzer::new();
        let stmt = Statement::If {
            cond: Expression::Literal(Literal::Boolean(true)),
            then: vec![],
            r#else: Some(Box::new(Statement::Return(None))),
        };
        analyzer.analyze(&stmt).unwrap();
    }

    #[test]
    fn test_analyzer_nested_block() {
        let mut analyzer = Analyzer::new();
        let stmt = Statement::Block(vec![
            Statement::Let {
                name: "x".to_string(),
                ty: None,
                value: Expression::Literal(Literal::Number("1".to_string())),
            },
        ]);
        analyzer.analyze(&stmt).unwrap();
    }

    #[test]
    fn test_analyzer_while() {
        let mut analyzer = Analyzer::new();
        let stmt = Statement::While {
            cond: Expression::Literal(Literal::Boolean(true)),
            body: vec![],
        };
        analyzer.analyze(&stmt).unwrap();
    }

    #[test]
    fn test_analyzer_while_with_body() {
        let mut analyzer = Analyzer::new();
        let stmt = Statement::While {
            cond: Expression::Literal(Literal::Boolean(true)),
            body: vec![
                Statement::Expr(Expression::Literal(Literal::Number("1".to_string()))),
            ],
        };
        analyzer.analyze(&stmt).unwrap();
    }

    #[test]
    fn test_analyzer_unary() {
        let mut analyzer = Analyzer::new();
        let stmt = Statement::Expr(Expression::Unary {
            op: xin_ast::UnaryOp::Not,
            operand: Box::new(Expression::Literal(Literal::Boolean(true))),
        });
        analyzer.analyze(&stmt).unwrap();
    }

    #[test]
    fn test_analyzer_function_call() {
        let mut analyzer = Analyzer::new();
        // Just test that function call syntax is supported
        let stmt = Statement::Expr(Expression::Call {
            callee: Box::new(Expression::Literal(Literal::Number("1".to_string()))),
            args: vec![],
        });
        analyzer.analyze(&stmt).unwrap();
    }

    #[test]
    fn test_analyzer_binop_chain() {
        let mut analyzer = Analyzer::new();
        let stmt = Statement::Expr(Expression::Binary {
            left: Box::new(Expression::Literal(Literal::Number("1".to_string()))),
            op: xin_ast::BinaryOp::Add,
            right: Box::new(Expression::Binary {
                left: Box::new(Expression::Literal(Literal::Number("2".to_string()))),
                op: xin_ast::BinaryOp::Mul,
                right: Box::new(Expression::Literal(Literal::Number("3".to_string()))),
            }),
        });
        analyzer.analyze(&stmt).unwrap();
    }
}
