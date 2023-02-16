use crate::tokenizer::token::token_type::reserved_token::ReservedToken;

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum Symbol {
    OpenBracket,
    CloseBracket,
    OpenBrace,
    CloseBrace,
}

impl ReservedToken for Symbol {
    fn to_str(&self) -> &str {
        match self {
            Symbol::OpenBracket => "(",
            Symbol::CloseBracket => ")",
            Symbol::OpenBrace => "{",
            Symbol::CloseBrace => "}",
        }
    }

    const fn get_len_order_list() -> Vec<Symbol> {
        vec![
            Symbol::OpenBracket,
            Symbol::CloseBracket,
            Symbol::OpenBrace,
            Symbol::CloseBrace,
        ]
    }
}

impl Symbol {
    pub const fn get_reserved_char_list() -> Vec<char> {
        vec!['(', ')', '{', '}']
    }
}