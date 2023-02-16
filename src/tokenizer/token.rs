use crate::tokenizer::token::token_type::TokenType;

pub mod token_type;

pub struct Token {
    pub typ: TokenType,
    pub line: usize,
    pub pos: usize,
}

impl Token {
    pub fn new(typ: TokenType, line: usize, pos: usize) -> Self {
        Token { typ, line, pos }
    }
}