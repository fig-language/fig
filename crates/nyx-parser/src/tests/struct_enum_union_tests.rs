// Comprehensive tests for Struct, Enum, and Union parsing

#[cfg(test)]
mod struct_tests {
    use crate::ast::*;
    use crate::{Lexer, parser};

    #[test]
    fn test_simple_struct() {
        let input = "struct Point
    x: i32
    y: i32
";
        let lexer = Lexer::new(input);
        let result = parser::StructParser::new().parse(lexer);
        assert!(result.is_ok(), "Failed to parse: {:?}", result);
        let s = result.unwrap();
        assert_eq!(s.name(), "Point");
        assert_eq!(s.fields().len(), 2);
        assert_eq!(s.generic_params().len(), 0);
        assert_eq!(s.where_clause().len(), 0);
        assert_eq!(s.requires_clause().len(), 0);

        assert_eq!(s.fields()[0].name(), "x");
        assert_eq!(s.fields()[0].ty(), &Type::I32);
        assert_eq!(s.fields()[1].name(), "y");
        assert_eq!(s.fields()[1].ty(), &Type::I32);
    }

    #[test]
    fn test_struct_with_generic_params() {
        let input = "struct Container[T]
    value: T
";
        let lexer = Lexer::new(input);
        let result = parser::StructParser::new().parse(lexer);
        assert!(result.is_ok(), "Failed to parse: {:?}", result);
        let s = result.unwrap();
        assert_eq!(s.name(), "Container");
        assert_eq!(s.generic_params().len(), 1);
        assert_eq!(s.fields().len(), 1);

        if let GenericParameter::Type { name, bounds } = &s.generic_params()[0] {
            assert_eq!(name, "T");
            assert_eq!(bounds.len(), 0);
        } else {
            panic!("Expected type parameter");
        }
    }

    #[test]
    fn test_struct_with_bounded_generic() {
        let input = "struct Container[T: Clone]
    value: T
";
        let lexer = Lexer::new(input);
        let result = parser::StructParser::new().parse(lexer);
        assert!(result.is_ok(), "Failed to parse: {:?}", result);
        let s = result.unwrap();

        if let GenericParameter::Type { name, bounds } = &s.generic_params()[0] {
            assert_eq!(name, "T");
            assert_eq!(bounds.len(), 1);
        } else {
            panic!("Expected bounded type parameter");
        }
    }

    #[test]
    fn test_struct_with_multiple_bounded_generics() {
        let input = "struct Pair[T: Clone + Copy, U: Send]
    first: T
    second: U
";
        let lexer = Lexer::new(input);
        let result = parser::StructParser::new().parse(lexer);
        assert!(result.is_ok(), "Failed to parse: {:?}", result);
        let s = result.unwrap();
        assert_eq!(s.generic_params().len(), 2);
        assert_eq!(s.fields().len(), 2);

        // Check T: Clone + Copy
        if let GenericParameter::Type { name, bounds } = &s.generic_params()[0] {
            assert_eq!(name, "T");
            assert_eq!(bounds.len(), 2);
        }

        // Check U: Send
        if let GenericParameter::Type { name, bounds } = &s.generic_params()[1] {
            assert_eq!(name, "U");
            assert_eq!(bounds.len(), 1);
        }
    }

    #[test]
    fn test_struct_with_const_param() {
        let input = "struct Array[const N: usize]
    data: i32
";
        let lexer = Lexer::new(input);
        let result = parser::StructParser::new().parse(lexer);
        assert!(result.is_ok(), "Failed to parse: {:?}", result);
        let s = result.unwrap();
        assert_eq!(s.generic_params().len(), 1);

        if let GenericParameter::Const { name, ty } = &s.generic_params()[0] {
            assert_eq!(name, "N");
            assert_eq!(ty, &Type::USize);
        } else {
            panic!("Expected const parameter");
        }
    }

    #[test]
    fn test_struct_with_where_clause() {
        let input = "struct Container[T, U]
    where
        T: Clone
        U: Copy
    first: T
    second: U
";
        let lexer = Lexer::new(input);
        let result = parser::StructParser::new().parse(lexer);
        assert!(result.is_ok(), "Failed to parse: {:?}", result);
        let s = result.unwrap();
        assert_eq!(s.where_clause().len(), 2);

        // Check T: Clone
        if let GenericParameter::Type { name, bounds } = &s.where_clause()[0] {
            assert_eq!(name, "T");
            assert_eq!(bounds.len(), 1);
            if let Type::Named {
                name: bound_name, ..
            } = &bounds[0]
            {
                assert_eq!(bound_name, "Clone");
            }
        }

        // Check U: Copy
        if let GenericParameter::Type { name, bounds } = &s.where_clause()[1] {
            assert_eq!(name, "U");
            assert_eq!(bounds.len(), 1);
        }
    }

    #[test]
    fn test_struct_with_requires_clause() {
        let input = "struct Container[T]
    requires
        Clone
        Send
    value: T
";
        let lexer = Lexer::new(input);
        let result = parser::StructParser::new().parse(lexer);
        assert!(result.is_ok(), "Failed to parse: {:?}", result);
        let s = result.unwrap();
        assert_eq!(s.requires_clause().len(), 2);

        if let Type::Named { name, .. } = &s.requires_clause()[0] {
            assert_eq!(name, "Clone");
        }
        if let Type::Named { name, .. } = &s.requires_clause()[1] {
            assert_eq!(name, "Send");
        }
    }

    #[test]
    fn test_struct_with_requires_and_where() {
        let input = "struct Complex[T, U] 
    requires
        Debug
    where
        T: Clone
        U: Copy
    first: T
    second: U
";
        let lexer = Lexer::new(input);
        let result = parser::StructParser::new().parse(lexer);
        assert!(result.is_ok(), "Failed to parse: {:?}", result);
        let s = result.unwrap();
        assert_eq!(s.requires_clause().len(), 1);
        assert_eq!(s.where_clause().len(), 2);
    }

    #[test]
    fn test_struct_with_complex_types() {
        let input = "struct ComplexStruct
    pointer: ?*i32
    array: []u8
    reference: *mut bool
";
        let lexer = Lexer::new(input);
        let result = parser::StructParser::new().parse(lexer);
        assert!(result.is_ok(), "Failed to parse: {:?}", result);
        let s = result.unwrap();
        assert_eq!(s.fields().len(), 3);

        // Check pointer field
        assert_eq!(s.fields()[0].name(), "pointer");
        if let Type::Pointer { element_type, nullable, mutable } = s.fields()[0].ty() {
            assert_eq!(nullable, &true);
            assert_eq!(mutable, &false);
            assert_eq!(**element_type, Type::I32);
        } else {
            panic!("Expected pointer type");
        }

        // Check array field
        assert_eq!(s.fields()[1].name(), "array");
        if let Type::Array { element_type, size } = s.fields()[1].ty() {
            assert_eq!(size, &None);
            assert_eq!(**element_type, Type::U8);
        } else {
            panic!("Expected array type");
        }

        // Check reference field
        assert_eq!(s.fields()[2].name(), "reference");
        if let Type::Pointer { element_type, nullable, mutable } = s.fields()[2].ty() {
            assert_eq!(nullable, &false);
            assert_eq!(mutable, &true);
            assert_eq!(**element_type, Type::Bool);
        } else {
            panic!("Expected reference type");
        }
    }

    #[test]
    fn test_struct_with_single_field() {
        let input = "struct Wrapper
    value: i32
";
        let lexer = Lexer::new(input);
        let result = parser::StructParser::new().parse(lexer);
        assert!(result.is_ok(), "Failed to parse: {:?}", result);
        let s = result.unwrap();
        assert_eq!(s.fields().len(), 1);
    }
}

#[cfg(test)]
mod enum_tests {
    use crate::ast::*;
    use crate::{Lexer, parser};

    #[test]
    fn test_simple_enum() {
        let input = "enum Color
    Red
    Green
    Blue
";
        let lexer = Lexer::new(input);
        let result = parser::EnumParser::new().parse(lexer);
        assert!(result.is_ok(), "Failed to parse: {:?}", result);
        let e = result.unwrap();
        assert_eq!(e.name(), "Color");
        assert_eq!(e.variants().len(), 3);
        assert_eq!(e.generic_params().len(), 0);
        assert_eq!(e.where_clause().len(), 0);
        assert_eq!(e.requires_clause().len(), 0);

        assert_eq!(e.variants()[0].name(), "Red");
        assert_eq!(e.variants()[0].value(), &None);
        assert_eq!(e.variants()[1].name(), "Green");
        assert_eq!(e.variants()[2].name(), "Blue");
        assert_eq!(e.representation(), &None);
    }

    #[test]
    fn test_enum_with_representation() {
        let input = "enum[u8] Color
    Red
    Green
    Blue
";
        let lexer = Lexer::new(input);
        let result = parser::EnumParser::new().parse(lexer);
        assert!(result.is_ok(), "Failed to parse: {:?}", result);
        let e = result.unwrap();
        assert_eq!(e.name(), "Color");
        assert_eq!(e.variants().len(), 3);
        
        // Check representation
        assert_eq!(e.representation(), &Some(Type::U8));
    }

    #[test]
    fn test_enum_with_i32_representation() {
        let input = "enum[i32] Status
    Ok = 0
    Error = -1
    Pending = 1
";
        let lexer = Lexer::new(input);
        let result = parser::EnumParser::new().parse(lexer);
        assert!(result.is_ok(), "Failed to parse: {:?}", result);
        let e = result.unwrap();
        assert_eq!(e.name(), "Status");
        assert_eq!(e.representation(), &Some(Type::I32));
        assert_eq!(e.variants().len(), 3);
    }

    #[test]
    fn test_enum_with_usize_representation() {
        let input = "enum[usize] Flags
    None = 0
    Read = 1
    Write = 2
    Execute = 4
";
        let lexer = Lexer::new(input);
        let result = parser::EnumParser::new().parse(lexer);
        assert!(result.is_ok(), "Failed to parse: {:?}", result);
        let e = result.unwrap();
        assert_eq!(e.representation(), &Some(Type::USize));
    }

    #[test]
    fn test_enum_with_representation_and_generics() {
        let input = "enum[u16] Option[T]
    Some
    None
";
        let lexer = Lexer::new(input);
        let result = parser::EnumParser::new().parse(lexer);
        assert!(result.is_ok(), "Failed to parse: {:?}", result);
        let e = result.unwrap();
        assert_eq!(e.representation(), &Some(Type::U16));
        assert_eq!(e.generic_params().len(), 1);
    }

    #[test]
    fn test_enum_with_discriminants() {
        let input = "enum Status
    Ok = 0
    Error = 1
    Pending = 2
";
        let lexer = Lexer::new(input);
        let result = parser::EnumParser::new().parse(lexer);
        assert!(result.is_ok(), "Failed to parse: {:?}", result);
        let e = result.unwrap();
        assert_eq!(e.variants().len(), 3);

        // Check that variants have values
        assert!(e.variants()[0].value().is_some());
        assert!(e.variants()[1].value().is_some());
        assert!(e.variants()[2].value().is_some());
    }

    #[test]
    fn test_enum_mixed_discriminants() {
        let input = "enum Mixed
    A
    B = 10
    C
";
        let lexer = Lexer::new(input);
        let result = parser::EnumParser::new().parse(lexer);
        assert!(result.is_ok(), "Failed to parse: {:?}", result);
        let e = result.unwrap();
        assert_eq!(e.variants().len(), 3);

        assert!(e.variants()[0].value().is_none());
        assert!(e.variants()[1].value().is_some());
        assert!(e.variants()[2].value().is_none());
    }

    #[test]
    fn test_enum_with_generic_params() {
        let input = "enum Option[T]
    Some
    None
";
        let lexer = Lexer::new(input);
        let result = parser::EnumParser::new().parse(lexer);
        assert!(result.is_ok(), "Failed to parse: {:?}", result);
        let e = result.unwrap();
        assert_eq!(e.name(), "Option");
        assert_eq!(e.generic_params().len(), 1);

        if let GenericParameter::Type { name, bounds } = &e.generic_params()[0] {
            assert_eq!(name, "T");
            assert_eq!(bounds.len(), 0);
        } else {
            panic!("Expected type parameter");
        }
    }

    #[test]
    fn test_enum_with_bounded_generic() {
        let input = "enum Result[T: Clone, E: Copy]
    Ok
    Err
";
        let lexer = Lexer::new(input);
        let result = parser::EnumParser::new().parse(lexer);
        assert!(result.is_ok(), "Failed to parse: {:?}", result);
        let e = result.unwrap();
        assert_eq!(e.generic_params().len(), 2);

        // Check T: Clone
        if let GenericParameter::Type { name, bounds } = &e.generic_params()[0] {
            assert_eq!(name, "T");
            assert_eq!(bounds.len(), 1);
        }

        // Check E: Copy
        if let GenericParameter::Type { name, bounds } = &e.generic_params()[1] {
            assert_eq!(name, "E");
            assert_eq!(bounds.len(), 1);
        }
    }

    #[test]
    fn test_enum_with_where_clause() {
        let input = "enum Container[T, E]
    where
        T: Clone
        E: Send
    Some
    None
";
        let lexer = Lexer::new(input);
        let result = parser::EnumParser::new().parse(lexer);
        assert!(result.is_ok(), "Failed to parse: {:?}", result);
        let e = result.unwrap();
        assert_eq!(e.where_clause().len(), 2);

        // Check T: Clone
        if let GenericParameter::Type { name, bounds } = &e.where_clause()[0] {
            assert_eq!(name, "T");
            assert_eq!(bounds.len(), 1);
        }

        // Check E: Send
        if let GenericParameter::Type { name, bounds } = &e.where_clause()[1] {
            assert_eq!(name, "E");
            assert_eq!(bounds.len(), 1);
        }
    }

    #[test]
    fn test_enum_with_requires_clause() {
        let input = "enum MyEnum[T]
    requires
        Clone
        Copy
    Variant1
    Variant2
";
        let lexer = Lexer::new(input);
        let result = parser::EnumParser::new().parse(lexer);
        assert!(result.is_ok(), "Failed to parse: {:?}", result);
        let e = result.unwrap();
        assert_eq!(e.requires_clause().len(), 2);

        if let Type::Named { name, .. } = &e.requires_clause()[0] {
            assert_eq!(name, "Clone");
        }
        if let Type::Named { name, .. } = &e.requires_clause()[1] {
            assert_eq!(name, "Copy");
        }
    }

    #[test]
    fn test_enum_with_requires_and_where() {
        let input = "enum Complex[T, E] 
    requires
        Debug
    where
        T: Clone
        E: Copy
    VariantA
    VariantB
";
        let lexer = Lexer::new(input);
        let result = parser::EnumParser::new().parse(lexer);
        assert!(result.is_ok(), "Failed to parse: {:?}", result);
        let e = result.unwrap();
        assert_eq!(e.requires_clause().len(), 1);
        assert_eq!(e.where_clause().len(), 2);
    }

    #[test]
    fn test_enum_single_variant() {
        let input = "enum Unit
    Value
";
        let lexer = Lexer::new(input);
        let result = parser::EnumParser::new().parse(lexer);
        assert!(result.is_ok(), "Failed to parse: {:?}", result);
        let e = result.unwrap();
        assert_eq!(e.variants().len(), 1);
        assert_eq!(e.variants()[0].name(), "Value");
    }

    #[test]
    fn test_enum_with_const_param() {
        let input = "enum Array[const N: usize]
    Empty
    Full
";
        let lexer = Lexer::new(input);
        let result = parser::EnumParser::new().parse(lexer);
        assert!(result.is_ok(), "Failed to parse: {:?}", result);
        let e = result.unwrap();
        assert_eq!(e.generic_params().len(), 1);

        if let GenericParameter::Const { name, ty } = &e.generic_params()[0] {
            assert_eq!(name, "N");
            assert_eq!(ty, &Type::USize);
        } else {
            panic!("Expected const parameter");
        }
    }
}

#[cfg(test)]
mod union_tests {
    use crate::ast::*;
    use crate::{Lexer, parser};

    #[test]
    fn test_simple_union() {
        let input = "union Value
    int_val: i32
    float_val: f32
";
        let lexer = Lexer::new(input);
        let result = parser::UnionParser::new().parse(lexer);
        assert!(result.is_ok(), "Failed to parse: {:?}", result);
        let u = result.unwrap();
        assert_eq!(u.name(), "Value");
        assert_eq!(u.variants().len(), 2);
        assert_eq!(u.generic_params().len(), 0);
        assert_eq!(u.where_clause().len(), 0);
        assert_eq!(u.requires_clause().len(), 0);

        assert_eq!(u.variants()[0].name(), "int_val");
        assert_eq!(u.variants()[0].ty(), &Type::I32);
        assert_eq!(u.variants()[1].name(), "float_val");
        assert_eq!(u.variants()[1].ty(), &Type::F32);
    }

    #[test]
    fn test_union_with_generic_params() {
        let input = "union Container[T]
    value: T
    pointer: *T
";
        let lexer = Lexer::new(input);
        let result = parser::UnionParser::new().parse(lexer);
        assert!(result.is_ok(), "Failed to parse: {:?}", result);
        let u = result.unwrap();
        assert_eq!(u.name(), "Container");
        assert_eq!(u.generic_params().len(), 1);

        if let GenericParameter::Type { name, bounds } = &u.generic_params()[0] {
            assert_eq!(name, "T");
            assert_eq!(bounds.len(), 0);
        } else {
            panic!("Expected type parameter");
        }
    }

    #[test]
    fn test_union_with_bounded_generic() {
        let input = "union Data[T: Clone, U: Copy]
    first: T
    second: U
";
        let lexer = Lexer::new(input);
        let result = parser::UnionParser::new().parse(lexer);
        assert!(result.is_ok(), "Failed to parse: {:?}", result);
        let u = result.unwrap();
        assert_eq!(u.generic_params().len(), 2);

        // Check T: Clone
        if let GenericParameter::Type { name, bounds } = &u.generic_params()[0] {
            assert_eq!(name, "T");
            assert_eq!(bounds.len(), 1);
        }

        // Check U: Copy
        if let GenericParameter::Type { name, bounds } = &u.generic_params()[1] {
            assert_eq!(name, "U");
            assert_eq!(bounds.len(), 1);
        }
    }

    #[test]
    fn test_union_with_where_clause() {
        let input = "union Container[T, U]
    where
        T: Clone
        U: Copy
    first: T
    second: U
";
        let lexer = Lexer::new(input);
        let result = parser::UnionParser::new().parse(lexer);
        assert!(result.is_ok(), "Failed to parse: {:?}", result);
        let u = result.unwrap();
        assert_eq!(u.where_clause().len(), 2);

        // Check T: Clone
        if let GenericParameter::Type { name, bounds } = &u.where_clause()[0] {
            assert_eq!(name, "T");
            assert_eq!(bounds.len(), 1);
        }

        // Check U: Copy
        if let GenericParameter::Type { name, bounds } = &u.where_clause()[1] {
            assert_eq!(name, "U");
            assert_eq!(bounds.len(), 1);
        }
    }

    #[test]
    fn test_union_with_requires_clause() {
        let input = "union MyUnion[T]
    requires
        Clone
        Send
    variant1: T
    variant2: i32
";
        let lexer = Lexer::new(input);
        let result = parser::UnionParser::new().parse(lexer);
        assert!(result.is_ok(), "Failed to parse: {:?}", result);
        let u = result.unwrap();
        assert_eq!(u.requires_clause().len(), 2);

        if let Type::Named { name, .. } = &u.requires_clause()[0] {
            assert_eq!(name, "Clone");
        }
        if let Type::Named { name, .. } = &u.requires_clause()[1] {
            assert_eq!(name, "Send");
        }
    }

    #[test]
    fn test_union_with_requires_and_where() {
        let input = "union Complex[T, U]
    requires
        Debug
    where
        T: Clone
        U: Copy
    first: T
    second: U
";
        let lexer = Lexer::new(input);
        let result = parser::UnionParser::new().parse(lexer);
        assert!(result.is_ok(), "Failed to parse: {:?}", result);
        let u = result.unwrap();
        assert_eq!(u.requires_clause().len(), 1);
        assert_eq!(u.where_clause().len(), 2);
    }

    #[test]
    fn test_union_with_complex_types() {
        let input = "union ComplexUnion
    pointer: *i32
    array: []u8
    reference: ?*mut bool
";
        let lexer = Lexer::new(input);
        let result = parser::UnionParser::new().parse(lexer);
        assert!(result.is_ok(), "Failed to parse: {:?}", result);
        let u = result.unwrap();
        assert_eq!(u.variants().len(), 3);

        // Check pointer variant
        assert_eq!(u.variants()[0].name(), "pointer");
        if let Type::Pointer { element_type: inner, nullable, mutable } = u.variants()[0].ty() {
            assert_eq!(**inner, Type::I32);
            assert_eq!(nullable, &false);
            assert_eq!(mutable, &false);
        } else {
            panic!("Expected pointer type");
        }

        // Check array variant
        assert_eq!(u.variants()[1].name(), "array");
        if let Type::Array { element_type, size } = u.variants()[1].ty() {
            assert_eq!(size, &None);
            assert_eq!(element_type.as_ref(), &Type::U8);
        } else {
            panic!("Expected array type");
        }

        // Check reference variant
        assert_eq!(u.variants()[2].name(), "reference");
        if let Type::Pointer { element_type, nullable, mutable } = u.variants()[2].ty() {
            assert_eq!(**element_type, Type::Bool);
            assert_eq!(nullable, &true);
            assert_eq!(mutable, &true);
        } else {
            panic!("Expected reference type");
        }
    }

    #[test]
    fn test_union_single_variant() {
        let input = "union Single
    value: i32
";
        let lexer = Lexer::new(input);
        let result = parser::UnionParser::new().parse(lexer);
        assert!(result.is_ok(), "Failed to parse: {:?}", result);
        let u = result.unwrap();
        assert_eq!(u.variants().len(), 1);
    }

    #[test]
    fn test_union_with_const_param() {
        let input = "union Array[const N: usize]
    data: i32
    size: usize
";
        let lexer = Lexer::new(input);
        let result = parser::UnionParser::new().parse(lexer);
        assert!(result.is_ok(), "Failed to parse: {:?}", result);
        let u = result.unwrap();
        assert_eq!(u.generic_params().len(), 1);

        if let GenericParameter::Const { name, ty } = &u.generic_params()[0] {
            assert_eq!(name, "N");
            assert_eq!(ty, &Type::USize);
        } else {
            panic!("Expected const parameter");
        }
    }
}
