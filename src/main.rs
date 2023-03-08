mod env_args;
mod file_reader;
mod tokenizer;
mod parser;
mod llvm_generator;

use std::env;

fn main() {
    let args = env::args().collect::<Vec<String>>();

    let error = env_args::check_args(&args);
    if let Some(e) = error {
        println!("Error occurred: {}", e);
        return;
    }

    let src_path = args[1].clone();
    let src = file_reader::read(src_path);

    let tokens = tokenizer::tokenize(&src);
    if let Err(message) = &tokens {
        println!("Error occurred: {}", message);
        return;
    }
    let tokens = tokens.unwrap();

    let program = parser::parse(tokens);
    if let Err(message) = &program {
        println!("Error occurred: {}", message);
        return;
    }
    let program = program.unwrap();

    let llvm = llvm_generator::generate(program);
    if let Err(message) = &llvm {
        println!("Error occurred: {}", message);
        return;
    }
    let llvm = llvm.unwrap();

    println!("{}", llvm);
}
