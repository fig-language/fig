# Nyx Compiler Test Suite

This directory contains the integration test suite for the Nyx programming language
compiler. Tests are `.nyx` source files organised by language feature and expected
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
| `single_line.nyx` | One `//` comment |
| `multiple_comments.nyx` | Several consecutive comments and comments between statements |
| `comments_between_definitions.nyx` | Comments before structs, enums, functions |

### `valid/literals`
| File | What it tests |
|---|---|
| `integers.nyx` | Various decimal integer literals |
| `floats.nyx` | Float literals with a decimal point |
| `booleans.nyx` | `true` and `false` |
| `strings_basic.nyx` | Plain string literals including `""` escaped quote |
| `strings_interpolated.nyx` | `$"…{expr}…"` interpolation |
| `strings_interpolated_field.nyx` | Interpolated string referencing struct fields |
| `mixed_literals.nyx` | All literal kinds in one file |

### `valid/variables`
| File | What it tests |
|---|---|
| `let_no_type.nyx` | `let` without type annotation (type inferred) |
| `let_with_type.nyx` | `let` with every primitive type annotation |
| `mut_no_type.nyx` | `mut` without type annotation |
| `mut_with_type.nyx` | `mut` with type annotation |
| `const_no_type.nyx` | `const` without type annotation |
| `const_with_type.nyx` | `const` with type annotation |
| `const_public.nyx` | `public const` |
| `const_generic.nyx` | `const[T]` generic constant |
| `const_type_associated.nyx` | Associated constants like `Seq[T]::CAPACITY` |

### `valid/types`
| File | What it tests |
|---|---|
| `primitive_aliases.nyx` | All primitive type aliases |
| `ok_null_optional.nyx` | `ok`, `null`, and `?T` optional types |
| `pointer_types.nyx` | `*T`, `*mut T`, `?*T`, pointer chains |
| `slice_types.nyx` | `[T]` slice types and slices of slices |
| `array_types.nyx` | `[T; N]` fixed-size arrays |
| `generic_aliases.nyx` | Generic type aliases `Slice[T] = [T]` |
| `namespaced_aliases.nyx` | Type aliases with namespaced names |
| `complex_nested_types.nyx` | Deeply nested / combined type forms |
| `generic_path_aliases.nyx` | Type aliases pointing to generic paths |
| `visibility_on_types.nyx` | `public`/`export`/`private` on `type` statements |

### `valid/functions`
| File | What it tests |
|---|---|
| `declarations_basic.nyx` | Forward declarations (no body) |
| `definitions_pass.nyx` | Minimal function bodies using `pass` |
| `return_statement.nyx` | `return` with various types |
| `multiple_parameters.nyx` | Functions with 2–4 parameters |
| `return_pointer.nyx` | Return pointer types and optional pointers |
| `pointer_parameters.nyx` | Pointer type parameters |
| `self_parameters.nyx` | `self`, `*self`, `*mut self` |
| `mut_self.nyx` | `mut self` (by-value mutable self) |
| `no_return_type.nyx` | Functions omitting `->` return type |
| `generic_basic.nyx` | Generic functions `[T]` |
| `generic_where.nyx` | Generic functions with `where` clause |
| `generic_scope_syntax.nyx` | `add::[T](…)` and `Seq[T]::method(…)` syntax |
| `effect_marker.nyx` | `func!` effect marker |
| `effect_generic_self.nyx` | `func!` combined with generics and `self` |
| `extern_declarations.nyx` | `extern func` C FFI declarations |
| `visibility.nyx` | `public`/`export`/`private` on functions |
| `namespaced_methods.nyx` | `func Type::method(…)` namespaced functions |
| `body_with_locals.nyx` | Function bodies with `let` bindings |
| `calling_functions.nyx` | Nested function call expressions |
| `return_slice_array.nyx` | Functions returning slice/array types |

### `valid/structs`
| File | What it tests |
|---|---|
| `empty.nyx` | Struct with no body |
| `single_field.nyx` | Struct with one field |
| `multiple_fields.nyx` | Structs with 2–4 fields |
| `pointer_fields.nyx` | Struct fields that are pointers (self-referential) |
| `slice_fields.nyx` | Struct fields that are slices |
| `generic_single.nyx` | Generic `struct Foo[T]` |
| `generic_multiple.nyx` | Generic struct with multiple type params |
| `packed.nyx` | `packed struct` (no padding) |
| `with_requires.nyx` | Struct with `requires` clause |
| `with_where.nyx` | Struct with `where` clause |
| `requires_and_where.nyx` | Struct with both clauses |
| `namespaced.nyx` | `struct ns::Name[T]` |
| `visibility.nyx` | Visibility modifiers on structs |
| `with_annotations.nyx` | `#repr(C)` and similar annotations |

### `valid/enums`
| File | What it tests |
|---|---|
| `simple.nyx` | Basic enum, auto-assigned values |
| `explicit_values.nyx` | Variants with `= expr` |
| `mixed_values.nyx` | Mix of explicit and auto-assigned values |
| `with_repr.nyx` | `enum[u8]`, `enum[u16]`, `enum[i32]` repr types |
| `with_requires.nyx` | `enum` with `requires` clause |
| `namespaced.nyx` | `enum ns::Name` |
| `visibility.nyx` | Visibility modifiers on enums |
| `with_annotations.nyx` | Annotations on enums |
| `single_variant.nyx` | Single-variant enums (edge case) |
| `many_variants.nyx` | Large enum (20 variants) |

### `valid/unions`
| File | What it tests |
|---|---|
| `simple.nyx` | Two-variant union |
| `generic_single.nyx` | `union Option[T]` |
| `generic_pair.nyx` | `union Either[L, R]`, `union Result[T, E]` |
| `with_requires.nyx` | Union with `requires` clause |
| `with_where.nyx` | Union with `where` clause |
| `namespaced.nyx` | `union ns::Name[T]` |
| `visibility.nyx` | Visibility modifiers on unions |
| `many_variants.nyx` | Six-variant union |

### `valid/interfaces`
| File | What it tests |
|---|---|
| `empty.nyx` | Empty interface (marker) |
| `single_method.nyx` | One method declaration |
| `multiple_methods.nyx` | Multiple method declarations |
| `effect_methods.nyx` | Methods with `func!` effect marker |
| `with_extends.nyx` | `extends` clause |
| `with_where.nyx` | `where` clause |
| `with_requires.nyx` | `requires` clause |
| `all_clauses.nyx` | `extends` + `where` + `requires` + methods |
| `visibility_generic.nyx` | Visibility + generic params |
| `namespaced.nyx` | `interface ns::Name` |
| `extends_many.nyx` | Extending many interfaces |

### `valid/namespaces`
| File | What it tests |
|---|---|
| `empty_pass.nyx` | Namespace with `pass` |
| `declaration_only.nyx` | Namespace without body |
| `with_functions.nyx` | Namespace containing function declarations |
| `with_types.nyx` | Namespace containing struct/enum definitions |
| `nested.nyx` | Nested namespace blocks |
| `path_name.nyx` | Namespace with `::` path name |
| `visibility.nyx` | Visibility modifiers on namespaces |
| `with_annotations.nyx` | Annotations on namespaces |

### `valid/expressions`
| File | What it tests |
|---|---|
| `arithmetic.nyx` | `+`, `-`, `*`, `/`, `%` |
| `comparisons.nyx` | `<`, `>`, `<=`, `>=`, `==`, `!=` |
| `logical.nyx` | `&&`, `\|\|` |
| `bitwise.nyx` | `&`, `\|`, `^`, `<<`, `>>` |
| `unary.nyx` | `&`, `*`, `-`, `+`, `~`, `!` |
| `as_cast.nyx` | `expr as Type` |
| `parenthesized.nyx` | `(expr)` precedence override |
| `field_access.nyx` | `expr.field` |
| `field_access_chained.nyx` | `a.b.c` chains |
| `type_access.nyx` | `Type::member` |
| `index_suffix.nyx` | `expr[index]` |
| `call_basic.nyx` | Function calls, nested calls |
| `method_call.nyx` | Method calls via `.` |
| `sizeof_alignof_offsetof.nyx` | Built-in size/align/offset operators |
| `chained_postfix.nyx` | Mixed postfix chains |
| `effect_call_suffix.nyx` | `expr.method!(…)` |
| `operator_precedence.nyx` | Cross-level precedence verification |
| `self_expr.nyx` | `self` as expression in methods |

### `valid/control_flow`
| File | What it tests |
|---|---|
| `if_simple.nyx` | Plain `if` |
| `if_else.nyx` | `if` / `else` |
| `if_elif_else.nyx` | `if` / `elif` / `else` |
| `if_many_elif.nyx` | Multiple `elif` branches |
| `if_bool_conditions.nyx` | Complex boolean conditions |
| `if_nested.nyx` | Nested `if` statements |
| `for_basic.nyx` | `for x in expr` |
| `for_with_body.nyx` | `for` with multi-statement body |
| `for_nested.nyx` | Nested `for` loops |
| `while_basic.nyx` | `while condition` |
| `return_various.nyx` | `return` in different positions |
| `pass_statement.nyx` | `pass` in functions, `if`, `for` |
| `block_named.nyx` | `block name { … }` |
| `block_anon.nyx` | Anonymous `block { … }` |
| `for_ptr_iteration.nyx` | Manual pointer loop |

### `valid/using` · `valid/annotations` · `valid/visibility`
| File | What it tests |
|---|---|
| `using/simple.nyx` | Basic `using path` |
| `using/with_visibility.nyx` | `public`/`export`/`private using` |
| `using/deep_path.nyx` | `using` with 3-level paths |
| `annotations/no_args.nyx` | `#name` annotations |
| `annotations/with_args.nyx` | `#name(arg)` annotations |
| `annotations/multi_args.nyx` | `#name(a, b)` |
| `annotations/stacked.nyx` | Multiple annotations on one definition |
| `visibility/all_modifiers.nyx` | All three visibility keywords on all definition kinds |

### `valid/generics`
| File | What it tests |
|---|---|
| `basic_params.nyx` | `[T]` on functions, structs, interfaces, unions |
| `default_params.nyx` | `[T = Default]` generic defaults |
| `where_compound_bounds.nyx` | `T: A + B + C` compound bounds |
| `where_multiple_params.nyx` | Multiple constrained type params |
| `type_arguments.nyx` | Generic type arguments at call sites |

### `valid/paths`
| File | What it tests |
|---|---|
| `simple_and_namespaced.nyx` | `foo`, `a::b::c` |
| `with_generics.nyx` | `Vec[T]`, `Map[K,V]` in path position |
| `type_prefixed.nyx` | `::Type[T]::member` type-prefixed paths |
| `builtin_namespaces.nyx` | `std`, `core`, `alloc` builtin namespaces |

### `valid/edge_cases`
| File | What it tests |
|---|---|
| `empty_file.nyx` | Completely empty source file |
| `only_comments.nyx` | File with only comments |
| `deeply_nested_types.nyx` | Chains of `?`, `*mut`, `[]` type qualifiers |
| `chained_casts.nyx` | `x as u8 as u16 as u32` |
| `zero_params.nyx` | Functions with zero parameters |
| `nested_structs.nyx` | Structs whose fields are other structs |
| `single_char_identifiers.nyx` | Single-letter variable/function names |
| `underscore_identifiers.nyx` | `_x`, `__x`, `x_` names |
| `long_identifiers.nyx` | Very long identifier names |
| `zero_literals.nyx` | `0` and `0.0` literals |
| `empty_string.nyx` | `""` empty string |
| `interface_clauses_only.nyx` | Interface with clauses but no methods |
| `consecutive_pass_functions.nyx` | Many small functions all using `pass` |
| `many_let_bindings.nyx` | Function with 15+ local variables |
| `where_before_pass.nyx` | `where` clause immediately before `pass` |
| `annotation_complex_arg.nyx` | Annotation argument that is a path/section name |
| `for_no_body.nyx` | `for` with no body (grammar allows it) |
| `while_no_body.nyx` | `while` with no body |
| `const_referencing_const.nyx` | Const expressions referencing other consts |

### `valid/integration`
| File | What it tests |
|---|---|
| `linked_list.nyx` | Generic linked list struct + methods |
| `math_vector.nyx` | `Vec2` / `Vec3` with arithmetic methods |
| `stack.nyx` | Generic stack over raw pointer array |
| `error_handling.nyx` | Union-based `Result[T]` + IO functions |
| `interpreter_ast.nyx` | Union-based expression tree + evaluator |
| `iterator_range.nyx` | `Iterator[T]` interface + `Range` implementation |
| `hash_map.nyx` | `Map[K,V]` interface + `HashMap` skeleton |
| `all_features.nyx` | Showcases all major features together |
| `string_utils_namespace.nyx` | Namespace full of string utility functions |
| `seq_dynamic_array.nyx` | Full `Seq[T]` implementation close to standard library |

### `valid/realistic`
Large, real-world examples that demonstrate practical usage of multiple language features together. These files are substantially longer (300+ lines) and mix structs, enums, unions, generics, pointers, effects, where clauses, and complex control flow to represent realistic systems programming scenarios.

| File | What it tests |
|---|---|
| `buddy_allocator.nyx` | Memory allocator with power-of-two blocks and free list coalescing |
| `json_parser.nyx` | JSON parser with union-based result types and recursive descent parsing |
| `arg_parser.nyx` | CLI argument parser supporting flags, options, and positional arguments |
| `logger.nyx` | Multi-target logging framework with log levels and formatters |
| `thread_pool.nyx` | Thread pool for concurrent task execution with work stealing |
| `http_types.nyx` | HTTP request/response types with header parsing |
| `binary_serializer.nyx` | Binary serialization with variable-length encoding |
| `ring_buffer.nyx` | Fixed-size circular buffer and multi-producer single-consumer variants |
| `utf8_string.nyx` | UTF-8 string handling with validation, iteration, and manipulation |
| `hash_map.nyx` | Hash table with open addressing and linear probing |
| `virtual_machine.nyx` | Stack-based bytecode interpreter with assembler |
| `lexer.nyx` | Tokenizing lexer for programming language source code |
| `async_runtime.nyx` | Async runtime with futures, tasks, and reactor pattern |
| `tcp_server.nyx` | TCP server with socket abstraction and connection handling |
| `btree.nyx` | B-tree implementation for ordered storage |
| `regex_engine.nyx` | Regular expression matcher with NFA-based engine |
| `compression.nyx` | LZ77 compression, Huffman coding, and run-length encoding |
| `elf_parser.nyx` | ELF file parser for reading executable format |
| `fixed_point_math.nyx` | Fixed-point arithmetic library with vector/matrix math |
| `slab_allocator.nyx` | Slab allocator for efficient fixed-size allocation |
| `cli_builder.nyx` | CLI builder with argument parsing and help generation |
| `database_index.nyx` | B+ tree database index with range query support |

---

## `invalid/syntax` — Expected Parse Errors

Each file is intentionally malformed and **must be rejected** by the parser.

| `missing_param_type.nyx` | Function parameter missing type annotation |
| `duplicate_visibility.nyx` | Multiple visibility modifiers on same declaration |
| `where_without_generics.nyx` | Where clause on non-generic function |
| `mismatched_indent.nyx` | Inconsistent indentation in function body |
| `effect_on_struct.nyx` | Effect marker `!` on struct definition |
| `union_missing_type.nyx` | Union variant without associated type |
| `interface_with_body.nyx` | Interface method with implementation body |
| `generic_using.nyx` | Generic parameters in using statement |
| `annotation_empty_parens.nyx` | Annotation with empty parentheses |
| `self_outside_context.nyx` | `Self` type outside method/interface context |
| `break_outside_loop.nyx` | Break statement not in loop |
| `continue_outside_loop.nyx` | Continue statement not in loop |
| `assign_to_literal.nyx` | Assignment to literal value |
| `keyword_as_identifier.nyx` | Reserved keyword used as identifier |
| `enum_duplicate_variants.nyx` | Enum with duplicate variant names |
| `struct_duplicate_fields.nyx` | Struct with duplicate field names |
| `requires_on_function.nyx` | Requires clause on function (not allowed) |
| `double_effect.nyx` | Multiple effect markers on same function |
| `wrong_generic_brackets.nyx` | Using `<>` instead of `[]` for generic parameters |
| `const_missing_type.nyx` | Const declaration without type annotation |
| File | Broken rule |
|---|---|
| `let_missing_eq.nyx` | `let x 10` — missing `=` |
| `let_missing_value.nyx` | `let x =` — no RHS expression |
| `let_missing_name.nyx` | `let = 10` — no identifier |
| `func_param_no_type.nyx` | Parameter without `: Type` |
| `func_body_no_indent.nyx` | Function body at wrong indentation level |
| `struct_field_no_type.nyx` | Struct field without type annotation |
| `struct_field_wrong_sep.nyx` | `x = i32` instead of `x: i32` |
| `enum_no_variants.nyx` | Empty enum body |
| `union_no_variants.nyx` | Empty union body |
| `call_missing_paren.nyx` | Unclosed `(` in function call |
| `generic_args_empty.nyx` | `Vec[]` — empty generic argument list |
| `func_missing_arrow.nyx` | `func f() ok` — no `->` before return type |
| `if_no_condition.nyx` | `if` with no condition expression |
| `for_missing_in.nyx` | `for x data` — missing `in` keyword |
| `type_missing_eq.nyx` | `type Foo i32` — missing `=` |
| `const_no_value.nyx` | `const X: u32` — missing `= expr` |
| `where_empty.nyx` | `where` block with no constraints |
| `requires_empty.nyx` | `requires` block with no requirements |
| `unbalanced_parens.nyx` | `(x + 1` — missing `)` |
| `binary_missing_rhs.nyx` | `x +` — no right-hand operand |
| `as_cast_missing_type.nyx` | `x as` — no target type |
| `annotation_missing_name.nyx` | `#` with no identifier |
| `func_no_name.nyx` | `func () -> ok` — no function name |
| `trailing_comma_in_call.nyx` | `foo(1, 2,)` — trailing comma in argument list |
| `interface_method_no_func.nyx` | Interface body with bare method, missing `func` |
| `namespace_no_indent.nyx` | Namespace body not indented |

---

## Token Validation with Insta Snapshots

The lexer tests use [**insta**](https://insta.rs/) for snapshot testing. Each `.nyx` file produces a snapshot of tokens in clean, readable YAML format - no manual token file maintenance required!

### How It Works

1. Each `.nyx` test file is tokenized
2. The token sequence is automatically serialized to YAML
3. Insta compares against stored snapshots (`.snap` files)
4. When tokens change, you review diffs visually

### Snapshot Location

Snapshots are stored in `crates/nyx-lexer/tests/snapshots/` with names like:
```
integration_tests__comments__single_line.snap
integration_tests__functions__declarations_basic.snap
```

### Example Snapshot

Here's what a snapshot looks like (`comments__single_line.snap`):

```yaml
---
source: crates/nyx-lexer/tests/integration_tests.rs
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
cargo test -p nyx-lexer --test integration_tests

# Run specific test
cargo test -p nyx-lexer --test integration_tests 'comments/single_line'
```

### Updating Snapshots

When you modify the lexer or add new features, snapshots may need updating:

```bash
# Review pending snapshot changes
cargo insta test --review -p nyx-lexer --test integration_tests

# Or auto-accept all changes (use carefully!)
cargo insta test --accept -p nyx-lexer --test integration_tests
```

The `cargo insta review` command opens an interactive UI showing:
- Old snapshot vs new snapshot
- Line-by-line diffs
- Accept/reject options

### Test Failure Example

When a snapshot doesn't match, you get clear output:

```
---- lexer_test::comments/single_line.nyx ----
Snapshot assertion failed:
  - Ident: "x"
  + Ident: "y"

To update snapshots run: cargo insta review
```

### Adding New Tests

1. Add a new `.nyx` file in the appropriate `tests/valid/` subdirectory
2. Run tests - insta will create a new snapshot
3. Run `cargo insta review` to inspect and accept the snapshot
4. Commit both the `.nyx` file and the `.snap` file

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
- **No `main`.** Nyx does not have a `main` function convention in these test
  fixtures; they are module-level files.

---

## Adding New Tests

1. Pick the appropriate subdirectory (or create a new one if a language area is
   not yet represented).
2. Name the file `<feature>_<variant>.nyx` (snake_case).
3. Add a brief top-of-file comment explaining what the file tests.
4. For invalid tests, also state the exact rule being violated and the expected
   error kind.
5. Update this README table.
