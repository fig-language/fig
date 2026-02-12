// Demonstration of the AST pretty printer features
//
// This example showcases the different formatting options available
// in the pretty_print module for debugging AST structures.
//
// To run this example:
// cargo run --example pretty_print_demo

use nyx_parser::{Lexer, parser::ExpressionParser, pretty_print::{PrettyPrinter, print_expression, print_expression_ascii}};

fn main() {
    println!("Nyx AST Pretty Printer Demo");
    println!("{}", "=".repeat(70));
    println!();

    // Parse a complex expression
    let input = "(2 + 3) * 4 - [1, 2, 3]";
    println!("Input Expression: {}", input);
    println!();
    
    let lexer = Lexer::new(input);
    let parser = ExpressionParser::new();
    let ast = parser.parse(lexer).expect("Failed to parse");

    // Demo 1: Default pretty printing with Unicode box characters
    println!("1. Default Pretty Print (Unicode):");
    println!("{}", "-".repeat(70));
    println!("{}", print_expression(&ast));

    // Demo 2: ASCII-only mode (for terminals that don't support Unicode)
    println!("2. ASCII-Only Mode:");
    println!("{}", "-".repeat(70));
    println!("{}", print_expression_ascii(&ast));

    // Demo 3: Custom indentation size
    println!("3. Custom Indentation (4 spaces):");
    println!("{}", "-".repeat(70));
    let mut custom_printer = PrettyPrinter::new().with_indent_size(4);
    println!("{}", custom_printer.print_expression(&ast));

    // Demo 4: Using the Display trait
    println!("4. Using Display Trait:");
    println!("{}", "-".repeat(70));
    println!("{}", ast);

    // Demo 5: Various expression types
    println!("5. Various Expression Types:");
    println!("{}", "-".repeat(70));
    
    let examples = vec![
        ("Boolean", "true && false"),
        ("Unary", "-42"),
        ("Comparison", "x >= y"),
        ("Bitwise", "5 & 3 | 1"),
        ("Shift", "1 << 4"),
        ("Array", "[1, 2, 3, 4, 5]"),
    ];

    for (name, expr) in examples {
        println!("\n{} expression: '{}'", name, expr);
        let lexer = Lexer::new(expr);
        let parser = ExpressionParser::new();
        if let Ok(ast) = parser.parse(lexer) {
            println!("{}", print_expression(&ast));
        }
    }

    println!("{}", "=".repeat(70));
    println!("Demo completed!");
}
