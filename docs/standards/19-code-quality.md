# 20. Code Quality Metrics

## Quality Dimensions

| Dimension | Description | Weight |
|-----------|-------------|--------|
| Correctness | Code does what it should | 30% |
| Test Coverage | Adequate testing | 20% |
| Documentation | Clear and complete docs | 15% |
| Security | Free of vulnerabilities | 20% |
| Performance | Meets performance targets | 10% |
| Maintainability | Easy to modify | 5% |

## Complexity Metrics

| Metric | Maximum | Critical If |
|--------|---------|-------------|
| Cyclomatic Complexity | 15 | > 25 |
| Cognitive Complexity | 20 | > 30 |
| Function Length | 50 lines | > 100 lines |
| File Length | 500 lines | > 1000 lines |
| Nesting Depth | 4 levels | > 6 levels |
| Parameters | 5 | > 7 |

## Maintainability Index

```
Maintainability Index = MAX(0, 171 - 5.2 * log(Halstead Volume)
                                - 0.23 * (Cyclomatic Complexity)
                                - 16.2 * log(Lines of Code)) * 100 / 171
```

| Rating | Index Range | Action |
|--------|-------------|--------|
| Excellent | 90-100 | None |
| Good | 80-89 | None |
| Fair | 70-79 | Consider refactoring |
| Poor | 60-69 | Refactor before merge |
| Very Poor | < 60 | Must refactor |

## Code Duplication

| Threshold | Action |
|-----------|--------|
| <= 2 lines | Ignore |
| 3-5 lines | Review |
| 6-10 lines | Refactor recommended |
| > 10 lines | Block merge until refactored |

## Anti-Patterns (Prohibited)

| Anti-Pattern | Reason |
|--------------|--------|
| Magic Numbers | Unmaintainable |
| God Functions | Violates SRP |
| Copy-Paste Code | DRY violation |
| Silent Failures | Obscures errors |
| Commented-Out Code | Technical debt |
| TODO Without Tracking | Technical debt |
| Premature Optimization | Wasted effort |
| Complex One-Liners | Unreadable |

## Static Analysis Requirements

All code must pass these checks:

```bash
make format:check     # Formatting check
make lint             # Linting (using xinc --lint)
make check            # Type checking
make audit            # Security audit (dependency scanning)
make analyze          # Full static analysis (complexity, duplication)
```
