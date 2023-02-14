mod env_args;

use std::env;

fn main() {
    let error = env_args::check_args(&env::args().collect::<Vec<String>>());
    match error {
        Some(e) => println!("Error occurred: {}", e),
        None => println!("Successfully!!")
    }
}
