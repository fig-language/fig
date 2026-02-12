# Generics in Nyx

Nyx provides powerful generic types, allowing you to write flexible and reusable code that works across various types and compile-time constants. This system ensures type safety while minimizing code duplication.

## Generic Structs

Generic structs allow you to define data structures that can hold or operate on different types without sacrificing type checking.

### Type Parameters

A struct can be generic over one or more type parameters, denoted by `[T]` where `T` is a placeholder for a concrete type.

```nyx
struct Buffer[T] {
    data: [64]T
}
```
In this example, `Buffer[T]` can be instantiated with any type, such as `Buffer[u8]`, `Buffer[f32]`, or `Buffer[MyCustomStruct]`. The `data` field will then be an array of 64 elements of that specific type.

### Const Generics

Nyx also supports *const generics*, which allow you to parameterize structs with compile-time constant values, such as sizes or capacities.

```nyx
struct ArrayBuffer[T, N: usize] {
    data: [N]T
}
```
Here, `ArrayBuffer` takes both a type parameter `T` and a const parameter `N` of type `usize`. This allows for fixed-size arrays where the size is part of the type signature, e.g., `ArrayBuffer[u8, 128]`.

### First-Order Generics

All generics in Nyx are first-order. This means that type parameters represent concrete types, and there are no higher-kinded types. This simplifies the type system while still offering significant flexibility.

## Generic Functions

Functions can also be generic, allowing them to operate on a variety of types.

```nyx
fn identity[T](value: T) -> T {
    value
}
```
The `identity` function can take any type `T` as input and return a value of the same type `T`.

## Monomorphization

Nyx employs a process called *monomorphization* for generics. At compile time, the compiler generates a specific version of a generic struct or function for each unique set of type and const parameters it is instantiated with. This means that:

*   There is **zero runtime cost** associated with using generics.
*   The generated code is as efficient as if you had written it specifically for each concrete type.

This approach combines the flexibility of generics with the performance of static dispatch.
