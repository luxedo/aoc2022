// Helper library for AOC 2022

use std::fs::{metadata, read_to_string};
use std::path::Path;

pub fn load_input(filename: &str) -> String {
    match metadata(Path::new(&filename)) {
        Ok(_) => read_to_string(filename)
            .expect("Failed reading file {filename}")
            .to_owned(),
        Err(_) => panic!("Input file {filename} not found"),
    }
}
