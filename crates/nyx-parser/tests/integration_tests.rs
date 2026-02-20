// Integration tests for the Nyx parser using datatest-stable + insta
// Each .nyx file in tests/valid/ will be tested automatically
// AST snapshots are stored in .snap.yml files using insta

use datatest_stable::Utf8Path;
use nyx_parser::{Lexer, SourceFileParser};

fn parser_test(path: &Utf8Path, contents: String) -> datatest_stable::Result<()> {
    let lexer = Lexer::new(&contents);
    let parser = SourceFileParser::new();

    // Parse the entire file
    let result = parser.parse(lexer);

    // Assert that parsing succeeded
    let ast = match result {
        Ok(ast) => ast,
        Err(e) => return Err(format!("Parse error in {}: {:?}", path, e).into()),
    };

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
    insta::assert_yaml_snapshot!(snapshot_name, ast);

    Ok(())
}

datatest_stable::harness! {
    { test = parser_test, root = "../../tests/valid", pattern = r"\.nyx$" },
}
