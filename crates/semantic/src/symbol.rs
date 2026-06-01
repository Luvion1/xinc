//! Symbol table.
//!
//! Tracks variable and function definitions.

use std::collections::HashMap;

use xin_ast::Type;

/// Symbol table for name resolution.
#[derive(Debug, Default)]
pub struct SymbolTable {
    // Storage for symbols
    symbols: HashMap<String, Symbol>,
}

impl SymbolTable {
    /// Create new symbol table.
    pub fn new() -> Self {
        Self::default()
    }

    /// Insert a symbol.
    pub fn insert(&mut self, name: String, symbol: Symbol) {
        self.symbols.insert(name, symbol);
    }

    /// Lookup a symbol.
    pub fn lookup(&self, name: &str) -> Option<&Symbol> {
        self.symbols.get(name)
    }
}

/// Symbol information.
#[derive(Debug, Clone)]
pub struct Symbol {
    /// Symbol type.
    pub ty: Option<Type>,
    /// Is mutable.
    pub mutable: bool,
}
