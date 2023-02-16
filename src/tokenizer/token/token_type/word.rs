#[derive(Copy, Clone, Eq, PartialEq)]
pub enum Word {
    Function,
}

impl Word {
    pub fn to_str(&self) -> &str {
        match self {
            Word::Function => "fn",
        }
    }

    pub fn get_len_order_list() -> Vec<Self> {
        vec![
            Word::Function,
        ]
    }
}