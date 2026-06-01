//! Codegen types.

/// Code generator error.
#[derive(Debug, thiserror::Error)]
pub enum CodegenError {
    /// Invalid statement.
    #[error("Invalid statement for codegen")]
    InvalidStatement,
}