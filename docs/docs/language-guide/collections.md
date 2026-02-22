---
sidebar_position: 5
---

# Fig Collections

This document provides an overview of array-based collections in the Fig programming language, focusing on syntax, indexing, iteration, and standard operations.

---

## **1. Arrays**

* Arrays are homogeneous, fixed-size or dynamically-sized, and use square brackets `[]` for literal construction.

```fig
let nums = [1, 2, 3, 4]
let mut data: [i32] = [10, 20, 30]
```

* Arrays can be indexed for reading and writing elements:

```fig
let first = nums[0]
mut data[1] = 50
```

* Arrays support iteration:

```fig
for n in nums {
    print(n)
}
```

* Standard library methods (to be implemented later) will provide operations like `push`, `pop`, `map`, `filter`, and more for mutable arrays.

---

## **2. Notes on Strings**

* Strings are **not a built-in type** in Fig; they will be provided by the standard library.
* A string is essentially a **type alias for a `u8` array**, with extension methods implemented in the standard library to provide string-specific operations.
* Any examples of strings should be understood as examples using this standard library type.

---

## **3. Notes**

* Arrays are the only built-in aggregate collection in Fig at the bootstrapping stage.
* Maps and dictionary-like structures will be implemented in the standard library.
* Collections support strong typing and safe access, with compile-time checks for indexing and iteration.
* Using `mut` allows modification of arrays where applicable.

---

This document summarizes **arrays and collection-like behavior in Fig**, providing developers with syntax, examples, and expected behavior for literals, indexing, iteration, and standard operations, while clarifying that strings are part of the standard library and built on arrays.
