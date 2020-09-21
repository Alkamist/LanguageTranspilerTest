mod instruction;
mod expression;
mod grouping_expression;
mod literal_expression;
mod binary_expression;

use std::fs;

use crate::instruction::*;
use crate::expression::*;

fn main() {
    let file_text = fs::read_to_string("test_file.txt").unwrap();

    match Expression::parse(&file_text) {
        Ok(res) => println!("{}", res.1.to_rust()),
        Err(e) => println!("{}", e),
    }
}
