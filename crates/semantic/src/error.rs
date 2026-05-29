//! Semantic error types.

use thiserror::Error;

/// Semantic analysis error.
#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum SemanticError {
    /// Undefined variable.
    #[error("Undefined variable: {0}")]
    UndefinedVariable(String),

    /// Type mismatch.
    #[error("Type mismatch: expected {expected}, found {found}")]
    TypeMismatch {
        /// Expected type.
        expected: String,
        /// Found type.
        found: String,
    },

    /// Invalid operation.
    #[error("Invalid operation: {0}")]
    InvalidOperation(String),

    /// Type error.
    #[error("Type error: {0}")]
    TypeError(String),
}
