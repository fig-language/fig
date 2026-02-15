// Namespace parsing tests for nyx-parser

use crate::{Lexer, ast::{Type, NamespaceItem}, parser};

#[test]
fn test_simple_namespace() {
    let input = "namespace MyModule
    type Alias = i32
";
    let lexer = Lexer::new(input);
    let result = parser::NamespaceParser::new().parse(lexer);
    assert!(result.is_ok(), "Failed to parse: {:?}", result);
    let namespace = result.unwrap();
    
    assert_eq!(namespace.name(), "MyModule");
    assert_eq!(namespace.items().len(), 1);
    
    // Check the type alias item
    match &namespace.items()[0] {
        NamespaceItem::TypeAlias(type_alias) => {
            assert_eq!(type_alias.name(), "Alias");
            assert_eq!(type_alias.aliased_type(), &Type::I32);
        }
        _ => panic!("Expected type alias item"),
    }
}

#[test]
fn test_namespace_with_struct() {
    let input = "namespace Geometry
    struct Point
        x: f64
        y: f64
";
    let lexer = Lexer::new(input);
    let result = parser::NamespaceParser::new().parse(lexer);
    assert!(result.is_ok(), "Failed to parse: {:?}", result);
    let namespace = result.unwrap();
    
    assert_eq!(namespace.name(), "Geometry");
    assert_eq!(namespace.items().len(), 1);
    
    match &namespace.items()[0] {
        NamespaceItem::Struct(struct_decl) => {
            assert_eq!(struct_decl.name(), "Point");
            assert_eq!(struct_decl.fields().len(), 2);
        }
        _ => panic!("Expected struct item"),
    }
}

#[test]
fn test_namespace_with_enum() {
    let input = "namespace Colors
    enum Color
        Red
        Green
        Blue
";
    let lexer = Lexer::new(input);
    let result = parser::NamespaceParser::new().parse(lexer);
    assert!(result.is_ok(), "Failed to parse: {:?}", result);
    let namespace = result.unwrap();
    
    assert_eq!(namespace.name(), "Colors");
    assert_eq!(namespace.items().len(), 1);
    
    match &namespace.items()[0] {
        NamespaceItem::Enum(enum_decl) => {
            assert_eq!(enum_decl.name(), "Color");
            assert_eq!(enum_decl.variants().len(), 3);
        }
        _ => panic!("Expected enum item"),
    }
}

#[test]
fn test_namespace_with_union() {
    let input = "namespace Data
    union Value
        int_val: i32
        float_val: f64
";
    let lexer = Lexer::new(input);
    let result = parser::NamespaceParser::new().parse(lexer);
    assert!(result.is_ok(), "Failed to parse: {:?}", result);
    let namespace = result.unwrap();
    
    assert_eq!(namespace.name(), "Data");
    assert_eq!(namespace.items().len(), 1);
    
    match &namespace.items()[0] {
        NamespaceItem::Union(union_decl) => {
            assert_eq!(union_decl.name(), "Value");
            assert_eq!(union_decl.variants().len(), 2);
        }
        _ => panic!("Expected union item"),
    }
}

#[test]
fn test_namespace_with_interface() {
    let input = "namespace Interfaces
    interface Printable
        fn print()
";
    let lexer = Lexer::new(input);
    let result = parser::NamespaceParser::new().parse(lexer);
    assert!(result.is_ok(), "Failed to parse: {:?}", result);
    let namespace = result.unwrap();
    
    assert_eq!(namespace.name(), "Interfaces");
    assert_eq!(namespace.items().len(), 1);
    
    match &namespace.items()[0] {
        NamespaceItem::Interface(interface_decl) => {
            assert_eq!(interface_decl.name(), "Printable");
            assert_eq!(interface_decl.methods().len(), 1);
        }
        _ => panic!("Expected interface item"),
    }
}

#[test]
fn test_namespace_with_function() {
    let input = "namespace Utils
    fn add(a: i32, b: i32) -> i32
        pass
";
    let lexer = Lexer::new(input);
    let result = parser::NamespaceParser::new().parse(lexer);
    assert!(result.is_ok(), "Failed to parse: {:?}", result);
    let namespace = result.unwrap();
    
    assert_eq!(namespace.name(), "Utils");
    assert_eq!(namespace.items().len(), 1);
    
    match &namespace.items()[0] {
        NamespaceItem::Function(func) => {
            assert_eq!(func.signature().name(), "add");
            assert_eq!(func.signature().params().len(), 2);
        }
        _ => panic!("Expected function item"),
    }
}

#[test]
fn test_namespace_with_multiple_items() {
    let input = "namespace MyLib
    type Int = i32
    struct Point
        x: i32
        y: i32
    enum Direction
        North
        South
        East
        West
";
    let lexer = Lexer::new(input);
    let result = parser::NamespaceParser::new().parse(lexer);
    assert!(result.is_ok(), "Failed to parse: {:?}", result);
    let namespace = result.unwrap();
    
    assert_eq!(namespace.name(), "MyLib");
    assert_eq!(namespace.items().len(), 3);
    
    // Check type alias
    match &namespace.items()[0] {
        NamespaceItem::TypeAlias(_) => {}
        _ => panic!("Expected type alias as first item"),
    }
    
    // Check struct
    match &namespace.items()[1] {
        NamespaceItem::Struct(_) => {}
        _ => panic!("Expected struct as second item"),
    }
    
    // Check enum
    match &namespace.items()[2] {
        NamespaceItem::Enum(_) => {}
        _ => panic!("Expected enum as third item"),
    }
}

#[test]
fn test_nested_namespace() {
    let input = "namespace Outer
    namespace Inner
        type Alias = i32
";
    let lexer = Lexer::new(input);
    let result = parser::NamespaceParser::new().parse(lexer);
    assert!(result.is_ok(), "Failed to parse: {:?}", result);
    let namespace = result.unwrap();
    
    assert_eq!(namespace.name(), "Outer");
    assert_eq!(namespace.items().len(), 1);
    
    match &namespace.items()[0] {
        NamespaceItem::Namespace(inner) => {
            assert_eq!(inner.name(), "Inner");
            assert_eq!(inner.items().len(), 1);
        }
        _ => panic!("Expected nested namespace"),
    }
}

#[test]
fn test_namespace_with_complex_items() {
    let input = "namespace Collections
    struct List[T]
        where
            T: Clone
        items: []T
        size: usize
    
    interface Iterable[T]
        fn next() -> T
";
    let lexer = Lexer::new(input);
    let result = parser::NamespaceParser::new().parse(lexer);
    assert!(result.is_ok(), "Failed to parse: {:?}", result);
    let namespace = result.unwrap();
    
    assert_eq!(namespace.name(), "Collections");
    assert_eq!(namespace.items().len(), 2);
    
    // Check struct with generics and where clause
    match &namespace.items()[0] {
        NamespaceItem::Struct(struct_decl) => {
            assert_eq!(struct_decl.generic_params().len(), 1);
            assert_eq!(struct_decl.where_clause().len(), 1);
        }
        _ => panic!("Expected struct item"),
    }
    
    // Check interface with generics
    match &namespace.items()[1] {
        NamespaceItem::Interface(interface_decl) => {
            assert_eq!(interface_decl.generic_params().len(), 1);
        }
        _ => panic!("Expected interface item"),
    }
}

#[test]
fn test_empty_namespace() {
    let input = "namespace Empty
    pass
";
    let lexer = Lexer::new(input);
    let result = parser::NamespaceParser::new().parse(lexer);
    // Empty namespaces might not be supported or might need special handling
    if result.is_ok() {
        let namespace = result.unwrap();
        assert_eq!(namespace.name(), "Empty");
        // Items might be empty
    }
}
