//! Integration tests.
//!
//! Tests the full pipeline: lexer → parser → semantic.

#[cfg(test)]
mod tests {
    use xin_lexer::tokenize;
    use xin_parser::parse_statement;
    use xin_semantic::Analyzer;

    #[test]
    fn test_pipeline_let_literal() {
        let source = "let x = 10;";
        let _tokens = tokenize(source).unwrap();
        let stmts = parse_statement(source).unwrap();
        let mut analyzer = Analyzer::new();
        for stmt in &stmts {
            analyzer.analyze(stmt).unwrap();
        }
        assert_eq!(stmts.len(), 1);
    }

    #[test]
    fn test_pipeline_let_with_type() {
        let source = "let x: i32 = 42;";
        let stmts = parse_statement(source).unwrap();
        assert_eq!(stmts.len(), 1);
        assert!(matches!(stmts[0], xin_ast::Statement::Let { .. }));
    }

    #[test]
    fn test_pipeline_if_statement() {
        let source = "if x { let y = 1; }";
        let stmts = parse_statement(source).unwrap();
        assert!(!stmts.is_empty());
    }

    #[test]
    fn test_pipeline_semantic_undefined_var() {
        let source = "let x = y;";
        let stmts = parse_statement(source).unwrap();
        let mut analyzer = Analyzer::new();
        let mut found_error = false;
        for stmt in &stmts {
            if analyzer.analyze(stmt).is_err() {
                found_error = true;
            }
        }
        assert!(found_error);
    }

    #[test]
    fn test_pipeline_empty_input() {
        let source = "";
        let stmts = parse_statement(source);
        assert!(stmts.is_ok());
        assert!(stmts.unwrap().is_empty());
    }

    #[test]
    fn test_pipeline_multiple_statements() {
        let source = "let x = 1; let y = 2;";
        let stmts = parse_statement(source).unwrap();
        assert_eq!(stmts.len(), 2);
    }

    #[test]
    fn test_pipeline_while_statement() {
        let source = "while x { x = 1; }";
        let stmts = parse_statement(source).unwrap();
        assert!(!stmts.is_empty());
    }

    #[test]
    fn test_pipeline_modulo() {
        let source = "let z = 10 % 3;";
        let stmts = parse_statement(source).unwrap();
        assert_eq!(stmts.len(), 1);
    }

    #[test]
    fn test_pipeline_bitwise_and() {
        let source = "let z = a & b;";
        let stmts = parse_statement(source).unwrap();
        assert_eq!(stmts.len(), 1);
    }

    #[test]
    fn test_pipeline_bitwise_or() {
        let source = "let z = a | b;";
        let stmts = parse_statement(source).unwrap();
        assert_eq!(stmts.len(), 1);
    }

    #[test]
    fn test_pipeline_bitwise_xor() {
        let source = "let z = a ^ b;";
        let stmts = parse_statement(source).unwrap();
        assert_eq!(stmts.len(), 1);
    }

    #[test]
    fn test_pipeline_shift_left() {
        let source = "let z = 1 << 4;";
        let stmts = parse_statement(source).unwrap();
        assert_eq!(stmts.len(), 1);
    }

    #[test]
    fn test_pipeline_shift_right() {
        let source = "let z = 8 >> 2;";
        let stmts = parse_statement(source).unwrap();
        assert_eq!(stmts.len(), 1);
    }

    #[test]
    fn test_pipeline_logical_and() {
        let source = "let z = a && b;";
        let stmts = parse_statement(source).unwrap();
        assert_eq!(stmts.len(), 1);
    }

    #[test]
    fn test_pipeline_logical_or() {
        let source = "let z = a || b;";
        let stmts = parse_statement(source).unwrap();
        assert_eq!(stmts.len(), 1);
    }

    #[test]
    fn test_pipeline_bitnot() {
        let source = "let z = ~x;";
        let stmts = parse_statement(source).unwrap();
        assert_eq!(stmts.len(), 1);
    }

    #[test]
    fn test_pipeline_codegen_binary() {
        use xin_codegen::generate;
        let source = "let x = 1 + 2;";
        let stmts = parse_statement(source).unwrap();
        let code = generate(&stmts).unwrap();
        assert!(code.contains("(1 + 2)"));
    }

    #[test]
    fn test_pipeline_codegen_bitwise() {
        use xin_codegen::generate;
        let source = "let x = a & b;";
        let stmts = parse_statement(source).unwrap();
        let code = generate(&stmts).unwrap();
        assert!(code.contains("(a & b)"));
    }

    #[test]
    fn test_pipeline_codegen_logical() {
        use xin_codegen::generate;
        let source = "let x = a && b;";
        let stmts = parse_statement(source).unwrap();
        let code = generate(&stmts).unwrap();
        assert!(code.contains("(a && b)"));
    }
}
