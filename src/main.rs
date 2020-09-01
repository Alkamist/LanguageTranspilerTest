mod rust_writing;
mod custom_language_parsing;

use std::fs;

use crate::rust_writing::*;
use crate::custom_language_parsing::*;

fn main() -> std::io::Result<()> {
    let contents = fs::read_to_string("test_file.txt").expect("Could not read the input file!");
    let instructions = create_instructions_from_custom_language_string(&contents);
    write_custom_instructions_to_rust_file(&instructions, 4, "test_output.rs")?;
    Ok(())
}
