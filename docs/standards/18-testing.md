# 19. Testing Standards

## Philosophy

- Tests are **first-class citizens**, not an afterthought
- Test behavior, not implementation
- Write tests that give confidence in correctness

## Coverage Requirements

| Category | Minimum | Target |
|----------|---------|--------|
| Statement Coverage | 80% | 90% |
| Branch Coverage | 75% | 85% |
| Function Coverage | 100% | 100% |
| Critical Path | 100% | 100% |
| Public API | 100% | 100% |

## Test Categories

| Type | Purpose | When to Use |
|------|---------|-------------|
| Unit | Test single function/module | Always |
| Integration | Test component interactions | When combining modules |
| Property | Test mathematical properties | For complex algorithms |
| Regression | Prevent bug recurrence | For every bug fix |
| Performance | Benchmark critical paths | For optimization |

## Test Naming

```xin
// Good - Descriptive names
#[test]
fn TestParserHandlesNestedExpressions() { }

#[test]
fn TestFactorialOfZeroReturnsOne() { }

#[test]
fn TestConfigValidationRejectsInvalidPort() { }

// Bad - Vague names
#[test]
fn TestParse() { }
#[test]
fn Test1() { }
```

## Test Structure

Follow AAA pattern (Arrange, Act, Assert):

```xin
#[test]
fn TestParserHandlesEmptyInputGracefully() {
    // Arrange
    let Input = "";
    let Parser = Parser.New();

    // Act
    let Result = Parser.Parse(Input);

    // Assert
    assert!(Result.IsErr());
    match Result.Err() {
        Some(ParseError.UnexpectedEnd) => {},
        _ => panic!("Wrong error type"),
    }
}
```

## Test Performance

| Test Type | Maximum Execution Time |
|-----------|------------------------|
| Unit Test | 100ms |
| Integration Test | 1s |
| Property Test | 10s |
| Full Test Suite | 5 minutes |

## Flaky Tests

**STRICTLY PROHIBITED**:
- Tests that pass/fail non-deterministically
- Tests that depend on timing
- Tests that depend on external resources without mocks

**Mitigation**:
- Use deterministic seeds
- Mock external dependencies
- Use time-independent assertions
