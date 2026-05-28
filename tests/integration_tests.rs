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
}