// Example demonstrating type alias parsing with where clauses
//
// This example shows how the parser supports type aliases with:
// - Generic parameters
// - Bounded type parameters
// - Const parameters
// - Where clauses with indentation
//
// To run this example:
// cargo run --example type_alias_demo -p nyx-parser

use nyx_parser::{Lexer, parser};

fn main() {
    println!("Nyx Parser - Type Alias Parsing Demo");
    println!("{}", "=".repeat(60));
    println!();

    // Example 1: Simple type alias
    println!("Example 1: Simple type alias");
    println!("{}", "-".repeat(60));
    let source1 = "type MyInt = i32";
    println!("Source: {}", source1);
    let lexer = Lexer::new(source1);
    match parser::TypeAliasParser::new().parse(lexer) {
        Ok(type_alias) => {
            println!("✓ Parsed successfully!");
            println!("  Name: {}", type_alias.name());
            println!("  Aliased type: {:?}", type_alias.aliased_type());
        }
        Err(e) => println!("✗ Parse error: {:?}", e),
    }
    println!();

    // Example 2: Type alias with generic parameters
    println!("Example 2: Type alias with generic parameters");
    println!("{}", "-".repeat(60));
    let source2 = "type MyVec[T] = []T";
    println!("Source: {}", source2);
    let lexer = Lexer::new(source2);
    match parser::TypeAliasParser::new().parse(lexer) {
        Ok(type_alias) => {
            println!("✓ Parsed successfully!");
            println!("  Name: {}", type_alias.name());
            println!("  Generic params: {} parameter(s)", type_alias.generic_params().len());
        }
        Err(e) => println!("✗ Parse error: {:?}", e),
    }
    println!();

    // Example 3: Type alias with bounded generic
    println!("Example 3: Type alias with bounded generic parameter");
    println!("{}", "-".repeat(60));
    let source3 = "type MyRef[T: Copy] = &T";
    println!("Source: {}", source3);
    let lexer = Lexer::new(source3);
    match parser::TypeAliasParser::new().parse(lexer) {
        Ok(type_alias) => {
            println!("✓ Parsed successfully!");
            println!("  Name: {}", type_alias.name());
            println!("  Generic params: {} parameter(s)", type_alias.generic_params().len());
        }
        Err(e) => println!("✗ Parse error: {:?}", e),
    }
    println!();

    // Example 4: Type alias with const parameter
    println!("Example 4: Type alias with const parameter");
    println!("{}", "-".repeat(60));
    let source4 = "type MyArray[const N: usize] = i32";
    println!("Source: {}", source4);
    let lexer = Lexer::new(source4);
    match parser::TypeAliasParser::new().parse(lexer) {
        Ok(type_alias) => {
            println!("✓ Parsed successfully!");
            println!("  Name: {}", type_alias.name());
            println!("  Generic params: {} parameter(s)", type_alias.generic_params().len());
        }
        Err(e) => println!("✗ Parse error: {:?}", e),
    }
    println!();

    // Example 5: Type alias with where clause
    println!("Example 5: Type alias with where clause");
    println!("{}", "-".repeat(60));
    let source5 = "type MyResult[T, E] = i32
where
    T: Clone
    E: Copy";
    println!("Source:");
    println!("{}", source5);
    let lexer = Lexer::new(source5);
    match parser::TypeAliasParser::new().parse(lexer) {
        Ok(type_alias) => {
            println!("✓ Parsed successfully!");
            println!("  Name: {}", type_alias.name());
            println!("  Generic params: {}", type_alias.generic_params().len());
            println!("  Where clause constraints: {}", type_alias.where_clause().len());
        }
        Err(e) => println!("✗ Parse error: {:?}", e),
    }
    println!();

    // Example 6: Type alias with complex where clause
    println!("Example 6: Type alias with complex where clause");
    println!("{}", "-".repeat(60));
    let source6 = "type ComplexType[T, U, V] = *T
where
    T: Clone + Send
    U: Copy
    V: Debug + Display";
    println!("Source:");
    println!("{}", source6);
    let lexer = Lexer::new(source6);
    match parser::TypeAliasParser::new().parse(lexer) {
        Ok(type_alias) => {
            println!("✓ Parsed successfully!");
            println!("  Name: {}", type_alias.name());
            println!("  Generic params: {}", type_alias.generic_params().len());
            println!("  Where clause constraints: {}", type_alias.where_clause().len());
        }
        Err(e) => println!("✗ Parse error: {:?}", e),
    }
    println!();

    // Example 7: Mixed parameters with where clause
    println!("Example 7: Mixed type and const parameters with where clause");
    println!("{}", "-".repeat(60));
    let source7 = "type MixedType[T: Clone, const N: usize] = *T
where
    T: Send";
    println!("Source:");
    println!("{}", source7);
    let lexer = Lexer::new(source7);
    match parser::TypeAliasParser::new().parse(lexer) {
        Ok(type_alias) => {
            println!("✓ Parsed successfully!");
            println!("  Name: {}", type_alias.name());
            println!("  Generic params: {}", type_alias.generic_params().len());
            println!("  Where clause constraints: {}", type_alias.where_clause().len());
        }
        Err(e) => println!("✗ Parse error: {:?}", e),
    }
    println!();

    println!("{}", "=".repeat(60));
    println!("All examples completed!");
}
