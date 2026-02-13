use nyx_lexer::{IndentLexer, Token};

fn main() {
    let source = r#"fn example
    if condition
        let x = 10
        let y = 20
    let z = 30
"#;

    println!("Source code:");
    println!("{}", source);
    println!("\nTokens:");
    
    let mut lexer = IndentLexer::new(source);
    
    while let Some(result) = lexer.next() {
        match result {
            Ok(token) => {
                match &token {
                    Token::Indent => println!("  -> INDENT"),
                    Token::Dedent => println!("  -> DEDENT"),
                    Token::Newline => println!("  -> NEWLINE"),
                    Token::Ident(name) => println!("  -> Ident({})", name),
                    Token::IntegerLiteral(lit) => println!("  -> Integer({})", lit),
                    other => println!("  -> {:?}", other),
                }
            }
            Err(_) => println!("  -> ERROR"),
        }
    }
}
