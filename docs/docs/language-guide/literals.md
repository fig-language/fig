---
sidebar_position: 4
---

# Fig Literals

This document provides an overview of the literal values available in the Fig programming language as of its bootstrapping stage. Literals allow developers to directly represent primitive values and aggregate structures in source code.

---

## **1. Numeric Literals**

### **Integers**

| Literal  | Description                                                    |
| -------- | -------------------------------------------------------------- |
| `42`     | Decimal integer literal (type inferred or explicitly declared) |
| `0b1010` | Binary integer literal                                         |
| `0o52`   | Octal integer literal                                          |
| `0x2A`   | Hexadecimal integer literal                                    |

* Integers can be assigned to any of the integer types (`i8..i64`, `u8..u64`).

### **Floating-point**

| Literal | Description                                                            |
| ------- | ---------------------------------------------------------------------- |
| `3.14`  | Standard floating-point literal (type inferred or explicitly declared) |
| `2.0e3` | Scientific notation for floats                                         |

* Floating-point literals default to `f64` unless specified otherwise.

---

## **2. Boolean Literals**

| Literal | Description         |
| ------- | ------------------- |
| `true`  | Boolean true value  |
| `false` | Boolean false value |

---

## **3. Character and String Literals**

### **Character**

| Literal | Description                                         |
| ------- | --------------------------------------------------- |
| `'a'`   | Single character literal                            |
| `'\n'`  | Escape sequences supported (`\n`, `\t`, `\\`, `\'`) |

### **String**

| Literal           | Description                              |
| ----------------- | ---------------------------------------- |
| `"Hello, World!"` | String literal enclosed in double quotes |
| `"Line1\nLine2"`  | Escape sequences supported in strings    |

* Strings are immutable by default. Mutable strings require a mutable variable.

---

## **4. Array Literals**

* Arrays can be constructed using square brackets `[]`.

```fig
let numbers = [1, 2, 3, 4]
let mixed: [i32] = [10, 20, 30]
```

* Arrays support type inference and can also explicitly declare the element type.

---

## **5. Unit Literal**

* The special `ok` literal represents the unit value for functions returning nothing:

```fig
fn do_nothing() -> ok {
    ok
}
```

* This is similar to `()` in Rust.

---

### **Notes**

* All literals correspond to the primitive or aggregate types defined in the language:

  * Integer and float literals map to `i*`/`u*` and `f32`/`f64` types.
  * Boolean literals map to `bool`.
  * `ok` is a unit type literal.
  * Arrays are aggregate types.
* Literals provide the foundation for expressions, initializations, and constant values in Fig.
* There are currently no tuples or map/dictionary literals in Fig's bootstrapped version; maps must be constructed using standard library functions or constructors.

---

This document summarizes all **literal types in Fig**, providing developers with the basic syntax and examples for numeric, boolean, character, string, array, and unit values.
