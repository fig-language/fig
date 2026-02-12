use lalrpop_util::lalrpop_mod;
use logos::{Logos, SpannedIter};
use nyx_lexer::Token;

pub mod ast;
pub mod pretty_print;

lalrpop_mod!(pub parser);

pub use parser::*;

#[derive(Default, Debug, Clone, PartialEq)]
pub enum LexicalError {
    #[default]
    InvalidToken,
}

impl std::fmt::Display for LexicalError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub type Spanned<Tok, Loc, Error> = Result<(Loc, Tok, Loc), Error>;

pub struct Lexer<'input> {
    token_stream: SpannedIter<'input, Token>,
}

impl<'input> Lexer<'input> {
    pub fn new(input: &'input str) -> Self {
        Self {
            token_stream: Token::lexer(input).spanned(),
        }
    }
}

impl<'input> Iterator for Lexer<'input> {
    type Item = Spanned<Token, usize, LexicalError>;

    fn next(&mut self) -> Option<Self::Item> {
        self.token_stream.next().map(|(token, span)| {
            match token {
                Ok(token) => Ok((span.start, token, span.end)),
                Err(_) => Err(LexicalError::InvalidToken),
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::*;

    #[test]
    fn test_lexer_integration() {
        let input = "fn main() {}";
        let lexer = Lexer::new(input);
        let tokens: Vec<_> = lexer.collect();
        assert!(!tokens.is_empty());
    }

    #[test]
    fn test_simple_integer() {
        let input = "42";
        let lexer = Lexer::new(input);
        let result = parser::ExpressionParser::new().parse(lexer);
        assert!(result.is_ok());
        let expr = result.unwrap();
        assert!(matches!(expr, Expression::IntegerLiteral(_)));
    }

    #[test]
    fn test_simple_boolean() {
        let input = "true";
        let lexer = Lexer::new(input);
        let result = parser::ExpressionParser::new().parse(lexer);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Expression::BooleanLiteral(true));
    }

    #[test]
    fn test_binary_add() {
        let input = "1 + 2";
        let lexer = Lexer::new(input);
        let result = parser::ExpressionParser::new().parse(lexer);
        assert!(result.is_ok());
        let expr = result.unwrap();
        if let Expression::BinaryOp(op) = expr {
            assert_eq!(*op.op(), BinaryOperator::Add);
        } else {
            panic!("Expected binary operation");
        }
    }

    #[test]
    fn test_precedence_mul_add() {
        let input = "2 + 3 * 4";
        let lexer = Lexer::new(input);
        let result = parser::ExpressionParser::new().parse(lexer);
        assert!(result.is_ok());
        // Should parse as 2 + (3 * 4), not (2 + 3) * 4
        let expr = result.unwrap();
        if let Expression::BinaryOp(add_op) = expr {
            assert_eq!(*add_op.op(), BinaryOperator::Add);
            // RHS should be a multiplication
            if let Expression::BinaryOp(mul_op) = add_op.rhs().as_ref() {
                assert_eq!(*mul_op.op(), BinaryOperator::Multiply);
            } else {
                panic!("Expected multiplication on RHS");
            }
        } else {
            panic!("Expected addition at root");
        }
    }

    #[test]
    fn test_unary_negation() {
        let input = "-5";
        let lexer = Lexer::new(input);
        let result = parser::ExpressionParser::new().parse(lexer);
        assert!(result.is_ok());
        let expr = result.unwrap();
        if let Expression::UnaryOp(op) = expr {
            assert_eq!(*op.op(), UnaryOperator::Negate);
        } else {
            panic!("Expected unary operation");
        }
    }

    #[test]
    fn test_logical_or() {
        let input = "true || false";
        let lexer = Lexer::new(input);
        let result = parser::ExpressionParser::new().parse(lexer);
        assert!(result.is_ok());
    }

    #[test]
    fn test_comparison() {
        let input = "5 < 10";
        let lexer = Lexer::new(input);
        let result = parser::ExpressionParser::new().parse(lexer);
        assert!(result.is_ok());
        let expr = result.unwrap();
        if let Expression::BinaryOp(op) = expr {
            assert_eq!(*op.op(), BinaryOperator::LessThan);
        } else {
            panic!("Expected comparison operation");
        }
    }

    #[test]
    fn test_parenthesized() {
        let input = "(2 + 3) * 4";
        let lexer = Lexer::new(input);
        let result = parser::ExpressionParser::new().parse(lexer);
        assert!(result.is_ok());
        // Should parse as (2 + 3) * 4
        let expr = result.unwrap();
        if let Expression::BinaryOp(mul_op) = expr {
            assert_eq!(*mul_op.op(), BinaryOperator::Multiply);
            // LHS should be parenthesized addition
            assert!(matches!(mul_op.lhs().as_ref(), Expression::Parenthesized(_)));
        } else {
            panic!("Expected multiplication at root");
        }
    }

    #[test]
    fn test_array_literal_empty() {
        let input = "[]";
        let lexer = Lexer::new(input);
        let result = parser::ExpressionParser::new().parse(lexer);
        assert!(result.is_ok());
        let expr = result.unwrap();
        if let Expression::ArrayLiteral(arr) = expr {
            assert_eq!(arr.elements().len(), 0);
        } else {
            panic!("Expected array literal");
        }
    }

    #[test]
    fn test_array_literal_with_elements() {
        let input = "[1, 2, 3]";
        let lexer = Lexer::new(input);
        let result = parser::ExpressionParser::new().parse(lexer);
        assert!(result.is_ok());
        let expr = result.unwrap();
        if let Expression::ArrayLiteral(arr) = expr {
            assert_eq!(arr.elements().len(), 3);
        } else {
            panic!("Expected array literal");
        }
    }

    #[test]
    fn test_bitwise_operations() {
        let input = "5 & 3 | 1";
        let lexer = Lexer::new(input);
        let result = parser::ExpressionParser::new().parse(lexer);
        assert!(result.is_ok());
    }

    #[test]
    fn test_shift_operations() {
        let input = "1 << 2";
        let lexer = Lexer::new(input);
        let result = parser::ExpressionParser::new().parse(lexer);
        assert!(result.is_ok());
        let expr = result.unwrap();
        if let Expression::BinaryOp(op) = expr {
            assert_eq!(*op.op(), BinaryOperator::ShiftLeft);
        } else {
            panic!("Expected shift operation");
        }
    }

    // Type parsing tests
    #[test]
    fn test_parse_primitive_type_u8() {
        let input = "u8";
        let lexer = Lexer::new(input);
        let result = parser::TypeParser::new().parse(lexer);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Type::U8);
    }

    #[test]
    fn test_parse_primitive_type_i32() {
        let input = "i32";
        let lexer = Lexer::new(input);
        let result = parser::TypeParser::new().parse(lexer);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Type::I32);
    }

    #[test]
    fn test_parse_primitive_type_f64() {
        let input = "f64";
        let lexer = Lexer::new(input);
        let result = parser::TypeParser::new().parse(lexer);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Type::F64);
    }

    #[test]
    fn test_parse_primitive_type_bool() {
        let input = "bool";
        let lexer = Lexer::new(input);
        let result = parser::TypeParser::new().parse(lexer);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Type::Bool);
    }

    #[test]
    fn test_parse_primitive_type_ok() {
        let input = "ok";
        let lexer = Lexer::new(input);
        let result = parser::TypeParser::new().parse(lexer);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Type::Ok);
    }

    #[test]
    fn test_parse_raw_pointer() {
        let input = "*raw";
        let lexer = Lexer::new(input);
        let result = parser::TypeParser::new().parse(lexer);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Type::RawPointer);
    }

    #[test]
    fn test_parse_typed_pointer() {
        let input = "*u32";
        let lexer = Lexer::new(input);
        let result = parser::TypeParser::new().parse(lexer);
        assert!(result.is_ok());
        if let Type::TypedPointer(inner) = result.unwrap() {
            assert_eq!(*inner, Type::U32);
        } else {
            panic!("Expected typed pointer");
        }
    }

    #[test]
    fn test_parse_nested_pointer() {
        let input = "**i64";
        let lexer = Lexer::new(input);
        let result = parser::TypeParser::new().parse(lexer);
        assert!(result.is_ok());
        if let Type::TypedPointer(outer) = result.unwrap() {
            if let Type::TypedPointer(inner) = *outer {
                assert_eq!(*inner, Type::I64);
            } else {
                panic!("Expected nested typed pointer");
            }
        } else {
            panic!("Expected typed pointer");
        }
    }

    #[test]
    fn test_parse_array_type() {
        let input = "[4]u8";
        let lexer = Lexer::new(input);
        let result = parser::TypeParser::new().parse(lexer);
        assert!(result.is_ok());
        if let Type::Array { size, element_type } = result.unwrap() {
            assert_eq!(size, Some(4));
            assert_eq!(*element_type, Type::U8);
        } else {
            panic!("Expected array type");
        }
    }
}