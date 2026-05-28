# 21. Security Standards

## Input Validation

- **Validate all external input**
- Use refinement types where possible
- Fail securely (default-deny)

```xin
// Good - Using refinement types
fn SetPort(Port: { u16 | _ > 0 && _ <= 65535 }) { }

// Good - Explicit validation
fn SetPort(Port: u16) -> Result<(), ValidationError> {
    if Port == 0 || Port > 65535 {
        return Err(ValidationError.InvalidPort);
    }
    // ...
}
```

## Sensitive Data

- Never log passwords, tokens, or secrets
- Use secure defaults
- Encrypt sensitive data at rest and in transit

## Resource Limits

- Implement timeouts for all I/O operations
- Limit memory usage where appropriate
- Handle denial-of-service scenarios

## Security Requirements

| Requirement | Severity | Verification |
|-------------|----------|--------------|
| No hardcoded secrets | Critical | Automated scan |
| Input validation | Critical | Manual review + tests |
| No SQL injection | Critical | Code review (if applicable) |
| No XSS vulnerabilities | Critical | Code review (if applicable) |
| No memory unsafe code | Critical | Compile-time analysis + runtime checks (bounds, no use-after-free) |
| No timing attacks | High | Crypto library usage |
| Secure defaults | High | Configuration review |

## Vulnerability Response

| Category | Example | Response Time |
|----------|---------|---------------|
| Critical (CVE) | Remote code execution | 24 hours |
| High | Privilege escalation | 7 days |
| Medium | Information disclosure | 30 days |
| Low | Unnecessary information | Next release |

## Prohibited Patterns

| Pattern | Vulnerability | Fix |
|---------|--------------|-----|
| `Format!("SELECT * FROM {}")` | SQL Injection | Parameterized queries |
| `Eval(UserInput)` | Code Injection | Parse, don't evaluate |
| `Http.Get(UserUrl)` without validation | SSRF | Validate URLs |
| `Password = UserInput` | Timing attack | Constant-time comparison |
| `Env.Get("SECRET")` | Secret leak | Use secrets management |
| `Deserialize(Untrusted)` | Deserialization | Use safe deserializers |

## Security Testing

```bash
make audit         # Check for vulnerabilities (dependency scanning)
make security      # Full security audit
make analyze       # Static analysis for unsafe patterns
```
