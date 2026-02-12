---
sidebar_position: 8
---

# Nyx Comments

Nyx supports **C-style comments**.

### **Single-line comments**

```nyx
// This is a single-line comment
let x = 10  // Inline comment
```

### **Multi-line comments**

```nyx
/* This is a
   multi-line comment */
let y = 20
```

Comments are ignored by the compiler and can be used to document code, explain logic, or temporarily disable code sections. Documentation-specific comments (like for generating docs) can be implemented using **special comment conventions**, e.g., `///` for docstrings, as defined in the standard library.

This document summarizes Nyx's **commenting mechanisms**, providing syntax, examples, and usage notes for this feature.
