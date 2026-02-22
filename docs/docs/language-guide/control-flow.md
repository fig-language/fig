# Control Flow

Fig provides a familiar set of control flow constructs, enabling you to manage the execution path of your programs effectively. These constructs are designed with an ergonomic syntax, allowing the omission of curly braces (`{ }`) for single-statement blocks, which can lead to more concise code.

---

## 1. Conditional Statements

Conditional statements allow your program to make decisions based on certain conditions.

### `if / else`

The `if` statement executes a block of code if a specified condition is `true`. An optional `else` block can be provided to execute code when the condition is `false`.

```fig
if condition {
    // block executed if condition is true
} else {
    // block executed if condition is false
}
```

For **single-statement blocks**, Fig allows you to omit the curly braces, reducing boilerplate:

```fig
if condition
    do_something()
else
    do_something_else()
```

### `elseif / else if`

You can chain multiple `if` conditions using `else if` (or `elseif`) to handle several possible scenarios:

```fig
if condition1 {
    // executed if condition1 is true
} else if condition2 {
    // executed if condition1 is false AND condition2 is true
} else {
    // executed if all preceding conditions are false
}
```

The rule for single-statement blocks also applies to `else if` branches.

---

## 2. Loops

Loops allow you to repeatedly execute a block of code.

### `while loop`

The `while` loop continues to execute its block as long as a specified condition remains `true`.

```fig
while condition {
    // block executed as long as condition is true
}
```

And its single-statement version:

```fig
while condition
    do_something()
```

### `for loop`

The `for` loop is used for iterating over ranges or collections.

Iterating over a numerical range:

```fig
for i in 0..10 { // block executed for i = 0, 1, ..., 9
    print(i)
}
```

Iterating over elements in a collection:

```fig
for item in collection {
    process(item)
}
```

Single-statement version for `for` loops:

```fig
for i in 0..10
    print(i)
```

---

## 3. Loop Control Statements

Fig provides keywords to control the flow within loops:

* **`break`**: Immediately terminates the innermost loop and continues execution at the statement following the loop.
* **`continue`**: Skips the remainder of the current iteration of the innermost loop and proceeds to the next iteration.

```fig
for i in 0..10 {
    if i % 2 == 0 {
        continue // skip even numbers
    }
    print(i) // only odd numbers are printed
    if i == 7 {
        break // stop the loop when i reaches 7
    }
}
```

Both `break` and `continue` can also be used as single statements without curly braces within a loop's single-statement body.

---

## 4. `match` statement (pattern matching)

The `match` statement allows you to compare a value against a series of patterns and execute code based on the first matching pattern.

```fig
match value {
    0 => handle_zero(),
    1 => handle_one(),
    _ => handle_other() // '_' acts as a wildcard, matching any other value
}
```

`match` can also be used as an **expression** that returns a value based on the matched arm:

```fig
let status_text = match status_code {
    200 => "OK",
    404 => "Not Found",
    500 => "Internal Server Error",
    _   => "Unknown Status"
}
```

For single-statement arms in a `match` expression, the curly braces can be omitted.

---

## 5. Early Return

The `return` keyword allows you to exit a function at any point and optionally provide a return value.

```fig
fn foo(x: i32) -> i32 {
    if x < 0 {
        return 0 // Exit early if x is negative
    }
    x * 2 // Only reached if x is non-negative
}
```

Similar to other control flow constructs, a function body or an `if` branch that consists of a single `return` statement can omit curly braces.

---

## 6. Comments on Scope Omission

A defining characteristic of Fig's control flow is its flexibility regarding block delimiters. For any control flow statement—`if`, `else`, `for`, `while`, `loop`, or `match` arms—the surrounding curly braces `{ }` **can be omitted if the body consists of only a single statement**. This design choice aims to:

* **Reduce boilerplate**: Making code more concise, especially for simple operations.
* **Maintain readability**: While allowing conciseness, curly braces remain available for multi-statement blocks, ensuring clarity for more complex logic.

This balance allows developers to choose the style that best suits the complexity and readability needs of their code.
