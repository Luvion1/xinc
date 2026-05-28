# 4. Type System

## Primitive Types

| Type | Description | Size |
|------|-------------|------|
| `bool` | Boolean values (true, false) | 1 byte |
| `u8`, `u16`, `u32`, `u64`, `u128`, `usize` | Unsigned integers | 1-8 bytes |
| `i8`, `i16`, `i32`, `i64`, `i128`, `isize` | Signed integers | 1-8 bytes |
| `f32`, `f64` | IEEE 754 floating-point | 4, 8 bytes |
| `char` | Unicode scalar value | 4 bytes |
| `()` | Unit type (empty tuple) | 0 bytes |

## Composite Types

### Tuples

```xin
let Point: (i32, i32) = (10, 20);
let (X, Y) = Point;
```

### Arrays

```xin
let Arr: [i32; 3] = [1, 2, 3];
let First = Arr[0];  // Bounds checked at compile time when possible
```

### Slices

```xin
let Slice: []i32 = Arr[1..3];
```

## Null Safety

Xin does not have null. All nullable values must use `Option<T>`:

```xin
let Value: Option<i32> = Some(42);
let Missing: Option<i32> = None;

fn Process(Opt: Option<i32>) -> i32 {
    match Opt {
        Some(N) => N * 2,
        None => 0,
    }
}

let Result = Opt.UnwrapOr(0);
let Safe = Opt?;

let Value = Opt ?? 0;        // Null coalescing
let Len = Opt?.Len();        // Optional chaining
```

## Refinement Types

Refinement types are base types narrowed with predicates that must be satisfied by values.

### Basic Syntax

```xin
type Nat = { i32 | _ >= 0 }
type Positive = { i32 | _ > 0 }
type Even = { i32 | _ % 2 == 0 }
type NonEmptyString = { String | _.Len() > 0 }
type Email = { String | _.Contains('@') && _.Contains('.') }
type Percent = { i32 | _ >= 0 && _ <= 100 }
type Port = { u16 | _ > 0 && _ <= 65535 }
```

### Usage

```xin
let X: Nat = 10;        // OK
let Y: Nat = -5;        // Compile error: -5 is not >= 0

fn Divide(A: i32, B: { i32 | _ != 0 }) -> i32 {
    A / B  // Compiler knows B is non-zero
}

fn Sqrt(X: { f64 | _ >= 0.0 }) -> f64 {
    X.Sqrt()
}
```

### Compile-Time vs Runtime Checking

- **Compile-time**: When value is known (literals, const expressions)
- **Runtime**: When value is unknown (user input, external data)

```xin
fn GetInput() -> i32 {
    // Read from stdin
}

let X: Nat = GetInput();  // Runtime check: panics if negative

fn ToNat(X: i32) -> Option<Nat> {
    if X >= 0 { Some(X) } else { None }
}
```

### Refinement in Function Signatures

```xin
fn AgeInMonths(Age: { i32 | _ >= 0 && _ <= 150 }) -> i32 {
    Age * 12
}

fn SafeDivide(A: i32, B: i32) -> { i32 | _ <= 100 } {
    let Result = A / B;
    if Result > 100 { 100 } else { Result }
}
```

### Refinement in Structs and Enums

```xin
struct Config {
    Host: String,
    Port: { u16 | _ > 0 && _ <= 65535 },
    TimeoutMs: { u64 | _ >= 1000 },
}

enum Temperature {
    Celsius({ f64 | _ >= -273.15 }),
    Fahrenheit({ f64 | _ >= -459.67 }),
}
```

### Limitations

Current refinement types are lightweight:
- Predicates can only reference the value itself
- Supported expressions: simple arithmetic, comparisons, logic, simple method calls
- Future versions will support stronger dependent types

## Sum Types and Pattern Matching

### Enums

```xin
enum Result<T, E> {
    Ok(T),
    Err(E),
}

enum Option<T> {
    Some(T),
    None,
}

enum Color {
    Red,
    Green,
    Blue,
    RGB { R: u8, G: u8, B: u8 },
}
```

### Exhaustive Pattern Matching

All enum variants must be handled:

```xin
fn ProcessColor(Color: Color) -> String {
    match Color {
        Color.Red => "red",
        Color.Green => "green",
        Color.Blue => "blue",
        Color.RGB { .. } => "composite",
    }
}
```

## Structural Types

### Records (Structs)

```xin
struct Point {
    X: i32,
    Y: i32,
}

struct Config {
    Host: String,
    Port: u16,
    TimeoutMs: u64,
}

let P = Point { X: 10, Y: 20 };
let P2 = Point { ..P };  // Copy with modification
```

## Type Inference

Xin uses local type inference:

```xin
let X = 42;           // Infers i32
let Y = "hello";      // Infers String
let Z = Vec.New();    // Infers Vec<T> from usage
```
