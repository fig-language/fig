---
sidebar_position: 7
---

# Fig Casting / Type Conversion

Fig supports **explicit type conversion (casting)** using the familiar `(T)` syntax.

```fig
let x: i32 = 42
let y: f64 = (f64)x

let a: u8 = 255
let b: i16 = (i16)a
```

Notes:

*   Casting is explicit; the compiler will not implicitly convert between types.
*   Valid casts include numeric types (`i*`, `u*`, `f*`) and pointers (`*T`, `*raw`).
*   Unsafe casts are possible with pointers but require caution.

This document summarizes Fig's **casting and type conversion mechanisms**, providing syntax, examples, and usage notes for this feature.
