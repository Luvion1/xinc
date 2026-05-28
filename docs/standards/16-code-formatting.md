# 17. Code Formatting

## Indentation

- **Use spaces, not tabs**
- **4 spaces per indentation level**
- Configure editor to convert tabs to spaces

## Line Length

- **Maximum line length: 100 characters**
- Exception: URLs, long strings (but avoid when possible)

## Spacing

```xin
// Good
fn Add(A: i32, B: i32) -> i32 {
    A + B
}

struct Point {
    X: i32,
    Y: i32,
}

// Bad
fn Add(A:i32,B:i32)->i32{
    A+B
}
```

## Blank Lines

- **2 blank lines** between top-level definitions (functions, structs, etc.)
- **1 blank line** between logical sections within a function
- **No blank lines** between related statements

## Braces

```xin
// Good
if Condition {
    DoSomething();
} else {
    DoOther();
}

match Value {
    Some(X) => Process(X),
    None => Default,
}

// Bad
if Condition
{
    DoSomething();
}
else
{
    DoOther();
}
```

## Import Organization

Order imports alphabetically within groups:

```xin
// Good
use std.Collections.HashMap;
use std.Collections.HashSet;
use std.IO;

use InternalModule.Lexer;
use InternalModule.Parser;
```
