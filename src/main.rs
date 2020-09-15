mod instruction;
mod variable;
mod function;
mod parsers;

use std::fs;

use crate::parsers::test_parse;

fn indented(input: &str, spaces: usize) -> String {
    let mut output = input.to_string();
    for _ in 0..spaces {
        output.push(' ');
    }
    output
}

fn main() {
    let file_text = fs::read_to_string("test_file.txt").unwrap();

    match test_parse(&file_text) {
        Ok(res) => {
            for instruction in res.1 {
                instruction.to_rust();
                println!("{}", instruction.to_rust());
            }
        },
        Err(e) => println!("{}", e),
    }
}
