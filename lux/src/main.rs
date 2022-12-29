use std::{env, fs};

// TODO: Remove this file and build a proper test runner against real .lux files!
use lux::parser::parse;

fn main() {
    let filename = env::args().nth(1).expect("provide a filename");
    let input = fs::read_to_string(&filename)
        .unwrap_or_else(|_| panic!("failed to load file: {}", filename));

    let result = parse(&input);

    eprintln!("{:#?}:", result);
}
