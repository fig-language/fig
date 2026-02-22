# Fig Purity & Effect System Specification

## Overview

Fig uses a **binary effect system** to separate pure computation from
side-effecting operations.

Functions are classified into two categories:

-   `fn` → Pure (no side effects)
-   `fn!` → Impure (may perform side effects)

This document defines the rules, guarantees, and implications of this
system.

------------------------------------------------------------------------

# 1. Function Categories

## 1.1 Pure Functions (`fn`)

A function declared with `fn` is guaranteed to:

-   Perform **no side effects**
-   Not mutate memory
-   Not allocate memory
-   Not perform I/O
-   Not call impure functions
-   Not call FFI functions
-   Not use any primitive marked as effectful

Pure functions **may read from memory via pointers**, but may not write.

Purity in Fig means:

> The function performs no side effects and does not mutate memory.

It does **not** mean referential transparency.

------------------------------------------------------------------------

## 1.2 Impure Functions (`fn!`)

A function declared with `fn!` may:

-   Write through pointers
-   Allocate memory
-   Perform I/O
-   Call FFI
-   Call other `fn!`
-   Use effectful primitives

Impure functions represent explicit effect boundaries.

------------------------------------------------------------------------

# 2. Pointer Rules

## 2.1 In `fn`

Allowed: - Reading through pointers (`*p`)

Forbidden: - Writing through pointers (`*p = value`) - Any intrinsic
memory write - Volatile writes - Atomic writes - Inline assembly that
mutates state

## 2.2 In `fn!`

All pointer operations are allowed.

------------------------------------------------------------------------

# 3. Allocation Rules

Allocation is considered a side effect.

-   Allocation primitives must be declared `fn!`
-   Pure functions cannot allocate
-   Pure functions cannot call allocation APIs

------------------------------------------------------------------------

# 4. FFI Rules

All `extern` functions must be declared `fn!`.

Pure functions: - Cannot declare extern functions - Cannot call extern
functions

This ensures unknown side effects cannot enter pure code.

------------------------------------------------------------------------

# 5. Global Variables

Fig only permits **constant globals**.

Mutable global state is not allowed.

This guarantees pure functions cannot observe implicit mutable global
state.

------------------------------------------------------------------------

# 6. Function Types

Function types encode purity at the type level.

-   `fn(T) -> U`
-   `fn!(T) -> U`

These are distinct and incompatible types.

Rules:

-   A pure function cannot accept a `fn!` parameter.
-   A pure function cannot call a `fn!`.
-   An impure function may call either.

This prevents effect leakage through higher-order functions.

------------------------------------------------------------------------

# 7. Method Semantics

Methods follow the same purity rules as functions.

Example:

    fn Vec.len(self: Vec) -> usize
    fn! Vec.push(self: *Vec[T], value: T)

Inside `fn`, calling `.push` is a compile-time error.

Inside `fn!`, both are allowed.

------------------------------------------------------------------------

# 8. Effect Propagation

Effects propagate upward through the call graph.

If a function:

-   Writes through a pointer
-   Allocates
-   Calls `fn!`
-   Calls FFI
-   Uses an effectful primitive

Then it must be declared `fn!`.

This creates an explicit mutation boundary in the program structure.

------------------------------------------------------------------------

# 9. Expression-Oriented Design

Fig remains expression-oriented:

-   Blocks evaluate to a value
-   Control flow constructs return values
-   Functions consist of a return expression

Pure combinator chains are encouraged:

    values.map(f).filter(g).sum()

Impure chains must occur inside `fn!`:

    vec.!push(x).!reserve(10)

------------------------------------------------------------------------

# 10. Guarantees Provided

This system guarantees:

-   No mutation can occur inside `fn`
-   Mutation cannot hide inside combinator chains
-   Side effects are syntactically visible
-   Higher-order functions cannot bypass purity
-   The compiler can aggressively optimize pure call graphs
-   Allocation and I/O are explicitly marked

------------------------------------------------------------------------

# 11. Non-Goals

Fig does NOT attempt:

-   Full referential transparency
-   Borrow checking
-   Ownership tracking
-   Concurrency safety (currently no concurrency model)

Pointer reads are allowed in pure functions for practicality in a
low-level systems language.

------------------------------------------------------------------------

# 12. Conceptual Model

Fig consists of two layers:

Layer 1: Pure computation graph (`fn`) Layer 2: Explicit effect layer
(`fn!`)

All mutation occurs in Layer 2. Layer 1 cannot perform mutation.

This keeps low-level flexibility while preserving strong reasoning
boundaries.

------------------------------------------------------------------------

# 13. Design Tradeoffs

Chosen: - Write-free purity - Explicit mutation boundary - Binary effect
model - Practical pointer-read allowance

Rejected: - Full referential transparency - Ownership/borrowing
complexity - Multi-effect tracking systems

------------------------------------------------------------------------

# 14. Summary

Fig implements a binary purity system where:

-   `fn` guarantees no side effects
-   `fn!` explicitly marks effectful code
-   Purity is enforced at compile time
-   Function types encode purity
-   Mutation is never implicit

This creates a clear, expression-oriented, manually managed systems
language with explicit effect boundaries and no hidden side effects.
