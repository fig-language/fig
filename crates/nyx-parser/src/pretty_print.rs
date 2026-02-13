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

    /// Pretty print a type alias to a string
    pub fn print_type_alias(&mut self, type_alias: &TypeAlias) -> String {
        let mut output = String::new();
        self.format_type_alias(type_alias, &mut output);
        output
    }

    /// Pretty print a generic parameter to a string
    pub fn print_generic_parameter(&mut self, param: &GenericParameter) -> String {
        let mut output = String::new();
        self.format_generic_parameter(param, &mut output, true);
        output
    }

    /// Pretty print an enum to a string
    pub fn print_enum(&mut self, enum_decl: &Enum) -> String {
        let mut output = String::new();
        self.format_enum(enum_decl, &mut output);
        output
    }

    /// Pretty print a union to a string
    pub fn print_union(&mut self, union_decl: &Union) -> String {
        let mut output = String::new();
        self.format_union(union_decl, &mut output);
        output
    }

    /// Pretty print a struct to a string
    pub fn print_struct(&mut self, struct_decl: &Struct) -> String {
        let mut output = String::new();
        self.format_struct(struct_decl, &mut output);
        output
    }

    /// Pretty print a function to a string
    pub fn print_function(&mut self, func: &Function) -> String {
        let mut output = String::new();
        self.format_function(func, &mut output);
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
            Type::Reference(inner_type) => {
                writeln!(output, "{}Type: Reference", prefix).unwrap();
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

    /// Format a type alias into the output buffer
    fn format_type_alias(&mut self, type_alias: &TypeAlias, output: &mut String) {
        writeln!(output, "TypeAlias: {}", type_alias.name()).unwrap();
        self.indent_level += 1;

        // Generic parameters
        if !type_alias.generic_params().is_empty() {
            writeln!(output, "{}generic_params:", self.indent()).unwrap();
            self.indent_level += 1;
            for (i, param) in type_alias.generic_params().iter().enumerate() {
                self.format_generic_parameter(param, output, i == type_alias.generic_params().len() - 1);
            }
            self.indent_level -= 1;
        }

        // Aliased type
        writeln!(output, "{}aliased_type:", self.indent()).unwrap();
        self.indent_level += 1;
        self.format_type(type_alias.aliased_type(), output, true);
        self.indent_level -= 1;

        // Where clause
        if !type_alias.where_clause().is_empty() {
            writeln!(output, "{}where_clause:", self.indent()).unwrap();
            self.indent_level += 1;
            for (i, param) in type_alias.where_clause().iter().enumerate() {
                self.format_generic_parameter(param, output, i == type_alias.where_clause().len() - 1);
            }
            self.indent_level -= 1;
        }

        self.indent_level -= 1;
    }

    /// Format a generic parameter into the output buffer
    fn format_generic_parameter(&mut self, param: &GenericParameter, output: &mut String, is_last: bool) {
        let prefix = format!("{}{}", self.indent(), if is_last { self.last_branch() } else { self.branch() });

        match param {
            GenericParameter::Type { name, bounds } => {
                if bounds.is_empty() {
                    writeln!(output, "{}TypeParam: {}", prefix, name).unwrap();
                } else {
                    writeln!(output, "{}TypeParam: {} with bounds", prefix, name).unwrap();
                    self.indent_level += 1;
                    for (i, bound) in bounds.iter().enumerate() {
                        self.format_type(bound, output, i == bounds.len() - 1);
                    }
                    self.indent_level -= 1;
                }
            }
            GenericParameter::Const { name, ty } => {
                writeln!(output, "{}ConstParam: {} :", prefix, name).unwrap();
                self.indent_level += 1;
                self.format_type(ty, output, true);
                self.indent_level -= 1;
            }
        }
    }

    /// Format an enum into the output buffer
    fn format_enum(&mut self, enum_decl: &Enum, output: &mut String) {
        writeln!(output, "Enum: {}", enum_decl.name()).unwrap();
        self.indent_level += 1;

        // Generic parameters
        if !enum_decl.generic_params().is_empty() {
            writeln!(output, "{}generic_params:", self.indent()).unwrap();
            self.indent_level += 1;
            for (i, param) in enum_decl.generic_params().iter().enumerate() {
                self.format_generic_parameter(param, output, i == enum_decl.generic_params().len() - 1);
            }
            self.indent_level -= 1;
        }

        // Requires clause
        if !enum_decl.requires_clause().is_empty() {
            writeln!(output, "{}requires_clause:", self.indent()).unwrap();
            self.indent_level += 1;
            for (i, ty) in enum_decl.requires_clause().iter().enumerate() {
                self.format_type(ty, output, i == enum_decl.requires_clause().len() - 1);
            }
            self.indent_level -= 1;
        }

        // Where clause
        if !enum_decl.where_clause().is_empty() {
            writeln!(output, "{}where_clause:", self.indent()).unwrap();
            self.indent_level += 1;
            for (i, param) in enum_decl.where_clause().iter().enumerate() {
                self.format_generic_parameter(param, output, i == enum_decl.where_clause().len() - 1);
            }
            self.indent_level -= 1;
        }

        // Variants
        if !enum_decl.variants().is_empty() {
            writeln!(output, "{}variants:", self.indent()).unwrap();
            self.indent_level += 1;
            for (i, variant) in enum_decl.variants().iter().enumerate() {
                self.format_enum_variant(variant, output, i == enum_decl.variants().len() - 1);
            }
            self.indent_level -= 1;
        }

        self.indent_level -= 1;
    }

    /// Format an enum variant into the output buffer
    fn format_enum_variant(&mut self, variant: &EnumVariant, output: &mut String, is_last: bool) {
        let prefix = format!("{}{}", self.indent(), if is_last { self.last_branch() } else { self.branch() });
        
        if let Some(value) = variant.value() {
            writeln!(output, "{}Variant: {} = {}", prefix, variant.name(), value).unwrap();
        } else {
            writeln!(output, "{}Variant: {}", prefix, variant.name()).unwrap();
        }
    }

    /// Format a union into the output buffer
    fn format_union(&mut self, union_decl: &Union, output: &mut String) {
        writeln!(output, "Union: {}", union_decl.name()).unwrap();
        self.indent_level += 1;

        // Generic parameters
        if !union_decl.generic_params().is_empty() {
            writeln!(output, "{}generic_params:", self.indent()).unwrap();
            self.indent_level += 1;
            for (i, param) in union_decl.generic_params().iter().enumerate() {
                self.format_generic_parameter(param, output, i == union_decl.generic_params().len() - 1);
            }
            self.indent_level -= 1;
        }

        // Requires clause
        if !union_decl.requires_clause().is_empty() {
            writeln!(output, "{}requires_clause:", self.indent()).unwrap();
            self.indent_level += 1;
            for (i, ty) in union_decl.requires_clause().iter().enumerate() {
                self.format_type(ty, output, i == union_decl.requires_clause().len() - 1);
            }
            self.indent_level -= 1;
        }

        // Where clause
        if !union_decl.where_clause().is_empty() {
            writeln!(output, "{}where_clause:", self.indent()).unwrap();
            self.indent_level += 1;
            for (i, param) in union_decl.where_clause().iter().enumerate() {
                self.format_generic_parameter(param, output, i == union_decl.where_clause().len() - 1);
            }
            self.indent_level -= 1;
        }

        // Variants
        if !union_decl.variants().is_empty() {
            writeln!(output, "{}variants:", self.indent()).unwrap();
            self.indent_level += 1;
            for (i, variant) in union_decl.variants().iter().enumerate() {
                self.format_union_variant(variant, output, i == union_decl.variants().len() - 1);
            }
            self.indent_level -= 1;
        }

        self.indent_level -= 1;
    }

    /// Format a union variant into the output buffer
    fn format_union_variant(&mut self, variant: &UnionVariant, output: &mut String, is_last: bool) {
        let prefix = format!("{}{}", self.indent(), if is_last { self.last_branch() } else { self.branch() });
        writeln!(output, "{}Variant: {} :", prefix, variant.name()).unwrap();
        self.indent_level += 1;
        self.format_type(variant.ty(), output, true);
        self.indent_level -= 1;
    }

    /// Format a struct into the output buffer
    fn format_struct(&mut self, struct_decl: &Struct, output: &mut String) {
        writeln!(output, "Struct: {}", struct_decl.name()).unwrap();
        self.indent_level += 1;

        // Generic parameters
        if !struct_decl.generic_params().is_empty() {
            writeln!(output, "{}generic_params:", self.indent()).unwrap();
            self.indent_level += 1;
            for (i, param) in struct_decl.generic_params().iter().enumerate() {
                self.format_generic_parameter(param, output, i == struct_decl.generic_params().len() - 1);
            }
            self.indent_level -= 1;
        }

        // Requires clause
        if !struct_decl.requires_clause().is_empty() {
            writeln!(output, "{}requires_clause:", self.indent()).unwrap();
            self.indent_level += 1;
            for (i, ty) in struct_decl.requires_clause().iter().enumerate() {
                self.format_type(ty, output, i == struct_decl.requires_clause().len() - 1);
            }
            self.indent_level -= 1;
        }

        // Where clause
        if !struct_decl.where_clause().is_empty() {
            writeln!(output, "{}where_clause:", self.indent()).unwrap();
            self.indent_level += 1;
            for (i, param) in struct_decl.where_clause().iter().enumerate() {
                self.format_generic_parameter(param, output, i == struct_decl.where_clause().len() - 1);
            }
            self.indent_level -= 1;
        }

        // Fields
        if !struct_decl.fields().is_empty() {
            writeln!(output, "{}fields:", self.indent()).unwrap();
            self.indent_level += 1;
            for (i, field) in struct_decl.fields().iter().enumerate() {
                self.format_struct_field(field, output, i == struct_decl.fields().len() - 1);
            }
            self.indent_level -= 1;
        }

        self.indent_level -= 1;
    }

    /// Format a struct field into the output buffer
    fn format_struct_field(&mut self, field: &StructField, output: &mut String, is_last: bool) {
        let prefix = format!("{}{}", self.indent(), if is_last { self.last_branch() } else { self.branch() });
        writeln!(output, "{}Field: {} :", prefix, field.name()).unwrap();
        self.indent_level += 1;
        self.format_type(field.ty(), output, true);
        self.indent_level -= 1;
    }

    /// Format a function into the output buffer
    fn format_function(&mut self, func: &Function, output: &mut String) {
        writeln!(output, "Function: {}", func.name()).unwrap();
        self.indent_level += 1;

        // Generic parameters
        if !func.generic_params().is_empty() {
            writeln!(output, "{}generic_params:", self.indent()).unwrap();
            self.indent_level += 1;
            for (i, param) in func.generic_params().iter().enumerate() {
                self.format_generic_parameter(param, output, i == func.generic_params().len() - 1);
            }
            self.indent_level -= 1;
        }

        // Parameters
        if !func.params().is_empty() {
            writeln!(output, "{}params:", self.indent()).unwrap();
            self.indent_level += 1;
            for (i, param) in func.params().iter().enumerate() {
                self.format_function_parameter(param, output, i == func.params().len() - 1);
            }
            self.indent_level -= 1;
        }

        // Return type
        if let Some(return_type) = func.return_type() {
            writeln!(output, "{}return_type:", self.indent()).unwrap();
            self.indent_level += 1;
            self.format_type(return_type, output, true);
            self.indent_level -= 1;
        }

        // Where clause
        if !func.where_clause().is_empty() {
            writeln!(output, "{}where_clause:", self.indent()).unwrap();
            self.indent_level += 1;
            for (i, param) in func.where_clause().iter().enumerate() {
                self.format_generic_parameter(param, output, i == func.where_clause().len() - 1);
            }
            self.indent_level -= 1;
        }

        // Body
        writeln!(output, "{}body:", self.indent()).unwrap();
        self.indent_level += 1;
        self.format_block(func.body(), output);
        self.indent_level -= 1;

        self.indent_level -= 1;
    }

    /// Format a function parameter into the output buffer
    fn format_function_parameter(&mut self, param: &FunctionParameter, output: &mut String, is_last: bool) {
        let prefix = format!("{}{}", self.indent(), if is_last { self.last_branch() } else { self.branch() });
        writeln!(output, "{}Param: {} :", prefix, param.name()).unwrap();
        self.indent_level += 1;
        self.format_type(param.ty(), output, true);
        self.indent_level -= 1;
    }

    /// Format a block into the output buffer
    fn format_block(&mut self, block: &Block, output: &mut String) {
        if block.statements().is_empty() {
            writeln!(output, "{}Block (empty)", self.indent()).unwrap();
        } else {
            writeln!(output, "{}Block with {} statement(s)", self.indent(), block.statements().len()).unwrap();
            self.indent_level += 1;
            for (i, stmt) in block.statements().iter().enumerate() {
                self.format_statement(stmt, output, i == block.statements().len() - 1);
            }
            self.indent_level -= 1;
        }
    }

    /// Format a statement into the output buffer
    fn format_statement(&mut self, stmt: &Statement, output: &mut String, is_last: bool) {
        let prefix = format!("{}{}", self.indent(), if is_last { self.last_branch() } else { self.branch() });
        match stmt {
            Statement::Empty => {
                writeln!(output, "{}Statement: Empty", prefix).unwrap();
            }
            Statement::Placeholder(name) => {
                writeln!(output, "{}Statement: Placeholder(\"{}\")", prefix, name).unwrap();
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

/// Convenience function to pretty print an enum with default settings
pub fn print_enum(enum_decl: &Enum) -> String {
    PrettyPrinter::new().print_enum(enum_decl)
}

/// Convenience function to pretty print a union with default settings
pub fn print_union(union_decl: &Union) -> String {
    PrettyPrinter::new().print_union(union_decl)
}

/// Convenience function to pretty print a struct with default settings
pub fn print_struct(struct_decl: &Struct) -> String {
    PrettyPrinter::new().print_struct(struct_decl)
}

/// Convenience function to pretty print a function with default settings
pub fn print_function(func: &Function) -> String {
    PrettyPrinter::new().print_function(func)
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

impl std::fmt::Display for TypeAlias {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", PrettyPrinter::new().print_type_alias(self))
    }
}

impl std::fmt::Display for GenericParameter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", PrettyPrinter::new().print_generic_parameter(self))
    }
}

impl std::fmt::Display for Enum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", print_enum(self))
    }
}

impl std::fmt::Display for Union {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", print_union(self))
    }
}

impl std::fmt::Display for Struct {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", print_struct(self))
    }
}

impl std::fmt::Display for Function {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", print_function(self))
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
}
