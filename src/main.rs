mod env_args;
mod file_reader;
mod tokenizer;

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
    }
    let tokens = tokens.unwrap();
}
