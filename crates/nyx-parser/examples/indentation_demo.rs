// Example demonstrating indentation-aware token parsing
//
// This example shows how the parser now supports INDENT, DEDENT, and NEWLINE tokens
// for Python-like indentation-based block structure.
//
// To run this example:
// cargo run --example indentation_demo

use nyx_parser::Lexer;
use nyx_lexer::Token;

fn main() {
    println!("Nyx Parser - Indentation-Aware Tokenization Demo");
    println!("{}", "=".repeat(60));
    println!();

    // Example 1: Basic indentation
    println!("Example 1: Basic indentation");
    println!("{}", "-".repeat(60));
    let source1 = "let x = 10
let y = 20";
    println!("Source:");
    println!("{}", source1);
    println!("\nTokens:");
    demonstrate_tokens(source1);
    println!();

    // Example 2: Nested indentation
    println!("Example 2: Nested indentation (simulating if-statement)");
    println!("{}", "-".repeat(60));
    let source2 = "if condition
    let x = 1
    let y = 2
let z = 3";
    println!("Source:");
    println!("{}", source2);
    println!("\nTokens:");
    demonstrate_tokens(source2);
    println!();

    // Example 3: Multiple indent/dedent levels
    println!("Example 3: Multiple nesting levels");
    println!("{}", "-".repeat(60));
    let source3 = "fn outer
    fn middle
        fn inner
            let x = 1
let y = 2";
    println!("Source:");
    println!("{}", source3);
    println!("\nTokens:");
    demonstrate_tokens(source3);
    println!();

    // Example 4: Blank lines (should be ignored for indentation)
    println!("Example 4: Blank lines (ignored for indentation)");
    println!("{}", "-".repeat(60));
    let source4 = "fn test

    let x = 1

let y = 2";
    println!("Source:");
    println!("{}", source4);
    println!("\nTokens:");
    demonstrate_tokens(source4);
    println!();

    // Example 5: Complex expression with indentation
    println!("Example 5: Complex expression with indentation");
    println!("{}", "-".repeat(60));
    let source5 = "if x > 10
    let a = x + 5
    let b = a * 2
        let nested = b / 2
    let c = 100";
    println!("Source:");
    println!("{}", source5);
    println!("\nTokens:");
    demonstrate_tokens(source5);
}

fn demonstrate_tokens(source: &str) {
    let lexer = Lexer::new(source);
    let mut indent_level: usize = 0;
    
    for (idx, token_result) in lexer.enumerate() {
        match token_result {
            Ok((start, token, end)) => {
                match &token {
                    Token::Indent => {
                        indent_level += 1;
                        println!("{:3}: [{:4}..{:4}] {:>4} INDENT", 
                                 idx, start, end, ">>>");
                    }
                    Token::Dedent => {
                        indent_level = indent_level.saturating_sub(1);
                        println!("{:3}: [{:4}..{:4}] {:>4} DEDENT", 
                                 idx, start, end, "<<<");
                    }
                    Token::Newline => {
                        println!("{:3}: [{:4}..{:4}] {:>4} NEWLINE", 
                                 idx, start, end, "↵");
                    }
                    Token::Ident(name) => {
                        println!("{:3}: [{:4}..{:4}] {}Ident({})", 
                                 idx, start, end, "    ".repeat(indent_level), name);
                    }
                    Token::IntegerLiteral(lit) => {
                        println!("{:3}: [{:4}..{:4}] {}Integer({})", 
                                 idx, start, end, "    ".repeat(indent_level), lit);
                    }
                    Token::Eq => {
                        println!("{:3}: [{:4}..{:4}] {}Eq", 
                                 idx, start, end, "    ".repeat(indent_level));
                    }
                    Token::Plus => {
                        println!("{:3}: [{:4}..{:4}] {}Plus", 
                                 idx, start, end, "    ".repeat(indent_level));
                    }
                    Token::Star => {
                        println!("{:3}: [{:4}..{:4}] {}Star", 
                                 idx, start, end, "    ".repeat(indent_level));
                    }
                    Token::Slash => {
                        println!("{:3}: [{:4}..{:4}] {}Slash", 
                                 idx, start, end, "    ".repeat(indent_level));
                    }
                    Token::Gt => {
                        println!("{:3}: [{:4}..{:4}] {}GreaterThan", 
                                 idx, start, end, "    ".repeat(indent_level));
                    }
                    other => {
                        println!("{:3}: [{:4}..{:4}] {}{:?}", 
                                 idx, start, end, "    ".repeat(indent_level), other);
                    }
                }
            }
            Err(err) => {
                println!("{:3}: ERROR - {:?}", idx, err);
            }
        }
    }
}
