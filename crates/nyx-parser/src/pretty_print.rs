//! Pretty printing utilities for the Nyx Abstract Syntax Tree
//!
//! This module provides detailed visualization of AST structures for debugging purposes.

use crate::ast::*;
use std::fmt::Write;

/// Pretty printer for AST nodes with configurable indentation and formatting
pub struct PrettyPrinter {
    /// Current indentation level
    indent_level: usize,
    /// Number of spaces per indentation level
    indent_size: usize,
    /// Use unicode box-drawing characters for tree structure
    use_unicode: bool,
}

impl Default for PrettyPrinter {
    fn default() -> Self {
        Self::new()
    }
}

impl PrettyPrinter {
    /// Create a new pretty printer with default settings
    pub fn new() -> Self {
        Self {
            indent_level: 0,
            indent_size: 2,
            use_unicode: true,
        }
    }

    /// Create a pretty printer with ASCII-only output
    pub fn ascii() -> Self {
        Self {
            indent_level: 0,
            indent_size: 2,
            use_unicode: false,
        }
    }

    /// Set the number of spaces per indentation level
    pub fn with_indent_size(mut self, size: usize) -> Self {
        self.indent_size = size;
        self
    }

    /// Pretty print an expression to a string
    pub fn print_expression(&mut self, expr: &Expression) -> String {
        let mut output = String::new();
        self.format_expression(expr, &mut output, true);
        output
    }

    /// Get the current indentation string
    fn indent(&self) -> String {
        " ".repeat(self.indent_level * self.indent_size)
    }

    /// Get the tree branch characters
    fn branch(&self) -> &'static str {
        if self.use_unicode { "├─ " } else { "|-- " }
    }

    /// Get the last branch characters
    fn last_branch(&self) -> &'static str {
        if self.use_unicode { "└─ " } else { "`-- " }
    }

    /// Format an expression into the output buffer
    fn format_expression(&mut self, expr: &Expression, output: &mut String, is_last: bool) {
        let prefix = if self.indent_level == 0 {
            String::new()
        } else {
            format!("{}{}", self.indent(), if is_last { self.last_branch() } else { self.branch() })
        };

        match expr {
            Expression::IntegerLiteral(lit) => {
                writeln!(output, "{}IntegerLiteral: {}", prefix, lit).unwrap();
            }
            
            Expression::FloatLiteral(lit) => {
                writeln!(output, "{}FloatLiteral: {}", prefix, lit).unwrap();
            }

            Expression::BooleanLiteral(value) => {
                writeln!(output, "{}BooleanLiteral: {}", prefix, value).unwrap();
            }

            Expression::CharLiteral(ch) => {
                writeln!(output, "{}CharLiteral: '{}'", prefix, ch).unwrap();
            }

            Expression::StringLiteral(s) => {
                writeln!(output, "{}StringLiteral: \"{}\"", prefix, s).unwrap();
            }

            Expression::OkLiteral => {
                writeln!(output, "{}OkLiteral", prefix).unwrap();
            }

            Expression::Identifier(name) => {
                writeln!(output, "{}Identifier: {}", prefix, name).unwrap();
            }

            Expression::ArrayLiteral(arr) => {
                writeln!(output, "{}ArrayLiteral", prefix).unwrap();
                self.indent_level += 1;
                let elements = arr.elements();
                if elements.is_empty() {
                    writeln!(output, "{}└─ (empty)", self.indent()).unwrap();
                } else {
                    writeln!(output, "{}elements: {} item(s)", self.indent(), elements.len()).unwrap();
                    self.indent_level += 1;
                    for (i, elem) in elements.iter().enumerate() {
                        let is_last_elem = i == elements.len() - 1;
                        self.format_expression(elem, output, is_last_elem);
                    }
                    self.indent_level -= 1;
                }
                self.indent_level -= 1;
            }

            Expression::BinaryOp(op) => {
                writeln!(output, "{}BinaryOp: {:?}", prefix, op.op()).unwrap();
                self.indent_level += 1;
                
                writeln!(output, "{}left:", self.indent()).unwrap();
                self.indent_level += 1;
                self.format_expression(op.lhs(), output, true);
                self.indent_level -= 1;
                
                writeln!(output, "{}right:", self.indent()).unwrap();
                self.indent_level += 1;
                self.format_expression(op.rhs(), output, true);
                self.indent_level -= 1;
                
                self.indent_level -= 1;
            }

            Expression::UnaryOp(op) => {
                writeln!(output, "{}UnaryOp: {:?}", prefix, op.op()).unwrap();
                self.indent_level += 1;
                writeln!(output, "{}operand:", self.indent()).unwrap();
                self.indent_level += 1;
                self.format_expression(op.operand(), output, true);
                self.indent_level -= 1;
                self.indent_level -= 1;
            }

            Expression::Parenthesized(inner) => {
                writeln!(output, "{}Parenthesized", prefix).unwrap();
                self.indent_level += 1;
                self.format_expression(inner, output, true);
                self.indent_level -= 1;
            }
        }
    }
}

/// Convenience function to pretty print an expression with default settings
pub fn print_expression(expr: &Expression) -> String {
    PrettyPrinter::new().print_expression(expr)
}

/// Convenience function to pretty print an expression with ASCII-only output
pub fn print_expression_ascii(expr: &Expression) -> String {
    PrettyPrinter::ascii().print_expression(expr)
}

impl std::fmt::Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", print_expression(self))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use nyx_lexer::{IntegerLiteral, Base};

    #[test]
    fn test_print_integer_literal() {
        let lit = IntegerLiteral::builder()
            .base(Base::Decimal)
            .digits("42".to_string())
            .suffix(None)
            .build()
            .unwrap();
        let expr = Expression::IntegerLiteral(lit);
        let output = print_expression(&expr);
        assert!(output.contains("IntegerLiteral"));
        assert!(output.contains("42"));
    }

    #[test]
    fn test_print_boolean() {
        let expr = Expression::BooleanLiteral(true);
        let output = print_expression(&expr);
        assert!(output.contains("BooleanLiteral: true"));
    }

    #[test]
    fn test_print_binary_op() {
        let left = Box::new(Expression::IntegerLiteral(
            IntegerLiteral::builder()
                .digits("1".to_string())
                .build()
                .unwrap()
        ));
        let right = Box::new(Expression::IntegerLiteral(
            IntegerLiteral::builder()
                .digits("2".to_string())
                .build()
                .unwrap()
        ));
        let expr = Expression::BinaryOp(BinaryOpExpr::new(left, BinaryOperator::Add, right));
        let output = print_expression(&expr);
        assert!(output.contains("BinaryOp: Add"));
        assert!(output.contains("left:"));
        assert!(output.contains("right:"));
    }

    #[test]
    fn test_print_unary_op() {
        let operand = Box::new(Expression::IntegerLiteral(
            IntegerLiteral::builder()
                .digits("5".to_string())
                .build()
                .unwrap()
        ));
        let expr = Expression::UnaryOp(UnaryOpExpr::new(UnaryOperator::Negate, operand));
        let output = print_expression(&expr);
        assert!(output.contains("UnaryOp: Negate"));
        assert!(output.contains("operand:"));
    }

    #[test]
    fn test_print_array_literal() {
        let elements = vec![
            Expression::IntegerLiteral(
                IntegerLiteral::builder().digits("1".to_string()).build().unwrap()
            ),
            Expression::IntegerLiteral(
                IntegerLiteral::builder().digits("2".to_string()).build().unwrap()
            ),
            Expression::IntegerLiteral(
                IntegerLiteral::builder().digits("3".to_string()).build().unwrap()
            ),
        ];
        let expr = Expression::ArrayLiteral(ArrayLiteralExpr::new(elements));
        let output = print_expression(&expr);
        assert!(output.contains("ArrayLiteral"));
        assert!(output.contains("elements: 3 item(s)"));
    }

    #[test]
    fn test_print_empty_array() {
        let expr = Expression::ArrayLiteral(ArrayLiteralExpr::new(vec![]));
        let output = print_expression(&expr);
        assert!(output.contains("ArrayLiteral"));
        assert!(output.contains("(empty)"));
    }

    #[test]
    fn test_print_nested_expression() {
        // (1 + 2) * 3
        let add_left = Box::new(Expression::IntegerLiteral(
            IntegerLiteral::builder().digits("1".to_string()).build().unwrap()
        ));
        let add_right = Box::new(Expression::IntegerLiteral(
            IntegerLiteral::builder().digits("2".to_string()).build().unwrap()
        ));
        let add_expr = Expression::BinaryOp(BinaryOpExpr::new(add_left, BinaryOperator::Add, add_right));
        
        let mul_left = Box::new(Expression::Parenthesized(Box::new(add_expr)));
        let mul_right = Box::new(Expression::IntegerLiteral(
            IntegerLiteral::builder().digits("3".to_string()).build().unwrap()
        ));
        let expr = Expression::BinaryOp(BinaryOpExpr::new(mul_left, BinaryOperator::Multiply, mul_right));
        
        let output = print_expression(&expr);
        assert!(output.contains("BinaryOp: Multiply"));
        assert!(output.contains("Parenthesized"));
        assert!(output.contains("BinaryOp: Add"));
    }

    #[test]
    fn test_ascii_output() {
        let expr = Expression::BooleanLiteral(true);
        let output = print_expression_ascii(&expr);
        // ASCII mode should not contain unicode box characters
        assert!(!output.contains("├"));
        assert!(!output.contains("└"));
    }

    #[test]
    fn test_custom_indent_size() {
        let expr = Expression::BooleanLiteral(false);
        let output = PrettyPrinter::new().with_indent_size(4).print_expression(&expr);
        assert!(output.contains("BooleanLiteral: false"));
    }

    #[test]
    fn test_display_trait() {
        let expr = Expression::Identifier("my_var".to_string());
        let output = format!("{}", expr);
        assert!(output.contains("Identifier: my_var"));
    }
}
