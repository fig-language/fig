# Data Types

This document provides an overview of all built-in data types available in the Fig programming language as of its bootstrapping stage. Fig offers a rich set of types, from fundamental primitives to powerful aggregate structures for complex data representation and robust error handling.

---

## Primitive Types

Primitive types are the most basic data types, directly representing fundamental values.

| Type                      | Description                                 | Notes                                  |
| :------------------------ | :------------------------------------------ | :------------------------------------- |
| ``u8``, ``u16``, ``u32``, ``u64`` | Unsigned integers                           | Standard fixed-width integer types     |
| ``i8``, ``i16``, ``i32``, ``i64`` | Signed integers                             | Standard fixed-width integer types     |
| ``f32``, ``f64``              | Floating-point numbers                      | Single and double precision            |
| ``bool``                    | Boolean                                     | ``true`` or ``false``                  |
| ``ok``                      | Function completion / void type             | Used for functions that return nothing |
| ``*T``                      | Typed pointer                               | Pointer to a value of type ``T``       |
| ``*raw``                    | Raw untyped pointer                         | Pointer to raw memory (``u8``)         |

---

## Aggregate Types

Aggregate types allow you to combine multiple values into a single, more complex structure.

| Type         | Description                                     | Notes                                                                                             |
| :----------- | :---------------------------------------------- | :------------------------------------------------------------------------------------------------ |
| ``struct``     | Aggregates multiple fields into a single object | Fields can be any type, including pointers or unions. Structs can also have functions defined within them.                                             |
| ``union``      | Compiler-enforced tagged union (safe union)     | Only one variant is active at a time; hidden tag managed by compiler. (Note: A dedicated page will cover unions in more detail.) |
| ``T ! E``      | Built-in two-variant union for error handling   | Holds either a value of type ``T`` or an error of type ``E``; safe access via pattern matching or helper methods |

---

## Special Notes on ``union``

Fig's ``union`` type is a powerful feature for memory-efficient data representation where only one of several possible variants is active at any given time. The compiler enforces safe access to ensure type safety.

*   **Syntax**:
    ```text
    union Either<T, U> {
        a: T,
        b: U
    }
    ```
*   Only one member of a union is active at a time.
*   **Memory Layout**: The memory occupied by a union is typically `max(sizeof(T), sizeof(U)) + hidden tag`. The "hidden tag" is managed by the compiler to track which variant is currently active.
*   **Safe Access**: Accessing the active variant of a union is strictly enforced by the compiler to prevent undefined behavior. This is primarily done through:
    *   Pattern matching:
        ```fig
        match x {
            a val => { /* handle T */ },
            b val => { /* handle U */ }
        }
        ```
*   Direct access to union fields outside of compiler-enforced safe mechanisms (like pattern matching) results in a **compile-time error**.
*   A more comprehensive guide on `union` types, including advanced usage and patterns, will be provided in a dedicated documentation page.

---

## Special Notes on ``T ! E``

The ``T ! E`` type is a fundamental built-in mechanism for robust error handling in Fig, representing a result that is either a successful value of type ``T`` or an error of type ``E``. It is a compiler-managed two-variant tagged union.

*   **Syntax**:
    ```fig
    fn allocate(size: u32) -> *raw ! AllocError {
        // ... function implementation ...
    }
    ```
*   **Safe Access**: Like `union`, `T ! E` guarantees safe access to its variants through:
    *   Pattern matching (similar to `union`).
    *   Standard library helper methods, such as:
        *   ``map(f: fn(T)->U) -> U ! E``: Transforms the successful value.
        *   ``map_error(f: fn(E)->F) -> T ! F``: Transforms the error value.
        *   ``unwrap_or(default: T) -> T``: Provides a default value if an error occurs.
*   The compiler seamlessly handles the **hidden tag and memory layout** for ``T ! E``, abstracting away the complexity from the programmer. You never need to manipulate the tag manually.

---

## Summary Table of All Types

| Type         | Kind        | Notes                                          |
| :----------- | :---------- | :--------------------------------------------- |
| ``u8``..``u64``  | Primitive   | Unsigned integers                              |
| ``i8``..``i64``  | Primitive   | Signed integers                                |
| ``f32``, ``f64`` | Primitive   | Floating point                                 |
| ``bool``       | Primitive   | Boolean                                        |
| ``ok``         | Primitive   | Function returns nothing                       |
| ``*T``         | Pointer     | Typed pointer                                  |
| ``*raw``       | Pointer     | Raw memory pointer                             |
| ``struct``     | Aggregate   | Multiple fields, can have functions            |
| ``union``      | Aggregate   | Single-active variant, compiler-enforced       |
| ``T ! E``      | Built-in union | Value or error, compiler-enforced           |

---

**Important Notes:**

*   All raw and typed pointers in Fig are considered unsafe and require explicit memory management by the programmer.
*   The ``union`` type and the ``T ! E`` built-in error handling type are the only compiler-managed tagged types available in the bootstrapped version of the language.
*   Future iterations of Fig may introduce more advanced features, such as multi-variant enums and a dedicated syntax for struct methods (e.g., ``impl`` blocks), further enhancing the language's expressiveness.
