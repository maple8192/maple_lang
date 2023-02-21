use std::collections::VecDeque;
use crate::tokenizer::token::Token;
use crate::tokenizer::token::token_type::symbol::Symbol;
use crate::tokenizer::token::token_type::TokenType;
use crate::tokenizer::token::token_type::word::Word;

pub mod token;

pub fn tokenize(src: &str) -> Result<VecDeque<Token>, String> {
    let mut tokens = VecDeque::new();

    let mut line = 0;
    let mut pos = 0;

    let mut index = 0;
    while index < src.len() {
        match src.chars().nth(index).unwrap() {
            '\n' => {
                index += 1;
                line += 1;
                pos = 0;
            },
            ' ' | '\t' | '\r' => {
                index += 1;
                pos += 1;
            }
            _ => {
                let new_token_type = create_token(&src[src.char_indices().nth(index).unwrap().0..]);
                if let Some(new_token_type) = new_token_type {
                    tokens.push_back(Token::new(new_token_type.clone(), line, pos));

                    let token_len = new_token_type.get_len();
                    index += token_len;
                    pos += token_len;
                } else {
                    let message = format!("Undefined Token({}:{})", line, pos);
                    return Err(message);
                }
            },
        }
    }

    Ok(tokens)
}

fn create_token(target: &str) -> Option<TokenType> {
    let first_char = target.chars().next().unwrap();
    if Symbol::get_reserved_char_list().contains(&first_char) {
        create_symbol_token(target)
    } else {
        Some(create_word_token(target))
    }
}

fn create_symbol_token(target: &str) -> Option<TokenType> {
    let list = Symbol::get_len_order_list();
    let symbol = list.iter().find(|symbol| target.starts_with(symbol.to_str()));
    if let Some(symbol) = symbol {
        Some(TokenType::Symbol(*symbol))
    } else {
        None
    }
}

fn create_word_token(target: &str) -> TokenType {
    let mut word = String::new();
    for i in 0..target.len() {
        let c = target.chars().nth(i).unwrap();
        if c == ' ' || c == '\n' || c == '\t' || Symbol::get_reserved_char_list().contains(&c) {
            let list = Word::get_list();
            let reserved = list.iter().find(|reserved| word == reserved.to_str().to_string());
            return if let Some(reserved) = reserved {
                TokenType::Word(*reserved)
            } else {
                TokenType::Ident(word)
            }
        }

        word.push(c);
    }

    TokenType::Ident(word)
}