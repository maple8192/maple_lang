pub mod node;

use crate::parser::node::operator::Operator;
use crate::parser::node::Node;
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

            let args_num = variables.len();

            if tokens[*pos].typ == TokenType::Symbol(Symbol::OpenBrace) {
                *pos += 1;

                let mut statements = Vec::new();

                while tokens[*pos].typ != TokenType::Symbol(Symbol::CloseBrace) {
                    let statement = statement(tokens, pos, &mut variables)?;

                    statements.push(statement);
                }
                *pos += 1;

                Ok(Node::Function { name: function_name.clone(), args_num, variables, statements })
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

    Ok(match tokens[*pos].typ {
        TokenType::Symbol(Symbol::Exchange) => { *pos += 1; Node::Operator { typ: Operator::Exchange, lhs: Box::new(node), rhs: Box::new(assign(tokens, pos, variables)?) } }
        _ => node,
    })
}

fn assign(tokens: &Vec<Token>, pos: &mut usize, variables: &mut Vec<String>) -> Result<Node, String> {
    let node = or(tokens, pos, variables)?;

    Ok(match tokens[*pos].typ {
        TokenType::Symbol(Symbol::Assign) => { *pos += 1; Node::Operator { typ: Operator::Assign, lhs: Box::new(node), rhs: Box::new(assign(tokens, pos, variables)?) } },
        TokenType::Symbol(Symbol::AddAssign) => { *pos += 1; Node::Operator { typ: Operator::Assign, lhs: Box::new(node.clone()), rhs: Box::new(Node::Operator { typ: Operator::Add, lhs: Box::new(node), rhs: Box::new(assign(tokens, pos, variables)?) }) } },
        TokenType::Symbol(Symbol::SubAssign) => { *pos += 1; Node::Operator { typ: Operator::Assign, lhs: Box::new(node.clone()), rhs: Box::new(Node::Operator { typ: Operator::Sub, lhs: Box::new(node), rhs: Box::new(assign(tokens, pos, variables)?) }) } },
        TokenType::Symbol(Symbol::MulAssign) => { *pos += 1; Node::Operator { typ: Operator::Assign, lhs: Box::new(node.clone()), rhs: Box::new(Node::Operator { typ: Operator::Mul, lhs: Box::new(node), rhs: Box::new(assign(tokens, pos, variables)?) }) } },
        TokenType::Symbol(Symbol::DivAssign) => { *pos += 1; Node::Operator { typ: Operator::Assign, lhs: Box::new(node.clone()), rhs: Box::new(Node::Operator { typ: Operator::Div, lhs: Box::new(node), rhs: Box::new(assign(tokens, pos, variables)?) }) } },
        TokenType::Symbol(Symbol::RemAssign) => { *pos += 1; Node::Operator { typ: Operator::Assign, lhs: Box::new(node.clone()), rhs: Box::new(Node::Operator { typ: Operator::Rem, lhs: Box::new(node), rhs: Box::new(assign(tokens, pos, variables)?) }) } },
        TokenType::Symbol(Symbol::PowerAssign) => { *pos += 1; Node::Operator { typ: Operator::Assign, lhs: Box::new(node.clone()), rhs: Box::new(Node::Operator { typ: Operator::Power, lhs: Box::new(node), rhs: Box::new(assign(tokens, pos, variables)?) }) } },
        TokenType::Symbol(Symbol::RootAssign) => { *pos += 1; Node::Operator { typ: Operator::Assign, lhs: Box::new(node.clone()), rhs: Box::new(Node::Operator { typ: Operator::Root, lhs: Box::new(node), rhs: Box::new(assign(tokens, pos, variables)?) }) } },
        TokenType::Symbol(Symbol::AndAssign) => { *pos += 1; Node::Operator { typ: Operator::Assign, lhs: Box::new(node.clone()), rhs: Box::new(Node::Operator { typ: Operator::BitAnd, lhs: Box::new(node), rhs: Box::new(assign(tokens, pos, variables)?) }) } },
        TokenType::Symbol(Symbol::XorAssign) => { *pos += 1; Node::Operator { typ: Operator::Assign, lhs: Box::new(node.clone()), rhs: Box::new(Node::Operator { typ: Operator::BitXor, lhs: Box::new(node), rhs: Box::new(assign(tokens, pos, variables)?) }) } },
        TokenType::Symbol(Symbol::OrAssign) => { *pos += 1; Node::Operator { typ: Operator::Assign, lhs: Box::new(node.clone()), rhs: Box::new(Node::Operator { typ: Operator::BitOr, lhs: Box::new(node), rhs: Box::new(assign(tokens, pos, variables)?) }) } },
        TokenType::Symbol(Symbol::LShiftAssign) => { *pos += 1; Node::Operator { typ: Operator::Assign, lhs: Box::new(node.clone()), rhs: Box::new(Node::Operator { typ: Operator::LShift, lhs: Box::new(node), rhs: Box::new(assign(tokens, pos, variables)?) }) } },
        TokenType::Symbol(Symbol::RShiftAssign) => { *pos += 1; Node::Operator { typ: Operator::Assign, lhs: Box::new(node.clone()), rhs: Box::new(Node::Operator { typ: Operator::RShift, lhs: Box::new(node), rhs: Box::new(assign(tokens, pos, variables)?) }) } },
        TokenType::Symbol(Symbol::ChangeMin) => { *pos += 1; Node::Operator { typ: Operator::ChangeMin, lhs: Box::new(node), rhs: Box::new(assign(tokens, pos, variables)?) } },
        TokenType::Symbol(Symbol::ChangeMax) => { *pos += 1; Node::Operator { typ: Operator::ChangeMax, lhs: Box::new(node), rhs: Box::new(assign(tokens, pos, variables)?) } },
        _ => node,
    })
}

fn or(tokens: &Vec<Token>, pos: &mut usize, variables: &mut Vec<String>) -> Result<Node, String> {
    let mut node = and(tokens, pos, variables)?;

    loop {
        node = match tokens[*pos].typ {
            TokenType::Symbol(Symbol::Or) => { *pos += 1; Node::Operator { typ: Operator::Equal, lhs: Box::new(Node::Number { num: 0 }), rhs: Box::new(Node::Operator { typ: Operator::Equal, lhs: Box::new(Node::Number { num: 0}), rhs: Box::new(Node::Operator { typ: Operator::BitOr, lhs: Box::new(node), rhs: Box::new(and(tokens, pos, variables)?) }) }) } },
            _ => return Ok(node),
        }
    }
}

fn and(tokens: &Vec<Token>, pos: &mut usize, variables: &mut Vec<String>) -> Result<Node, String> {
    let mut node = bit_or(tokens, pos, variables)?;

    loop {
        node = match tokens[*pos].typ {
            TokenType::Symbol(Symbol::And) => { *pos += 1; Node::Operator { typ: Operator::BitAnd, lhs: Box::new(Node::Operator { typ: Operator::Equal, lhs: Box::new(Node::Number { num: 0 }), rhs: Box::new(Node::Operator { typ: Operator::Equal, lhs: Box::new(Node::Number { num: 0 }), rhs: Box::new(node) }) }), rhs: Box::new(Node::Operator { typ: Operator::Equal, lhs: Box::new(Node::Number { num: 0 }), rhs: Box::new(Node::Operator { typ: Operator::Equal, lhs: Box::new(Node::Number { num: 0 }), rhs: Box::new(bit_or(tokens, pos, variables)?) }) }) } },
            _ => return Ok(node),
        }
    }
}

fn bit_or(tokens: &Vec<Token>, pos: &mut usize, variables: &mut Vec<String>) -> Result<Node, String> {
    let mut node = bit_xor(tokens, pos, variables)?;

    loop {
        node = match tokens[*pos].typ {
            TokenType::Symbol(Symbol::BitOr) => { *pos += 1; Node::Operator { typ: Operator::BitOr, lhs: Box::new(node), rhs: Box::new(bit_xor(tokens, pos, variables)?) } },
            _ => return Ok(node),
        }
    }
}

fn bit_xor(tokens: &Vec<Token>, pos: &mut usize, variables: &mut Vec<String>) -> Result<Node, String> {
    let mut node = bit_and(tokens, pos, variables)?;

    loop {
        node = match tokens[*pos].typ {
            TokenType::Symbol(Symbol::BitXor) => { *pos += 1; Node::Operator { typ: Operator::BitXor, lhs: Box::new(node), rhs: Box::new(bit_and(tokens, pos, variables)?) } },
            _ => return Ok(node),
        }
    }
}

fn bit_and(tokens: &Vec<Token>, pos: &mut usize, variables: &mut Vec<String>) -> Result<Node, String> {
    let mut node = equality(tokens, pos, variables)?;

    loop {
        node = match tokens[*pos].typ {
            TokenType::Symbol(Symbol::BitAnd) => { *pos += 1; Node::Operator { typ: Operator::BitAnd, lhs: Box::new(node), rhs: Box::new(equality(tokens, pos, variables)?) } },
            _ => return Ok(node),
        }
    }
}

fn equality(tokens: &Vec<Token>, pos: &mut usize, variables: &mut Vec<String>) -> Result<Node, String> {
    let mut node = relational(tokens, pos, variables)?;

    loop {
        node = match tokens[*pos].typ {
            TokenType::Symbol(Symbol::Equal) => { *pos += 1; Node::Operator { typ: Operator::Equal, lhs: Box::new(node), rhs: Box::new(relational(tokens, pos, variables)?) } },
            TokenType::Symbol(Symbol::NotEqual) => { *pos += 1; Node::Operator { typ: Operator::Equal, lhs: Box::new(Node::Number { num: 0 }), rhs: Box::new(Node::Operator { typ: Operator::Equal, lhs: Box::new(node), rhs: Box::new(relational(tokens, pos, variables)?) }) } },
            _ => return Ok(node),
        };
    }
}

fn relational(tokens: &Vec<Token>, pos: &mut usize, variables: &mut Vec<String>) -> Result<Node, String> {
    let mut node = shift(tokens, pos, variables)?;

    loop {
        node = match tokens[*pos].typ {
            TokenType::Symbol(Symbol::Less) => { *pos += 1; Node::Operator { typ: Operator::Less, lhs: Box::new(node), rhs: Box::new(shift(tokens, pos, variables)?) } },
            TokenType::Symbol(Symbol::LessOrEqual) => { *pos += 1; Node::Operator { typ: Operator::Greater, lhs: Box::new(shift(tokens, pos, variables)?), rhs: Box::new(node) } },
            TokenType::Symbol(Symbol::Greater) => { *pos += 1; Node::Operator { typ: Operator::Greater, lhs: Box::new(node), rhs: Box::new(shift(tokens, pos, variables)?) } },
            TokenType::Symbol(Symbol::GreaterOrEqual) => { *pos += 1; Node::Operator { typ: Operator::Less, lhs: Box::new(shift(tokens, pos, variables)?), rhs: Box::new(node) } },
            _ => return Ok(node),
        };
    }
}

fn shift(tokens: &Vec<Token>, pos: &mut usize, variables: &mut Vec<String>) -> Result<Node, String> {
    let mut node = add(tokens, pos, variables)?;

    loop {
        node = match tokens[*pos].typ {
            TokenType::Symbol(Symbol::LShift) => { *pos += 1; Node::Operator { typ: Operator::LShift, lhs: Box::new(node), rhs: Box::new(add(tokens, pos, variables)?) } },
            TokenType::Symbol(Symbol::RShift) => { *pos += 1; Node::Operator { typ: Operator::RShift, lhs: Box::new(node), rhs: Box::new(add(tokens, pos, variables)?) } },
            _ => return Ok(node),
        };
    }
}

fn add(tokens: &Vec<Token>, pos: &mut usize, variables: &mut Vec<String>) -> Result<Node, String> {
    let mut node = mul(tokens, pos, variables)?;

    loop {
        node = match tokens[*pos].typ {
            TokenType::Symbol(Symbol::Add) => { *pos += 1; Node::Operator { typ: Operator::Add, lhs: Box::new(node), rhs: Box::new(add(tokens, pos, variables)?) } },
            TokenType::Symbol(Symbol::Sub) => { *pos += 1; Node::Operator { typ: Operator::Sub, lhs: Box::new(node), rhs: Box::new(add(tokens, pos, variables)?) } },
            _ => return Ok(node),
        };
    }
}

fn mul(tokens: &Vec<Token>, pos: &mut usize, variables: &mut Vec<String>) -> Result<Node, String> {
    let mut node = power_root(tokens, pos, variables)?;

    loop {
        node = match tokens[*pos].typ {
            TokenType::Symbol(Symbol::Mul) => { *pos += 1; Node::Operator { typ: Operator::Mul, lhs: Box::new(node), rhs: Box::new(power_root(tokens, pos, variables)?) } },
            TokenType::Symbol(Symbol::Div) => { *pos += 1; Node::Operator { typ: Operator::Div, lhs: Box::new(node), rhs: Box::new(power_root(tokens, pos, variables)?) } },
            TokenType::Symbol(Symbol::Rem) => { *pos += 1; Node::Operator { typ: Operator::Rem, lhs: Box::new(node), rhs: Box::new(power_root(tokens, pos, variables)?) } },
            _ => return Ok(node),
        };
    }
}

fn power_root(tokens: &Vec<Token>, pos: &mut usize, variables: &mut Vec<String>) -> Result<Node, String> {
    let node = unary(tokens, pos, variables)?;

    Ok(match tokens[*pos].typ {
        TokenType::Symbol(Symbol::Power) => { *pos += 1; Node::Operator { typ: Operator::Power, lhs: Box::new(node), rhs: Box::new(power_root(tokens, pos, variables)?) } },
        TokenType::Symbol(Symbol::Root) => { *pos += 1; Node::Operator { typ: Operator::Root, lhs: Box::new(node), rhs: Box::new(power_root(tokens, pos, variables)?) } },
        _ => return Ok(node),
    })
}

fn unary(tokens: &Vec<Token>, pos: &mut usize, variables: &mut Vec<String>) -> Result<Node, String> {
    Ok(match tokens[*pos].typ {
        TokenType::Symbol(Symbol::Sub) => { *pos += 1; Node::Operator { typ: Operator::Sub, lhs: Box::new(Node::Number { num: 0 }), rhs: Box::new(primary(tokens, pos, variables)?) } },
        TokenType::Symbol(Symbol::BitNot) => { *pos += 1; Node::Operator { typ: Operator::BitXor, lhs: Box::new(Node::Number { num: -1 }), rhs: Box::new(primary(tokens, pos, variables)?) } },
        TokenType::Symbol(Symbol::Not) => { *pos += 1; Node::Operator { typ: Operator::Equal, lhs: Box::new(Node::Number { num: 0 }), rhs: Box::new(primary(tokens, pos, variables)?) } },
        _ => primary(tokens, pos, variables)?,
    })
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
                return Ok(match tokens[*pos].typ {
                    TokenType::Symbol(Symbol::Increment) => { *pos += 1; Node::Operator { typ: Operator::Assign, lhs: Box::new(Node::Variable { offset: i }), rhs: Box::new(Node::Operator { typ: Operator::Add, lhs: Box::new(Node::Variable { offset: i }), rhs: Box::new(Node::Number { num: 1 }) }) } },
                    TokenType::Symbol(Symbol::Decrement) => { *pos += 1; Node::Operator { typ: Operator::Assign, lhs: Box::new(Node::Variable { offset: i }), rhs: Box::new(Node::Operator { typ: Operator::Sub, lhs: Box::new(Node::Variable { offset: i }), rhs: Box::new(Node::Number { num: 1 }) }) } },
                    _ => Node::Variable { offset: i },
                });
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