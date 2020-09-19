//mod instruction;
//mod expression;
mod operator;
//mod literal;
//mod variable;
//mod function;
//mod parsers;

use std::fs;

//use nom::{
//    IResult,
//    branch::alt,
//};

//use crate::instruction::*;
//use crate::literal::*;
use crate::operator::*;

fn main() {
    let file_text = fs::read_to_string("test_file.txt").unwrap();

    let res = Expr::parse(&file_text).unwrap();
    println!("{}", res.1.to_rust());

    //match addition(&file_text) {
    //    //Ok(res) => println!("{}", res.1.to_rust()),
    //    Ok(res) => (),
    //    Err(e) => println!("{}", e),
    //}
}