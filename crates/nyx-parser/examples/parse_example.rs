// Example usage of the Nyx expression parser
//
// This file demonstrates how to parse expressions using the lexer and parser,
// and how to pretty print the resulting AST for debugging purposes.
//
// To run this example:
// cargo run --example parse_example

use nyx_parser::{Lexer, parser::ExpressionParser, pretty_print};

fn main() {
    // Example expressions to parse
    let test_cases = vec![
        "42",
        "3.14",
        "true",
        "false",
        "ok",
        "1 + 2",
        "2 * 3 + 4",
        "(2 + 3) * 4",
        "5 < 10",
        "true && false",
        "true || false",
        "-5",
        "!true",
        "~0xFF",
        "[1, 2, 3]",
        "[]",
        "1 + 2 * 3 - 4 / 2",
        "5 & 3 | 1 ^ 2",
        "1 << 4",
        "100 >> 2",
        "a == b",
        "x != y",
        "a >= b",
        "x <= y",
    ];

    println!("Nyx Expression Parser Examples");
    println!("{}", "=".repeat(50));
    println!();

    for (i, input) in test_cases.iter().enumerate() {
        println!("[{}] Parsing: {}", i + 1, input);
        println!("{}", "-".repeat(50));
        
        let lexer = Lexer::new(input);
        let parser = ExpressionParser::new();
        
        match parser.parse(lexer) {
            Ok(ast) => {
                println!("✓ Success!");
                println!("\nPretty-printed AST:");
                println!("{}", pretty_print::print_expression(&ast));
            }
            Err(e) => {
                println!("✗ Parse error:");
                println!("{:?}", e);
            }
        }
        
        println!();
    }

    println!("{}", "=".repeat(50));
    println!("All examples completed!");
}
