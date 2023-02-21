use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Copy, Clone, Eq, PartialEq, EnumIter)]
pub enum Symbol {
    Add,
    Sub,
    Mul,
    Div,
    Rem,
    Increment,
    Decrement,
    Power,
    Root,
    BitNot,
    BitAnd,
    BitXor,
    BitOr,
    Equal,
    NotEqual,
    Less,
    LessOrEqual,
    Greater,
    GreaterOrEqual,
    Not,
    And,
    Or,
    Create,
    Assign,
    AddAssign,
    SubAssign,
    MulAssign,
    DivAssign,
    RemAssign,
    ChangeMin,
    ChangeMax,
    Exchange,
    OpenBracket,
    CloseBracket,
    OpenBrace,
    CloseBrace,
    Comma,
    End,
}

impl Symbol {
    pub fn to_str(&self) -> &str {
        match self {
            Symbol::Add => "+",
            Symbol::Sub => "-",
            Symbol::Mul => "*",
            Symbol::Div => "/",
            Symbol::Rem => "%",
            Symbol::Increment => "++",
            Symbol::Decrement => "--",
            Symbol::Power => "**",
            Symbol::Root => "//",
            Symbol::BitNot => "~",
            Symbol::BitAnd => "&",
            Symbol::BitXor => "^",
            Symbol::BitOr => "|",
            Symbol::Equal => "==",
            Symbol::NotEqual => "!=",
            Symbol::Less => "<",
            Symbol::LessOrEqual => "<==",
            Symbol::Greater => ">",
            Symbol::GreaterOrEqual => ">==",
            Symbol::Not => "!",
            Symbol::And => "&&",
            Symbol::Or => "||",
            Symbol::Create => ":=",
            Symbol::Assign => "=",
            Symbol::AddAssign => "+=",
            Symbol::SubAssign => "-=",
            Symbol::MulAssign => "*=",
            Symbol::DivAssign => "/=",
            Symbol::RemAssign => "%=",
            Symbol::ChangeMin => "<=",
            Symbol::ChangeMax => ">=",
            Symbol::Exchange => "<=>",
            Symbol::OpenBracket => "(",
            Symbol::CloseBracket => ")",
            Symbol::OpenBrace => "{",
            Symbol::CloseBrace => "}",
            Symbol::Comma => ",",
            Symbol::End => ";",
        }
    }

    pub fn get_len_order_list() -> Vec<Symbol> {
        let mut symbols = Symbol::iter().collect::<Vec<Symbol>>();
        symbols.sort_by(|a, b| { (-(a.to_str().len() as isize)).cmp(&(-(b.to_str().len() as isize))) });
        symbols
    }

    pub fn get_symbol_char_list() -> Vec<char> {
        let mut symbols = Symbol::iter().map(|s| { s.to_str().chars().nth(0).unwrap() }).collect::<Vec<char>>();
        symbols.sort();
        symbols.dedup();
        symbols
    }
}