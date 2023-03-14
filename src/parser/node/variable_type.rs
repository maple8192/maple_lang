#[derive(Debug, Clone)]
pub enum VariableType {
    Int,
    Float,
    Void,
}

impl VariableType {
    pub fn str(&self) -> String {
        match self {
            VariableType::Int => "i64",
            VariableType::Float => "double",
            VariableType::Void => "void",
        }.to_string()
    }
}