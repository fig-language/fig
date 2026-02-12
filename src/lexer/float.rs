use derive_builder::Builder;

use crate::lexer::Token;

#[derive(Debug, Clone, PartialEq)]
pub enum FloatSuffix {
    F32,
    F64,
}

impl From<&str> for FloatSuffix {
    fn from(s: &str) -> Self {
        match s {
            "f32" => FloatSuffix::F32,
            "f64" => FloatSuffix::F64,
            _ => panic!("Invalid float suffix: {}", s),
        }
    }
}

impl std::fmt::Display for FloatSuffix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            FloatSuffix::F32 => "f32",
            FloatSuffix::F64 => "f64",
        };
        write!(f, "{s}")
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum FloatExponent {
    Positive(u32),
    Negative(u32),
    Unsigned(u32),
}

#[derive(Debug, Clone, PartialEq, Builder)]
pub struct FloatLiteral {
    digits: String,

    #[builder(default)]
    exponent: Option<FloatExponent>,

    #[builder(default)]
    suffix: Option<FloatSuffix>,
}

impl FloatLiteral {
    pub fn builder() -> FloatLiteralBuilder {
        FloatLiteralBuilder::default()
    }
}

impl std::fmt::Display for FloatExponent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FloatExponent::Positive(val) => write!(f, "e+{}", val),
            FloatExponent::Negative(val) => write!(f, "e-{}", val),
            FloatExponent::Unsigned(val) => write!(f, "e{}", val),
        }
    }
}

impl std::fmt::Display for FloatLiteral {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.digits)?;
        if let Some(exp) = &self.exponent {
            write!(f, "{exp}")?;
        }
        if let Some(suf) = &self.suffix {
            write!(f, "{suf}")?;
        }
        Ok(())
    }
}

pub fn parse_float(lex: &mut logos::Lexer<Token>) -> Option<FloatLiteral> {
    let raw = lex.slice();

    // List of valid suffixes
    const SUFFIXES: &[&str] = &["f32", "f64"];

    // Find valid suffix at the end
    let suffix = SUFFIXES.iter().find(|&&suf| raw.ends_with(suf)).copied();

    let number_part = if let Some(suf) = &suffix {
        &raw[..raw.len() - suf.len()]
    } else {
        raw
    };

    // Split exponent if present
    let (digits, exponent) = if let Some(e_idx) = number_part.find(['e', 'E']) {
        let digits = number_part[..e_idx].to_string();
        let exp_str = &number_part[e_idx + 1..];
        let exp_sign = number_part.chars().nth(e_idx + 1);
        let (_, exp_enum) = if let Some(sign) = exp_sign {
            match sign {
                '+' => {
                    let val = exp_str[1..].replace('_', "").parse().ok()?;
                    (val, FloatExponent::Positive(val))
                }
                '-' => {
                    let val = exp_str[1..].replace('_', "").parse().ok()?;
                    (val, FloatExponent::Negative(val))
                }
                c if c.is_ascii_digit() => {
                    let val = exp_str.replace('_', "").parse().ok()?;
                    (val, FloatExponent::Unsigned(val))
                }
                _ => return None,
            }
        } else {
            return None;
        };
        (digits, Some(exp_enum))
    } else {
        (number_part.to_string(), None)
    };

    Some(FloatLiteral {
        digits,
        exponent,
        suffix: suffix.map(FloatSuffix::from),
    })
}
