// Debug test to see token stream for enum with where clause

use nyx_parser::Lexer;

fn main() {
    let input = "struct Container[T, U]
    where
        T: Clone
        U: Copy
    first: T
    second: U";
    
    println!("Input:");
    println!("{}", input);
    println!("\nToken stream:");
    
    let lexer = Lexer::new(input);
    for result in lexer {
        match result {
            Ok((pos, token, end_pos)) => {
                println!("{:?} at {}-{}", token, pos, end_pos);
            }
            Err(e) => {
                println!("Error: {:?}", e);
            }
        }
    }
    
    println!("\n\nNow trying to parse as Struct:");
    let lexer2 = nyx_parser::Lexer::new(input);
    match nyx_parser::parser::StructParser::new().parse(lexer2) {
        Ok(s) => {
            println!("✓ Parsed successfully!");
            println!("  Name: {}", s.name());
            println!("  Generic params: {}", s.generic_params().len());
            println!("  Where clause: {}", s.where_clause().len());
            println!("  Fields: {}", s.fields().len());
        }
        Err(e) => {
            println!("✗ Parse error: {:?}", e);
        }
    }
}
