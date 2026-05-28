# 5. Language Syntax

## Functions

```xin
fn Greet(Name: String) {
    IO.Println(f"Hello, {Name}!");
}

fn Max(A: i32, B: i32) -> i32 {
    if A > B { return A; }
    B
}

fn DivideSafe(A: i32, B: { i32 | _ != 0 }) -> i32 {
    A / B
}

fn PositiveOrZero(X: i32) -> { i32 | _ >= 0 } {
    if X < 0 { 0 } else { X }
}
```

## Closures

```xin
let Add = |A: i32, B: i32| -> i32 { A + B };
let Increment = |X| X + 1;

let Factor = 10;
let Scaled = |X| X * Factor;
```

## Control Flow

### If-Else

```xin
if Condition {
    // ...
} else if Other {
    // ...
} else {
    // ...
}

let Max = if A > B { A } else { B };
```

### Match

```xin
match Value {
    Pattern1 => Expr,
    Pattern2 if Condition => Expr,
    _ => DefaultExpr,
}
```

### Loops

```xin
loop {
    if Done { break; }
}

while Count > 0 {
    Count -= 1;
}

    for Item in Items {
        IO.Println(f"{Item}");
    }

for I in 0..10 {    // Exclusive end
    // I: i32
}

for I in 0..=10 {   // Inclusive end
    // I: i32
}
```
