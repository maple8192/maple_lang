use crate::parser::node::operator::Operator;

pub mod operator;

#[derive(Debug, Clone)]
pub enum Node {
    Program { functions: Vec<Node> },
    Function { name: String, args_num: usize, variables: Vec<String>, statement: Box<Node> },
    Statement { node: Box<Node> },
    Block { statements: Vec<Node> },
    Return { node: Box<Node> },
    If { condition: Box<Node>, true_case: Box<Node>, false_case: Box<Option<Node>> },
    For { init: Box<Option<Node>>, condition: Box<Option<Node>>, update: Box<Option<Node>>, statement: Box<Node> },
    While { condition: Box<Node>, node: Box<Node> },
    Operator { typ: Operator, lhs: Box<Node>, rhs: Box<Node> },
    Variable { offset: usize },
    FuncCall { function_name: String, arguments: Vec<Node> },
    Number { num: i64 }
}