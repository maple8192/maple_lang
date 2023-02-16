use crate::tokenizer::token::token_type::symbol::Symbol;
use crate::tokenizer::token::token_type::word::Word;

pub mod symbol;
pub mod word;

#[derive(Clone, Eq, PartialEq)]
pub enum TokenType {
    Symbol(Symbol),
    Word(Word),
    Ident(String),
}

impl TokenType {
    pub fn get_len(&self) -> usize {
        match &self {
            TokenType::Symbol(symbol) => symbol.to_str().len(),
            TokenType::Word(word) => word.to_str().len(),
            TokenType::Ident(ident) => ident.len(),
        }
    }
}