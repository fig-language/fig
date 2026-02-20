// Integration tests for the Nyx lexer using datatest-stable + insta
// Each .nyx file in tests/valid/ will be tested automatically
// Token snapshots are stored in .snap.yml files using insta

use datatest_stable::Utf8Path;
use nyx_lexer::{IndentLexer, LexicalError, Token};
use std::path::PathBuf;

fn lexer_test(path: &Utf8Path, contents: String) -> datatest_stable::Result<()> {
    // Tokenize the entire file
    let mut lexer = IndentLexer::new(&contents);
    let mut tokens = Vec::new();
    let mut errors = Vec::new();

    while let Some(result) = lexer.next() {
        match result {
            Ok(token) => tokens.push(token),
            Err(e) => errors.push(e),
        }
    }

    // Check for lexer errors
    if !errors.is_empty() {
        // Format errors with detailed information
        let mut error_details = String::new();
        for (i, error) in errors.iter().enumerate() {
            error_details.push_str(&format!("\n  Error {}: {}", i + 1, error));
            if let Some(span) = error.span() {
                let (line, col) = LexicalError::position_from_source(&contents, span.start);
                error_details.push_str(&format!(" (line {}, column {})", line, col));
                // Show the problematic text
                let snippet = contents.get(span.clone()).unwrap_or("<invalid span>");
                error_details.push_str(&format!("\n     Text: {:?}", snippet));
            }
        }
        return Err(format!("Lexer errors in {}:{}", path, error_details).into());
    }

    // Create snapshot name from the test path
    // Convert path like "../../tests/valid/comments/single_line.nyx" 
    // to snapshot name "comments__single_line"
    let snapshot_name = path
        .as_str()
        .trim_start_matches("../../tests/valid/")
        .trim_end_matches(".nyx")
        .replace('/', "__")
        .replace('\\', "__");

    // Assert snapshot using insta with YAML format
    insta::assert_yaml_snapshot!(snapshot_name, tokens);

    Ok(())
}

datatest_stable::harness! {
    { test = lexer_test, root = "../../tests/valid", pattern = r"\.nyx$" },
}
