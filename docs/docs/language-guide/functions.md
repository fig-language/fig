# Functions in Fig

This document outlines the syntax, generic capabilities, and principles behind functions in the Fig programming language, reflecting its core design philosophies of explicitness, readability, and modern style.

---

## 1. Function Syntax

Fig functions are straightforward to define, emphasizing clarity and type safety.

*   All functions are introduced by the `fn` keyword.
*   Fig requires explicit declaration of **generic type parameters** using the `[T, U, ...]` syntax; generic inference is not supported for function signatures.
*   Parameters intended for modification within the function must be explicitly marked with the `mutable` keyword.
*   Return types can be explicitly stated or, in obvious cases, inferred by the compiler. However, any generic return types must always be explicit.
*   Functions can return multiple values, providing a concise way to return related data.

### Basic Examples

```fig
// Simple function with explicit return type
fn add(a: Int, b: Int): Int {
    a + b
}

// Function with multiple return values
fn divide(a: Int, b: Int): (Int, Int) {
    (a / b, a % b)
}

// Mutable parameter
fn toggle(flag: mutable Bool) {
    flag = !flag
}
```

---

## 2. Generic Functions

Fig's generic functions promote code reusability without sacrificing type safety or runtime performance.

*   All generic functions must explicitly declare their type parameters. There is no generic inference in function signatures.
*   Generic parameters are listed within square brackets `[]` immediately following the function name.
*   Constraints on generic parameters are exclusively expressed using **interfaces**, offering a clear and composable mechanism for defining required capabilities. Fig does not use `type` or `where` clauses for constraints.
*   Const generics are fully supported and integrate naturally with type generics.

### Generic Function Examples

```fig
// Single generic
fn swap[T](a: T, b: T): (T, T) {
    (b, a)
}

// Multiple generics with constraints
interface NumericSerializable: Addable + Comparable + Serializable;
interface HashSerializable: Hashable + Serializable;
interface CloneEq: Clone + PartialEq;

fn merge_sorted[
    T: NumericSerializable,
    U: HashSerializable,
    V: CloneEq
](a: T, b: U, c: V) -> Result[T, V] {
    // ... function implementation ...
}
```

### Notes on Constraints

*   **Interfaces define constraints** that generic type parameters must satisfy. This means `T: MyInterface` indicates that `T` must implement `MyInterface`.
*   Multiple constraints for a single generic parameter are combined using the `+` syntax (e.g., `T: InterfaceA + InterfaceB`).
*   For functions with many generic parameters and complex constraints, using **multiline generic lists** significantly enhances readability.
*   The use of interfaces for constraint bundling often removes the need for type aliases, as the interfaces themselves serve as well-defined capability sets.

### Const Generics

Fig supports const generics in functions, enabling parameterization by compile-time constant values.

```fig
fn fill[T, N: usize](buf: [N]T, val: T) {
    for i in range(0, N) { // Note: 'range(start, end)' is used for for-loops
        buf[i] = val
    }
}
```
In the `fill` function, `N` is a compile-time constant of type `usize`, allowing the function to operate on arrays of a fixed, type-level known size. This integrates naturally with generic types (`T`).

---

## 3. Best Practices

Adhering to these best practices will help you write clear, maintainable, and idiomatic Fig functions:

*   **Explicit Generics**: Always explicitly declare all generic type parameters using the `[T, U, ...]` syntax for clarity and to prevent ambiguity.
*   **Multiline Generic Lists**: For functions with numerous generics or constraints, arrange the generic list across multiple lines to improve readability.
*   **Use Interfaces for Constraint Bundles**: Define interfaces to group related constraints. This simplifies function signatures and promotes reusability of constraint sets.
*   **Mutable Parameters**: Clearly indicate parameters that will be modified within the function by using the `mutable` keyword.
*   **Multiple Return Values**: Utilize Fig's support for multiple return values for functions that logically produce more than one result.

---

## 4. Summary

Fig functions are meticulously designed to be:

*   **Explicit**: All generic behavior is clearly declared, eliminating ambiguity.
*   **Modern and Readable**: With a clean syntax and minimal punctuation, functions are easy to understand.
*   **Flexible with Constraints**: Interfaces provide a powerful and composable mechanism for defining generic capabilities.
*   **Safe**: All generic interactions are statically checked, ensuring type safety.

This design philosophy ensures that whether you are writing a small utility function or a complex library component, Fig functions remain readable, maintainable, and easy to document.
