# `T ! E` for Error Handling

The `T ! E` type in Nyx is a **special built-in two-variant union** designed specifically for **error handling**. It elegantly represents the outcome of an operation that can either succeed with a value of type `T` or fail with an error of type `E`. This mechanism provides compiler-enforced error propagation and handling, promoting robust and reliable software.

### Overview

`T ! E` simplifies error management by explicitly stating that a function's return may be either a successful result or an error. It removes the need for traditional exception handling or manual error code checks, integrating error flow directly into the type system.

### Syntax

Functions that can fail will declare `T ! E` as their return type. The `!` symbol serves as a clear separator between the success type and the error type.

```nyx
fn allocate(size: u32) -> *raw ! AllocError {
    // ... function implementation that might return a raw pointer or an AllocError ...
}
```

*   A function returning `T ! E` explicitly signals to the caller that it might not produce a `T` but an `E` instead.
*   The programmer is not required to handle any manual tag management; the compiler handles this implicitly.

### Access and Usage

Similar to general `union` types, Nyx ensures safe access to the variants of `T ! E`.

*   **Pattern Matching**: The most explicit way to handle a `T ! E` result is through pattern matching, which forces the programmer to consider both success and error cases:

    ```nyx
    let result = allocate(1024);
    match result {
        ok ptr => {
            // Success: 'ptr' is of type *raw
            use(ptr);
        },
        err e => {
            // Error: 'e' is of type AllocError
            handle_error(e);
        }
    }
    ```

*   **Helper Functions**: The standard library provides ergonomic helper functions to streamline common operations on `T ! E` types, allowing for a more functional style of error handling:

    *   ``result.map(f: fn(T)->U) -> U ! E``: Applies function `f` to the successful value `T` if it exists, transforming it into `U ! E`. If the result is an error, it propagates the error without calling `f`.
    *   ``result.map_error(f: fn(E)->F) -> T ! F``: Applies function `f` to the error value `E` if it exists, transforming it into `T ! F`. If the result is a success, it propagates the success value without calling `f`.
    *   ``result.unwrap_or(default: T) -> T``: Returns the successful value `T` if present, otherwise returns a specified `default` value. This should be used with caution as it discards the error information.

*   **Memory Layout**: The memory layout for `T ! E` is analogous to a general `union`, requiring storage for the largest of `T` or `E`, plus a hidden tag managed by the compiler.

### Benefits

*   **Compiler-Enforced Error Handling**: `T ! E` makes error handling an explicit part of the function signature, ensuring that potential errors are addressed at compile time.
*   **Avoids Runtime Checks with Helpers**: When using helper functions like `map` or `map_error`, the active variant is handled efficiently without explicit runtime checks by the programmer.
*   **Integrates with Low-Level Model**: `T ! E` fits naturally with Nyx's philosophy of providing control over memory and types, offering robust error handling without sacrificing low-level performance or predictability.

---

### Summary Table

Both `union` and `T ! E` types are fundamentally tagged unions designed to manage mutually exclusive data variants safely and efficiently.

| Type    | Purpose                         | Access Methods                                              | Safety                        |
| :------ | :------------------------------ | :---------------------------------------------------------- | :---------------------------- |
| `union` | Holds one of several variants   | `match`                                                     | Compiler-enforced, hidden tag |
| `T ! E` | Error handling (value or error) | `match`, helper functions (`map`, `map_error`, `unwrap_or`) | Compiler-enforced, hidden tag |

**Note**: Both types are essentially **tagged unions under the hood** with **single allocation for the largest variant** and **hidden tag management** by the compiler for safe access.
