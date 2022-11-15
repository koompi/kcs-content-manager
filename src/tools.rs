use std::fs::{File, OpenOptions};

pub fn continue_file(source_file: &str) -> File {
    OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .append(true)
        .open(source_file)
        .unwrap()
}