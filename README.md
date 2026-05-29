# Xin Language

[![Tests](https://github.com/Luvion1/xinc/actions/workflows/ci.yml/badge.svg)](https://github.com/Luvion1/xinc/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

A statically-typed compiled language with clean architecture and test-driven development.

## Features

- **Multi-crate workspace**: Clean separation of lexer, parser, AST, semantic analysis, codegen
- **390+ passing tests** across all crates
- **Clean Architecture**: Domain-driven design with strict layer boundaries
- **Zero-panic production**: All errors use `Result<T, E>` with `thiserror`
- **Edition 2024**: Modern Rust with latest language features
- **IR Pipeline**: HIR → MIR → LIR → LLVM code generation

## Crates

| Crate | Description | Tests |
|-------|-------------|-------|
| `xin-lexer` | Lexical analysis with token recognition | 138 |
| `xin-parser` | Recursive descent parser for statements/expressions | 50 |
| `xin-ast` | AST node definitions | 18 |
| `xin-semantic` | Type checking and name resolution | 40 |
| `xin-codegen` | LLVM IR generation pipeline | 27 |
| Integration | End-to-end pipeline tests | 30 |

## Quick Start

```bash
# Build all crates
cargo build --workspace

# Run tests
cargo test --workspace

# Clippy check
cargo clippy --workspace -- -D warnings
```

## Language Syntax

```
// Variable declaration
let x = 1;
let y: i32 = 42;

// Functions
fn add(a: i32, b: i32) -> i32 {
    a + b
}

// Control flow
if x > 0 {
    x = x - 1
} else {
    x = 0
}

while x {
    x = x - 1
}
```

## Architecture

```
crates/
├── lexer/      # Token stream generation
├── parser/     # Parse tokens to AST
├── ast/        # AST node definitions
├── semantic/   # Type checking, symbol resolution
└── codegen/    # HIR->MIR->LIR->LLVM pipeline
```

## Development Standards

- Max 200 SLOC per file
- Max 5 files per folder
- 10+ levels folder nesting encouraged
- Documentation required (///, //!)
- `thiserror` for all error types

## License

MIT