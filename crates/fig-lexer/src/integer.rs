use derive_builder::Builder;
use std::fmt::Display;
use serde::Serialize;

use crate::{Token, LexicalError};

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub enum Base {
    Binary,
    Octal,
    #[default]
    Decimal,
    Hex,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub enum IntegerSuffix {
    U8,
    U16,
    U32,
    U64,
    I8,
    I16,
    #[default]
    I32,
    I64,
    USize,
    ISize,
}

impl From<&str> for IntegerSuffix {
    fn from(s: &str) -> Self {
        match s {
            "u8" => IntegerSuffix::U8,
            "u16" => IntegerSuffix::U16,
            "u32" => IntegerSuffix::U32,
            "u64" => IntegerSuffix::U64,
            "i8" => IntegerSuffix::I8,
            "i16" => IntegerSuffix::I16,
            "i32" => IntegerSuffix::I32,
            "i64" => IntegerSuffix::I64,
            "usize" => IntegerSuffix::USize,
            "isize" => IntegerSuffix::ISize,
            _ => panic!("Invalid integer suffix: {}", s),
        }
    }
}

impl Display for IntegerSuffix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let suffix_str = match self {
            IntegerSuffix::U8 => "u8",
            IntegerSuffix::U16 => "u16",
            IntegerSuffix::U32 => "u32",
            IntegerSuffix::U64 => "u64",
            IntegerSuffix::I8 => "i8",
            IntegerSuffix::I16 => "i16",
            IntegerSuffix::I32 => "i32",
            IntegerSuffix::I64 => "i64",
            IntegerSuffix::USize => "usize",
            IntegerSuffix::ISize => "isize",
        };
        write!(f, "{}", suffix_str)
    }
}

#[derive(Debug, Clone, PartialEq, Builder, Serialize)]
pub struct IntegerLiteral {
    #[builder(default)]
    base: Base,

    digits: String,

    #[builder(default)]
    suffix: Option<IntegerSuffix>,
}

impl IntegerLiteral {
    pub fn builder() -> IntegerLiteralBuilder {
        IntegerLiteralBuilder::default()
    }

    /// Get the base of the integer literal
    pub fn base(&self) -> &Base {
        &self.base
    }

    /// Get the digits as a string
    pub fn digits(&self) -> &str {
        &self.digits
    }

    /// Get the suffix if present
    pub fn suffix(&self) -> Option<&IntegerSuffix> {
        self.suffix.as_ref()
    }

    /// Parse the integer literal as a u64
    pub fn as_u64(&self) -> Result<u64, std::num::ParseIntError> {
        let radix = match self.base {
            Base::Binary => 2,
            Base::Octal => 8,
            Base::Decimal => 10,
            Base::Hex => 16,
        };
        u64::from_str_radix(&self.digits, radix)
    }

    /// Parse the integer literal as a usize
    pub fn as_usize(&self) -> Result<usize, std::num::ParseIntError> {
        self.as_u64().map(|v| v as usize)
    }
}

impl Display for IntegerLiteral {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let base_prefix = match self.base {
            Base::Binary => "0b",
            Base::Octal => "0o",
            Base::Decimal => "",
            Base::Hex => "0x",
        };
        let suffix_str = if let Some(suf) = &self.suffix {
            suf.to_string()
        } else {
            String::new()
        };
        write!(f, "{}{}{}", base_prefix, self.digits, suffix_str)
    }
}

pub fn parse_integer(lex: &mut logos::Lexer<Token>) -> Result<IntegerLiteral, LexicalError> {
    let raw = lex.slice();
    let span = lex.span();

    // List of valid suffixes
    const SUFFIXES: &[&str] = &[
        "u8", "u16", "u32", "u64", "i8", "i16", "i32", "i64", "usize", "isize",
    ];

    // Find valid suffix at the end
    let suffix = SUFFIXES.iter().find(|&suf| raw.ends_with(suf)).copied();

    let number_part = if let Some(suf) = &suffix {
        // Remove suffix from number part
        &raw[..raw.len() - suf.len()]
    } else {
        raw
    };

    // 2️⃣ Detect base
    let (base, digits) = if let Some(start) = number_part.strip_prefix("0b") {
        (Base::Binary, start)
    } else if let Some(start) = number_part.strip_prefix("0o") {
        (Base::Octal, start)
    } else if let Some(start) = number_part.strip_prefix("0x") {
        (Base::Hex, start)
    } else {
        (Base::Decimal, number_part)
    };

    // 3️⃣ Remove underscores
    let cleaned_digits = digits.replace('_', "");
    
    // Validate that digits are not empty
    if cleaned_digits.is_empty() {
        return Err(LexicalError::InvalidInteger {
            span,
            reason: "integer literal cannot be empty".to_string(),
        });
    }

    Ok(IntegerLiteral {
        base,
        digits: cleaned_digits,
        suffix: suffix.map(IntegerSuffix::from),
    })
}
