#[derive(Copy, Clone, Eq, PartialEq)]
pub enum Symbol {
    OpenBracket,
    CloseBracket,
    OpenBrace,
    CloseBrace,
}

impl Symbol {
    pub fn to_str(&self) -> &str {
        match self {
            Symbol::OpenBracket => "(",
            Symbol::CloseBracket => ")",
            Symbol::OpenBrace => "{",
            Symbol::CloseBrace => "}",
        }
    }

    pub fn get_len_order_list() -> Vec<Symbol> {
        vec![
            Symbol::OpenBracket,
            Symbol::CloseBracket,
            Symbol::OpenBrace,
            Symbol::CloseBrace,
        ]
    }

    pub fn get_reserved_char_list() -> Vec<char> {
        vec!['(', ')', '{', '}']
    }
}