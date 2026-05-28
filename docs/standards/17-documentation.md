# 18. Documentation Standards

## Philosophy

- **Explain WHY, not WHAT**
- Comments should add context that code cannot convey
- If you need a comment to understand the code, consider refactoring instead

## Coverage Requirements

| Element | Required | Format |
|---------|----------|--------|
| Public Functions | YES | Doc comment |
| Public Structs | YES | Doc comment |
| Public Traits | YES | Doc comment |
| Public Enums | YES | Doc comment |
| Complex Algorithms | YES | Doc comment + inline comments |
| Configuration | YES | README or config docs |
| Module Overview | YES | Module doc comment |
| Examples | YES | For all public APIs |

## Doc Comment Format

All public APIs **MUST** have documentation comments using `////` prefix:

```xin
//// Calculates the factorial of a non-negative integer.
////
//// # Arguments
//// * `N` - The input number (must be >= 0)
////
//// # Returns
//// The factorial result as a positive integer
////
//// # Errors
//// Returns `Err` if N is negative (checked via refinement type)
////
//// # Example
//// ```
//// let result = Math.Factorial(5);
//// assert_eq(result, 120);
//// ```
pub const fn Factorial(N: Nat) -> Positive {
    if N == 0 { 1 } else { N * Factorial(N - 1) }
}
```

## Inline Comments

Use sparingly and only for complex logic:

```xin
// Good
// Using bitwise shift for performance (equivalent to * 2)
let Shifted = Value << 1;

// Bad
// This is a for loop (explains WHAT, not WHY)
for I in 0..10 { }
```

## TODO Comments

Only with issue tracking:

```xin
// TODO(issue-123): Refactor to use more efficient algorithm
// TODO(issue-456): Add support for Unicode identifiers
```

## Prohibited Comments

```xin
// BAD - Will be rejected
// This function does X (WHAT, not WHY)
// HACK: Something that works but is wrong
// FIXME: Without issue reference
// NOTE: Unnecessary noise
```

## Documentation Verification

```bash
# Documentation must build without warnings
make docs

# Broken links must be fixed
make docs-check
```
