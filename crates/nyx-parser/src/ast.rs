//! Abstract Syntax Tree definitions for Nyx

use derive_new::new;
use getset::Getters;
use nyx_lexer::{FloatLiteral, IntegerLiteral};

// ============================================================================
// Expressions
// ============================================================================

/// Main expression type for Nyx
#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    /// Integer literal expression
    IntegerLiteral(IntegerLiteral),

    /// Float literal expression
    FloatLiteral(FloatLiteral),

    /// Boolean literal (true or false)
    BooleanLiteral(bool),

    /// Character literal
    CharLiteral(String),

    /// String literal
    StringLiteral(String),

    /// Ok literal (unit type)
    OkLiteral,

    /// Identifier/variable reference
    Identifier(String),

    /// Array literal expression
    ArrayLiteral(ArrayLiteralExpr),

    /// Binary operation
    BinaryOp(BinaryOpExpr),

    /// Unary operation
    UnaryOp(UnaryOpExpr),

    /// Parenthesized expression
    Parenthesized(Box<Expression>),
}

// ============================================================================
// Array Literal
// ============================================================================

#[derive(Debug, Clone, PartialEq, new, Getters)]
#[getset(get = "pub")]
pub struct ArrayLiteralExpr {
    /// Elements in the array
    elements: Vec<Expression>,
}

// ============================================================================
// Binary Operations
// ============================================================================

#[derive(Debug, Clone, PartialEq, new, Getters)]
#[getset(get = "pub")]
pub struct BinaryOpExpr {
    /// Left-hand side expression
    lhs: Box<Expression>,
    /// Binary operator
    op: BinaryOperator,
    /// Right-hand side expression
    rhs: Box<Expression>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinaryOperator {
    // Arithmetic operators
    Add,      // +
    Subtract, // -
    Multiply, // *
    Divide,   // /
    Modulo,   // %

    // Comparison operators
    Equal,              // ==
    NotEqual,           // !=
    LessThan,           // <
    GreaterThan,        // >
    LessThanOrEqual,    // <=
    GreaterThanOrEqual, // >=

    // Logical operators
    LogicalAnd, // &&
    LogicalOr,  // ||

    // Bitwise operators
    BitwiseAnd, // &
    BitwiseOr,  // |
    BitwiseXor, // ^
    ShiftLeft,  // <<
    ShiftRight, // >>
}

// ============================================================================
// Unary Operations
// ============================================================================

#[derive(Debug, Clone, PartialEq, new, Getters)]
#[getset(get = "pub")]
pub struct UnaryOpExpr {
    /// Unary operator
    op: UnaryOperator,
    /// Operand expression
    operand: Box<Expression>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnaryOperator {
    /// Logical negation (!)
    LogicalNot,
    /// Bitwise negation (~)
    BitwiseNot,
    /// Arithmetic negation (-)
    Negate,
    /// Unary plus (+)
    Plus,
    /// Address-of operator (&)
    AddressOf,
    /// Dereference operator (*)
    Dereference,
}

// ============================================================================
// Types (for future use with type annotations)
// ============================================================================

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    /// Primitive types
    U8,
    U16,
    U32,
    U64,
    USize,
    I8,
    I16,
    I32,
    I64,
    ISize,
    F32,
    F64,
    Bool,
    Ok,

    /// Self type (used in methods and interfaces)
    SelfType,

    /// Raw pointer
    RawPointer,

    /// Typed pointer
    TypedPointer(Box<Type>),

    /// Named type (struct, union, or custom type)
    Named{ name: String, generic_args: Vec<Type> },

    /// Array type with element type and optional size
    Array {
        element_type: Box<Type>,
        size: Option<usize>,
    },

    /// Result/Error type (T ! E)
    Result {
        ok_type: Box<Type>,
        err_type: Box<Type>,
    },
}

// ============================================================================
// Generic Parameters
// ============================================================================

#[derive(Debug, Clone, PartialEq)]
pub enum GenericParameter {
    Type { name: String, bounds: Vec<Type> },
    Const { name: String, ty: Type },
}

// ============================================================================
// Functions
// ============================================================================

/// Function parameter
#[derive(Debug, Clone, PartialEq, new, Getters)]
#[getset(get = "pub")]
pub struct Parameter {
    /// Parameter name
    name: String,
    /// Whether the parameter is mutable
    is_mutable: bool,
    /// Parameter type
    param_type: Type,
}

/// Function declaration (signature only)
#[derive(Debug, Clone, PartialEq, new, Getters)]
#[getset(get = "pub")]
pub struct FunctionDeclaration {
    /// Function name
    name: String,
    /// Generic parameters
    generic_params: Vec<GenericParameter>,
    /// Function parameters
    parameters: Vec<Parameter>,
    /// Return type (None if not specified)
    return_type: Option<Type>,
}

/// Function definition (signature + body)
#[derive(Debug, Clone, PartialEq, new, Getters)]
#[getset(get = "pub")]
pub struct FunctionDefinition {
    /// Function declaration (signature)
    declaration: FunctionDeclaration,
    /// Function body
    body: CodeBlock,
}

/// Code block (placeholder for statements)
#[derive(Debug, Clone, PartialEq, new, Getters)]
#[getset(get = "pub")]
pub struct CodeBlock {
    /// Statements in the block (empty for now)
    statements: Vec<Statement>,
}

/// Statement placeholder
#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    /// Placeholder for future statement types
    Empty,
}

// ============================================================================
// Interfaces
// ============================================================================

/// Interface declaration
#[derive(Debug, Clone, PartialEq, new, Getters)]
#[getset(get = "pub")]
pub struct InterfaceDeclaration {
    /// Interface name
    name: String,
    /// Generic parameters
    generic_params: Vec<GenericParameter>,
    /// Super interfaces (interfaces this interface extends)
    super_interfaces: Vec<Type>,
    /// Method declarations in the interface
    methods: Vec<FunctionDeclaration>,
}
