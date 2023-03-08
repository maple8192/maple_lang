use std::collections::VecDeque;
use crate::parser::node::Node;
use crate::parser::node::operator::Operator;

pub fn generate(program: Node) -> Result<String, String> {
    if let Node::Program { functions } = program {
        let mut code = String::new();

        for function in functions {
            let mut stack = VecDeque::new();
            let mut i = 0;
            code.push_str(&gen(&function, &mut stack, &mut i)?);
        }

        Ok(code)
    } else {
        Err(format!("Not a program"))
    }
}

fn gen(node: &Node, stack: &mut VecDeque<usize>, last_index: &mut usize) -> Result<String, String> {
    let mut code = String::new();

    match node {
        Node::Function { name, args_num, variables, statements } => {
            code.push_str(&format!("define i64 @{}(", name));

            let mut first = true;
            for _ in 0..*args_num {
                code.push_str(&format!("{}i64", if first { "" } else { ", " }));
                first = false;
            }

            code.push_str(") {\n");
            code.push_str("entry:\n");

            for i in *args_num..variables.len() {
                code.push_str(&format!("  %{} = alloca i64\n", i));
                code.push_str(&format!("  store i64 0, i64* %{}\n", i));
            }

            *last_index += variables.len();

            for s in statements {
                code.push_str(&gen(s, stack, last_index)?);
            }

            code.push_str(&format!("  ret i64 %{}\n", stack.pop_back().unwrap()));
            code.push_str("}\n");
        },
        Node::Statement { nodes } => {
            for i in 0..nodes.len() {
                code.push_str(&gen(&nodes[i], stack, last_index)?);
                if i != nodes.len() - 1 { stack.pop_back(); }
            }
        },
        Node::Operator { typ, lhs, rhs } => {
            match typ {
                Operator::Add => {
                    code.push_str(&gen(rhs.as_ref(), stack, last_index)?);
                    code.push_str(&gen(lhs.as_ref(), stack, last_index)?);
                    code.push_str(&format!("  %{} = add i64 %{}, %{}\n", last_index, stack.pop_back().unwrap(), stack.pop_back().unwrap()));
                    stack.push_back(*last_index);
                    *last_index += 1;
                },
                Operator::Sub => {
                    code.push_str(&gen(rhs.as_ref(), stack, last_index)?);
                    code.push_str(&gen(lhs.as_ref(), stack, last_index)?);
                    code.push_str(&format!("  %{} = sub i64 %{}, %{}\n", last_index, stack.pop_back().unwrap(), stack.pop_back().unwrap()));
                    stack.push_back(*last_index);
                    *last_index += 1;
                },
                Operator::Mul => {
                    code.push_str(&gen(rhs.as_ref(), stack, last_index)?);
                    code.push_str(&gen(lhs.as_ref(), stack, last_index)?);
                    code.push_str(&format!("  %{} = mul i64 %{}, %{}\n", last_index, stack.pop_back().unwrap(), stack.pop_back().unwrap()));
                    stack.push_back(*last_index);
                    *last_index += 1;
                },
                Operator::Div => {
                    code.push_str(&gen(rhs.as_ref(), stack, last_index)?);
                    code.push_str(&gen(lhs.as_ref(), stack, last_index)?);
                    code.push_str(&format!("  %{} = sdiv i64 %{}, %{}\n", last_index, stack.pop_back().unwrap(), stack.pop_back().unwrap()));
                    stack.push_back(*last_index);
                    *last_index += 1;
                },
                Operator::Rem => {
                    code.push_str(&gen(rhs.as_ref(), stack, last_index)?);
                    code.push_str(&gen(lhs.as_ref(), stack, last_index)?);
                    code.push_str(&format!("  %{} = srem i64 %{}, %{}\n", last_index, stack.pop_back().unwrap(), stack.pop_back().unwrap()));
                    stack.push_back(*last_index);
                    *last_index += 1;
                },
                Operator::Power => (),
                Operator::Root => (),
                Operator::And => {
                    code.push_str(&gen(rhs.as_ref(), stack, last_index)?);
                    code.push_str(&gen(lhs.as_ref(), stack, last_index)?);
                    code.push_str(&format!("  %{} = and i64 %{}, %{}\n", last_index, stack.pop_back().unwrap(), stack.pop_back().unwrap()));
                    stack.push_back(*last_index);
                    *last_index += 1;
                }
                Operator::Xor => {
                    code.push_str(&gen(rhs.as_ref(), stack, last_index)?);
                    code.push_str(&gen(lhs.as_ref(), stack, last_index)?);
                    code.push_str(&format!("  %{} = xor i64 %{}, %{}\n", last_index, stack.pop_back().unwrap(), stack.pop_back().unwrap()));
                    stack.push_back(*last_index);
                    *last_index += 1;
                }
                Operator::Or => {
                    code.push_str(&gen(rhs.as_ref(), stack, last_index)?);
                    code.push_str(&gen(lhs.as_ref(), stack, last_index)?);
                    code.push_str(&format!("  %{} = or i64 %{}, %{}\n", last_index, stack.pop_back().unwrap(), stack.pop_back().unwrap()));
                    stack.push_back(*last_index);
                    *last_index += 1;
                }
                Operator::LShift => {
                    code.push_str(&gen(rhs.as_ref(), stack, last_index)?);
                    code.push_str(&gen(lhs.as_ref(), stack, last_index)?);
                    code.push_str(&format!("  %{} = shl i64 %{}, %{}\n", last_index, stack.pop_back().unwrap(), stack.pop_back().unwrap()));
                    stack.push_back(*last_index);
                    *last_index += 1;
                },
                Operator::RShift => {
                    code.push_str(&gen(rhs.as_ref(), stack, last_index)?);
                    code.push_str(&gen(lhs.as_ref(), stack, last_index)?);
                    code.push_str(&format!("  %{} = ashr i64 %{}, %{}\n", last_index, stack.pop_back().unwrap(), stack.pop_back().unwrap()));
                    stack.push_back(*last_index);
                    *last_index += 1;
                },
                Operator::Equal => {
                    code.push_str(&gen(rhs.as_ref(), stack, last_index)?);
                    code.push_str(&gen(lhs.as_ref(), stack, last_index)?);
                    code.push_str(&format!("  %{} = icmp eq i64 %{}, %{}\n", last_index, stack.pop_back().unwrap(), stack.pop_back().unwrap()));
                    *last_index += 1;
                    code.push_str(&format!("  %{} = zext i1 %{} to i64\n", last_index, *last_index - 1));
                    stack.push_back(*last_index);
                    *last_index += 1;
                },
                Operator::Less => {
                    code.push_str(&gen(rhs.as_ref(), stack, last_index)?);
                    code.push_str(&gen(lhs.as_ref(), stack, last_index)?);
                    code.push_str(&format!("  %{} = icmp slt i64 %{}, %{}\n", last_index, stack.pop_back().unwrap(), stack.pop_back().unwrap()));
                    *last_index += 1;
                    code.push_str(&format!("  %{} = zext i1 %{} to i64\n", last_index, *last_index - 1));
                    stack.push_back(*last_index);
                    *last_index += 1;
                },
                Operator::Greater => {
                    code.push_str(&gen(rhs.as_ref(), stack, last_index)?);
                    code.push_str(&gen(lhs.as_ref(), stack, last_index)?);
                    code.push_str(&format!("  %{} = icmp sgt i64 %{}, %{}\n", last_index, stack.pop_back().unwrap(), stack.pop_back().unwrap()));
                    *last_index += 1;
                    code.push_str(&format!("  %{} = zext i1 %{} to i64\n", last_index, *last_index - 1));
                    stack.push_back(*last_index);
                    *last_index += 1;
                },
                Operator::Assign => {
                    if let Node::Variable { offset } = lhs.as_ref() {
                        code.push_str(&gen(rhs.as_ref(), stack, last_index)?);
                        code.push_str(&format!("  store i64 %{}, i64* %{}\n", stack.pop_back().unwrap(), offset));
                        code.push_str(&format!("  %{} = load i64, i64* %{}\n", last_index, offset));
                        stack.push_back(*last_index);
                        *last_index += 1;
                    } else {
                        return Err(format!("Not a variable"));
                    }
                },
                Operator::ChangeMin => (),
                Operator::ChangeMax => (),
                Operator::Exchange => {
                    if let Node::Variable { offset: ol } = lhs.as_ref() {
                        if let Node::Variable { offset: or } = rhs.as_ref() {
                            code.push_str(&format!("  %{} = load i64, i64* %{}\n", last_index, ol));
                            *last_index += 1;
                            code.push_str(&format!("  %{} = load i64, i64* %{}\n", last_index, or));
                            *last_index += 1;
                            code.push_str(&format!("  store i64 %{}, i64* %{}\n", *last_index - 2, or));
                            code.push_str(&format!("  store i64 %{}, i64* %{}\n", *last_index - 1, ol));
                            code.push_str(&format!("  %{} = load i64, i64* %{}\n", *last_index, ol));
                            stack.push_back(*last_index);
                            *last_index += 1;
                        } else {
                            return Err(format!("Not a variable"));
                        }
                    } else {
                        return Err(format!("Not a variable"));
                    }
                },
            }
        },
        Node::Variable { offset } => {
            code.push_str(&format!("  %{} = load i64, i64* %{}\n", last_index, offset));
            stack.push_back(*last_index);
            *last_index += 1;
        },
        Node::Number { num } => {
            code.push_str(&format!("  %{} = add i64 {}, 0\n", last_index, num));
            stack.push_back(*last_index);
            *last_index += 1;
        },
        _ => return Err(format!("Error")),
    }

    Ok(code)
}