use std::fs::File;
use std::io::Write;
use std::path::Path;

pub fn write<P: AsRef<Path>>(path: P, context: String) {
    let mut file = File::create(&path).unwrap();
    file.write_all(context.as_bytes()).unwrap();
}