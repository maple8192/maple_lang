use std::collections::VecDeque;
use crate::parser::node::Node;
use crate::parser::node::operator::Operator;
use crate::parser::node::variable_type::VariableType;

pub fn generate(program: Node) -> Result<String, String> {
    if let Node::Program { functions } = program {
        let mut code = String::new();

        let mut function_info = Vec::new();
        function_info.push((format!("debug"), vec![VariableType::Int], VariableType::Void));
        for function in &functions {
            if let Node::Function { name, args_num, variables, ret_type, statement: _ } = function {
                let mut args = Vec::new();
                for i in 0..*args_num {
                    args.push(variables[i].clone());
                }
                function_info.push((name.clone(), args, ret_type.clone()))
            } else {
                return Err(format!("Not a function"));
            }
        }

        code.push_str("declare i32 @printf(i8*, ...)\n");
        code.push_str("@str = constant [4 x i8] c\"%d\\0A\\00\"\n");
        code.push_str("define void @debug(i64 %n) {\n");
        code.push_str("entry:\n");
        code.push_str("  %0 = getelementptr [4 x i8], [4 x i8]* @str, i32 0, i32 0\n");
        code.push_str("  call i32 (i8*, ...) @printf(i8* %0, i64 %n)\n");
        code.push_str("  ret void\n");
        code.push_str("}\n");

        for function in &functions {
            let mut stack = VecDeque::new();
            let mut i = 0;
            let mut l = 0;
            code.push_str(&gen(function, &mut stack, &function_info, &VariableType::Void, &mut i, &mut l)?);
            if !stack.is_empty() {
                return Err(format!("Stack not empty"));
            }
        }

        Ok(code)
    } else {
        Err(format!("Not a program"))
    }
}

fn gen(node: &Node, stack: &mut VecDeque<usize>, functions: &Vec<(String, Vec<VariableType>, VariableType)>, ret_type: &VariableType, last_index: &mut usize, last_label: &mut usize) -> Result<String, String> {
    let mut code = String::new();

    match node {
        Node::Program { functions: _ } => {
            return Err(format!("Error"));
        },
        Node::Function { name, args_num, variables, ret_type, statement } => {
            code.push_str("define ");
            code.push_str(&format!("{} ", ret_type.str()));
            code.push_str(&format!("@{}(", name));

            for i in 0..*args_num {
                code.push_str(&format!("{}{} %arg{}", if i == 0 { "" } else { ", " }, variables[i].str(), i));
            }

            code.push_str(") {\n");
            code.push_str("entry:\n");

            for i in 0..*args_num {
                code.push_str(&format!("  %{} = alloca {}\n", i, variables[i].str()));
                code.push_str(&format!("  store {} %arg{}, {}* %{}\n", variables[i].str(), i, variables[i].str(), i));
            }

            for i in *args_num..variables.len() {
                code.push_str(&format!("  %{} = alloca {}\n", i, variables[i].str()));
                code.push_str(&format!("  store {} 0, {}* %{}\n", variables[i].str(), variables[i].str(), i));
            }

            *last_index += variables.len();

            code.push_str(&gen(statement, stack, functions, ret_type, last_index, last_label)?);

            code.push_str(&format!("  ret {}{}\n", ret_type.str(), if let VariableType::Void = ret_type { "" } else { " 0" }));
            code.push_str("}\n");
        },
        Node::Statement { node } => {
            code.push_str(&gen(node.as_ref(), stack, functions, ret_type, last_index, last_label)?);
            stack.pop_back().unwrap();
        },
        Node::Block { statements } => {
            for node in statements {
                code.push_str(&gen(node, stack, functions, ret_type, last_index, last_label)?);
            }
        }
        Node::Return { node } => {
            code.push_str(&gen(node.as_ref(), stack, functions, ret_type, last_index, last_label)?);
            code.push_str(&format!("  ret i64 %{}\n", stack.pop_back().unwrap()));
            *last_index += 1;
        },
        Node::If { condition, true_case, false_case } => {
            let label = *last_label;
            *last_label += 1;
            code.push_str(&gen(condition.as_ref(), stack, functions, ret_type, last_index, last_label)?);
            code.push_str(&format!("  %{} = icmp ne i64 %{}, 0\n", last_index, stack.pop_back().unwrap()));
            code.push_str(&format!("  br i1 %{}, label %then{}, label %else{}\n", last_index, label, label));
            *last_index += 1;
            code.push_str(&format!("then{}:\n", label));
            code.push_str(&gen(true_case.as_ref(), stack, functions, ret_type, last_index, last_label)?);
            code.push_str(&format!("  br label %end{}\n", label));
            code.push_str(&format!("else{}:\n", label));
            if let Some(false_case) = false_case.as_ref() {
                code.push_str(&gen(false_case, stack, functions, ret_type, last_index, last_label)?);
            }
            code.push_str(&format!("  br label %end{}\n", label));
            code.push_str(&format!("end{}:\n", label));
        },
        Node::For { init, condition, update, statement } => {
            let label = *last_label;
            *last_label += 1;
            if let Some(init) = init.as_ref() {
                code.push_str(&gen(init, stack, functions, ret_type, last_index, last_label)?);
                stack.pop_back().unwrap();
            }
            code.push_str(&format!("  br label %begin{}\n", label));
            code.push_str(&format!("begin{}:\n", label));
            if let Some(condition) = condition.as_ref() {
                code.push_str(&gen(condition, stack, functions, ret_type, last_index, last_label)?);
                code.push_str(&format!("  %{} = icmp ne i64 %{}, 0\n", last_index, stack.pop_back().unwrap()));
                code.push_str(&format!("  br i1 %{}, label %then{}, label %end{}\n", *last_index, label, label));
                *last_index += 1;
            } else {
                code.push_str(&format!("  br label %then{}\n", label));
            }
            code.push_str(&format!("then{}:\n", label));
            code.push_str(&gen(statement.as_ref(), stack, functions, ret_type, last_index, last_label)?);
            if let Some(update) = update.as_ref() {
                code.push_str(&gen(update, stack, functions, ret_type, last_index, last_label)?);
                stack.pop_back().unwrap();
            }
            code.push_str(&format!("  br label %begin{}\n", label));
            code.push_str(&format!("end{}:\n", label));
        },
        Node::While { condition, node } => {
            let label = *last_label;
            *last_label += 1;
            code.push_str(&format!("  br label %begin{}\n", label));
            code.push_str(&format!("begin{}:\n", label));
            code.push_str(&gen(condition, stack, functions, ret_type, last_index, last_label)?);
            code.push_str(&format!("  %{} = icmp ne i64 %{}, 0\n", last_index, stack.pop_back().unwrap()));
            code.push_str(&format!("  br i1 %{}, label %then{}, label %end{}\n", last_index, label, label));
            *last_index += 1;
            code.push_str(&format!("then{}:\n", label));
            code.push_str(&gen(node, stack, functions, ret_type, last_index, last_label)?);
            code.push_str(&format!("  br label %begin{}\n", label));
            code.push_str(&format!("end{}:", label));
        },
        Node::Operator { typ, lhs, rhs } => {
            match typ {
                Operator::Add => {
                    code.push_str(&gen(rhs.as_ref(), stack, functions, ret_type, last_index, last_label)?);
                    code.push_str(&gen(lhs.as_ref(), stack, functions, ret_type, last_index, last_label)?);
                    code.push_str(&format!("  %{} = add i64 %{}, %{}\n", last_index, stack.pop_back().unwrap(), stack.pop_back().unwrap()));
                    stack.push_back(*last_index);
                    *last_index += 1;
                },
                Operator::Sub => {
                    code.push_str(&gen(rhs.as_ref(), stack, functions, ret_type, last_index, last_label)?);
                    code.push_str(&gen(lhs.as_ref(), stack, functions, ret_type, last_index, last_label)?);
                    code.push_str(&format!("  %{} = sub i64 %{}, %{}\n", last_index, stack.pop_back().unwrap(), stack.pop_back().unwrap()));
                    stack.push_back(*last_index);
                    *last_index += 1;
                },
                Operator::Mul => {
                    code.push_str(&gen(rhs.as_ref(), stack, functions, ret_type, last_index, last_label)?);
                    code.push_str(&gen(lhs.as_ref(), stack, functions, ret_type, last_index, last_label)?);
                    code.push_str(&format!("  %{} = mul i64 %{}, %{}\n", last_index, stack.pop_back().unwrap(), stack.pop_back().unwrap()));
                    stack.push_back(*last_index);
                    *last_index += 1;
                },
                Operator::Div => {
                    code.push_str(&gen(rhs.as_ref(), stack, functions, ret_type, last_index, last_label)?);
                    code.push_str(&gen(lhs.as_ref(), stack, functions, ret_type, last_index, last_label)?);
                    code.push_str(&format!("  %{} = sdiv i64 %{}, %{}\n", last_index, stack.pop_back().unwrap(), stack.pop_back().unwrap()));
                    stack.push_back(*last_index);
                    *last_index += 1;
                },
                Operator::Rem => {
                    code.push_str(&gen(rhs.as_ref(), stack, functions, ret_type, last_index, last_label)?);
                    code.push_str(&gen(lhs.as_ref(), stack, functions, ret_type, last_index, last_label)?);
                    code.push_str(&format!("  %{} = srem i64 %{}, %{}\n", last_index, stack.pop_back().unwrap(), stack.pop_back().unwrap()));
                    stack.push_back(*last_index);
                    *last_index += 1;
                },
                Operator::Power => (),
                Operator::Root => (),
                Operator::And => {
                    code.push_str(&gen(rhs.as_ref(), stack, functions, ret_type, last_index, last_label)?);
                    code.push_str(&gen(lhs.as_ref(), stack, functions, ret_type, last_index, last_label)?);
                    code.push_str(&format!("  %{} = and i64 %{}, %{}\n", last_index, stack.pop_back().unwrap(), stack.pop_back().unwrap()));
                    stack.push_back(*last_index);
                    *last_index += 1;
                }
                Operator::Xor => {
                    code.push_str(&gen(rhs.as_ref(), stack, functions, ret_type, last_index, last_label)?);
                    code.push_str(&gen(lhs.as_ref(), stack, functions, ret_type, last_index, last_label)?);
                    code.push_str(&format!("  %{} = xor i64 %{}, %{}\n", last_index, stack.pop_back().unwrap(), stack.pop_back().unwrap()));
                    stack.push_back(*last_index);
                    *last_index += 1;
                }
                Operator::Or => {
                    code.push_str(&gen(rhs.as_ref(), stack, functions, ret_type, last_index, last_label)?);
                    code.push_str(&gen(lhs.as_ref(), stack, functions, ret_type, last_index, last_label)?);
                    code.push_str(&format!("  %{} = or i64 %{}, %{}\n", last_index, stack.pop_back().unwrap(), stack.pop_back().unwrap()));
                    stack.push_back(*last_index);
                    *last_index += 1;
                }
                Operator::LShift => {
                    code.push_str(&gen(rhs.as_ref(), stack, functions, ret_type, last_index, last_label)?);
                    code.push_str(&gen(lhs.as_ref(), stack, functions, ret_type, last_index, last_label)?);
                    code.push_str(&format!("  %{} = shl i64 %{}, %{}\n", last_index, stack.pop_back().unwrap(), stack.pop_back().unwrap()));
                    stack.push_back(*last_index);
                    *last_index += 1;
                },
                Operator::RShift => {
                    code.push_str(&gen(rhs.as_ref(), stack, functions, ret_type, last_index, last_label)?);
                    code.push_str(&gen(lhs.as_ref(), stack, functions, ret_type, last_index, last_label)?);
                    code.push_str(&format!("  %{} = ashr i64 %{}, %{}\n", last_index, stack.pop_back().unwrap(), stack.pop_back().unwrap()));
                    stack.push_back(*last_index);
                    *last_index += 1;
                },
                Operator::Equal => {
                    code.push_str(&gen(rhs.as_ref(), stack, functions, ret_type, last_index, last_label)?);
                    code.push_str(&gen(lhs.as_ref(), stack, functions, ret_type, last_index, last_label)?);
                    code.push_str(&format!("  %{} = icmp eq i64 %{}, %{}\n", last_index, stack.pop_back().unwrap(), stack.pop_back().unwrap()));
                    *last_index += 1;
                    code.push_str(&format!("  %{} = zext i1 %{} to i64\n", last_index, *last_index - 1));
                    stack.push_back(*last_index);
                    *last_index += 1;
                },
                Operator::Less => {
                    code.push_str(&gen(rhs.as_ref(), stack, functions, ret_type, last_index, last_label)?);
                    code.push_str(&gen(lhs.as_ref(), stack, functions, ret_type, last_index, last_label)?);
                    code.push_str(&format!("  %{} = icmp slt i64 %{}, %{}\n", last_index, stack.pop_back().unwrap(), stack.pop_back().unwrap()));
                    *last_index += 1;
                    code.push_str(&format!("  %{} = zext i1 %{} to i64\n", last_index, *last_index - 1));
                    stack.push_back(*last_index);
                    *last_index += 1;
                },
                Operator::Assign => {
                    if let Node::Variable { offset } = lhs.as_ref() {
                        code.push_str(&gen(rhs.as_ref(), stack, functions, ret_type, last_index, last_label)?);
                        code.push_str(&format!("  store i64 %{}, i64* %{}\n", stack.pop_back().unwrap(), offset));
                        code.push_str(&format!("  %{} = load i64, i64* %{}\n", last_index, offset));
                        stack.push_back(*last_index);
                        *last_index += 1;
                    } else {
                        return Err(format!("Not a variable"));
                    }
                },
                Operator::ChangeMin => {
                    if let Node::Variable { offset } = lhs.as_ref() {
                        let label = *last_label;
                        *last_label += 1;
                        code.push_str(&gen(rhs.as_ref(), stack, functions, ret_type, last_index, last_label)?);
                        code.push_str(&format!("  %{} = load i64, i64* %{}\n", last_index, offset));
                        *last_index += 1;
                        let ch_ptr = stack.pop_back().unwrap();
                        code.push_str(&format!("  %{} = icmp sgt i64 %{}, %{}\n", last_index, *last_index - 1, ch_ptr));
                        code.push_str(&format!("  br i1 %{}, label %then{}, label %end{}\n", last_index, label, label));
                        *last_index += 1;
                        code.push_str(&format!("then{}:\n", label));
                        code.push_str(&format!("  store i64 %{}, i64* %{}\n", ch_ptr, offset));
                        code.push_str(&format!("  br label %end{}\n", label));
                        code.push_str(&format!("end{}:\n", label));
                        code.push_str(&format!("  %{} = load i64, i64* %{}\n", last_index, offset));
                        stack.push_back(*last_index);
                        *last_index += 1;
                    } else {
                        return Err(format!("Not a variable"));
                    }
                },
                Operator::ChangeMax => {
                    if let Node::Variable { offset } = lhs.as_ref() {
                        let label = *last_label;
                        *last_label += 1;
                        code.push_str(&gen(rhs.as_ref(), stack, functions, ret_type, last_index, last_label)?);
                        code.push_str(&format!("  %{} = load i64, i64* %{}\n", last_index, offset));
                        *last_index += 1;
                        let ch_ptr = stack.pop_back().unwrap();
                        code.push_str(&format!("  %{} = icmp slt i64 %{}, %{}\n", last_index, *last_index - 1, ch_ptr));
                        code.push_str(&format!("  br i1 %{}, label %then{}, label %end{}\n", last_index, label, label));
                        *last_index += 1;
                        code.push_str(&format!("then{}:\n", label));
                        code.push_str(&format!("  store i64 %{}, i64* %{}\n", ch_ptr, offset));
                        code.push_str(&format!("  br label %end{}\n", label));
                        code.push_str(&format!("end{}:\n", label));
                        code.push_str(&format!("  %{} = load i64, i64* %{}\n", last_index, offset));
                        stack.push_back(*last_index);
                        *last_index += 1;
                    } else {
                        return Err(format!("Not a variable"));
                    }
                },
                Operator::Exchange => {
                    if let Node::Variable { offset: ol } = lhs.as_ref() {
                        if let Node::Variable { offset: or } = rhs.as_ref() {
                            code.push_str(&format!("  %{} = load i64, i64* %{}\n", last_index, ol));
                            *last_index += 1;
                            code.push_str(&format!("  %{} = load i64, i64* %{}\n", last_index, or));
                            code.push_str(&format!("  store i64 %{}, i64* %{}\n", *last_index - 1, or));
                            code.push_str(&format!("  store i64 %{}, i64* %{}\n", last_index, ol));
                            *last_index += 1;
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
        Node::FuncCall { function_name, arguments } => {
            let mut found = false;
            for (name, args, ret_type) in functions {
                if name == function_name && args.len() == arguments.len() {
                    found = true;

                    let mut args = Vec::new();
                    for arg in arguments {
                        code.push_str(&gen(arg, stack, functions, ret_type, last_index, last_label)?);
                        args.push(stack.pop_back().unwrap());
                    }
                    if let VariableType::Void = ret_type {
                        code.push_str(&format!("  call void @{}(", function_name));
                        let mut first = true;
                        for i in args {
                            if !first {
                                code.push_str(", ");
                            }
                            first = false;
                            code.push_str(&format!("i64 %{}", i));
                        }
                        code.push_str(")\n");
                        code.push_str(&format!("  %{} = alloca i64\n", last_index));
                        code.push_str(&format!("  store i64 0, i64* %{}", last_index));
                        stack.push_back(*last_index);
                        *last_index += 1;
                    } else {
                        code.push_str(&format!("  %{} = call {} @{}(", last_index, ret_type.str(), function_name));
                        let mut first = true;
                        for i in args {
                            if !first {
                                code.push_str(", ");
                            }
                            first = false;
                            code.push_str(&format!("i64 %{}", i));
                        }
                        code.push_str(")\n");
                        stack.push_back(*last_index);
                        *last_index += 1;
                    }

                    break;
                }
            }
            if !found {
                return Err(format!("Function not found"));
            }
        },
        Node::Number { num } => {
            code.push_str(&format!("  %{} = add i64 {}, 0\n", last_index, num));
            stack.push_back(*last_index);
            *last_index += 1;
        },
    }

    Ok(code)
}