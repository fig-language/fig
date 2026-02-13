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

    /// Pretty print a type to a string
    pub fn print_type(&mut self, ast_type: &Type) -> String {
        let mut output = String::new();
        self.format_type(ast_type, &mut output, true);
        output
    }

    /// Pretty print a function declaration to a string
    pub fn print_function_declaration(&mut self, func_decl: &FunctionDeclaration) -> String {
        let mut output = String::new();
        self.format_function_declaration(func_decl, &mut output, true);
        output
    }

    /// Pretty print a function definition to a string
    pub fn print_function_definition(&mut self, func_def: &FunctionDefinition) -> String {
        let mut output = String::new();
        self.format_function_definition(func_def, &mut output, true);
        output
    }

    /// Pretty print an interface declaration to a string
    pub fn print_interface_declaration(&mut self, interface: &InterfaceDeclaration) -> String {
        let mut output = String::new();
        self.format_interface_declaration(interface, &mut output, true);
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

    /// Format a type into the output buffer
    fn format_type(&mut self, ast_type: &Type, output: &mut String, is_last: bool) {
        let prefix = if self.indent_level == 0 {
            String::new()
        } else {
            format!("{}{}", self.indent(), if is_last { self.last_branch() } else { self.branch() })
        };

        match ast_type {
            Type::U8 => {
                writeln!(output, "{}Type: U8", prefix).unwrap();
            }
            Type::U16 => {
                writeln!(output, "{}Type: U16", prefix).unwrap();
            }
            Type::U32 => {
                writeln!(output, "{}Type: U32", prefix).unwrap();
            }
            Type::U64 => {
                writeln!(output, "{}Type: U64", prefix).unwrap();
            }
            Type::USize => {
                writeln!(output, "{}Type: USize", prefix).unwrap();
            }
            Type::I8 => {
                writeln!(output, "{}Type: I8", prefix).unwrap();
            }
            Type::I16 => {
                writeln!(output, "{}Type: I16", prefix).unwrap();
            }
            Type::I32 => {
                writeln!(output, "{}Type: I32", prefix).unwrap();
            }
            Type::I64 => {
                writeln!(output, "{}Type: I64", prefix).unwrap();
            }
            Type::ISize => {
                writeln!(output, "{}Type: ISize", prefix).unwrap();
            }
            Type::F32 => {
                writeln!(output, "{}Type: F32", prefix).unwrap();
            }
            Type::F64 => {
                writeln!(output, "{}Type: F64", prefix).unwrap();
            }
            Type::Bool => {
                writeln!(output, "{}Type: Bool", prefix).unwrap();
            }
            Type::Ok => {
                writeln!(output, "{}Type: Ok", prefix).unwrap();
            }
            Type::SelfType => {
                writeln!(output, "{}Type: Self", prefix).unwrap();
            }
            Type::RawPointer => {
                writeln!(output, "{}Type: RawPointer", prefix).unwrap();
            }
            Type::TypedPointer(inner_type) => {
                writeln!(output, "{}Type: TypedPointer", prefix).unwrap();
                self.indent_level += 1;
                writeln!(output, "{}element_type:", self.indent()).unwrap();
                self.indent_level += 1;
                self.format_type(inner_type, output, true);
                self.indent_level -= 1;
                self.indent_level -= 1;
            }
            Type::Named{ name, generic_args } => {
                writeln!(output, "{}Type: Named(\"{}\")", prefix, name).unwrap();
                if !generic_args.is_empty() {
                    self.indent_level += 1;
                    writeln!(output, "{}generic_args:", self.indent()).unwrap();
                    self.indent_level += 1;
                    for (i, arg) in generic_args.iter().enumerate() {
                        self.format_type(arg, output, i == generic_args.len() - 1);
                    }
                    self.indent_level -= 1;
                    self.indent_level -= 1;
                }
            }
            Type::Array { element_type, size } => {
                writeln!(output, "{}Type: Array", prefix).unwrap();
                self.indent_level += 1;
                if let Some(s) = size {
                    writeln!(output, "{}size: {}", self.indent(), s).unwrap();
                } else {
                    writeln!(output, "{}size: dynamic", self.indent()).unwrap();
                }
                writeln!(output, "{}element_type:", self.indent()).unwrap();
                self.indent_level += 1;
                self.format_type(element_type, output, true);
                self.indent_level -= 1;
                self.indent_level -= 1;
            }
            Type::Result { ok_type, err_type } => {
                writeln!(output, "{}Type: Result", prefix).unwrap();
                self.indent_level += 1;
                writeln!(output, "{}ok_type:", self.indent()).unwrap();
                self.indent_level += 1;
                self.format_type(ok_type, output, true);
                self.indent_level -= 1;
                writeln!(output, "{}err_type:", self.indent()).unwrap();
                self.indent_level += 1;
                self.format_type(err_type, output, true);
                self.indent_level -= 1;
                self.indent_level -= 1;
            }
        }
    }

    /// Format a parameter into the output buffer
    fn format_parameter(&mut self, param: &Parameter, output: &mut String, is_last: bool) {
        let prefix = if self.indent_level == 0 {
            String::new()
        } else {
            format!("{}{}", self.indent(), if is_last { self.last_branch() } else { self.branch() })
        };

        let mutability = if *param.is_mutable() { "mutable " } else { "" };
        writeln!(output, "{}Parameter: {}{}", prefix, mutability, param.name()).unwrap();
        self.indent_level += 1;
        writeln!(output, "{}type:", self.indent()).unwrap();
        self.indent_level += 1;
        self.format_type(param.param_type(), output, true);
        self.indent_level -= 1;
        self.indent_level -= 1;
    }

    /// Format a function declaration into the output buffer
    fn format_function_declaration(&mut self, func_decl: &FunctionDeclaration, output: &mut String, is_last: bool) {
        let prefix = if self.indent_level == 0 {
            String::new()
        } else {
            format!("{}{}", self.indent(), if is_last { self.last_branch() } else { self.branch() })
        };

        writeln!(output, "{}FunctionDeclaration: {}", prefix, func_decl.name()).unwrap();
        self.indent_level += 1;

        // Generic parameters
        if !func_decl.generic_params().is_empty() {
            writeln!(output, "{}generic_params: {} item(s)", self.indent(), func_decl.generic_params().len()).unwrap();
            self.indent_level += 1;
            for (i, param) in func_decl.generic_params().iter().enumerate() {
                self.format_generic_parameter(param, output, i == func_decl.generic_params().len() - 1);
            }
            self.indent_level -= 1;
        }

        // Parameters
        if !func_decl.parameters().is_empty() {
            writeln!(output, "{}parameters: {} item(s)", self.indent(), func_decl.parameters().len()).unwrap();
            self.indent_level += 1;
            for (i, param) in func_decl.parameters().iter().enumerate() {
                self.format_parameter(param, output, i == func_decl.parameters().len() - 1);
            }
            self.indent_level -= 1;
        }

        // Return type
        if let Some(ret_type) = func_decl.return_type() {
            writeln!(output, "{}return_type:", self.indent()).unwrap();
            self.indent_level += 1;
            self.format_type(ret_type, output, true);
            self.indent_level -= 1;
        }

        self.indent_level -= 1;
    }

    /// Format a function definition into the output buffer
    fn format_function_definition(&mut self, func_def: &FunctionDefinition, output: &mut String, is_last: bool) {
        let prefix = if self.indent_level == 0 {
            String::new()
        } else {
            format!("{}{}", self.indent(), if is_last { self.last_branch() } else { self.branch() })
        };

        writeln!(output, "{}FunctionDefinition", prefix).unwrap();
        self.indent_level += 1;
        
        writeln!(output, "{}declaration:", self.indent()).unwrap();
        self.indent_level += 1;
        self.format_function_declaration(func_def.declaration(), output, true);
        self.indent_level -= 1;

        writeln!(output, "{}body:", self.indent()).unwrap();
        self.indent_level += 1;
        self.format_code_block(func_def.body(), output, true);
        self.indent_level -= 1;

        self.indent_level -= 1;
    }

    /// Format a code block into the output buffer
    fn format_code_block(&mut self, block: &CodeBlock, output: &mut String, is_last: bool) {
        let prefix = if self.indent_level == 0 {
            String::new()
        } else {
            format!("{}{}", self.indent(), if is_last { self.last_branch() } else { self.branch() })
        };

        if block.statements().is_empty() {
            writeln!(output, "{}CodeBlock (empty)", prefix).unwrap();
        } else {
            writeln!(output, "{}CodeBlock: {} statement(s)", prefix, block.statements().len()).unwrap();
        }
    }

    /// Format an interface declaration into the output buffer
    fn format_interface_declaration(&mut self, interface: &InterfaceDeclaration, output: &mut String, is_last: bool) {
        let prefix = if self.indent_level == 0 {
            String::new()
        } else {
            format!("{}{}", self.indent(), if is_last { self.last_branch() } else { self.branch() })
        };

        writeln!(output, "{}InterfaceDeclaration: {}", prefix, interface.name()).unwrap();
        self.indent_level += 1;

        // Generic parameters
        if !interface.generic_params().is_empty() {
            writeln!(output, "{}generic_params: {} item(s)", self.indent(), interface.generic_params().len()).unwrap();
            self.indent_level += 1;
            for (i, param) in interface.generic_params().iter().enumerate() {
                self.format_generic_parameter(param, output, i == interface.generic_params().len() - 1);
            }
            self.indent_level -= 1;
        }

        // Super interfaces
        if !interface.super_interfaces().is_empty() {
            writeln!(output, "{}super_interfaces: {} item(s)", self.indent(), interface.super_interfaces().len()).unwrap();
            self.indent_level += 1;
            for (i, super_type) in interface.super_interfaces().iter().enumerate() {
                self.format_type(super_type, output, i == interface.super_interfaces().len() - 1);
            }
            self.indent_level -= 1;
        }

        // Methods
        if !interface.methods().is_empty() {
            writeln!(output, "{}methods: {} item(s)", self.indent(), interface.methods().len()).unwrap();
            self.indent_level += 1;
            for (i, method) in interface.methods().iter().enumerate() {
                self.format_function_declaration(method, output, i == interface.methods().len() - 1);
            }
            self.indent_level -= 1;
        }

        self.indent_level -= 1;
    }

    /// Format a generic parameter into the output buffer
    fn format_generic_parameter(&mut self, param: &GenericParameter, output: &mut String, is_last: bool) {
        let prefix = if self.indent_level == 0 {
            String::new()
        } else {
            format!("{}{}", self.indent(), if is_last { self.last_branch() } else { self.branch() })
        };

        match param {
            GenericParameter::Type { name, bounds } => {
                if bounds.is_empty() {
                    writeln!(output, "{}TypeParameter: {}", prefix, name).unwrap();
                } else {
                    writeln!(output, "{}TypeParameter: {} with {} bound(s)", prefix, name, bounds.len()).unwrap();
                    self.indent_level += 1;
                    for (i, bound) in bounds.iter().enumerate() {
                        self.format_type(bound, output, i == bounds.len() - 1);
                    }
                    self.indent_level -= 1;
                }
            }
            GenericParameter::Const { name, ty } => {
                writeln!(output, "{}ConstParameter: {} :", prefix, name).unwrap();
                self.indent_level += 1;
                self.format_type(ty, output, true);
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

impl std::fmt::Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", PrettyPrinter::new().print_type(self))
    }
}

impl std::fmt::Display for FunctionDeclaration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", PrettyPrinter::new().print_function_declaration(self))
    }
}

impl std::fmt::Display for FunctionDefinition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", PrettyPrinter::new().print_function_definition(self))
    }
}

impl std::fmt::Display for InterfaceDeclaration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", PrettyPrinter::new().print_interface_declaration(self))
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

    #[test]
    fn test_print_primitive_type() {
        let ast_type = Type::I32;
        let output = PrettyPrinter::new().print_type(&ast_type);
        assert!(output.contains("Type: I32"));
    }

    #[test]
    fn test_print_raw_pointer_type() {
        let ast_type = Type::RawPointer;
        let output = PrettyPrinter::new().print_type(&ast_type);
        assert!(output.contains("Type: RawPointer"));
    }

    #[test]
    fn test_print_typed_pointer_type() {
        let ast_type = Type::TypedPointer(Box::new(Type::Bool));
        let output = PrettyPrinter::new().print_type(&ast_type);
        assert!(output.contains("Type: TypedPointer"));
        assert!(output.contains("element_type:"));
        assert!(output.contains("Type: Bool"));
    }

    #[test]
    fn test_print_named_type() {
        let ast_type = Type::Named{ name: "MyStruct".to_string(), generic_args: vec![] };
        let output = PrettyPrinter::new().print_type(&ast_type);
        assert!(output.contains("Type: Named(\"MyStruct\")"));
    }

    #[test]
    fn test_print_array_type_fixed_size() {
        let ast_type = Type::Array {
            element_type: Box::new(Type::U8),
            size: Some(10),
        };
        let output = PrettyPrinter::new().print_type(&ast_type);
        assert!(output.contains("Type: Array"));
        assert!(output.contains("size: 10"));
        assert!(output.contains("element_type:"));
        assert!(output.contains("Type: U8"));
    }

    #[test]
    fn test_print_array_type_dynamic_size() {
        let ast_type = Type::Array {
            element_type: Box::new(Type::Named{ name: "Foo".to_string(), generic_args: vec![] }),
            size: None,
        };
        let output = PrettyPrinter::new().print_type(&ast_type);
        assert!(output.contains("Type: Array"));
        assert!(output.contains("size: dynamic"));
        assert!(output.contains("element_type:"));
        assert!(output.contains("Type: Named(\"Foo\")"));
    }

    #[test]
    fn test_print_result_type() {
        let ast_type = Type::Result {
            ok_type: Box::new(Type::I32),
            err_type: Box::new(Type::Named{ name: "MyError".to_string(), generic_args: vec![] }),
        };
        let output = PrettyPrinter::new().print_type(&ast_type);
        assert!(output.contains("Type: Result"));
        assert!(output.contains("ok_type:"));
        assert!(output.contains("Type: I32"));
        assert!(output.contains("err_type:"));
        assert!(output.contains("Type: Named(\"MyError\")"));
    }

    #[test]
    fn test_display_trait_for_type() {
        let ast_type = Type::TypedPointer(Box::new(Type::RawPointer));
        let output = format!("{}", ast_type);
        assert!(output.contains("Type: TypedPointer"));
        assert!(output.contains("element_type:"));
        assert!(output.contains("Type: RawPointer"));
    }

    #[test]
    fn test_print_self_type() {
        let ast_type = Type::SelfType;
        let output = PrettyPrinter::new().print_type(&ast_type);
        assert!(output.contains("Type: Self"));
    }

    #[test]
    fn test_print_simple_function_declaration() {
        use crate::ast::*;
        let func_decl = FunctionDeclaration::new(
            "add".to_string(),
            vec![],
            vec![
                Parameter::new("a".to_string(), false, Type::I32),
                Parameter::new("b".to_string(), false, Type::I32),
            ],
            Some(Type::I32),
        );
        let output = PrettyPrinter::new().print_function_declaration(&func_decl);
        assert!(output.contains("FunctionDeclaration: add"));
        assert!(output.contains("parameters: 2 item(s)"));
        assert!(output.contains("Parameter: a"));
        assert!(output.contains("Parameter: b"));
        assert!(output.contains("return_type:"));
        assert!(output.contains("Type: I32"));
    }

    #[test]
    fn test_print_generic_function_declaration() {
        use crate::ast::*;
        let func_decl = FunctionDeclaration::new(
            "identity".to_string(),
            vec![GenericParameter::Type {
                name: "T".to_string(),
                bounds: vec![],
            }],
            vec![Parameter::new("value".to_string(), false, Type::Named {
                name: "T".to_string(),
                generic_args: vec![],
            })],
            Some(Type::Named {
                name: "T".to_string(),
                generic_args: vec![],
            }),
        );
        let output = PrettyPrinter::new().print_function_declaration(&func_decl);
        assert!(output.contains("FunctionDeclaration: identity"));
        assert!(output.contains("generic_params: 1 item(s)"));
        assert!(output.contains("TypeParameter: T"));
    }

    #[test]
    fn test_print_function_with_mutable_param() {
        use crate::ast::*;
        let func_decl = FunctionDeclaration::new(
            "toggle".to_string(),
            vec![],
            vec![Parameter::new("flag".to_string(), true, Type::Bool)],
            None,
        );
        let output = PrettyPrinter::new().print_function_declaration(&func_decl);
        assert!(output.contains("Parameter: mutable flag"));
    }

    #[test]
    fn test_print_function_definition() {
        use crate::ast::*;
        let func_decl = FunctionDeclaration::new(
            "main".to_string(),
            vec![],
            vec![],
            None,
        );
        let func_def = FunctionDefinition::new(func_decl, CodeBlock::new(vec![]));
        let output = PrettyPrinter::new().print_function_definition(&func_def);
        assert!(output.contains("FunctionDefinition"));
        assert!(output.contains("declaration:"));
        assert!(output.contains("FunctionDeclaration: main"));
        assert!(output.contains("body:"));
        assert!(output.contains("CodeBlock (empty)"));
    }

    #[test]
    fn test_print_simple_interface() {
        use crate::ast::*;
        let interface = InterfaceDeclaration::new(
            "Writer".to_string(),
            vec![],
            vec![],
            vec![],
        );
        let output = PrettyPrinter::new().print_interface_declaration(&interface);
        assert!(output.contains("InterfaceDeclaration: Writer"));
    }

    #[test]
    fn test_print_interface_with_generics() {
        use crate::ast::*;
        let interface = InterfaceDeclaration::new(
            "Writer".to_string(),
            vec![GenericParameter::Type {
                name: "T".to_string(),
                bounds: vec![],
            }],
            vec![],
            vec![],
        );
        let output = PrettyPrinter::new().print_interface_declaration(&interface);
        assert!(output.contains("InterfaceDeclaration: Writer"));
        assert!(output.contains("generic_params: 1 item(s)"));
        assert!(output.contains("TypeParameter: T"));
    }

    #[test]
    fn test_print_interface_with_super_interfaces() {
        use crate::ast::*;
        let interface = InterfaceDeclaration::new(
            "Serializer".to_string(),
            vec![],
            vec![Type::Named {
                name: "Writer".to_string(),
                generic_args: vec![],
            }],
            vec![],
        );
        let output = PrettyPrinter::new().print_interface_declaration(&interface);
        assert!(output.contains("InterfaceDeclaration: Serializer"));
        assert!(output.contains("super_interfaces: 1 item(s)"));
        assert!(output.contains("Type: Named(\"Writer\")"));
    }

    #[test]
    fn test_print_interface_with_methods() {
        use crate::ast::*;
        let method = FunctionDeclaration::new(
            "write".to_string(),
            vec![],
            vec![
                Parameter::new("self".to_string(), false, Type::SelfType),
                Parameter::new("data".to_string(), false, Type::TypedPointer(Box::new(Type::U8))),
            ],
            Some(Type::USize),
        );
        let interface = InterfaceDeclaration::new(
            "Writer".to_string(),
            vec![],
            vec![],
            vec![method],
        );
        let output = PrettyPrinter::new().print_interface_declaration(&interface);
        assert!(output.contains("InterfaceDeclaration: Writer"));
        assert!(output.contains("methods: 1 item(s)"));
        assert!(output.contains("FunctionDeclaration: write"));
        assert!(output.contains("Parameter: self"));
        assert!(output.contains("Type: Self"));
    }

    #[test]
    fn test_display_trait_for_function_declaration() {
        use crate::ast::*;
        let func_decl = FunctionDeclaration::new(
            "test".to_string(),
            vec![],
            vec![],
            None,
        );
        let output = format!("{}", func_decl);
        assert!(output.contains("FunctionDeclaration: test"));
    }

    #[test]
    fn test_display_trait_for_interface_declaration() {
        use crate::ast::*;
        let interface = InterfaceDeclaration::new(
            "MyInterface".to_string(),
            vec![],
            vec![],
            vec![],
        );
        let output = format!("{}", interface);
        assert!(output.contains("InterfaceDeclaration: MyInterface"));
    }
}
