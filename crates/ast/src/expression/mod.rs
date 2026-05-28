//! Expression AST nodes.
//!
//! All expression types in the AST.

/// Expression enum.
#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    /// Literal value.
    Literal(Literal),
    /// Identifier reference.
    Identifier(String),
    /// Binary operation.
    Binary { left: Box<Expression>, op: BinaryOp, right: Box<Expression> },
    /// Unary operation.
    Unary { op: UnaryOp, operand: Box<Expression> },
    /// Function call.
    Call { callee: Box<Expression>, args: Vec<Expression> },
}

/// Literal value.
#[derive(Debug, Clone, PartialEq, Default)]
pub enum Literal {
    /// String literal.
    String(String),
    /// Number literal.
    Number(String),
    /// Boolean literal.
    Boolean(bool),
/// Null literal.
    #[default]
    Null,
}

/// Binary operator.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BinaryOp {
    /// Addition.
    Add,
    /// Subtraction.
    Sub,
    /// Multiplication.
    Mul,
    /// Division.
    Div,
    /// Equality.
    Eq,
    /// Inequality.
    Neq,
    /// Less than.
    Lt,
    /// Greater than.
    Gt,
}

/// Unary operator.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum UnaryOp {
    /// Negation.
    Neg,
    /// Not.
    Not,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_literal_default_is_null() {
        let lit = Literal::default();
        assert_eq!(lit, Literal::Null);
    }

    #[test]
    fn test_expression_clone() {
        let expr = Expression::Literal(Literal::Number("42".to_string()));
        let cloned = expr.clone();
        assert_eq!(expr, cloned);
    }

    #[test]
    fn test_binary_op_eq() {
        let left = Expression::Literal(Literal::Number("1".to_string()));
        let right = Expression::Literal(Literal::Number("2".to_string()));
        let expr = Expression::Binary {
            left: Box::new(left.clone()),
            op: BinaryOp::Add,
            right: Box::new(right.clone()),
        };
        assert!(matches!(expr, Expression::Binary { op: BinaryOp::Add, .. }));
    }

    #[test]
    fn test_unary_op_neg() {
        let operand = Expression::Literal(Literal::Number("5".to_string()));
        let expr = Expression::Unary {
            op: UnaryOp::Neg,
            operand: Box::new(operand),
        };
        assert!(matches!(expr, Expression::Unary { op: UnaryOp::Neg, .. }));
    }

    #[test]
    fn test_call_expression() {
        let callee = Expression::Identifier("print".to_string());
        let expr = Expression::Call {
            callee: Box::new(callee),
            args: vec![Expression::Literal(Literal::String("hi".to_string()))],
        };
        assert!(matches!(expr, Expression::Call { .. }));
    }

    #[test]
    fn test_literal_string() {
        let lit = Literal::String("hello".to_string());
        assert_eq!(lit, Literal::String("hello".to_string()));
    }

    #[test]
    fn test_literal_boolean_true() {
        let lit = Literal::Boolean(true);
        assert_eq!(lit, Literal::Boolean(true));
    }

    #[test]
    fn test_binary_op_variants() {
        let ops = [BinaryOp::Add, BinaryOp::Sub, BinaryOp::Mul, BinaryOp::Div,
                    BinaryOp::Eq, BinaryOp::Neq, BinaryOp::Lt, BinaryOp::Gt];
        for op in ops {
            assert_eq!(op, op);
        }
    }
}