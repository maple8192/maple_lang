use crate::parser::node::binary_operator::BinaryOperator;
use crate::parser::node::unary_operator::UnaryOperator;

pub mod unary_operator;
pub mod binary_operator;

#[derive(Debug)]
pub enum Node {
    Program { functions: Vec<Node> },
    Function { name: String, variables: Vec<String>, statements: Vec<Node> },
    Statement { nodes: Vec<Node> },
    UnaryOperator { typ: UnaryOperator, node: Box<Node> },
    BinaryOperator { typ: BinaryOperator, lhs: Box<Node>, rhs: Box<Node> },
    Variable { offset: usize },
    Number { num: i64 }
}