use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Copy, Clone, Eq, PartialEq, EnumIter)]
pub enum Word {
    Function,
    If,
    Else,
    For,
    While,
    Loop,
    Int,
    Float,
}

impl Word {
    pub fn to_str(&self) -> &str {
        match self {
            Word::Function => "fn",
            Word::If => "if",
            Word::Else => "else",
            Word::For => "for",
            Word::While => "while",
            Word::Loop => "loop",
            Word::Int => "int",
            Word::Float => "flt",
        }
    }

    pub fn get_list() -> Vec<Word> {
        Word::iter().collect::<Vec<Word>>()
    }
}