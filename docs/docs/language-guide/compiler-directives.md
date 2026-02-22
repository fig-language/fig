---
sidebar_position: 6
---

# Fig Compiler Directives (Metadata)

Metadata in Fig provides **compiler directives** that convey additional information or instructions to the compiler. Metadata uses the ` @` symbol, followed by a lowercase identifier, similar to Java annotations.

```fig
 @inline
fn add(a: i32, b: i32) -> i32 {
    a + b
}

 @deprecated("Use add_v2 instead")
fn add_old(a: i32, b: i32) -> i32 {
    a + b
}
```

Common uses for metadata include:

*   Indicating inline expansion (` @inline`)
*   Deprecation warnings (` @deprecated`)
*   Optimization hints, documentation flags, or other compiler-specific instructions

Metadata appears **before the declaration** it applies to.

This document summarizes Fig's **compiler directives (metadata)**, providing syntax, examples, and usage notes for this feature.
