//! Statement AST nodes.
//!
//! All statement types in the AST.

/// Statement enum.
#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    /// Variable declaration.
    Let { name: String, ty: Option<Type>, value: super::Expression },
    /// Function declaration.
    Fn { name: String, params: Vec<Param>, body: Vec<Statement>, ret_ty: Option<Type> },
    /// Expression statement.
    Expr(super::Expression),
    /// Return statement.
    Return(Option<super::Expression>),
    /// Block statement.
    Block(Vec<Statement>),
    /// If statement.
    If { cond: super::Expression, then: Vec<Statement>, r#else: Option<Box<Statement>> },
    /// While loop statement.
    While { cond: super::Expression, body: Vec<Statement> },
    /// Assignment statement.
    Assign { target: String, value: super::Expression },
}

/// Function parameter.
#[derive(Debug, Clone, PartialEq)]
pub struct Param {
    /// Parameter name.
    pub name: String,
    /// Parameter type.
    pub ty: Option<Type>,
}

/// Type reference.
#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    /// Built-in type.
    Builtin(BuiltinType),
    /// Custom type.
    Named(String),
}

/// Built-in types.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BuiltinType {
    /// 32-bit signed integer.
    I32,
    /// 64-bit signed integer.
    I64,
    /// 32-bit float.
    F32,
    /// 64-bit float.
    F64,
    /// Boolean.
    Bool,
    /// String.
    Str,
}

/// Type alias.
pub type TypeRef = Type;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Expression, Literal};

    #[test]
    fn test_let_statement_clone() {
        let stmt = Statement::Let {
            name: "x".to_string(),
            ty: Some(Type::Builtin(BuiltinType::I32)),
            value: Expression::Literal(Literal::Number("42".to_string())),
        };
        let cloned = stmt.clone();
        assert_eq!(stmt, cloned);
    }

    #[test]
    fn test_block_statement() {
        let stmt = Statement::Block(vec![]);
        assert!(matches!(stmt, Statement::Block(_)));
    }

    #[test]
    fn test_if_statement() {
        let stmt = Statement::If {
            cond: Expression::Literal(Literal::Boolean(true)),
            then: vec![],
            r#else: None,
        };
        assert!(matches!(stmt, Statement::If { .. }));
    }

    #[test]
    fn test_fn_statement() {
        let stmt = Statement::Fn {
            name: "main".to_string(),
            params: vec![],
            body: vec![],
            ret_ty: None,
        };
        assert!(matches!(stmt, Statement::Fn { .. }));
    }

    #[test]
    fn test_return_statement() {
        let stmt = Statement::Return(None);
        assert!(matches!(stmt, Statement::Return(None)));
    }

    #[test]
    fn test_expr_statement() {
        let stmt = Statement::Expr(Expression::Literal(Literal::Null));
        assert!(matches!(stmt, Statement::Expr(_)));
    }

    #[test]
    fn test_builtin_type_variants() {
        let types = [BuiltinType::I32, BuiltinType::I64, BuiltinType::F32,
                     BuiltinType::F64, BuiltinType::Bool, BuiltinType::Str];
        for t in types {
            assert_eq!(t, t);
        }
    }

    #[test]
    fn test_type_equality() {
        assert_eq!(
            Type::Builtin(BuiltinType::I32),
            Type::Builtin(BuiltinType::I32)
        );
    }
}