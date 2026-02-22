---
sidebar_position: 2
---

# Fig Variables

Fig offers a clear and robust system for variable declaration and management, emphasizing strong typing and lexical scoping.

### **1. Declaration and Initialization**

*   Variables are declared using `let` by default.
*   For **mutable variables**, the `let` keyword can be **omitted** and `mut` alone suffices.

    ```fig
    let x: i32 = 42        // immutable variable with explicit type
    let y = 3.14           // type inferred as f64

    mut counter = 0        // mutable variable, 'let' omitted
    counter += 1
    ```

*   Variables can also be **declared without initialization**, but then a type must be provided:

    ```fig
    let z: String
    z = "hello"
    ```

---

### **2. Mutability**

*   By default, variables are **immutable**.
*   Mutable variables can be declared using `mut` (with or without `let`):

    ```fig
    let mut total = 10
    mut counter = 0
    counter += 1
    ```

*   Compile-time checks ensure safe mutation.

---

### **3. Shadowing**

*   You can **redeclare a variable with the same name** in the same scope, which **shadows the previous binding**:

    ```fig
    let x = 10
    let x = x + 5  // x is now 15
    ```

*   Shadowing allows **changing type** as well:

    ```fig
    let s = "123"
    let s = s.parse::<i32>()  // s is now i32
    ```

---

### **4. Destructuring**

*   Tuples, structs, and unions can be **destructured** into multiple variables:

    ```fig
    let (a, b) = (1, 2)
    let Point { x, y } = p
    ```

*   This is useful for extracting multiple values from a single object or return value.

---

### **5. Scoping**

*   Variables are scoped to the **block** in which they are declared:

    ```fig
    {
        let x = 10
        print(x)  // accessible here
    }
    // print(x)  // ❌ not accessible here
    ```

*   `if`, `for`, `while`, and other blocks introduce new scopes.

---

### **6. Constants**

*   Immutable compile-time constants are declared using `const`:

    ```fig
    const PI: f64 = 3.14159
    ```

*   Constants must always be **initialized at declaration**.

---

### **7. Notes**

*   Variables in Fig follow **strong typing** and **lexical scoping**.
*   **Mutability, shadowing, and destructuring** allow ergonomic and safe variable management.
*   The compiler enforces **initialization, type correctness, and access rules** at compile time.
*   Omitting `let` for mutable variables provides a concise, readable syntax.
