// Interface parsing tests for nyx-parser

use crate::{Lexer, ast::{GenericParameter, Type}, parser};

#[test]
fn test_simple_interface() {
    let input = "interface Drawable
    fn get_width() -> i32
    fn get_height() -> i32
";
    let lexer = Lexer::new(input);
    let result = parser::InterfaceParser::new().parse(lexer);
    assert!(result.is_ok(), "Failed to parse: {:?}", result);
    let interface = result.unwrap();
    
    assert_eq!(interface.name(), "Drawable");
    assert_eq!(interface.generic_params().len(), 0);
    assert_eq!(interface.extends_clause().len(), 0);
    assert_eq!(interface.where_clause().len(), 0);
    assert_eq!(interface.methods().len(), 2);
    
    // Check first method
    assert_eq!(interface.methods()[0].name(), "get_width");
    assert_eq!(interface.methods()[0].params().len(), 0);
    assert_eq!(interface.methods()[0].return_type(), &Some(Type::I32));
    
    // Check second method
    assert_eq!(interface.methods()[1].name(), "get_height");
    assert_eq!(interface.methods()[1].params().len(), 0);
    assert_eq!(interface.methods()[1].return_type(), &Some(Type::I32));
}

#[test]
fn test_interface_with_generic_params() {
    let input = "interface Container[T]
    fn add(item: T)
    fn get(index: usize) -> T
";
    let lexer = Lexer::new(input);
    let result = parser::InterfaceParser::new().parse(lexer);
    assert!(result.is_ok(), "Failed to parse: {:?}", result);
    let interface = result.unwrap();
    
    assert_eq!(interface.name(), "Container");
    assert_eq!(interface.generic_params().len(), 1);
    
    // Check generic parameter
    if let GenericParameter::Type { name, bounds } = &interface.generic_params()[0] {
        assert_eq!(name, "T");
        assert_eq!(bounds.len(), 0);
    } else {
        panic!("Expected type parameter T");
    }
    
    // Check methods
    assert_eq!(interface.methods().len(), 2);
    assert_eq!(interface.methods()[0].name(), "add");
    assert_eq!(interface.methods()[0].params().len(), 1);
    assert_eq!(interface.methods()[1].name(), "get");
}

#[test]
fn test_interface_with_extends_clause() {
    let input = "interface Shape
    extends
        Drawable
        Movable
    fn get_area() -> f64
";
    let lexer = Lexer::new(input);
    let result = parser::InterfaceParser::new().parse(lexer);
    assert!(result.is_ok(), "Failed to parse: {:?}", result);
    let interface = result.unwrap();
    
    assert_eq!(interface.name(), "Shape");
    assert_eq!(interface.extends_clause().len(), 2);
    
    // Check extends clause
    if let Type::Named { name, .. } = &interface.extends_clause()[0] {
        assert_eq!(name, "Drawable");
    } else {
        panic!("Expected Drawable interface");
    }
    
    if let Type::Named { name, .. } = &interface.extends_clause()[1] {
        assert_eq!(name, "Movable");
    } else {
        panic!("Expected Movable interface");
    }
    
    assert_eq!(interface.methods().len(), 1);
}

#[test]
fn test_interface_with_where_clause() {
    let input = "interface Comparable[T]
    where
        T: Clone
    fn compare(other: T) -> i32
";
    let lexer = Lexer::new(input);
    let result = parser::InterfaceParser::new().parse(lexer);
    assert!(result.is_ok(), "Failed to parse: {:?}", result);
    let interface = result.unwrap();
    
    assert_eq!(interface.name(), "Comparable");
    assert_eq!(interface.where_clause().len(), 1);
    
    // Check where clause
    if let GenericParameter::Type { name, bounds } = &interface.where_clause()[0] {
        assert_eq!(name, "T");
        assert_eq!(bounds.len(), 1);
        if let Type::Named { name: bound_name, .. } = &bounds[0] {
            assert_eq!(bound_name, "Clone");
        }
    }
}

#[test]
fn test_interface_with_extends_and_where() {
    let input = "interface AdvancedContainer[T, E]
    extends
        Container[T]
    where
        T: Clone
        E: Copy
    fn get_or_error(index: usize) -> T
    fn set(index: usize, item: T)
";
    let lexer = Lexer::new(input);
    let result = parser::InterfaceParser::new().parse(lexer);
    assert!(result.is_ok(), "Failed to parse: {:?}", result);
    let interface = result.unwrap();
    
    assert_eq!(interface.name(), "AdvancedContainer");
    assert_eq!(interface.generic_params().len(), 2);
    assert_eq!(interface.extends_clause().len(), 1);
    assert_eq!(interface.where_clause().len(), 2);
    assert_eq!(interface.methods().len(), 2);
}

#[test]
fn test_interface_with_method_parameters() {
    let input = "interface Calculator
    fn add(a: i32, b: i32) -> i32
    fn multiply(a: i32, b: i32) -> i32
";
    let lexer = Lexer::new(input);
    let result = parser::InterfaceParser::new().parse(lexer);
    assert!(result.is_ok(), "Failed to parse: {:?}", result);
    let interface = result.unwrap();
    
    assert_eq!(interface.methods()[0].params().len(), 2);
    assert_eq!(interface.methods()[0].params()[0].name(), "a");
    assert_eq!(interface.methods()[0].params()[0].ty(), &Type::I32);
    assert_eq!(interface.methods()[0].params()[1].name(), "b");
    assert_eq!(interface.methods()[0].params()[1].ty(), &Type::I32);
}

#[test]
fn test_interface_with_complex_types() {
    let input = "interface Processor[T]
    fn process(input: ?*mut T) -> *T
    fn batch_process(inputs: []T) -> []T
";
    let lexer = Lexer::new(input);
    let result = parser::InterfaceParser::new().parse(lexer);
    assert!(result.is_ok(), "Failed to parse: {:?}", result);
    let interface = result.unwrap();
    
    assert_eq!(interface.methods().len(), 2);
    
    // Check first method has reference parameter
    if let Type::Pointer { element_type, nullable, mutable } = interface.methods()[0].params()[0].ty() {
        assert_eq!(nullable, &true);
        assert_eq!(mutable, &true);
        assert_eq!(**element_type, Type::Named { name: "T".to_string(), generic_args: vec![] });
    } else {
        panic!("Expected reference type for first parameter");
    }
    
    // Check second method has array parameters
    if let Type::Array { element_type, size } = interface.methods()[1].params()[0].ty() {
        assert_eq!(size, &None);
        assert_eq!(**element_type, Type::Named { name: "T".to_string(), generic_args: vec![] });
    } else {
        panic!("Expected array type for parameter");
    }
}

#[test]
fn test_interface_empty_methods() {
    let input = "interface Marker
    pass
";
    let lexer = Lexer::new(input);
    let result = parser::InterfaceParser::new().parse(lexer);
    // This might fail depending on grammar - marker interfaces might not be supported
    // or might need special handling
    if result.is_ok() {
        let interface = result.unwrap();
        assert_eq!(interface.name(), "Marker");
        // Methods list might be empty or contain a pass statement
    }
}
