use lalrpop_util::lalrpop_mod;
use fig_lexer::{IndentLexer, Token};

pub mod ast;
pub mod pretty_print;

#[cfg(test)]
mod tests;

lalrpop_mod!(pub parser);

pub use parser::*;

#[derive(Default, Debug, Clone, PartialEq)]
pub enum LexicalError {
    #[default]
    InvalidToken,
}

impl std::fmt::Display for LexicalError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub type Spanned<Tok, Loc, Error> = Result<(Loc, Tok, Loc), Error>;

pub struct Lexer<'input> {
    indent_lexer: IndentLexer<'input>,
    position: usize,
}

impl<'input> Lexer<'input> {
    pub fn new(input: &'input str) -> Self {
        Self {
            indent_lexer: IndentLexer::new(input),
            position: 0,
        }
    }
}

impl<'input> Iterator for Lexer<'input> {
    type Item = Spanned<Token, usize, LexicalError>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.indent_lexer.next() {
            Some(Ok(token)) => {
                let span = self.indent_lexer.span();
                self.position = span.end;
                Some(Ok((span.start, token, span.end)))
            }
            Some(Err(_)) => {
                let _pos = self.position;
                self.position += 1;
                Some(Err(LexicalError::InvalidToken))
            }
            None => None,
        }
    }
}
