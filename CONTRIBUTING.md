# Contributing to Xin Language

## Development Setup

```bash
# Clone repository
git clone https://github.com/Luvion1/xinc.git
cd xinc

# Build
cargo build --workspace

# Run tests
cargo test --workspace

# Format and lint
cargo fmt --all && cargo clippy --workspace -- -D warnings
```

## Code Standards

### File Structure
- Each file must be ≤200 SLOC
- Each folder ≤5 files
- Use 10+ level nesting for domain separation

### Documentation
- All public APIs require `///` doc comments
- Modules require `//!` doc comments
- Every public function must have examples

### Testing
- Unit tests in `#[cfg(test)]` module within source files
- Integration tests in `tests/` directory
- Test coverage goal: 90% for domain crate

### Error Handling
- Use `thiserror` for all error enums
- No `unwrap()`, `expect()`, or `panic!` in library code
- Return `Result<T, E>` for all fallible operations

## Pull Request Process

1. Run `cargo fmt --all` before commit
2. Ensure `cargo clippy --workspace -- -D warnings` passes
3. All tests must pass: `cargo test --workspace`
4. Update documentation if adding public APIs