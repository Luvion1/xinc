# 9. Compile-Time Evaluation

## Const Functions

```xin
const fn Square(X: i32) -> i32 {
    X * X
}

const SIZE: i32 = Square(4);  // Evaluated at compile time
```

## Const Evaluation

```xin
const fn Factorial(N: u64) -> u64 {
    if N <= 1 { 1 } else { N * Factorial(N - 1) }
}

const FACT_10: u64 = Factorial(10);  // 3628800

let Arr: [i32; FACT_10] = [0; FACT_10];
```

## Compile-Time Refinement Checking

```xin
const fn MakeNat(X: i32) -> Nat {
    if X < 0 { panic!("Not natural"); }
    X
}

const VALID: Nat = MakeNat(5);   // OK
// const INVALID: Nat = MakeNat(-1); // Compile error
```

## Built-In Functions

| Function | Description |
|----------|-------------|
| `panic!("msg")` | Unconditional panic with message. Used in `const fn` for compile-time assertion. |
