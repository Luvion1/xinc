#![deny(clippy::all, clippy::pedantic, clippy::nursery)]
#![allow(
    clippy::module_name_repetitions,
    clippy::missing_docs_in_private_items,
    clippy::must_use_candidate,
    clippy::missing_const_for_fn,
    clippy::missing_errors_doc,
    clippy::missing_panics_doc,
    clippy::option_if_let_else,
    clippy::unnested_or_patterns,
    clippy::redundant_closure_for_method_calls,
    clippy::collapsible_if
)]

//! Xin semantic analysis.
//!
//! Performs name resolution and type checking over an AST produced by
//! [`xin_parser`](https://docs.rs/xin-parser). The output is suitable for
//! handing to [`xin_codegen`](https://docs.rs/xin-codegen).
//!
//! # Main types
//!
//! - [`Analyzer`] — the workhorse. Holds a [`SymbolTable`] and walks a
//!   stream of statements, validating each in turn. Built via
//!   [`Analyzer::new`].
//! - [`SymbolTable`] — a flat name → [`Symbol`] map. The analyzer inserts
//!   entries for every `let` and `fn` declaration, and rejects assignments
//!   to unknown names.
//! - [`Symbol`] — currently a `{ ty, mutable }` pair. The `ty` is stored
//!   as a `String` (the [`Debug`] form of the [`xin_ast::Type`]) rather
//!   than the [`xin_ast::Type`] itself; this keeps semantic independent of
//!   the AST's type system evolution.
//! - [`SemanticError`] — analysis failures: undefined variables, type
//!   mismatches, non-integer operands to bitwise/shift operators, etc.
//!
//! # Pipeline position
//!
//! ```text
//! source → [xin-lexer] → tokens → [xin-parser] → AST → [xin-semantic] → checked AST → [xin-codegen]
//! ```
//!
//! # Example
//!
//! ```ignore
//! use xin_semantic::Analyzer;
//! let mut analyzer = Analyzer::new();
//! // for each statement: analyzer.analyze(stmt)?;
//! ```
//!
//! # Limitations
//!
//! - Single-scope: the analyzer does not yet implement block scoping.
//!   A `let` inside a block is visible to the whole function.
//! - No control-flow analysis: a `let` followed by an assignment to the
//!   same name in a different branch is not flagged.
//! - Type inference is shallow: bitwise/shift operands are required to be
//!   integer literals or identifiers; no actual numeric type checking.

mod analysis;
mod error;
mod symbol;

pub use analysis::Analyzer;
pub use error::SemanticError;
pub use symbol::{Symbol, SymbolTable};
