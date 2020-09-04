//mod rust_writing;
mod common;
mod float_literal;
mod variable_definition;
mod function_definition;
mod custom_language_parsing;

//use std::fs;

//use crate::rust_writing::*;
use crate::custom_language_parsing::parse_program;


fn main() {
    parse_program("test_file.txt");
}

//fn main() -> std::io::Result<()> {
//    let contents = fs::read_to_string("test_file.txt").expect("Could not read the input file!");
//    //let instructions = create_instructions_from_custom_language_string(&contents);
//    //write_custom_instructions_to_rust_file(&instructions, 4, "test_output.rs")?;
//    Ok(())
//}
