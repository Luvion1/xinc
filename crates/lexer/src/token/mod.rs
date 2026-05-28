//! Token definitions for Xin language lexer.
//!
//! This module defines the complete token set used by the lexer to represent
//! the source code as a sequence of meaningful units. Tokens include:
//!
//! - **Literals**: numeric, string, character, boolean, null
//! - **Keywords**: language reserved words (let, fn, if, etc.)
//! - **Operators**: arithmetic, logical, bitwise, assignment
//! - **Punctuation**: grouping, separation, referencing symbols
//! - **Identifiers**: user-defined names
//!
//! Each token carries a `TokenKind` (the syntactic category) and a `Position`
//! (line/column) for error reporting and source mapping.
//!
//! # Example
//! ```ignore
//! use xin_lexer::token::{Token, TokenKind, Keyword};
//! let tok = Token::new(TokenKind::Keyword(Keyword::Let), 1, 1);
//! assert!(matches!(tok.kind, TokenKind::Keyword(Keyword::Let)));
//! ```

//!
//! The lexer produces a stream of tokens from source text, which the parser
//! consumes to build the AST. Invalid tokens are reported as `LexerError`.

pub mod identifier;
pub mod keyword;
pub mod literal;
pub mod operator;
pub mod punctuation;

// Re-export only what is actually used in TokenKind
pub use keyword::Keyword;

use super::diagnostics::Position;

/// A token with position information.
#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub position: Position,
}

impl Token {
    pub fn new(kind: TokenKind, line: u32, column: u32) -> Self {
        Self { kind, position: Position { line, column } }
    }

    pub fn eof() -> Self {
        Self { kind: TokenKind::Eof, position: Position { line: 0, column: 0 } }
    }
}

/// All possible token kinds in Xin language.
#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    /// End of file
    Eof,

    /// Numeric literal
    Number(String),

    /// String literal (value without quotes)
    String(String),

    /// Character literal
    Char(char),

    /// Boolean literal
    Bool(bool),

    /// Null literal
    Null,

    /// Keyword token.
    ///
    /// Contains a `Keyword` enum representing a reserved word.
    Keyword(Keyword),

    /// Identifier (variable or function name)
    Identifier(String),

    // --- Operators ---
    Plus,    // +
    Minus,   // -
    Star,    // *
    Slash,   // /
    Percent, // %
    Eq,      // ==
    Neq,     // !=
    Lt,      // <
    Lte,     // <=
    Gt,      // >
    Gte,     // >=
    And,     // &&
    Or,      // ||
    Not,     // !
    /// Bitwise NOT: ~
    BitNot,
    BitAnd,    // &
    BitOr,     // |
    BitXor,    // ^
    Shl,       // <<
    Shr,       // >>
    Assign,    // =
    AddAssign, // +=
    SubAssign, // -=
    MulAssign, // *=
    DivAssign, // /=
    ModAssign, // %=
    /// AND assignment: &=
    AndAssign,
    /// OR assignment: |=
    OrAssign,
    /// XOR assignment: ^=
    XorAssign,
    /// Left shift assignment: <<=
    ShlAssign,
    /// Right shift assignment: >>=
    ShrAssign,

    /// Concatenation operator: ~
    Concat,
    /// Null coalescing: ??
    Coalesce,
    /// Optional chaining: ?.
    OptionChain,

    // --- Punctuation ---
    LParen,    // (
    RParen,    // )
    LBrace,    // {
    RBrace,    // }
    LBracket,  // [
    RBracket,  // ]
    Comma,     // ,
    Semicolon, // ;
    Colon,     // :
    /// At sign: @
    At,
    /// Hash/pound: #
    Hash,
    Dot,            // .
    Arrow,          // ->
    FatArrow,       // =>
    DoubleColon,    // ::
    Range,          // ..
    RangeInclusive, // ..=
    Underscore,     // _
}
