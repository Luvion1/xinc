# 1. Introduction

## Quick Reference

Common syntax patterns at a glance:

| Pattern | Example | See |
|---------|---------|-----|
| **Variable declaration** | `let X = 42;` | Sec 4.7 |
| **Mutable variable** | `let mut Y = 10;` | Sec 4.7 |
| **Function definition** | `fn Name(Param: Type) -> ReturnType { }` | Sec 5.1 |
| **If-else** | `if Cond { } else { }` | Sec 5.3 |
| **Match expression** | `match Val { Pattern => Expr, _ => Default }` | Sec 4.5 |
| **Error propagation** | `let Data = ReadFile(path)?;` | Sec 6.1 |
| **Optional handling** | `let Value = Opt ?? default;` | Sec 4.3 |
| **Loop** | `for Item in Items { }` | Sec 5.3 |
| **Struct** | `struct Point { X: i32, Y: i32 }` | Sec 4.6 |
| **Enum** | `enum Color { Red, Green, Blue }` | Sec 4.5 |
| **Traits** | `trait Printable { fn Format(Self) -> String; }` | Sec 8.1 |
| **Refinement type** | `type Nat = { i32 | _ >= 0 }` | Sec 4.4 |
| **Import** | `use std.IO;` | Sec 10.3 |
| **Async function** | `async fn Fetch() -> Result<String, Error> { }` | Sec 7.3 |
| **Closure** | `let Add = |A: i32, B: i32| -> i32 { A + B };` | Sec 5.2 |
| **Nullable type** | `Option<T>` | Sec 4.3, 4.5 |
| **Error type** | `Result<T, E>` | Sec 4.5, 6 |

## Language Overview

Xin is a modern programming language designed for building reliable, high-performance systems. It combines automatic memory management (via XGC, a ZGC-inspired garbage collector) with a strict type system that guarantees operational correctness. Xin compiles to LLVM IR for native performance while maintaining clean, developer-friendly syntax.

**Target Audience**: Developers who demand security and speed—those who need provably correct systems without sacrificing productivity.

## Design Philosophy

| Principle | Description |
|-----------|-------------|
| **Simple & Clean** | Minimal syntax and intuitive abstractions. Focus on business logic, not memory management or pointer arithmetic. |
| **Memory Managed** | XGC handles automatic memory allocation with low-latency GC designed for real-time systems. |
| **Immutability by Default** | Data is immutable by default. Use `let mut` syntax for mutable variables to prevent unintended state mutation and enable safer concurrency. |
| **Correctness by Construction** | Type system prevents entire categories of bugs before runtime. Exhaustiveness checking, bounds validation, and null safety are enforced at compile time. Refinement types extend these guarantees with richer invariant specifications. |
| **Modular Architecture** | Compiler built with high modularity using C, enabling isolated development and testing. |

## Design Goals

| Goal | Description |
|------|-------------|
| Reliability | Eliminate entire bug categories through strict static typing, including refinement types |
| Performance | Compiles to optimized native code via LLVM with minimal runtime overhead |
| Productivity | Modern tooling, clear error messages, fast compilation times |
| Safety | Null safety, bounds checking, exhaustive pattern matching, refinement validation |
| Concurrency | Structured concurrency with fiber-based parallelism |
| Correctness | Refinement types and compile-time verification to guarantee business invariants |
| Maintainability | Modular architecture with comprehensive documentation |
