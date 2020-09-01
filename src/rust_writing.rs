use std::fs::File;
use std::io::prelude::*;

use crate::custom_language_parsing::CustomInstruction;

fn add_indentation(input: &mut String, indentation: u32) {
    for _ in 0..indentation {
        input.push(' ');
    }
}

enum RustInstruction {
    EmptyLine,
    VariableDefinition {
        is_mutable: bool,
        name: String,
        value: String,
    },
    VariableAssignment {
        name: String,
        value: String,
    },
    ReturnStatement {
        is_final: bool,
        value: String,
    },
    FunctionDefinition {
        is_public: bool,
        name: String,
        argument_names: Vec<String>,
        argument_types: Vec<String>,
        return_type: String,
        instructions: Vec<RustInstruction>,
    },
    Loop {
        count: String,
        counter_name: String,
        instructions: Vec<RustInstruction>,
    },
    IfStatement {
        condition: String,
        instructions: Vec<RustInstruction>,
    },
    ElseStatement {
        instructions: Vec<RustInstruction>,
    },
    ElseIfStatement {
        condition: String,
        instructions: Vec<RustInstruction>,
    },
}

impl RustInstruction {
    fn get_string(&self, indentation_level: u32, indentation_spaces: u32) -> String {

        match self {

            Self::EmptyLine => String::from("\n"),

            Self::VariableDefinition {
                is_mutable,
                name,
                value } => {

                let indentation = indentation_level * indentation_spaces;
                let mut output = String::new();
                add_indentation(&mut output, indentation);
                output.push_str("let ");
                if *is_mutable {
                    output.push_str("mut ");
                }
                output.push_str(name);
                output.push_str(" = ");
                output.push_str(value);
                output.push_str(";\n");
                output
            },

            Self::VariableAssignment {
                name,
                value } => {

                let indentation = indentation_level * indentation_spaces;
                let mut output = String::new();
                add_indentation(&mut output, indentation);
                output.push_str(name);
                output.push_str(" = ");
                output.push_str(value);
                output.push_str(";\n");
                output
            },

            Self::ReturnStatement {
                is_final,
                value } => {

                let indentation = indentation_level * indentation_spaces;
                let mut output = String::new();
                add_indentation(&mut output, indentation);
                if !*is_final {
                    output.push_str("return ");
                }
                output.push_str(value);
                if !*is_final {
                    output.push_str(";");
                }
                output.push_str("\n");
                output
            },

            Self::FunctionDefinition {
                is_public,
                name,
                argument_names,
                argument_types,
                return_type,
                instructions } => {

                let indentation = indentation_level * indentation_spaces;

                let mut output = String::new();
                add_indentation(&mut output, indentation);

                if *is_public {
                    output.push_str("pub ");
                }

                output.push_str("fn ");
                output.push_str(name);
                output.push_str("(");

                let num_arguments = argument_names.len();
                for i in 0..num_arguments {
                    let argument_name = &argument_names[i];
                    let argument_type = &argument_types[i];

                    output.push_str(argument_name);
                    output.push_str(": ");
                    output.push_str(argument_type);

                    let is_last_argument = i >= num_arguments - 1;
                    if !is_last_argument {
                        output.push_str(", ");
                    }
                }

                output.push_str(")");

                if return_type != "" {
                    output.push_str(" -> ");
                    output.push_str(return_type);
                }

                output.push_str(" {\n");

                for instruction in instructions.iter() {
                    output.push_str(&instruction.get_string(indentation_level + 1, indentation_spaces));
                }

                add_indentation(&mut output, indentation);
                output.push_str("}\n");

                output
            },

            Self::Loop {
                count,
                counter_name,
                instructions } => {

                let indentation = indentation_level * indentation_spaces;

                let mut output = String::new();
                add_indentation(&mut output, indentation);

                output.push_str("for ");
                output.push_str(counter_name);
                output.push_str(" in 0..");
                output.push_str(count);
                output.push_str(" {\n");

                for instruction in instructions.iter() {
                    output.push_str(&instruction.get_string(indentation_level + 1, indentation_spaces));
                }

                add_indentation(&mut output, indentation);
                output.push_str("}\n");

                output
            },

            Self::IfStatement {
                condition,
                instructions } => {

                let indentation = indentation_level * indentation_spaces;

                let mut output = String::new();
                add_indentation(&mut output, indentation);

                output.push_str("if ");
                output.push_str(condition);
                output.push_str(" {\n");

                for instruction in instructions.iter() {
                    output.push_str(&instruction.get_string(indentation_level + 1, indentation_spaces));
                }

                add_indentation(&mut output, indentation);
                output.push_str("}\n");

                output
            },

            Self::ElseStatement {
                instructions } => {

                let indentation = indentation_level * indentation_spaces;

                let mut output = String::new();
                add_indentation(&mut output, indentation);

                output.push_str("else {\n");

                for instruction in instructions.iter() {
                    output.push_str(&instruction.get_string(indentation_level + 1, indentation_spaces));
                }

                add_indentation(&mut output, indentation);
                output.push_str("}\n");

                output
            },

            Self::ElseIfStatement {
                condition,
                instructions } => {

                let indentation = indentation_level * indentation_spaces;

                let mut output = String::new();
                add_indentation(&mut output, indentation);

                output.push_str("else if ");
                output.push_str(condition);
                output.push_str(" {\n");

                for instruction in instructions.iter() {
                    output.push_str(&instruction.get_string(indentation_level + 1, indentation_spaces));
                }

                add_indentation(&mut output, indentation);
                output.push_str("}\n");

                output
            },
        }
    }
}

fn custom_instruction_to_rust_instruction(custom_instruction: &CustomInstruction) -> RustInstruction {
    match custom_instruction {
        CustomInstruction::EmptyLine => RustInstruction::EmptyLine,

        CustomInstruction::VariableDefinition {
            is_mutable,
            name,
            value } => {

            RustInstruction::VariableDefinition {
                is_mutable: *is_mutable,
                name: name.clone(),
                value: value.clone(),
            }
        },

        CustomInstruction::VariableAssignment {
            name,
            value } => {

            RustInstruction::VariableAssignment {
                name: name.clone(),
                value: value.clone(),
            }
        },
    }
}

fn custom_instructions_to_rust_instructions(
    custom_instructions: &Vec<CustomInstruction>) -> Vec<RustInstruction> {

    let mut output_instructions: Vec<RustInstruction> = Vec::new();

    for custom_instruction in custom_instructions.iter() {
        output_instructions.push(custom_instruction_to_rust_instruction(custom_instruction));
    }

    output_instructions
}

fn write_rust_instructions_to_file(
    instructions: &Vec<RustInstruction>,
    indentation: u32,
    file_path: &str) -> std::io::Result<()> {

    let mut file = File::create(file_path)?;

    let mut output = String::new();

    for instruction in instructions.iter() {
        output.push_str(&instruction.get_string(0, indentation));
    }

    file.write_all(output.as_bytes())?;
    Ok(())
}

pub fn write_custom_instructions_to_rust_file(
    instructions: &Vec<CustomInstruction>,
    indentation: u32,
    file_path: &str) -> std::io::Result<()> {

    let rust_instructions = custom_instructions_to_rust_instructions(instructions);
    write_rust_instructions_to_file(&rust_instructions, indentation, file_path)?;
    Ok(())
}
