# Fig Operators

Fig provides a rich and intuitive set of operators, mirroring those found in many modern programming languages. A core design principle in Fig is that operators are essentially **syntactic sugar** for calling methods defined on specific interfaces that the operands must implement. This elegant approach enables flexible, type-safe operator overloading, allowing users to define how operators behave for their custom types.

---

## 1. Arithmetic Operators

These operators perform mathematical calculations.

| Operator | Meaning        | Example                                                | Interface Method Call          |
| :------- | :------------- | :----------------------------------------------------- | :----------------------------- |
| ``+``      | Addition       | ``a + b``                                              | ``a.add(b)``                   |
| ``-``      | Subtraction    | ``a - b``                                              | ``a.sub(b)``                   |
| ``*``      | Multiplication | ``a * b``                                              | ``a.mul(b)``                   |
| ``/``      | Division       | ``a / b``                                              | ``a.div(b)``                   |
| ``%``      | Modulo         | ``a % b``                                              | ``a.rem(b)``                   |
| ``-a``     | Negation       | ``-a``                                                 | ``a.neg()``                    |

---

## 2. Comparison Operators

Comparison operators are used to compare two values and return a boolean result.

| Operator | Meaning               | Example                          | Interface Method Call          |
| :------- | :-------------------- | :------------------------------- | :----------------------------- |
| ``==``     | Equality              | ``a == b``                       | ``a.eq(b)``                    |
| ``!=``     | Inequality            | ``a != b``                       | ``a.ne(b)``                    |
| ``<``      | Less than             | ``a < b``                        | ``a.lt(b)``                    |
| ``>``      | Greater than          | ``a > b``                        | ``a.gt(b)``                    |
| ``<=``     | Less than or equal    | ``a <= b``                       | ``a.le(b)``                    |
| ``>=``     | Greater than or equal | ``a >= b``                       | ``a.ge(b)``                    |

---

## 3. Logical Operators

Logical operators are used with boolean values and typically result in a boolean value.

| Operator | Meaning     | Example     | Notes                                            |
| :------- | :---------- | :---------- | :----------------------------------------------- |
| ``&&``     | Logical AND | ``a && b``  | Short-circuits; operands must implement ``Bool`` interface. |
| ``||``     | Logical OR  | ``a || b``  | Short-circuits; operands must implement ``Bool`` interface. |
| ``!``      | Logical NOT | ``!a``      | Operand must implement ``Bool`` interface.       |

---

## 4. Bitwise Operators

Bitwise operators perform operations on the individual bits of integer types.

| Operator | Meaning     | Example     | Interface Method Call          |
| :------- | :---------- | :---------- | :----------------------------- |
| ``&``      | Bitwise AND | ``a & b``   | ``a.bit_and(b)``               |
| ``|``      | Bitwise OR  | ``a | b``   | ``a.bit_or(b)``                |
| ``^``      | Bitwise XOR | ``a ^ b``   | ``a.bit_xor(b)``               |
| ``~``      | Bitwise NOT | ``~a``      | ``a.not()``                    |
| ``<<``     | Left shift  | ``a << b``  | ``a.shl(b)``                   |
| ``>>``     | Right shift | ``a >> b``  | ``a.shr(b)``                   |

---

## 5. Assignment Operators

Assignment operators are used to assign values to variables. Compound assignment operators combine an operation with an assignment.

| Operator | Meaning               | Example                      | Equivalent Operation   |
| :------- | :-------------------- | :--------------------------- | :--------------------- |
| ``=``      | Assignment            | ``a = b``                    |                        |
| ``+=``     | Add-assign            | ``a += b``                   | ``a = a + b``          |
| ``-=``     | Subtract-assign       | ``a -= b``                   | ``a = a - b``          |
| ``*=``     | Multiply-assign       | ``a *= b``                   | ``a = a * b``          |
| ``/=``     | Divide-assign         | ``a /= b``                   | ``a = a / b``          |
| ``%=``     | Modulo-assign         | ``a %= b``                   | ``a = a % b``          |
| ``&=``     | Bitwise AND-assign    | ``a &= b``                   | ``a = a & b``          |
| ``|=``     | Bitwise OR-assign     | ``a |= b``                   | ``a = a | b``          |
| ``^=``     | Bitwise XOR-assign    | ``a ^= b``                   | ``a = a ^ b``          |
| ``<<=``    | Left shift-assign     | ``a <<= b``                  | ``a = a << b``         |
| ``>>=``    | Right shift-assign    | ``a >>= b``                  | ``a = a >> b``         |

---

## 6. Other Operators

Fig includes several other operators for common programming tasks.

| Operator  | Meaning                       | Example                   |
| :-------- | :---------------------------- | :------------------------ |
| ``.``       | Member access                 | ``obj.field``             |
| ``[]``      | Index access                  | ``arr[i]``                |
| ``()``      | Function call                 | ``f(x, y)``               |
| ``,``      | Sequence / tuple construction | ``let t = (x, y)``        |
| ``->``      | Function return type          | ``fn foo() -> i32 { ... }`` |

---

## 7. Ternary Conditional Operator

Fig offers a concise ternary conditional operator for expressing conditional logic within an expression.

*   Fig uses the syntax ``value_if_true if condition else value_if_false``.
*   For single-statement expressions, curly braces can be omitted:

    ```fig
    let a = 10 if condition else 20
    ```

*   Multi-statement blocks can be included using curly braces, with each branch needing to resolve to a value of the same type:

    ```fig
    let result = action1() if condition {
        // ... some side effect or complex logic ...
        side_effect()
        value_for_true_case()
    } else {
        // ... another side effect or complex logic ...
        alternative()
        value_for_false_case()
    }
    ```

---

## 8. Notes on Operator Implementation

The underlying mechanism for all operators in Fig is a powerful feature:

*   Every operator in Fig is **syntactic sugar for calling a method defined by an interface** on the operand(s). This is what enables Fig's flexible and type-safe approach to operator overloading.
*   For example, the addition operator `+` for two values `a` and `b` conceptually translates to a method call `a.add(b)`. For this to be valid, the type of `a` must implement the `Add` interface, which defines the `add` method.

    ```fig
    interface Add {
        fn add(self, other: Self) -> Self
    }

    // When you write:
    let x = a + b;
    // The compiler understands this as:
    let x = a.add(b);
    ```

*   This principle applies consistently across all operator categories—arithmetic, logical, bitwise, and comparison operators. By implementing the relevant interface, any custom type can define how these operators interact with its instances, ensuring type safety and intuitive code.
