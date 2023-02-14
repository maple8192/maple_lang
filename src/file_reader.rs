use std::fs;
use std::path::Path;

pub fn read<P: AsRef<Path>>(path: P) -> String {
    fs::read_to_string(path).unwrap()
}