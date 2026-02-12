# Union Types

A `union` in Nyx is a powerful construct for memory-efficient data representation and type-safe polymorphism. It is a **compiler-managed, safe tagged union** that allows a value to hold **exactly one of multiple possible types** at a time, with the compiler enforcing safety and memory correctness.

### Overview

The primary purpose of a `union` is to store different types of data in the same memory location, but never simultaneously. This means a union instance can represent various states or data forms, but at any given moment, only one of its defined variants is active.

### Syntax

A `union` is defined using the `union` keyword, followed by its name (which can be generic) and a list of its variants:

```nyx
union Either<T, U> {
    a: T,
    b: U
}
```

*   `T` and `U` can be any valid Nyx types, including other structs, pointers, or even other unions.
*   Critically, only **one variant is active at any time**. The compiler keeps track of which variant is currently stored.

### Memory Layout

The memory footprint of a `union` is optimized for space efficiency:

```text
sizeof(union) = max(sizeof(T), sizeof(U)) + 1 byte for hidden tag
```

*   The union allocates enough memory to accommodate its largest variant.
*   A **hidden tag** (typically 1 byte) is automatically managed by the compiler. This tag stores information about which variant is currently active, but it is **not directly exposed to the programmer**.

### Usage and Access

Nyx strictly enforces safe access to `union` variants to prevent undefined behavior.

*   **Construction**: Instances of a `union` can only be constructed via their variant constructors:

    ```nyx
    let x = Either::a(some_value_of_T); // x now holds a T
    let y = Either::b(some_value_of_U); // y now holds a U
    ```

*   **Safe Access via Pattern Matching**: The primary and safest way to access the value within a `union` is through pattern matching. This allows you to handle each possible variant explicitly:

    ```nyx
    match x {
        a val => println("The active variant is 'a' with value: {}", val),
        b val => println("The active variant is 'b' with value: {}", val)
    }
    ```

*   **Direct Field Access Forbidden**: Attempting to directly access a field of a `union` without using a safe mechanism like pattern matching is a **compile-time error**. This strict rule ensures that you cannot accidentally read from an inactive variant.

### Benefits

*   **Prevents Undefined Behavior**: By preventing direct access to inactive variants, Nyx's unions eliminate a common source of bugs and security vulnerabilities found in C-style unions.
*   **Encodes Safety in the Type System**: The safety guarantees are built directly into the language's type system, not relying on runtime checks or programmer discipline.
*   **Memory Efficiency**: The single-allocation memory layout makes unions suitable for low-level programming where memory footprint is a critical concern.
*   **Clear State Representation**: Unions provide an explicit way to model data that can be in one of several distinct states.
