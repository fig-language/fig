# Fig Compiler Test Suite

This directory contains the integration test suite for the Fig programming language
compiler. Tests are `.fig` source files organised by language feature and expected
outcome. They are intended to be consumed by any crate in the compiler pipeline —
lexer, parser, checker, CLI, etc. — that needs end-to-end source-file fixtures.

> **Status:** Early draft — the language grammar and semantics are still rapidly
> evolving. Expect tests to be added, removed, or changed frequently.

---

## Directory Structure

```
tests/
├── valid/            # Files that must be accepted (parse + lex successfully)
│   ├── comments/
│   ├── literals/
│   ├── variables/
│   ├── types/
│   ├── functions/
│   ├── structs/
│   ├── enums/
│   ├── unions/
│   ├── interfaces/
│   ├── namespaces/
│   ├── expressions/
│   ├── control_flow/
│   ├── using/
│   ├── annotations/
│   ├── visibility/
│   ├── generics/
│   ├── paths/
│   ├── edge_cases/
│   ├── integration/
│   └── realistic/    # Large realistic examples mixing multiple features
└── invalid/
    └── syntax/       # Files that must be REJECTED with a parse error
```

---

## Coverage

### `valid/comments`
| File | What it tests |
|---|---|
| `single_line.fig` | One `//` comment |
| `multiple_comments.fig` | Several consecutive comments and comments between statements |
| `comments_between_definitions.fig` | Comments before structs, enums, functions |

### `valid/literals`
| File | What it tests |
|---|---|
| `integers.fig` | Various decimal integer literals |
| `floats.fig` | Float literals with a decimal point |
| `booleans.fig` | `true` and `false` |
| `strings_basic.fig` | Plain string literals including `""` escaped quote |
| `strings_interpolated.fig` | `$"…{expr}…"` interpolation |
| `strings_interpolated_field.fig` | Interpolated string referencing struct fields |
| `mixed_literals.fig` | All literal kinds in one file |

### `valid/variables`
| File | What it tests |
|---|---|
| `let_no_type.fig` | `let` without type annotation (type inferred) |
| `let_with_type.fig` | `let` with every primitive type annotation |
| `mut_no_type.fig` | `mut` without type annotation |
| `mut_with_type.fig` | `mut` with type annotation |
| `const_no_type.fig` | `const` without type annotation |
| `const_with_type.fig` | `const` with type annotation |
| `const_public.fig` | `public const` |
| `const_generic.fig` | `const[T]` generic constant |
| `const_type_associated.fig` | Associated constants like `Seq[T]::CAPACITY` |

### `valid/types`
| File | What it tests |
|---|---|
| `primitive_aliases.fig` | All primitive type aliases |
| `ok_null_optional.fig` | `ok`, `null`, and `?T` optional types |
| `pointer_types.fig` | `*T`, `*mut T`, `?*T`, pointer chains |
| `slice_types.fig` | `[T]` slice types and slices of slices |
| `array_types.fig` | `[T; N]` fixed-size arrays |
| `generic_aliases.fig` | Generic type aliases `Slice[T] = [T]` |
| `namespaced_aliases.fig` | Type aliases with namespaced names |
| `complex_nested_types.fig` | Deeply nested / combined type forms |
| `generic_path_aliases.fig` | Type aliases pointing to generic paths |
| `visibility_on_types.fig` | `public`/`export`/`private` on `type` statements |

### `valid/functions`
| File | What it tests |
|---|---|
| `declarations_basic.fig` | Forward declarations (no body) |
| `definitions_pass.fig` | Minimal function bodies using `pass` |
| `return_statement.fig` | `return` with various types |
| `multiple_parameters.fig` | Functions with 2–4 parameters |
| `return_pointer.fig` | Return pointer types and optional pointers |
| `pointer_parameters.fig` | Pointer type parameters |
| `self_parameters.fig` | `self`, `*self`, `*mut self` |
| `mut_self.fig` | `mut self` (by-value mutable self) |
| `no_return_type.fig` | Functions omitting `->` return type |
| `generic_basic.fig` | Generic functions `[T]` |
| `generic_where.fig` | Generic functions with `where` clause |
| `generic_scope_syntax.fig` | `add::[T](…)` and `Seq[T]::method(…)` syntax |
| `effect_marker.fig` | `func!` effect marker |
| `effect_generic_self.fig` | `func!` combined with generics and `self` |
| `extern_declarations.fig` | `extern func` C FFI declarations |
| `visibility.fig` | `public`/`export`/`private` on functions |
| `namespaced_methods.fig` | `func Type::method(…)` namespaced functions |
| `body_with_locals.fig` | Function bodies with `let` bindings |
| `calling_functions.fig` | Nested function call expressions |
| `return_slice_array.fig` | Functions returning slice/array types |

### `valid/structs`
| File | What it tests |
|---|---|
| `empty.fig` | Struct with no body |
| `single_field.fig` | Struct with one field |
| `multiple_fields.fig` | Structs with 2–4 fields |
| `pointer_fields.fig` | Struct fields that are pointers (self-referential) |
| `slice_fields.fig` | Struct fields that are slices |
| `generic_single.fig` | Generic `struct Foo[T]` |
| `generic_multiple.fig` | Generic struct with multiple type params |
| `packed.fig` | `packed struct` (no padding) |
| `with_requires.fig` | Struct with `requires` clause |
| `with_where.fig` | Struct with `where` clause |
| `requires_and_where.fig` | Struct with both clauses |
| `namespaced.fig` | `struct ns::Name[T]` |
| `visibility.fig` | Visibility modifiers on structs |
| `with_annotations.fig` | `#repr(C)` and similar annotations |

### `valid/enums`
| File | What it tests |
|---|---|
| `simple.fig` | Basic enum, auto-assigned values |
| `explicit_values.fig` | Variants with `= expr` |
| `mixed_values.fig` | Mix of explicit and auto-assigned values |
| `with_repr.fig` | `enum[u8]`, `enum[u16]`, `enum[i32]` repr types |
| `with_requires.fig` | `enum` with `requires` clause |
| `namespaced.fig` | `enum ns::Name` |
| `visibility.fig` | Visibility modifiers on enums |
| `with_annotations.fig` | Annotations on enums |
| `single_variant.fig` | Single-variant enums (edge case) |
| `many_variants.fig` | Large enum (20 variants) |

### `valid/unions`
| File | What it tests |
|---|---|
| `simple.fig` | Two-variant union |
| `generic_single.fig` | `union Option[T]` |
| `generic_pair.fig` | `union Either[L, R]`, `union Result[T, E]` |
| `with_requires.fig` | Union with `requires` clause |
| `with_where.fig` | Union with `where` clause |
| `namespaced.fig` | `union ns::Name[T]` |
| `visibility.fig` | Visibility modifiers on unions |
| `many_variants.fig` | Six-variant union |

### `valid/interfaces`
| File | What it tests |
|---|---|
| `empty.fig` | Empty interface (marker) |
| `single_method.fig` | One method declaration |
| `multiple_methods.fig` | Multiple method declarations |
| `effect_methods.fig` | Methods with `func!` effect marker |
| `with_extends.fig` | `extends` clause |
| `with_where.fig` | `where` clause |
| `with_requires.fig` | `requires` clause |
| `all_clauses.fig` | `extends` + `where` + `requires` + methods |
| `visibility_generic.fig` | Visibility + generic params |
| `namespaced.fig` | `interface ns::Name` |
| `extends_many.fig` | Extending many interfaces |

### `valid/namespaces`
| File | What it tests |
|---|---|
| `empty_pass.fig` | Namespace with `pass` |
| `declaration_only.fig` | Namespace without body |
| `with_functions.fig` | Namespace containing function declarations |
| `with_types.fig` | Namespace containing struct/enum definitions |
| `nested.fig` | Nested namespace blocks |
| `path_name.fig` | Namespace with `::` path name |
| `visibility.fig` | Visibility modifiers on namespaces |
| `with_annotations.fig` | Annotations on namespaces |

### `valid/expressions`
| File | What it tests |
|---|---|
| `arithmetic.fig` | `+`, `-`, `*`, `/`, `%` |
| `comparisons.fig` | `<`, `>`, `<=`, `>=`, `==`, `!=` |
| `logical.fig` | `&&`, `\|\|` |
| `bitwise.fig` | `&`, `\|`, `^`, `<<`, `>>` |
| `unary.fig` | `&`, `*`, `-`, `+`, `~`, `!` |
| `as_cast.fig` | `expr as Type` |
| `parenthesized.fig` | `(expr)` precedence override |
| `field_access.fig` | `expr.field` |
| `field_access_chained.fig` | `a.b.c` chains |
| `type_access.fig` | `Type::member` |
| `index_suffix.fig` | `expr[index]` |
| `call_basic.fig` | Function calls, nested calls |
| `method_call.fig` | Method calls via `.` |
| `sizeof_alignof_offsetof.fig` | Built-in size/align/offset operators |
| `chained_postfix.fig` | Mixed postfix chains |
| `effect_call_suffix.fig` | `expr.method!(…)` |
| `operator_precedence.fig` | Cross-level precedence verification |
| `self_expr.fig` | `self` as expression in methods |

### `valid/control_flow`
| File | What it tests |
|---|---|
| `if_simple.fig` | Plain `if` |
| `if_else.fig` | `if` / `else` |
| `if_elif_else.fig` | `if` / `elif` / `else` |
| `if_many_elif.fig` | Multiple `elif` branches |
| `if_bool_conditions.fig` | Complex boolean conditions |
| `if_nested.fig` | Nested `if` statements |
| `for_basic.fig` | `for x in expr` |
| `for_with_body.fig` | `for` with multi-statement body |
| `for_nested.fig` | Nested `for` loops |
| `while_basic.fig` | `while condition` |
| `return_various.fig` | `return` in different positions |
| `pass_statement.fig` | `pass` in functions, `if`, `for` |
| `block_named.fig` | `block name { … }` |
| `block_anon.fig` | Anonymous `block { … }` |
| `for_ptr_iteration.fig` | Manual pointer loop |

### `valid/using` · `valid/annotations` · `valid/visibility`
| File | What it tests |
|---|---|
| `using/simple.fig` | Basic `using path` |
| `using/with_visibility.fig` | `public`/`export`/`private using` |
| `using/deep_path.fig` | `using` with 3-level paths |
| `annotations/no_args.fig` | `#name` annotations |
| `annotations/with_args.fig` | `#name(arg)` annotations |
| `annotations/multi_args.fig` | `#name(a, b)` |
| `annotations/stacked.fig` | Multiple annotations on one definition |
| `visibility/all_modifiers.fig` | All three visibility keywords on all definition kinds |

### `valid/generics`
| File | What it tests |
|---|---|
| `basic_params.fig` | `[T]` on functions, structs, interfaces, unions |
| `default_params.fig` | `[T = Default]` generic defaults |
| `where_compound_bounds.fig` | `T: A + B + C` compound bounds |
| `where_multiple_params.fig` | Multiple constrained type params |
| `type_arguments.fig` | Generic type arguments at call sites |

### `valid/paths`
| File | What it tests |
|---|---|
| `simple_and_namespaced.fig` | `foo`, `a::b::c` |
| `with_generics.fig` | `Vec[T]`, `Map[K,V]` in path position |
| `type_prefixed.fig` | `::Type[T]::member` type-prefixed paths |
| `builtin_namespaces.fig` | `std`, `core`, `alloc` builtin namespaces |

### `valid/edge_cases`
| File | What it tests |
|---|---|
| `empty_file.fig` | Completely empty source file |
| `only_comments.fig` | File with only comments |
| `deeply_nested_types.fig` | Chains of `?`, `*mut`, `[]` type qualifiers |
| `chained_casts.fig` | `x as u8 as u16 as u32` |
| `zero_params.fig` | Functions with zero parameters |
| `nested_structs.fig` | Structs whose fields are other structs |
| `single_char_identifiers.fig` | Single-letter variable/function names |
| `underscore_identifiers.fig` | `_x`, `__x`, `x_` names |
| `long_identifiers.fig` | Very long identifier names |
| `zero_literals.fig` | `0` and `0.0` literals |
| `empty_string.fig` | `""` empty string |
| `interface_clauses_only.fig` | Interface with clauses but no methods |
| `consecutive_pass_functions.fig` | Many small functions all using `pass` |
| `many_let_bindings.fig` | Function with 15+ local variables |
| `where_before_pass.fig` | `where` clause immediately before `pass` |
| `annotation_complex_arg.fig` | Annotation argument that is a path/section name |
| `for_no_body.fig` | `for` with no body (grammar allows it) |
| `while_no_body.fig` | `while` with no body |
| `const_referencing_const.fig` | Const expressions referencing other consts |

### `valid/integration`
| File | What it tests |
|---|---|
| `linked_list.fig` | Generic linked list struct + methods |
| `math_vector.fig` | `Vec2` / `Vec3` with arithmetic methods |
| `stack.fig` | Generic stack over raw pointer array |
| `error_handling.fig` | Union-based `Result[T]` + IO functions |
| `interpreter_ast.fig` | Union-based expression tree + evaluator |
| `iterator_range.fig` | `Iterator[T]` interface + `Range` implementation |
| `hash_map.fig` | `Map[K,V]` interface + `HashMap` skeleton |
| `all_features.fig` | Showcases all major features together |
| `string_utils_namespace.fig` | Namespace full of string utility functions |
| `seq_dynamic_array.fig` | Full `Seq[T]` implementation close to standard library |

### `valid/realistic`
Large, real-world examples that demonstrate practical usage of multiple language features together. These files are substantially longer (300+ lines) and mix structs, enums, unions, generics, pointers, effects, where clauses, and complex control flow to represent realistic systems programming scenarios.

| File | What it tests |
|---|---|
| `buddy_allocator.fig` | Memory allocator with power-of-two blocks and free list coalescing |
| `json_parser.fig` | JSON parser with union-based result types and recursive descent parsing |
| `arg_parser.fig` | CLI argument parser supporting flags, options, and positional arguments |
| `logger.fig` | Multi-target logging framework with log levels and formatters |
| `thread_pool.fig` | Thread pool for concurrent task execution with work stealing |
| `http_types.fig` | HTTP request/response types with header parsing |
| `binary_serializer.fig` | Binary serialization with variable-length encoding |
| `ring_buffer.fig` | Fixed-size circular buffer and multi-producer single-consumer variants |
| `utf8_string.fig` | UTF-8 string handling with validation, iteration, and manipulation |
| `hash_map.fig` | Hash table with open addressing and linear probing |
| `virtual_machine.fig` | Stack-based bytecode interpreter with assembler |
| `lexer.fig` | Tokenizing lexer for programming language source code |
| `async_runtime.fig` | Async runtime with futures, tasks, and reactor pattern |
| `tcp_server.fig` | TCP server with socket abstraction and connection handling |
| `btree.fig` | B-tree implementation for ordered storage |
| `regex_engine.fig` | Regular expression matcher with NFA-based engine |
| `compression.fig` | LZ77 compression, Huffman coding, and run-length encoding |
| `elf_parser.fig` | ELF file parser for reading executable format |
| `fixed_point_math.fig` | Fixed-point arithmetic library with vector/matrix math |
| `slab_allocator.fig` | Slab allocator for efficient fixed-size allocation |
| `cli_builder.fig` | CLI builder with argument parsing and help generation |
| `database_index.fig` | B+ tree database index with range query support |

---

## `invalid/syntax` — Expected Parse Errors

Each file is intentionally malformed and **must be rejected** by the parser.

| `missing_param_type.fig` | Function parameter missing type annotation |
| `duplicate_visibility.fig` | Multiple visibility modifiers on same declaration |
| `where_without_generics.fig` | Where clause on non-generic function |
| `mismatched_indent.fig` | Inconsistent indentation in function body |
| `effect_on_struct.fig` | Effect marker `!` on struct definition |
| `union_missing_type.fig` | Union variant without associated type |
| `interface_with_body.fig` | Interface method with implementation body |
| `generic_using.fig` | Generic parameters in using statement |
| `annotation_empty_parens.fig` | Annotation with empty parentheses |
| `self_outside_context.fig` | `Self` type outside method/interface context |
| `break_outside_loop.fig` | Break statement not in loop |
| `continue_outside_loop.fig` | Continue statement not in loop |
| `assign_to_literal.fig` | Assignment to literal value |
| `keyword_as_identifier.fig` | Reserved keyword used as identifier |
| `enum_duplicate_variants.fig` | Enum with duplicate variant names |
| `struct_duplicate_fields.fig` | Struct with duplicate field names |
| `requires_on_function.fig` | Requires clause on function (not allowed) |
| `double_effect.fig` | Multiple effect markers on same function |
| `wrong_generic_brackets.fig` | Using `<>` instead of `[]` for generic parameters |
| `const_missing_type.fig` | Const declaration without type annotation |
| File | Broken rule |
|---|---|
| `let_missing_eq.fig` | `let x 10` — missing `=` |
| `let_missing_value.fig` | `let x =` — no RHS expression |
| `let_missing_name.fig` | `let = 10` — no identifier |
| `func_param_no_type.fig` | Parameter without `: Type` |
| `func_body_no_indent.fig` | Function body at wrong indentation level |
| `struct_field_no_type.fig` | Struct field without type annotation |
| `struct_field_wrong_sep.fig` | `x = i32` instead of `x: i32` |
| `enum_no_variants.fig` | Empty enum body |
| `union_no_variants.fig` | Empty union body |
| `call_missing_paren.fig` | Unclosed `(` in function call |
| `generic_args_empty.fig` | `Vec[]` — empty generic argument list |
| `func_missing_arrow.fig` | `func f() ok` — no `->` before return type |
| `if_no_condition.fig` | `if` with no condition expression |
| `for_missing_in.fig` | `for x data` — missing `in` keyword |
| `type_missing_eq.fig` | `type Foo i32` — missing `=` |
| `const_no_value.fig` | `const X: u32` — missing `= expr` |
| `where_empty.fig` | `where` block with no constraints |
| `requires_empty.fig` | `requires` block with no requirements |
| `unbalanced_parens.fig` | `(x + 1` — missing `)` |
| `binary_missing_rhs.fig` | `x +` — no right-hand operand |
| `as_cast_missing_type.fig` | `x as` — no target type |
| `annotation_missing_name.fig` | `#` with no identifier |
| `func_no_name.fig` | `func () -> ok` — no function name |
| `trailing_comma_in_call.fig` | `foo(1, 2,)` — trailing comma in argument list |
| `interface_method_no_func.fig` | Interface body with bare method, missing `func` |
| `namespace_no_indent.fig` | Namespace body not indented |

---

## Token Validation with Insta Snapshots

The lexer tests use [**insta**](https://insta.rs/) for snapshot testing. Each `.fig` file produces a snapshot of tokens in clean, readable YAML format - no manual token file maintenance required!

### How It Works

1. Each `.fig` test file is tokenized
2. The token sequence is automatically serialized to YAML
3. Insta compares against stored snapshots (`.snap` files)
4. When tokens change, you review diffs visually

### Snapshot Location

Snapshots are stored in `crates/fig-lexer/tests/snapshots/` with names like:
```
integration_tests__comments__single_line.snap
integration_tests__functions__declarations_basic.snap
```

### Example Snapshot

Here's what a snapshot looks like (`comments__single_line.snap`):

```yaml
---
source: crates/fig-lexer/tests/integration_tests.rs
expression: tokens
---
- Newline
- Let
- Ident: x
- Eq
- IntegerLiteral:
    base: Decimal
    digits: "42"
    suffix: ~
- Newline
```

**Much cleaner than Rust Debug format!** No `Token::`, no `.to_string()`, no manual maintenance.

### Running Tests

```bash
# Run all lexer tests
cargo test -p fig-lexer --test integration_tests

# Run specific test
cargo test -p fig-lexer --test integration_tests 'comments/single_line'
```

### Updating Snapshots

When you modify the lexer or add new features, snapshots may need updating:

```bash
# Review pending snapshot changes
cargo insta test --review -p fig-lexer --test integration_tests

# Or auto-accept all changes (use carefully!)
cargo insta test --accept -p fig-lexer --test integration_tests
```

The `cargo insta review` command opens an interactive UI showing:
- Old snapshot vs new snapshot
- Line-by-line diffs
- Accept/reject options

### Test Failure Example

When a snapshot doesn't match, you get clear output:

```
---- lexer_test::comments/single_line.fig ----
Snapshot assertion failed:
  - Ident: "x"
  + Ident: "y"

To update snapshots run: cargo insta review
```

### Adding New Tests

1. Add a new `.fig` file in the appropriate `tests/valid/` subdirectory
2. Run tests - insta will create a new snapshot
3. Run `cargo insta review` to inspect and accept the snapshot
4. Commit both the `.fig` file and the `.snap` file

### Benefits Over Manual Token Files

✅ **Automatic**: Snapshots generated from actual lexer output  
✅ **Readable**: Clean YAML format instead of Rust syntax noise  
✅ **Maintainable**: Visual diffs when reviewing changes  
✅ **Standard**: Used by many Rust compiler projects  
✅ **No encoding bugs**: Direct serialization with serde

---

## Conventions

- **One concern per file.** Each file tests exactly one feature or variant of a
  feature. This makes failures easy to attribute.
- **`pass` as stub.** Valid files use `pass` wherever a body is structurally
  required but the semantic content is irrelevant to the test.
- **Invalid files explain themselves.** Every file under `invalid/` starts with a
  comment stating what is broken and what error is expected.
- **No `main`.** Fig does not have a `main` function convention in these test
  fixtures; they are module-level files.

---

## Adding New Tests

1. Pick the appropriate subdirectory (or create a new one if a language area is
   not yet represented).
2. Name the file `<feature>_<variant>.fig` (snake_case).
3. Add a brief top-of-file comment explaining what the file tests.
4. For invalid tests, also state the exact rule being violated and the expected
   error kind.
5. Update this README table.
