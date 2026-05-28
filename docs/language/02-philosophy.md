# 2. Language Philosophy

## Core Values

1. **Correctness Over Convenience**: The language prioritizes correctness over developer convenience. If code is ambiguous, the compiler rejects it.

2. **Explicit Over Implicit**: Everything is explicit. Nullability must be explicit via `Option<T>`, errors must be handled via `Result<T, E>`, and side effects must be marked.

3. **Safety Without Compromise**: Memory safety, type safety, and thread safety are non-negotiable. The language achieves these properties at compile time where possible, and at runtime where necessary.

4. **Performance Without Obscurity**: Low-level control is available when needed, but never at the expense of safety or maintainability.

## Design Principles

1. **Single Responsibility**: Every construct has a clear, focused purpose.
2. **Orthogonality**: Features are independent and compose without unexpected interactions.
3. **Predictability**: Behavior is consistent and intuitive across all contexts.
4. **Progressive Disclosure**: Simple use cases are simple; advanced use cases are possible without compromising simplicity.
