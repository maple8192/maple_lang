pub mod node;

use crate::parser::node::binary_operator::BinaryOperator;
use crate::parser::node::Node;
use crate::parser::node::unary_operator::UnaryOperator;
use crate::tokenizer::token::Token;
use crate::tokenizer::token::token_type::symbol::Symbol;
use crate::tokenizer::token::token_type::TokenType;
use crate::tokenizer::token::token_type::word::Word;

pub fn parse(tokens: Vec<Token>) -> Result<Node, String> {
    program(&tokens)
}

fn program(tokens: &Vec<Token>) -> Result<Node, String> {
    let mut pos = 0;

    let mut functions = Vec::new();

    while tokens[pos].typ != TokenType::Eof {
        let function = function(tokens, &mut pos)?;

        functions.push(function);
    }

    Ok(Node::Program { functions })
}

fn function(tokens: &Vec<Token>, pos: &mut usize) -> Result<Node, String> {
    if tokens[*pos].typ == TokenType::Word(Word::Function) {
        *pos += 1;
        if let TokenType::Ident(function_name) = &tokens[*pos].typ {
            *pos += 1;

            let mut variables = Vec::new();

            if tokens[*pos].typ == TokenType::Symbol(Symbol::OpenSquare) {
                *pos += 1;

                let mut first = true;
                while tokens[*pos].typ != TokenType::Symbol(Symbol::CloseSquare) {
                    if !first {
                        if tokens[*pos].typ == TokenType::Symbol(Symbol::Comma) {
                            *pos += 1;
                        } else {
                            return Err(format!("Unexpected Token ({}:{})", tokens[*pos].line, tokens[*pos].pos));
                        }
                    }
                    first = false;

                    if let TokenType::Ident(argument_name) = &tokens[*pos].typ {
                        *pos += 1;

                        variables.push(argument_name.clone());
                    } else {
                        return Err(format!("Unexpected Token ({}:{})", tokens[*pos].line, tokens[*pos].pos));
                    }
                }
                *pos += 1;
            }

            if tokens[*pos].typ == TokenType::Symbol(Symbol::OpenBrace) {
                *pos += 1;

                let mut statements = Vec::new();

                while tokens[*pos].typ != TokenType::Symbol(Symbol::CloseBrace) {
                    let statement = statement(tokens, pos, &mut variables)?;

                    statements.push(statement);
                }
                *pos += 1;

                Ok(Node::Function { name: function_name.clone(), variables, statements })
            } else {
                Err(format!("Unexpected Token ({}:{})", tokens[*pos].line, tokens[*pos].pos))
            }
        } else {
            Err(format!("Unexpected Token ({}:{})", tokens[*pos].line, tokens[*pos].pos))
        }
    } else {
        Err(format!("Unexpected Token ({}:{})", tokens[*pos].line, tokens[*pos].pos))
    }
}

fn statement(tokens: &Vec<Token>, pos: &mut usize, variables: &mut Vec<String>) -> Result<Node, String> {
    let expression = expression(tokens, pos, variables)?;

    if tokens[*pos].typ == TokenType::Symbol(Symbol::End) {
        *pos += 1;

        Ok(Node::Statement { nodes: vec![expression] })
    } else {
        Err(format!("Unexpected Token ({}:{})", tokens[*pos].line, tokens[*pos].pos))
    }
}

fn expression(tokens: &Vec<Token>, pos: &mut usize, variables: &mut Vec<String>) -> Result<Node, String> {
    exchange(tokens, pos, variables)
}

fn exchange(tokens: &Vec<Token>, pos: &mut usize, variables: &mut Vec<String>) -> Result<Node, String> {
    let node = assign(tokens, pos, variables)?;

    if tokens[*pos].typ == TokenType::Symbol(Symbol::Exchange) {
        *pos += 1;

        Ok(Node::BinaryOperator { typ: BinaryOperator::Exchange, lhs: Box::new(node), rhs: Box::new(assign(tokens, pos, variables)?) })
    } else {
        Ok(node)
    }
}

fn assign(tokens: &Vec<Token>, pos: &mut usize, variables: &mut Vec<String>) -> Result<Node, String> {
    let node = or(tokens, pos, variables)?;

    let operator = match tokens[*pos].typ {
        TokenType::Symbol(Symbol::Assign) => { *pos += 1; return Ok(Node::BinaryOperator { typ: BinaryOperator::Assign, lhs: Box::new(node), rhs: Box::new(assign(tokens, pos, variables)?) }) },
        TokenType::Symbol(Symbol::AddAssign) => BinaryOperator::Add,
        TokenType::Symbol(Symbol::SubAssign) => BinaryOperator::Sub,
        TokenType::Symbol(Symbol::MulAssign) => BinaryOperator::Mul,
        TokenType::Symbol(Symbol::DivAssign) => BinaryOperator::Div,
        TokenType::Symbol(Symbol::RemAssign) => BinaryOperator::Rem,
        TokenType::Symbol(Symbol::PowerAssign) => BinaryOperator::Power,
        TokenType::Symbol(Symbol::RootAssign) => BinaryOperator::Root,
        TokenType::Symbol(Symbol::AndAssign) => BinaryOperator::BitAnd,
        TokenType::Symbol(Symbol::XorAssign) => BinaryOperator::BitXor,
        TokenType::Symbol(Symbol::OrAssign) => BinaryOperator::BitOr,
        TokenType::Symbol(Symbol::LShiftAssign) => BinaryOperator::LShift,
        TokenType::Symbol(Symbol::RShiftAssign) => BinaryOperator::RShift,
        TokenType::Symbol(Symbol::ChangeMin) => { *pos += 1; return Ok(Node::BinaryOperator { typ: BinaryOperator::ChangeMin, lhs: Box::new(node), rhs: Box::new(assign(tokens, pos, variables)?) }) },
        TokenType::Symbol(Symbol::ChangeMax) => { *pos += 1; return Ok(Node::BinaryOperator { typ: BinaryOperator::ChangeMax, lhs: Box::new(node), rhs: Box::new(assign(tokens, pos, variables)?) }) },
        _ => return Ok(node),
    };
    *pos += 1;

    Ok(Node::BinaryOperator { typ: BinaryOperator::Assign, lhs: Box::new(node.clone()), rhs: Box::new(Node::BinaryOperator { typ: operator, lhs: Box::new(node), rhs: Box::new(assign(tokens, pos, variables)?) }) })
}

fn or(tokens: &Vec<Token>, pos: &mut usize, variables: &mut Vec<String>) -> Result<Node, String> {
    let mut node = and(tokens, pos, variables)?;

    loop {
        let operator = match tokens[*pos].typ {
            TokenType::Symbol(Symbol::Or) => BinaryOperator::Or,
            _ => return Ok(node),
        };
        *pos += 1;

        node = Node::BinaryOperator { typ: operator, lhs: Box::new(node), rhs: Box::new(and(tokens, pos, variables)?) };
    }
}

fn and(tokens: &Vec<Token>, pos: &mut usize, variables: &mut Vec<String>) -> Result<Node, String> {
    let mut node = bit_or(tokens, pos, variables)?;

    loop {
        let operator = match tokens[*pos].typ {
            TokenType::Symbol(Symbol::And) => BinaryOperator::And,
            _ => return Ok(node),
        };
        *pos += 1;

        node = Node::BinaryOperator { typ: operator, lhs: Box::new(node), rhs: Box::new(bit_or(tokens, pos, variables)?) };
    }
}

fn bit_or(tokens: &Vec<Token>, pos: &mut usize, variables: &mut Vec<String>) -> Result<Node, String> {
    let mut node = bit_xor(tokens, pos, variables)?;

    loop {
        let operator = match tokens[*pos].typ {
            TokenType::Symbol(Symbol::BitOr) => BinaryOperator::BitOr,
            _ => return Ok(node),
        };
        *pos += 1;

        node = Node::BinaryOperator { typ: operator, lhs: Box::new(node), rhs: Box::new(bit_xor(tokens, pos, variables)?) };
    }
}

fn bit_xor(tokens: &Vec<Token>, pos: &mut usize, variables: &mut Vec<String>) -> Result<Node, String> {
    let mut node = bit_and(tokens, pos, variables)?;

    loop {
        let operator = match tokens[*pos].typ {
            TokenType::Symbol(Symbol::BitXor) => BinaryOperator::BitXor,
            _ => return Ok(node),
        };
        *pos += 1;

        node = Node::BinaryOperator { typ: operator, lhs: Box::new(node), rhs: Box::new(bit_and(tokens, pos, variables)?) };
    }
}

fn bit_and(tokens: &Vec<Token>, pos: &mut usize, variables: &mut Vec<String>) -> Result<Node, String> {
    let mut node = equality(tokens, pos, variables)?;

    loop {
        let operator = match tokens[*pos].typ {
            TokenType::Symbol(Symbol::BitAnd) => BinaryOperator::BitAnd,
            _ => return Ok(node),
        };
        *pos += 1;

        node = Node::BinaryOperator { typ: operator, lhs: Box::new(node), rhs: Box::new(equality(tokens, pos, variables)?) };
    }
}

fn equality(tokens: &Vec<Token>, pos: &mut usize, variables: &mut Vec<String>) -> Result<Node, String> {
    let mut node = relational(tokens, pos, variables)?;

    loop {
        let operator = match tokens[*pos].typ {
            TokenType::Symbol(Symbol::Equal) => BinaryOperator::Equal,
            TokenType::Symbol(Symbol::NotEqual) => BinaryOperator::NotEqual,
            _ => return Ok(node),
        };
        *pos += 1;

        node = Node::BinaryOperator { typ: operator, lhs: Box::new(node), rhs: Box::new(relational(tokens, pos, variables)?) };
    }
}

fn relational(tokens: &Vec<Token>, pos: &mut usize, variables: &mut Vec<String>) -> Result<Node, String> {
    let mut node = shift(tokens, pos, variables)?;

    loop {
        let operator = match tokens[*pos].typ {
            TokenType::Symbol(Symbol::Less) => BinaryOperator::Less,
            TokenType::Symbol(Symbol::LessOrEqual) => BinaryOperator::LessOrEqual,
            TokenType::Symbol(Symbol::Greater) => BinaryOperator::Greater,
            TokenType::Symbol(Symbol::GreaterOrEqual) => BinaryOperator::GreaterOrEqual,
            _ => return Ok(node),
        };
        *pos += 1;

        node = Node::BinaryOperator { typ: operator, lhs: Box::new(node), rhs: Box::new(shift(tokens, pos, variables)?) };
    }
}

fn shift(tokens: &Vec<Token>, pos: &mut usize, variables: &mut Vec<String>) -> Result<Node, String> {
    let mut node = add(tokens, pos, variables)?;

    loop {
        let operator = match tokens[*pos].typ {
            TokenType::Symbol(Symbol::LShift) => BinaryOperator::LShift,
            TokenType::Symbol(Symbol::RShift) => BinaryOperator::RShift,
            _ => return Ok(node),
        };
        *pos += 1;

        node = Node::BinaryOperator { typ: operator, lhs: Box::new(node), rhs: Box::new(add(tokens, pos, variables)?) };
    }
}

fn add(tokens: &Vec<Token>, pos: &mut usize, variables: &mut Vec<String>) -> Result<Node, String> {
    let mut node = mul(tokens, pos, variables)?;

    loop {
        let operator = match tokens[*pos].typ {
            TokenType::Symbol(Symbol::Add) => BinaryOperator::Add,
            TokenType::Symbol(Symbol::Sub) => BinaryOperator::Sub,
            _ => return Ok(node),
        };
        *pos += 1;

        node = Node::BinaryOperator { typ: operator, lhs: Box::new(node), rhs: Box::new(add(tokens, pos, variables)?) };
    }
}

fn mul(tokens: &Vec<Token>, pos: &mut usize, variables: &mut Vec<String>) -> Result<Node, String> {
    let mut node = power_root(tokens, pos, variables)?;

    loop {
        let operator = match tokens[*pos].typ {
            TokenType::Symbol(Symbol::Mul) => BinaryOperator::Mul,
            TokenType::Symbol(Symbol::Div) => BinaryOperator::Div,
            TokenType::Symbol(Symbol::Rem) => BinaryOperator::Rem,
            _ => return Ok(node),
        };
        *pos += 1;

        node = Node::BinaryOperator { typ: operator, lhs: Box::new(node), rhs: Box::new(power_root(tokens, pos, variables)?) };
    }
}

fn power_root(tokens: &Vec<Token>, pos: &mut usize, variables: &mut Vec<String>) -> Result<Node, String> {
    let node = unary(tokens, pos, variables)?;

    let operator = match tokens[*pos].typ {
        TokenType::Symbol(Symbol::Power) => BinaryOperator::Power,
        TokenType::Symbol(Symbol::Root) => BinaryOperator::Root,
        _ => return Ok(node),
    };
    *pos += 1;

    Ok(Node::BinaryOperator { typ: operator, lhs: Box::new(node), rhs: Box::new(power_root(tokens, pos, variables)?) })
}

fn unary(tokens: &Vec<Token>, pos: &mut usize, variables: &mut Vec<String>) -> Result<Node, String> {
    let operator = match tokens[*pos].typ {
        TokenType::Symbol(Symbol::Sub) => UnaryOperator::Minus,
        TokenType::Symbol(Symbol::BitNot) => UnaryOperator::BitNot,
        TokenType::Symbol(Symbol::Not) => UnaryOperator::Not,
        _ => return primary(tokens, pos, variables),
    };
    *pos += 1;

    Ok(Node::UnaryOperator { typ: operator, node: Box::new(primary(tokens, pos, variables)?) })
}

fn primary(tokens: &Vec<Token>, pos: &mut usize, variables: &mut Vec<String>) -> Result<Node, String> {
    if tokens[*pos].typ == TokenType::Symbol(Symbol::OpenBracket) {
        *pos += 1;

        let node = expression(tokens, pos, variables)?;

        if tokens[*pos].typ == TokenType::Symbol(Symbol::CloseBracket) {
            *pos += 1;
            Ok(node)
        } else {
            Err(format!("Unexpected Token ({}:{})", tokens[*pos].line, tokens[*pos].pos))
        }
    } else if let TokenType::Ident(variable_name) = &tokens[*pos].typ {
        *pos += 1;
        for i in 0..variables.len() {
            if variables[i] == *variable_name {
                let operator = match tokens[*pos].typ {
                    TokenType::Symbol(Symbol::Increment) => UnaryOperator::Increment,
                    TokenType::Symbol(Symbol::Decrement) => UnaryOperator::Decrement,
                    _ => return Ok(Node::Variable { offset: i }),
                };
                *pos += 1;

                return Ok(Node::UnaryOperator { typ: operator, node: Box::new(Node::Variable { offset: i }) });
            }
        }
        let offset = variables.len();
        variables.push(variable_name.clone());
        Ok(Node::Variable { offset })
    } else if let TokenType::Number(num) = &tokens[*pos].typ {
        *pos += 1;
        Ok(Node::Number { num: *num })
    } else {
        Err(format!("Unexpected Token ({}:{})", tokens[*pos].line, tokens[*pos].pos))
    }
}