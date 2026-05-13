use std::fs::OpenOptions;
use std::io::Write;

pub fn write_file(path: &str, text: &str) {
    let mut f = OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)
        .unwrap();

    writeln!(f, "{}", text).unwrap();
}