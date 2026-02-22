# Interfaces in Fig

Fig's interface system allows you to define a contract of behavior that types can implement. This enables polymorphism and helps in designing modular and extensible code. Interfaces can also be generic, leveraging the generic type system discussed in the [Generics documentation](generics.md).

## Defining Interfaces

Interfaces in Fig define a set of methods that implementing types must provide.

### Generic Interfaces

Interfaces can be generic over types and const parameters, similar to structs.

```fig
interface Writer[T] {
    fn write(self, data: *T) -> usize
}

interface FixedWriter[T, N: usize] {
    fn write(self, data: [N]T) -> usize
}
```

In these examples, `Writer[T]` requires an implementing type to provide a `write` method that works with a reference to `T`. `FixedWriter[T, N: usize]` extends this by also taking a const parameter `N`, allowing for interfaces that operate on fixed-size collections.

When using a generic interface, its parameters must be *monomorphized* (given concrete types or constants) at the point of use.

### Interface Inheritance / Composition

Interfaces can declare that any type implementing them must also implement one or more other interfaces. This is achieved using the `+` syntax in the interface definition.

```fig
interface Example[T]: Writer[T] + Serializer[T] {
    fn example_method(self, data: *T)
}
```

A type implementing `Example[T]` is automatically required by the compiler to also implement both `Writer[T]` and `Serializer[T]`. This promotes code reuse and ensures a consistent set of capabilities.

## Implementing Interfaces

Fig uses explicit, nominal interface implementation. All methods are defined directly within the struct, and the struct must explicitly declare which interfaces it implements.

### Upfront Declaration (in struct)

You can declare that a struct implements one or more interfaces directly in its definition using the `:` and `+` syntax:

```fig
interface Writer[T] { fn write(self, data: *T) -> usize }
interface Serializer[T] { fn serialize(self, data: *T) -> []u8 }

struct Buffer[T]: Writer[T] + Serializer[T] {
    data: [64]T

    fn write(self, data: *T) -> usize { /* ... */ }
    fn serialize(self, data: *T) -> []u8 { /* ... */ }
}
```

Here, `Buffer[T]` declares that it implements both `Writer[T]` and `Serializer[T]`. The compiler immediately checks that `Buffer[T]` provides all the methods required by both interfaces with the correct signatures. This works seamlessly with both type and const generics.

### Retroactive Implementation

Interfaces can also be implemented for a struct after the struct's initial definition. This is done using the `impl` keyword:

```fig
interface Logger { fn log(self, message: []u8) }

impl Buffer[T]: Logger;
```

Retroactive implementation is particularly useful in several scenarios:

* When defining an interface for a struct that is part of a foreign module or library you don't control.
* When adding new interfaces to an existing codebase without modifying every struct definition.
* For providing a specific implementation for a particular generic instantiation (e.g., `impl Buffer[u8]: Logger`).

The compiler verifies that all required methods are implemented with matching signatures, mapping any generic parameters as needed.

### Generic Structs Implementing Generic Interfaces

When a generic struct implements a generic interface, Fig provides flexibility in how generic parameters are handled.

#### Matching Generic Parameters

The simplest case is when the struct and interface share the same generic parameters.

```fig
interface Writer[T] { fn write(self, data: *T) -> usize }
struct Buffer[T] { fn write(self, data: *T) -> usize { /* ... */ } }
impl Buffer[T]: Writer[T]
```

The implementation of `Writer[T]` for `Buffer[T]` means that `Buffer[u8]` implements `Writer[u8]`, `Buffer[f32]` implements `Writer[f32]`, and so on. Monomorphization occurs for each specific type instantiation.

#### Different Number of Parameters

A struct with more generic parameters can implement an interface with fewer, by mapping the struct's parameters to the interface's.

```fig
interface Writer[T] { fn write(self, data: *T) -> usize }
struct Buffer[K, V] { fn write(self, data: *K) -> usize { /* ... */ } }
impl Buffer[K, V]: Writer[K]
```

Here, `Buffer[K, V]` implements `Writer[K]`, effectively making the `V` parameter irrelevant to the `Writer` interface.

#### Interface with More Parameters than Struct

A struct can implement an interface that has more generic parameters by fixing some of the interface's parameters during implementation.

```fig
interface Mapper[K, V] { fn map(self, key: *K, value: *V) }
struct SimpleMap[T] { fn map(self, key: *T, value: *T) { /* ... */ } }
impl SimpleMap[T]: Mapper[T, T]
```

`SimpleMap[T]` implements `Mapper[T, T]`, meaning it only maps keys and values of the same type `T`.

### Const Generics in Interfaces

Const generics can also be involved in interface implementations.

```fig
interface FixedWriter[T, N: usize] { fn write(self, data: [N]T) -> usize }
struct ArrayBuffer[T, M: usize] { fn write(self, data: [M]T) -> usize { /* ... */ } }
impl ArrayBuffer[T, M]: FixedWriter[T, M]
```

In this example, the `M` const parameter from `ArrayBuffer` is mapped to the `N` const parameter of `FixedWriter`. If an interface's const parameter is not dependent on the implementing struct's const parameters, it can be fixed to a specific value during implementation.

## Rules & Principles for Fig Interfaces

Fig's interface system adheres to a clear set of rules to ensure predictability and maintainability:

1. **Generics from Struct/Interface:** All generic parameters used in an `impl` block must originate from the struct or interface being implemented. You cannot have independent generic `impl`s.
2. **Explicit Parameter Mapping:** When the number or names of generic parameters differ between a struct and the interface it implements, an explicit mapping is required (e.g., `impl MyStruct[A, B]: MyInterface[B]`).
3. **No Blanket Implementations:** Fig does not support blanket `impl`s (e.g., `impl<T> MyInterface for T where T: AnotherInterface`). An interface can only be implemented for types you own or for interfaces you define.
4. **Multiple Interfaces with `+`:** Multiple interfaces can be declared upfront in a struct definition using the `+` syntax.
5. **Retroactive Implementation:** Implementing an interface for an existing struct using `impl` is allowed, providing flexibility.
6. **Compiler Monomorphization:** All generics, including those in interfaces, are monomorphized by the compiler. This ensures zero runtime cost for interface dispatch; method calls are direct.
7. **Methods in Structs:** All methods for an interface are defined directly within the implementing struct. The compiler then ensures that the struct's methods satisfy the signatures required by the declared interfaces.

## Summary

Fig’s generic and interface system is designed with a focus on:

* **Explicitness:** Clear `impl` declarations and `struct: interface` statements.
* **Minimalism:** Avoids complex features like blanket or independent generic `impl`s to keep the system straightforward.
* **Monomorphization:** Guarantees zero-cost abstractions by resolving all generics at compile time.
* **Flexibility:** Supports multiple interface implementations with a concise `+` syntax and allows for retroactive implementation.
* **Readability:** Aims for intuitive syntax that combines upfront declarations with optional later `impl`s for clarity and ease of understanding.
