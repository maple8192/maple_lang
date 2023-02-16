use crate::tokenizer::token::token_type::reserved_token::ReservedToken;

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum Word {
    Function,
}

impl ReservedToken for Word {
    fn to_str(&self) -> &str {
        match self {
            Word::Function => "fn",
        }
    }

    const fn get_len_order_list() -> Vec<Self> {
        vec![
            Word::Function,
        ]
    }
}