# Owned Generics and Nested Namespaces in Nyx

## Overview

Nyx introduces a unique namespace and generic system to solve common problems seen in languages like C++, Rust, and Zig. The core idea is that every type, function, interface, union, struct, or enum can belong to a namespace **and simultaneously have its own nested namespace**, allowing generics and members to be scoped hierarchically.

This design provides a clear distinction between **type generics** and **function generics**, avoids ambiguities in name resolution, and maintains explicit ownership of generics.

---

## Namespaces

* Namespaces are purely organizational; they do not impact runtime behavior.
* Any entity (struct, enum, interface, union, function) can define a nested namespace.
* Long paths are supported but can be shortened using a `using` statement to bring a namespace into scope.

**Example:**

```nyx
namespace Geometry:
    struct Vector2:
        x: f32
        y: f32

namespace Math:
    fn Vector2::add(a: Vector2, b: Vector2) -> Vector2:
        ...
```

---

## Type and Function Generics

* **Type generics** belong to the type’s own namespace.
* **Function generics** belong to the function’s namespace.
* This avoids the C++/Rust problem where type and function generics share the same scope, causing unintended name clashes or verbosity.

**Example:**

```nyx
struct Either[T, U]:

fn Either[T, U]::map[V](self: Either[T, U], f: fn(T) -> V) -> Either[V, U]:
    ...
```

* `T` and `U` are type-level generics owned by `Either`.
* `V` is a function-level generic owned by `map`.

---

## Implications of This Design

1. **Scoped Generics:** Each generic has a clearly defined scope, reducing ambiguity and preventing accidental shadowing.
2. **Readable Hierarchy:** Long paths can precisely indicate the ownership of types, functions, or interfaces.
3. **Explicit Using Statements:** Programmers can import namespaces selectively to reduce verbosity.
4. **Separate Generic Contexts:** Type generics and function generics do not collide or overlap.

---

## Example Usage

```nyx
namespace Collections:
    struct List[T]:
        items: [T]

    fn List[T]::append(self: List[T], value: T):
        self.items += value

namespace Option:
    union Option[T]:
        Some: T
        None

    fn Option[T]::map[U](self: Option[T], f: fn(T) -> U) -> Option[U]:
        match self:
            Some(value) => Some(f(value))
            None => None
```

* Here, `T` in `List` is different from `T` in `Option`.
* Each `map` function introduces its own function generic `U`.

---

## Comparison to Rust/C++

* **Rust:** Type and function generics share the same namespace within a type or trait, leading to potential shadowing and verbose annotations.
* **C++:** Templates can collide and cause unexpected compilation errors in nested or complex hierarchies.
* **Nyx:** Separates generics by ownership (type vs function), avoiding these pitfalls entirely.

---

## Advantages

* Clear separation of concerns between type-level and function-level generics.
* Avoids shadowing or ambiguous type resolution.
* Explicit and hierarchical scoping of all entities.
* Supports readable, maintainable, and predictable code organization.

---

## Notes

* Using statements reduce verbosity:

```nyx
using Geometry
fn Vector2::scale(self: Vector2, factor: f32):
    ...
```

* Nested namespaces are purely static; they do not introduce runtime overhead.
* Functions, structs, and interfaces within a namespace maintain their own independent generic parameters.
