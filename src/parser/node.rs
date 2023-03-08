use crate::parser::node::operator::Operator;

pub mod operator;

#[derive(Debug, Clone)]
pub enum Node {
    Program { functions: Vec<Node> },
    Function { name: String, args_num: usize, variables: Vec<String>, statements: Vec<Node> },
    Statement { nodes: Vec<Node> },
    Operator { typ: Operator, lhs: Box<Node>, rhs: Box<Node> },
    Variable { offset: usize },
    Number { num: i64 }
}