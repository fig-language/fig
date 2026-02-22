# Extension Methods

Fig's extension methods provide a powerful and flexible mechanism to **add new functions to existing types** without altering their original definitions. This allows for greater code organization, modularity, and the ability to enhance external types with custom functionality.

### What They Are

Extension methods allow you to attach functions to types—be it `struct`s, `union`s, or even built-in types—from an external context. They are defined within a dedicated **`ext TypeName` block**. This approach ensures that the original type definition remains clean and focused, while supplementary functionalities can be added as needed.

---

### Syntax Example

The basic syntax for defining extension methods involves the `ext` keyword followed by the type to be extended:

```fig
ext Array<T> {
    fn push(self: &mut Array<T>, value: T) {
        // implementation to add an element to the array
    }

    fn map<U>(self: Array<T>, f: fn(T) -> U) -> Array<U> {
        let out: Array<U> = Array<U>::new()
        for item in self { // Assuming 'for item in Array' is valid syntax for iteration
            out.push(f(item))
        }
        return out
    }
}
```

In this example, `push` and `map` are added as methods to the `Array<T>` type. Note that the `map` function itself is generic over `U`, the output type of the mapping function `f`.

---

### Key Characteristics

Fig extension methods possess several important characteristics that define their behavior and usage:

1. **Scope-local**
    Extension methods are strictly **scope-local**. They are only available within the scope where their `ext` block is visible or in modules that explicitly import them. This prevents unintended side effects and "polluting" the global namespace with methods.

2. **Methods are just functions**
    Internally, extension methods are treated by the compiler as **normal functions within their scope**. The special method call syntax is merely syntactic sugar. The **first parameter** of an extension method always acts as the receiver (often named `self`), determining the type the method operates on.

3. **Method call syntax**
    Despite being defined externally, functions within an `ext` block can be invoked using the familiar method call syntax, making them feel like native methods of the type:

    ```fig
    let arr: Array<i32> = Array::new()
    arr.push(42) // Calls the 'push' extension method
    let doubled = arr.map(lambda x -> x * 2) // Calls the 'map' extension method
    ```

4. **Works for built-in and user-defined types**
    Extension methods are versatile and can be defined for any type:
    * **Built-in types**: Standard library modules can provide extension methods for types like `T ! E` or `Array<T>`, enhancing their functionality.
    * **User-defined types**: You can define extensions for your own `struct`s or `union`s to add custom behavior.

5. **No changes to original type**
    A key design principle is that the original type definition remains entirely untouched. The `ext` block merely informs the compiler how to interpret method calls for that type in a specific context.

6. **Access to fields**
    Extension methods have controlled access to the fields of the extended type. They can only access **fields explicitly marked with the `ext` visibility modifier** (e.g., `ext(super)`). This maintains encapsulation by preventing arbitrary code from directly manipulating internal state while allowing controlled access from within authorized extension blocks.

---

### How It Works Under the Hood

When you write code like `instance.method(args)`, the Fig compiler performs a lookup. If `method` is not defined directly on the `instance`'s type, the compiler checks visible `ext TypeName` blocks for a function matching `method` whose first parameter's type matches `instance`'s type. If found, the compiler translates the method call into a direct function call, passing `instance` as the first argument.

This mechanism provides the **ergonomic benefits of method calls** (e.g., `.` syntax, chaining) without requiring the methods to be physically declared inside the type's definition, promoting a cleaner separation of concerns.

---

### Example with Controlled Visibility

The `ext` visibility modifier provides fine-grained control over which extension methods can access a type's internal state.

```fig
struct Counter {
    ext(super) value: i32   // 'value' is accessible only within this module and its extension blocks
}

ext Counter {
    // This 'ext' block is in the same module as 'Counter'
    fn increment(self) {
        self.value += 1  // Allowed: 'ext(super)' permits access from 'ext' in the same module
    }
}

// Imagine this code is in an entirely separate module that imports the module containing Counter
// import mymodule

ext Counter {
    fn cheat(self) {
        // self.value = 999  // ❌ Compiler error: 'value' field is not accessible here
    }
}
```

In this example, `Counter.value` is marked `ext(super)`, meaning it's only accessible from within the `Counter`'s defining module and `ext` blocks within that same module. An `ext` block in an external module would not have access to `value`, leading to a compile-time error. This ensures strong encapsulation while still enabling type-specific extensions.
