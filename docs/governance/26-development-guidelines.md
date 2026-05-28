# 27. Development Guidelines

## General Principles

### SOLID Principles

| Principle | Application |
|-----------|-------------|
| **S**ingle Responsibility | Each type/function has one reason to change |
| **O**pen/Closed | Open for extension, closed for modification |
| **L**iskov Substitution | Subtypes are substitutable for base types |
| **I**nterface Segregation | Many specific interfaces > one general |
| **D**ependency Inversion | Depend on abstractions, not concretions |

### DRY (Don't Repeat Yourself)

- Every piece of knowledge must have a single, unambiguous representation
- Extract common patterns into functions, macros, or utilities
- Use inheritance or composition to avoid duplication

### YAGNI (You Aren't Gonna Need It)

- Do not add functionality "just in case"
- Implement what's needed now, refactor when requirements change
- Avoid speculative generalization

### KISS (Keep It Simple, Stupid)

- Simple solutions are preferred over complex ones
- Complexity should be justified by necessity, not assumed
- If you can't explain it simply, it's too complex

### Law of Demeter

- Only talk to immediate friends
- Don't talk to strangers (no chain calls like `A.GetB().GetC().DoSomething()`)

## Code Ethics

1. **Honesty**: Never misrepresent code functionality or hide defects.
2. **Respect**: Treat all contributors with respect. Critique code, not people.
3. **Accountability**: Take responsibility for your code.
4. **Collaboration**: Share knowledge freely. Help others understand the codebase.
5. **Excellence**: Strive for quality in every line of code.

## Ethical Code Writing

1. **Write for Humans First**: Code is read more often than written.
2. **No Hidden Agendas**: Do not introduce malicious code, backdoors, or functionality that serves only the author's interests.
3. **Preserve Privacy**: Do not log, store, or transmit sensitive information without explicit justification.
4. **Fair Representation**: Do not misrepresent the functionality, performance, or safety of your code.
5. **Inclusive Code**: Avoid language or patterns that exclude others. Use inclusive terminology.
