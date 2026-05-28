# 24. Code Review

## Self-Review Checklist

Before submitting code for review, verify:

- [ ] Code follows naming conventions (PascalCase)
- [ ] Code is formatted correctly (4 spaces, max 100 chars)
- [ ] All public APIs are documented with `////` doc comments
- [ ] Tests cover new functionality
- [ ] No TODO without issue reference
- [ ] No commented-out code
- [ ] No secrets or sensitive data
- [ ] Error handling is appropriate (Result<T, E> for recoverable)
- [ ] Code compiles without warnings

## Review Requirements

| Change Type | Reviewers | Approval |
|-------------|-----------|----------|
| Documentation | 1 | +1 |
| Bug fix | 1 | +1 |
| Feature | 2 | +2 |
| Security | 2 | +2 + Security lead |
| Architecture | 3 | +3 + Tech lead |

## Review Criteria

Reviewers should check:

1. **Correctness**: Does the code do what it claims?
2. **Design**: Does it fit the architecture?
3. **Readability**: Is it easy to understand?
4. **Testing**: Are tests adequate?
5. **Security**: Are there vulnerabilities?
6. **Performance**: Are there obvious issues?

## Response Time

| Review Type | Maximum Response Time |
|-------------|----------------------|
| Small PR (< 100 lines) | 24 hours |
| Medium PR (100-500 lines) | 48 hours |
| Large PR (> 500 lines) | 72 hours |

## Enforcement

| Severity | Description | Action |
|----------|-------------|--------|
| **Critical** | Security vulnerability, malicious code | Immediate rejection, possible ban |
| **High** | Major style violation, missing tests | Require fixes |
| **Medium** | Minor style issues, incomplete docs | Request fixes |
| **Low** | Suggestions, improvements | Optional |
