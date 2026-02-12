# AST Pretty Printer Module

## Overview

The `pretty_print` module provides comprehensive visualization tools for the Nyx Abstract Syntax Tree (AST). This module is designed specifically for debugging purposes, offering detailed and readable representations of complex AST structures.

## Features

- **Tree-based visualization**: Displays AST structures in a hierarchical tree format
- **Unicode support**: Uses Unicode box-drawing characters for elegant output
- **ASCII fallback**: Optional ASCII-only mode for compatibility
- **Customizable indentation**: Configurable spacing for different readability needs
- **Comprehensive coverage**: Supports all Expression variants including:
  - Literals (integers, floats, booleans, characters, strings)
  - Binary operations (arithmetic, logical, bitwise, comparison)
  - Unary operations (negation, logical NOT, bitwise NOT)
  - Array literals
  - Parenthesized expressions
  - Identifiers

## Basic Usage

### Quick Start

```rust
use nyx_parser::{Lexer, parser::ExpressionParser, pretty_print};

let input = "2 + 3 * 4";
let lexer = Lexer::new(input);
let parser = ExpressionParser::new();
let ast = parser.parse(lexer).unwrap();

// Print using the default settings
println!("{}", pretty_print::print_expression(&ast));
```

Output:
```
BinaryOp: Add
  left:
    └─ IntegerLiteral: 2
  right:
    └─ BinaryOp: Multiply
      left:
        └─ IntegerLiteral: 3
      right:
        └─ IntegerLiteral: 4
```

### Using the Display Trait

The `Expression` type implements `Display`, so you can use it directly with formatting macros:

```rust
println!("{}", ast);  // Same as print_expression(&ast)
```

### ASCII-Only Mode

For terminals that don't support Unicode box-drawing characters:

```rust
use nyx_parser::pretty_print;

let output = pretty_print::print_expression_ascii(&ast);
println!("{}", output);
```

Output:
```
BinaryOp: Add
  left:
    `-- IntegerLiteral: 2
  right:
    `-- BinaryOp: Multiply
      left:
        `-- IntegerLiteral: 3
      right:
        `-- IntegerLiteral: 4
```

## Advanced Configuration

### Custom Indentation

Adjust the indentation size for deeper or shallower nesting:

```rust
use nyx_parser::pretty_print::PrettyPrinter;

let mut printer = PrettyPrinter::new()
    .with_indent_size(4);  // 4 spaces instead of default 2

let output = printer.print_expression(&ast);
```

### Creating Custom Printers

```rust
use nyx_parser::pretty_print::PrettyPrinter;

// Default configuration (2-space indent, Unicode)
let mut printer1 = PrettyPrinter::new();

// ASCII-only configuration
let mut printer2 = PrettyPrinter::ascii();

// Custom configuration
let mut printer3 = PrettyPrinter::new()
    .with_indent_size(4);
```

## API Reference

### Functions

#### `print_expression(expr: &Expression) -> String`
Convenience function to pretty print an expression with default settings (2-space indent, Unicode).

#### `print_expression_ascii(expr: &Expression) -> String`
Convenience function to pretty print an expression using ASCII-only characters.

### Struct: `PrettyPrinter`

The main pretty printing engine.

#### Methods

- `new() -> Self` - Create a printer with default settings
- `ascii() -> Self` - Create a printer with ASCII-only output
- `with_indent_size(size: usize) -> Self` - Set custom indentation size
- `print_expression(&mut self, expr: &Expression) -> String` - Format an expression

## Expression Type Support

### Literals
- **IntegerLiteral**: Displays the value with base and suffix information
- **FloatLiteral**: Shows the value with optional exponent and suffix
- **BooleanLiteral**: Shows `true` or `false`
- **CharLiteral**: Displays character in single quotes
- **StringLiteral**: Displays string in double quotes
- **OkLiteral**: Shows the unit type literal

### Operations
- **BinaryOp**: Shows operator type and both operands with clear hierarchy
- **UnaryOp**: Displays operator and single operand

### Collections
- **ArrayLiteral**: Lists all elements with count, shows "(empty)" for empty arrays

### Other
- **Parenthesized**: Clearly indicates parenthesized sub-expressions
- **Identifier**: Shows variable/identifier names

## Design Principles

### Separation of Concerns
The pretty printer is implemented as a separate module from the AST definitions, maintaining clear separation between:
- **Structure**: AST node definitions in `ast.rs`
- **Presentation**: Visualization logic in `pretty_print.rs`

### Debugging Focus
The output is optimized for human readability during debugging, not for programmatic parsing. It includes:
- Clear labels for each node type
- Hierarchical indentation showing structure
- Visual branches showing relationships

### Performance Considerations
The pretty printer uses string building with `write!` macros for efficient formatting. While suitable for debugging output, it's not optimized for extremely large ASTs in production scenarios.

## Examples

See the `examples/` directory for complete demonstrations:
- `parse_example.rs` - Shows pretty printing of various parsed expressions
- `pretty_print_demo.rs` - Demonstrates all pretty printer features and configurations

## Testing

The module includes comprehensive tests covering:
- All expression types
- Nested expressions
- Empty collections
- Custom configurations
- ASCII vs Unicode output

Run tests with:
```bash
cargo test --package nyx-parser pretty_print
```

## Future Enhancements

Potential improvements for future versions:
- Color output support for terminals
- Collapsible sections for large trees
- Export to other formats (JSON, XML, Graphviz)
- Line number and source location information
- Performance optimizations for large ASTs
