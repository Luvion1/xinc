# 13. Tooling

## Compiler

```
xinc [options] <inputs>

Compiler written in C with LLVM backend for native code generation.

Options:
  -o <file>           Output file (executable or object)
  --emit <type>      Emit: llvm-ir, asm, obj, bin, depfile
  --opt <level>      Optimization level: 0-3 (0=debug, 3=aggressive)
  --target <triple>  Target triple (default: host)
  --verbose          Verbose output
  --refine-check     Enable stricter refinement checking
  -c                 Compile to object file only
  -E                 Preprocess only
  -M, -MM            Generate dependency information
  -j <n>             Parallel compilation (number of jobs)
  --gc <type>        Garbage collector type: xgc, refcount, none
```

The compiler pipeline:
1. Lexical analysis (Lexer)
2. Parsing (Parser) → AST
3. Type checking & refinement verification (TypeCheck)
4. HIR (High-level IR) generation
5. MIR (Mid-level IR) optimization
6. LLVM IR generation
7. LLVM optimization passes
8. Native code emission

Target platforms: x86_64, aarch64, riscv64, wasm32

## Package Manager

```
xinpkg <command> [options]

Package manager for Xin language dependencies (C-based installation).

Commands:
  new <name>         Create new package/project
  add <dep>          Add dependency to xin.toml
  remove <dep>       Remove dependency
  build              Build package and dependencies
  test               Run test suite
  run [args]         Execute built binary with args
  install [path]     Install local or remote package
  publish            Publish package to registry (requires auth)
  update             Update dependencies to latest versions
  clean              Remove build artifacts
  doc                Generate documentation
  check              Type check without building
  bench              Run benchmarks
  fmt                Format source files

Configuration: xin.toml (package manifest)
Registry: https://pkg.xin-lang.org (default)
Cache: ~/.xin/cache (dependency cache)
```

## LSP (Language Server Protocol)

LSP server implemented in C/C++ for optimal performance:

- Code completion (context-aware)
- Go-to definition / declaration
- Find all references
- Rename refactoring (with project-wide updates)
- Hover information (type, docs, constraints)
- Error diagnostics & warnings (real-time)
- Signature help
- Inlay hints (type annotations, refinement constraints)
- Document symbols (outline view)
- Code actions (quick fixes)
- Formatting (using xinc --format)
- Diagnostics for refinement violations, unused variables, etc.

Configuration: `.xin/config.json` for LSP settings
