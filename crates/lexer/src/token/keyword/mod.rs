//! Keyword token definitions.
//! All reserved words in the Xin language.

/// Complete set of Xin keywords.
///
/// Keywords have special meaning and cannot be used as identifiers.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Keyword {
    /// Function definition: `fn`
    Fn,
    /// Variable declaration: `let`
    Let,
    /// Mutable binding: `mut`
    Mut,
    /// Constant definition: `const`
    Const,
    /// Conditional: `if`
    If,
    /// Else branch: `else`
    Else,
    /// Pattern matching: `match`
    Match,
    /// For loop: `for`
    For,
    /// While loop: `while`
    While,
    /// Infinite loop: `loop`
    Loop,
    /// Return from function: `return`
    Return,
    /// Break from loop: `break`
    Break,
    /// Continue loop: `continue`
    Continue,
    /// Struct definition: `struct`
    Struct,
    /// Enum definition: `enum`
    Enum,
    /// Trait definition: `trait`
    Trait,
    /// Implementation block: `impl`
    Impl,
    /// Type alias: `type`
    Type,
    /// Module import: `use`
    Use,
    /// Module declaration: `mod`
    Mod,
    /// Public visibility: `pub`
    Pub,
    /// Async function: `async`
    Async,
    /// Await expression: `await`
    Await,
    /// Move closure: `move`
    Move,
    /// Reference pattern: `ref`
    Ref,
    /// Self parameter: `self`
    Self_,
    /// Self type: `Self`
    SelfType,
    /// Boolean true: `true`
    True,
    /// Boolean false: `false`
    False,
    /// Null value: `null`
    Null,
    /// Signed integer types
    I8,
    I16,
    I32,
    I64,
    I128,
    Isize,
    /// Unsigned integer types
    U8,
    U16,
    U32,
    U64,
    U128,
    Usize,
    /// Floating point types
    F32,
    F64,
    /// Type keywords
    Bool,
    Char,
    Str,
    /// Standard library types
    Result,
    Option,
}

/// Get keyword from string identifier.
///
/// Checks if a string matches a keyword.
///
/// # Arguments
/// * `name` - String to check
///
/// # Returns
/// Some(Keyword) if it's a keyword, None otherwise
pub fn from_str(name: &str) -> Option<Keyword> {
    match name {
        "fn" => Some(Keyword::Fn),
        "let" => Some(Keyword::Let),
        "mut" => Some(Keyword::Mut),
        "const" => Some(Keyword::Const),
        "if" => Some(Keyword::If),
        "else" => Some(Keyword::Else),
        "match" => Some(Keyword::Match),
        "for" => Some(Keyword::For),
        "while" => Some(Keyword::While),
        "loop" => Some(Keyword::Loop),
        "return" => Some(Keyword::Return),
        "break" => Some(Keyword::Break),
        "continue" => Some(Keyword::Continue),
        "struct" => Some(Keyword::Struct),
        "enum" => Some(Keyword::Enum),
        "trait" => Some(Keyword::Trait),
        "impl" => Some(Keyword::Impl),
        "type" => Some(Keyword::Type),
        "use" => Some(Keyword::Use),
        "mod" => Some(Keyword::Mod),
        "pub" => Some(Keyword::Pub),
        "async" => Some(Keyword::Async),
        "await" => Some(Keyword::Await),
        "move" => Some(Keyword::Move),
        "ref" => Some(Keyword::Ref),
        "self" => Some(Keyword::Self_),
        "Self" => Some(Keyword::SelfType),
        "true" => Some(Keyword::True),
        "false" => Some(Keyword::False),
        "null" => Some(Keyword::Null),
        "i8" => Some(Keyword::I8),
        "i16" => Some(Keyword::I16),
        "i32" => Some(Keyword::I32),
        "i64" => Some(Keyword::I64),
        "i128" => Some(Keyword::I128),
        "isize" => Some(Keyword::Isize),
        "u8" => Some(Keyword::U8),
        "u16" => Some(Keyword::U16),
        "u32" => Some(Keyword::U32),
        "u64" => Some(Keyword::U64),
        "u128" => Some(Keyword::U128),
        "usize" => Some(Keyword::Usize),
        "f32" => Some(Keyword::F32),
        "f64" => Some(Keyword::F64),
        "bool" => Some(Keyword::Bool),
        "char" => Some(Keyword::Char),
        "str" => Some(Keyword::Str),
        "Result" => Some(Keyword::Result),
        "Option" => Some(Keyword::Option),
        _ => None,
    }
}

#[cfg(test)]
pub mod tests;
