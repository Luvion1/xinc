# 3. Lexical Structure

## Source File Representation

- **File Extension**: `.xin`
- **Encoding**: UTF-8
- **Line Endings**: LF (Unix) or CRLF (Windows) - both supported

```
source_file → (item | stmt)* EOF
```

## Identifiers

```
identifier → (letter | '_') (letter | digit | '_')*
```

### Reserved Keywords

```
_, as, async, await, break, const, continue, crate, defer, else, enum, err, export,
extern, false, fn, for, if, impl, in, let, loop, match, mod, mut, pub, pub(crate),
pub(super), return, self, Self, static, struct, super, trait, true, type, use, while, with, yield
```

## Literals

### Integer Literals

```xin
42          // decimal
0xFF        // hexadecimal
0b1010      // binary
0o77        // octal
42u8        // u8 type suffix
42i64       // i64 type suffix
42usize     // usize type suffix
```

### Floating-Point Literals

```xin
3.14        // f64
3.14f32     // f32
1e10        // scientific notation
```

### Character and String Literals

```xin
'a'         // character
"hello"     // string
f"hello {name}"  // f-string interpolation
r"rawstr"   // raw string (no escape processing)
```

### Boolean Literals

```xin
true
false
```

## Operators

```
+   -   *   /   %   =   ==  !=  <   >   <=  >=
&&  ||  !   &   |   ^   ~   <<   >>  ++  --
+=  -=  *=  /=  %=  &=  |=  ^=  <<= >>=
?   ..  ..=  ... @   #   .   ->  =>  ::  ;  ??  ?.
```

| Operator | Description |
|----------|-------------|
| `?` | Error propagation (early return on `Err`) and optional unwrapping (early return on `None`) |
| `??` | Null coalescing - provides a default value when `Option` is `None` or `Result` is `Err` |
| `?.` | Optional chaining - short-circuits to `None` if the receiver is `None` |

## Whitespace and Comments

```xin
// Single-line comment

/* Multi-line
   comment */

//// Doc comment (attaches to next item)
```
