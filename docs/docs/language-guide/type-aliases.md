---
sidebar_position: 3
---

# Nyx Type Aliases

Nyx allows developers to define **type aliases** using the `type` keyword. Type aliases provide a new name for an existing type, improving code readability and maintainability. They can also be **generic**, making them flexible for use with multiple types.

---

### **1. Basic Type Aliases**

```nyx
// Simple alias for i32
type Age = i32
let my_age: Age = 30
```

* `Age` is now interchangeable with `i32` wherever a type is expected.

---

### **2. Generic Type Aliases**

* Type aliases can accept **type parameters in square brackets**, allowing reuse for different concrete types.

```nyx
// Alias for a pair of values
type Pair[T, U] = (T, U)

let p: Pair[i32, f64] = (10, 3.14)
```

* Generics make aliases flexible for collections, tuples, or custom types.

---

### **3. Using Type Aliases in Function Signatures**

```nyx
type StringMap[V] = Map[String, V]

fn get_value(map: StringMap[i32], key: String) -> i32 {
    map[key]
}
```

* This improves code readability by giving semantic meaning to commonly used types.

---

### **4. Notes and Best Practices**

* Type aliases **do not create new types**; they are purely **syntactic sugar**. The compiler treats them as the original type.
* Use type aliases to:

  * Improve readability
  * Reduce repetitive type signatures
  * Add semantic meaning to otherwise generic types
* Generic aliases can simplify working with parameterized types like collections, tuples, and other complex types.

---

This document summarizes the usage of **type aliases in Nyx**, including **generic type aliases using square brackets** and best practices for cleaner, more maintainable code.
