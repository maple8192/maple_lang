use std::path::Path;

pub fn check_args(args: &Vec<String>) -> Option<String> {
    if !check_len(args) { return Some("One argument required.".to_string()); }
    if !check_source_path(&args[1]) { return Some("Wrong with source path.".to_string()); }

    None
}

fn check_len(args: &Vec<String>) -> bool {
    args.len() >= 3
}

fn check_source_path(str: &String) -> bool {
    Path::new(str).exists()
}