# 6. Error Handling

## Result Type

Xin uses `Result<T, E>` for recoverable errors:

`Result<T, E>` is a sum type with two variants: `Ok(T)` for success and `Err(E)` for errors.

### Basic Usage

```xin
fn ReadFile(Path: String) -> Result<String, IOError> {
    let Content = FS.ReadFileToString(Path)?;
    Ok(Content)
}

match ReadFile("config.txt") {
    Ok(Contents) => IO.Println(Contents),
    Err(E) => IO.Eprintln(f"Failed: {E}"),
}
```

### Convenience Methods

```xin
Result.Unwrap()
Result.UnwrapOr(Default)
Result.Expect("Message")
Result.IsOk()
Result.IsErr()
Result.Map(|V| Transform(V))
Result.AndThen(|V| F(V))
Result ?? Default // Null coalescing for Result
```

For handling nullable values (non-error case), see `Option<T>`.

## Custom Error Types

```xin
enum AppError {
    IO(IOError),
    Parse(String),
    Validation { Field: String, Reason: String },
}

impl std.Fmt.Display for AppError {
    fn Format(Self, mut F: Fmt.Formatter) -> Fmt.Result {
        match Self {
            AppError.IO(E) => Write!(F, "IO error: {}", E),
            AppError.Parse(Msg) => Write!(F, "Parse error: {}", Msg),
            AppError.Validation { Field, Reason } =>
                Write!(F, "Validation error on {}: {}", Field, Reason),
        }
    }
}
```

## Refinement Type Validation Errors

```xin
fn TryIntoNat(X: i32) -> Result<Nat, String> {
    if X >= 0 { Ok(X) } else { Err(format!("{} is not natural", X)) }
}
```

## Error Handling Best Practices

Always use `Result<T, E>` for recoverable errors:

```xin
// Good
fn ParseInput(Input: String) -> Result<ParseResult, ParseError> {
    // ...
}

// Bad - Using exceptions or panic for recoverable errors
fn ParseInput(Input: String) -> ParseResult {
    // ...
}
```

Error messages should be specific and actionable:

```xin
// Good
Err(ParseError.InvalidNumber {
    Input: Input,
    Position: Position,
    Reason: "Expected digit but found letter".into(),
})

// Bad
Err("Parse error".into())
```

Use the `?` operator for clean error propagation:

```xin
fn ProcessData(Input: String) -> Result<Output, AppError> {
    let Parsed = ParseInput(Input)?;           // Propagate parse error
    let Validated = ValidateData(Parsed)?;     // Propagate validation error
    Ok(TransformData(Validated))               // Success
}
```

Only use `panic` in truly unrecoverable situations:

```xin
// Acceptable panic usage
fn UnwrapSome<T>(Opt: Option<T>) -> T {
    match Opt {
        Some(V) => V,
        None => panic!("Called UnwrapSome on None"),  // Logic error
    }
}

// Unacceptable
fn GetConfig() -> Config {
    // Don't panic for recoverable missing config
    Config.Load().Expect("Config must exist")  // Should return Result
}
```
