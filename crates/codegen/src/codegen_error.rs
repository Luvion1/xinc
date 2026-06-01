//! Codegen error types.

use thiserror::Error;

/// Code generator error.
#[derive(Debug, Error)]
pub enum CodegenError {
    /// Invalid statement.
    #[error("Invalid statement for codegen")]
    InvalidStatement,
}
