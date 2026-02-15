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

    Reference(Box<Type>),

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
// Type Alias
// ============================================================================

#[derive(Debug, Clone, PartialEq, new, Getters)]
#[getset(get = "pub")]
pub struct TypeAlias {
    /// Name of the type alias
    name: String,
    /// Generic parameters
    generic_params: Vec<GenericParameter>,
    /// Type being aliased
    aliased_type: Type,
    /// Optional where clause bounds
    where_clause: Vec<GenericParameter>,
}

// ============================================================================
// Enum
// ============================================================================

#[derive(Debug, Clone, PartialEq, new, Getters)]
#[getset(get = "pub")]
pub struct Enum {
    /// Name of the enum
    name: String,
    /// Optional enum representation
    representation: Option<Type>,
    /// Generic parameters
    generic_params: Vec<GenericParameter>,
    /// Optional requires clause
    requires_clause: Vec<Type>,
    /// Optional where clause bounds
    where_clause: Vec<GenericParameter>,
    /// Enum variants
    variants: Vec<EnumVariant>,
}

#[derive(Debug, Clone, PartialEq, new, Getters)]
#[getset(get = "pub")]
pub struct EnumVariant {
    /// Name of the variant
    name: String,
    /// Optional discriminant value
    value: Option<Expression>,
}

// ============================================================================
// Union
// ============================================================================

#[derive(Debug, Clone, PartialEq, new, Getters)]
#[getset(get = "pub")]
pub struct Union {
    /// Name of the union
    name: String,
    /// Generic parameters
    generic_params: Vec<GenericParameter>,
    /// Optional requires clause
    requires_clause: Vec<Type>,
    /// Optional where clause bounds
    where_clause: Vec<GenericParameter>,
    /// Union variants
    variants: Vec<UnionVariant>,
}

#[derive(Debug, Clone, PartialEq, new, Getters)]
#[getset(get = "pub")]
pub struct UnionVariant {
    /// Name of the variant
    name: String,
    /// Type of the variant
    ty: Type,
}

// ============================================================================
// Struct
// ============================================================================

#[derive(Debug, Clone, PartialEq, new, Getters)]
#[getset(get = "pub")]
pub struct Struct {
    /// Name of the struct
    name: String,
    /// Generic parameters
    generic_params: Vec<GenericParameter>,
    /// Optional requires clause
    requires_clause: Vec<Type>,
    /// Optional where clause bounds
    where_clause: Vec<GenericParameter>,
    /// Struct fields
    fields: Vec<StructField>,
}

#[derive(Debug, Clone, PartialEq, new, Getters)]
#[getset(get = "pub")]
pub struct StructField {
    /// Name of the field
    name: String,
    /// Type of the field
    ty: Type,
}

// ============================================================================
// Function
// ============================================================================

#[derive(Debug, Clone, PartialEq, new, Getters)]
#[getset(get = "pub")]
pub struct Function {
    /// Function signature
    signature: FunctionSignature,
    /// Function body (statements)
    body: Block,
}

#[derive(Debug, Clone, PartialEq, new, Getters)]
#[getset(get = "pub")]
pub struct FunctionSignature {
    /// Name of the function
    name: String,
    /// Generic parameters
    generic_params: Vec<GenericParameter>,
    /// Function parameters
    params: Vec<FunctionParameter>,
    /// Return type (None for no return type)
    return_type: Option<Type>,
    /// Optional where clause bounds
    where_clause: Vec<GenericParameter>,
}

#[derive(Debug, Clone, PartialEq, new, Getters)]
#[getset(get = "pub")]
pub struct FunctionParameter {
    /// Parameter name
    name: String,
    /// Parameter type
    ty: Type,
}

#[derive(Debug, Clone, PartialEq, new, Getters)]
#[getset(get = "pub")]
pub struct Block {
    /// Statements in the block
    statements: Vec<Statement>,
}

// ============================================================================
// Interfaces
// ============================================================================

#[derive(Debug, Clone, PartialEq, new, Getters)]
#[getset(get = "pub")]
pub struct Interface {
    /// Name of the interface
    name: String,
    /// Generic parameters
    generic_params: Vec<GenericParameter>,
    /// Optional extends clause
    extends_clause: Vec<Type>,
    /// Optional where clause bounds
    where_clause: Vec<GenericParameter>,
    /// Interface methods (not fully defined yet)
    methods: Vec<FunctionSignature>,
}

// ============================================================================
// Namespaces
// ============================================================================

#[derive(Debug, Clone, PartialEq, new, Getters)]
#[getset(get = "pub")]
pub struct Namespace {
    /// Name of the namespace
    name: String,
    /// Items in the namespace (functions, types, etc.)
    items: Vec<NamespaceItem>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum NamespaceItem {
    Namespace(Namespace),
    Function(Function),
    TypeAlias(TypeAlias),
    Struct(Struct),
    Enum(Enum),
    Union(Union),
    Interface(Interface),
}

// ============================================================================
// Statements
// ============================================================================

#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    /// Placeholder for future statement types
    Pass,
}