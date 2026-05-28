# Xin Language Documentation

Documentation for the Xin programming language - a modern systems programming language with automatic memory management and a strict type system.

## Overview

Xin is a modern systems programming language that combines automatic memory management (via XGC) with a strict type system guaranteeing operational correctness. It compiles to LLVM IR for native performance while maintaining developer-friendly syntax, immutability by default, and structured concurrency.

## Documentation Structure

### Language Reference

| Document | Description |
|----------|-------------|
| [Introduction](language/01-introduction.md) | Quick reference, overview, and design philosophy |
| [Lexical Structure](language/02-lexical-structure.md) | Source files, identifiers, literals, operators, comments |
| [Type System](language/03-type-system.md) | Primitive types, composite types, null safety, refinement types, enums, structs |
| [Language Syntax](language/04-language-syntax.md) | Functions, closures, control flow |
| [Error Handling](language/05-error-handling.md) | Result type, custom errors, best practices |
| [Concurrency Model](language/06-concurrency.md) | Structured concurrency, fibers, async/await, synchronization |
| [Traits and Generics](language/07-traits-generics.md) | Trait definitions, implementations, bounds, generic functions |
| [Compile-Time Evaluation](language/08-compile-time.md) | Const functions, const evaluation, compile-time refinement checking |
| [Modules and Visibility](language/09-modules.md) | Module system, visibility modifiers, imports |
| [Memory Management](language/10-memory-management.md) | XGC garbage collector, memory safety guarantees, value semantics |
| [Standard Library](language/11-standard-library.md) | Core types, collections, I/O, concurrency primitives |
| [Tooling](language/12-tooling.md) | Compiler, package manager, LSP |
| [Complete Examples](language/13-examples.md) | Full program examples with refinement types |

### Standards

| Document | Description |
|----------|-------------|
| [Keywords Reference](standards/14-keywords.md) | Complete keyword reference with descriptions |
| [Naming Conventions](standards/15-naming-conventions.md) | PascalCase standards, file naming, acronyms |
| [Code Formatting](standards/16-code-formatting.md) | Indentation, line length, spacing, braces |
| [Documentation Standards](standards/17-documentation.md) | Doc comments, inline comments, coverage requirements |
| [Testing Standards](standards/18-testing.md) | Coverage requirements, test categories, naming, performance |
| [Code Quality Metrics](standards/19-code-quality.md) | Complexity metrics, maintainability, anti-patterns |
| [Security Standards](standards/20-security.md) | Input validation, vulnerability response, prohibited patterns |
| [Performance Standards](standards/21-performance.md) | Performance requirements, benchmarking, regression handling |
| [Build and CI](standards/22-build-ci.md) | Build requirements, CI pipeline, quality gates |
| [Code Review](standards/23-code-review.md) | Review checklist, requirements, response times |

### Governance

| Document | Description |
|----------|-------------|
| [Risk Management](governance/24-risk-management.md) | Risk categories, issue severity |
| [Compliance](governance/25-compliance.md) | Standards compliance, audit requirements, definitions |
| [Development Guidelines](governance/26-development-guidelines.md) | SOLID, DRY, YAGNI, KISS, code ethics |
| [References](governance/27-references.md) | Language influences, academic research, version history |

## Quick Reference

```xin
// Variable declaration
let X = 42;

// Mutable variable
let mut Y = 10;

// Function definition
fn Name(Param: Type) -> ReturnType { }

// Error propagation
let Data = ReadFile(path)?;

// Optional handling
let Value = Opt ?? default;

// Struct
struct Point { X: i32, Y: i32 }

// Enum
enum Color { Red, Green, Blue }

// Trait
trait Printable { fn Format(Self) -> String; }

// Refinement type
type Nat = { i32 | _ >= 0 }
```

## Links

- [SPEC.md](../SPEC.md) - Single-file specification document
- [GitHub Repository](https://github.com/xin-lang/xin)

## Status

| Property | Value |
|----------|-------|
| Version | 1.0.0-stable |
| Edition | C/LLVM Backend |
| Status | Stable Release |
