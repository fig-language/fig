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
        self.token_stream.next().map(|(token, span)| match token {
            Ok(token) => Ok((span.start, token, span.end)),
            Err(_) => Err(LexicalError::InvalidToken),
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
            assert!(matches!(
                mul_op.lhs().as_ref(),
                Expression::Parenthesized(_)
            ));
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
    fn test_address_of_expression() {
        let input = "&x";
        let lexer = Lexer::new(input);
        let result = parser::ExpressionParser::new().parse(lexer);
        assert!(result.is_ok());
        let expr = result.unwrap();
        if let Expression::UnaryOp(op) = expr {
            assert_eq!(*op.op(), UnaryOperator::AddressOf);
            assert!(matches!(&**op.operand(), Expression::Identifier(_)));
        } else {
            panic!("Expected AddressOf unary operation");
        }
    }

    #[test]
    fn test_dereference_expression() {
        let input = "*y";
        let lexer = Lexer::new(input);
        let result = parser::ExpressionParser::new().parse(lexer);
        assert!(result.is_ok());
        let expr = result.unwrap();
        if let Expression::UnaryOp(op) = expr {
            assert_eq!(*op.op(), UnaryOperator::Dereference);
            assert!(matches!(&**op.operand(), Expression::Identifier(_)));
        } else {
            panic!("Expected Dereference unary operation");
        }
    }

    #[test]
    fn test_nested_address_of_dereference() {
        let input = "&*z";
        let lexer = Lexer::new(input);
        let result = parser::ExpressionParser::new().parse(lexer);
        assert!(result.is_ok());
        let expr = result.unwrap();
        if let Expression::UnaryOp(outer_op) = expr {
            assert_eq!(*outer_op.op(), UnaryOperator::AddressOf);
            if let Expression::UnaryOp(inner_op) = &**outer_op.operand() {
                assert_eq!(*inner_op.op(), UnaryOperator::Dereference);
                assert!(matches!(&**inner_op.operand(), Expression::Identifier(_)));
            } else {
                panic!("Expected nested Dereference operation");
            }
        } else {
            panic!("Expected outer AddressOf operation");
        }
    }

    #[test]
    fn test_dereference_address_of_expression() {
        let input = "*&z";
        let lexer = Lexer::new(input);
        let result = parser::ExpressionParser::new().parse(lexer);
        assert!(result.is_ok());
        let expr = result.unwrap();
        if let Expression::UnaryOp(outer_op) = expr {
            assert_eq!(*outer_op.op(), UnaryOperator::Dereference);
            if let Expression::UnaryOp(inner_op) = &**outer_op.operand() {
                assert_eq!(*inner_op.op(), UnaryOperator::AddressOf);
                assert!(matches!(&**inner_op.operand(), Expression::Identifier(_)));
            } else {
                panic!("Expected nested AddressOf operation");
            }
        } else {
            panic!("Expected outer Dereference operation");
        }
    }

    #[test]
    fn test_unary_with_binary_precedence() {
        let input = "&x + 5";
        let lexer = Lexer::new(input);
        let result = parser::ExpressionParser::new().parse(lexer);
        assert!(result.is_ok());
        let expr = result.unwrap();
        if let Expression::BinaryOp(binary_op) = expr {
            assert_eq!(*binary_op.op(), BinaryOperator::Add);
            if let Expression::UnaryOp(unary_op) = &**binary_op.lhs() {
                assert_eq!(*unary_op.op(), UnaryOperator::AddressOf);
                assert!(matches!(&**unary_op.operand(), Expression::Identifier(_)));
            } else {
                panic!("Expected AddressOf on LHS of binary op");
            }
            assert!(matches!(&**binary_op.rhs(), Expression::IntegerLiteral(_)));
        } else {
            panic!("Expected binary operation");
        }
    }

    #[test]
    fn test_unary_with_parenthesized() {
        let input = "&(x + 5)";
        let lexer = Lexer::new(input);
        let result = parser::ExpressionParser::new().parse(lexer);
        assert!(result.is_ok());
        let expr = result.unwrap();
        if let Expression::UnaryOp(unary_op) = expr {
            assert_eq!(*unary_op.op(), UnaryOperator::AddressOf);
            if let Expression::Parenthesized(inner_expr) = &**unary_op.operand() {
                assert!(matches!(&**inner_expr, Expression::BinaryOp(_)));
            } else {
                panic!("Expected parenthesized expression after AddressOf");
            }
        } else {
            panic!("Expected Unary operation");
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

    // Pointer type tests
    #[test]
    fn test_parse_raw_pointer() {
        let input = "*raw";
        let lexer = Lexer::new(input);
        let result = parser::TypeParser::new().parse(lexer);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Type::RawPointer);
    }

    #[test]
    fn test_parse_typed_pointer_u32() {
        let input = "*u32";
        let lexer = Lexer::new(input);
        let result = parser::TypeParser::new().parse(lexer);
        assert!(result.is_ok());
        if let Type::TypedPointer(inner) = result.unwrap() {
            assert_eq!(*inner, Type::U32);
        } else {
            panic!("Expected typed pointer to U32");
        }
    }

    #[test]
    fn test_parse_typed_pointer_bool() {
        let input = "*bool";
        let lexer = Lexer::new(input);
        let result = parser::TypeParser::new().parse(lexer);
        assert!(result.is_ok());
        if let Type::TypedPointer(inner) = result.unwrap() {
            assert_eq!(*inner, Type::Bool);
        } else {
            panic!("Expected typed pointer to Bool");
        }
    }

    #[test]
    fn test_parse_nested_pointer_i64() {
        let input = "**i64";
        let lexer = Lexer::new(input);
        let result = parser::TypeParser::new().parse(lexer);
        assert!(result.is_ok());
        if let Type::TypedPointer(outer) = result.unwrap() {
            if let Type::TypedPointer(inner) = *outer {
                assert_eq!(*inner, Type::I64);
            } else {
                panic!("Expected nested typed pointer to I64");
            }
        } else {
            panic!("Expected outer typed pointer");
        }
    }

    #[test]
    fn test_parse_triple_nested_pointer_u8() {
        let input = "***u8";
        let lexer = Lexer::new(input);
        let result = parser::TypeParser::new().parse(lexer);
        assert!(result.is_ok());
        if let Type::TypedPointer(outer) = result.unwrap() {
            if let Type::TypedPointer(middle) = *outer {
                if let Type::TypedPointer(inner) = *middle {
                    assert_eq!(*inner, Type::U8);
                } else {
                    panic!("Expected innermost typed pointer to U8");
                }
            } else {
                panic!("Expected middle typed pointer");
            }
        } else {
            panic!("Expected outer typed pointer");
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

    #[test]
    fn test_parse_generics_list() {
        // Test a generic parameter list with type parameters and bounds
        let input = "[T: Mappable[i32, i32], U: Copy + Clone, const N: usize]";
        let lexer = Lexer::new(input);
        let result = parser::GenericParameterListParser::new().parse(lexer);
        assert!(result.is_ok());
        let params = result.unwrap();
        assert_eq!(params.len(), 3);

        // First parameter: T with no bounds
        if let GenericParameter::Type { name, bounds } = &params[0] {
            assert_eq!(name, "T");
            assert_eq!(bounds.len(), 1);
            assert_eq!(bounds[0], Type::Named{ name: "Mappable".to_string(), generic_args: vec![Type::I32, Type::I32] });
        } else {
            panic!("Expected type parameter T");
        }

        // Second parameter: U with two bounds (Copy + Clone)
        if let GenericParameter::Type { name, bounds } = &params[1] {
            assert_eq!(name, "U");
            assert_eq!(bounds.len(), 2);
            assert_eq!(bounds[0], Type::Named{ name: "Copy".to_string(), generic_args: vec![] });
            assert_eq!(bounds[1], Type::Named{ name: "Clone".to_string(), generic_args: vec![] });
        } else {
            panic!("Expected type parameter U with bounds");
        }

        // Third parameter: const N: usize
        if let GenericParameter::Const { name, ty } = &params[2] {
            assert_eq!(name, "N");
            assert_eq!(ty, &Type::USize);
        } else {
            panic!("Expected const parameter N");
        }
    }

    // ========================================================================
    // Function Declaration Tests
    // ========================================================================

    #[test]
    fn test_simple_function_declaration() {
        let input = "fn add(a: i32, b: i32): i32";
        let lexer = Lexer::new(input);
        let result = parser::FunctionDeclarationParser::new().parse(lexer);
        assert!(result.is_ok());
        let func = result.unwrap();
        assert_eq!(func.name(), "add");
        assert_eq!(func.parameters().len(), 2);
        assert_eq!(func.generic_params().len(), 0);
        assert!(func.return_type().is_some());
        assert_eq!(func.return_type().as_ref().unwrap(), &Type::I32);
    }

    #[test]
    fn test_function_declaration_no_return_type() {
        let input = "fn main()";
        let lexer = Lexer::new(input);
        let result = parser::FunctionDeclarationParser::new().parse(lexer);
        assert!(result.is_ok());
        let func = result.unwrap();
        assert_eq!(func.name(), "main");
        assert_eq!(func.parameters().len(), 0);
        assert!(func.return_type().is_none());
    }

    #[test]
    fn test_function_declaration_with_arrow_return() {
        let input = "fn multiply(x: u32, y: u32): u32";
        let lexer = Lexer::new(input);
        let result = parser::FunctionDeclarationParser::new().parse(lexer);
        assert!(result.is_ok());
        let func = result.unwrap();
        assert_eq!(func.name(), "multiply");
        assert_eq!(func.return_type().as_ref().unwrap(), &Type::U32);
    }

    #[test]
    fn test_function_declaration_mutable_parameter() {
        let input = "fn toggle(flag: mutable bool)";
        let lexer = Lexer::new(input);
        let result = parser::FunctionDeclarationParser::new().parse(lexer);
        assert!(result.is_ok());
        let func = result.unwrap();
        assert_eq!(func.parameters().len(), 1);
        let param = &func.parameters()[0];
        assert_eq!(param.name(), "flag");
        assert_eq!(*param.is_mutable(), true);
        assert_eq!(param.param_type(), &Type::Bool);
    }

    #[test]
    fn test_function_declaration_generic_single() {
        let input = "fn identity[T](value: T): T";
        let lexer = Lexer::new(input);
        let result = parser::FunctionDeclarationParser::new().parse(lexer);
        assert!(result.is_ok());
        let func = result.unwrap();
        assert_eq!(func.name(), "identity");
        assert_eq!(func.generic_params().len(), 1);
        if let GenericParameter::Type { name, bounds } = &func.generic_params()[0] {
            assert_eq!(name, "T");
            assert_eq!(bounds.len(), 0);
        } else {
            panic!("Expected type parameter T");
        }
    }

    #[test]
    fn test_function_declaration_generic_with_constraints() {
        let input = "fn compare[T: Comparable](a: T, b: T): bool";
        let lexer = Lexer::new(input);
        let result = parser::FunctionDeclarationParser::new().parse(lexer);
        assert!(result.is_ok());
        let func = result.unwrap();
        assert_eq!(func.generic_params().len(), 1);
        if let GenericParameter::Type { name, bounds } = &func.generic_params()[0] {
            assert_eq!(name, "T");
            assert_eq!(bounds.len(), 1);
        } else {
            panic!("Expected type parameter T with bounds");
        }
    }

    #[test]
    fn test_function_declaration_multiple_generics() {
        let input = "fn swap[T, U](a: T, b: U): T";
        let lexer = Lexer::new(input);
        let result = parser::FunctionDeclarationParser::new().parse(lexer);
        assert!(result.is_ok());
        let func = result.unwrap();
        assert_eq!(func.generic_params().len(), 2);
    }

    #[test]
    fn test_function_declaration_const_generic() {
        let input = "fn fill[T, const N: usize](buf: *T, val: T)";
        let lexer = Lexer::new(input);
        let result = parser::FunctionDeclarationParser::new().parse(lexer);
        assert!(result.is_ok());
        let func = result.unwrap();
        assert_eq!(func.generic_params().len(), 2);
        if let GenericParameter::Const { name, ty } = &func.generic_params()[1] {
            assert_eq!(name, "N");
            assert_eq!(ty, &Type::USize);
        } else {
            panic!("Expected const parameter N, got: {:?}", func.generic_params()[1]);
        }
    }

    #[test]
    fn test_function_declaration_pointer_parameter() {
        let input = "fn write(data: *u8, len: usize)";
        let lexer = Lexer::new(input);
        let result = parser::FunctionDeclarationParser::new().parse(lexer);
        assert!(result.is_ok());
        let func = result.unwrap();
        assert_eq!(func.parameters().len(), 2);
        if let Type::TypedPointer(inner) = func.parameters()[0].param_type() {
            assert_eq!(**inner, Type::U8);
        } else {
            panic!("Expected typed pointer to u8");
        }
    }

    // ========================================================================
    // Function Definition Tests
    // ========================================================================

    #[test]
    fn test_simple_function_definition() {
        let input = "fn main() {}";
        let lexer = Lexer::new(input);
        let result = parser::FunctionDefinitionParser::new().parse(lexer);
        assert!(result.is_ok());
        let func_def = result.unwrap();
        assert_eq!(func_def.declaration().name(), "main");
        assert_eq!(func_def.body().statements().len(), 0);
    }

    #[test]
    fn test_function_definition_with_params() {
        let input = "fn add(a: i32, b: i32): i32 {}";
        let lexer = Lexer::new(input);
        let result = parser::FunctionDefinitionParser::new().parse(lexer);
        assert!(result.is_ok());
        let func_def = result.unwrap();
        assert_eq!(func_def.declaration().name(), "add");
        assert_eq!(func_def.declaration().parameters().len(), 2);
    }

    #[test]
    fn test_function_definition_generic() {
        let input = "fn swap[T](a: T, b: T) {}";
        let lexer = Lexer::new(input);
        let result = parser::FunctionDefinitionParser::new().parse(lexer);
        assert!(result.is_ok());
        let func_def = result.unwrap();
        assert_eq!(func_def.declaration().generic_params().len(), 1);
    }

    // ========================================================================
    // Interface Declaration Tests
    // ========================================================================

    #[test]
    fn test_simple_interface_declaration() {
        let input = "interface Writer {}";
        let lexer = Lexer::new(input);
        let result = parser::InterfaceDeclarationParser::new().parse(lexer);
        assert!(result.is_ok());
        let interface = result.unwrap();
        assert_eq!(interface.name(), "Writer");
        assert_eq!(interface.generic_params().len(), 0);
        assert_eq!(interface.super_interfaces().len(), 0);
        assert_eq!(interface.methods().len(), 0);
    }

    #[test]
    fn test_interface_with_generic() {
        let input = "interface Writer[T] {}";
        let lexer = Lexer::new(input);
        let result = parser::InterfaceDeclarationParser::new().parse(lexer);
        assert!(result.is_ok());
        let interface = result.unwrap();
        assert_eq!(interface.name(), "Writer");
        assert_eq!(interface.generic_params().len(), 1);
        if let GenericParameter::Type { name, .. } = &interface.generic_params()[0] {
            assert_eq!(name, "T");
        } else {
            panic!("Expected type parameter T");
        }
    }

    #[test]
    fn test_interface_with_const_generic() {
        let input = "interface FixedWriter[T, const N: usize] {}";
        let lexer = Lexer::new(input);
        let result = parser::InterfaceDeclarationParser::new().parse(lexer);
        assert!(result.is_ok());
        let interface = result.unwrap();
        assert_eq!(interface.generic_params().len(), 2);
    }

    #[test]
    fn test_interface_with_single_method() {
        let input = "interface Writer[T] { fn write(self, data: *T): usize }";
        let lexer = Lexer::new(input);
        let result = parser::InterfaceDeclarationParser::new().parse(lexer);
        assert!(result.is_ok());
        let interface = result.unwrap();
        assert_eq!(interface.methods().len(), 1);
        let method = &interface.methods()[0];
        assert_eq!(method.name(), "write");
        assert_eq!(method.parameters().len(), 2);
    }

    #[test]
    fn test_interface_with_multiple_methods() {
        let input = r#"interface Writer[T] {
            fn write(self, data: *T): usize
            fn flush(self): bool
        }"#;
        let lexer = Lexer::new(input);
        let result = parser::InterfaceDeclarationParser::new().parse(lexer);
        assert!(result.is_ok());
        let interface = result.unwrap();
        assert_eq!(interface.methods().len(), 2);
        assert_eq!(interface.methods()[0].name(), "write");
        assert_eq!(interface.methods()[1].name(), "flush");
    }

    #[test]
    fn test_interface_with_super_interface() {
        let input = "interface Serializer: Writer {}";
        let lexer = Lexer::new(input);
        let result = parser::InterfaceDeclarationParser::new().parse(lexer);
        assert!(result.is_ok());
        let interface = result.unwrap();
        assert_eq!(interface.super_interfaces().len(), 1);
        if let Type::Named { name, .. } = &interface.super_interfaces()[0] {
            assert_eq!(name, "Writer");
        } else {
            panic!("Expected named type Writer");
        }
    }

    #[test]
    fn test_interface_with_multiple_super_interfaces() {
        let input = "interface Example: Writer + Reader + Closeable {}";
        let lexer = Lexer::new(input);
        let result = parser::InterfaceDeclarationParser::new().parse(lexer);
        assert!(result.is_ok());
        let interface = result.unwrap();
        assert_eq!(interface.super_interfaces().len(), 3);
    }

    #[test]
    fn test_interface_generic_with_super_generic() {
        let input = "interface Example[T]: Writer[T] + Serializer[T] {}";
        let lexer = Lexer::new(input);
        let result = parser::InterfaceDeclarationParser::new().parse(lexer);
        assert!(result.is_ok());
        let interface = result.unwrap();
        assert_eq!(interface.name(), "Example");
        assert_eq!(interface.generic_params().len(), 1);
        assert_eq!(interface.super_interfaces().len(), 2);
    }

    #[test]
    fn test_interface_full_featured() {
        let input = r#"interface Example[T]: Writer[T] + Serializer[T] {
            fn example_method(self, data: *T)
            fn another_method(self): bool
        }"#;
        let lexer = Lexer::new(input);
        let result = parser::InterfaceDeclarationParser::new().parse(lexer);
        assert!(result.is_ok());
        let interface = result.unwrap();
        assert_eq!(interface.name(), "Example");
        assert_eq!(interface.generic_params().len(), 1);
        assert_eq!(interface.super_interfaces().len(), 2);
        assert_eq!(interface.methods().len(), 2);
    }
}
