# 22. Performance Standards

## Requirements

| Component | Metric | Target | Maximum |
|-----------|--------|--------|---------|
| Compiler | Compile time (small) | < 1s | 5s |
| Compiler | Compile time (large) | < 30s | 2min |
| Memory | Runtime memory | < 100MB | 500MB |
| Binary | Binary size | < 10MB | 50MB |
| GC | Pause time | < 1ms | 10ms |

## Premature Optimization

**DO NOT** optimize before:
1. You have profiled the code
2. You have identified a bottleneck
3. You have benchmarks to measure improvement

## Algorithm Complexity

Choose appropriate algorithms:

| Size | Acceptable Complexity |
|------|----------------------|
| Small (< 100) | O(n^2) acceptable |
| Medium (100-10K) | O(n log n) |
| Large (> 10K) | O(n) required |

## Memory Guidelines

- Prefer stack allocation over heap when possible
- Use appropriate data structures (Vec for sequential, HashMap for random access)
- Release resources promptly

## I/O Guidelines

- Batch I/O operations
- Use buffering for file operations
- Async I/O for concurrent operations

## Performance Testing

All performance-critical code **MUST** have benchmarks:

```xin
#[bench]
fn BenchmarkLexerLargeInput(Bencher: &mut Bencher) {
    let Input = generate_large_input(100_000);
    Bencher.iter(|| {
        let mut Lexer = Lexer.New();
        Lexer.Lex(Input)
    });
}
```

## Performance Regression

| Regression | Action |
|------------|--------|
| > 10% slower | Block merge |
| > 50% slower | Revert immediately |
| Memory increase > 20% | Block merge |
