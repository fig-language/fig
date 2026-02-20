use std::fmt;
use std::num::ParseIntError;
use std::ops::Range;

/// Detailed lexical error type with position information
#[derive(Debug, Clone, PartialEq)]
pub enum LexicalError {
    /// Invalid integer literal (e.g., overflow, invalid digits)
    InvalidInteger {
        span: Range<usize>,
        reason: String,
    },
    
    /// Invalid float literal
    InvalidFloat {
        span: Range<usize>,
        reason: String,
    },
    
    /// Invalid character literal
    InvalidCharLiteral {
        span: Range<usize>,
        reason: String,
    },
    
    /// Invalid string literal (e.g., unclosed string)
    InvalidStringLiteral {
        span: Range<usize>,
        reason: String,
    },
    
    /// Invalid escape sequence in string or character literal
    InvalidEscapeSequence {
        span: Range<usize>,
        sequence: String,
    },
    
    /// Unexpected character or sequence
    UnexpectedCharacter {
        span: Range<usize>,
        character: char,
    },
    
    /// Generic error for unrecognized tokens
    UnrecognizedToken {
        span: Range<usize>,
        text: String,
    },
    
    /// Default error variant (for invalid tokens that don't match any pattern)
    InvalidToken,
}

impl LexicalError {
    /// Create an error from the lexer with span information
    pub fn from_lexer(lex: &logos::Lexer<'_, crate::Token>) -> Self {
        let span = lex.span();
        let slice = lex.slice();
        
        // Try to identify what kind of invalid token this is
        if let Some(first_char) = slice.chars().next() {
            if first_char.is_ascii_control() && first_char != '\n' && first_char != '\t' {
                // Control character
                LexicalError::UnexpectedCharacter {
                    span,
                    character: first_char,
                }
            } else if !first_char.is_ascii() {
                // Non-ASCII character
                LexicalError::UnexpectedCharacter {
                    span,
                    character: first_char,
                }
            } else {
                // Generic unrecognized token
                LexicalError::UnrecognizedToken {
                    span,
                    text: slice.to_string(),
                }
            }
        } else {
            LexicalError::InvalidToken
        }
    }
    
    /// Get the span of the error
    pub fn span(&self) -> Option<Range<usize>> {
        match self {
            LexicalError::InvalidInteger { span, .. }
            | LexicalError::InvalidFloat { span, .. }
            | LexicalError::InvalidCharLiteral { span, .. }
            | LexicalError::InvalidStringLiteral { span, .. }
            | LexicalError::InvalidEscapeSequence { span, .. }
            | LexicalError::UnexpectedCharacter { span, .. }
            | LexicalError::UnrecognizedToken { span, .. } => Some(span.clone()),
            LexicalError::InvalidToken => None,
        }
    }
    
    /// Convert byte offset to line and column
    pub fn position_from_source(source: &str, offset: usize) -> (usize, usize) {
        let mut line = 1;
        let mut column = 1;
        
        for (i, ch) in source.chars().enumerate() {
            if i >= offset {
                break;
            }
            if ch == '\n' {
                line += 1;
                column = 1;
            } else {
                column += 1;
            }
        }
        
        (line, column)
    }
}

impl Default for LexicalError {
    fn default() -> Self {
        LexicalError::InvalidToken
    }
}

impl fmt::Display for LexicalError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LexicalError::InvalidInteger { span, reason } => {
                write!(f, "Invalid integer literal at {}..{}: {}", span.start, span.end, reason)
            }
            LexicalError::InvalidFloat { span, reason } => {
                write!(f, "Invalid float literal at {}..{}: {}", span.start, span.end, reason)
            }
            LexicalError::InvalidCharLiteral { span, reason } => {
                write!(f, "Invalid character literal at {}..{}: {}", span.start, span.end, reason)
            }
            LexicalError::InvalidStringLiteral { span, reason } => {
                write!(f, "Invalid string literal at {}..{}: {}", span.start, span.end, reason)
            }
            LexicalError::InvalidEscapeSequence { span, sequence } => {
                write!(f, "Invalid escape sequence '{}' at {}..{}", sequence, span.start, span.end)
            }
            LexicalError::UnexpectedCharacter { span, character } => {
                write!(f, "Unexpected character '{:?}' at {}..{}", character, span.start, span.end)
            }
            LexicalError::UnrecognizedToken { span, text } => {
                write!(f, "Unrecognized token '{}' at {}..{}", text, span.start, span.end)
            }
            LexicalError::InvalidToken => {
                write!(f, "Invalid token")
            }
        }
    }
}

impl std::error::Error for LexicalError {}

/// Implement From<ParseIntError> for integer parsing errors
impl From<ParseIntError> for LexicalError {
    fn from(err: ParseIntError) -> Self {
        use std::num::IntErrorKind::*;
        let reason = match err.kind() {
            PosOverflow => "integer overflow (too large)".to_string(),
            NegOverflow => "integer underflow (too small)".to_string(),
            InvalidDigit => "invalid digit in integer literal".to_string(),
            _ => format!("failed to parse integer: {}", err),
        };
        
        // We don't have span information here, so use an empty span
        // The callback that uses this will need to add the span
        LexicalError::InvalidInteger {
            span: 0..0,
            reason,
        }
    }
}
