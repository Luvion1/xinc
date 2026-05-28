# 14. Complete Examples

## Basic Program with Refinement Types

```xin
type Nat = { i32 | _ >= 0 }
type Positive = { i32 | _ > 0 }
type NonEmptyString = { String | _.Len() > 0 }
type Email = { String | _.Contains('@') && _.Contains('.') }
type Port = { u16 | _ > 0 && _ <= 65535 }

mod Math {
    pub const fn Factorial(N: Nat) -> Positive {
        if N == 0 { 1 } else { N * Factorial(N - 1) }
    }

    pub fn SafeDivide(A: i32, B: { i32 | _ != 0 }) -> i32 {
        A / B
    }

    pub fn Sqrt(X: { f64 | _ >= 0.0 }) -> f64 {
        X.Sqrt()
    }
}

fn Main() -> Result<(), AppError> {
    let X: Nat = 42;
    let Fact5 = Math.Factorial(5);
    IO.Println(f"5! = {Fact5}");

    Ok(())
}
```
