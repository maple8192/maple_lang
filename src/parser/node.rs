use crate::parser::node::operator::Operator;

pub mod operator;

#[derive(Debug, Clone)]
pub enum Node {
    Program { functions: Vec<Node> },
    Function { name: String, args_num: usize, variables: Vec<String>, statements: Vec<Node> },
    Statement { nodes: Vec<Node> },
    Return { node: Box<Node> },
    If { condition: Box<Node>, true_case: Box<Node>, false_case: Box<Option<Node>> },
    Operator { typ: Operator, lhs: Box<Node>, rhs: Box<Node> },
    Variable { offset: usize },
    FuncCall { function_name: String, arguments: Vec<Node> },
    Number { num: i64 }
}