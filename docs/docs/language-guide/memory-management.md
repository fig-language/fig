---
sidebar_position: 9
---

# Nyx Memory Management

This document summarizes the memory management model in the Nyx programming language as of its bootstrapping stage. Nyx currently provides **manual memory management** without garbage collection or ownership systems.

---

## **1. No Garbage Collector**

* Nyx does **not include a garbage collector** in its bootstrapped version.
* All memory management is **manual**, and the programmer is responsible for allocating and deallocating memory correctly.
* There are **no hidden allocations**; every heap allocation requires explicit instruction.

---

## **2. Stack vs Heap Allocation**

*   **Stack allocation**:

    *   Automatic allocation for local variables and function frames.
    *   Memory is freed automatically when the variable goes out of scope.
    *   Suitable for fixed-size, short-lived data.

*   **Heap allocation**:

    *   Explicit allocation via standard library or language-provided functions.
    *   Programmer must specify the allocator.
    *   Memory persists beyond the current scope until explicitly freed.

```nyx
mut x: i32 = 10       // stack allocation
let buffer: *raw = allocator.allocate(1024)  // heap allocation
```

---

## **3. Manual Allocation**

*   Allocation must always specify an **allocator**; there is no default heap allocation.
*   Example syntax:

```nyx
let mem: *raw ! AllocError = my_allocator.allocate(size)
```

*   Deallocation must also be explicit:

```nyx
my_allocator.deallocate(mem)
```

*   Failure to deallocate memory results in memory leaks.

---

## **4. No Ownership or Borrowing System**

*   Nyx **does not track ownership** of allocated memory.
*   There is **no borrowing or lifetime system** like in Rust.
*   All pointers (`*T` or `*raw`) are **raw**; the programmer is fully responsible for correct usage.
*   Unsafe memory operations are allowed but must be handled carefully to avoid undefined behavior.

---

### **Summary**

*   Nyx uses **manual, explicit memory management**.
*   Stack and heap allocations are distinct; heap allocations always require an explicit allocator.
*   No garbage collector, no ownership, no borrowing: all memory safety is the responsibility of the programmer.
*   This approach provides maximum control and minimal runtime overhead, suitable for a low-level, high-performance language.

---

This document serves as a reference for Nyx's **memory management principles**, explaining stack vs heap, manual allocation, and the absence of ownership or garbage collection.
