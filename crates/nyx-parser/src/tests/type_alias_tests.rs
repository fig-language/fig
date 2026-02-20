// Additional tests for type aliases in nyx-parser

use crate::{
    Lexer,
    ast::{GenericParameter, Type},
    parser,
};

#[test]
fn test_simple_type_alias() {
    let input = "type MyInt = i32";
    let lexer = Lexer::new(input);
    let result = parser::TypeAliasParser::new().parse(lexer);
    assert!(result.is_ok(), "Failed to parse: {:?}", result);
    let type_alias = result.unwrap();
    assert_eq!(type_alias.name(), "MyInt");
    assert_eq!(type_alias.generic_params().len(), 0);
    assert_eq!(type_alias.aliased_type(), &Type::I32);
    assert_eq!(type_alias.where_clause().len(), 0);
}

#[test]
fn test_type_alias_with_generic_params() {
    let input = "type MyVec[T] = []T";
    let lexer = Lexer::new(input);
    let result = parser::TypeAliasParser::new().parse(lexer);
    assert!(result.is_ok(), "Failed to parse: {:?}", result);
    let type_alias = result.unwrap();
    assert_eq!(type_alias.name(), "MyVec");
    assert_eq!(type_alias.generic_params().len(), 1);

    if let GenericParameter::Type { name, bounds } = &type_alias.generic_params()[0] {
        assert_eq!(name, "T");
        assert_eq!(bounds.len(), 0);
    } else {
        panic!("Expected type parameter");
    }

    // Check aliased type is []T (dynamic array of T)
    if let Type::Array { element_type, size } = type_alias.aliased_type() {
        assert_eq!(size, &None);
        if let Type::Named { name, generic_args } = element_type.as_ref() {
            assert_eq!(name, "T");
            assert_eq!(generic_args.len(), 0);
        } else {
            panic!("Expected named type T");
        }
    } else {
        panic!("Expected array type");
    }
}

#[test]
fn test_type_alias_with_bounded_generic() {
    let input = "type MyRef[T: Copy] = &T";
    let lexer = Lexer::new(input);
    let result = parser::TypeAliasParser::new().parse(lexer);
    assert!(result.is_ok(), "Failed to parse: {:?}", result);
    let type_alias = result.unwrap();
    assert_eq!(type_alias.name(), "MyRef");
    assert_eq!(type_alias.generic_params().len(), 1);

    if let GenericParameter::Type { name, bounds } = &type_alias.generic_params()[0] {
        assert_eq!(name, "T");
        assert_eq!(bounds.len(), 1);
        if let Type::Named {
            name: bound_name, ..
        } = &bounds[0]
        {
            assert_eq!(bound_name, "Copy");
        } else {
            panic!("Expected Copy bound");
        }
    } else {
        panic!("Expected type parameter");
    }
}

#[test]
fn test_type_alias_with_multiple_bounds() {
    let input = "type MyType[T: Clone + Copy] = *T";
    let lexer = Lexer::new(input);
    let result = parser::TypeAliasParser::new().parse(lexer);
    assert!(result.is_ok(), "Failed to parse: {:?}", result);
    let type_alias = result.unwrap();

    if let GenericParameter::Type { name, bounds } = &type_alias.generic_params()[0] {
        assert_eq!(name, "T");
        assert_eq!(bounds.len(), 2);

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
    } else {
        panic!("Expected type parameter");
    }
}

#[test]
fn test_type_alias_with_const_param() {
    let input = "type MyArray[const N: usize] = i32";
    let lexer = Lexer::new(input);
    let result = parser::TypeAliasParser::new().parse(lexer);
    assert!(result.is_ok(), "Failed to parse: {:?}", result);
    let type_alias = result.unwrap();
    assert_eq!(type_alias.generic_params().len(), 1);

    if let GenericParameter::Const { name, ty } = &type_alias.generic_params()[0] {
        assert_eq!(name, "N");
        assert_eq!(ty, &Type::USize);
    } else {
        panic!("Expected const parameter");
    }
}

#[test]
fn test_type_alias_with_where_clause() {
    let input = "type MyResult[T, E]
    where
        T: Clone
        E: Copy
    = i32";
    let lexer = Lexer::new(input);
    let result = parser::TypeAliasParser::new().parse(lexer);
    assert!(result.is_ok(), "Failed to parse: {:?}", result);
    let type_alias = result.unwrap();

    assert_eq!(type_alias.name(), "MyResult");
    assert_eq!(type_alias.generic_params().len(), 2);
    assert_eq!(type_alias.where_clause().len(), 2);

    // Check where clause bounds
    if let GenericParameter::Type { name, bounds } = &type_alias.where_clause()[0] {
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

    if let GenericParameter::Type { name, bounds } = &type_alias.where_clause()[1] {
        assert_eq!(name, "E");
        assert_eq!(bounds.len(), 1);
        if let Type::Named {
            name: bound_name, ..
        } = &bounds[0]
        {
            assert_eq!(bound_name, "Copy");
        }
    } else {
        panic!("Expected E: Copy in where clause");
    }
}

#[test]
fn test_type_alias_with_complex_where_clause() {
    let input = "type ComplexType[T, U, V] 
    where
        T: Clone + Send
        U: Copy
        V: Debug + Display
    = *T";
    let lexer = Lexer::new(input);
    let result = parser::TypeAliasParser::new().parse(lexer);
    assert!(result.is_ok(), "Failed to parse: {:?}", result);
    let type_alias = result.unwrap();

    assert_eq!(type_alias.where_clause().len(), 3);

    // Check T: Clone + Send
    if let GenericParameter::Type { name, bounds } = &type_alias.where_clause()[0] {
        assert_eq!(name, "T");
        assert_eq!(bounds.len(), 2);
    } else {
        panic!("Expected T with multiple bounds");
    }

    // Check U: Copy
    if let GenericParameter::Type { name, bounds } = &type_alias.where_clause()[1] {
        assert_eq!(name, "U");
        assert_eq!(bounds.len(), 1);
    } else {
        panic!("Expected U: Copy");
    }

    // Check V: Debug + Display
    if let GenericParameter::Type { name, bounds } = &type_alias.where_clause()[2] {
        assert_eq!(name, "V");
        assert_eq!(bounds.len(), 2);
    } else {
        panic!("Expected V with multiple bounds");
    }
}

#[test]
fn test_type_alias_with_mixed_params_and_where_clause() {
    let input = "type MixedType[T: Clone, const N: usize]
    where
        T: Send
    = *T";
    let lexer = Lexer::new(input);
    let result = parser::TypeAliasParser::new().parse(lexer);
    assert!(result.is_ok(), "Failed to parse: {:?}", result);
    let type_alias = result.unwrap();

    // Check generic params
    assert_eq!(type_alias.generic_params().len(), 2);

    // First param should be T: Clone
    if let GenericParameter::Type { name, bounds } = &type_alias.generic_params()[0] {
        assert_eq!(name, "T");
        assert_eq!(bounds.len(), 1);
    }

    // Second param should be const N: usize
    if let GenericParameter::Const { name, ty } = &type_alias.generic_params()[1] {
        assert_eq!(name, "N");
        assert_eq!(ty, &Type::USize);
    }

    // Check where clause has T: Send
    assert_eq!(type_alias.where_clause().len(), 1);
    if let GenericParameter::Type { name, bounds } = &type_alias.where_clause()[0] {
        assert_eq!(name, "T");
        assert_eq!(bounds.len(), 1);
        if let Type::Named {
            name: bound_name, ..
        } = &bounds[0]
        {
            assert_eq!(bound_name, "Send");
        }
    }
}

#[test]
fn test_type_alias_pointer_types() {
    let input = "type RawPtr = *Something";
    let lexer = Lexer::new(input);
    let result = parser::TypeAliasParser::new().parse(lexer);
    assert!(result.is_ok());
    let type_alias = result.unwrap();
    assert_eq!(
        type_alias.aliased_type(),
        &Type::Pointer {
            nullable: false,
            mutable: false,
            element_type: Box::new(Type::Named {
                name: "Something".to_string(),
                generic_args: vec![]
            })
        }
    );
}

#[test]
fn test_type_alias_complex_type() {
    let input = "type ComplexArray[T] = [10]i32";
    let lexer = Lexer::new(input);
    let result = parser::TypeAliasParser::new().parse(lexer);
    assert!(result.is_ok(), "Failed to parse: {:?}", result);
    let type_alias = result.unwrap();

    // Should be [10]i32 (array of 10 i32)
    if let Type::Array { element_type, size } = type_alias.aliased_type() {
        assert_eq!(size, &Some(10));
        assert_eq!(element_type.as_ref(), &Type::I32);
    } else {
        panic!("Expected array type");
    }
}
