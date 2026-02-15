// Comprehensive tests for Function parsing

use crate::ast::*;
use crate::{Lexer, parser};

#[test]
fn test_simple_function() {
    let input = "fn add(x: i32, y: i32) -> i32:
    pass";
    let lexer = Lexer::new(input);
    let result = parser::FunctionParser::new().parse(lexer);
    assert!(result.is_ok(), "Failed to parse: {:?}", result);
    let f = result.unwrap();
    assert_eq!(f.name(), "add");
    assert_eq!(f.params().len(), 2);
    assert_eq!(f.return_type(), &Some(Type::I32));
    assert_eq!(f.generic_params().len(), 0);
    assert_eq!(f.where_clause().len(), 0);

    assert_eq!(f.params()[0].name(), "x");
    assert_eq!(f.params()[0].ty(), &Type::I32);
    assert_eq!(f.params()[1].name(), "y");
    assert_eq!(f.params()[1].ty(), &Type::I32);
}

#[test]
fn test_function_no_params() {
    let input = "fn main() -> i32:
    pass";
    let lexer = Lexer::new(input);
    let result = parser::FunctionParser::new().parse(lexer);
    assert!(result.is_ok(), "Failed to parse: {:?}", result);
    let f = result.unwrap();
    assert_eq!(f.name(), "main");
    assert_eq!(f.params().len(), 0);
    assert_eq!(f.return_type(), &Some(Type::I32));
}

#[test]
fn test_function_no_return_type() {
    let input = "fn print(x: i32):
    pass";
    let lexer = Lexer::new(input);
    let result = parser::FunctionParser::new().parse(lexer);
    assert!(result.is_ok(), "Failed to parse: {:?}", result);
    let f = result.unwrap();
    assert_eq!(f.name(), "print");
    assert_eq!(f.params().len(), 1);
    assert_eq!(f.return_type(), &None);
}

#[test]
fn test_function_with_generic_params() {
    let input = "fn identity[T](x: T) -> T:
    pass";
    let lexer = Lexer::new(input);
    let result = parser::FunctionParser::new().parse(lexer);
    assert!(result.is_ok(), "Failed to parse: {:?}", result);
    let f = result.unwrap();
    assert_eq!(f.name(), "identity");
    assert_eq!(f.generic_params().len(), 1);
    assert_eq!(f.params().len(), 1);

    if let GenericParameter::Type { name, bounds } = &f.generic_params()[0] {
        assert_eq!(name, "T");
        assert_eq!(bounds.len(), 0);
    } else {
        panic!("Expected type parameter");
    }
}

#[test]
fn test_function_with_bounded_generic() {
    let input = "fn clone[T: Clone](x: T) -> T:
    pass";
    let lexer = Lexer::new(input);
    let result = parser::FunctionParser::new().parse(lexer);
    assert!(result.is_ok(), "Failed to parse: {:?}", result);
    let f = result.unwrap();
    assert_eq!(f.generic_params().len(), 1);

    if let GenericParameter::Type { name, bounds } = &f.generic_params()[0] {
        assert_eq!(name, "T");
        assert_eq!(bounds.len(), 1);
        if let Type::Named {
            name: bound_name, ..
        } = &bounds[0]
        {
            assert_eq!(bound_name, "Clone");
        }
    } else {
        panic!("Expected bounded type parameter");
    }
}

#[test]
fn test_function_with_multiple_generics() {
    let input = "fn pair[T, U](x: T, y: U) -> T:
    pass";
    let lexer = Lexer::new(input);
    let result = parser::FunctionParser::new().parse(lexer);
    assert!(result.is_ok(), "Failed to parse: {:?}", result);
    let f = result.unwrap();
    assert_eq!(f.generic_params().len(), 2);
    assert_eq!(f.params().len(), 2);
}

#[test]
fn test_function_with_where_clause() {
    let input = "fn process[T, U](x: T, y: U) -> T
where
    T: Clone
    U: Copy:
    pass";
    let lexer = Lexer::new(input);
    let result = parser::FunctionParser::new().parse(lexer);
    assert!(result.is_ok(), "Failed to parse: {:?}", result);
    let f = result.unwrap();
    assert_eq!(f.where_clause().len(), 2);

    // Check T: Clone
    if let GenericParameter::Type { name, bounds } = &f.where_clause()[0] {
        assert_eq!(name, "T");
        assert_eq!(bounds.len(), 1);
        if let Type::Named {
            name: bound_name, ..
        } = &bounds[0]
        {
            assert_eq!(bound_name, "Clone");
        }
    } else {
        panic!("Expected T: Clone in where clause");
    }

    // Check U: Copy
    if let GenericParameter::Type { name, bounds } = &f.where_clause()[1] {
        assert_eq!(name, "U");
        assert_eq!(bounds.len(), 1);
        if let Type::Named {
            name: bound_name, ..
        } = &bounds[0]
        {
            assert_eq!(bound_name, "Copy");
        }
    } else {
        panic!("Expected U: Copy in where clause");
    }
}

#[test]
fn test_function_with_complex_types() {
    let input = "fn process(ptr: *i32, arr: []u8, ref_val: &bool) -> *u32:
    pass";
    let lexer = Lexer::new(input);
    let result = parser::FunctionParser::new().parse(lexer);
    assert!(result.is_ok(), "Failed to parse: {:?}", result);
    let f = result.unwrap();
    assert_eq!(f.params().len(), 3);

    // Check pointer parameter
    assert_eq!(f.params()[0].name(), "ptr");
    if let Type::TypedPointer(inner) = f.params()[0].ty() {
        assert_eq!(inner.as_ref(), &Type::I32);
    } else {
        panic!("Expected pointer type");
    }

    // Check array parameter
    assert_eq!(f.params()[1].name(), "arr");
    if let Type::Array { element_type, size } = f.params()[1].ty() {
        assert_eq!(size, &None);
        assert_eq!(element_type.as_ref(), &Type::U8);
    } else {
        panic!("Expected array type");
    }

    // Check reference parameter
    assert_eq!(f.params()[2].name(), "ref_val");
    if let Type::Reference(inner) = f.params()[2].ty() {
        assert_eq!(inner.as_ref(), &Type::Bool);
    } else {
        panic!("Expected reference type");
    }

    // Check return type
    if let Some(Type::TypedPointer(inner)) = f.return_type() {
        assert_eq!(inner.as_ref(), &Type::U32);
    } else {
        panic!("Expected pointer return type");
    }
}

#[test]
fn test_function_with_array_return() {
    let input = "fn get_array() -> []i32:
    pass";
    let lexer = Lexer::new(input);
    let result = parser::FunctionParser::new().parse(lexer);
    assert!(result.is_ok(), "Failed to parse: {:?}", result);
    let f = result.unwrap();

    if let Some(Type::Array { element_type, size }) = f.return_type() {
        assert_eq!(size, &None);
        assert_eq!(element_type.as_ref(), &Type::I32);
    } else {
        panic!("Expected array return type");
    }
}

#[test]
fn test_function_with_const_param() {
    let input = "fn array_func[const N: usize](x: i32) -> i32:
    pass";
    let lexer = Lexer::new(input);
    let result = parser::FunctionParser::new().parse(lexer);
    assert!(result.is_ok(), "Failed to parse: {:?}", result);
    let f = result.unwrap();
    assert_eq!(f.generic_params().len(), 1);

    if let GenericParameter::Const { name, ty } = &f.generic_params()[0] {
        assert_eq!(name, "N");
        assert_eq!(ty, &Type::USize);
    } else {
        panic!("Expected const parameter");
    }
}

#[test]
fn test_function_with_mixed_generics() {
    let input = "fn complex[T: Clone, const N: usize, U](x: T, y: U) -> T:
    pass";
    let lexer = Lexer::new(input);
    let result = parser::FunctionParser::new().parse(lexer);
    assert!(result.is_ok(), "Failed to parse: {:?}", result);
    let f = result.unwrap();
    assert_eq!(f.generic_params().len(), 3);

    // Check T: Clone
    if let GenericParameter::Type { name, bounds } = &f.generic_params()[0] {
        assert_eq!(name, "T");
        assert_eq!(bounds.len(), 1);
    }

    // Check const N: usize
    if let GenericParameter::Const { name, ty } = &f.generic_params()[1] {
        assert_eq!(name, "N");
        assert_eq!(ty, &Type::USize);
    }

    // Check U
    if let GenericParameter::Type { name, bounds } = &f.generic_params()[2] {
        assert_eq!(name, "U");
        assert_eq!(bounds.len(), 0);
    }
}

#[test]
fn test_function_with_multiple_bounds() {
    let input = "fn multi_bound[T: Clone + Copy + Send](x: T) -> T:
    pass";
    let lexer = Lexer::new(input);
    let result = parser::FunctionParser::new().parse(lexer);
    assert!(result.is_ok(), "Failed to parse: {:?}", result);
    let f = result.unwrap();

    if let GenericParameter::Type { name, bounds } = &f.generic_params()[0] {
        assert_eq!(name, "T");
        assert_eq!(bounds.len(), 3);

        if let Type::Named {
            name: bound_name, ..
        } = &bounds[0]
        {
            assert_eq!(bound_name, "Clone");
        }
        if let Type::Named {
            name: bound_name, ..
        } = &bounds[1]
        {
            assert_eq!(bound_name, "Copy");
        }
        if let Type::Named {
            name: bound_name, ..
        } = &bounds[2]
        {
            assert_eq!(bound_name, "Send");
        }
    } else {
        panic!("Expected bounded type parameter");
    }
}

#[test]
fn test_function_with_single_param() {
    let input = "fn square(x: i32) -> i32:
    pass";
    let lexer = Lexer::new(input);
    let result = parser::FunctionParser::new().parse(lexer);
    assert!(result.is_ok(), "Failed to parse: {:?}", result);
    let f = result.unwrap();
    assert_eq!(f.params().len(), 1);
}

#[test]
fn test_function_with_trailing_comma() {
    let input = "fn add(x: i32, y: i32,) -> i32:
    pass";
    let lexer = Lexer::new(input);
    let result = parser::FunctionParser::new().parse(lexer);
    assert!(result.is_ok(), "Failed to parse: {:?}", result);
    let f = result.unwrap();
    assert_eq!(f.params().len(), 2);
}
