#[derive(Copy, Clone, Eq, PartialEq)]
pub enum Word {
    Function,
    If,
    For,
    While,
    Loop,
    Int,
    Float,
    String,
}

impl Word {
    pub fn to_str(&self) -> &str {
        match self {
            Word::Function => "fn",
            Word::If => "if",
            Word::For => "for",
            Word::While => "while",
            Word::Loop => "loop",
            Word::Int => "int",
            Word::Float => "float",
            Word::String => "str",
        }
    }

    pub fn get_list() -> Vec<Self> {
        vec![
            Word::Function,
            Word::If,
            Word::For,
            Word::While,
            Word::Loop,
            Word::Int,
            Word::Float,
            Word::String,
        ]
    }
}