use std::fs::File;
use std::io::prelude::*;

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
}

impl RustInstruction {
    fn get_string(&self, indentation_level: u32, indentation_spaces: u32) -> String {

        match self {

            Self::EmptyLine => String::from("\n"),

            Self::VariableDefinition {
                is_mutable,
                name,
                value
                } => {

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
                value
                } => {

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
                value
                } => {

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
                instructions,
                } => {

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
                instructions,
                } => {

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
            }
        }
    }
}

fn main() -> std::io::Result<()> {
    let instructions = vec![
        RustInstruction::FunctionDefinition {
            is_public: false,
            name: "add_numbers".to_string(),
            argument_names: vec!["x".to_string(), "y".to_string()],
            argument_types: vec!["f64".to_string(), "f64".to_string()],
            return_type: "f64".to_string(),
            instructions: vec![
                RustInstruction::ReturnStatement {
                    is_final: true,
                    value: "x + y".to_string()
                },
            ],
        },
        RustInstruction::EmptyLine,
        RustInstruction::FunctionDefinition {
            is_public: false,
            name: "main".to_string(),
            argument_names: Vec::new(),
            argument_types: Vec::new(),
            return_type: "".to_string(),
            instructions: vec![
                RustInstruction::VariableDefinition {
                    is_mutable: true,
                    name: "x".to_string(),
                    value: "10.0".to_string()
                },
                RustInstruction::EmptyLine,
                RustInstruction::Loop {
                    count: "10".to_string(),
                    counter_name: "i".to_string(),
                    instructions: vec![
                        RustInstruction::Loop {
                            count: "10".to_string(),
                            counter_name: "j".to_string(),
                            instructions: vec![
                                RustInstruction::VariableAssignment { name: "x".to_string(), value: "add_numbers(x, 1)".to_string() },
                            ]
                        }
                    ]
                },
            ],
        },
    ];

    let mut file = File::create("test_output.txt")?;

    let mut output = String::new();
    for instruction in instructions.iter() {
        output.push_str(&instruction.get_string(0, 4));
    }

    file.write_all(output.as_bytes())?;

    Ok(())
}
